pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Rect {
        Rect {
            x,
            y,
            width,
            height,
        }
    }

    // TODO left, right, bottom, top, center_x, center_y

    pub fn get_size(&self) -> (i32, i32) {
        (self.width, self.height)
    }

    pub fn set_size(&mut self, size: (i32, i32)) {
        self.width = size.0;
        self.height = size.1;
    }

    pub fn get_top_left(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_top_left(&mut self, top_left: (i32, i32)) {
        self.x = top_left.0;
        self.y = top_left.1;
    }

    pub fn get_top_right(&self) -> (i32, i32) {
        (self.x + self.width, self.y)
    }

    pub fn set_top_right(&mut self, top_right: (i32, i32)) {
        self.x = top_right.0 - self.width;
        self.y = top_right.1;
    }

    pub fn get_bottom_left(&self) -> (i32, i32) {
        (self.x, self.y + self.height)
    }

    pub fn set_bottom_left(&mut self, bottom_left: (i32, i32)) {
        self.x = bottom_left.0;
        self.y = bottom_left.1 - self.height;
    }

    pub fn get_bottom_right(&self) -> (i32, i32) {
        (self.x + self.width, self.y + self.height)
    }

    pub fn set_bottom_right(&mut self, bottom_right: (i32, i32)) {
        self.x = bottom_right.0 - self.width;
        self.y = bottom_right.1 - self.height;
    }

    pub fn get_center(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height / 2)
    }

    pub fn set_center(&mut self, center: (i32, i32)) {
        self.x = center.0 - self.width / 2;
        self.y = center.1 - self.height / 2;
    }

    pub fn get_mid_top(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y)
    }

    pub fn set_mid_top(&mut self, mid_top: (i32, i32)) {
        self.x = mid_top.0 - self.width / 2;
        self.y = mid_top.1;
    }

    pub fn get_mid_left(&self) -> (i32, i32) {
        (self.x, self.y + self.height / 2)
    }

    pub fn set_mid_left(&mut self, mid_left: (i32, i32)) {
        self.x = mid_left.0;
        self.y = mid_left.1 - self.height / 2;
    }

    pub fn get_mid_bottom(&self) -> (i32, i32) {
        (self.x + self.width / 2, self.y + self.height)
    }

    pub fn set_mid_bottom(&mut self, mid_bottom: (i32, i32)) {
        self.x = mid_bottom.0 - self.width / 2;
        self.y = mid_bottom.1 - self.height;
    }

    pub fn get_mid_right(&self) -> (i32, i32) {
        (self.x + self.width, self.y + self.height / 2)
    }

    pub fn set_mid_right(&mut self, mid_right: (i32, i32)) {
        self.x = mid_right.0 - self.width;
        self.y = mid_right.1 - self.height / 2;
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
        assert!(rect.width == 3);
        assert!(rect.height == 4);
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
        assert!(rect.width == 30);
        assert!(rect.height == 40);
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
