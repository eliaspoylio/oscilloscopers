use hound;
use std::i16;
use std::f32::consts::PI;

mod matrix;
mod vector;
use crate::vector::{create_line_float, draw_points_float, Point, VertexF};

mod effects;

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
    let mut writer = hound::WavWriter::create("letters.wav", spec).unwrap();
    let mut scene: Vec<(f32, f32)> = Vec::new();

    ////////////////////////////////////////////

    
    for i in effects::line2::line2() {
        scene.push(i);
    }
    for i in effects::line::line() {
        scene.push(i);
    }
    for i in effects::lines::lines() {
        scene.push(i);
    }
    for i in effects::landscape::landscape() {
        scene.push(i);
    }
    /* 
    let lines = effects::lines::lines();
    let landscape = effects::landscape::landscape();
    for i in 0..(lines.len() - lines.len() / 2) {
        scene.push(lines[i]);
    }
    let mut counter = 0;
    for i in (lines.len() - lines.len() / 2)..lines.len() {
        let f = i as f32 / SIZE_F;
        let sine = (0.01 * PI * 0.01 + f/2.).sin() / 10.;
        scene.push((lines[i].0 + sine, lines[i].1 - sine));
        counter += 1;
    }
    for i in counter..landscape.len() {
        scene.push(landscape[i]);
    }
    */
    
    for _i in 0..200 {
        let c = effects::text::letter((1., 1.), 10.);
        let points = draw_points_float(1. / 50., c, 5);
        for point in points {
            scene.push(point);
        }
    }

    ////////////////////////////////////////////

    for f in scene {
        writer.write_sample((f.0 * amplitude) as i16).unwrap();
        writer.write_sample((f.1 * amplitude) as i16).unwrap();
    }

    println!("Length: {}", writer.len());
    println!("Duration: {}", writer.duration() / spec.sample_rate);
    writer.finalize()
}
