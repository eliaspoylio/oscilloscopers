use std::fs;
use std::path::Path;

use itertools::{Itertools, WithPosition};

use crate::vector::{self, draw_points_float, Point};

const SIZE: f32 = super::SIZE_F;

struct PointBytes {
    x: u8,
    y: u8,
}

impl PointBytes {
    fn to_point(self) -> Point {
        Point::new(to_f32(self.x), to_f32(self.y))
    }
}

pub fn to_f32(x:u8) -> f32 {
    (x as f32)/255.*(2.*SIZE)-SIZE
}

pub fn to_signal(x:u8) -> f32 {
    ((x as f32)/255.*(2.*SIZE)-SIZE)/SIZE
}

fn euclidean_distance(point1: &Point, point2: &Point) -> f32 {
    ((point1.x - point2.x).powi(2) + (point1.y - point2.y).powi(2)).sqrt()
}

fn write_bytes() {
    let file_contents: Vec<u8> = vec![255,2,3];
    let path: &Path = Path::new("bytes");
    fs::write(path, file_contents).unwrap();
}

pub fn read_bytes(path: &Path) -> Vec<u8> {
    let read = match fs::read(path) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error reading the file: {:?}", e);
            Vec::new()
        },
    };
    read
}

pub fn read_points(path: &Path) -> Vec<Point> {
    let read = read_bytes(path);

    let pb: Vec<Point> = read
       .chunks(2)
       .map(|point| Point { x: to_f32(point[0]), y: to_f32(point[1]) })
       .collect();
    let sorted = sort_points_by_proximity(pb);
    sorted
}

pub fn sort_points_by_proximity(points: Vec<Point>) -> Vec<Point> {
    let mut sorted_points = Vec::with_capacity(points.len());
    let mut remaining_points = points.clone();

    // Start with an arbitrary point and find the nearest neighbor iteratively
    if let Some(start_point) = remaining_points.pop() {
        sorted_points.push(start_point);

        while let Some(last_point) = sorted_points.last() {
            if let Some(nearest_point_idx) = remaining_points
                .iter()
                .enumerate()
                .min_by_key(|(_, point)| {
                    (euclidean_distance(last_point, point)).to_bits() as i64
                })
                .map(|(idx, _)| idx)
            {
                let nearest_point = remaining_points.remove(nearest_point_idx);
                sorted_points.push(nearest_point);
            } else {
                break; // No more points left
            }
        }
    }

    sorted_points
}

pub fn read_spritesheet(path: &Path, size: usize, fps: f32) -> Vec<(f32, f32)> {
    let read = read_bytes(path);
    let mut scene: Vec<(f32, f32)> = Vec::new();

    let mut cursor = 0;

    while cursor + 2 <= read.len() {
        // Read the size field (u16) as big-endian bytes
        let size_bytes: [u8; 2] = [read[cursor], read[cursor + 1]];
        let chunk_size = u16::from_be_bytes(size_bytes);

        // Update the cursor
        cursor += 2;

        // If the size is zero, it indicates the end of the data
        if chunk_size == 0 {
            break;
        }

        // Ensure that there is enough data remaining
        if cursor + chunk_size as usize <= read.len() {
            // Read the actual chunk data
            let chunk_data = &read[cursor..cursor + chunk_size as usize];

            // Process the chunk data (replace this with your logic)
            println!("Read chunk: {:?}", chunk_data);
            let mut frame: Vec<Point> = Vec::new();
            for b in chunk_data.chunks_exact(2) {
                // Process each chunk of two elements
                // Replace this with your specific processing logic
                frame.push(vector::Point::new(to_f32(b[0]), to_f32(b[1])));
            }
            let frame_points = draw_points_float(1. / fps, frame, 2);
            for point in frame_points {
                    scene.push(point);    
            }

            // Update the cursor
            cursor += chunk_size as usize;
        } else {
            // Handle the case where there is not enough data remaining
            eprintln!("Error: Not enough data for the specified chunk size.");
            break;
        }
    }
    scene
}


pub fn read_frames(path: &Path) -> Vec<(f32, f32)> {
    let mut scene: Vec<(f32, f32)> = Vec::new();
    let bytes = read_bytes(path);
    let mut frames: Vec<Vec<Point>> = Vec::new();

    let mut i = 6;
    while i < bytes.len() {
        match bytes[i] {
            0 => {
                //println!("{}: {}", i, bytes[i]);
                frames.push(vec![Point{x:0.,y:0.}]);
                i += 2;
            },
            _ => {
                //println!("{}: {}", i, bytes[i]);
                let big_endian_size = (u16::from(bytes[i]) << 8) | u16::from(bytes[i + 1]);
                println!("{}: {}", i, big_endian_size);
                let mut frame: Vec<Point> = Vec::new();
                for n in i..i+(big_endian_size as usize) {
                    if (i + 2 + n as usize) < bytes.len() {
                        let x = to_f32(bytes[i + 1 + n as usize]);
                        let y = to_f32(bytes[i + 2 + n as usize]);
    
                        frame.push(Point { x, y });
                    }
                }
                frames.push(frame);
                //println!("{}", frame.len());
                //i += (bytes[i]-1) as usize;
                //i += 2;
                i += big_endian_size as usize
            }
        }
    }
    println!("{}", frames.len());
    for frame in frames {
        let frame_points = draw_points_float(1. / 50., frame, 2);
        for point in frame_points {
            scene.push(point);   
        }
    }
    scene
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_() {
        assert_eq!(to_f32(u8::MIN),-SIZE);
        assert_eq!(to_f32(u8::MAX),SIZE);
        assert!(to_f32(128)<=1. && to_f32(128)>=-1.)
    }
}