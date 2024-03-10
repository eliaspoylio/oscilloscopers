use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }

    pub fn spread(x: f32, y: f32) -> Point {
        Point {
            x: x - super::SIZE_F/super::SIZE_F,
            y: y - super::SIZE_F/super::SIZE_F
        }
    }

    pub fn scroll(&mut self, x:f32, y:f32) {
        self.x = self.x + x;
        self.y = self.y + y;
    } 
}
#[derive(Debug, Clone, Copy)]
pub struct VertexF {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl VertexF {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn rotate(&mut self, pitch: f32, yaw: f32, roll: f32) {
        let cosa = yaw.cos();
        let sina = yaw.sin();
    
        let cosb = pitch.cos();
        let sinb = pitch.sin();
    
        let cosc = roll.cos();
        let sinc = roll.sin();
    
        let axx = cosa*cosb;
        let axy = cosa*sinb*sinc - sina*cosc;
        let axz = cosa*sinb*cosc + sina*sinc;
    
        let ayx = sina*cosb;
        let ayy = sina*sinb*sinc + cosa*cosc;
        let ayz = sina*sinb*cosc - cosa*sinc;
    
        let azx = -sinb;
        let azy = cosb*sinc;
        let azz = cosb*cosc;

        let px = self.x;
        let py = self.y;
        let pz = self.z;

        self.x = axx*px + axy*py + axz*pz;
        self.y = ayx*px + ayy*py + ayz*pz;
        self.z = azx*px + azy*py + azz*pz;
    }

    pub fn min(&mut self, f: f32) {
        if self.x > 0. {
            self.x -= f; 
        }
        else {
            self.x += f; 
        }
        if self.y > 0. {
            self.y -= f; 
        }
        else {
            self.y += f; 
        }
        //self.z += f;
    }
}

fn interpolate_float(i0: f32, d0: f32, i1: f32, d1: f32, step: f32) -> Vec<f32> {
    if i0 == i1 {
        return vec![d0];
    }
    let mut values = Vec::new();
    let a = (d1 - d0) / (i1 - i0);
    let mut d = d0;
    let j = i0 as i8;
    let k = i1 as i8;
    let mut i = i0;
    for _ in j..k {
        i += step;
        if i > i1 {break;}
        values.push(d);
        d = d + a;
    }
    values
}

pub fn create_line_float(p0: Point, p1: Point, step: f32) -> Vec<Point> {
    let mut a = p0;
    let mut b: Point = p1;
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
        for x in (j..k).map(|x| x as f32) {
            if ys.len() > (x - a.x) as usize {
                vec.push(Point::new(x, ys[(x - a.x) as usize]));
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
        for y in (j..k).map(|y| y as f32) {
            if xs.len() > (y - a.y) as usize {
                vec.push(Point::new(xs[(y - a.y) as usize], y));
            }
        }
    }
    vec
}

pub fn draw_points_float(length: f32, points: Vec<Point>, stay: usize) -> Vec<(f32, f32)> {
    let l = (length * super::SAMPLE_RATE_F) as i32;
    let mut vec = Vec::new();
    for point in points {
        for _ in 0..stay {
            vec.push((point.x / super::SIZE_F, point.y / super::SIZE_F));
        }
    }
    let repeat = vec.iter().cloned().cycle().take(l as usize).collect();
    repeat
}

pub fn draw_wireframe_triangle (p0: Point, p1: Point, p2: Point, step: f32) -> Vec<Point> { 
    let mut a = create_line_float(p0, p1, step);
    let mut b = create_line_float(p1, p2, step);
    let mut c = create_line_float(p2, p0, step);
    let mut abc = Vec::new();
    abc.append(&mut a);
    abc.append(&mut b);
    abc.append(&mut c);
    abc
}

fn viewport_to_canvas_f(p: Point) -> Point {
    Point {
        x: p.x * super::CANVAS_F / super::SIZE_F,
        y: p.y * super::CANVAS_F / super::SIZE_F
    }
}

pub fn project_vertex_f(v: &mut VertexF) -> Point {
    viewport_to_canvas_f(Point {
        x: (v.x * super::DISTANCE_F / v.z),
        y: (v.y * super::DISTANCE_F / v.z),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swap_tuples() {
    }
}
