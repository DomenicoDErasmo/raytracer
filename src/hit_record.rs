use crate::ray::Ray;

use super::vec::{Point3, Vec3};

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub time: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        // outward_normal is assumed to have unit length

        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else {-*outward_normal};
    }
}