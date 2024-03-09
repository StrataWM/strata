// Copyright 2023 the Strata authors
// SPDX-License-Identifier: GPL-3.0-or-later

use std::{
	cell::RefCell,
	rc::Rc,
	time::Duration,
};

use piccolo as lua;
use smithay::{
	backend::{
		renderer::{
			damage::OutputDamageTracker,
			glow::GlowRenderer,
		},
		winit::{
			self,
			WinitEvent,
			WinitEventLoop,
			WinitGraphicsBackend,
		},
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
	utils::Transform,
};

use crate::{
	backends::Backend,
	bindings,
	decorations::BorderShader,
	state::{
		self,
		Compositor,
		Strata,
	},
};

pub struct WinitData {
	pub backend: WinitGraphicsBackend<GlowRenderer>,
	pub damage_tracker: OutputDamageTracker,
}

pub fn init_winit() {
	let mut event_loop: EventLoop<Strata> = EventLoop::try_new().unwrap();
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
	let _global = output.create_global::<Compositor>(&display_handle);
	output.change_current_state(Some(mode), Some(Transform::Flipped180), None, Some((0, 0).into()));
	output.set_preferred(mode);
	let damage_tracker = OutputDamageTracker::from_output(&output);
	let mut comp = Compositor::new(
		&event_loop,
		&display,
		socket,
		"winit".to_string(),
		Backend::Winit(WinitData { backend, damage_tracker }),
	);
	BorderShader::init(comp.backend.winit().backend.renderer());
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

				local _ = Key({ Mod.Alt_L }, Key.Return, function()
					strata.spawn('kitty')
				end)

				local _ = Key({ Mod.Alt_L }, Key.m, function()
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

	let mut data = Strata { lua: lua_vm, comp, display };
	event_loop.run(None, &mut data, move |_| {}).unwrap();
}

pub fn winit_dispatch(winit: &mut WinitEventLoop, state: &mut Strata, output: &Output) {
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
		state.comp.borrow_mut().winit_update();
	}
}
