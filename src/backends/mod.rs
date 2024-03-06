use log::error;

use crate::backends::{
	udev::UdevData,
	winit::WinitData,
};

pub mod cursor;
mod drawing;
pub mod udev;
pub mod winit;

pub enum Backend {
	Winit(WinitData),
	Udev(UdevData),
}

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
