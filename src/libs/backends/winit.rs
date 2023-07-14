use std::{
	process::Command,
	time::Duration,
};

use smithay::{
	backend::{
		renderer::{
			damage::OutputDamageTracker,
			element::surface::WaylandSurfaceRenderElement,
			gles::GlesRenderer,
		},
		winit::{
			self,
			WinitError,
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
		wayland_server::Display,
	},
	utils::{
		Rectangle,
		Transform,
	},
};

use crate::libs::log::*;
pub use crate::libs::structs::{
	CalloopData,
	Strata,
};

pub fn init_winit() -> Result<(), Box<dyn std::error::Error>> {
	let mut event_loop: EventLoop<CalloopData> = EventLoop::try_new()?;
	let mut display: Display<Strata> = Display::new()?;
	let (mut backend, mut winit) = winit::init()?;
	let mode = Mode { size: backend.window_size().physical_size, refresh: 60_000 };

	let output = Output::new(
		"winit".to_string(),
		PhysicalProperties {
			size: (0, 0).into(),
			subpixel: Subpixel::Unknown,
			make: "strata".into(),
			model: "Winit".into(),
		},
	);
	let _global = output.create_global::<Strata>(&display.handle());

	let state = Strata::new(&mut event_loop, &mut display);
	let mut data = CalloopData { state, display };
	let state = &mut data.state;

	output.change_current_state(Some(mode), Some(Transform::Flipped180), None, Some((0, 0).into()));
	output.set_preferred(mode);

	state.space.map_output(&output, (0, 0));

	let mut damage_tracker = OutputDamageTracker::from_output(&output);

	std::env::set_var("WAYLAND_DISPLAY", &state.socket_name);

	let mut full_redraw = 0u8;

	let timer = Timer::immediate();
	event_loop.handle().insert_source(timer, move |_, _, data| {
		winit_dispatch(
			&mut backend,
			&mut winit,
			data,
			&output,
			&mut damage_tracker,
			&mut full_redraw,
		)
		.unwrap();
		TimeoutAction::ToDuration(Duration::from_millis(16))
	})?;

	Command::new("kitty").spawn().ok();
	Command::new("kitty").spawn().ok();

	event_loop.run(None, &mut data, move |_| {})?;
	Ok(())
}

pub fn winit_dispatch(
	backend: &mut WinitGraphicsBackend<GlesRenderer>,
	winit: &mut WinitEventLoop,
	data: &mut CalloopData,
	output: &Output,
	damage_tracker: &mut OutputDamageTracker,
	full_redraw: &mut u8,
) -> Result<(), Box<dyn std::error::Error>> {
	let display = &mut data.display;
	let state = &mut data.state;

	let res = winit.dispatch_new_events(|event| {
		match event {
			WinitEvent::Resized { size, .. } => {
				output.change_current_state(Some(Mode { size, refresh: 60_000 }), None, None, None);
			}
			WinitEvent::Input(event) => state.process_input_event(event),
			_ => (),
		}
	});

	if let Err(WinitError::WindowClosed) = res {
		// Stop the loop
		state.loop_signal.stop();

		return Ok(());
	} else {
		res?;
	}

	*full_redraw = full_redraw.saturating_sub(1);

	let size = backend.window_size().physical_size;
	let damage = Rectangle::from_loc_and_size((0, 0), size);

	backend.bind()?;
	smithay::desktop::space::render_output::<_, WaylandSurfaceRenderElement<GlesRenderer>, _, _>(
		output,
		backend.renderer(),
		1.0,
		0,
		[&state.space],
		&[],
		damage_tracker,
		[0.131, 0.141, 0.242, 1.0],
	)?;
	backend.submit(Some(&[damage]))?;

	state.space.elements().for_each(|window| {
		window.send_frame(output, state.start_time.elapsed(), Some(Duration::ZERO), |_, _| {
			Some(output.clone())
		})
	});

	state.space.refresh();
	display.flush_clients()?;

	Ok(())
}
