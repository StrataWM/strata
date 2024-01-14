use crate::{
	decorations::{
		BorderShader,
		CustomRenderElements,
	},
	state::{
		self,
		StrataComp,
		StrataState,
	}, handlers::input::{KeyPattern, ModFlags},
};
use piccolo::{
	Closure,
	Lua,
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
			WinitError,
			WinitEvent,
			WinitEventLoop,
		},
	},
	desktop::{
		layer_map_for_output,
		space::SpaceElement,
		LayerSurface,
	},
	input::keyboard::{
		keysyms,
		xkb,
		Keysym,
	},
	output::{
		Mode,
		Output,
		PhysicalProperties,
		Subpixel,
	},
	reexports::calloop::{
		timer::{
			TimeoutAction,
			Timer,
		},
		EventLoop,
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
	collections::HashMap,
	rc::Rc,
	time::Duration,
};

pub fn init_winit() {
	let mut event_loop: EventLoop<StrataState> = EventLoop::try_new().unwrap();
	let (display, socket) = state::init_wayland_listener(&event_loop);
	let display_handle = display.handle();
	let (backend, mut winit) = winit::init().unwrap();
	let mode = Mode { size: backend.window_size().physical_size, refresh: 60_000 };
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

	let mut lua = Lua::core();
	let comp = Rc::new(RefCell::new(comp));
	let mut config = HashMap::new();

	lua.try_enter(|ctx| {
		let quit = Closure::load(
			ctx,
			None,
			r#"
			strata:quit()
			"#
			.as_bytes(),
		)?;
		let quit_key = KeyPattern {
			mods: ModFlags::Super_L | ModFlags::Control_L,
			key: keysyms::KEY_Escape.into(),
		};
		config.insert(quit_key, ctx.stash(quit).into());

		let ud = StrataComp::ud_from_rc_refcell(ctx, Rc::clone(&comp))?;
		ctx.globals().set(ctx, "strata", ud)?;

		Ok(())
	})
	.unwrap();

	let mut data = StrataState { lua, config, comp, display };
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
	let size = state.backend.window_size().physical_size;
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
	// workspace.windows().for_each(|e| e.refresh());
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
				let _ = state.process_input_event(event);
			}
			_ => (),
		}
	});

	if let Err(WinitError::WindowClosed) = res {
		state.comp.borrow().loop_signal.stop();
	} else {
		state_winit_update(&mut state.comp.borrow_mut());
	}
}
