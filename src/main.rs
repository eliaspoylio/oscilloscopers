use hound;
use std::i16;

mod matrix;
mod vector;
mod raster;
use crate::vector::draw_points_float;

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
    
    let font_size = 1.1;
    let mut f_x_0 = 0.;
    let mut f_x_1 = 50.;
    let mut f_x_2 = 150.;
    let f_y = 0.;
    for _i in 0..1000 {
        f_x_0 -= 0.3;
        f_x_1 -= 0.27;
        f_x_2 -= 0.32;
        let mut code = effects::text::letter('c', (f_x_0-30., f_y), font_size, 1.);
        let mut o = effects::text::letter('o', (f_x_0-15., f_y), font_size, 1.);
        let mut d = effects::text::letter('d', (f_x_0, f_y), font_size, 1.);
        let mut e = effects::text::letter('e', (f_x_0+15., f_y), font_size, 1.);
        code.append(&mut o);
        code.append(&mut d);
        code.append(&mut e);

        let mut music = effects::text::letter('m', (f_x_1-30., f_y-15.), font_size, 1.);
        let mut u = effects::text::letter('u', (f_x_1-15., f_y-15.), font_size, 1.);
        let mut s = effects::text::letter('s', (f_x_1, f_y-15.), font_size, 1.);
        let mut i = effects::text::letter('i', (f_x_1+15., f_y-15.), font_size, 1.);
        let mut c = effects::text::letter('c', (f_x_1+30., f_y-15.), font_size, 1.);
        music.append(&mut u);
        music.append(&mut s);
        music.append(&mut i);
        music.append(&mut c);
        
        let mut spew = effects::text::letter('b', (f_x_2-30., f_y), font_size, 1.);
        let mut y = effects::text::letter('y', (f_x_2-15., f_y), font_size, 1.);
        let mut s = effects::text::letter('s', (f_x_2+15., f_y), font_size, 1.);
        let mut p = effects::text::letter('p', (f_x_2+30., f_y), font_size, 1.);
        let mut e = effects::text::letter('e', (f_x_2+45., f_y), font_size, 1.);
        let mut w = effects::text::letter('w', (f_x_2+60., f_y), font_size, 1.);
        spew.append(&mut y);
        spew.append(&mut s);
        spew.append(&mut p);
        spew.append(&mut e);
        spew.append(&mut w);

        code.append(&mut music);
        code.append(&mut spew);
        let text = code.into_iter().filter(|p| p.x > -SIZE_F*0.7).collect();
        let points = draw_points_float(1. / 50., text, 8);
        for point in points {
            if point.0 > -0.7 {scene.push(point);}    
        }
    }
    
    for i in effects::lines::lines() {
        scene.push(i);
    }
    for i in effects::landscape::landscape() {
        scene.push(i);
    }
    
    for i in effects::blocks::blocks() {
        scene.push(i);
    }

    for i in effects::cube::cube() {
        scene.push(i);
    }
    
    for i in effects::stars::stars() {
        scene.push(i);
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
