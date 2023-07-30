use crate::vector::{draw_points_float, Point};

use rand::{thread_rng, Rng};

use crate::SIZE_F;

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

pub fn stars(l: i32) -> Vec<(f32, f32)> {
    let mut scene: Vec<(f32, f32)> = Vec::new();

    ////////////////////////////////////////////
    
    const MAX_STARS: usize = 800;
    let stars = &mut [Star::empty(); MAX_STARS];
    let center_x = 0;//SIZE >> 1;
    let center_y = 0;//SIZE >> 1;
    let mut fade = 1.;

    for star in stars.iter_mut() {
        star.init_star();
    }

    for _i in 1..l {
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
        let frame_points = draw_points_float(1. / 24., frame, 2);
        for point in frame_points {
            if _i > l-l/4 {
                scene.push((
                    point.0 * fade,
                    point.1 * fade
                ));
                fade -= 0.00001;
            }
            else {
                scene.push(point);   
            }
        }
    }

    /////////////////////////////////////////////

    scene

}