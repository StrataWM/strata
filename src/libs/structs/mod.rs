pub mod args;
pub mod decorations;
pub mod state;
pub mod workspaces;

pub struct comms_channel {
	pub sender: crossbeam_channel::Sender<_>,
	pub receiver: crossbeam_channel::Receiver<_>,
}
