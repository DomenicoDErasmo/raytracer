use crate::{color::Color, material::Material, vec::{reflect, random_unit_vector}, ray::Ray,};

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(color: &Color, fuzz: f32) -> Self {
        Self {
            albedo: *color,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
            &self,
            in_ray: &crate::ray::Ray,
            hit_record: &crate::hit_record::HitRecord,
            attenuation: &mut Color,
            scattered: &mut crate::ray::Ray
        ) -> bool {
        let reflected = reflect(&in_ray.direction.unit_vector(), &hit_record.normal);
        *scattered = Ray {
            origin: hit_record.point, 
            direction: reflected + self.fuzz * random_unit_vector()
        };
        *attenuation = self.albedo;
        scattered.direction.dot(&hit_record.normal) > 0.0
    }
}