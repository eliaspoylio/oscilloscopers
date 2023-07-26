use std::f32::consts::PI;

use crate::vector::{create_line_float, draw_points_float, project_vertex_f, Point, VertexF};

const STEP: f32 = 1.;

pub fn cube() -> Vec<(f32, f32)> {
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

    for _i in 1..2000 {
        let vertices = [
            &mut v_af, &mut v_bf, &mut v_cf, &mut v_df, &mut v_ab, &mut v_bb, &mut v_cb, &mut v_db,
        ];
        for v in vertices {
            v.rotate(PI / 180., PI / 180., PI / 180.)
        }

        let array: [Vec<Point>; 12] = [
            create_line_float(Point::new(v_af.x, v_af.y), Point::new(v_bf.x, v_bf.y), STEP),
            create_line_float(Point::new(v_bf.x, v_bf.y), Point::new(v_cf.x, v_cf.y), STEP),
            create_line_float(Point::new(v_cf.x, v_cf.y), Point::new(v_df.x, v_df.y), STEP),
            create_line_float(Point::new(v_df.x, v_df.y), Point::new(v_af.x, v_af.y), STEP),
            create_line_float(Point::new(v_ab.x, v_ab.y), Point::new(v_bb.x, v_bb.y), STEP),
            create_line_float(Point::new(v_bb.x, v_bb.y), Point::new(v_cb.x, v_cb.y), STEP),
            create_line_float(Point::new(v_cb.x, v_cb.y), Point::new(v_db.x, v_db.y), STEP),
            create_line_float(Point::new(v_db.x, v_db.y), Point::new(v_ab.x, v_ab.y), STEP),
            create_line_float(Point::new(v_af.x, v_af.y), Point::new(v_ab.x, v_ab.y), STEP),
            create_line_float(Point::new(v_bf.x, v_bf.y), Point::new(v_bb.x, v_bb.y), STEP),
            create_line_float(Point::new(v_cf.x, v_cf.y), Point::new(v_cb.x, v_cb.y), STEP),
            create_line_float(Point::new(v_df.x, v_df.y), Point::new(v_db.x, v_db.y), STEP),
        ];

        let mut lines: Vec<Point> = vec![];
        for line in array {
            for l in line {
                lines.push(l);
            }
        }
        // Heuristics to fit for 50 fps
        let mut c = -1;
        lines.retain(|_| { c += 1; return c % 2 == 0 });

        let cli = draw_points_float(1. / 50., lines, 10);
        for cl in cli {
            cube.push(cl);
        }
    }
    cube
}
