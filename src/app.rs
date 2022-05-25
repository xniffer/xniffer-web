/// We derive Deserialize/Serialize so we can persist app state on shutdown.
pub struct XnifferApp {}

impl Default for XnifferApp {
	fn default() -> Self {
		Self {}
	}
}

impl XnifferApp {
	/// Called once before the first frame.
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customized the look at feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		Default::default()
	}
}

impl eframe::App for XnifferApp {
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		egui::SidePanel::left("top_panel").show(ctx, |ui| {
			// The top panel is often a good place for a menu bar:
			egui::menu::bar(ui, |ui| {
				ui.menu_button("File", |ui| {
					if ui.button("Quit").clicked() {
						frame.quit();
					}
				});
			});
		});

		egui::CentralPanel::default().show(ctx, |ui| {
			// The central panel the region left after adding TopPanel's and SidePanel's

			ui.heading("eframe template");
			ui.hyperlink("https://github.com/emilk/eframe_template");
			ui.add(egui::github_link_file!(
				"https://github.com/emilk/eframe_template/blob/master/",
				"Source code."
			));
			egui::warn_if_debug_build(ui);
		});

		if false {
			egui::Window::new("Window").show(ctx, |ui| {
				ui.label("Windows can be moved by dragging them.");
				ui.label("They are automatically sized based on contents.");
				ui.label("You can turn on resizing and scrolling if you like.");
				ui.label("You would normally chose either panels OR windows.");
			});
		}
	}
}
