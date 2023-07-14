use crate::libs::structs::Strata;

impl Strata {
	pub fn refresh_geometry(&mut self) {
		let space = &mut self.space;

		space.refresh();
		let output = space.outputs().next().cloned().unwrap();
		let output_geometry = space.output_geometry(&output).unwrap();
		let output_width = output_geometry.size.w;
		let output_height = output_geometry.size.h;
		let gap = 6;
		let elements_count = space.elements().count() as i32;

		let mut resizes = vec![];

		for (i, window) in space.elements().enumerate() {
			let (mut x, mut y) = (gap, gap);
			let (mut width, mut height) = (output_width - gap * 2, output_height - gap * 2);

			if elements_count > 1 {
				width -= gap;
				width /= 2;
			}

			if i > 0 {
				height /= elements_count - 1;
				x += width + gap;
				y += height * (i as i32 - 1);
			}

			if i > 1 {
				height -= gap;
				y += gap;
			}
			resizes.push((window.clone(), (width, height), (x, y)));
		}

		for (window, dimensions, position) in resizes {
			window.toplevel().with_pending_state(|state| {
				state.size = Some(dimensions.into());
			});
			window.toplevel().send_configure();

			space.map_element(window, position, false);
		}
	}
}
