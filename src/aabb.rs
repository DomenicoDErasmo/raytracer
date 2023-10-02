use crate::axis::{Axis, AxisIndex};

use super::{vec::Point3, ray::Ray, interval::Interval};
use strum::IntoEnumIterator;

/// Refers to an axis-aligned bounding box
#[derive(Clone, Copy, Default)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn from_points(a: &Point3, b: &Point3) -> Self {
        Self {
            x: Interval {max: a.x.max(b.x), min: a.x.min(b.x)},
            y: Interval {max: a.y.max(b.y), min: a.y.min(b.y)},
            z: Interval {max: a.z.max(b.z), min: a.z.min(b.z)},
        }
    }

    pub fn from_boxes(box0: &AABB, box1: &AABB) -> Self {
        Self {
            x: Interval::from_two_intervals(&box0.x, &box1.x),
            y: Interval::from_two_intervals(&box0.y, &box1.y),
            z: Interval::from_two_intervals(&box0.z, &box1.z),
        }
    }

    pub fn hit(&self, ray: &Ray, mut ray_time: Interval) -> bool {
        for axis in Axis::iter() {
            let inverse_direction = 1.0 / ray.direction.axis(&axis);
            let origin = ray.origin.axis(&axis);

            let mut t0 = (self.axis(&axis).min - origin) * inverse_direction;
            let mut t1 = (self.axis(&axis).max - origin) * inverse_direction;

            if inverse_direction < 0.0 {
                (t0, t1) = (t1, t0);
            }

            ray_time.min = ray_time.min.max(t0);
            ray_time.max = ray_time.max.min(t1);

            if ray_time.max <= ray_time.min {
                return false;
            }
        }
        true
    }
}

impl AxisIndex<Interval> for AABB {
    fn axis(&self, axis: &Axis) -> Interval {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
            Axis::Z => self.z,
        }
    }
}