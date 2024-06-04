use egui_tetra2::{egui, State, StateWrapper};
use std::error::Error;
use tetra::graphics::{self, DrawParams, Rectangle, Shader};
use tetra::{Context, ContextBuilder};

struct GameState {
	shader_string: String,
	shader_result: String,
	shader: Shader,
	timer: f32,
}

impl GameState {
	fn new(ctx: &mut Context) -> tetra::Result<GameState> {
		let shader_string =
			"#version 150\n\nuniform float iTime;\nout vec4 o;\n\nvoid main() {\n\to = vec4(sin(gl_FragCoord.x/40+iTime*10), 0, sin(gl_FragCoord.y/20+sin(iTime*20)), 1.0);\n}"
				.to_string();
		let shader = Shader::from_fragment_string(ctx, &shader_string)?;

		Ok(GameState {
			shader_string,
			shader_result: "Shader compiled OK".to_string(),
			shader,
			timer: 0.0,
		})
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
		ctx: &mut tetra::Context,
		egui_ctx: &egui::Context,
	) -> Result<(), Box<dyn Error>> {
		egui::Window::new("Custom Shader").show(egui_ctx, |ui| {
			let mut theme = egui_extras::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
			ui.collapsing("Theme", |ui| {
				ui.group(|ui| {
					theme.ui(ui);
					theme.clone().store_in_memory(ui.ctx());
				});
			});

			ui.label(&self.shader_result);

			let mut layouter = |ui: &egui::Ui, string: &str, wrap_width: f32| {
				let mut layout_job =
					egui_extras::syntax_highlighting::highlight(ui.ctx(), &theme, string, "c");
				layout_job.wrap.max_width = wrap_width;
				ui.fonts(|f| f.layout_job(layout_job))
			};

			egui::ScrollArea::vertical().show(ui, |ui| {
				if ui
					.add(
						egui::TextEdit::multiline(&mut self.shader_string)
							.code_editor()
							.desired_rows(10)
							.desired_width(f32::INFINITY)
							.layouter(&mut layouter),
					)
					.changed()
				{
					self.shader_result =
						match Shader::from_fragment_string(ctx, &self.shader_string) {
							Ok(shader) => {
								self.shader = shader;
								"Shader compiled OK".to_string()
							}
							Err(err) => format!("{}", err),
						}
				};
			});
		});
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context, _egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
		graphics::set_shader(ctx, &self.shader);

		self.shader.set_uniform(ctx, "iTime", self.timer / 60.0);

		let window_size = tetra::window::get_size(ctx);

		graphics::mesh::Mesh::rectangle(
			ctx,
			graphics::mesh::ShapeStyle::Fill,
			Rectangle::new(0.0, 0.0, window_size.0 as f32, window_size.1 as f32),
		)
		.unwrap()
		.draw(ctx, DrawParams::new());

		graphics::reset_shader(ctx);

		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	ContextBuilder::new("Custom Shader", 1280, 720)
		.high_dpi(true)
		.show_mouse(true)
		.resizable(true)
		.build()?
		.run(|ctx| Ok(StateWrapper::new(GameState::new(ctx)?)))
}
