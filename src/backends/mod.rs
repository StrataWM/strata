pub mod winit;
use log::error;

pub fn init_with_backend(backend_name: &str) {
	match backend_name {
		"winit" => {
			winit::init_winit();
		}
		"udev" => {
			error!("Udev is not implemented yet!");
		}
		unknown => {
			error!("Unknown backend provided: {}", unknown)
		}
	}
}
