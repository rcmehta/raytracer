use std::sync::Arc;

use crate::{hittable::H, ray::Ray, vec3::*};

#[derive(Clone, Copy)]
pub struct AABB {
    min: Point3, 
    max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> Self {
        Self { min, max }
    }

    pub fn min(&self) -> Point3 {
        self.min
    }

    pub fn max(&self) -> Point3 {
        self.max
    }

    pub fn surrounding_box(box0: Self, box1: Self) -> Self {
        let min_point = Point3::new(
            box0.min().x().min(box1.min().x()),
            box0.min().y().min(box1.min().y()),
            box0.min().z().min(box1.min().z()),
        );

        let max_point = Point3::new(
            box0.max().x().max(box1.max().x()),
            box0.max().y().max(box1.max().y()),
            box0.max().z().max(box1.max().z()),
        );

        Self { min: min_point, max: max_point }
    }

    pub fn box_compare(a: Arc<H>, b: Arc<H>, axis: usize) -> bool {
        let box_a = a.bounding_box(0.0, 0.0);
        let box_b = b.bounding_box(0.0, 0.0);

        if let (Some(box_a), Some(box_b)) = (box_a, box_b) {
            box_a.min().ix(axis) < box_b.min().ix(axis)
        } else {
            panic!("No bounding box in bvh node constructor")
        }
    }

    pub fn hit(&self, ray: &Ray, mut t_min: F, mut t_max: F) -> bool {
        for i in 0..3 {
            let inv = 1.0 / ray.direction().ix(i);

            // For ray P(t) = A + tB to plane x = x0
            // t0 = (x0 - A0) / B0
            // Similarly for x=x1, use min, max to find range

            let mut t0 =
                ((self.min.ix(i) - ray.origin().ix(i)) * inv).min
                ((self.max.ix(i) - ray.origin().ix(i)) * inv);
            let mut t1 = 
                ((self.min.ix(i) - ray.origin().ix(i)) * inv).max
                ((self.max.ix(i) - ray.origin().ix(i)) * inv);

            if inv < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            if t_max <= t_min {
                return false;
            }
        }

        return true;
    }
}