mod libs;
pub use libs::{
	backends::winit::init_winit,
	parse_config::parse_config,
	structs::{
		CalloopData,
		Strata,
	},
};
use smithay::reexports::{
	calloop::EventLoop,
	wayland_server::Display,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
	// if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
	// 	tracing_subscriber::fmt().with_env_filter(env_filter).init();
	// } else {
	// 	tracing_subscriber::fmt().init();
	// }

	let mut event_loop: EventLoop<CalloopData> = EventLoop::try_new()?;

	let mut display: Display<Strata> = Display::new()?;
	let state = Strata::new(&mut event_loop, &mut display);

	let mut data = CalloopData { state, display };

	init_winit(&mut event_loop, &mut data)?;

	let mut args = std::env::args().skip(1);
	let flag = args.next();
	let arg = args.next();

	parse_config();

	std::process::Command::new("kitty").spawn().ok();
	std::process::Command::new("kitty").spawn().ok();

	event_loop.run(None, &mut data, move |_| {
		// Strata is running
	})?;

	Ok(())
}
