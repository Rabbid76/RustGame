# Notes

## TODO

Generate screenshots with `image.store`.

### Sprite

Possible `Sprite` trait interfaces:

- `fn angle(&self) -> f32`
- `fn circle(&'a self) -> Option<&'a Circle>`
- `fn radius(&self) -> f32`

### OpenCV Blending

### Surface

- Move Surface implementation from sdl2_rust_game to rust_game (raw(): &Vec[u8], raw_mut(): &mut [u8]), SurfaceProxy in sdl2_rust_game for `blit`  
- Subsurface

### Lode and store image

- move image implementation from sdl2_rust_game to rust_game
- emoji to surface
- C++ Image + SVG Reader, link with Rust, test with Windows and Mac. Deploy Crate?
- capture surface object

### Mask

### Events

- iterator instead of vec
- keyboard
- timer

### Key module

create key codes with macros from ascii characters

### Mouse module

### Textures

- checker texture

### Rectangle module

- rename `Rect` -> `Rectangle`

### Circle module

New circle module similar the rectangle module.

### Canvas

- `blit_background(surf, mode)`, mode: center, stretched, fit, tiled

### Math module

- line intersection
- point in triangle
- point in convex polygon
