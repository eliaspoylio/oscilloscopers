use hound;
use std::{f32::consts::PI, i16};

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

fn main() -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let amplitude = i16::MAX as f32;
    let mut writer = hound::WavWriter::create("lines.wav", spec).unwrap();
    let mut scene: Vec<(f32, f32)> = Vec::new();

    ////////////////////////////////////////////
    const LINES: usize = 16;
    for i in (1..2000) {
        let mut lines: Vec<Point> = Vec::new();
        for j in 0..LINES {
            let f = i as f32 / SIZE_F;
            let point1 = Point {
                x: -(0.01 * PI * 130. * f).sin() * (j * 10) as f32,
                y: (0.01 * PI * 60. * f).sin() * 64.,
            };
            let point2 = Point {
                x: (0.01 * PI * 60. * f).sin() * 64.,
                y: (0.01 * PI * 60. * f).cos() * (j * 10) as f32,
            };
            let line = create_line_float(point1, point2, 0.99);
            line.iter().for_each(|p| lines.push(*p));
        }
        let points = draw_points_float(1. / 50., lines, 5);
        for point in points {
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