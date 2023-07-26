use std::f32::consts::PI;
use crate::vector::{draw_points_float, Point};

const SIZE_F: f32 = crate::SIZE_F;

pub fn line() -> Vec<(f32, f32)> {
    let mut scene: Vec<(f32, f32)> = Vec::new();

    ////////////////////////////////////////////
    const LINES: usize = 3;
    for i in (1..2000).step_by(LINES) {
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
            //let line = create_line_float(point1, point2, 0.99);
            //let points = draw_points_float(1. / 50., line, 2);
            let points = draw_points_float(
                1. / 50.,
                vec![point1, point2],
                2);
            for point in points {
                scene.push((point.0, point.1));
            }
        }
    }
    scene
}
/*


use hound;
use std::{i16, f32::consts::PI};

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

fn sine(f: f32) -> f32 {
    SIZE_F + (f * PI).sin()
}

fn main() -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let amplitude = i16::MAX as f32;
    let mut writer = hound::WavWriter::create("line.wav", spec).unwrap();
    let mut scene: Vec<(f32, f32)> = Vec::new();

    ////////////////////////////////////////////

    let mut rng = thread_rng();

    for i in (1..2000).step_by(3) {
        let f = i as f32 / SIZE_F;
        let point1 = Point { x: -(7.7*PI*f).sin() * SIZE_F as f32, y: sine(0.5*f) * SIZE_F };
        let point2 = Point { x: (0.4*PI*(f+90.)).sin() * SIZE_F, y: -(7.7*PI*f).sin() * SIZE_F };
        let line = create_line_float(point1, point2, 0.1);
        let points = draw_points_float(1./50., line, 1);

        for point in points {
            scene.push((point.0 * 2., point.1 * 2.));
            scene.push((point.0 + 0.01 * 2., point.1*2.));
            scene.push((point.0 + 0.02 * 2., point.1*2.));
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


*/
//////////////////////////////////////////////////////////
/*
use hound;
use std::{i16, f32::consts::PI};

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

fn sine(f: f32) -> f32 {
    SIZE_F + (f * PI).sin()
}

fn main() -> Result<(), hound::Error> {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let amplitude = i16::MAX as f32;
    let mut writer = hound::WavWriter::create("line.wav", spec).unwrap();
    let mut scene: Vec<(f32, f32)> = Vec::new();

    ////////////////////////////////////////////

    let mut rng = thread_rng();

    for i in (1..2000).step_by(4) {
        let f = i as f32 / SIZE_F;
        let point1 = Point { x: -(7.7*PI*f).sin() * SIZE_F * SIZE_F as f32, y: sine(2.5*f) * SIZE_F };
        let point2 = Point { x: (2.7*PI*(f+90.)).sin() * SIZE_F, y: -(7.7*PI*f).sin() * SIZE_F };
        let line = create_line_float(point1, point2, 0.1);
        let points = draw_points_float(1./50., line, 1);

        for point in points {
            scene.push(point);
            scene.push(point);
            scene.push(point);
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

*/
