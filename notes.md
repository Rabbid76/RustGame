# Notes

## TODO

Generate screenshots with `image.store`.

### Sprite

Possible `Sprite` trait interfaces:

- `fn angle(&self) -> f32`
- `fn circle(&'a self) -> Option<&'a Circle>`
- `fn radius(&self) -> f32`

### OpenCV Blending

### Draw methods

- rect, line, polygon, ellipse

### Surface

- Subsurface

### Lode and store image

- emoji to surface
- C++ Image + SVG Reader, link with Rust, test with Windows and Mac. Deploy Crate?
- capture surface object

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
