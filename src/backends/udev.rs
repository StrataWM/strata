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
		input::{
			Event,
			InputBackend,
			InputEvent,
			KeyState,
			KeyboardKeyEvent,
		},
		renderer::{
			damage::OutputDamageTracker,
			element::texture::TextureBuffer,
			gles::GlesRenderer,
			glow::GlowRenderer,
			multigpu::{
				gbm::GbmGlesBackend,
				GpuManager,
				MultiTexture,
			},
		},
		session::libseat::LibSeatSession,
		winit::WinitGraphicsBackend,
	},
	desktop::{
		layer_map_for_output,
		space::SpaceElement,
		PopupManager,
	},
	input::{
		keyboard::{
			FilterResult,
			Keysym,
			ModifiersState,
			XkbConfig,
		},
		Seat,
		SeatState,
	},
	reexports::{
		calloop::{
			generic::{
				FdWrapper,
				Generic,
			},
			EventLoop,
			Interest,
			LoopSignal,
			Mode,
			PostAction,
		},
		wayland_server::{
			backend::{
				ClientData,
				ClientId,
				DisconnectReason,
			},
			Display,
			DisplayHandle,
		},
	},
	utils::{
		Logical,
		Point,
		Rectangle,
		SERIAL_COUNTER,
	},
	wayland::{
		compositor::{
			CompositorClientState,
			CompositorState,
		},
		dmabuf::{
			DmabufGlobal,
			DmabufState,
		},
		output::OutputManagerState,
		selection::{
			data_device::DataDeviceState,
			primary_selection::PrimarySelectionState,
		},
		shell::{
			wlr_layer::{
				Layer,
				WlrLayerShellState,
			},
			xdg::{
				decoration::XdgDecorationState,
				XdgShellState,
			},
		},
		shm::ShmState,
		socket::ListeningSocketSource,
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
