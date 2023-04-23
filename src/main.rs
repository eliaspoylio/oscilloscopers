use hound;
use itertools::{EitherOrBoth::*, Itertools};
use std::f32::consts::PI;
use std::i16;
use std::ops::Add;

mod raster;

const SAMPLE_RATE: u32 = 96000;
const SAMPLE_RATE_U: usize = SAMPLE_RATE as usize;
const SAMPLE_RATE_F: f32 = SAMPLE_RATE as f32;
const SIZE: i32 = 100;

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

fn draw(freq: usize, length: f32, points: Vec<(i32, i32)>) -> Vec<(f32, f32)> {
    let f = SAMPLE_RATE_U / freq;
    let l = length * SAMPLE_RATE_F;
    let mut vec = Vec::new();
    for point in points {
        for _ in 0..f {
            vec.push((point.0 as f32 / SIZE as f32, point.1 as f32 / SIZE as f32));
        }
        for _ in 0..f {
            vec.push((0., 0.));
        }
    }
    let repeat = vec.iter().cloned().cycle().take(l as usize).collect();
    repeat
}

fn draw2(length: f32, points: Vec<Point>) -> Vec<(f32, f32)> {
    let l = (length * SAMPLE_RATE_F) as i32;
    let mut vec = Vec::new();
    for point in points {
        vec.push((point.x as f32 / SIZE as f32, point.y as f32 / SIZE as f32));
    }
    let repeat = vec.iter().cloned().cycle().take(l as usize).collect();
    repeat
}

fn draw_points(length: f32, points: Vec<Point>) -> Vec<(f32, f32)> {
    let l = (length * SAMPLE_RATE_F) as i32;
    let mut vec = Vec::new();
    for point in points {
        for _ in 1..8 {
            vec.push((point.x as f32 / SIZE as f32, point.y as f32 / SIZE as f32));
        }
    }
    let repeat = vec.iter().cloned().cycle().take(l as usize).collect();
    repeat
}

fn draw_with_freq(length: f32, points: Vec<Point>, hz: f32) -> Vec<(f32, f32)> {
    let l = (length * SAMPLE_RATE_F) as i32;
    let period = (SAMPLE_RATE_F / hz) as usize;
    let mut vec = Vec::new();
    for point in &points {
        vec.push((point.x as f32 / SIZE as f32, point.y as f32 / SIZE as f32));
    }
    let start = points.len();
    println!("{start}, {period}");
    if start < period {
        for _i in points.len()..points.len()+period {
            vec.push((0.,0.));
        }
    }
    let repeat = vec.iter().cloned().cycle().take(l as usize).collect();
    repeat
}

fn create_signal(length: f32, a: f32, a_mult: f32, b: f32, b_mult: f32) -> Vec<(f32, f32)> {
    let mut vec = Vec::new();
    for t in 0..((length * SAMPLE_RATE_F) as i32) {
        let x = {
            (2. * PI * (a * a_mult) * t as f32).sin() * (2. * PI * (b * b_mult) * t as f32).cos()
        };
        let y = {
            (2. * PI * (a * a_mult) * t as f32).cos() * (2. * PI * (b * b_mult) * t as f32).cos()
        };
        vec.push((x, y))
    }
    vec
}

fn is_even(x: usize) -> bool {
    let y = x % 2;
    match y {
        0 => true,
        _ => false,
    }
}

fn swap_tuples(a: (i32, i32), b: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let x = a;
    let y = b;
    (y, x)
}

fn swap_points(a: Point, b: Point) -> (Point, Point) {
    (b, a)
}

fn mix_signals(x: Vec<(f32, f32)>, y: Vec<(f32, f32)>, size: usize) -> Vec<(f32, f32)> {
    let (x_left, x_right): (Vec<_>, Vec<_>) = x.iter().cloned().unzip();
    let (y_left, y_right): (Vec<_>, Vec<_>) = y.iter().cloned().unzip();

    let left = mix(x_left, y_left, size);
    let rigth = mix(x_right, y_right, size);

    let mut iter = left.into_iter().zip(rigth.into_iter()).collect();
    iter
}

fn mix(first: Vec<f32>, second: Vec<f32>, size: usize) -> Vec<f32> {
    let mut vec = Vec::new();

    let f = first.chunks(size);
    let s = second.chunks(size);
    let mut counter = 0;

    for pair in f.zip_longest(s) {
        match pair {
            Both(l, r) => match is_even(counter) {
                true => vec.extend_from_slice(&l),
                false => vec.extend_from_slice(&r),
            },
            Left(l) => vec.extend_from_slice(&l),
            Right(r) => vec.extend_from_slice(&r),
        }
        counter += 1;
    }

    vec
}

fn mix_p(first: Vec<Point>, second: Vec<Point>, size: usize) -> Vec<Point> {
    let mut vec = Vec::new();

    let f = first.chunks(size);
    let s = second.chunks(size);
    let mut counter = 0;

    for pair in f.zip_longest(s) {
        match pair {
            Both(l, r) => match is_even(counter) {
                true => vec.extend_from_slice(&l),
                false => vec.extend_from_slice(&r),
            },
            Left(l) => vec.extend_from_slice(&l),
            Right(r) => vec.extend_from_slice(&r),
        }
        counter += 1;
    }

    vec
}

fn interpolate_float(i0: f32, d0: f32, i1: f32, d1: f32) -> Vec<f32> {
    if i0 == i1 {
        return vec![d0];
    }
    let mut values = Vec::new();
    let a = (d1 - d0) / (i1 - i0);
    let mut d = d0;
    let j = (i0 * 100.) as i32;
    let k = (i1 * 100.) as i32;
    for _ in (j..k).map(|x| x as f32 * 0.01) {
        values.push(d);
        d = d + a;
    }
    values
}

fn interpolate(i0: i32, d0: i32, i1: i32, d1: i32) -> Vec<i32> {
    if i0 == i1 {
        return vec![d0];
    }
    let mut values = Vec::new();
    let a = (d1 - d0) as f32 / (i1 - i0) as f32;
    let mut d = d0 as f32;
    for _ in (i0..i1) {
        values.push(d as i32);
        d = d as f32 + a;
    }
    values
}

fn create_line(P0: Point, P1: Point) -> Vec<(Point)> {
    let mut a = P0;
    let mut b: Point = P1;
    let mut vec = Vec::new();
    if (P1.x - P0.x).abs() > (P1.y - P0.y).abs() {
        // Line is horizontal-ish
        if P0.x > P1.x {
            //let (a, b) = swap_points(a, b);
            let helper = a;
            a = b;
            b = helper;
        }
        let ys = interpolate(a.x, a.y, b.x, b.y);
        for x in a.x..b.x {
            // draw(x, ys[x - x0])
            //vec.push((x, ys[(x - a.x) as usize]));
            vec.push(Point::new(x, ys[(x - a.x) as usize]))
        }
    } else {
        // Line is vertical-ish
        if P0.y > P1.y {
            //let (a, b) = swap_points(a, b);
            let helper = a;
            a = b;
            b = helper;
        }
        let xs = interpolate(a.y, a.x, b.y, b.x);
        for y in a.y..b.y {
            // draw(x, ys[x - x0])
            //vec.push((xs[(y - a.y) as usize], y));
            vec.push(Point::new(xs[(y - a.y) as usize], y))
        }
    }
    vec
}

fn sum(x: Vec<Point>, y: Vec<Point>) -> Vec<Point> {
    let mut z = Vec::new();
    for it in x.iter().zip_longest(y.iter()) {
        match it {
            Both(x, y) => z.push(x + y),
            Left(x) => z.push(*x),
            Right(y) => z.push(*y),
        }
    }
    z
}

fn sum_div(x: Vec<Point>, y: Vec<Point>) -> Vec<Point> {
    let mut z = Vec::new();
    for it in x.iter().zip_longest(y.iter()) {
        match it {
            Both(x, y) => z.push((x + y).half()),
            Left(x) => z.push(*x),
            Right(y) => z.push(*y),
        }
    }
    z
}

fn main() -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let amplitude = i16::MAX as f32;

    let mut writer = hound::WavWriter::create("test3.wav", spec).unwrap();

    // /////////////////////////
    
    let mut _line1 = create_line(Point { x: 0, y: 50 }, Point { x: -50, y: -50 });
    let mut _line2 = create_line(Point { x: 0, y: 50 }, Point { x: 50, y: -50 });
    let mut _line3 = create_line(Point { x: -50, y: -50 }, Point { x: 50, y: -50 });

    _line1.append(&mut _line2);
    _line1.append(&mut _line3);

    let _draw_triangle = draw_points(5., _line1);

    for d in _draw_triangle {
        writer.write_sample((d.0 * amplitude) as i16).unwrap();
        writer.write_sample((d.1 * amplitude) as i16).unwrap();
    }

    let mut p1 = create_line(Point { x: -90, y: 90 }, Point { x: -90, y: 0 });
    let mut p2 = create_line(Point { x: -90, y: 90 }, Point { x: -20, y: 60 });
    let mut p3 = create_line(Point { x: -90, y: 40 }, Point { x: -30, y: 60 });

    let mut e1 = create_line(Point { x: -20, y: 90 }, Point { x: -20, y: 0 });
    let mut e2 = create_line(Point { x: -20, y: 90 }, Point { x: 0, y: 90 });
    let mut e3 = create_line(Point { x: -20, y: 45 }, Point { x: 0, y: 45 });
    let mut e4 = create_line(Point { x: -20, y: 0 }, Point { x: 0, y: 0 });

    let mut l1 = create_line(Point { x: 10, y: 90 }, Point { x: 10, y: 0 });
    let mut l2 = create_line(Point { x: 10, y: 0 }, Point { x: 40, y: 0 });

    let mut i = create_line(Point { x: 60, y: 90 }, Point { x: 60, y: 0 });

    let mut m1 = create_line(Point { x: -90, y: -10 }, Point { x: -90, y: -90 });
    let mut m2 = create_line(Point { x: -90, y: -10 }, Point { x: -60, y: -90 });
    let mut m3 = create_line(Point { x: -60, y: -90 }, Point { x: -30, y: -10 });
    let mut m4 = create_line(Point { x: -30, y: -10 }, Point { x: -30, y: -90 });

    let mut i2 = create_line(Point { x: 0, y: -90 }, Point { x: 0, y: 0 });
    let mut i3 = create_line(Point { x: 10, y: -90 }, Point { x: 10, y: 0 });

    let mut t1 = create_line(Point { x: 20, y: -10 }, Point { x: 80, y: -10 });
    let mut t2 = create_line(Point { x: 40, y: -10 }, Point { x: 40, y: -90 });
    let mut t3 = create_line(Point { x: 70, y: -10 }, Point { x: 70, y: -90 });

    let mut i4 = create_line(Point { x: 90, y: -10 }, Point { x: 90, y: -90 });

    let mut asdf = Vec::new();
    let mut asdf2 = Vec::new();
    asdf.append(&mut p1);
    asdf.append(&mut p2);
    asdf.append(&mut p3);

    asdf.append(&mut e1);
    asdf.append(&mut e2);
    asdf.append(&mut e3);
    asdf.append(&mut e4);

    asdf.append(&mut l1);
    asdf.append(&mut l2);

    asdf.append(&mut i);

    asdf.append(&mut m1);
    asdf.append(&mut m2);
    asdf.append(&mut m3);
    asdf.append(&mut m4);

    asdf.append(&mut i2);
    asdf.append(&mut i3);

    asdf.append(&mut t1);
    asdf.append(&mut t2);
    asdf.append(&mut t3);

    asdf.append(&mut i4);


    asdf2.append(&mut m1);
    asdf2.append(&mut m2);
    asdf2.append(&mut m3);
    asdf2.append(&mut m4);

    asdf2.append(&mut i2);
    asdf2.append(&mut i3);

    asdf2.append(&mut t1);
    asdf2.append(&mut t2);
    asdf2.append(&mut t3);

    asdf2.append(&mut i4);

    let asdfs = mix_p(asdf.clone(), asdf2, 5);

    let draw_text = draw_points(5., asdf);
    let draw_text2 = draw_points(5., asdfs);

    for d in draw_text {
        writer.write_sample((d.0 * amplitude) as i16).unwrap();
        writer.write_sample((d.1 * amplitude) as i16).unwrap();
    }
    for d in draw_text2 {
        writer.write_sample((d.0 * amplitude) as i16).unwrap();
        writer.write_sample((d.1 * amplitude) as i16).unwrap();
    }

    println!("Length: {}", writer.len());
    println!("Duration: {}", writer.duration() / spec.sample_rate);
    writer.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert_eq!(is_even(0), true);
        assert_eq!(is_even(1), false);
        assert_eq!(is_even(2), true);
        assert_eq!(is_even(3), false);
    }

    #[test]
    fn test_mix_with_equally_sized_vectors() {
        let a: Vec<f32> = vec![0.1, 0.2, 0.3];
        let b: Vec<f32> = vec![-0.1, -0.2, -0.3];
        let ab: Vec<f32> = vec![0.1, -0.2, 0.3];

        let mix = mix(a, b, 1);
        assert_eq!(mix, ab)
    }

    #[test]
    fn test_mix_with_inequally_sized_vectors() {
        let a: Vec<f32> = vec![0.1, 0.2, 0.3, 0.4, 0.5];
        let b: Vec<f32> = vec![-0.1, -0.2, -0.3];
        let ab: Vec<f32> = vec![0.1, -0.2, 0.3, 0.4, 0.5];

        let c: Vec<f32> = vec![0.1, 0.2, 0.3];
        let d: Vec<f32> = vec![-0.1, -0.2, -0.3, -0.4, -0.5, -0.6];
        let cd: Vec<f32> = vec![0.1, -0.2, 0.3, -0.4, -0.5, -0.6];

        let mix_ab = mix(a, b, 1);
        let mix_cd = mix(c, d, 1);
        assert_eq!(mix_ab, ab);
        assert_eq!(mix_cd, cd);
    }

    #[test]
    fn test_interpolate() {
        let (i0, i1, d0, d1) = (-100, 100, -100, 100);
        let i = interpolate(i0, d0, i1, d1);
        println!("{:?}", i);
    }

    #[test]
    fn test_swap_tuples() {
        let a = (1, 1);
        let b = (2, 2);
        let (a, b) = swap_tuples(a, b);
        assert_eq!(a, (2, 2));
        assert_eq!(b, (1, 1));
    }

    #[test]
    fn test_create_line() {
        let point1 = Point { x: 1, y: 50 };
        let point2 = Point { x: 50, y: 10 };
        let line = create_line(point1, point2);
        println!("{:?}", line);
    }

    #[test]
    fn test_sum() {
        let mut vec1 = vec![Point::new(5, 5)];
        let mut vec2 = vec![Point::new(5, 5)];
        let sum = sum(vec1, vec2);
        assert_eq!(sum, vec![Point { x: 10, y: 10 }])
    }

    #[test]
    fn test_sum_div() {
        let mut vec1 = vec![Point::new(5, 5)];
        let mut vec2 = vec![Point::new(5, 5)];
        let sum = sum_div(vec1, vec2);
        assert_eq!(sum, vec![Point { x: 5, y: 5 }])
    }
}
