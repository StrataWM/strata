use crate::{
	decorations::{
		BorderShader,
		CustomRenderElements,
	},
	state::{
		StrataComp,
		StrataData,
		StrataRT,
	},
};
use gc_arena::{
	lock::RefLock,
	Rootable,
};
use piccolo::{
	Lua,
	UserData,
};
use smithay::{
	backend::{
		input::{
			AbsolutePositionEvent,
			Axis,
			AxisSource,
			Event,
			InputBackend,
			InputEvent,
			KeyboardKeyEvent,
			PointerAxisEvent,
			PointerButtonEvent,
			PointerMotionEvent,
		},
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
	input::{
		keyboard::FilterResult,
		pointer::{
			AxisFrame,
			ButtonEvent,
			MotionEvent,
			RelativeMotionEvent,
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
		wayland_server::Display,
	},
	utils::{
		Rectangle,
		Scale,
		Transform,
		SERIAL_COUNTER,
	},
	wayland::shell::wlr_layer::Layer,
};
use std::{
	cell::RefCell,
	process::Command,
	time::{Duration, Instant},
};

pub fn init_winit() {
	let mut event_loop: EventLoop<StrataData> = EventLoop::try_new().unwrap();
	let display: Display<StrataComp> = Display::new().unwrap();
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
	let mut state = StrataComp::new(
		&mut event_loop,
		display,
		"winit".to_string(),
		backend,
		damage_tracked_renderer,
	);
	BorderShader::init(state.backend.renderer());
	for workspace in state.workspaces.iter() {
		workspace.add_output(output.clone());
	}

	std::env::set_var("WAYLAND_DISPLAY", &state.socket_name);
	let timer = Timer::immediate();

	event_loop
		.handle()
		.insert_source(timer, move |_, _, data| {
			winit_dispatch(&mut winit, data, &output);
			TimeoutAction::ToDuration(Duration::from_millis(16))
		})
		.unwrap();

	// // Autostart applications
	// for cmd in &CONFIG.read().autostart {
	// 	Command::new("/bin/sh").arg("-c").args(cmd).spawn().ok();
	// }

	let rt = StrataRT::new(state);
	let mut data = StrataData { rt, display_handle };

	event_loop.run(None, &mut data, move |_| {}).unwrap();
}

pub fn state_winit_dispatch(state: &mut StrataComp) {
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

pub fn process_input_event<I: InputBackend>(state: &RefCell<StrataComp>, event: InputEvent<I>) {
	match event {
		InputEvent::Keyboard { event, .. } => {
			let s = &mut state.borrow_mut();
			let serial = SERIAL_COUNTER.next_serial();
			let time = Event::time_msec(&event);

			let keyboard = s.seat.get_keyboard().unwrap();
			if let Some(action) = keyboard.input(
				s,
				event.key_code(),
				event.state(),
				serial,
				time,
				|_, mods, handle| {
					// for binding in &CONFIG.read().bindings {
					// 	let mut keysym: Keysym =
					// 		xkb::utf32_to_keysym(xkb::keysyms::KEY_NoSymbol);
					// 	let mut modifier_state = ModifiersState::default();
					//
					// 	for key in &binding.keys {
					// 		match key.as_str() {
					// 			"Super_L" => modifier_state.logo = true,
					// 			"Super_R" => modifier_state.logo = true,
					// 			"Shift_L" => modifier_state.shift = true,
					// 			"Shift_R" => modifier_state.shift = true,
					// 			"Alt_L" => modifier_state.alt = true,
					// 			"Alt_R" => modifier_state.alt = true,
					// 			"Ctrl_L" => modifier_state.ctrl = true,
					// 			"Ctrl_R" => modifier_state.ctrl = true,
					// 			"CapsLck" => modifier_state.caps_lock = true,
					// 			"Caps" => modifier_state.caps_lock = true,
					// 			&_ => {
					// 				let sym = xkb::keysym_from_name(
					// 					key.as_str(),
					// 					xkb::KEYSYM_NO_FLAGS,
					// 				);
					// 				keysym = sym;
					// 			}
					// 		}
					// 	}
					// 	if event.state() == KeyState::Released
					// 		&& modifier_state.alt == mods.alt && modifier_state.ctrl
					// 		== mods.ctrl && modifier_state.shift == mods.shift
					// 		&& modifier_state.logo == mods.logo && modifier_state.caps_lock
					// 		== mods.caps_lock && handle.raw_syms().contains(&keysym)
					// 	{
					// 		// let _ = binding.action.call(&LUA.lock());
					// 		// let action = CHANNEL.lock().unwrap().receiver.recv().unwrap();
					// 		return FilterResult::Intercept(());
					// 	}
					// }
					FilterResult::Intercept(())
				},
			) {
				// println!("do stuff");
			}
		}
		InputEvent::PointerMotion { event } => {
			let s = &mut state.borrow_mut();
			let serial = SERIAL_COUNTER.next_serial();
			let delta = (event.delta_x(), event.delta_y()).into();
			s.pointer_location += delta;
			s.pointer_location = s.clamp_coords(s.pointer_location);

			let under = s.surface_under();

			s.set_input_focus_auto();

			if let Some(ptr) = s.seat.get_pointer() {
				ptr.motion(
					s,
					under.clone(),
					&MotionEvent {
						location: state.borrow().pointer_location,
						serial,
						time: event.time_msec(),
					},
				);

				ptr.relative_motion(
					s,
					under,
					&RelativeMotionEvent {
						delta,
						delta_unaccel: event.delta_unaccel(),
						utime: event.time(),
					},
				)
			}
		}
		InputEvent::PointerMotionAbsolute { event, .. } => {
			let s = &mut state.borrow_mut();
			let output = s.workspaces.current().outputs().next().unwrap().clone();
			let output_geo = s.workspaces.current().output_geometry(&output).unwrap();
			let pos = event.position_transformed(output_geo.size) + output_geo.loc.to_f64();
			let serial = SERIAL_COUNTER.next_serial();
			let pointer = s.seat.get_pointer().unwrap();
			s.pointer_location = s.clamp_coords(pos);
			let under = s.surface_under();
			s.set_input_focus_auto();
			pointer.motion(
				s,
				under,
				&MotionEvent { location: pos, serial, time: event.time_msec() },
			);
		}
		InputEvent::PointerButton { event, .. } => {
			let s = &mut state.borrow_mut();
			let pointer = s.seat.get_pointer().unwrap();
			let serial = SERIAL_COUNTER.next_serial();
			let button = event.button_code();
			let button_state = event.state();
			s.set_input_focus_auto();
			pointer.button(
				s,
				&ButtonEvent { button, state: button_state, serial, time: event.time_msec() },
			);
		}
		InputEvent::PointerAxis { event, .. } => {
			let s = &mut state.borrow_mut();
			let horizontal_amount = event
				.amount(Axis::Horizontal)
				.unwrap_or_else(|| event.amount_discrete(Axis::Horizontal).unwrap_or(0.0) * 3.0);
			let vertical_amount = event
				.amount(Axis::Vertical)
				.unwrap_or_else(|| event.amount_discrete(Axis::Vertical).unwrap_or(0.0) * 3.0);
			let horizontal_amount_discrete = event.amount_discrete(Axis::Horizontal);
			let vertical_amount_discrete = event.amount_discrete(Axis::Vertical);

			{
				let mut frame = AxisFrame::new(event.time_msec()).source(event.source());
				if horizontal_amount != 0.0 {
					frame = frame.value(Axis::Horizontal, horizontal_amount);
					if let Some(discrete) = horizontal_amount_discrete {
						frame = frame.discrete(Axis::Horizontal, discrete as i32);
					}
				} else if event.source() == AxisSource::Finger {
					frame = frame.stop(Axis::Horizontal);
				}
				if vertical_amount != 0.0 {
					frame = frame.value(Axis::Vertical, vertical_amount);
					if let Some(discrete) = vertical_amount_discrete {
						frame = frame.discrete(Axis::Vertical, discrete as i32);
					}
				} else if event.source() == AxisSource::Finger {
					frame = frame.stop(Axis::Vertical);
				}
				s.seat.get_pointer().unwrap().axis(s, frame);
			}
		}
		_ => {}
	}
}

pub fn winit_dispatch(winit: &mut WinitEventLoop, data: &mut StrataData, output: &Output) {
	data.rt
		.with_state(|_, state| {
			// process winit events
			let res = winit.dispatch_new_events(|event| {
				match event {
					WinitEvent::Resized { size, .. } => {
						output.change_current_state(
							Some(Mode { size, refresh: 60_000 }),
							None,
							None,
							None,
						);
					}
					WinitEvent::Input(event) => process_input_event(state, event),
					_ => (),
				}
			});

			if let Err(WinitError::WindowClosed) = res {
				state.borrow().loop_signal.stop();
			} else {
				res.unwrap();
			}

			state_winit_dispatch(&mut state.borrow_mut());
		})
		.unwrap();
}
