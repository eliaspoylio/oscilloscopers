use std::f32::consts::PI;
use crate::vector::{draw_points_float, Point};

const SIZE_F: f32 = crate::SIZE_F;

pub fn line2() -> Vec<(f32, f32)> {
    let mut scene: Vec<(f32, f32)> = Vec::new();
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

            let points = draw_points_float(
                1. / 50.,
                vec![point1, point2],
                2);
            for point in points {
                scene.push((point.0*j as f32, point.1*j as f32));
            }
        }
    }
    scene
}