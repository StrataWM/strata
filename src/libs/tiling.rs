use crate::{
	libs::structs::{
		CompWorkspace,
		Dwindle,
		HorizontalOrVertical,
		StrataWindow,
		Workspace,
	},
	CONFIG,
};
use log::{
	debug,
	info,
};
use smithay::{
	desktop::layer_map_for_output,
	reexports::wayland_protocols::xdg::shell::server::xdg_toplevel,
	utils::{
		Logical,
		Physical,
		Point,
		Rectangle,
		Size,
	},
};
use std::{
	cell::RefCell,
	rc::Rc,
};

pub fn refresh_geometry(workspace: &mut CompWorkspace) {
	let config = CONFIG.lock().unwrap();
	let gaps = (config.general.win_gaps, config.general.out_gaps);

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
		Dwindle::Split { split, ratio, left, right } => {
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
	debug!("{:#?}", workspace.layout_tree);

	for strata_window in workspace.strata_windows() {
		let xdg_toplevel = strata_window.window.toplevel();
		xdg_toplevel.with_pending_state(|state| {
			state.size = Some(strata_window.rec.size);
		});
		xdg_toplevel.send_configure();
	}
}
