use lazy_static::lazy_static;
use serde_derive::Deserialize;
use std::sync::Mutex;
lazy_static! {
	pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::default());
}

#[derive(Debug, Deserialize)]
pub struct AutostartCmd {
	pub cmd: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Autostart {
	pub cmd: Vec<AutostartCmd>,
}

#[derive(Debug, Default, Deserialize)]
pub struct General {
	pub workspaces: u8,
	pub in_gaps: i32,
	pub out_gaps: i32,
	pub kb_repeat: Vec<i32>,
}

#[derive(Debug, Default, Deserialize)]
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

#[derive(Debug, Default, Deserialize)]
pub struct Tiling {
	pub layout: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct Animations {
	pub anim_enabled: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Triggers {
	pub event: String,
	pub class_name: String,
	pub workspace: Option<i32>,
}

#[derive(Debug, Clone)]
pub struct Rules {
	pub triggers: Triggers,
	pub action: String,
}

#[derive(Debug, Clone)]
pub struct Keybinding {
	pub keys: Vec<String>,
	pub func: String,
}

#[derive(Debug, Default)]
pub struct Config {
	pub autostart: Autostart,
	pub general: General,
	pub window_decorations: WindowDecorations,
	pub tiling: Tiling,
	pub animations: Animations,
	pub rules: Vec<Rules>,
	pub bindings: Vec<Keybinding>,
}

unsafe impl Send for Config {}
