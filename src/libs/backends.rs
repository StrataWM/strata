pub mod winit;
use log::error;
use std::error::Error;

pub async fn init_with_backend(backend_name: String) -> Result<(), Box<dyn Error>> {
	match backend_name.as_str() {
		"winit" => {
			winit::init_winit();
			Ok(())
		}
		"udev" => {
			error!("Udev is not implemented yet!");
			Ok(())
		}
		unknown => {
			error!("Unknown backend provided: {}", unknown);
			Ok(())
		}
	}
}
