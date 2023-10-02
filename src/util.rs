use std::f32::consts::PI;

use rand::prelude::Distribution;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_float(min: Option<f32>, max: Option<f32>) -> f32 {
    let range = rand::distributions::Uniform::new(
        min.unwrap_or(0.0), 
        max.unwrap_or(1.0)
    );
    let mut rng = rand::thread_rng();
    range.sample(&mut rng)
}

pub fn random_integer(min: i32, max: i32) -> i32 {
    random_float(Some(min as _), Some(max as _)) as i32
}