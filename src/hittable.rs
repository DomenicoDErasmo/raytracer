use crate::{ray::Ray, hit_record::HitRecord, interval::Interval};

pub trait Hittable {
    fn hit(&mut self, ray: &Ray, ray_time: Interval, hit_record: &mut HitRecord) -> bool;
}