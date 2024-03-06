use std::collections::HashMap;

use smithay::{
	backend::{
		allocator::gbm::GbmDevice,
		drm::{
			DrmDevice,
			DrmDeviceFd,
			DrmNode,
		},
		renderer::{
			element::memory::MemoryRenderBuffer,
			gles::GlesRenderer,
			multigpu::{
				gbm::GbmGlesBackend,
				GpuManager,
			},
			DebugFlags,
		},
		session::libseat::LibSeatSession,
	},
	reexports::{
		calloop::RegistrationToken,
		drm::control::{
			connector,
			crtc,
		},
		wayland_server::DisplayHandle,
	},
	wayland::{
		compositor::SurfaceData,
		dmabuf::{
			DmabufGlobal,
			DmabufState,
		},
		drm_lease::{
			DrmLease,
			DrmLeaseState,
		},
	},
};
use smithay_drm_extras::drm_scanner::DrmScanner;

use crate::backends::{
	cursor::Cursor,
	drawing::PointerElement,
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
	gpus: GpuManager<GbmGlesBackend<GlesRenderer>>,
	backends: HashMap<DrmNode, BackendData>,
	pointer_images: Vec<(xcursor::parser::Image, MemoryRenderBuffer)>,
	pointer_element: PointerElement,
	pointer_image: Cursor,
	debug_flags: DebugFlags,
	keyboards: Vec<smithay::reexports::input::Device>,
}
