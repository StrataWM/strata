use crate::libs::parse_config::parse_config;
use once_cell::sync::Lazy;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AutostartCmd {
	pub cmd: String,
}

#[derive(Debug, Deserialize)]
pub struct Autostart {
	pub cmd: Vec<AutostartCmd>,
}

#[derive(Debug, Deserialize)]
pub struct General {
	pub workspaces: u8,
	pub in_gaps: i32,
	pub out_gaps: i32,
	pub kb_repeat: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct WindowDecorations {
	pub border_width: u32,
	pub border_active: String,
	pub border_inactive: String,
	pub border_radius: f64,
	pub window_opacity: f64,
	pub blur_enable: bool,
	pub blur_size: u32,
	pub blur_passes: u32,
	pub blur_optimization: bool,
	pub shadows_enabled: bool,
	pub shadow_size: u32,
	pub shadow_blur: u32,
	pub shadow_color: String,
}

#[derive(Debug, Deserialize)]
pub struct Tiling {
	pub layout: String,
}

#[derive(Debug, Deserialize)]
pub struct Animations {
	pub anim_enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct Workspace {
	pub workspace: i32,
	pub class_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Rules {
	pub workspace: Vec<Workspace>,
	pub floating: Vec<Floating>,
}

#[derive(Debug, Deserialize)]
pub struct Floating {
	pub class_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
	pub autostart: Autostart,
	pub general: General,
	pub window_decorations: WindowDecorations,
	pub tiling: Tiling,
	pub animations: Animations,
	pub rules: Rules,
}

pub static CONFIG: Lazy<Config> = Lazy::new(parse_config);
