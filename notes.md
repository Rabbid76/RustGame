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

- store png, jpg, bmp
- load and render svg
- load gif to surface vector
- store frames to gif
- emoji to surface

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
