use rand::{thread_rng, Rng};

use crate::vector::{create_line_float, draw_points_float, project_vertex_f, Point, VertexF};

const SIZE: i32 = (crate::SIZE as f32 * 0.7) as i32;
const CUBE_SIZE: f32 = 10.;
const STEP: f32 = 1.;
const SPEED: f32 = 10.;
const START: f32 = 500.;
const END: f32 = 100.;
const DEPTH: f32 = 40.;

struct Cube {
    vertices: [VertexF; 8],
    startx: f32,
    starty: f32,
    direction: bool,
}

impl Cube {
    fn new(startx: f32, starty: f32) -> Cube {
        Cube {
            vertices: [
                VertexF::new(0., 0., 290.),
                VertexF::new(0., 0., 290.),
                VertexF::new(0., 0., 290.),
                VertexF::new(0., 0., 290.),
                VertexF::new(0., 0., 270.),
                VertexF::new(0., 0., 270.),
                VertexF::new(0., 0., 270.),
                VertexF::new(0., 0., 270.),
            ],
            startx: startx,
            starty: starty,
            direction: true,
        }
    }

    fn init(&mut self) {
        let centerx = self.startx as f32;
        let centery = self.starty as f32;
        let init_z = 0.;
        self.vertices = [
            VertexF::new(centerx - CUBE_SIZE, centery + CUBE_SIZE, START + init_z),
            VertexF::new(centerx + CUBE_SIZE, centery + CUBE_SIZE, START + init_z),
            VertexF::new(centerx + CUBE_SIZE, centery - CUBE_SIZE, START + init_z),
            VertexF::new(centerx - CUBE_SIZE, centery - CUBE_SIZE, START + init_z),
            VertexF::new(
                centerx - CUBE_SIZE,
                centery + CUBE_SIZE,
                START + DEPTH + init_z,
            ),
            VertexF::new(
                centerx + CUBE_SIZE,
                centery + CUBE_SIZE,
                START + DEPTH + init_z,
            ),
            VertexF::new(
                centerx + CUBE_SIZE,
                centery - CUBE_SIZE,
                START + DEPTH + init_z,
            ),
            VertexF::new(
                centerx - CUBE_SIZE,
                centery - CUBE_SIZE,
                START + DEPTH + init_z,
            ),
        ];
        self.direction = true;
    }

    fn update(&mut self) {
        if self.direction == true {
            for v in self.vertices.iter_mut() {
                v.z -= SPEED;
            }
        }
        if self.direction == false {
            for v in self.vertices.iter_mut() {
                v.z += SPEED;
            }
        }
    }
}

fn zoom(vertices: [&mut VertexF; 8]) {
    for v in vertices {
        if v.z > 0. {
            v.z -= 1.;
        }
    }
}

pub fn blocks(l: i32) -> Vec<(f32, f32)> {
    let mut blocks: Vec<(f32, f32)> = vec![];
    let mut cubes = [
        &mut Cube::new(-20., 20.),
        &mut Cube::new(20., 20.),
        &mut Cube::new(20., -20.),
        &mut Cube::new(-20., -20.),
    ];
    for cube in cubes.iter_mut() {
        cube.init();
    }

    for _i in 1..l {
        for cube in cubes.iter_mut() {
            if cube.vertices[0].z <= END {
                cube.direction = false;
            }
            if cube.vertices[0].z >= START {
                cube.direction = true;
            }
            cube.update();
        }
        let mut lines: Vec<Point> = vec![];

        for cube in cubes.iter() {
            let mut af = cube.vertices[0];
            let mut bf = cube.vertices[1];
            let mut cf = cube.vertices[2];
            let mut df = cube.vertices[3];
            let mut ab = cube.vertices[4];
            let mut bb = cube.vertices[5];
            let mut cb = cube.vertices[6];
            let mut db = cube.vertices[7];
            let array: [Vec<Point>; 12] = [
                // The front face
                create_line_float(project_vertex_f(&mut af), project_vertex_f(&mut bf), STEP),
                create_line_float(project_vertex_f(&mut bf), project_vertex_f(&mut cf), STEP),
                create_line_float(project_vertex_f(&mut cf), project_vertex_f(&mut df), STEP),
                create_line_float(project_vertex_f(&mut df), project_vertex_f(&mut af), STEP),
                // The back face
                create_line_float(project_vertex_f(&mut ab), project_vertex_f(&mut bb), STEP),
                create_line_float(project_vertex_f(&mut bb), project_vertex_f(&mut cb), STEP),
                create_line_float(project_vertex_f(&mut cb), project_vertex_f(&mut db), STEP),
                create_line_float(project_vertex_f(&mut db), project_vertex_f(&mut ab), STEP),
                // The front-to-back edges
                create_line_float(project_vertex_f(&mut af), project_vertex_f(&mut ab), STEP),
                create_line_float(project_vertex_f(&mut bf), project_vertex_f(&mut bb), STEP),
                create_line_float(project_vertex_f(&mut cf), project_vertex_f(&mut cb), STEP),
                create_line_float(project_vertex_f(&mut df), project_vertex_f(&mut db), STEP),
            ];

            for line in array {
                for l in line {
                    lines.push(l);
                }
            }
        }

        let cli = draw_points_float(1. / 50., lines, 7);
        for cl in cli {
            blocks.push(cl);
        }
    }
    blocks
}
