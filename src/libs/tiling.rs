use crate::libs::structs::Strata;

impl Strata {
	pub fn refresh_geometry(&mut self) {
		let space = &mut self.space;

		// Remove dead elements
		space.refresh();

		// Get the first output available.
		let output = space.outputs().next().cloned().unwrap();

		// Find the size of the output.
		let output_geometry = space.output_geometry(&output).unwrap();
		let output_width = output_geometry.size.w;
		let output_height = output_geometry.size.h;

		// The gap between windows in px.
		let gap = 6;

		// The total number of windows.
		let elements_count = space.elements().count() as i32;

		// A vec to store the windows and their new sizes. This is used because space is a mutable
		// reference. The for loop will take ownership, meaning space.map_element can't be called
		// until the loop is finished.
		let mut resizes = vec![];

		for (i, window) in space.elements().enumerate() {
			// Move the window to start at the gap size creating a gap around the window.
			let (mut x, mut y) = (gap, gap);
			// The width/height should be subtracted from twice the gap size, since there are gaps
			// on both sides of the window.
			let (mut width, mut height) = (output_width - gap * 2, output_height - gap * 2);

			// If there is more than one window, subtract an additional gap from the width and
			// divide the width in two giving room for another window.
			if elements_count > 1 {
				width -= gap;
				width /= 2;
			}

			// Size the windows on the stack (the non-master windows).
			if i > 0 {
				// Get the height on the stack by dividing the height by the total number of
				// elements on the stack.
				height /= elements_count - 1;

				// Offset the x value by the width and gap.
				x += width + gap;
				// Offset the y value by the total number of windows above on the stack.
				y += height * (i as i32 - 1);
			}

			// Make all the windows on the stack, after the first one, have a gap on the top.
			if i > 1 {
				height -= gap;
				// By adding the gap to y, the window is pushed down, causing the gap.
				y += gap;
			}

			resizes.push((window.clone(), (width, height), (x, y)));
		}

		// Loop through the resizes vec and update the window state.
		for (window, dimensions, position) in resizes {
			// Resize the window to a suggested size. The client may not resize to this exact size,
			// for example a terminal emulator might resize to the closest size based on monospaced
			// rows and columns.
			window.toplevel().with_pending_state(|state| {
				state.size = Some(dimensions.into());
			});
			// Send a xdg_toplevel::configure event because of the state change.
			window.toplevel().send_configure();

			// Move window to new position.
			space.map_element(window, position, false);
		}
	}
}
