use crate::{ray::Ray, hit_record::HitRecord, color::Color};
use dyn_clone::DynClone;

dyn_clone::clone_trait_object!(Material);

pub trait Material: DynClone {
    fn scatter(
        &self,
        in_ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray
    ) -> bool;
}