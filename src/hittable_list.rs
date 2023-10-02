use crate::{hittable::Hittable, hit_record::HitRecord, interval::Interval, aabb::AABB};

pub struct HittableList {
    pub objects: std::vec::Vec<Box<dyn Hittable>>,
    pub bounding_box: AABB,
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        let bounding_box = object.bounding_box();
        self.objects.insert(0, object);
        self.bounding_box = AABB::from_boxes(
            &self.bounding_box, 
            &bounding_box
        );
    }
}

impl Hittable for HittableList {
    fn hit(
        &mut self, 
        ray: &crate::ray::Ray, 
        ray_time: Interval,
        hit_record: &mut HitRecord
    ) -> bool {
        let temp_hit_record = HitRecord::default();    
        let mut hit_anything = false;
        let mut closest_so_far = ray_time.max;

        for object in self.objects.iter_mut() {
            let mut temp_hit_record_clone = temp_hit_record.clone();
            if object.hit(
                ray, 
                Interval {min: ray_time.min, max: closest_so_far}, 
                &mut temp_hit_record_clone,
            ) {
                hit_anything = true;
                closest_so_far = temp_hit_record_clone.time;
                *hit_record = temp_hit_record_clone;
            }
        }
        
        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bounding_box
    }
}