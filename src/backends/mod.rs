// Copyright 2023 the Strata authors
// SPDX-License-Identifier: GPL-3.0-or-later

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
	Unset,
}

impl Backend {
	pub fn winit(&mut self) -> &mut WinitData {
		match self {
			Backend::Winit(data) => data,
			_ => unreachable!("Tried to retrieve Winit backend when not initialized with it."),
		}
	}

	pub fn udev(&mut self) -> &mut UdevData {
		match self {
			Backend::Udev(data) => data,
			_ => unreachable!("Tried to retrieve Udev backend when not initialized with it."),
		}
	}
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
