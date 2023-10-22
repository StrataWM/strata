pub struct CommsChannel<T> {
	pub sender: crossbeam_channel::Sender<T>,
	pub receiver: crossbeam_channel::Receiver<T>,
}

pub enum ConfigCommands {
	Spawn(String),
	CloseWindow,
	SwitchWS(u8),
	MoveWindow(u8),
	MoveWindowAndFollow(u8),
	Quit,
}
