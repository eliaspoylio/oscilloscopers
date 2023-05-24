use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VPoint {
    pub x: f32,
    pub y: f32,
}

impl VPoint {
    pub fn new(x: f32, y: f32) -> VPoint {
        VPoint { x: x, y: y }
    }

}

pub struct VertexF {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl VertexF {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }
}

const f: f32 = 0.1;

fn interpolate_float(i0: f32, d0: f32, i1: f32, d1: f32, step: f32) -> Vec<f32> {
    if i0 == i1 {
        return vec![d0];
    }
    let mut values = Vec::new();
    let a = (d1 - d0) / (i1 - i0);
    let mut d = d0;
    let j = i0 as i8;
    let k = i1 as i8;
    // TODO: step argument?
    for _ in (j..k).map(|x| x as f32 * step) {
        values.push(d);
        d = d + a;
    }
    values
}

pub fn create_line_float(p0: VPoint, p1: VPoint, step: f32) -> Vec<VPoint> {
    let mut a = p0;
    let mut b: VPoint = p1;
    let mut vec = Vec::new();
    if (p1.x - p0.x).abs() > (p1.y - p0.y).abs() {
        // Line is horizontal-ish
        if p0.x > p1.x {
            let helper = a;
            a = b;
            b = helper;
        }
        let ys = interpolate_float(a.x, a.y, b.x, b.y, step);
        let j = a.x as i8;
        let k = b.x as i8;
        for x in (j..k).map(|x| x as f32 * step) {
            if ys.len() > (x - a.x) as usize {
                vec.push(VPoint::new(x, ys[(x - a.x) as usize]));
            }
        }
    } else {
        // Line is vertical-ish
        if p0.y > p1.y {
            let helper = a;
            a = b;
            b = helper;
        }
        let xs = interpolate_float(a.y, a.x, b.y, b.x, step);
        let j = a.y as i8;
        let k = b.y as i8;
        for y in (j..k).map(|y| y as f32 * step) {
            if xs.len() > (y - a.y) as usize {
                vec.push(VPoint::new(xs[(y - a.y) as usize], y));
            }
        }
    }
    vec
}

pub fn draw_points_float(length: f32, points: Vec<VPoint>, stay: usize) -> Vec<(f32, f32)> {
    let l = (length * super::SAMPLE_RATE_F) as i32;
    let mut vec = Vec::new();
    for point in points {
        for _ in 0..stay {
            vec.push((point.x / super::SIZE as f32, point.y / super::SIZE as f32));
        }
    }
    let repeat = vec.iter().cloned().cycle().take(l as usize).collect();
    repeat
}

fn viewport_to_canvas_f(p: VPoint) -> VPoint {
    VPoint {
        x: p.x * (super::CANVAS/super::SIZE) as f32,
        y: p.y * (super::CANVAS/super::SIZE) as f32
    }
}

pub fn project_vertex_f(v: &mut VertexF) -> VPoint {
    viewport_to_canvas_f(VPoint {
        x: (v.x * super::DISTANCE as f32 / v.z),
        y: (v.y * super::DISTANCE as f32 / v.z),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_tuples() {
    }
}
