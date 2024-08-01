use std::error::Error;

use egui_tetra2::{egui, State, StateWrapper};
use tetra::{graphics::Color, Context};

const SCREEN_WIDTH: i32 = 800;
const SCREEN_HEIGHT: i32 = 600;

struct MainState {}

impl MainState {
	pub fn new(_ctx: &mut Context) -> Result<Self, Box<dyn Error>> {
		Ok(Self {})
	}
}

impl State<Box<dyn Error>> for MainState {
	fn ui(
		&mut self,
		_ctx: &mut tetra::Context,
		egui_ctx: &egui::Context,
	) -> Result<(), Box<dyn Error>> {
		egui_extras::install_image_loaders(egui_ctx);

		egui::Window::new("Image").show(egui_ctx, |ui| {
			ui.image(egui::include_image!("wabbit_alpha.png"));
			ui.label("label");
		});
		Ok(())
	}

	fn draw(&mut self, ctx: &mut Context, _egui_ctx: &egui::Context) -> Result<(), Box<dyn Error>> {
		tetra::graphics::clear(ctx, Color::BLACK);
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	tetra::ContextBuilder::new("Image example", SCREEN_WIDTH, SCREEN_HEIGHT)
		.show_mouse(true)
		.high_dpi(true)
		.build()?
		.run(|ctx| Ok(StateWrapper::new(MainState::new(ctx)?)))
}
