use std::{
	cell::RefCell,
	rc::Rc,
};

use crate::{
	libs::structs::workspaces::{
		Dwindle,
		HorizontalOrVertical,
		StrataWindow,
		Workspace,
	},
	CONFIG,
};
use smithay::{
	desktop::layer_map_for_output,
	utils::{
		Logical,
		Physical,
		Point,
		Rectangle,
		Size,
	},
};

pub fn refresh_geometry(workspace: &mut Workspace) {
	let gaps = {
		let options = &CONFIG.read().options;
		(options.general.gaps_out, options.general.gaps_in)
	};
	let output = layer_map_for_output(workspace.outputs().next().unwrap()).non_exclusive_zone();
	let output_full = workspace.outputs().next().unwrap().current_mode().unwrap().size;

	match &mut workspace.layout_tree {
		Dwindle::Empty => {}
		Dwindle::Window(w) => {
			w.borrow_mut().rec = Rectangle {
				loc: Point::from((gaps.0 + gaps.1 + output.loc.x, gaps.0 + gaps.1 + output.loc.y)),
				size: Size::from((
					output.size.w - ((gaps.0 + gaps.1) * 2),
					output.size.h - ((gaps.0 + gaps.1) * 2),
				)),
			};
		}
		Dwindle::Split { left, right, split, ratio } => {
			if let Dwindle::Window(w) = left.as_mut() {
				generate_layout(
					right.as_mut(),
					w,
					Rectangle {
						loc: Point::from((gaps.0 + output.loc.x, gaps.0 + output.loc.y)),
						size: Size::from((
							output.size.w - (gaps.0 * 2),
							output.size.h - (gaps.0 * 2),
						)),
					},
					*split,
					*ratio,
					Size::from((output_full.w - gaps.0, output_full.h - gaps.0)),
					gaps,
				)
			}
		}
	}
	for strata_window in workspace.strata_windows() {
		let xdg_toplevel = strata_window.window.toplevel();
		xdg_toplevel.with_pending_state(|state| {
			state.size = Some(strata_window.rec.size);
		});
		xdg_toplevel.send_configure();
	}
}

pub fn generate_layout(
	tree: &mut Dwindle,
	lastwin: &Rc<RefCell<StrataWindow>>,
	lastgeo: Rectangle<i32, Logical>,
	split: HorizontalOrVertical,
	ratio: f32,
	output: Size<i32, Physical>,
	gaps: (i32, i32),
) {
	let size = match split {
		HorizontalOrVertical::Horizontal => {
			Size::from(((lastgeo.size.w as f32 * ratio) as i32, lastgeo.size.h))
		}
		HorizontalOrVertical::Vertical => {
			Size::from((lastgeo.size.w, (lastgeo.size.h as f32 * ratio) as i32))
		}
	};

	let loc: Point<i32, Logical> = match split {
		HorizontalOrVertical::Horizontal => Point::from((lastgeo.loc.x, output.h - size.h)),
		HorizontalOrVertical::Vertical => Point::from((output.w - size.w, lastgeo.loc.y)),
	};

	let rec_with_gaps = Rectangle {
		size: Size::from((size.w - (gaps.1 * 2), (size.h - (gaps.1 * 2)))),
		loc: Point::from((loc.x + gaps.1, loc.y + gaps.1)),
	};

	lastwin.borrow_mut().rec = rec_with_gaps;

	let loc = match split {
		HorizontalOrVertical::Horizontal => Point::from((output.w - size.w, lastgeo.loc.y)),
		HorizontalOrVertical::Vertical => Point::from((lastgeo.loc.x, output.h - size.h)),
	};

	let rec = Rectangle { size, loc };
	let rec_with_gaps = Rectangle {
		size: Size::from((size.w - (gaps.1 * 2), (size.h - (gaps.1 * 2)))),
		loc: Point::from((loc.x + gaps.1, loc.y + gaps.1)),
	};
	match tree {
		Dwindle::Empty => {}
		Dwindle::Window(w) => w.borrow_mut().rec = rec_with_gaps,
		Dwindle::Split { split, ratio, left, right } => {
			if let Dwindle::Window(w) = left.as_mut() {
				w.borrow_mut().rec = rec;
				generate_layout(right.as_mut(), w, rec, *split, *ratio, output, gaps)
			}
		}
	}
}
