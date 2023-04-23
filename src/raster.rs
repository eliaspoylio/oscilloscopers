use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    fn half(&mut self) -> Point {
        self.x = self.x / 2;
        self.y = self.y / 2;
        *self
    }
}

impl Add for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point {
            x: &self.x + &other.x,
            y: &self.y + &other.y,
        }
    }
}