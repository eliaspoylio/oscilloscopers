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
        for _i in points.len()..points.len() + period {
            vec.push((0., 0.));
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
}