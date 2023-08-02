use hound;
use itertools::{EitherOrBoth::*, Itertools};
use libxm::XMContext;
use std::i16;

mod matrix;
mod raster;
mod vector;
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

fn main() -> () {
    let spec = hound::WavSpec {
        channels: 4,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let amplitude = i16::MAX as f32;
    let mut writer = hound::WavWriter::create("cesium.wav", spec).unwrap();
    let mut title: Vec<(f32, f32)> = Vec::new();
    let mut scene: Vec<(f32, f32)> = Vec::new();

    ////////////////////////////////////////////

    let bytes = include_bytes!("../test.xm");
    let data = bytes.to_vec();

    let mut xm: XMContext = XMContext::new(&data, 48000).unwrap();
    xm.set_max_loop_count(1);

    ////////////////////////////////////////////

    let font_size = 1.4;
    let mut title_font_size = 1.5;
    let mut f_x_0 = 0.;
    let mut f_x_1 = 0.;
    let mut f_x_2 = 0.;
    let f_y_0 = 20.;
    let f_y_1 = 0.;
    let f_y_2 = 0.;
    let f_y = 0.;

    let mut spread = 7.;
    for i in 0..320 {
        if i < 40 {
            spread -= 0.125;
        }
        if i == 40 {
            spread = 2.;
        }
        if i > 280 {
            spread += 0.125;
        }
        title_font_size -= 0.001;
        f_x_0 -= 0.0;
        f_x_1 -= 0.0;
        f_x_2 -= 0.0;
        let mut cesium = effects::text::letter('c', (f_x_0 - 50., 0.), title_font_size, spread);
        let mut e = effects::text::letter('e', (f_x_0 - 30., 0.), title_font_size, spread);
        let mut s = effects::text::letter('s', (f_x_0 - 10., 0.), title_font_size, spread);
        let mut i = effects::text::letter('i', (f_x_0 + 10., 0.), title_font_size, spread);
        let mut u = effects::text::letter('u', (f_x_0 + 30., 0.), title_font_size, spread);
        let mut m = effects::text::letter('m', (f_x_0 + 50., 0.), title_font_size, spread);
        cesium.append(&mut e);
        cesium.append(&mut s);
        cesium.append(&mut i);
        cesium.append(&mut u);
        cesium.append(&mut m);

        let mut c = -1;
        cesium.retain(|_| {
            c += 1;
            return c % 2 == 0;
        });
        let points = draw_points_float(1. / 40., cesium, 8);
        for point in points {
            scene.push(point);
        }
    }

    let mut spread = 7.;
    for i in 0..160 {
        if i < 40 {
            spread -= 0.125;
        }
        if i == 40 {
            spread = 2.;
        }
        if i > 120 {
            spread += 0.125;
        }
        f_x_0 -= 0.0;
        f_x_1 -= 0.0;
        f_x_2 -= 0.0;
        let mut code = effects::text::letter('c', (f_x_0 - 30., f_y_0), font_size, spread);
        let mut o = effects::text::letter('o', (f_x_0 - 15., f_y_0), font_size, spread);
        let mut d = effects::text::letter('d', (f_x_0, f_y_0), font_size, spread);
        let mut e = effects::text::letter('e', (f_x_0 + 15., f_y_0), font_size, spread);
        let mut and = effects::text::letter('&', (f_x_0 + 30., f_y_0), font_size, spread);
        code.append(&mut o);
        code.append(&mut d);
        code.append(&mut e);
        code.append(&mut and);

        let mut music = effects::text::letter('m', (f_x_1 - 30., f_y_1), font_size, spread);
        let mut u = effects::text::letter('u', (f_x_1 - 15., f_y_1), font_size, spread);
        let mut s = effects::text::letter('s', (f_x_1, f_y_1), font_size, spread);
        let mut i = effects::text::letter('i', (f_x_1 + 15., f_y_1), font_size, spread);
        let mut c = effects::text::letter('c', (f_x_1 + 30., f_y_1), font_size, spread);
        music.append(&mut u);
        music.append(&mut s);
        music.append(&mut i);
        music.append(&mut c);

        code.append(&mut music);
        let mut text: Vec<vector::Point> =
            code.into_iter().filter(|p| p.x > -SIZE_F * 0.7).collect();
        let mut c = -1;
        text.retain(|_| {
            c += 1;
            return c % 2 == 0;
        });
        let points = draw_points_float(1. / 40., text, 8);
        for point in points {
            if point.0 > -0.7 {
                scene.push(point);
            }
        }
    }

    spread = 7.;
    for i in 0..160 {
        if i < 40 {
            spread -= 0.125;
        }
        if i == 40 {
            spread = 2.;
        }
        if i > 120 {
            spread += 0.125;
        }
        let mut by = effects::text::letter('b', (f_x_2 - 10., f_y_0), font_size, spread);
        let mut y = effects::text::letter('y', (f_x_2 + 10., f_y_0), font_size, spread);
        let mut spew = effects::text::letter('s', (f_x_2 - 15., f_y_1), font_size, spread);
        let mut p = effects::text::letter('p', (f_x_2 - 5., f_y_1), font_size, spread);
        let mut e = effects::text::letter('e', (f_x_2 + 5., f_y_1), font_size, spread);
        let mut w = effects::text::letter('w', (f_x_2 + 15., f_y_1), font_size, spread);
        by.append(&mut y);

        spew.append(&mut w);

        spew.append(&mut p);
        spew.append(&mut e);

        let mut c = -1;
        spew.retain(|_| {
            c += 1;
            return c % 2 == 0;
        });
        by.append(&mut spew);
        let points = draw_points_float(1. / 40., by, 10);
        for point in points {
            if point.0 > -0.7 {
                scene.push(point);
            }
        }
    }

    for i in effects::lines::lines(1600) {
        scene.push(i);
    }

    for i in effects::landscape::landscape(1600) {
        scene.push(i);
    }

    for i in effects::blocks::blocks(800) {
        scene.push(i);
    }

    for i in effects::cube::cube(1600) {
        scene.push(i);
    }

    for i in effects::stars::stars(4000) {
        scene.push(i);
    }

    ////////////////////////////////////////////

    let mut xm_channel = Vec::new();
    let mut buffer = [0.0; 4096];
    while xm.loop_count() == 0 {
        xm.generate_samples(&mut buffer);
        for b in buffer {
            xm_channel.push(b*2.);
        }
    }

    for _ in 0..SAMPLE_RATE/10 {
        writer.write_sample((0.2 * amplitude) as i16).unwrap();
        writer.write_sample((0.2 * amplitude) as i16).unwrap();
        writer.write_sample((0.2 * amplitude) as i16).unwrap();
        writer.write_sample((0.2 * amplitude) as i16).unwrap();
    }
/*
    let decay = 0.0000001;
    let mut factor = 1.;
    for f in title {
        writer.write_sample((f.0 * factor * amplitude) as i16).unwrap();
        writer.write_sample((f.1 * factor * amplitude) as i16).unwrap();
        //writer.write_sample((0. * amplitude) as i16).unwrap();
        //writer.write_sample((0. * amplitude) as i16).unwrap();
        factor -= decay;
    }
 */
    for pair in xm_channel.iter().zip_longest(scene.clone().iter()) {
        match pair {
            Both(l, r) => {
                writer.write_sample((r.0 * amplitude) as i16).unwrap();
                writer.write_sample((r.1 * amplitude) as i16).unwrap();
                writer.write_sample((l * amplitude) as i16).unwrap();
                writer.write_sample((l * amplitude) as i16).unwrap();
            }
            Left(_l) => (),
            Right(_r) => (),
        }
    }

    println!("Length: {}", writer.len());
    println!("Duration: {}", writer.duration() / spec.sample_rate);
    writer.finalize().unwrap();
}
