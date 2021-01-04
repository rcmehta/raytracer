use rand::{thread_rng, Rng};
use std::sync::Arc;

use crate::{aabb::AABB, hittable::*, ray::Ray, vec3::*};

pub struct BVH {
    bbox: AABB,
    left: Arc<H>,
    right: Arc<H>,
}

impl BVH {
    pub fn new(src_objects: &HittableList, start: usize, end: usize, time0: F, time1: F) -> Self {
        let left: Arc<H>;
        let right: Arc<H>;

        let objects = src_objects.clone();

        let mut rng = thread_rng();
        let axis = rng.gen_range(0, 3) as usize;

        let object_span = end - start;

        match object_span {
            1 => {
                left = Arc::clone(&(objects.ix(start)));
                right = Arc::clone(&(objects.ix(start)));
            },
            2 => {
                if AABB::box_compare(objects.ix(start), objects.ix(start + 1), axis) {
                    left = Arc::clone(&(objects.ix(start)));
                    right = Arc::clone(&(objects.ix(start + 1)));
                } else {
                    left = Arc::clone(&(objects.ix(start + 1)));
                    right = Arc::clone(&(objects.ix(start)));
                }
            },
            _ => {
                let mid = start + object_span  / 2;

                left = Arc::new(BVH::new(objects, start, mid, time0, time1));
                right = Arc::new(BVH::new(objects, mid, end, time0, time1)); 
            },
        }

        let box_left = left.bounding_box(time0, time1);
        let box_right = right.bounding_box(time0, time1);

        if let (Some(box_left), Some(box_right)) = (box_left, box_right) {
            let bbox = AABB::surrounding_box(box_left, box_right);
            BVH { bbox, left, right }
        } else {
            panic!("No bounding box in bvh node constructor")
        }
    }
}

impl Hittable for BVH {
    fn hit(&self, ray: &Ray, t_min: F, t_max: F) -> Option<HitRecord> {
        if self.bbox.hit(&ray, t_min, t_max) {
            let hit_record = self.left.hit(&ray, t_min, t_max);

            let t_max = if let Some(ref record) = hit_record { 
                record.t() 
            } else { 
                t_max 
            };

            if let Some(record) = self.right.hit(&ray, t_min, t_max) {
                return Some(record);
            }

            hit_record
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: F, _time1: F) -> Option<AABB> {
        Some(self.bbox)
    }
}

