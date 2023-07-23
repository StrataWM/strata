use smithay::backend::{
	renderer::{
		damage::OutputDamageTracker,
		glow::GlowRenderer,
	},
	winit::WinitGraphicsBackend,
};

pub struct WinitData {
	pub backend: WinitGraphicsBackend<GlowRenderer>,
	pub damage_tracker: OutputDamageTracker,
}
