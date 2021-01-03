use std::sync::Arc;

use crate::{aabb::AABB, hittable::*, ray::Ray, sphere::Sphere, vec3::*};

pub struct MovingSphere {
    centre0: Point3, centre1: Point3,
    time0: F, time1: F,
    radius: F,
    material: Arc<M>,
}

impl MovingSphere {
    pub fn new(
        centre0: Point3, centre1: Point3,
        time0: F, time1: F,
        radius: F,
        material: Arc<M>,
    ) -> Self {
        MovingSphere { centre0, centre1, time0, time1, radius, material }
    }

    pub fn centre(&self, time: F) -> Point3 {
        self.centre0 + (self.centre1 - self.centre0) * ((time - self.time0) / (self.time1 - self.time0))
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: F, t_max: F) -> Option<HitRecord> {
        let oc = ray.origin() - self.centre(ray.time());
        let a = dot(&ray.direction(), &ray.direction());
        let half_b = dot(&oc, &ray.direction());
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;

        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = ray.at(root);
        let outward_normal = (p - self.centre(ray.time())) / self.radius;
        let tp = Sphere::tp(outward_normal);
        let mut hit_record =
            HitRecord::new(p, outward_normal, root, tp, true, Arc::clone(&self.material));
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }

    fn bounding_box(&self, time0: F, time1: F) -> Option<AABB> {
        let radius_vector = Vec3::new(self.radius, self.radius, self.radius);

        let box0 = AABB::new(
            self.centre(time0) - radius_vector,
            self.centre(time1) + radius_vector,
        );
        let box1 = AABB::new(
            self.centre(time1) - radius_vector,
            self.centre(time1) + radius_vector,
        );

        Some(AABB::surrounding_box(box0, box1))
    }
}
