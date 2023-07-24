use std::f32::consts::PI;

use crate::vector::{create_line_float, draw_points_float, project_vertex_f, Point, VertexF};


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

        let array: [Vec<Point>; 12] = [
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

            create_line_float(Point::new(v_af.x, v_af.y), Point::new(v_bf.x, v_bf.y), 1.),
            create_line_float(Point::new(v_bf.x, v_bf.y), Point::new(v_cf.x, v_cf.y), 1.),
            create_line_float(Point::new(v_cf.x, v_cf.y), Point::new(v_df.x, v_df.y), 1.),
            create_line_float(Point::new(v_df.x, v_df.y), Point::new(v_af.x, v_af.y), 1.),

            create_line_float(Point::new(v_ab.x, v_ab.y), Point::new(v_bb.x, v_bb.y), 1.),
            create_line_float(Point::new(v_bb.x, v_bb.y), Point::new(v_cb.x, v_cb.y), 1.),
            create_line_float(Point::new(v_cb.x, v_cb.y), Point::new(v_db.x, v_db.y), 1.),
            create_line_float(Point::new(v_db.x, v_db.y), Point::new(v_ab.x, v_ab.y), 1.),

            create_line_float(Point::new(v_af.x, v_af.y), Point::new(v_ab.x, v_ab.y), 1.),
            create_line_float(Point::new(v_bf.x, v_bf.y), Point::new(v_bb.x, v_bb.y), 1.),
            create_line_float(Point::new(v_cf.x, v_cf.y), Point::new(v_cb.x, v_cb.y), 1.),
            create_line_float(Point::new(v_df.x, v_df.y), Point::new(v_db.x, v_db.y), 1.),
        ];

        let mut lines: Vec<Point> = vec![];
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
    cube
}