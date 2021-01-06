use std::sync::Arc;

use crate::{aabb::AABB, hittable::*, ray::Ray, texture::*, vec3::*};

pub enum Plane {
    XY,
    YZ,
    ZX,
}

pub struct AARect {
    plane: Plane,
    a0: F,
    a1: F,
    b0: F,
    b1: F,
    k: F,
    material: Arc<M>,
}

impl AARect {
    pub fn new(plane: Plane, a0: F, a1: F, b0: F, b1: F, k: F, material: Arc<M>) -> Self {
        Self { plane, a0, a1, b0, b1, k, material }
    }

    pub fn axes(&self) -> (usize, usize, usize) {
        // returns in order (a, b, k)
        match self.plane {
            Plane::XY => (0, 1, 2),
            Plane::YZ => (1, 2, 0),
            Plane::ZX => (2, 0, 1),
        }
    }
}

impl Hittable for AARect {
    fn hit(&self, ray: &Ray, t_min: F, t_max: F) -> Option<HitRecord> {
        let (a_axis, b_axis, k_axis) = self.axes();

        let t = (self.k - ray.origin().ix(k_axis)) / ray.direction().ix(k_axis);

        if t < t_min || t > t_max {
            return None;
        } else {
            let a = ray.origin().ix(a_axis) + t * ray.direction().ix(a_axis);
            let b = ray.origin().ix(b_axis) + t * ray.direction().ix(b_axis);

            if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
                return None;
            } else {
                let p = ray.at(t);

                let mut outward_normal = Vec3::zero();
                outward_normal.set(k_axis, 1.0);
                
                let u = (a - self.a0) / (self.a1 - self.a0);
                let v = (b - self.b0) / (self.b1 - self.b0);
                let tp = TexturePoint::new(u, v);

                let mut hit_record = HitRecord::new(p, outward_normal, t, tp, true, Arc::clone(&self.material));
                hit_record.set_face_normal(ray, outward_normal);

                return Some(hit_record);
            }
        }
    }

    fn bounding_box(&self, _time0: F, _time1: F) -> Option<AABB> {
        let order = self.axes();

        let mut min = Point3::zero();
        min.set_all(order, (self.a0, self.b0, self.k - 0.001));
        let mut max = Point3::zero();
        max.set_all(order, (self.a1, self.b1, self.k + 0.001)); 

        Some(AABB::new(min, max))

    }
}


