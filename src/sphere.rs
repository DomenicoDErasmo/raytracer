use crate::hittable::Hittable;
use crate::hit_record::HitRecord;
use crate::ray::Ray;
use super::vec3::Point3;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(
        &mut self, 
        ray: &Ray, 
        ray_tmin: f32, 
        ray_tmax: f32, 
        hit_record: 
        &mut HitRecord
    ) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {return false;}
        let sqrt_discriminat = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_discriminat) / a;
        if !(ray_tmin..=root).contains(&root) {
            root = (-half_b + sqrt_discriminat) / a;
            if !(ray_tmin..=ray_tmax).contains(&root) {return false;}
        }

        hit_record.time = root;
        hit_record.point = ray.at(hit_record.time);
        let outward_normal = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        return true;
    }
}