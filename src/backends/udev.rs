use std::collections::HashMap;
use smithay::{
	backend::{
		allocator::{
			dmabuf::{
				AnyError,
				Dmabuf,
			},
			Allocator,
		},
		drm::DrmNode,
		renderer::{
			element::texture::TextureBuffer,
			gles::GlesRenderer,
			multigpu::{
				gbm::GbmGlesBackend,
				GpuManager,
				MultiTexture,
			},
		},
		session::libseat::LibSeatSession,
	},
	reexports::{
		wayland_server::{
			DisplayHandle,
		},
	},
	wayland::{
		dmabuf::{
			DmabufGlobal,
			DmabufState,
		},
	},
};

pub struct UdevData {
	pub session: LibSeatSession,
	dh: DisplayHandle,
	dmabuf_state: Option<(DmabufState, DmabufGlobal)>,
	primary_gpu: DrmNode,
	allocator: Option<Box<dyn Allocator<Buffer = Dmabuf, Error = AnyError>>>,
	gpus: GpuManager<GbmGlesBackend<GlesRenderer>>,
	backends: HashMap<DrmNode, BackendData>,
	pointer_images: Vec<(xcursor::parser::Image, TextureBuffer<MultiTexture>)>,
	pointer_element: PointerElement<MultiTexture>,
	pointer_image: crate::backends::cursor::Cursor,
}
