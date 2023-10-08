use crate::{
    color::Color, 
    material::Material, 
    vec::random_unit_vector, 
    ray::Ray, 
    hit_record::HitRecord
};

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
            &self,
            _: &Ray,
            hit_record: &HitRecord,
            attenuation: &mut Color,
            scattered: &mut Ray
        ) -> bool {
        let mut scatter_direction = hit_record.normal + random_unit_vector();
        if scatter_direction.near_zero() {scatter_direction = hit_record.normal;}
        *scattered = Ray {origin: hit_record.point, direction: scatter_direction};
        *attenuation = self.albedo;
        true
    }
}