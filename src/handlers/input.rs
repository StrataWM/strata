use std::{
	collections::HashMap,
	fmt::Display,
};

use crate::{
	state::StrataComp,
	workspaces::FocusTarget,
};
use bitflags::bitflags;
use smithay::{
	backend::input::{
		AbsolutePositionEvent,
		Axis,
		AxisSource,
		Event,
		InputBackend,
		PointerAxisEvent,
		PointerButtonEvent,
		PointerMotionEvent,
	},
	input::{
		keyboard::{
			Keysym,
			ModifiersState,
		},
		pointer::{
			AxisFrame,
			ButtonEvent,
			MotionEvent,
			RelativeMotionEvent,
		},
	},
	utils::{
		Logical,
		Point,
		SERIAL_COUNTER,
	},
};

#[derive(Debug)]
pub struct Mods {
	pub flags: ModFlags,
	pub state: ModifiersState,
}

// complete list, for future reference
//
// Shift_L Shift_R
// Control_L Control_R
// Meta_L Meta_R
// Alt_L Alt_R
// Super_L Super_R
// Hyper_L Hyper_R
// ISO_Level2_Latch
// ISO_Level3_Shift ISO_Level3_Latch ISO_Level3_Lock
// ISO_Level5_Shift ISO_Level5_Latch ISO_Level5_Lock
bitflags! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
	pub struct ModFlags: u8 {
		const Shift_L = 1;
		const Shift_R = 1 << 1;
		const Control_L = 1 << 1 + 1;
		const Control_R = 1 << 2;
		const Alt_L = 1 << 2 + 1;
		const Alt_R = 1 << 3;
		const Super_L = 1 << 3 + 1;
		const Super_R = 1 << 4;
		const ISO_Level3_Shift = 1 << 4 + 1;
		const ISO_Level5_Shift = 1 << 5;
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyPattern {
	pub mods: ModFlags,
	pub key: Keysym,
}

impl StrataComp {
	pub fn clamp_coords(&self, pos: Point<f64, Logical>) -> Point<f64, Logical> {
		if self.workspaces.current().outputs().next().is_none() {
			return pos;
		}

		let (pos_x, pos_y) = pos.into();
		let (max_x, max_y) = self
			.workspaces
			.current()
			.output_geometry(self.workspaces.current().outputs().next().unwrap())
			.unwrap()
			.size
			.into();
		let clamped_x = pos_x.max(0.0).min(max_x as f64);
		let clamped_y = pos_y.max(0.0).min(max_y as f64);
		(clamped_x, clamped_y).into()
	}

	pub fn set_input_focus(&mut self, target: FocusTarget) {
		let keyboard = self.seat.get_keyboard().unwrap();
		let serial = SERIAL_COUNTER.next_serial();
		keyboard.set_focus(self, Some(target), serial);
	}

	pub fn set_input_focus_auto(&mut self) {
		let under = self.surface_under();
		if let Some(d) = under {
			self.set_input_focus(d.0);
		}
	}

	pub fn pointer_motion<I: InputBackend>(
		&mut self,
		event: I::PointerMotionEvent,
	) -> anyhow::Result<()> {
		let serial = SERIAL_COUNTER.next_serial();
		let delta = (event.delta_x(), event.delta_y()).into();
		self.pointer_location += delta;
		self.pointer_location = self.clamp_coords(self.pointer_location);

		self.set_input_focus_auto();

		if let Some(ptr) = self.seat.get_pointer() {
			let under = self.surface_under();

			let location = self.pointer_location;
			ptr.motion(
				self,
				under.clone(),
				&MotionEvent { location, serial, time: event.time_msec() },
			);

			ptr.relative_motion(
				self,
				under,
				&RelativeMotionEvent {
					delta,
					delta_unaccel: event.delta_unaccel(),
					utime: event.time(),
				},
			)
		}

		Ok(())
	}

	pub fn pointer_motion_absolute<I: InputBackend>(
		&mut self,
		event: I::PointerMotionAbsoluteEvent,
	) -> anyhow::Result<()> {
		let serial = SERIAL_COUNTER.next_serial();

		let curr_workspace = self.workspaces.current();
		let output = curr_workspace.outputs().next().unwrap().clone();
		let output_geo = curr_workspace.output_geometry(&output).unwrap();
		let pos = event.position_transformed(output_geo.size) + output_geo.loc.to_f64();

		self.pointer_location = self.clamp_coords(pos);

		self.set_input_focus_auto();

		let under = self.surface_under();
		if let Some(ptr) = self.seat.get_pointer() {
			ptr.motion(
				self,
				under,
				&MotionEvent { location: pos, serial, time: event.time_msec() },
			);
		}

		Ok(())
	}
	pub fn pointer_button<I: InputBackend>(
		&mut self,
		event: I::PointerButtonEvent,
	) -> anyhow::Result<()> {
		let serial = SERIAL_COUNTER.next_serial();

		let button = event.button_code();
		let button_state = event.state();
		self.set_input_focus_auto();
		if let Some(ptr) = self.seat.get_pointer() {
			ptr.button(
				self,
				&ButtonEvent { button, state: button_state, serial, time: event.time_msec() },
			);
		}

		Ok(())
	}

	pub fn pointer_axis<I: InputBackend>(
		&mut self,
		event: I::PointerAxisEvent,
	) -> anyhow::Result<()> {
		let horizontal_amount = event
			.amount(Axis::Horizontal)
			.unwrap_or_else(|| event.amount_discrete(Axis::Horizontal).unwrap_or(0.0) * 3.0);
		let vertical_amount = event
			.amount(Axis::Vertical)
			.unwrap_or_else(|| event.amount_discrete(Axis::Vertical).unwrap_or(0.0) * 3.0);
		let horizontal_amount_discrete = event.amount_discrete(Axis::Horizontal);
		let vertical_amount_discrete = event.amount_discrete(Axis::Vertical);

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

		if let Some(ptr) = self.seat.get_pointer() {
			ptr.axis(self, frame);
		}

		Ok(())
	}
}
