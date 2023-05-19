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

fn swap_tuples(a: (i32, i32), b: (i32, i32)) -> ((i32, i32), (i32, i32)) {
    let x = a;
    let y = b;
    (y, x)
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_swap_tuples() {
        let a = (1, 1);
        let b = (2, 2);
        let (a, b) = swap_tuples(a, b);
        assert_eq!(a, (2, 2));
        assert_eq!(b, (1, 1));
    }
}