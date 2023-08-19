use lazy_static::lazy_static;
use parking_lot::RwLock;
use serde::Deserialize;
use std::sync::Arc;

lazy_static! {
	pub static ref CONFIG: Arc<Config> = Arc::new(Config::default());
}

#[derive(Debug, Default, Deserialize)]
pub struct General {
	pub workspaces: u8,
	pub gaps_in: i32,
	pub gaps_out: i32,
	pub kb_repeat: Vec<i32>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct WindowDecorations {
	pub border: Border,
	pub window: Window,
	pub blur: Blur,
	pub shadow: Shadow,
}

#[derive(Debug, Default, Deserialize)]
pub struct Border {
	pub width: u32,
	pub active: String,
	pub inactive: String,
	pub radius: f64,
}

#[derive(Debug, Default, Deserialize)]
pub struct Window {
	pub opacity: f64,
}

#[derive(Debug, Default, Deserialize)]
pub struct Blur {
	pub enable: bool,
	pub size: u32,
	pub passes: u32,
	pub optimize: bool,
}

#[derive(Debug, Default, Deserialize)]
pub struct Shadow {
	pub enable: bool,
	pub size: u32,
	pub blur: u32,
	pub color: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct Tiling {
	pub layout: String,
}

#[derive(Debug, Default, Deserialize)]
pub struct Animations {
	pub enable: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Triggers {
	pub event: String,
	pub class_name: String,
	pub workspace: Option<i32>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Rules {
	pub triggers: Triggers,
	pub action: String, // FIXME
}

#[derive(Debug, Clone, Deserialize)]
pub struct Keybinding {
	pub keys: Vec<String>,
	pub action: String, // FIXME
}

pub type Cmd = Vec<String>;

#[derive(Debug, Default, Deserialize)]
pub struct Options {
	pub autostart: Vec<Cmd>,
	pub general: General,
	pub decorations: WindowDecorations,
	pub tiling: Tiling,
	pub animations: Animations,
}

#[derive(Debug, Default)]
pub struct Config {
	pub options: RwLock<Options>,
	// pub rules: RwLock<Vec<Rules>>,
	// pub bindings: RwLock<Vec<Keybinding>>,
}
