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
	reexports::wayland_server::DisplayHandle,
	wayland::dmabuf::{
		DmabufGlobal,
		DmabufState,
	},
};

struct BackendData {
	surfaces: HashMap<crtc::Handle, SurfaceData>,
	non_desktop_connectors: Vec<(connector::Handle, crtc::Handle)>,
	leasing_global: Option<DrmLeaseState>,
	active_leases: Vec<DrmLease>,
	gbm: GbmDevice<DrmDeviceFd>,
	drm: DrmDevice,
	drm_scanner: DrmScanner,
	render_node: DrmNode,
	registration_token: RegistrationToken,
}

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
