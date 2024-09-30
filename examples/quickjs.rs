use egui_tetra2::{egui, State, StateWrapper};
use std::error::Error;
use tetra::graphics::{self, DrawParams, Rectangle};
use tetra::{Context, ContextBuilder};

struct GameState {
	js_ctx: rquickjs::Context,
	js_string: String,
	js_result: String,
	timer: f32,
}

impl GameState {
	fn new() -> GameState {
		let rt = rquickjs::Runtime::new().unwrap();
		let js_ctx = rquickjs::Context::full(&rt).unwrap();

		let js_string = "function getCirclePosition(t) {
    // Define parameters for the movement
    const centerX = 900; // X coordinate of the center of the circle
    const centerY = 300; // Y coordinate of the center of the circle
    const baseRadius = 100; // Base radius for the circular movement
    const speed = 5; // Speed of rotation
    const spiralFactor = 0.05; // Factor to increase radius over time

    // Calculate the radius at time t
    const radius = baseRadius + spiralFactor * t;

    // Calculate the angle based on time
    const angle = speed * t;

    // Calculate the position (x, y) using sine and cosine
    const x = centerX + radius * Math.cos(angle);
    const y = centerY + radius * Math.sin(angle);

    return { x, y };
}

getCirclePosition(t)"
			.to_string();

		GameState { js_ctx, js_string, js_result: "".to_string(), timer: 0.0 }
	}
}

impl State<Box<dyn Error>> for GameState {
	fn update(
		&mut self,
		_ctx: &mut Context,
		_egui_ctx: &egui::Context,
	) -> Result<(), Box<dyn Error>> {
		self.timer += 1.0;
		Ok(())
	}

	fn ui(
		&mut self,
		_ctx: &mut tetra::Context,
		egui_ctx: &egui::Context,
	) -> Result<(), Box<dyn Error>> {
		egui::Window::new("QuickJS Example").show(egui_ctx, |ui| {
			// puffin_egui::show_viewport_if_enabled(egui_ctx);

			let mut theme =
				egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx(), ui.style());
			ui.collapsing("Theme", |ui| {
				ui.group(|ui| {
					theme.ui(ui);
					theme.clone().store_in_memory(ui.ctx());
				});
			});

			ui.label(&self.js_result);

			let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
				let mut layout_job = egui_extras::syntax_highlighting::highlight(
					ui.ctx(),
					ui.style(),
					&theme,
					string,
					"js",
				);
				layout_job.wrap.max_width = wrap_width;
				ui.fonts(|f| f.layout_job(layout_job))
			};

			egui::ScrollArea::vertical().show(ui, |ui| {
				ui.add(
					egui::TextEdit::multiline(&mut self.js_string)
						.code_editor()
						.desired_rows(10)
						.desired_width(f32::INFINITY)
						.layouter(&mut layouter),
				)
			});
		});
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context, _egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
		// puffin::profile_function!();
		// puffin::GlobalProfiler::lock().new_frame();

		tetra::graphics::clear(ctx, tetra::graphics::Color::BLACK);

		self.js_ctx.with(|js_ctx| {
			let global = js_ctx.globals();
			global.set("t", self.timer / 60.0).unwrap();

			self.js_result = match js_ctx.eval::<rquickjs::Value, _>(self.js_string.as_bytes()) {
				Ok(result) => {
					let result = result.as_object();
					if let Some(result) = result {
						graphics::mesh::Mesh::rectangle(
							ctx,
							graphics::mesh::ShapeStyle::Fill,
							Rectangle::new(
								result.get("x").unwrap_or(0.0) as f32,
								result.get("y").unwrap_or(0.0) as f32,
								100.0,
								100.0,
							),
						)
						.unwrap()
						.draw(ctx, DrawParams::new());
					}

					"JS eval'd OK".to_string()
				}
				Err(err) => format!("{}", err),
			};
		});

		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	// puffin::set_scopes_on(true);

	ContextBuilder::new("QuickJS Example", 1280, 720)
		.high_dpi(true)
		.show_mouse(true)
		.resizable(true)
		.build()?
		.run(|_| Ok(StateWrapper::new(GameState::new())))
}
