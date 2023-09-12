use crate::{
	libs::{
		decorations::AsGlowRenderer,
		structs::{
			state::BorderShader,
			workspaces::{
				Dwindle,
				StrataWindow,
				Workspace,
				Workspaces,
			},
		},
		tiling::refresh_geometry,
	},
	CONFIG,
};
use smithay::{
	backend::renderer::{
		element::{
			surface::WaylandSurfaceRenderElement,
			AsRenderElements,
		},
		gles::element::PixelShaderElement,
		ImportAll,
		Renderer,
		Texture,
	},
	desktop::{
		space::SpaceElement,
		Window,
	},
	output::Output,
	utils::{
		Logical,
		Point,
		Rectangle,
		Scale,
		Transform,
	},
};
use std::{
	cell::{
		Ref,
		RefCell,
	},
	rc::Rc,
};

impl StrataWindow {
	fn bbox(&self) -> Rectangle<i32, Logical> {
		let mut bbox = self.smithay_window.bbox();
		bbox.loc += self.rec.loc - self.smithay_window.geometry().loc;
		bbox
	}

	fn render_location(&self) -> Point<i32, Logical> {
		self.rec.loc - self.smithay_window.geometry().loc
	}
}
impl Workspace {
	pub fn new() -> Self {
		Workspace { windows: Vec::new(), outputs: Vec::new(), layout_tree: Dwindle::new() }
	}

	pub fn windows(&self) -> impl Iterator<Item = Ref<'_, Window>> {
		self.windows.iter().map(|w| Ref::map(w.borrow(), |hw| &hw.smithay_window))
	}

	pub fn strata_windows(&self) -> impl Iterator<Item = Ref<'_, StrataWindow>> {
		self.windows.iter().map(|w| Ref::map(w.borrow(), |hw| hw))
	}

	pub fn add_window(&mut self, window: Rc<RefCell<StrataWindow>>) {
		self.windows.retain(|w| w.borrow().smithay_window != window.borrow().smithay_window);
		self.windows.push(window.clone());
		self.layout_tree.insert(window, self.layout_tree.next_split(), 0.5);
		refresh_geometry(self);
	}

	pub fn remove_window(&mut self, window: &Window) -> Option<Rc<RefCell<StrataWindow>>> {
		let mut removed = None;
		self.windows.retain(|w| {
			if &w.borrow().smithay_window == window {
				removed = Some(w.clone());
				false
			} else {
				true
			}
		});
		self.layout_tree.remove(window);
		refresh_geometry(self);
		removed
	}

	pub fn render_elements<
		R: Renderer + ImportAll + AsGlowRenderer,
		C: From<WaylandSurfaceRenderElement<R>> + From<PixelShaderElement>,
	>(
		&self,
		renderer: &mut R,
	) -> Vec<C>
	where
		<R as Renderer>::TextureId: Texture + 'static,
	{
		let mut render_elements: Vec<C> = Vec::new();
		for element in &self.windows {
			let window = &element.borrow().smithay_window;
			if CONFIG.read().decorations.border.width > 0 {
				render_elements.push(C::from(BorderShader::element(
					renderer.glow_renderer_mut(),
					window,
					element.borrow().rec.loc,
				)));
			}
			render_elements.append(&mut window.render_elements(
				renderer,
				element.borrow().render_location().to_physical(1),
				Scale::from(1.0),
				1.0,
			));
		}
		render_elements
	}

	pub fn outputs(&self) -> impl Iterator<Item = &Output> {
		self.outputs.iter()
	}

	pub fn add_output(&mut self, output: Output) {
		self.outputs.push(output);
	}

	pub fn remove_output(&mut self, output: &Output) {
		self.outputs.retain(|o| o != output);
	}

	pub fn output_geometry(&self, o: &Output) -> Option<Rectangle<i32, Logical>> {
		if !self.outputs.contains(o) {
			return None;
		}

		let transform: Transform = o.current_transform();
		o.current_mode().map(|mode| {
			Rectangle::from_loc_and_size(
				(0, 0),
				transform
					.transform_size(mode.size)
					.to_f64()
					.to_logical(o.current_scale().fractional_scale())
					.to_i32_ceil(),
			)
		})
	}

	pub fn window_under<P: Into<Point<f64, Logical>>>(
		&self,
		point: P,
	) -> Option<(Ref<'_, Window>, Point<i32, Logical>)> {
		let point = point.into();
		self.windows.iter().filter(|e| e.borrow().bbox().to_f64().contains(point)).find_map(|e| {
			// we need to offset the point to the location where the surface is actually drawn
			let render_location = e.borrow().render_location();
			if e.borrow().smithay_window.is_in_input_region(&(point - render_location.to_f64())) {
				Some((Ref::map(e.borrow(), |hw| &hw.smithay_window), render_location))
			} else {
				None
			}
		})
	}

	pub fn contains_window(&self, window: &Window) -> bool {
		self.windows.iter().any(|w| &w.borrow().smithay_window == window)
	}
}

impl Default for Workspace {
	fn default() -> Self {
		Self::new()
	}
}

impl Workspaces {
	pub fn new(workspaceamount: u8) -> Self {
		Workspaces {
			workspaces: (0..workspaceamount).map(|_| Workspace::new()).collect(),
			current: 0,
		}
	}

	pub fn outputs(&self) -> impl Iterator<Item = &Output> {
		self.workspaces.iter().flat_map(|w| w.outputs())
	}

	pub fn iter(&mut self) -> impl Iterator<Item = &mut Workspace> {
		self.workspaces.iter_mut()
	}

	pub fn current_mut(&mut self) -> &mut Workspace {
		&mut self.workspaces[self.current as usize]
	}

	pub fn current(&self) -> &Workspace {
		&self.workspaces[self.current as usize]
	}

	pub fn all_windows(&self) -> impl Iterator<Item = Ref<'_, Window>> {
		self.workspaces.iter().flat_map(|w| w.windows())
	}

	pub fn workspace_from_window(&mut self, window: &Window) -> Option<&mut Workspace> {
		self.workspaces.iter_mut().find(|w| w.contains_window(window))
	}

	pub fn activate(&mut self, id: u8) {
		self.current = id;
	}
	pub fn move_window_to_workspace(&mut self, window: &Window, workspace: u8) {
		let mut removed = None;
		if let Some(ws) = self.workspace_from_window(window) {
			removed = ws.remove_window(window);
			refresh_geometry(ws)
		}
		if let Some(removed) = removed {
			self.workspaces[workspace as usize].add_window(removed);
			refresh_geometry(&mut self.workspaces[workspace as usize])
		}
	}
}
