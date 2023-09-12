use mlua::{
	UserData,
	UserDataMethods,
};
use smithay::{
	desktop::{
		LayerSurface,
		PopupKind,
		Window,
	},
	output::Output,
	utils::{
		Logical,
		Rectangle,
	},
};
use std::{
	cell::RefCell,
	rc::Rc,
};

pub struct StrataWindow {
	pub smithay_window: Window,
	pub rec: Rectangle<i32, Logical>,
}

pub struct Workspace {
	pub windows: Vec<Rc<RefCell<StrataWindow>>>,
	pub outputs: Vec<Output>,
	pub layout_tree: Dwindle,
}

pub struct Workspaces {
	pub workspaces: Vec<Workspace>,
	pub current: u8,
}

#[derive(Clone)]
pub enum Dwindle {
	Empty,
	Window(Rc<RefCell<StrataWindow>>),
	Split { split: HorizontalOrVertical, ratio: f32, left: Box<Dwindle>, right: Box<Dwindle> },
}

#[derive(Clone, Copy, PartialEq)]
pub enum HorizontalOrVertical {
	Horizontal,
	Vertical,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FocusTarget {
	Window(Window),
	LayerSurface(LayerSurface),
	Popup(PopupKind),
}

impl UserData for StrataWindow {
	fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
		methods.add_method_mut("close", |_, this, _: ()| {
			Ok(this.smithay_window.toplevel().send_close())
		});
	}
}
