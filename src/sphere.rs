use crate::{hittable::Hittable, aabb::AABB};
use crate::hit_record::HitRecord;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct Sphere {
    /// The center of the sphere.
    pub center: Point3,
    /// The radius of the sphere.
    pub radius: f32,
    /// Determines how rays of light act when they hit the surface.
    pub material: Box<dyn Material>,
    /// Gives the direction that the ball is moving in. 
    /// Should be a zero vector if is_moving is false.
    pub center_vec: Option<Vec3>,
    /// Axis-aligned bounding box
    pub bounding_box: AABB,
}

impl Sphere {
    pub fn make_stationary_sphere(
        center: Point3, 
        radius: f32,
        material: Box<dyn Material>,
    ) -> Self {
        let radius_vector = Vec3 {x: radius, y: radius, z: radius};
        let bounding_box = AABB::from_points(
            &(center - radius_vector), 
            &(center + radius_vector)
        );
        Self {
            center,
            radius,
            material,
            center_vec: None,
            bounding_box,
        }
    }
    pub fn make_moving_sphere(
        start_center: Point3,
        end_center: Point3,
        radius: f32,
        material: Box<dyn Material>,
    ) -> Self {
        let radius_vector = Vec3 {x: radius, y: radius, z: radius};
        let start_bounding_box = AABB::from_points(
            &(start_center - radius_vector), 
            &(start_center + radius_vector)
        );
        let end_bounding_box = AABB::from_points(
            &(end_center - radius_vector), 
            &(end_center + radius_vector)
        );
        let bounding_box = AABB::from_boxes(&start_bounding_box, &end_bounding_box);
        Self {
            center: start_center,
            radius,
            material,
            center_vec: Some(end_center - start_center),
            bounding_box,
        }
    }
}

impl Hittable for Sphere {
    fn hit(
        &mut self, 
        ray: &Ray, 
        ray_time: Interval,
        hit_record: 
        &mut HitRecord
    ) -> bool {
        let center = match self.center_vec {
            Some(vector) => self.center + ray.time * vector,
            None => self.center,
        };
        let oc = ray.origin - center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {return false;}
        let sqrt_discriminat = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_discriminat) / a;
        if !ray_time.surrounds(root) {
            root = (-half_b + sqrt_discriminat) / a;
            if !ray_time.surrounds(root) {return false;}
        }

        hit_record.time = root;
        hit_record.point = ray.at(hit_record.time);
        let outward_normal = (hit_record.point - center) / self.radius;
        hit_record.set_face_normal(ray, &outward_normal);
        hit_record.material = self.material.clone();
        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}