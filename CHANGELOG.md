# Changelog

## [0.2.1] - Unreleased

### Changed

- Use linear interpolation for text sampling.

### Fixed

- Fixed text shader for light text.

## [0.2.0] - 05.12.2020

### Added

- Added transparency blending for shapes and textures.
- Added `Send` and `Sync` traits to Handle.
- Added a `Duku` creation convenience function.
- Added `window` to default features.
- Added more conversions to `VectorN`.
- Added color gradients.
- Added per-mesh tinting.

### Changed

- Changed color API.
- Changed drawing API.
- Made autosized `Camera` the default.
- Renamed `Framebuffer` to `Canvas`.
- Renamed drawing functions.
- Renamed math structs.
- Added `Option` to some creation functions.

### Removed

- Removed `Duku::create_texture_color` function.
- Removed `Transform` struct.

### Fixed

- A lot of bugs were fixed.
