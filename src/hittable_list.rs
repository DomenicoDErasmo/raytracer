use crate::{hittable::Hittable, hit_record::HitRecord};

pub struct HittableList {
    pub objects: std::vec::Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.insert(0, object);
    }
}

impl Hittable for HittableList {
    fn hit(
        &mut self, 
        ray: &crate::ray::Ray, 
        ray_tmin: f32, 
        ray_tmax: f32, 
        hit_record: &mut HitRecord
    ) -> bool {
        let mut temp_hit_record = HitRecord::default();    
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in self.objects.iter_mut() {
            if object.hit(ray, ray_tmin, closest_so_far, &mut temp_hit_record) {
                hit_anything = true;
                closest_so_far = temp_hit_record.time;
                *hit_record = temp_hit_record;
            }
        }
        
        hit_anything
    }
}