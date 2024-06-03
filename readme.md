# egui-tetra2

This is an up-to-date and maintained fork of `tesselode/egui-tetra`, which has been archived. Issues and PRs welcome.

#### [crates.io](https://crates.io/crates/egui-tetra2) | [docs](https://docs.rs/egui-tetra2)

egui-tetra2 is a library that helps integrate [egui](https://crates.io/crates/egui),
an immediate mode GUI library, with [Tetra](https://crates.io/crates/tetra),
a 2D game framework.

## Basic example

```rust
use std::error::Error;

struct MainState;

impl egui_tetra2::State<Box<dyn Error>> for MainState {
	fn ui(
		&mut self,
		_ctx: &mut tetra::Context,
		egui_ctx: &egui::Context,
	) -> Result<(), Box<dyn Error>> {
		egui::CentralPanel::default().show(egui_ctx, |_ui| {
			egui::Window::new("hi!").show(egui_ctx, |ui| {
				ui.label("Hello world!");
			});
		});
		Ok(())
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	tetra::ContextBuilder::new("example", 800, 600)
		.show_mouse(true)
		.build()?
		.run(|_| Ok(egui_tetra2::StateWrapper::new(MainState)))
}
```

## License

This project is licensed under either of

- [Apache License, Version 2.0](https://github.com/trevyn/egui-tetra2/blob/main/LICENSE-Apache)
- [MIT license](https://github.com/trevyn/egui-tetra2/blob/main/LICENSE-MIT)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
time by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
