use std::sync::Arc;

use crate::{aabb::*, aarect::*, hittable::*, ray::*, vec3::*};

pub struct Translate {
    object: Arc<H>,
    offset: Vec3,
}

impl Translate {
    pub fn new(object: Arc<H>, offset: Vec3) -> Self {
        Self { object, offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, t_min: F, t_max: F) -> Option<HitRecord> {
        let translated_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());

        if let Some(mut hit_record) = self.object.hit(&translated_ray, t_min, t_max) {
            hit_record.set_p(hit_record.p() + self.offset);
            hit_record.set_face_normal(&translated_ray, hit_record.n());
            Some(hit_record)
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: F, time1: F) -> Option<AABB> {
        if let Some(output_box) = self.object.bounding_box(time0, time1) {
            Some(AABB::new(output_box.min() + self.offset, output_box.max() + self.offset))
        } else {
            None
        }
    }
}

pub struct Rotate {
    object: Arc<H>,
    plane: Plane,
    cos_theta: F,
    sin_theta: F,
    bbox: Option<AABB>,
}

impl Rotate {
    pub fn new(object: Arc<H>, plane: Plane, theta: F) -> Self {
        let radians = deg_to_rad(theta);
        let cos_theta = radians.cos();
        let sin_theta = radians.sin();

        let bbox = object.bounding_box(0.0, 1.0).unwrap();

        let infinity = f64::INFINITY;
        let mut min = Point3::new(infinity, infinity, infinity);
        let mut max = Point3::new(-infinity, -infinity, -infinity);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as F * bbox.max().x() + (1 - i) as F * bbox.min().x();
                    let y = j as F * bbox.max().y() + (1 - j) as F * bbox.min().y();
                    let z = k as F * bbox.max().z() + (1 - k) as F * bbox.min().z();

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let temp = Vec3::new(new_x, y, new_z);
                    
                    for a in 0..2 {
                        min.set(a, min.ix(a).min(temp.ix(a)));
                        max.set(a, max.ix(a).max(temp.ix(a)));
                    }

                }
            }
        }

        let bbox = Some(AABB::new(min, max));

        Self { object, plane, sin_theta, cos_theta, bbox }
    }
}

impl Hittable for Rotate {
    fn hit(&self, ray: &Ray, t_min: F, t_max: F) -> Option<HitRecord> {
        let (i, j, _k) = self.plane.axes();
        let mut origin = ray.origin();
        let mut direction = ray.direction();

        origin.set(i, self.cos_theta * ray.origin().ix(i) - self.sin_theta * ray.origin().ix(j));
        origin.set(j, self.sin_theta * ray.origin().ix(i) + self.cos_theta * ray.origin().ix(j));

        direction.set(i, self.cos_theta * ray.direction().ix(i) - self.sin_theta * ray.direction().ix(j));
        direction.set(j, self.sin_theta * ray.direction().ix(i) + self.cos_theta * ray.direction().ix(j));

        let rotated_ray = Ray::new(origin, direction, ray.time());

        if let Some(mut hit_record) = self.object.hit(&rotated_ray, t_min, t_max) {
            let mut new_p = hit_record.p();
            new_p.set(i, self.cos_theta * hit_record.p().ix(i) + self.sin_theta * hit_record.p().ix(j));
            new_p.set(j, -self.sin_theta * hit_record.p().ix(i) + self.cos_theta * hit_record.p().ix(j));

            let mut new_n = hit_record.n();
            new_n.set(i, self.cos_theta * hit_record.n().ix(i) + self.sin_theta * hit_record.n().ix(j));
            new_n.set(j, -self.sin_theta * hit_record.n().ix(i) + self.cos_theta * hit_record.n().ix(j));

            hit_record.set_p(new_p);
            hit_record.set_face_normal(&rotated_ray, new_n);

            Some(hit_record)
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: F, _time1: F) -> Option<AABB> {
        self.bbox
    }
}

