use std::{collections::VecDeque, cmp::Ordering};

use crate::{hittable::Hittable, axis::{Axis, AxisIndex}, aabb::AABB, interval::Interval, util::random_integer};

/// A Bounded Volume Hierarchy tree, which lets us use bianry search to find bounding volumes.
#[derive(Default)]
pub struct BVHNode {
    pub bounding_box: AABB,
    pub left: Option<Box<dyn Hittable>>,
    pub right: Option<Box<dyn Hittable>>,
}

// TODO: fix
impl BVHNode {
    /// Uses a binary sort to sort objects into a tree.
    pub fn from_objects_and_times(
        source_objects: &mut VecDeque<Box<dyn Hittable>>, 
        start: usize, 
        end: usize,
    ) -> Self {
        let mut result = Self::default();

        let axis = random_integer(0, 2);
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        if source_objects.len()  == 1 {
            result.left = source_objects.pop_front(); 
            result.bounding_box = result.left.as_ref().unwrap().bounding_box();
        } else if source_objects.len() == 2 {
            match source_objects.get(start) {
                Some(_) => {},
                None => {
                    println!("Empty when shouldn't be.")
                }
            }
            match comparator(
                source_objects.get(start).unwrap(),
                source_objects.get(start + 1).unwrap(),
            ) {
                Ordering::Less => {
                    result.left = source_objects.pop_front();
                    result.right = source_objects.pop_back();
                },
                _ => {
                    result.left = source_objects.pop_back();
                    result.right = source_objects.pop_front();
                }
            }

            let left_reference = result.left.as_ref().unwrap();
            let right_reference = result.right.as_ref().unwrap();
            result.bounding_box = AABB::from_boxes(
                &left_reference.bounding_box(), 
                &right_reference.bounding_box(),
            );
        } else {
            source_objects.make_contiguous().sort_by(comparator);
            let mid = start + source_objects.len() / 2;
            // TODO: instetad of passing all of source_objects, restrict what I pass in
            let mut left_vec_deque = VecDeque::new();
            for i in 0..mid {
                left_vec_deque.push_back(source_objects.get(i).unwrap().clone());
            }
            let mut right_vec_deque = VecDeque::new();
            for i in mid..end {
                right_vec_deque.push_back(source_objects.get(i).unwrap().clone());
            }
            result.left = Some(
                Box::<_>::new(
                    BVHNode::from_objects_and_times(
                        &mut left_vec_deque, 
                        start, 
                        mid
                    )
                )
            );
            result.right = Some(
                Box::<_>::new(
                    BVHNode::from_objects_and_times(
                        &mut right_vec_deque, 
                        mid, 
                        end
                    )
                )
            );

            let left_reference = result.left.as_ref().unwrap();
            let right_reference = result.right.as_ref().unwrap();
            result.bounding_box = AABB::from_boxes(
                &left_reference.bounding_box(), 
                &right_reference.bounding_box(),
            );
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

pub fn box_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>, axis: Axis) -> Ordering {
    a.bounding_box().axis(&axis).min.total_cmp(&b.bounding_box().axis(&axis).min)
}

pub fn box_x_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, Axis::X)
}

pub fn box_y_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, Axis::Y)
}

pub fn box_z_compare(a: &Box<dyn Hittable>, b: &Box<dyn Hittable>) -> Ordering {
    box_compare(a, b, Axis::Z)
}
