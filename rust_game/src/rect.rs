pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect {
            x,
            y,
            w,
            h,
        }
    }

    pub fn get_width(&self) -> i32 {
        self.w
    }

    pub fn set_width(&mut self, width: i32) {
        self.w = width;
    }

    pub fn get_height(&self) -> i32 {
        self.h
    }

    pub fn set_height(&mut self, height: i32) {
        self.h = height;
    }

    pub fn get_left(&self) -> i32 {
        self.x
    }

    pub fn set_left(&mut self, left: i32) {
        self.x = left;
    }

    pub fn get_right(&self) -> i32 {
        self.x + self.w
    }

    pub fn set_right(&mut self, right: i32) {
        self.x = right - self.w;
    }

    pub fn get_top(&self) -> i32 {
        self.y
    }

    pub fn set_top(&mut self, top: i32) {
        self.y = top;
    }

    pub fn get_bottom(&self) -> i32 {
        self.y + self.h
    }

    pub fn set_bottom(&mut self, bottom: i32) {
        self.y = bottom - self.h
    }

    pub fn get_center_x(&self) -> i32 {
        self.x + self.w / 2
    }

    pub fn set_center_x(&mut self, center_x: i32) {
        self.x = center_x - self.w / 2;
    }

    pub fn get_center_y(&self) -> i32 {
        self.y + self.h / 2
    }

    pub fn set_center_y(&mut self, center_y: i32) {
        self.y = center_y - self.h / 2
    }

    pub fn get_size(&self) -> (i32, i32) {
        (self.w, self.h)
    }

    pub fn set_size(&mut self, size: (i32, i32)) {
        self.w = size.0;
        self.h = size.1;
    }

    pub fn get_top_left(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_top_left(&mut self, top_left: (i32, i32)) {
        self.x = top_left.0;
        self.y = top_left.1;
    }

    pub fn get_top_right(&self) -> (i32, i32) {
        (self.x + self.w, self.y)
    }

    pub fn set_top_right(&mut self, top_right: (i32, i32)) {
        self.x = top_right.0 - self.w;
        self.y = top_right.1;
    }

    pub fn get_bottom_left(&self) -> (i32, i32) {
        (self.x, self.y + self.h)
    }

    pub fn set_bottom_left(&mut self, bottom_left: (i32, i32)) {
        self.x = bottom_left.0;
        self.y = bottom_left.1 - self.h;
    }

    pub fn get_bottom_right(&self) -> (i32, i32) {
        (self.x + self.w, self.y + self.h)
    }

    pub fn set_bottom_right(&mut self, bottom_right: (i32, i32)) {
        self.x = bottom_right.0 - self.w;
        self.y = bottom_right.1 - self.h;
    }

    pub fn get_center(&self) -> (i32, i32) {
        (self.x + self.w / 2, self.y + self.h / 2)
    }

    pub fn set_center(&mut self, center: (i32, i32)) {
        self.x = center.0 - self.w / 2;
        self.y = center.1 - self.h / 2;
    }

    pub fn get_mid_top(&self) -> (i32, i32) {
        (self.x + self.w / 2, self.y)
    }

    pub fn set_mid_top(&mut self, mid_top: (i32, i32)) {
        self.x = mid_top.0 - self.w / 2;
        self.y = mid_top.1;
    }

    pub fn get_mid_left(&self) -> (i32, i32) {
        (self.x, self.y + self.h / 2)
    }

    pub fn set_mid_left(&mut self, mid_left: (i32, i32)) {
        self.x = mid_left.0;
        self.y = mid_left.1 - self.h / 2;
    }

    pub fn get_mid_bottom(&self) -> (i32, i32) {
        (self.x + self.w / 2, self.y + self.h)
    }

    pub fn set_mid_bottom(&mut self, mid_bottom: (i32, i32)) {
        self.x = mid_bottom.0 - self.w / 2;
        self.y = mid_bottom.1 - self.h;
    }

    pub fn get_mid_right(&self) -> (i32, i32) {
        (self.x + self.w, self.y + self.h / 2)
    }

    pub fn set_mid_right(&mut self, mid_right: (i32, i32)) {
        self.x = mid_right.0 - self.w;
        self.y = mid_right.1 - self.h / 2;
    }
}

#[cfg(test)]
mod rect_test {
    use super::*;

    #[test]
    fn new_test() {
        let rect = Rect::new(1, 2, 3, 4);
        assert!(rect.x == 1);
        assert!(rect.y == 2);
        assert!(rect.w == 3);
        assert!(rect.h == 4);
    }

    #[test]
    fn get_width_test() {
        let rect = Rect::new(4, 5, 6, 7);
        assert!(rect.get_width() == 6);
    }

    #[test]
    fn set_width_test() {
        let mut rect = Rect::new(4, 5, 6, 7);
        rect.set_width(1);
        assert!(rect.x == 4);
        assert!(rect.y == 5);
        assert!(rect.w == 1);
        assert!(rect.h == 7);
    }

    #[test]
    fn get_height_test() {
        let rect = Rect::new(4, 5, 6, 7);
        assert!(rect.get_height() == 7);
    }

    #[test]
    fn set_height_test() {
        let mut rect = Rect::new(4, 5, 6, 7);
        rect.set_height(1);
        assert!(rect.x == 4);
        assert!(rect.y == 5);
        assert!(rect.w == 6);
        assert!(rect.h == 1);
    }

    #[test]
    fn get_left_test() {
        let rect = Rect::new(4, 5, 6, 7);
        assert!(rect.get_left() == 4);
    }

    #[test]
    fn set_left_test() {
        let mut rect = Rect::new(4, 5, 6, 7);
        rect.set_left(1);
        assert!(rect.x == 1);
        assert!(rect.y == 5);
        assert!(rect.w == 6);
        assert!(rect.h == 7);
    }

    #[test]
    fn get_right_test() {
        let rect = Rect::new(4, 5, 6, 7);
        assert!(rect.get_right() == 10);
    }

    #[test]
    fn set_right_test() {
        let mut rect = Rect::new(4, 5, 6, 7);
        rect.set_right(1);
        assert!(rect.x == -5);
        assert!(rect.y == 5);
        assert!(rect.w == 6);
        assert!(rect.h == 7);
    }

    #[test]
    fn get_top_test() {
        let rect = Rect::new(4, 5, 6, 7);
        assert!(rect.get_top() == 5);
    }

    #[test]
    fn set_top_test() {
        let mut rect = Rect::new(4, 5, 6, 7);
        rect.set_top(1);
        assert!(rect.x == 4);
        assert!(rect.y == 1);
        assert!(rect.w == 6);
        assert!(rect.h == 7);
    }

    #[test]
    fn get_bottom_test() {
        let rect = Rect::new(4, 5, 6, 7);
        assert!(rect.get_bottom() == 12);
    }

    #[test]
    fn set_bottom_test() {
        let mut rect = Rect::new(4, 5, 6, 7);
        rect.set_bottom(1);
        assert!(rect.x == 4);
        assert!(rect.y == -6);
        assert!(rect.w == 6);
        assert!(rect.h == 7);
    }

    #[test]
    fn get_center_x_test() {
        let rect = Rect::new(4, 5, 6, 7);
        assert!(rect.get_center_x() == 7);
    }

    #[test]
    fn set_center_x_test() {
        let mut rect = Rect::new(4, 5, 6, 7);
        rect.set_center_x(1);
        assert!(rect.x == -2);
        assert!(rect.y == 5);
        assert!(rect.w == 6);
        assert!(rect.h == 7);
    }

    #[test]
    fn get_center_y_test() {
        let rect = Rect::new(4, 5, 6, 7);
        assert!(rect.get_center_y() == 8);
    }

    #[test]
    fn set_center_y_test() {
        let mut rect = Rect::new(4, 5, 6, 7);
        rect.set_center_y(1);
        assert!(rect.x == 4);
        assert!(rect.y == -2);
        assert!(rect.w == 6);
        assert!(rect.h == 7);
    }

    #[test]
    fn get_size_test() {
        let rect = Rect::new(0, 0, 10, 20);
        assert!(rect.get_size() == (10, 20));
    }

    #[test]
    fn set_size_test() {
        let mut rect = Rect::new(0, 0, 0, 0);
        rect.set_size((30, 40));
        assert!(rect.w == 30);
        assert!(rect.h == 40);
    }

    #[test]
    fn get_top_left_test() {
        let rect = Rect::new(5, 6, 7, 8);
        assert!(rect.get_top_left() == (5, 6));
    }

    #[test]
    fn set_top_left_test() {
        let mut rect = Rect::new(5, 6, 7, 8);
        rect.set_top_left((11, 12));
        assert!(rect.x == 11);
        assert!(rect.y == 12);
    }

    #[test]
    fn get_top_right_test() {
        let rect = Rect::new(5, 6, 7, 8);
        assert!(rect.get_top_right() == (12, 6));
    }

    #[test]
    fn set_top_right_test() {
        let mut rect = Rect::new(5, 6, 7, 8);
        rect.set_top_right((11, 12));
        assert!(rect.x == 4);
        assert!(rect.y == 12);
    }

    #[test]
    fn get_bottom_left_test() {
        let rect = Rect::new(5, 6, 7, 8);
        assert!(rect.get_bottom_left() == (5, 14));
    }

    #[test]
    fn set_bottom_left_test() {
        let mut rect = Rect::new(5, 6, 7, 8);
        rect.set_bottom_left((11, 12));
        assert!(rect.x == 11);
        assert!(rect.y == 4);
    }

    #[test]
    fn get_bottom_right_test() {
        let rect = Rect::new(5, 6, 7, 8);
        assert!(rect.get_bottom_right() == (12, 14));
    }

    #[test]
    fn set_bottom_right_test() {
        let mut rect = Rect::new(5, 6, 7, 8);
        rect.set_bottom_right((11, 12));
        assert!(rect.x == 4);
        assert!(rect.y == 4);
    }

    #[test]
    fn get_center_test() {
        let rect = Rect::new(5, 6, 7, 8);
        assert!(rect.get_center() == (8, 10));
    }

    #[test]
    fn set_center_test() {
        let mut rect = Rect::new(5, 6, 7, 8);
        rect.set_center((11, 12));
        assert!(rect.x == 8);
        assert!(rect.y == 8);
    }

    #[test]
    fn get_mid_top_test() {
        let rect = Rect::new(5, 6, 7, 8);
        assert!(rect.get_mid_top() == (8, 6));
    }

    #[test]
    fn set_mid_top_test() {
        let mut rect = Rect::new(5, 6, 7, 8);
        rect.set_mid_top((11, 12));
        assert!(rect.x == 8);
        assert!(rect.y == 12);
    }

    #[test]
    fn get_mid_left_test() {
        let rect = Rect::new(5, 6, 7, 8);
        assert!(rect.get_mid_left() == (5, 10));
    }

    #[test]
    fn set_mid_left_test() {
        let mut rect = Rect::new(5, 6, 7, 8);
        rect.set_mid_left((11, 12));
        assert!(rect.x == 11);
        assert!(rect.y == 8);
    }

    #[test]
    fn get_mid_bottom_test() {
        let rect = Rect::new(5, 6, 7, 8);
        assert!(rect.get_mid_bottom() == (8, 14));
    }

    #[test]
    fn set_mid_bottom_test() {
        let mut rect = Rect::new(5, 6, 7, 8);
        rect.set_mid_bottom((11, 12));
        assert!(rect.x == 8);
        assert!(rect.y == 4);
    }

    #[test]
    fn get_mid_right_test() {
        let rect = Rect::new(5, 6, 7, 8);
        assert!(rect.get_mid_right() == (12, 10));
    }

    #[test]
    fn set_mid_right_test() {
        let mut rect = Rect::new(5, 6, 7, 8);
        rect.set_mid_right((11, 12));
        assert!(rect.x == 4);
        assert!(rect.y == 8);
    }
}
