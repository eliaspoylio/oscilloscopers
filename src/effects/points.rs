use std::f32::consts::PI;

use crate::vector::{create_line_float, draw_points_float, Point};

const SIZE: i32 = crate::SIZE;

pub fn points(l: i32) -> Vec<(f32, f32)> {
    let mut scene: Vec<(f32, f32)> = Vec::new();
    let a = SIZE / 10; 

    for i in 1..l {
        let mut po: Vec<Point> = Vec::new();
        for x in -9..9 {
            for y in -9..9 {
                let point = Point {
                    x: (x*a) as f32,
                    y: (y*a) as f32,
                };
                let stay = ((x+y+i) as f32 / 5.).sin();
                for _ in 1..((stay*10.) as usize) {
                    po.push(point);
                }
            }
            
        }
        let ps = draw_points_float(1. / 50., po, 1);
        for p in ps {
            scene.push(p);
        }
    }

    scene
}