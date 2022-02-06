# Notes

## TODO

### Sprite

```rust
trait RectAnimation {

}
trait ImageAnimation {

}
trait Sprite {
    pub fn update(&mut self);
    pub fn rectangleAnimation() -> &dny RectAnimation;
    pub fn imageAnimation() -> &dny SpriteAnimation;
    pub fn rectangle(&self) -> Rect;
    pub fn image(&self) -> &dyn Sprite
}
```

Animation implementations:

- `HypotrochoidAnimation{ center, a, b, c }`
- `HSVColorAnimation`

### OpenCV Blending

### Draw methods

- rect, line, polygon, ellipse
- antialiased draw methods

### Surface

- Subsurface

### Lode and store image

- load png, jpg, bmp
- store png, jpg, bmp
- load and render svg
- load gif to surface vector
- store frames to gif
