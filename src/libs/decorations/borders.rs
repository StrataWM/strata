use crate::{
	libs::structs::decorations::BorderShader,
	CONFIG,
};
use smithay::{
	backend::renderer::{
		element::Element,
		gles::{
			element::PixelShaderElement,
			GlesRenderer,
			Uniform,
			UniformName,
			UniformType,
		},
		glow::GlowRenderer,
	},
	desktop::Window,
	utils::{
		IsAlive,
		Logical,
		Point,
		Rectangle,
		Size,
	},
};
use std::{
	borrow::BorrowMut,
	cell::RefCell,
	collections::HashMap,
};

const ROUNDED_BORDER_FRAG: &str = include_str!("shaders/rounded_corners.frag");
const BORDER_FRAG: &str = include_str!("shaders/borders.frag");

struct BorderShaderElements(RefCell<HashMap<Window, PixelShaderElement>>);

impl BorderShader {
	pub fn init(renderer: &mut GlowRenderer) {
		let renderer: &mut GlesRenderer = renderer.borrow_mut();
		let rounded = renderer
			.compile_custom_pixel_shader(
				ROUNDED_BORDER_FRAG,
				&[
					UniformName::new("startColor", UniformType::_3f),
					UniformName::new("endColor", UniformType::_3f),
					UniformName::new("thickness", UniformType::_1f),
					UniformName::new("halfThickness", UniformType::_1f),
					UniformName::new("radius", UniformType::_1f),
					UniformName::new("gradientDirection", UniformType::_2f),
				],
			)
			.unwrap();
		let default = renderer
			.compile_custom_pixel_shader(
				BORDER_FRAG,
				&[
					UniformName::new("startColor", UniformType::_3f),
					UniformName::new("endColor", UniformType::_3f),
					UniformName::new("thickness", UniformType::_1f),
					UniformName::new("halfThickness", UniformType::_1f),
					UniformName::new("gradientDirection", UniformType::_2f),
				],
			)
			.unwrap();
		renderer.egl_context().user_data().insert_if_missing(|| BorderShader { rounded, default });
		renderer
			.egl_context()
			.user_data()
			.insert_if_missing(|| BorderShaderElements(RefCell::new(HashMap::new())));
	}
	pub fn get(renderer: &GlowRenderer) -> &BorderShader {
		renderer
			.egl_context()
			.user_data()
			.get::<BorderShader>()
			.expect("Border Shader not initialized")
	}
	pub fn element(
		renderer: &mut GlowRenderer,
		window: &Window,
		loc: Point<i32, Logical>,
	) -> PixelShaderElement {
		let thickness: f32 = CONFIG.read().decorations.border.width as f32;
		let thickness_loc = (thickness as i32, thickness as i32);
		let thickness_size = ((thickness * 2.0) as i32, (thickness * 2.0) as i32);
		let geo = Rectangle::from_loc_and_size(
			loc - Point::from(thickness_loc),
			window.geometry().size + Size::from(thickness_size),
		);
		let elements = &mut renderer
			.egl_context()
			.user_data()
			.get::<BorderShaderElements>()
			.expect("Border Shader not initialized")
			.0
			.borrow_mut();
		if let Some(elem) = elements.get_mut(window) {
			if elem.geometry(1.0.into()).to_logical(1) != geo {
				elem.resize(geo, None);
			}
			elem.clone()
		} else {
			let angle = 45 as f32 * std::f32::consts::PI;
			let gradient_direction = [angle.cos(), angle.sin()];
			let elem = if CONFIG.read().decorations.border.radius > 0.0 {
				PixelShaderElement::new(
					Self::get(renderer).rounded.clone(),
					geo,
					None,
					1.0,
					vec![
						Uniform::new("startColor", [0.880, 1.0, 1.0]),
						Uniform::new(
							"endColor",
							Some([0.580, 0.921, 0.921]).unwrap_or([0.880, 1.0, 1.0]),
						),
						Uniform::new("thickness", thickness),
						Uniform::new("halfThickness", thickness * 0.5),
						Uniform::new(
							"radius",
							CONFIG.read().decorations.border.radius as f32 + thickness + 2.0,
						),
						Uniform::new("gradientDirection", gradient_direction),
					],
				)
			} else {
				PixelShaderElement::new(
					Self::get(renderer).default.clone(),
					geo,
					None,
					1.0,
					vec![
						Uniform::new("startColor", [0.880, 1.0, 1.0]),
						Uniform::new(
							"endColor",
							Some([0.580, 0.921, 0.921]).unwrap_or([0.880, 1.0, 1.0]),
						),
						Uniform::new("thickness", thickness),
						Uniform::new("halfThickness", thickness * 0.5),
						Uniform::new("gradientDirection", gradient_direction),
					],
				)
			};
			elements.insert(window.clone(), elem.clone());
			elem
		}
	}
	pub fn cleanup(renderer: &mut GlowRenderer) {
		let elements = &mut renderer
			.egl_context()
			.user_data()
			.get::<BorderShaderElements>()
			.expect("Border Shader not initialized")
			.0
			.borrow_mut();
		elements.retain(|w, _| w.alive())
	}
}
