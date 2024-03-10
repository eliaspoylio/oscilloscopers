use hound;
use itertools::{EitherOrBoth::*, Itertools};
use libxm::XMContext;
use std::{i16, path};

mod audio;
mod matrix;
mod raster;
mod vector;
mod bytes;
use crate::{audio::delay, effects::gameoflife, vector::draw_points_float};
mod util;
mod obj;

mod effects;

const SAMPLE_RATE: u32 = 44100;
const SAMPLE_RATE_F: f32 = SAMPLE_RATE as f32;
const SIZE: i32 = 100;

const CANVAS: i32 = 200;
const DISTANCE: i32 = 50;

const SIZE_F: f32 = 100.;
const CANVAS_F: f32 = 200.;
const DISTANCE_F: f32 = 50.;

fn main() -> () {
    let spec = hound::WavSpec {
        channels: 4,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let amplitude = i16::MAX as f32;
    let mut writer = hound::WavWriter::create("testi.wav", spec).unwrap();
    let mut scene: Vec<(f32, f32)> = Vec::new();

    ////////////////////////////////////////////

    //for i in effects::points::points(1000) {
    //    scene.push(i);
    //}
    let mut v: Vec<u8> = vec![63,193];
    /* 
    for t in 0..1000000 {
        let f: Vec<(f32,f32)> = v
        .clone()
        .into_iter()
        .map(|b|(bytes::to_f32(b.wrapping_shr(t/10000)),bytes::to_f32(b.wrapping_shl(t/10000))))
        .collect();

        for t in f {
            scene.push(t);
        }
    }
    */
    /* 
    for t in 0..100000 {
        for num in v.iter_mut() {
            let x = num.overflowing_add(t as u8 %100);
            *num = x.0;
        }
        for f in v.clone() {
            //println!("{}", f);
            scene.push((
                //bytes::to_f32(f.overflowing_add(t as u8 %100).0)/SIZE_F,
                //bytes::to_f32(f.overflowing_add(t as u8 %200).0)/SIZE_F
                bytes::to_signal(f as u8 %20)*t as f32,
                bytes::to_signal(f as u8 %200)
            ));
        }
    }
    */

    ////////////////////////////////////////////

    //delay(&mut scene, 1, 4, 4, 1.0, 4, 16, SAMPLE_RATE as usize);

    ////////////////////////////////////////////

    //for s in scene {
    //    writer.write_sample((s.0 * amplitude) as i16).unwrap();
    //    writer.write_sample((s.1 * amplitude) as i16).unwrap();
    //}

    //println!("Length: {}", writer.len());
    //println!("Duration: {}", writer.duration() / spec.sample_rate);
    //writer.finalize().unwrap();

    use std::fs;
    use std::path::Path;

    //let file_contents: Vec<u8> = vec![255,2,3];
    let path: &Path = Path::new("bytes");

    //fs::write(path, file_contents).unwrap();
    /* 
    let read = match fs::read(path) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error reading the file: {:?}", e);
            Vec::new()
        },
    };
    */
    
    //let read = bytes::read_points(path);
    let read = bytes::read_points(path);
    println!("{:?}", read.len());
    //let read = bytes::connect(read.clone());
    for _ in 0..1000 {
        for r in read.clone() {
            scene.push((r.x/SIZE_F,r.y/SIZE_F));
        }
    }
    

    //for i in effects::gameoflife::game_of_life(2000) {
    //    scene.push(i);
    //}


    for s in scene {
        writer.write_sample((s.0 * amplitude) as i16).unwrap();
        writer.write_sample((s.1 * amplitude) as i16).unwrap();
    }
/* 
    let path: &Path = Path::new("eels");
    let frames = bytes::read_frames(path);
    for f in frames {
        writer.write_sample((f.0 * amplitude) as i16).unwrap();
        writer.write_sample((f.1 * amplitude) as i16).unwrap();
    }
*/

    let path = Path::new("eels2");
    let frames = bytes::read_spritesheet(path, 32, 6.);
    for _ in 0..10 {
        for f in &frames {
            writer.write_sample((f.0 * amplitude) as i16).unwrap();
            writer.write_sample((f.1 * amplitude) as i16).unwrap();
        }
    }

    //obj::obj();

    println!("Length: {}", writer.len());
    println!("Duration: {}", writer.duration() / spec.sample_rate);
}
