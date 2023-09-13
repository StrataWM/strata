use smithay::backend::renderer::gles::GlesPixelProgram;

pub struct BorderShader {
	pub rounded: GlesPixelProgram,
	pub default: GlesPixelProgram,
}
