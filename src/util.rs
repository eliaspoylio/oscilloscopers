use std::{collections::HashSet, usize};
use rand::{Rng, SeedableRng};
use crate::vector::Point;

const SIZE: i32 = crate::SIZE;
const SIZE_F: f32 = crate::SIZE_F;
const SAMPLE_RATE: u32 = crate::SAMPLE_RATE;

pub fn remove_random_points(vec: Vec<Point>, frame_rate: f32) -> Vec<Point> {
    let max = (frame_rate*SAMPLE_RATE as f32) as usize;
    match vec.len() > max {
        false => vec,
        true => {
            let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(1);
            let l = vec.len() - max;
            let mut new = vec.clone();
            for _ in 0..l {
                let i = rng.gen_range(0..new.len());
                new.remove(i);
            }
            new
        }
    }
}


pub fn remove_random_samples(vec: Vec<(f32, f32)>, frame_rate: f32) ->  Vec<(f32, f32)> {
    let max = (frame_rate*SAMPLE_RATE as f32) as usize;
    match vec.len() > max {
        false => vec,
        true => {
            let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(1);
            let l = vec.len() - max;
            let mut new = vec.clone();
            for _ in 0..l {
                let i = rng.gen_range(0..new.len());
                new.remove(i);
            }
            new
        }
    }
}

/* 
pub fn take_frame(vec: Vec<(f32, f32)>, samples: usize) -> Vec<(Point)> {
    let start_index = vec.len() - samples;
    let frame: Vec<(f32, f32)> = vec[start_index..].to_vec();
    let frame_points =
    frame
    .iter()
    .map(|&(x,y)| ((x*SIZE_F) as i32, (y*SIZE_F) as i32))
    /* 
    let mut int_vec: Vec<(i32, i32)> = 
    frame
    .iter()
    .map(|&(x,y)| ((x*SIZE_F) as i32, (y*SIZE_F) as i32))
    .collect();
    let mut set = HashSet::new();
    int_vec.retain(|&tuple| set.insert(tuple));
    let float_vec: Vec<(f32, f32)> = 
    int_vec
    .iter()
    .map(|&(x,y)| (x as f32 / SIZE_F, y as f32 / SIZE_F))
    .collect();
    float_vec
    */
}
*/