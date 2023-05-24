use std::ops::{Add};


#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }

    pub fn half(&mut self) -> Point {
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

#[derive(Copy, Clone)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl Vertex {
   pub fn new(x: i32, y: i32, z:i32) -> Self {
       Self { x: x, y: y, z: z }
   } 
}

fn _swap_points(a: Point, b: Point) -> (Point, Point) {
    (b, a)
}

pub fn draw_points(length: f32, points: Vec<Point>, stay: usize) -> Vec<(f32, f32)> {
    let l = (length * super::SAMPLE_RATE_F) as i32;
    let mut vec = Vec::new();
    for point in points {
                for _ in 1..stay {
                    vec.push((point.x as f32 / super::SIZE as f32, point.y as f32 / super::SIZE as f32));
                }

    }
    let repeat = vec.iter().cloned().cycle().take(l as usize).collect();
    repeat
}

fn interpolate(i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<i32> {
    if i0 == i1 {
        return vec![d0];
    }
    let mut values = Vec::new();
    let a = (d1 - d0) as f32 / (i1 - i0) as f32;
    let mut d = d0 as f32;
    for _ in i0..i1 {
        values.push(d as i32);
        d = d as f32 + a;
    }
    values
}

pub fn create_line(p0: Point, p1: Point, step: usize) -> Vec<Point> {
    let mut a = p0;
    let mut b: Point = p1;
    let mut vec = Vec::new();
    if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
        // Line is horizontal-ish
        if p0.x > p1.x {
            //let (a, b) = swap_points(a, b);
            let helper = a;
            a = b;
            b = helper;
        }
        let ys = interpolate(a.x, a.y, b.x, b.y);
        for x in (a.x..b.x).step_by(step) {
            // draw(x, ys[x - x0])
            //vec.push((x, ys[(x - a.x) as usize]));
            vec.push(Point::new(x, ys[(x - a.x) as usize]))
        }
    } else {
        // Line is vertical-ish
        if p0.y > p1.y {
            //let (a, b) = swap_points(a, b);
            let helper = a;
            a = b;
            b = helper;
        }
        let xs = interpolate(a.y, a.x, b.y, b.x);
        for y in (a.y..b.y).step_by(step) {
            // draw(x, ys[x - x0])
            //vec.push((xs[(y - a.y) as usize], y));
            vec.push(Point::new(xs[(y - a.y) as usize], y))
        }
    }
    vec
}


pub fn draw_wireframe_triangle (p0: Point, p1: Point, p2: Point, step: usize) -> Vec<Point> { 
    let mut a = create_line(p0, p1, step);
    let mut b = create_line(p1, p2, step);
    let mut c = create_line(p2, p0, step);
    let mut abc = Vec::new();
    abc.append(&mut a);
    abc.append(&mut b);
    abc.append(&mut c);
    abc
}

fn viewport_to_canvas (p: Point) -> Point {
    Point {
        x: p.x * super::CANVAS/super::SIZE,
        y: p.y * super::CANVAS/super::SIZE
    }
}

pub fn project_vertex (v: &mut Vertex) -> Point {
    match v.z {
        0 => {
            Point {x: 0, y:0}
        }
        _ => {
            viewport_to_canvas(Point { 
                x: (v.x * super::DISTANCE / v.z), 
                y: (v.y * super::DISTANCE / v.z)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_interpolate() {
        let (i0, i1, d0, d1) = (-100, 100, -100, 100);
        let i = interpolate(i0, d0, i1, d1);
        println!("{:?}", i);
    }

    #[test]
    fn test_create_line() {
        let point1 = Point { x: 1, y: 50 };
        let point2 = Point { x: 50, y: 10 };
        let line = create_line(point1, point2, 1);
        println!("{:?}", line);
    }
}