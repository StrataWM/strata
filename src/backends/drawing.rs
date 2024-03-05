use smithay::{
	backend::renderer::{
		element::{
			memory::{
				MemoryRenderBuffer,
				MemoryRenderBufferRenderElement,
			},
			surface::WaylandSurfaceRenderElement,
			AsRenderElements,
			Kind,
		},
		ImportAll,
		ImportMem,
		Renderer,
		Texture,
	},
	input::pointer::CursorImageStatus,
	render_elements,
	utils::{
		Physical,
		Point,
		Scale,
	},
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

impl<T: Texture + 'static, R> AsRenderElements<R> for PointerElement
where
	R: Renderer<TextureId = T> + ImportAll + ImportMem,
{
	type RenderElement = PointerRenderElement<R>;
	fn render_elements<E>(
		&self,
		renderer: &mut R,
		location: Point<i32, Physical>,
		scale: Scale<f64>,
		alpha: f32,
	) -> Vec<E>
	where
		E: From<PointerRenderElement<R>>,
	{
		match &self.status {
			CursorImageStatus::Hidden => vec![],
			CursorImageStatus::Named(_) => {
				if let Some(buffer) = self.buffer.as_ref() {
					vec![PointerRenderElement::<R>::from(
						MemoryRenderBufferRenderElement::from_buffer(
							renderer,
							location.to_f64(),
							buffer,
							None,
							None,
							None,
							Kind::Cursor,
						)
						.expect("Lost system pointer buffer"),
					)
					.into()]
				} else {
					vec![]
				}
			}
			CursorImageStatus::Surface(surface) => {
				let elements: Vec<PointerRenderElement<R>> =
					smithay::backend::renderer::element::surface::render_elements_from_surface_tree(
						renderer,
						surface,
						location,
						scale,
						alpha,
						Kind::Cursor,
					);
				elements.into_iter().map(E::from).collect()
			}
		}
	}
}
