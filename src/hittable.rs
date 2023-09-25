use crate::{ray::Ray, hit_record::HitRecord};

pub trait Hittable {
    fn hit(&mut self, ray: &Ray, ray_tmin: f32, ray_tmax: f32, hit_record: &mut HitRecord) -> bool;
}