use hound;
use std::f32::consts::PI;
use std::i16;

mod raster;
mod vector;
use crate::vector::{create_line_float, draw_points_float, project_vertex_f, VPoint, VertexF};

const SAMPLE_RATE: u32 = 96000;
const SAMPLE_RATE_F: f32 = SAMPLE_RATE as f32;
const SIZE: i32 = 100;
const CANVAS: i32 = 200;
const DISTANCE: i32 = 50;

const SIZE_F: f32 = 100.;
const CANVAS_F: f32 = 200.;
const DISTANCE_F: f32 = 50.;

fn rotate(angle_x: f32, angle_y: f32, v: &mut VertexF) {
    let sin_x = angle_x.sin();
    let cos_x = angle_x.cos();
    let sin_y = angle_y.sin();
    let cos_y = angle_y.cos();
    let x = v.x as f32;
    let y = v.y as f32;
    let mut z = v.z as f32;
    v.x = x * cos_x - z * sin_x;
    v.z = z * cos_x + x * sin_x;
    z = v.z as f32;
    v.y = y * cos_y - z * sin_y;
    v.z = z * cos_y + y * sin_y;
}

fn rotate_x(angle: f32, vertice: &mut VertexF) {
    let sin = angle.sin();
    let cos = angle.cos();
    let y = vertice.y.clone();
    let z = vertice.z.clone();
    vertice.y = y * cos - z * sin;
    vertice.z = y * sin + z * cos;
}

fn rotate_y(angle: f32, vertice: &mut VertexF) {
    let sin = angle.sin();
    let cos = angle.cos();
    vertice.x = vertice.z * sin + vertice.x * cos;
    vertice.z = vertice.z * cos - vertice.x * sin;
}

fn rotate_z(angle: f32, vertice: &mut VertexF) {
    let sin = angle.sin();
    let cos = angle.cos();
    let x = vertice.x.clone();
    vertice.x = x * cos - vertice.y * sin;
    vertice.y = x * sin + vertice.y * cos;
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

    // /////////////////////////

    let mut cube: Vec<(f32, f32)> = Vec::new();
    let mut v_af = VertexF::new(-20., 20., 0.);
    let mut v_bf = VertexF::new(20., 20., 0.);
    let mut v_cf = VertexF::new(20., -20., 0.);
    let mut v_df = VertexF::new(-20., -20., 0.);

    let mut v_ab = VertexF::new(-20., 20., 40.);
    let mut v_bb = VertexF::new(20., 20., 40.);
    let mut v_cb = VertexF::new(20., -20., 40.);
    let mut v_db = VertexF::new(-20., -20., 40.);

    let vertices = [
        &mut v_af, &mut v_bf, &mut v_cf, &mut v_df, &mut v_ab, &mut v_bb, &mut v_cb, &mut v_db,
    ];
    for v in vertices {
        //rotate(PI / 4., (2. as f32).sqrt().atan(), v);
    }

    for _i in 1..1000 {
        let vertices = [
            &mut v_af, &mut v_bf, &mut v_cf, &mut v_df, &mut v_ab, &mut v_bb, &mut v_cb, &mut v_db,
        ];
        for v in vertices {
            //rotate_x(PI / 180., v);
            //rotate_y(PI / 180., v);
            //rotate_z(PI / 180., v);
            //rotate(PI as f32, 0., v)
            v.rotate(PI / 180., PI / 180., PI / 180.)
        }

        let array: [Vec<VPoint>; 12] = [
            /* 
            // The front face
            create_line_float(project_vertex_f(&mut v_af), project_vertex_f(&mut v_bf), 1.),
            create_line_float(project_vertex_f(&mut v_bf), project_vertex_f(&mut v_cf), 1.),
            create_line_float(project_vertex_f(&mut v_cf), project_vertex_f(&mut v_df), 1.),
            create_line_float(project_vertex_f(&mut v_df), project_vertex_f(&mut v_af), 1.),
            // The back face
            create_line_float(project_vertex_f(&mut v_ab), project_vertex_f(&mut v_bb), 1.),
            create_line_float(project_vertex_f(&mut v_bb), project_vertex_f(&mut v_cb), 1.),
            create_line_float(project_vertex_f(&mut v_cb), project_vertex_f(&mut v_db), 1.),
            create_line_float(project_vertex_f(&mut v_db), project_vertex_f(&mut v_ab), 1.),
            // The front-to-back edges
            create_line_float(project_vertex_f(&mut v_af), project_vertex_f(&mut v_ab), 1.),
            create_line_float(project_vertex_f(&mut v_bf), project_vertex_f(&mut v_bb), 1.),
            create_line_float(project_vertex_f(&mut v_cf), project_vertex_f(&mut v_cb), 1.),
            create_line_float(project_vertex_f(&mut v_df), project_vertex_f(&mut v_db), 1.),
            */

            create_line_float(VPoint::new(v_af.x, v_af.y), VPoint::new(v_bf.x, v_bf.y), 1.),
            create_line_float(VPoint::new(v_bf.x, v_bf.y), VPoint::new(v_cf.x, v_cf.y), 1.),
            create_line_float(VPoint::new(v_cf.x, v_cf.y), VPoint::new(v_df.x, v_df.y), 1.),
            create_line_float(VPoint::new(v_df.x, v_df.y), VPoint::new(v_af.x, v_af.y), 1.),

            create_line_float(VPoint::new(v_ab.x, v_ab.y), VPoint::new(v_bb.x, v_bb.y), 1.),
            create_line_float(VPoint::new(v_bb.x, v_bb.y), VPoint::new(v_cb.x, v_cb.y), 1.),
            create_line_float(VPoint::new(v_cb.x, v_cb.y), VPoint::new(v_db.x, v_db.y), 1.),
            create_line_float(VPoint::new(v_db.x, v_db.y), VPoint::new(v_ab.x, v_ab.y), 1.),

            create_line_float(VPoint::new(v_af.x, v_af.y), VPoint::new(v_ab.x, v_ab.y), 1.),
            create_line_float(VPoint::new(v_bf.x, v_bf.y), VPoint::new(v_bb.x, v_bb.y), 1.),
            create_line_float(VPoint::new(v_cf.x, v_cf.y), VPoint::new(v_cb.x, v_cb.y), 1.),
            create_line_float(VPoint::new(v_df.x, v_df.y), VPoint::new(v_db.x, v_db.y), 1.),
        ];

        let mut lines: Vec<VPoint> = vec![];
        for line in array {
            for l in line {
                lines.push(l);
            }
        }

        let cli = draw_points_float(1. / 24., lines, 8);
        for cl in cli {
            cube.push(cl);
        }
    }

    for c in cube {
        writer.write_sample((c.0 * amplitude) as i16).unwrap();
        writer.write_sample((c.1 * amplitude) as i16).unwrap();
    }

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
    fn test_trig() {}
}
