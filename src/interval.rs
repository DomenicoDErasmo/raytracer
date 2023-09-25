#[derive(Clone, Copy)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: UNIVERSE.min,
            max: UNIVERSE.max,
        }
    }
}

impl Interval {
    pub fn contains(&self, x: f32) -> bool {
        (self.min..=self.max).contains(&x)
    }

    pub fn surrounds(&self, x: f32) -> bool {
        ((self.min + f32::EPSILON)..self.max).contains(&x)
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {return self.min;}
        if x > self.max {return self.max;}
        x
    }
}

pub const EMPTY: Interval = Interval {min: f32::INFINITY, max: -f32::INFINITY};
pub const UNIVERSE: Interval = Interval {min: -f32::INFINITY, max: f32::INFINITY};