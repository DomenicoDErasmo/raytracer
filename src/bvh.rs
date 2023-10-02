use crate::{hittable::Hittable, axis::{Axis, AxisIndex}, aabb::AABB, interval::Interval, util::random_integer};

/// A Bounded Volume Hierarchy tree, which lets us use bianry search to find bounding volumes.
#[derive(Default)]
pub struct BVHNode {
    pub bounding_box: AABB,
    pub left: Option<Box<dyn Hittable>>,
    pub right: Option<Box<dyn Hittable>>,
}

impl BVHNode {
    pub fn from_objects_and_times(
        source_objects: &Vec<Box<dyn Hittable>>, 
        start: usize, 
        end: usize,
    ) -> Self {
        let mut result = Self::default();

        let axis = random_integer(0, 2);
        let object_span = end - start;

        if object_span  == 1 {
            result.left = *source_objects.first();    
        }

        result
    }
}

impl Hittable for BVHNode {
    fn bounding_box(&self) -> crate::aabb::AABB {
        self.bounding_box
    }

    fn hit(
        &mut self, 
        ray: &crate::ray::Ray, 
        ray_time: crate::interval::Interval, 
        hit_record: &mut crate::hit_record::HitRecord
    ) -> bool {
        if !self.bounding_box.hit(ray, ray_time) {
            return false;
        }

        let hit_left = match &mut self.left {
            Some(left_obj) => left_obj.hit(
                ray, 
                ray_time, 
                hit_record
            ),
            None => false,
        };
        let hit_right = match &mut self.right {
            Some(right_obj) => right_obj.hit(
                ray, 
                Interval {
                    min: ray_time.min, 
                    max: if hit_left {hit_record.time} else {ray_time.max},
                }, 
                hit_record
            ),
            None => false,
        };

        hit_left || hit_right
    }
}

pub fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: Axis) -> bool {
    a.bounding_box().axis(&axis).min < b.bounding_box().axis(&axis).min
}
