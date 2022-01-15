use std::cmp;

#[derive(Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x, y, w, h }
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

    pub fn move_(&self, x: i32, y: i32) -> Rect {
        Rect::new(self.x + x, self.y + y, self.w, self.h)
    }

    pub fn move_ip(&mut self, x: i32, y: i32) {
        self.x += x;
        self.y += y;
    }

    pub fn inflate(&self, x: i32, y: i32) -> Rect {
        Rect::new(self.x - x / 2, self.y - y / 2, self.w + x, self.h + y)
    }

    pub fn inflate_ip(&mut self, x: i32, y: i32) {
        self.x -= x / 2;
        self.y -= y / 2;
        self.w += x;
        self.h += y;
    }

    pub fn update(&mut self, x: i32, y: i32, width: i32, height: i32) {
        self.x = x;
        self.y = y;
        self.w = width;
        self.h = height;
    }

    pub fn clamp(&self, rect: &Rect) -> Rect {
        let mut x = cmp::max(rect.x, cmp::min(self.x + self.w, rect.x + rect.w) - self.w);
        let mut y = cmp::max(rect.y, cmp::min(self.y + self.h, rect.y + rect.h) - self.h);
        if self.w > rect.w {
            x -= (self.w - rect.w) / 2;
        }
        if self.h > rect.h {
            y -= (self.h - rect.h) / 2;
        }
        Rect::new(x, y, self.w, self.h)
    }

    pub fn clamp_ip(&mut self, rect: &Rect) {
        self.x = cmp::max(rect.x, cmp::min(self.x + self.w, rect.x + rect.w) - self.w);
        self.y = cmp::max(rect.y, cmp::min(self.y + self.h, rect.y + rect.h) - self.h);
        if self.w > rect.w {
            self.x -= (self.w - rect.w) / 2;
        }
        if self.h > rect.h {
            self.y -= (self.h - rect.h) / 2;
        }
    }

    pub fn clip(&self, rect: &Rect) -> Rect {
        if self.collide_rect(rect) {
            let x = cmp::max(self.x, rect.x);
            let y = cmp::max(self.y, rect.y);
            let w = cmp::min(self.get_right(), rect.get_right()) - x;
            let h = cmp::min(self.get_bottom(), rect.get_bottom()) - y;
            Rect::new(x, y, w, h)
        } else {
            Rect::new(rect.x, rect.y, 0, 0)
        }
    }

    // clipline
    // union
    // union_ip
    // unionall
    // unionall_ip
    // fit

    pub fn normalize(&mut self) {
        if self.w < 0 {
            self.x += self.w;
            self.w *= -1;
        }
        if self.h < 0 {
            self.y += self.h;
            self.h *= -1;
        }
    }

    pub fn contains(&self, rect: &Rect) -> bool {
        self.get_left() <= rect.get_left()
            && self.get_right() >= rect.get_right()
            && self.get_top() <= rect.get_top()
            && self.get_bottom() >= rect.get_bottom()
    }

    pub fn collide_point(&self, x: i32, y: i32) -> bool {
        self.get_left() <= x
            && x <= self.get_right()
            && self.get_top() <= y
            && y <= self.get_bottom()
    }

    pub fn collide_rect(&self, rect: &Rect) -> bool {
        self.get_left() < rect.get_right()
            && rect.get_left() < self.get_right()
            && self.get_top() < rect.get_bottom()
            && rect.get_top() < self.get_bottom()
    }

    // collidelist
    // collidelistall
}

#[cfg(test)]
mod rect_test {
    use super::*;

    fn assert_equal_rect(rect: &Rect, x: i32, y: i32, w: i32, h: i32) {
        assert!(rect.x == x);
        assert!(rect.y == y);
        assert!(rect.w == w);
        assert!(rect.h == h);
    }

    #[test]
    fn new_test() {
        let rect = Rect::new(1, 2, 3, 4);
        assert_equal_rect(&rect, 1, 2, 3, 4);
    }

    #[test]
    fn copy_test() {
        let rect = Rect::new(1, 2, 3, 4).clone();
        assert_equal_rect(&rect, 1, 2, 3, 4);
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
        assert_equal_rect(&rect, 4, 5, 1, 7);
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
        assert_equal_rect(&rect, 4, 5, 6, 1);
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
        assert_equal_rect(&rect, 1, 5, 6, 7);
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
        assert_equal_rect(&rect, -5, 5, 6, 7);
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
        assert_equal_rect(&rect, 4, 1, 6, 7);
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
        assert_equal_rect(&rect, 4, -6, 6, 7);
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
        assert_equal_rect(&rect, -2, 5, 6, 7);
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
        assert_equal_rect(&rect, 4, -2, 6, 7);
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

    #[test]
    fn move_test() {
        let rect = Rect::new(0, 0, 1, 2).move_(5, 10);
        assert_equal_rect(&rect, 5, 10, 1, 2);
    }

    #[test]
    fn move_ip_test() {
        let mut rect = Rect::new(0, 0, 1, 2);
        rect.move_ip(5, 10);
        assert_equal_rect(&rect, 5, 10, 1, 2);
    }

    #[test]
    fn inflate_test() {
        let rect = Rect::new(0, 0, 1, 2).inflate(8, 10);
        assert_equal_rect(&rect, -4, -5, 9, 12);
    }

    #[test]
    fn inflate_ip_test() {
        let mut rect = Rect::new(0, 0, 1, 2);
        rect.inflate_ip(8, 10);
        assert_equal_rect(&rect, -4, -5, 9, 12);
    }

    #[test]
    fn update_test() {
        let mut rect = Rect::new(0, 0, 0, 0);
        rect.update(1, 2, 3, 4);
        assert_equal_rect(&rect, 1, 2, 3, 4);
    }

    #[test]
    fn clamp_test() {
        let rect1 = Rect::new(10, 20, 10, 10).clamp(&Rect::new(20, 20, 100, 100));
        assert_equal_rect(&rect1, 20, 20, 10, 10);
        let rect2 = Rect::new(10, 20, 10, 10).clamp(&Rect::new(20, 20, 100, 100));
        assert_equal_rect(&rect2, 20, 20, 10, 10);
        let rect3 = Rect::new(200, 20, 10, 10).clamp(&Rect::new(20, 20, 100, 100));
        assert_equal_rect(&rect3, 110, 20, 10, 10);
        let rect4 = Rect::new(20, 200, 10, 10).clamp(&Rect::new(20, 20, 100, 100));
        assert_equal_rect(&rect4, 20, 110, 10, 10);
        let rect5 = Rect::new(20, 20, 20, 20).clamp(&Rect::new(100, 100, 10, 10));
        assert_equal_rect(&rect5, 95, 95, 20, 20);
    }

    #[test]
    fn clamp_ip_test() {
        let mut rect1 = Rect::new(10, 20, 10, 10);
        rect1.clamp_ip(&Rect::new(20, 20, 100, 100));
        assert_equal_rect(&rect1, 20, 20, 10, 10);
        let mut rect2 = Rect::new(20, 10, 10, 10);
        rect2.clamp_ip(&Rect::new(20, 20, 100, 100));
        assert_equal_rect(&rect2, 20, 20, 10, 10);
        let mut rect3 = Rect::new(200, 20, 10, 10);
        rect3.clamp_ip(&Rect::new(20, 20, 100, 100));
        assert_equal_rect(&rect3, 110, 20, 10, 10);
        let mut rect4 = Rect::new(10, 200, 10, 10);
        rect4.clamp_ip(&Rect::new(20, 20, 100, 100));
        assert_equal_rect(&rect4, 20, 110, 10, 10);
        let mut rect5 = Rect::new(200, 200, 20, 20);
        rect5.clamp_ip(&Rect::new(100, 100, 10, 10));
        assert_equal_rect(&rect5, 95, 95, 20, 20);
    }

    #[test]
    fn clip_test() {
        let rect1 = Rect::new(10, 10, 10, 10).clip(&Rect::new(15, 15, 5, 5));
        assert_equal_rect(&rect1, 15, 15, 5, 5);
    }

    #[test]
    fn normalize_test() {
        let mut rect1 = Rect::new(10, 10, 10, 10);
        rect1.normalize();
        assert_equal_rect(&rect1, 10, 10, 10, 10);
        let mut rect2 = Rect::new(10, 10, -10, 10);
        rect2.normalize();
        assert_equal_rect(&rect2, 00, 10, 10, 10);
        let mut rect3 = Rect::new(10, 10, 10, -10);
        rect3.normalize();
        assert_equal_rect(&rect3, 10, 0, 10, 10);
        let mut rect4 = Rect::new(10, 10, -10, -10);
        rect4.normalize();
        assert_equal_rect(&rect4, 0, 0, 10, 10);
    }

    #[test]
    fn contains_test() {
        assert!(Rect::new(10, 10, 10, 10).contains(&Rect::new(10, 10, 10, 10)) == true);
        assert!(Rect::new(10, 10, 10, 10).contains(&Rect::new(9, 10, 10, 10)) == false);
        assert!(Rect::new(10, 10, 10, 10).contains(&Rect::new(11, 10, 10, 10)) == false);
        assert!(Rect::new(10, 10, 10, 10).contains(&Rect::new(10, 9, 10, 10)) == false);
        assert!(Rect::new(10, 10, 10, 10).contains(&Rect::new(10, 11, 10, 10)) == false);
    }

    #[test]
    fn collide_point_test() {
        assert!(Rect::new(10, 10, 10, 10).collide_point(15, 15) == true);
        assert!(Rect::new(10, 10, 10, 10).collide_point(10, 15) == true);
        assert!(Rect::new(10, 10, 10, 10).collide_point(15, 10) == true);
        assert!(Rect::new(10, 10, 10, 10).collide_point(20, 15) == true);
        assert!(Rect::new(10, 10, 10, 10).collide_point(15, 20) == true);
        assert!(Rect::new(10, 10, 10, 10).collide_point(5, 15) == false);
        assert!(Rect::new(10, 10, 10, 10).collide_point(15, 5) == false);
        assert!(Rect::new(10, 10, 10, 10).collide_point(25, 15) == false);
        assert!(Rect::new(10, 10, 10, 10).collide_point(15, 25) == false);
    }

    #[test]
    fn collide_rect_test() {
        assert!(Rect::new(10, 10, 10, 10).collide_rect(&Rect::new(20, 20, 10, 10)) == false);
        assert!(Rect::new(10, 10, 10, 10).collide_rect(&Rect::new(10, 20, 10, 10)) == false);
        assert!(Rect::new(10, 10, 10, 10).collide_rect(&Rect::new(20, 10, 10, 10)) == false);
        assert!(Rect::new(10, 10, 10, 10).collide_rect(&Rect::new(15, 15, 10, 10)) == true);
    }
}
