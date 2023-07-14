use hound;
use std::i16;

//mod raster;
mod matrix;
mod vector;
use crate::vector::{create_line_float, draw_points_float, Point, VertexF};

use rand::{thread_rng, Rng};

const SAMPLE_RATE: u32 = 96000;
const SAMPLE_RATE_F: f32 = SAMPLE_RATE as f32;
const SIZE: i32 = 100;

const CANVAS: i32 = 200;
const DISTANCE: i32 = 50;

const SIZE_F: f32 = 100.;
const CANVAS_F: f32 = 200.;
const DISTANCE_F: f32 = 50.;

#[derive(Clone, Copy)]
struct Star {
    x: f32,
    y: f32,
    z: i32,
    speed: i32,
    bright: f32
}

impl Star {
    fn empty() -> Star {
        Star {x: 0., y: 0., z: 0, speed: 0, bright: 0.}
    }

    fn init_star(&mut self) {
        let mut rng = thread_rng();
        let sx = (SIZE_F * 0.7) as i32;
        let sy = (SIZE_F * 0.7) as i32;
    
        let x = rng.gen_range(-sx..sx) as f32;
        let y = rng.gen_range(-sy..sy) as f32;

        /*
        let star = Star {
            x: (x.cos() * y * 2500.),
            y: (x.sin() * y * 2500.),
            z: rng.gen_range(80..160) << 6,
            bright: 0.001
        };
        */
        //star
        self.x = x.cos() * y * SIZE_F;
        self.y = x.sin() * y * SIZE_F;
        self.z = rng.gen_range(80..160) << 6;
        self.speed = rng.gen_range(10..20);
        self.bright = 0.01;
    }
}

fn main() -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let amplitude = i16::MAX as f32;
    let mut writer = hound::WavWriter::create("stars.wav", spec).unwrap();
    let mut scene: Vec<(f32, f32)> = Vec::new();

    ////////////////////////////////////////////
    
    const MAX_STARS: usize = 800;
    let stars = &mut [Star::empty(); MAX_STARS];
    let center_x = 0;//SIZE >> 1;
    let center_y = 0;//SIZE >> 1;

    for star in stars.iter_mut() {
        star.init_star();
    }

    for _i in 1..1000 {
        let mut frame: Vec<Point> = Vec::new();
        for star in stars.iter_mut() {
            star.z -= star.speed;

            if star.z <= 0 { star.init_star() };

            let ix = (star.x / star.z as f32) + (center_x) as f32;
            let iy = (star.y / star.z as f32) + (center_y) as f32;

            star.bright += 0.0095;
            //println!("{}", star.bright.ceil() as i32);

            if ix > -SIZE_F && ix < SIZE_F && iy > -SIZE_F && iy < SIZE_F {
                for _f in 0..star.bright.ceil() as i32 {
                    frame.push(Point::new(ix, iy));   
                }
            }
            else {
                star.init_star();
            }
        }
        let frame_points = draw_points_float(1. / 24., frame, 2);
        for point in frame_points {
            scene.push(point);
        }
    }

    /////////////////////////////////////////////

    for f in scene {
        writer.write_sample((f.0 * amplitude) as i16).unwrap();
        writer.write_sample((f.1 * amplitude) as i16).unwrap();
    }

    println!("Length: {}", writer.len());
    println!("Duration: {}", writer.duration() / spec.sample_rate);
    writer.finalize()
}

/*
use hound;
use matrix::Triangle;
use std::collections::LinkedList;
use std::f32::consts::PI;
use std::i16;

mod raster;
mod matrix;
mod vector;
use crate::vector::{create_line_float, draw_points_float, Point, VertexF};

const SAMPLE_RATE: u32 = 96000;
const SAMPLE_RATE_F: f32 = SAMPLE_RATE as f32;
const SIZE: i32 = 100;

const CANVAS: i32 = 200;
const DISTANCE: i32 = 50;

const SIZE_F: f32 = 100.;
const CANVAS_F: f32 = 200.;
const DISTANCE_F: f32 = 50.;

fn triangle_to_float(t: &Triangle) -> f32 {
    t.p[0].z + t.p[1].z + t.p[2].z / 3.0
}

fn random() -> f32 {
    let num1 = vec![2, 3];
    let address1 = &num1 as *const Vec<i32>;
    let number1 = address1 as i32 as f32;
    number1
}

fn create_landscape() -> Vec<matrix::Triangle> {
    let mut tris: Vec<matrix::Triangle> = Vec::new();
    for n in 1..101 {
        let f = n as f32;
        let triangle = matrix::Triangle {
            p: {
                [
                    matrix::Vec_3d::new(0.1*f, 0.2*f, 100.),
                    matrix::Vec_3d::new(0.2*f, -0.3*f, 100.),
                    matrix::Vec_3d::new(-0.3*f, -0.4*f, 100.),
                ]
            },
        };
        tris.push(triangle);
    }
    tris
}

fn main() -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let amplitude = i16::MAX as f32;
    let mut writer = hound::WavWriter::create("3d.wav", spec).unwrap();
    let mut scene: Vec<(f32, f32)> = Vec::new();

    let mut mat_proj = matrix::Mat_4x4 { m: [[0.; 4]; 4] }; // Matrix that converts from view space to screen space
    let mut v_camera = matrix::Vec_3d::new(1., 1., 0.); // Location of camera in world space
    let mut look_dir = matrix::Vec_3d::new(0., 0., 0.); // Direction vector along the direction camera points
    let yaw: f32 = 0.; // FPS Camera rotation in XZ plane
    let theta: f32 = 0.; // Spins World transform

    let mut mountains = matrix::Mesh::new();
    //mountains.load_from_object_file("./mountains.obj").unwrap_or(());

    let tris = create_landscape();

    mat_proj.make_projection(90., 1., 0.1, 1000.);

    for _i in 1..100 {
        v_camera.x += 1.0;
        let mut trans = matrix::Mat_4x4::new();
        trans.make_translation(0., 0., 5.);
        let mut world = matrix::Mat_4x4::new();
        world.make_identity();
        // matWorld = Matrix_MultiplyMatrix(matRotZ, matRotX); // Transform by rotation
        world.multiply_matrix(trans);

        // Create "Point At" Matrix for camera
        let up = matrix::Vec_3d::new(0., 1., 0.);
        let mut target = matrix::Vec_3d::new(0., 0., 1.);
        let mut camera_rot = matrix::Mat_4x4::new();
        camera_rot.make_rotation_y(yaw);
        look_dir = matrix::Mat_4x4::matrix_multiply_vector(&camera_rot, target);
        target = matrix::Vec_3d::vector_add(&v_camera, &target);
        let mat_camera = matrix::Mat_4x4::matrix_point_at(&v_camera, &target, &up);

        // Make view matrix from camera
        let view = matrix::Mat_4x4::matrix_quick_inverse(&mat_camera);

        // Store triagles for rastering later
        let mut triangles_to_raster: Vec<matrix::Triangle> = Vec::new();

        // Draw Triangles
        for tri in tris.iter() {
            let mut projected = matrix::Triangle::new();
            let mut transformed = matrix::Triangle::new();
            let mut viewed = matrix::Triangle::new();
            // World Matrix Transform
            transformed.p[0] = matrix::Mat_4x4::matrix_multiply_vector(&world, tri.p[0]);
            transformed.p[1] = matrix::Mat_4x4::matrix_multiply_vector(&world, tri.p[1]);
            transformed.p[2] = matrix::Mat_4x4::matrix_multiply_vector(&world, tri.p[2]);

            // Calculate triangle Normal
            // Get lines either side of triangle
            let line1 = matrix::Vec_3d::vector_sub(&transformed.p[1], &transformed.p[0]);
            let line2 = matrix::Vec_3d::vector_sub(&transformed.p[2], &transformed.p[0]);
            // Take cross product of lines to get normal to triangle surface
            let mut normal = matrix::Vec_3d::cross_product(&line1, &line2);
            // You normally need to normalise a normal!
            normal.normalize_self();

            // Get Ray from triangle to camera
            let v_camera_ray = matrix::Vec_3d::vector_sub(&transformed.p[0], &v_camera);

            // If ray is aligned with normal, then triangle is visible
            if matrix::Vec_3d::dot_product(&normal, &v_camera_ray) < 0. {
                // TODO: "color" & illumination

                // Convert World Space --> View Space
                viewed.p[0] = matrix::Mat_4x4::matrix_multiply_vector(&view, transformed.p[0]);
                viewed.p[1] = matrix::Mat_4x4::matrix_multiply_vector(&view, transformed.p[1]);
                viewed.p[2] = matrix::Mat_4x4::matrix_multiply_vector(&view, transformed.p[2]);

                // Clip Viewed Triangle against near plane, this could form two additional
                // additional triangles.
                let mut n_clipped_triangles = 0;
                let mut clipped: [matrix::Triangle; 2] =
                    [matrix::Triangle::new(), matrix::Triangle::new()];
                // https://www.reddit.com/r/rust/comments/zi5e03/cannot_borrow_as_mutable_more_than_once_at_a_time/
                let mut a = std::mem::take(&mut clipped[0]);
                let mut b = std::mem::take(&mut clipped[1]);
                n_clipped_triangles = matrix::Triangle::triangle_clip_against_plane(
                    matrix::Vec_3d::new(0., 0., 0.1),
                    matrix::Vec_3d::new(0., 0., 0.1),
                    &viewed,
                    &mut a,
                    &mut b,
                );
                clipped[0] = a;
                clipped[1] = b;

                // We may end up with multiple triangles form the clip, so project as
                // required
                for n in 0..n_clipped_triangles {
                    // Project triangles from 3D --> 2D
                    projected.p[0] =
                        matrix::Mat_4x4::matrix_multiply_vector(&mat_proj, clipped[n].p[0]);
                    projected.p[1] =
                        matrix::Mat_4x4::matrix_multiply_vector(&mat_proj, clipped[n].p[1]);
                    projected.p[2] =
                        matrix::Mat_4x4::matrix_multiply_vector(&mat_proj, clipped[n].p[2]);
                    //triProjected.col = clipped[n].col;

                    // Scale into view, we moved the normalising into cartesian space
                    // out of the matrix.vector function from the previous videos, so
                    // do this manually
                    projected.p[0] = matrix::Vec_3d::vector_div(projected.p[0], projected.p[0].w);
                    projected.p[1] = matrix::Vec_3d::vector_div(projected.p[1], projected.p[1].w);
                    projected.p[2] = matrix::Vec_3d::vector_div(projected.p[2], projected.p[2].w);

                    // X/Y are inverted so put them back
                    projected.p[0].x *= -1.0;
                    projected.p[1].x *= -1.0;
                    projected.p[2].x *= -1.0;
                    projected.p[0].y *= -1.0;
                    projected.p[1].y *= -1.0;
                    projected.p[2].y *= -1.0;

                    // Offset verts into visible normalised space
                    let offset_view = matrix::Vec_3d::new(1., 1., 0.);
                    projected.p[0] = matrix::Vec_3d::vector_add(&projected.p[0], &offset_view);
                    projected.p[1] = matrix::Vec_3d::vector_add(&projected.p[1], &offset_view);
                    projected.p[2] = matrix::Vec_3d::vector_add(&projected.p[2], &offset_view);
                    projected.p[0].x *= 0.5 * SIZE as f32;
                    projected.p[0].y *= 0.5 * SIZE as f32;
                    projected.p[1].x *= 0.5 * SIZE as f32;
                    projected.p[1].y *= 0.5 * SIZE as f32;
                    projected.p[2].x *= 0.5 * SIZE as f32;
                    projected.p[2].y *= 0.5 * SIZE as f32;

                    // Store triangle for sorting
                    triangles_to_raster.push(projected);
                }
            }
        }
        // Sort triangles from back to front
        //println!("{:?}", triangles_to_raster);
        triangles_to_raster.sort_by(|a, b| a.to_float().partial_cmp(&b.to_float()).unwrap());

        // Loop through all transformed, viewed, projected, and sorted triangles
        for tri_to_raster in triangles_to_raster.iter() {
            // Clip triangles against all four screen edges, this could yield
            // a bunch of triangles, so create a queue that we traverse to
            //  ensure we only test new triangles generated against planes
            let mut clipped: [matrix::Triangle; 2] =
                [matrix::Triangle::new(), matrix::Triangle::new()];
            let mut triangles: LinkedList<matrix::Triangle> = LinkedList::new();

            // Add initial triangle
            triangles.push_back(*tri_to_raster);
            let mut new_triangles: i32 = 1;

            for p in 0..4 {
                let mut tris_to_add = 0;
                while new_triangles > 0 {
                    let test = triangles.front().unwrap().to_owned();
                    triangles.pop_front();
                    new_triangles = -1;

                    // Clip it against a plane. We only need to test each
                    // subsequent plane, against subsequent new triangles
                    // as all triangles after a plane clip are guaranteed
                    // to lie on the inside of the plane. I like how this
                    // comment is almost completely and utterly justified
                    match p {
                        0 => {
                            let mut a = std::mem::take(&mut clipped[0]);
                            let mut b = std::mem::take(&mut clipped[1]);
                            tris_to_add = matrix::Triangle::triangle_clip_against_plane(
                                matrix::Vec_3d::new(0., 0., 0.),
                                matrix::Vec_3d::new(0., 1., 0.),
                                &test,
                                &mut a,
                                &mut b,
                            );
                            clipped[0] = a;
                            clipped[1] = b;
                            
                        }
                        1 => {
                            let mut a = std::mem::take(&mut clipped[0]);
                            let mut b = std::mem::take(&mut clipped[1]);
                            tris_to_add = matrix::Triangle::triangle_clip_against_plane(
                                matrix::Vec_3d::new(0., SIZE_F - 1., 0.),
                                matrix::Vec_3d::new(0., -1., 0.),
                                &test,
                                &mut a,
                                &mut b,
                            );
                            clipped[0] = a;
                            clipped[1] = b;
                            
                        }
                        2 => {
                            let mut a = std::mem::take(&mut clipped[0]);
                            let mut b = std::mem::take(&mut clipped[1]);
                            tris_to_add = matrix::Triangle::triangle_clip_against_plane(
                                matrix::Vec_3d::new(0., 0., 0.),
                                matrix::Vec_3d::new(1., 0., 0.),
                                &test,
                                &mut a,
                                &mut b,
                            );
                            clipped[0] = a;
                            clipped[1] = b;
                            
                        }
                        3 => {
                            let mut a = std::mem::take(&mut clipped[0]);
                            let mut b = std::mem::take(&mut clipped[1]);
                            tris_to_add = matrix::Triangle::triangle_clip_against_plane(
                                matrix::Vec_3d::new(SIZE_F - 1., 0., 0.),
                                matrix::Vec_3d::new(-1., 0., 0.),
                                &test,
                                &mut a,
                                &mut b,
                            );
                            clipped[0] = a;
                            clipped[1] = b;
                            
                        }
                        _ => ()
                    }
                    // Clipping may yield a variable number of triangles, so
					// add these new ones to the back of the queue for subsequent
					// clipping against next planes
                    for w in 0..tris_to_add {
                        triangles.push_back(clipped[w]);   
                    }
                
                }
                new_triangles = triangles.len() as i32;
            }
            // Draw the transformed, viewed, clipped, projected, sorted, clipped triangles
            for t in triangles.iter() {
                let tr = vector::draw_wireframe_triangle(
                    vector::Point::new(t.p[0].x, t.p[0].y),
                    vector::Point::new(t.p[1].x, t.p[1].y),
                    vector::Point::new(t.p[2].x, t.p[2].y),
                    0.1
                );

                let cli = draw_points_float(1. / 24., tr, 1);
                for cl in cli {
                    scene.push(cl);
                }
            };
        }
    }
    for c in scene {
        writer.write_sample((c.0 * amplitude) as i16).unwrap();
        writer.write_sample((c.1 * amplitude) as i16).unwrap();
    }

    println!("Length: {}", writer.len());
    println!("Duration: {}", writer.duration() / spec.sample_rate);
    writer.finalize()
}
*/
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
////////////////////////////////////////////////////////////////////////
/*
use hound;
use std::f32::consts::PI;
use std::i16;

//mod raster;
mod vector;
mod matrix;
use crate::vector::{create_line_float, draw_points_float, Point, VertexF};

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


    let mut obj1 = [&mut Point::new(0., -40.), &mut Point::new(0., -60.)];
    let mut obj2 = [&mut Point::new(0., -20.), &mut Point::new(0., -40.)];
    let mut obj3 = [&mut Point::new(0., 0.), &mut Point::new(0., -20.)];

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
        let linesarr: [Vec<Point>; 3] = [
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
*/
