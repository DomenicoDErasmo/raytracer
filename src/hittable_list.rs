use crate::{hittable::Hittable, hit_record::HitRecord, interval::Interval};

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
        ray_time: Interval,
        hit_record: &mut HitRecord
    ) -> bool {
        let mut temp_hit_record = HitRecord::default();    
        let mut hit_anything = false;
        let mut closest_so_far = ray_time.max;

        for object in self.objects.iter_mut() {
            if object.hit(
                ray, 
                Interval {min: ray_time.min, max: closest_so_far}, 
                &mut temp_hit_record
            ) {
                hit_anything = true;
                closest_so_far = temp_hit_record.time;
                *hit_record = temp_hit_record;
            }
        }
        
        hit_anything
    }
}