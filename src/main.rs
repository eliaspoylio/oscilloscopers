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

fn main() -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let amplitude = i16::MAX as f32;
    let mut writer = hound::WavWriter::create("float2.wav", spec).unwrap();

    // /////////////////////////

  
    let mut obj1 = [&mut VPoint::new(0., -40.), &mut VPoint::new(0., -60.)];
    let mut obj2 = [&mut VPoint::new(0., -20.), &mut VPoint::new(0., -40.)];
    let mut obj3 = [&mut VPoint::new(0., 0.), &mut VPoint::new(0., -20.)];

    let mut cube: Vec<(f32, f32)> = Vec::new();

    for i in 0..1000 {
        let mut lines = vec![];
        obj1
        .iter_mut()
        .inspect(|o| {
        })
        .for_each(
            |o|
            if o.x > -SIZE_F {
                o.scroll(-0.7, 0.)
            }
            else { o.scroll(2. * SIZE_F, 0.) }
        );
        obj2
        .iter_mut()
        .inspect(|o| {
        })
        .for_each(
            |o|
            if o.x > -SIZE_F {
                o.scroll(-0.5, 0.)
            }
            else { o.scroll(2. * SIZE_F, 0.) }
        );
        obj3
        .iter_mut()
        .inspect(|o| {
        })
        .for_each(
            |o|
            if o.x > -SIZE_F {
                o.scroll(-0.3, 0.)
            }
            else { o.scroll(2. * SIZE_F, 0.) }
        );
        let linesarr: [Vec<VPoint>; 3] = [
            create_line_float(*obj1[0], *obj1[1], 1.),
            create_line_float(*obj2[0], *obj2[1], 1.),
            create_line_float(*obj3[0], *obj3[1], 1.),
        ];
        for line in linesarr {
            for l in line {
                lines.push(l);
            }
        }
        let cli = draw_points_float(1. / 24., lines, 8);

        for cl in cli {
            cube.push(cl);
        }
    }

    for l in cube {
        writer.write_sample((l.0 * amplitude) as i16).unwrap();
        writer.write_sample((l.1 * amplitude) as i16).unwrap();
    }

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
