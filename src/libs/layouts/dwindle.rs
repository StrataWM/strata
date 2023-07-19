use crate::libs::structs::{
	Dwindle,
	HorizontalOrVertical,
	StrataWindow,
};
use smithay::desktop::Window;
use std::{
	cell::RefCell,
	fmt::{
		Debug,
		Result,
	},
	rc::Rc,
};

impl Debug for Dwindle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
		match self {
			Self::Empty => write!(f, "Empty"),
			Self::Window(w) => w.borrow().rec.fmt(f),
			Self::Split { split, ratio, left, right } => {
				f.debug_struct("Split")
					.field("split", split)
					.field("ratio", ratio)
					.field("left", left)
					.field("right", right)
					.finish()
			}
		}
	}
}

impl Dwindle {
	pub fn new() -> Self {
		Dwindle::Empty
	}

	pub fn insert(
		&mut self,
		window: Rc<RefCell<StrataWindow>>,
		splitnew: HorizontalOrVertical,
		ratio: f32,
	) {
		match self {
			Dwindle::Empty => {
				*self = Dwindle::Window(window);
			}
			Dwindle::Window(w) => {
				*self = Dwindle::Split {
					left: Box::new(Dwindle::Window(w.clone())),
					right: Box::new(Dwindle::Window(window)),
					split: splitnew,
					ratio: rationew,
				}
			}
			Dwindle::Split { split: _, ratio: _, left: _, right } => {
				right.insert(window, splitnew, rationew);
			}
		}
	}

	pub fn remove(&mut self, window: &Window) {
		match self {
			Dwindle::Empty => {}
			Dwindle::Window(w) => {
				if w.borrow().window == *window {
					*self = Dwindle::Empty;
				}
			}
			Dwindle::Split { left, right, split: _, ratio: _ } => {
				if let Dwindle::Window(w) = left.as_ref() {
					if w.borrow().window == *window {
						*self = *right.clone();
						return;
					}
				}
				if let Dwindle::Window(w) = right.as_ref() {
					if w.borrow().window == *window {
						*self = *left.clone();
						return;
					}
				}
				left.remove(window);
				right.remove(window);
			}
		}
	}

	pub fn next_split(&self) -> HorizontalOrVertical {
		match self {
			Dwindle::Empty => HorizontalOrVertical::Horizontal,
			Dwindle::Window(_w) => HorizontalOrVertical::Horizontal,
			Dwindle::Split { left: _, right, split, ratio: _ } => {
				if let Dwindle::Split { left: _, right: _, split: _, ratio: _ } = right.as_ref() {
					right.next_split()
				} else if *split == HorizontalOrVertical::Horizontal {
					HorizontalOrVertical::Vertical
				} else {
					HorizontalOrVertical::Horizontal
				}
			}
		}
	}
}

impl Default for Dwindle {
	fn default() -> Self {
		Self::new()
	}
}
