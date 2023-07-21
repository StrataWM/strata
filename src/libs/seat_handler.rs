use crate::libs::structs::Strata;

use smithay::{
	delegate_data_device,
	delegate_output,
	delegate_seat,
	input::{
		SeatHandler,
		SeatState,
	},
	reexports::wayland_server::protocol::wl_surface::WlSurface,
	wayland::data_device::{
		ClientDndGrabHandler,
		DataDeviceHandler,
		ServerDndGrabHandler,
	},
};

impl<BackendData> SeatHandler for Strata<BackendData> {
	type KeyboardFocus = WlSurface;
	type PointerFocus = WlSurface;

	fn seat_state(&mut self) -> &mut SeatState<Strata<BackendData>> {
		&mut self.seat_state
	}

	fn cursor_image(
		&mut self,
		_seat: &smithay::input::Seat<Self>,
		_image: smithay::input::pointer::CursorImageStatus,
	) {
	}
	fn focus_changed(&mut self, _seat: &smithay::input::Seat<Self>, _focused: Option<&WlSurface>) {}
}
delegate_seat!(Strata<BackendData>);

impl<BackendData> DataDeviceHandler for Strata<BackendData> {
	type SelectionUserData = ();
	fn data_device_state(&self) -> &smithay::wayland::data_device::DataDeviceState {
		&self.data_device_state
	}
}

impl<BackendData> ClientDndGrabHandler for Strata<BackendData> {}
impl<BackendData> ServerDndGrabHandler for Strata<BackendData> {}

delegate_data_device!(Strata<BackendData>);
delegate_output!(Strata<BackendData>);
