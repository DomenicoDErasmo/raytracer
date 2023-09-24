use super::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, time: f32) -> Point3 {
        self.origin + time * self.direction
    }
}