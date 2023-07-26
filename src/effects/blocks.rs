use rand::{thread_rng, Rng};

use crate::vector::{create_line_float, draw_points_float, project_vertex_f, Point, VertexF};

const SIZE: i32 = (crate::SIZE as f32 * 0.7) as i32;
const CUBE_SIZE: f32 = 10.;
const STEP: f32 = 1.;
const SPEED: f32 = 2.;
const START: f32 = 300.;
const END: f32 = 75.;
const DEPTH: f32 = 20.;
const SIZE_F: f32 = 1.;

struct Cube {
    vertices: [VertexF; 8],
}

impl Cube {
    fn new() -> Cube {
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
        }
    }

    fn init(&mut self) {
        let mut rng = thread_rng();
        let centerx = rng.gen_range(-SIZE..SIZE) as f32;
        let centery = rng.gen_range(-SIZE..SIZE) as f32;
        let init_z = rng.gen_range(100..5 * SIZE) as f32;
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
        ]
    }

    fn update(&mut self) {
        for v in self.vertices.iter_mut() {
            v.z -= SPEED;
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

pub fn blocks() -> Vec<(f32, f32)> {
    let mut blocks: Vec<(f32, f32)> = vec![];
    let mut cubes = [&mut Cube::new()];
    for cube in cubes.iter_mut() {
        cube.init();
    }

    for _i in 1..3000 {
        for cube in cubes.iter_mut() {
            if cube.vertices[0].z <= END {
                cube.init()
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

            if _i % 2 == 0 {
                let centerlines = [
                    create_line_float(Point { x: 0., y: 0. }, Point { x: -SIZE_F, y: SIZE_F }, STEP),
                    create_line_float(Point { x: 0., y: 0. }, Point { x: SIZE_F, y: SIZE_F }, STEP),
                    create_line_float(Point { x: 0., y: 0. }, Point { x: SIZE_F, y: -SIZE_F }, STEP),
                    create_line_float(Point { x: 0., y: 0. }, Point { x: -SIZE_F, y: -SIZE_F }, STEP),
                ];
                for line in centerlines.into_iter() {
                    for l in line {
                        lines.push(l);
                    }
                }
            }
        }

        let cli = draw_points_float(1. / 50., lines, 8);
        for cl in cli {
            blocks.push(cl);
        }
    }
    blocks
}
