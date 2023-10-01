use crate::{material::Material, color::Color, vec::{refract, reflect}, ray::Ray, util::random_double};

#[derive(Clone)]
pub struct Dielectric {
    pub ir: f32,
}

/// Uses Schlick's approximation for reflectance.
fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0-cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(
            &self,
            in_ray: &crate::ray::Ray,
            hit_record: &crate::hit_record::HitRecord,
            attenuation: &mut crate::vec::Vec3,
            scattered: &mut crate::ray::Ray
        ) -> bool {
        *attenuation = Color {x: 1.0, y: 1.0, z: 1.0};
        let refraction_ratio = if hit_record.front_face {1.0 / self.ir} else {self.ir};
        let unit_direction = in_ray.direction.unit_vector();
        let cos_theta = -unit_direction.dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract 
                || reflectance(cos_theta, refraction_ratio) 
                > random_double(None, None) {
            reflect(&unit_direction, &hit_record.normal)
        } else {
            refract(&unit_direction, &hit_record.normal, refraction_ratio)
        };
        *scattered = Ray {origin: hit_record.point, direction, time: in_ray.time};
        true
    }
}