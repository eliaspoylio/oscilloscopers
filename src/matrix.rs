//https://github.com/OneLoneCoder/Javidx9/blob/master/ConsoleGameEngine/BiggerProjects/Engine3D/OneLoneCoder_olcEngine3D_Part3.cpp
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct Vec_3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32, // for matrix vector multiplication
}

impl Vec_3d {
    fn default() -> Vec_3d {
        Vec_3d {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 1.,
        }
    }

    pub fn new(x: f32, y: f32, z: f32) -> Vec_3d {
        Vec_3d {
            x: x,
            y: y,
            z: z,
            w: 1.,
        }
    }

    pub fn vector_add(v1: &Vec_3d, v2: &Vec_3d) -> Vec_3d {
        Vec_3d {
            x: (v1.x + v2.x),
            y: (v1.y + v2.y),
            z: (v1.z + v2.z),
            w: (1.),
        }
    }

    pub fn vector_sub(v1: &Vec_3d, v2: &Vec_3d) -> Vec_3d {
        Vec_3d {
            x: (v1.x - v2.x),
            y: (v1.y - v2.y),
            z: (v1.z - v2.z),
            w: (1.),
        }
    }

    pub fn vector_mul(v1: &Vec_3d, k: f32) -> Vec_3d {
        Vec_3d {
            x: (v1.x * k),
            y: (v1.y * k),
            z: (v1.z * k),
            w: (1.),
        }
    }

    pub fn vector_div(v1: Vec_3d, k: f32) -> Vec_3d {
        Vec_3d {
            x: (v1.x / k),
            y: (v1.y / k),
            z: (v1.z / k),
            w: (1.),
        }
    }

    pub fn dot_product(v1: &Vec_3d, v2: &Vec_3d) -> f32 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    fn length(v1: &Vec_3d) -> f32 {
        Vec_3d::dot_product(&v1, &v1).sqrt()
    }

    pub fn cross_product(v1: &Vec_3d, v2: &Vec_3d) -> Vec_3d {
        Vec_3d {
            x: (v1.y * v2.z - v1.z * v2.y),
            y: (v1.z * v2.x - v1.x * v2.z),
            z: (v1.x * v2.y - v1.y * v2.x),
            w: (1.),
        }
    }

    pub fn normalize(v1: &Vec_3d) -> Vec_3d {
        let l = Vec_3d::length(v1);
        Vec_3d {
            x: (v1.x / l),
            y: (v1.y / l),
            z: (v1.z / l),
            w: (1.),
        }
    }

    pub fn normalize_self(&mut self) {
        let l = Vec_3d::length(&self);
        self.x = self.x / l;
        self.y = self.y / l;
        self.z = self.z / l;
    }

    fn intersect_plane(
        plane_p: &Vec_3d,
        plane_n: &Vec_3d,
        line_start: &Vec_3d,
        line_end: &Vec_3d,
    ) -> Vec_3d {
        let plane_n = Vec_3d::normalize(plane_n);
        let plane_d = -Vec_3d::dot_product(&plane_n, &plane_p);
        let ad = Vec_3d::dot_product(line_start, &plane_n);
        let bd = Vec_3d::dot_product(line_end, &plane_n);
        let t = (-plane_d - ad) / (bd - ad);
        let line_start_to_end = Vec_3d::vector_sub(line_end, line_start);
        let line_to_intersect = Vec_3d::vector_mul(&line_start_to_end, t);
        Vec_3d::vector_add(line_start, &line_to_intersect)
    }

}

#[derive(Debug, Copy, Clone, Default, PartialEq, PartialOrd)]
pub struct Triangle {
    pub p: [Vec_3d; 3],
}

impl Triangle {
    pub fn new() -> Triangle {
        Triangle { p: [Vec_3d::default(); 3] }
    }

    fn replace(&mut self, new: &Triangle) {
        self.p = new.p;
    }

    pub fn to_float(&self) -> f32 {
        self.p[0].z + self.p[1].z + self.p[2].z / 3.0
    }

    pub fn triangle_clip_against_plane(
        plane_p: Vec_3d,
        plane_n: Vec_3d,
        in_tri: &Triangle,
        out_tri1: &mut Triangle,
        out_tri2: &mut Triangle,
    ) -> usize {
        // Make sure plane normal is indeed normal
        let plane_n = Vec_3d::normalize(&plane_n);

        // Return signed shortest distance from point to plane, plane normal must be normalised
        let dist = |p: &Vec_3d| {
            let n = Vec_3d::normalize(p);
            plane_n.x * p.x + plane_n.y * p.y + plane_n.z * p.z
                - Vec_3d::dot_product(&plane_n, &plane_p)
        };

        // Create two temporary storage arrays to classify points either side of plane
        // If distance sign is positive, point lies on "inside" of plane
        let mut inside_points = [(); 3].map(|_| Vec_3d::default());
        let mut outside_points = [(); 3].map(|_| Vec_3d::default());
        let mut inside_point_count: usize = 0;
        let mut outside_point_count: usize = 0;

        // Get signed distance of each point in triangle to plane
        let d0 = dist(&in_tri.p[0]);
        let d1 = dist(&in_tri.p[1]);
        let d2 = dist(&in_tri.p[2]);

        if d0 >= 0. {
            inside_points[inside_point_count] = in_tri.p[0];
            inside_point_count += 1;
        } else {
            outside_points[outside_point_count] = in_tri.p[0];
            outside_point_count += 1;
        }
        if d1 >= 0. {
            inside_points[inside_point_count] = in_tri.p[1];
            inside_point_count += 1;
        } else {
            outside_points[outside_point_count] = in_tri.p[1];
            outside_point_count += 1;
        }
        if d2 >= 0. {
            inside_points[inside_point_count] = in_tri.p[2];
            inside_point_count += 1;
        } else {
            outside_points[outside_point_count] = in_tri.p[2];
            outside_point_count += 1;
        }

        // Now classify triangle points, and break the input triangle into
        // smaller output triangles if required. There are four possible
        // outcomes...
        match (inside_point_count, outside_point_count) {
            (0, _) => 0, // All points lie on the outside of plane, so clip whole triangle.
            (3, _) => {
                // All points lie on the inside of plane, so do nothing and allow the triangle to simply pass through.
                out_tri1.replace(in_tri);
                1
            },
            (1, 2) => {
                // Triangle should be clipped. As two points lie outside
			    // the plane, the triangle simply becomes a smaller triangle

                // TODO: Copy appearance info to new triangle
                // out_tri1.col =  in_tri.col;

                // The inside point is valid, so keep that...
                out_tri1.p[0] = inside_points[0];

                // but the two new points are at the locations where the 
			    // original sides of the triangle (lines) intersect with the plane
                out_tri1.p[1] = Vec_3d::intersect_plane(&plane_p, &plane_n, &inside_points[0], &outside_points[0]);
                out_tri1.p[2] = Vec_3d::intersect_plane(&plane_p, &plane_n, &inside_points[0], &outside_points[1]);

                // Return the newly formed single triangle
                1
            },
            (2, 1) => {
                // Triangle should be clipped. As two points lie inside the plane,
			    // the clipped triangle becomes a "quad". Fortunately, we can
			    // represent a quad with two new triangles

                // Copy appearance info to new triangles
                // TODO out_tri1.col =  in_tri.col;
                // TODO out_tri2.col =  in_tri.col;
                
                // The first triangle consists of the two inside points and a new
			    // point determined by the location where one side of the triangle
			    // intersects with the plane
                out_tri1.p[0] = inside_points[0];
                out_tri1.p[1] = inside_points[1];
                out_tri1.p[2] = Vec_3d::intersect_plane(&plane_p, &plane_n, &inside_points[0], &outside_points[0]);

                // The second triangle is composed of one of he inside points, a
			    // new point determined by the intersection of the other side of the 
			    // triangle and the plane, and the newly created point above
                out_tri2.p[0] = inside_points[1];
                out_tri2.p[1] = out_tri1.p[2];
                out_tri2.p[2] = Vec_3d::intersect_plane(&plane_p, &plane_n, &inside_points[1], &outside_points[0]);

                // Return two newly formed triangles which form a quad
                2
            },
            (_, _) => {
                0
            }
        }
    }
}

pub struct Mat_4x4 {
    pub m: [[f32; 4]; 4],
}

impl Mat_4x4 {
    pub fn new() -> Mat_4x4 {
        Mat_4x4 { m: [[0.; 4]; 4] }
    }

    pub fn make_identity(&mut self) {
        self.m[0][0] = 1.0;
        self.m[1][1] = 1.0;
        self.m[2][2] = 1.0;
        self.m[3][3] = 1.0;
    }

    pub fn make_rotation_x(&mut self, angle_rad: f32) {
        self.m[0][0] = 1.0;
        self.m[1][1] = angle_rad.cos();
        self.m[1][2] = angle_rad.sin();
        self.m[2][1] = -angle_rad.sin();
        self.m[2][2] = angle_rad.cos();
        self.m[3][3] = 1.0;
    }

    pub fn make_rotation_y(&mut self, angle_rad: f32) {
        self.m[0][0] = angle_rad.cos();
        self.m[0][2] = angle_rad.sin();
        self.m[2][0] = -angle_rad.sin();
        self.m[1][1] = 1.0;
        self.m[2][2] = angle_rad.cos();
        self.m[3][3] = 1.0;
    }

    pub fn make_rotation_z(&mut self, angle_rad: f32) {
        self.m[0][0] = angle_rad.cos();
        self.m[0][1] = angle_rad.sin();
        self.m[1][0] = -angle_rad.sin();
        self.m[1][1] = angle_rad.cos();
        self.m[2][2] = 1.0;
        self.m[3][3] = 1.0;
    }

    pub fn make_translation(&mut self, x: f32, y: f32, z: f32) {
        self.m[0][0] = 1.0;
        self.m[1][1] = 1.0;
        self.m[2][2] = 1.0;
        self.m[3][3] = 1.0;
        self.m[3][0] = x;
        self.m[3][1] = y;
        self.m[3][2] = z;
    }

    pub fn make_projection(&mut self, fov_degree: f32, aspect_ratio: f32, near: f32, far: f32) {
        let fov_rad = 1.0 / (fov_degree * 0.5 / 180. * 3.14159).tan();
        self.m[0][0] = aspect_ratio * fov_rad;
        self.m[1][1] = fov_rad;
        self.m[2][2] = far / (far - near);
        self.m[3][2] = (-far * near) / (far - near);
        self.m[2][3] = 1.0;
        self.m[3][3] = 0.0;
    }

    pub fn multiply_matrix(&mut self, m: Mat_4x4) {
        for c in 0..4 {
            for r in 0..4 {
                self.m[r][c] = self.m[r][0] * m.m[0][c]
                    + self.m[r][1] * m.m[1][c]
                    + self.m[r][2] * m.m[2][c]
                    + self.m[r][3] * m.m[3][c];
            }
        }
    }

    pub fn matrix_multiply_matrix(m1: Mat_4x4, m2: Mat_4x4) -> Mat_4x4 {
        let mut matrix = Mat_4x4 { m: [[0.; 4]; 4] };
        for c in 0..4 {
            for r in 0..4 {
                matrix.m[r][c] = m1.m[r][0] * m2.m[0][c]
                    + m1.m[r][1] * m2.m[1][c]
                    + m1.m[r][2] * m2.m[2][c]
                    + m1.m[r][3] * m2.m[3][c];
            }
        }
        matrix
    }

    pub fn matrix_multiply_vector(m: &Mat_4x4, i: Vec_3d) -> Vec_3d {
        let v = Vec_3d {
            x: i.x * m.m[0][0] + i.y * m.m[1][0] + i.z * m.m[2][0] + i.w * m.m[3][0],
            y: i.x * m.m[0][1] + i.y * m.m[1][1] + i.z * m.m[2][1] + i.w * m.m[3][1],
            z: i.x * m.m[0][2] + i.y * m.m[1][2] + i.z * m.m[2][2] + i.w * m.m[3][2],
            w: i.x * m.m[0][3] + i.y * m.m[1][3] + i.z * m.m[2][3] + i.w * m.m[3][3],
        };
        v
    }

    pub fn matrix_point_at(pos: &Vec_3d, target: &Vec_3d, up: &Vec_3d) -> Mat_4x4 {
        let mut new_forward = Vec_3d::vector_sub(target, pos);
        new_forward = Vec_3d::normalize(&new_forward);

        let a = Vec_3d::vector_mul(&new_forward, Vec_3d::dot_product(up, &new_forward));
        let mut new_up = Vec_3d::vector_sub(up, &a);
        new_up = Vec_3d::normalize(&new_up);

        let new_right = Vec_3d::cross_product(&new_up, &new_forward);

        let mut matrix = Mat_4x4 { m: [[0.; 4]; 4] };
        matrix.m[0][0] = new_right.x;
        matrix.m[0][1] = new_right.y;
        matrix.m[0][2] = new_right.z;
        matrix.m[0][3] = 0.0;
        matrix.m[1][0] = new_up.x;
        matrix.m[1][1] = new_up.y;
        matrix.m[1][2] = new_up.z;
        matrix.m[1][3] = 0.0;
        matrix.m[2][0] = new_forward.x;
        matrix.m[2][1] = new_forward.y;
        matrix.m[2][2] = new_forward.z;
        matrix.m[2][3] = 0.0;
        matrix.m[3][0] = pos.x;
        matrix.m[3][1] = pos.y;
        matrix.m[3][2] = pos.z;
        matrix.m[3][3] = 1.0;
        matrix
    }

    pub fn matrix_quick_inverse(m: &Mat_4x4) -> Mat_4x4 {
        let mut matrix = Mat_4x4 { m: [[0.; 4]; 4] };
        matrix.m[0][0] = m.m[0][0];
        matrix.m[0][1] = m.m[1][0];
        matrix.m[0][2] = m.m[2][0];
        matrix.m[0][3] = 0.0;
        matrix.m[1][0] = m.m[0][1];
        matrix.m[1][1] = m.m[1][1];
        matrix.m[1][2] = m.m[2][1];
        matrix.m[1][3] = 0.0;
        matrix.m[2][0] = m.m[0][2];
        matrix.m[2][1] = m.m[1][2];
        matrix.m[2][2] = m.m[2][2];
        matrix.m[2][3] = 0.0;
        matrix.m[3][0] =
            -(m.m[3][0] * matrix.m[0][0] + m.m[3][1] * matrix.m[1][0] + m.m[3][2] * matrix.m[2][0]);
        matrix.m[3][1] =
            -(m.m[3][0] * matrix.m[0][1] + m.m[3][1] * matrix.m[1][1] + m.m[3][2] * matrix.m[2][1]);
        matrix.m[3][2] =
            -(m.m[3][0] * matrix.m[0][2] + m.m[3][1] * matrix.m[1][2] + m.m[3][2] * matrix.m[2][2]);
        matrix.m[3][3] = 1.0;
        matrix
    }

}

pub struct Mesh {
    pub tris: Vec<Triangle>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh { tris: Vec::new() }
    }

    pub fn load_from_object_file(&mut self, filename: &str) -> io::Result<()> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);

        let mut verts: Vec<Vec_3d> = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let mut words = line.split_whitespace();

            if let Some(cmd) = words.next() {
                match cmd {
                    "v" => {
                        let mut v = Vec_3d::new(0., 0., 0.);
                        v.x = words.next().unwrap().parse().unwrap();
                        v.y = words.next().unwrap().parse().unwrap();
                        v.z = words.next().unwrap().parse().unwrap();
                        verts.push(v);
                    }
                    "f" => {
                        let mut f = [0; 3];
                        f[0] = words.next().unwrap().parse().unwrap();
                        f[1] = words.next().unwrap().parse().unwrap();
                        f[2] = words.next().unwrap().parse().unwrap();
                        self.tris.push(Triangle {
                            p: [
                                verts[f[0] - 1],
                                verts[f[1] - 1],
                                verts[f[2] - 1],
                            ],
                        });
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }
}


pub struct Scene {
    mat_proj: Mat_4x4,
    camera: Vec_3d,
    look_dir: Vec_3d,
    yaw: f32,
    theta: f32,
}

impl Scene {


}
