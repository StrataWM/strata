use smithay::{
	backend::renderer::{
		element::{
			memory::{
				MemoryRenderBuffer,
				MemoryRenderBufferRenderElement,
			},
			surface::WaylandSurfaceRenderElement,
		},
		ImportAll,
		ImportMem,
	},
	input::pointer::CursorImageStatus,
	render_elements,
};

pub struct PointerElement {
	buffer: Option<MemoryRenderBuffer>,
	status: CursorImageStatus,
}

impl Default for PointerElement {
	fn default() -> Self {
		Self { buffer: Default::default(), status: CursorImageStatus::default_named() }
	}
}

impl PointerElement {
	pub fn set_status(&mut self, status: CursorImageStatus) {
		self.status = status;
	}

	pub fn set_buffer(&mut self, buffer: MemoryRenderBuffer) {
		self.buffer = Some(buffer);
	}
}

render_elements! {
	pub PointerRenderElement<R> where R: ImportAll + ImportMem;
	Surface=WaylandSurfaceRenderElement<R>,
	Memory=MemoryRenderBufferRenderElement<R>,
}
