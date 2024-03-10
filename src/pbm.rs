use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // Replace "path/to/your/image.pbm" with the actual path to your PBM file
    let file_path = "path/to/your/image.pbm";

    if let Ok(file) = File::open(file_path) {
        let reader = BufReader::new(file);

        // Vector to store Point instances
        let mut points: Vec<Point> = Vec::new();

        // Variables to store image dimensions
        let mut width = 0;
        let mut height = 0;

        // Flag to indicate whether the pixel data section has started
        let mut pixel_data_started = false;

        for line in reader.lines() {
            if let Ok(line) = line {
                let trimmed_line = line.trim();

                if trimmed_line.is_empty() {
                    // Skip empty lines
                    continue;
                }

                if trimmed_line.starts_with('#') {
                    // Skip comment lines
                    continue;
                }

                if !pixel_data_started {
                    if trimmed_line.starts_with("P1") {
                        // P1 is the magic number for PBM plain format
                        continue;
                    } else if trimmed_line.starts_with("P4") {
                        // P4 is the magic number for PBM binary format
                        pixel_data_started = true;
                    } else if trimmed_line.starts_with(char::is_numeric) {
                        // Parse image dimensions
                        let dimensions: Vec<&str> = trimmed_line.split_whitespace().collect();
                        if dimensions.len() == 2 {
                            width = dimensions[0].parse().unwrap();
                            height = dimensions[1].parse().unwrap();
                        } else {
                            eprintln!("Invalid dimensions format.");
                            break;
                        }
                    } else {
                        eprintln!("Invalid PBM format.");
                        break;
                    }
                } else {
                    // Parse binary pixel data
                    let mut binary_data = Vec::new();
                    for byte_str in trimmed_line.split_whitespace() {
                        if let Ok(byte) = u8::from_str_radix(byte_str, 2) {
                            binary_data.push(byte);
                        } else {
                            eprintln!("Invalid binary pixel data.");
                            break;
                        }
                    }

                    // Process binary data and populate points vector
                    for (i, &byte) in binary_data.iter().enumerate() {
                        points.push(Point {
                            x: (points.len() as i32 + i as i32) % width,
                            y: (points.len() as i32 + i as i32) / width,
                        });
                        // You can use the 'byte' variable here if needed
                    }

                    // Break after processing one line of binary data
                    break;
                }
            }
        }

        // Print the points vector
        println!("Image dimensions: {} x {}", width, height);
        for point in &points {
            println!("Pixel at ({}, {})", point.x, point.y);
        }
    } else {
        eprintln!("Error opening the image file.");
    }
}