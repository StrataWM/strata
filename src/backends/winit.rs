use crate::{
	bindings,
	decorations::{
		BorderShader,
		CustomRenderElements,
	},
	state::{
		self,
		StrataComp,
		StrataState,
	},
};
use piccolo::{
	self as lua,
};
use smithay::{
	backend::{
		renderer::{
			damage::OutputDamageTracker,
			element::AsRenderElements,
			glow::GlowRenderer,
		},
		winit::{
			self,
			WinitEvent,
			WinitEventLoop,
		},
	},
	desktop::{
		layer_map_for_output,
		space::SpaceElement,
		LayerSurface,
	},
	output::{
		Mode,
		Output,
		PhysicalProperties,
		Subpixel,
	},
	reexports::{
		calloop::{
			timer::{
				TimeoutAction,
				Timer,
			},
			EventLoop,
		},
		winit::platform::pump_events::PumpStatus,
	},
	utils::{
		Rectangle,
		Scale,
		Transform,
	},
	wayland::shell::wlr_layer::Layer,
};
use std::{
	cell::RefCell,
	rc::Rc,
	time::Duration,
};

pub fn init_winit() {
	let mut event_loop: EventLoop<StrataState> = EventLoop::try_new().unwrap();
	let (display, socket) = state::init_wayland_listener(&event_loop);
	let display_handle = display.handle();
	let (backend, mut winit) = winit::init().unwrap();
	let mode = Mode { size: backend.window_size(), refresh: 60_000 };
	let output = Output::new(
		"winit".to_string(),
		PhysicalProperties {
			size: (0, 0).into(),
			subpixel: Subpixel::Unknown,
			make: "Strata".into(),
			model: "Winit".into(),
		},
	);
	let _global = output.create_global::<StrataComp>(&display_handle);
	output.change_current_state(Some(mode), Some(Transform::Flipped180), None, Some((0, 0).into()));
	output.set_preferred(mode);
	let damage_tracked_renderer = OutputDamageTracker::from_output(&output);
	let mut comp = StrataComp::new(
		&event_loop,
		&display,
		socket,
		"winit".to_string(),
		backend,
		damage_tracked_renderer,
	);
	BorderShader::init(comp.backend.renderer());
	for workspace in comp.workspaces.iter() {
		workspace.add_output(output.clone());
	}

	std::env::set_var("WAYLAND_DISPLAY", &comp.socket_name);

	let timer = Timer::immediate();
	event_loop
		.handle()
		.insert_source(timer, move |_, _, data| {
			winit_dispatch(&mut winit, data, &output);
			TimeoutAction::ToDuration(Duration::from_millis(16))
		})
		.unwrap();

	let mut lua_vm = lua::Lua::full();
	let comp = Rc::new(RefCell::new(comp));

	let ex = lua_vm
		.try_enter(|ctx| {
			bindings::register(ctx, Rc::clone(&comp))?;

			let main = lua::Closure::load(
				ctx,
				None,
				r#"
				local Key = strata.input.Key
				local Mod = strata.input.Mod

				-- print(Mod.Super_L)
				-- print(Key.Escape)

				local _ = Key({ Mod.Control_L, Mod.Alt_L }, Key.Return, function()
					strata.spawn('kitty')
				end)

				local _ = Key({ Mod.Control_L, Mod.Alt_L }, Key.Escape, function()
					strata:quit()
				end)
				"#
				.as_bytes(),
			)?;

			Ok(ctx.stash(lua::Executor::start(ctx, main.into(), ())))
		})
		.unwrap();

	if let Err(e) = lua_vm.execute::<()>(&ex) {
		println!("{:#?}", e);
	}

	let mut data = StrataState { lua: lua_vm, comp, display };
	event_loop.run(None, &mut data, move |_| {}).unwrap();
}

pub fn state_winit_update(state: &mut StrataComp) {
	// get layer surfaces from the current workspace
	let workspace = state.workspaces.current_mut();
	let output = workspace.outputs().next().unwrap();
	let layer_map = layer_map_for_output(output);
	let (lower, upper): (Vec<&LayerSurface>, Vec<&LayerSurface>) = layer_map
		.layers()
		.rev()
		.partition(|s| matches!(s.layer(), Layer::Background | Layer::Bottom));

	// render layers
	let mut renderelements: Vec<CustomRenderElements<_>> = vec![];
	renderelements.extend(
		upper
			.into_iter()
			.filter_map(|surface| layer_map.layer_geometry(surface).map(|geo| (geo.loc, surface)))
			.flat_map(|(loc, surface)| {
				AsRenderElements::<GlowRenderer>::render_elements::<CustomRenderElements<_>>(
					surface,
					state.backend.renderer(),
					loc.to_physical_precise_round(1),
					Scale::from(1.0),
					1.0,
				)
			}),
	);
	renderelements.extend(workspace.render_elements(state.backend.renderer()));
	renderelements.extend(
		lower
			.into_iter()
			.filter_map(|surface| layer_map.layer_geometry(surface).map(|geo| (geo.loc, surface)))
			.flat_map(|(loc, surface)| {
				AsRenderElements::<GlowRenderer>::render_elements::<CustomRenderElements<_>>(
					surface,
					state.backend.renderer(),
					loc.to_physical_precise_round(1),
					Scale::from(1.0),
					1.0,
				)
			}),
	);
	state
		.damage_tracker
		.render_output(state.backend.renderer(), 0, &renderelements, [0.1, 0.1, 0.1, 1.0])
		.unwrap();

	// damage tracking
	let size = state.backend.window_size();
	let damage = Rectangle::from_loc_and_size((0, 0), size);
	state.backend.bind().unwrap();
	state.backend.submit(Some(&[damage])).unwrap();

	// sync and cleanups
	workspace.windows().for_each(|window| {
		window.send_frame(output, state.start_time.elapsed(), Some(Duration::ZERO), |_, _| {
			Some(output.clone())
		});

		window.refresh();
	});
	state.dh.flush_clients().unwrap();
	state.popup_manager.cleanup();
	BorderShader::cleanup(state.backend.renderer());
}

pub fn winit_dispatch(winit: &mut WinitEventLoop, state: &mut StrataState, output: &Output) {
	// process winit events
	let res = winit.dispatch_new_events(|event| {
		match event {
			WinitEvent::Resized { size, .. } => {
				output.change_current_state(Some(Mode { size, refresh: 60_000 }), None, None, None);
			}
			WinitEvent::Input(event) => {
				if let Err(e) = state.process_input_event(event) {
					panic!("{:#?}", e);
				}
			}
			_ => (),
		}
	});

	if let PumpStatus::Exit(_) = res {
		state.comp.borrow().loop_signal.stop();
	} else {
		state_winit_update(&mut state.comp.borrow_mut());
	}
}
