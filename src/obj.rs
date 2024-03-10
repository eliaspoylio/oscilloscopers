use std::fs::File;
use std::io::{self, BufRead};

struct Vertex {
    x: f32,
    y: f32,
    z: f32,
}

struct Face {
    vertices: Vec<usize>,
}

pub fn obj() {
    let file_path = "cube.obj";

    if let Ok(file) = File::open(file_path) {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                let parts: Vec<&str> = line.trim().split_whitespace().collect();

                if parts.is_empty() {
                    continue;
                }

                match parts[0] {
                    "v" => {
                        if parts.len() == 4 {
                            let vertex = Vertex {
                                x: parts[1].parse().unwrap_or(0.0),
                                y: parts[2].parse().unwrap_or(0.0),
                                z: parts[3].parse().unwrap_or(0.0),
                            };
                            vertices.push(vertex);
                        }
                    }
                    "f" => {
                        let face = Face {
                            vertices: parts[1..]
                                .iter()
                                .map(|s| s.split('/').next().unwrap().parse().unwrap_or(0))
                                .collect(),
                        };
                        faces.push(face);
                    }
                    _ => {}
                }
            }
        }

        // Print information about the model
        println!("Vertices: {}", vertices.len());
        println!("Faces: {}", faces.len());

        // Print the vertices
        println!("Vertices:");
        for vertex in &vertices {
            println!("  {:?} {:?} {:?}", vertex.x, vertex.y, vertex.z);
        }

        // Print the indexes
        println!("Indexes:");
        for face in &faces {
            println!("  {:?}", face.vertices);
        }

        // Print the faces
        println!("Faces:");
        for face in &faces {
            for index in &face.vertices {
                let v = &vertices[*index-1];
                println!("{}: {} {} {}", index, v.x,v.y,v.z);
            }
        }
        
    } else {
        println!("Failed to open file");
    }
}