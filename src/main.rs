mod libs;
use smithay::reexports::{calloop::EventLoop, wayland_server::Display};
pub use libs::structs::Strata;
pub use libs::structs::CalloopData;
pub use libs::backends::winit::init_winit;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
        tracing_subscriber::fmt().with_env_filter(env_filter).init();
    } else {
        tracing_subscriber::fmt().init();
    }

    let mut event_loop: EventLoop<CalloopData> = EventLoop::try_new()?;

    let mut display: Display<Strata> = Display::new()?;
    let state = Strata::new(&mut event_loop, &mut display);

    let mut data = CalloopData { state, display };

    init_winit(&mut event_loop, &mut data)?;

    let mut args = std::env::args().skip(1);
    let flag = args.next();
    let arg = args.next();

    match (flag.as_deref(), arg) {
        (Some("-c") | Some("--command"), Some(command)) => {
            std::process::Command::new(command).spawn().ok();
        }
        _ => {
            std::process::Command::new("kitty").spawn().ok();
        }
    }

    event_loop.run(None, &mut data, move |_| {
        // Strata is running
    })?;

    Ok(())
}