use hound;
use std::i16;

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

    for _i in 1..2000 {
        let mut frame: Vec<Point> = Vec::new();
        for star in stars.iter_mut() {
            star.z -= star.speed;

            if star.z <= 0 { star.init_star() };

            let ix = (star.x / star.z as f32) + (center_x) as f32;
            let iy = (star.y / star.z as f32) + (center_y) as f32;

            star.bright += 0.0095;

            if ix > -SIZE_F && ix < SIZE_F && iy > -SIZE_F && iy < SIZE_F {
                for _f in 0..star.bright.ceil() as i32 {
                    frame.push(Point::new(ix, iy));   
                }
            }
            else {
                star.init_star();
            }
        }
        let frame_points = draw_points_float(1. / 50., frame, 2);
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