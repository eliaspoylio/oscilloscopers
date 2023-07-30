use std::f64::consts::PI;

use crate::vector::{draw_points_float, Point};

use rand::{thread_rng, Rng, SeedableRng};
use rand_chacha;

const SIZE_F: f32 = crate::SIZE_F;

#[derive(Clone, Copy)]
struct Point3D {
    coord: [f64; 5],
    trans: [f64; 5],
    // TODO brightness?
}

#[derive(Clone, Copy)]
struct Pos {
    x: f64,
    y: f64,
    z: f64,
}

impl Pos {
    fn init() -> Pos {
        Pos {
            x: 0.,
            y: 100.,
            z: 0.,
        }
    }
}

fn d2r(degrees: f64) -> f64 {
    let conversion = 0.1745327; // --> 3.14159 / 180.0;
    degrees * conversion
}

fn atan(x: f64, y: f64) -> f64 {
    if x == 0. {
        0.
    } else {
        let mut angle = (y / x).atan();
        if x < 0. {
            angle = PI + angle;
        }
        angle
    }
}

fn vector_matrix_mult(rpt: &mut [f64; 5], ppt: [f64; 5], a: [[f64; 4]; 4]) -> () {
    let mut val = 0.;

    for i in 0..4 {
        val = 0.;
        for j in 0..4 {
            val += ppt[j] * a[j][i];
        }
        rpt[i] = val;
    }

    rpt[0] = rpt[0] * val;
    rpt[1] = rpt[1] * val;
    rpt[2] = rpt[2] * val;
    rpt[3] = 1.;
}

fn calculate_transformation(eyex: f64, eyey: f64, eyez: f64) -> [[f64; 4]; 4] {
    //let mut rng = thread_rng();
    let mut t1 = &mut [[0.; 4]; 4];
    let mut t2 = &mut [[0.; 4]; 4];

    let r1 = (eyex * eyex + eyey * eyey).sqrt();
    let stheta = eyex / r1;
    let ctheta = eyey / r1;
    make_identity(&mut t1);
    t1[0][0] = ctheta;
    t1[0][1] = stheta;
    t1[1][0] = -stheta;
    t1[1][1] = ctheta;

    let r2 = (eyex * eyex + eyey * eyey + eyez * eyez).sqrt();
    let sphi = -r1;
    let cphi = -eyez / r2;
    make_identity(&mut t2);
    t2[1][1] = cphi;
    t2[1][2] = sphi;
    t2[2][1] = -sphi;
    t2[2][2] = cphi;

    let t = matrix_matrix_mult(t1, t2);
    t
}

fn make_identity(m: &mut [[f64; 4]; 4]) -> () {
    for i in 0..4 {
        for j in 0..4 {
            if i == j {
                m[i][j] = 1.
            } else {
                m[i][j] = 0.
            }
        }
    }
}

fn matrix_matrix_mult(a: &mut [[f64; 4]; 4], b: &mut [[f64; 4]; 4]) -> [[f64; 4]; 4] {
    let mut r = [[0.; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            let mut val = 0.;
            for k in 0..4 {
                val += a[i][k] * b[k][j];
            }
            r[i][j] = val;
        }
    }
    r
}

pub fn landscape(frames: i32) -> Vec<(f32, f32)> {
    let mut scene: Vec<(f32, f32)> = Vec::new();

    ////////////////////////////////////////////

    const MAPDIM: usize = 145; //336;
    const PIXEL_SPACING: usize = 1;
    let mut eye_x = 30.;
    let mut eye_y = 60.;
    let eye_z = -360.;

    //let center_x = 0;

    let mut count = 0;

    let heightmap: &mut [[f64; MAPDIM * 3]] = &mut [[0.; MAPDIM * 3]; MAPDIM * 3];
    //let mut rng = thread_rng();
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(8);
    const D: usize = ((MAPDIM * 2) / PIXEL_SPACING) + 1;
    let point_3d = &mut [Pos::init(); D];

    for x in 0..MAPDIM * 2 {
        for y in 0..MAPDIM * 2 {
            heightmap[x][y] = rng.gen();
        }
    }

    for _idx in 0..2 {
        for x in 1..(MAPDIM * 2 - 1) {
            for y in 1..(MAPDIM * 2 - 1) {
                heightmap[x][y] = (heightmap[x - 1][y - 1]
                    + heightmap[x - 1][y + 1]
                    + heightmap[x + 1][y - 1]
                    + heightmap[x + 1][y + 1]
                    + heightmap[x][y - 1]
                    + heightmap[x][y + 1]
                    + heightmap[x - 1][y]
                    + heightmap[x + 1][y])
                    / 5.2;
            }
        }
    }

    let mut cc: usize = 0;
    let mut countx: usize = 20;
    //let mut county: usize = 0;

    for x in (-(MAPDIM as i32)..(MAPDIM as i32)).step_by(PIXEL_SPACING) {
        let mut county = 20;
        for z in (-(MAPDIM as i32)..(MAPDIM as i32)).step_by(PIXEL_SPACING) {
            let y = heightmap[countx][county];
            point_3d[cc].x = x as f64 * 3.;
            point_3d[cc].y = y;
            point_3d[cc].z = z as f64;

            //cc += 1;
            county += 1;
        }
        cc += 1;
        countx += 1;
    }

    let mut points = [[Point3D {
        coord: [0.; 5],
        trans: [0.; 5],
    }; 50]; 50];

    countx = points.len() - 10;

    for i in 0..points.len() {
        countx += 1;
        let mut county = points.len() - 10;
        for j in 0..points.len() {
            points[i][j].coord[0] = i as f64 - 20.;
            points[i][j].coord[1] = j as f64 - 20.;
            points[i][j].coord[3] = 0.8;

            let y = heightmap[countx][county];
            // brightness?
            points[i][j].coord[2] = y * 0.06;
            points[i][j].coord[4] = points[i][j].coord[2];

            county += 1;
        }
    }
    countx = 31;
    let mut county = 31;

    //let mut xcounter = 1.;
    //let mut ycounter = 1.;
    let mut xcountermover = 0.4;
    let mut ycountermover = 0.4;

    //const PI: f64 = 3.14159;
    let d_theta = 0.005; // rotation speed
                         //let d_phi = PI / 8.;

    let mut s_count = 0;

    for _i in 1..frames {
        let mut frame: Vec<Point> = Vec::new();

        count += 1;

        let mut theta = atan(d2r(eye_x), d2r(eye_y));
        let r1 = (eye_x * eye_x + eye_y * eye_y).sqrt();
        //let r2 = (eye_x * eye_x + eye_y * eye_y + eye_z * eye_z).sqrt();

        //let phi = atan(r1,eye_z);

        theta = theta - d_theta;

        eye_x = r1 * theta.cos();
        eye_y = r1 * theta.sin();

        let t = calculate_transformation(eye_x, eye_y, eye_z);

        if count % 5 == 0 {
            countx += xcountermover as usize;
            county += ycountermover as usize;
        }

        if countx >= MAPDIM - 145 {
            xcountermover = -xcountermover;
            countx = MAPDIM - 145;
        }

        if countx <= 30 {
            xcountermover = -xcountermover;
            countx = 30;
        }

        let mut xcounter = countx as f64;

        for i in 0..points.len() {
            xcounter += 0.5;

            if county >= MAPDIM - 145 {
                ycountermover = -ycountermover;
                county = MAPDIM - 145;
            }

            if county <= 30 {
                ycountermover = -ycountermover;
                county = 30;
            }

            let mut ycounter = county as f64;

            for j in 0..points.len() {
                let y = heightmap[xcounter as usize][ycounter as usize];

                // TODO brigthness?

                let mut dy = 0.;
                //if s_count == points.len() { s_count = 0 };
                //if i == s_count {
                //dy = (2. * PI* 0.5 * _i as f64).sin() * 640000000.;
                //}
                //s_count += 1;

                points[i][j].coord[2] = y * (0.06 + dy);
                points[i][j].coord[4] = points[i][j].coord[2] + dy;

                ycounter += 1.;
            }
        }

        for i in 0..points.len() {
            for j in 0..points.len() {
                vector_matrix_mult(&mut points[i][j].trans, points[i][j].coord, t);
            }
        }

        let distance = 12.;

        for i in 10..points.len() - 10 {
            for j in 10..points.len() - 10 {
                // TODO: brightness?

                let current_x = distance * points[i][j].trans[0] * 0.5;
                let current_y = distance * points[i][j].trans[1] * 0.5;

                let ix = (current_x) as f32;
                let iy = (current_y - 50.) as f32;

                //if s_count == points.len() { s_count = 0 };
                //if j == s_count {
                //    iy += (2. * PI as f32 * 60. * i as f32).sin() * 8.;
                //}
                //s_count += 1;

                //if ix > -SIZE_F && ix < SIZE_F && iy > -SIZE_F && iy < SIZE_F {
                frame.push(Point::new(ix, iy));
                frame.push(Point::new(ix + 1., iy));
                frame.push(Point::new(ix, iy + 1.));
                //}
            }
        }

        let frame_points = draw_points_float(1. / 50., frame, 1);
        for point in frame_points {
            scene.push(point);
        }
    }

    /////////////////////////////////////////////

    scene
}
