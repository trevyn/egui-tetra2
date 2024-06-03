## v0.4.1 - June 3, 2024

- Fix README.md on crates.io

## v0.4.0 - June 3, 2024

- Fork to `egui-tetra2` due to original repository being archived.
- Update `egui` to 0.27
- Update `tetra` to 0.8

## v0.3.0 - January 7, 2022

- Update `egui` to 0.16.1
- Add support for zooming
- Increase scroll speed
- Add FPS plot example

## v0.2.1 - September 18, 2021

- Fix egui not receiving screen size information

## v0.2.0 - August 26, 2021

### Improvements:

- Fix UI not rendering correctly when using fixed timestep
  with vsync disabled
- Implement `Default` for `EguiWrapper`
- Update `egui` to 0.14.0
- Update `open` to 2.0.1

### Breaking changes

- Remove `OpenError` and change `Error::OpenError` to contain
  an `std::io::Error`
- `EguiWrapper::end_frame` now takes a `&mut tetra::Context`
  argument
- `EguiWrapper::draw_frame` no longer returns a `Result`

## v0.1.0 - June 26, 2021

First official release
