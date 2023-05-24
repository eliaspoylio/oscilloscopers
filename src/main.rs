use hound;
use itertools::{EitherOrBoth::*, Itertools};
use std::f32::consts::PI;
use std::i16;

mod raster;
use crate::raster::{
    create_line, draw_points, draw_wireframe_triangle, project_vertex, Point, Vertex,
};
mod vector;
use crate::vector::{create_line_float, draw_points_float, project_vertex_f, VPoint, VertexF};

const SAMPLE_RATE: u32 = 96000;
const SAMPLE_RATE_U: usize = SAMPLE_RATE as usize;
const SAMPLE_RATE_F: f32 = SAMPLE_RATE as f32;
const SIZE: i32 = 100;
const CANVAS: i32 = 200;
const DISTANCE: i32 = 50;

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

fn rotate_cube(angleX: f32, angleY: f32, vertices: [&mut Vertex; 8]) {
    println!("angleX: {}", angleX);
    let sinX = angleX.sin();
    let cosX = angleX.cos();
    let sinY = angleY.sin();
    let cosY = angleY.cos();
    for v in vertices {
        let x = v.x as f32;
        let y = v.y as f32;
        let mut z = v.z as f32;
        v.x = (x * cosX - z * sinX) as i32;
        v.z = (z * cosX + x * sinX) as i32;
        z = v.z as f32;
        v.x = (y * cosY - z * sinY) as i32;
        v.y = (z * cosY + y * sinY) as i32;
    }
}

fn rotate(angleX: f32, angleY: f32, vertices: [&mut Vertex; 8]) {
    let sinX = angleX.sin();
    let cosX = angleX.cos();
    let sinY = angleY.sin();
    let cosY = angleY.cos();
    for v in vertices {
        let x = v.x as f32;
        let y = v.y as f32;
        let mut z = v.z as f32;
        v.x = (x * cosX - z * sinX) as i32;
        v.z = (z * cosX + x * sinX) as i32;
        //z = v.z as f32;
        v.y = (y * cosY - z * sinY) as i32;
        v.z = (z * cosY + y * sinY) as i32;
    }
}

fn rotate_x(angle: f32, vertice: &mut VertexF) {
    let sin = angle.sin();
    let cos = angle.cos();
    vertice.y = vertice.y * cos - vertice.z * sin;
    vertice.z = vertice.y * sin + vertice.z * cos;
}

/*
 rotateY(angle) {
   let cos = Math.cos(angle);
   let sin = Math.sin(angle);
   for (let v of this.verData) {
     v = v.pos;
     let x = v.z * sin + v.x * cos;
     let z = v.z * cos - v.x * sin;
     v.x = x;
     v.z = z;
   }
 }
*/
fn rotate_y(angle: f32, vertice: &mut VertexF) {
    let sin = angle.sin();
    let cos = angle.cos();
    vertice.x = vertice.z * sin + vertice.x * cos;
    vertice.z = vertice.z * cos - vertice.x * sin;
}

fn main() -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let amplitude = i16::MAX as f32;
    let mut writer = hound::WavWriter::create("float.wav", spec).unwrap();

    // /////////////////////////.2

    let mut cube: Vec<(f32, f32)> = Vec::new();
    /*
    // The four "front" vertices
    let mut vAf = Vertex::new(-20, 80, 100);
    let mut vBf = Vertex::new(20, 80, 100);
    let mut vCf = Vertex::new(20, 60, 100);
    let mut vDf = Vertex::new(-20, 60, 100);

    // The four "back" vertices
    let mut vAb = Vertex::new(-20, 80, 140);
    let mut vBb = Vertex::new(20, 80, 140);
    let mut vCb = Vertex::new(20, 60, 140);
    let mut vDb = Vertex::new(-20, 60, 140);
    */
    let mut vAf = VertexF::new(-20., 80., 100.);
    let mut vBf = VertexF::new(20., 80., 100.);
    let mut vCf = VertexF::new(20., 60., 100.);
    let mut vDf = VertexF::new(-20., 60., 100.);

    let mut vAb = VertexF::new(-20., 80., 140.);
    let mut vBb = VertexF::new(20., 80., 140.);
    let mut vCb = VertexF::new(20., 60., 140.);
    let mut vDb = VertexF::new(-20., 60., 140.);

    //let vertices = [
    //    &mut vAf, &mut vBf, &mut vCf, &mut vDf, &mut vAb, &mut vBb, &mut vCb, &mut vDb,
    //];
    //rotate_cube(PI / 4., (2. as f32).sqrt().atan(), vertices);

    for i in 1..500 {
        let vertices = [
            &mut vAf, &mut vBf, &mut vCf, &mut vDf, &mut vAb, &mut vBb, &mut vCb, &mut vDb,
        ];
        //let vertices = [vAf, vBf, vCf, vDf, vAb, vBb, vCb, vDb];
        //rotate_cube(((i as f32 / 1000.) * PI / 180.), 0., vertices);
        //rotate(PI / 180., 0., vertices);
        for v in vertices {
            rotate_x(PI / 280., v);
        }

        //vAf.x = 4;
        //println!("vAf x {} {} {} {}", vAf.x, vBf.x, vCf.x, vDf.x);

        let array: [Vec<VPoint>; 12] = [
            /*
            // The front face
            create_line(project_vertex(&mut vAf), project_vertex(&mut vBf), 1),
            create_line(project_vertex(&mut vBf), project_vertex(&mut vCf), 1),
            create_line(project_vertex(&mut vCf), project_vertex(&mut vDf), 1),
            create_line(project_vertex(&mut vDf), project_vertex(&mut vAf), 1),
            // The back face
            create_line(project_vertex(&mut vAb), project_vertex(&mut vBb), 1),
            create_line(project_vertex(&mut vBb), project_vertex(&mut vCb), 1),
            create_line(project_vertex(&mut vCb), project_vertex(&mut vDb), 1),
            create_line(project_vertex(&mut vDb), project_vertex(&mut vAb), 1),
            // The front-to-back edges
            create_line(project_vertex(&mut vAf), project_vertex(&mut vAb), 1),
            create_line(project_vertex(&mut vBf), project_vertex(&mut vBb), 1),
            create_line(project_vertex(&mut vCf), project_vertex(&mut vCb), 1),
            create_line(project_vertex(&mut vDf), project_vertex(&mut vDb), 1),
            */
            create_line_float(project_vertex_f(&mut vAf), project_vertex_f(&mut vBf), 1.),
            create_line_float(project_vertex_f(&mut vBf), project_vertex_f(&mut vCf), 1.),
            create_line_float(project_vertex_f(&mut vCf), project_vertex_f(&mut vDf), 1.),
            create_line_float(project_vertex_f(&mut vDf), project_vertex_f(&mut vAf), 1.),
            // The back face
            create_line_float(project_vertex_f(&mut vAb), project_vertex_f(&mut vBb), 1.),
            create_line_float(project_vertex_f(&mut vBb), project_vertex_f(&mut vCb), 1.),
            create_line_float(project_vertex_f(&mut vCb), project_vertex_f(&mut vDb), 1.),
            create_line_float(project_vertex_f(&mut vDb), project_vertex_f(&mut vAb), 1.),
            // The front-to-back edges
            create_line_float(project_vertex_f(&mut vAf), project_vertex_f(&mut vAb), 1.),
            create_line_float(project_vertex_f(&mut vBf), project_vertex_f(&mut vBb), 1.),
            create_line_float(project_vertex_f(&mut vCf), project_vertex_f(&mut vCb), 1.),
            create_line_float(project_vertex_f(&mut vDf), project_vertex_f(&mut vDb), 1.),
        ];

        let mut lines: Vec<VPoint> = vec![];
        for line in array {
            //println!("Length: {}", line.len());
            for l in line {
                lines.push(l);
            }
        }

        let cli = draw_points_float(1. / 50., lines, 8);
        for cl in cli {
            cube.push(cl);
        }

        let vertices = [
            &mut vAf, &mut vBf, &mut vCf, &mut vDf, &mut vAb, &mut vBb, &mut vCb, &mut vDb,
        ];
        for v in vertices {
            rotate_y(PI / 280., v);
        }

        let array: [Vec<VPoint>; 12] = [
            create_line_float(project_vertex_f(&mut vAf), project_vertex_f(&mut vBf), 1.),
            create_line_float(project_vertex_f(&mut vBf), project_vertex_f(&mut vCf), 1.),
            create_line_float(project_vertex_f(&mut vCf), project_vertex_f(&mut vDf), 1.),
            create_line_float(project_vertex_f(&mut vDf), project_vertex_f(&mut vAf), 1.),
            // The back face
            create_line_float(project_vertex_f(&mut vAb), project_vertex_f(&mut vBb), 1.),
            create_line_float(project_vertex_f(&mut vBb), project_vertex_f(&mut vCb), 1.),
            create_line_float(project_vertex_f(&mut vCb), project_vertex_f(&mut vDb), 1.),
            create_line_float(project_vertex_f(&mut vDb), project_vertex_f(&mut vAb), 1.),
            // The front-to-back edges
            create_line_float(project_vertex_f(&mut vAf), project_vertex_f(&mut vAb), 1.),
            create_line_float(project_vertex_f(&mut vBf), project_vertex_f(&mut vBb), 1.),
            create_line_float(project_vertex_f(&mut vCf), project_vertex_f(&mut vCb), 1.),
            create_line_float(project_vertex_f(&mut vDf), project_vertex_f(&mut vDb), 1.),
        ];

        let mut lines: Vec<VPoint> = vec![];
        for line in array {
            //println!("Length: {}", line.len());
            for l in line {
                lines.push(l);
            }
        }
        //println!("Length: {}", lines.len());
        //let cli = draw_points(1./24., lines, 8);
        let cli = draw_points_float(1. / 50., lines, 8);
        for cl in cli {
            cube.push(cl);
        }
    }

    //let cl = draw_points(5., cube, 8);
    for c in cube {
        writer.write_sample((c.0 * amplitude) as i16).unwrap();
        writer.write_sample((c.1 * amplitude) as i16).unwrap();
    }

    /*
    let mut abcc = Vec::new();
    for i in 1..300 {
        let drawtr = draw_wireframe_triangle(
            Point { x: -8 + i, y: -9 + i},
            Point { x: -2 - i/2, y: 7 - i/2 },
            Point { x: 8 + i , y: -5 + i },
            4);
        println!("Length: {}", drawtr.len());
        let abc = draw_points(1./50., drawtr, 8);
        for v in abc {
            abcc.push(v)
        }
    }
    for _d in abcc {
        writer.write_sample((_d.0 * amplitude) as i16).unwrap();
        writer.write_sample((_d.1 * amplitude) as i16).unwrap();
    }

    let drawtr = draw_wireframe_triangle(
        Point { x: -80, y: -99 },
        Point { x: -20, y: 70 },
        Point { x: 80, y: -50 },
        4);
    println!("Length: {}", drawtr.len());
    let ddrawtr = draw_points(5., drawtr, 8);
    for _d in ddrawtr {
        writer.write_sample((_d.0 * amplitude) as i16).unwrap();
        writer.write_sample((_d.1 * amplitude) as i16).unwrap();
    }
    */

    println!("Length: {}", writer.len());
    println!("Duration: {}", writer.duration() / spec.sample_rate);
    writer.finalize()
}

#[cfg(test)]
mod animation;

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
    fn test_trig() {
        let angleX: f32 = PI / 1300.;
        let angleY: f32 = (2. as f32).sqrt().atan();
        let sinX = angleX.sin();
        let cosX = angleX.cos();
        let sinY = angleY.sin();
        let cosY = angleY.cos();
        let x = 90 as f32;
        let y = 80 as f32;
        let mut z = 70 as f32;

        let x = (x * cosX - z * sinX);
        let z = (z * cosX + x * sinX);
        println!("{}", x);
        let mut z = 90 as f32;
        let x = (y * cosY - z * sinY);
        let y = (z * cosY + y * sinY);
    }
}
