use smithay::{
	backend::renderer::{
		element::{
			memory::MemoryRenderBuffer,
			AsRenderElements,
		},
		Renderer,
	},
	input::pointer::CursorImageStatus,
};

pub struct PointerElement {
	buffer: Option<MemoryRenderBuffer>,
	status: CursorImageStatus,
}
