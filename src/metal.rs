use crate::{color::Color, material::Material, vec::reflect, ray::Ray,};

#[derive(Clone)]
pub struct Metal {
    pub albedo: Color,
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
        *scattered = Ray {origin: hit_record.point, direction: reflected};
        *attenuation = self.albedo;
        true
    }
}