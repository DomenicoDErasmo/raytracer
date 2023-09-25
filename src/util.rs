use rand::prelude::Distribution;

pub const PI: f32 = 3.1415926535897932385;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_double() -> f32 {
    let range = rand::distributions::Uniform::new(0.0, 1.0);
    let mut rng = rand::thread_rng();
    range.sample(&mut rng)
}