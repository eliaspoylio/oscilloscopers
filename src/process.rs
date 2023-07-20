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

fn mix_signals(x: Vec<(f32, f32)>, y: Vec<(f32, f32)>, size: usize) -> Vec<(f32, f32)> {
    let (x_left, x_right): (Vec<_>, Vec<_>) = x.iter().cloned().unzip();
    let (y_left, y_right): (Vec<_>, Vec<_>) = y.iter().cloned().unzip();

    let left = mix(x_left, y_left, size);
    let rigth = mix(x_right, y_right, size);

    let mut iter = left.into_iter().zip(rigth.into_iter()).collect();
    iter
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

pub fn avg(x: Vec<Point>, y: Vec<Point>) -> Vec<Point> {
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

#[cfg(test)]
mod tests {
    use super::*;

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