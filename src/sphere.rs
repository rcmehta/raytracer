use std::sync::Arc;

use crate::{aabb::AABB, hittable::*, ray::*, texture::*, vec3::*};

pub struct Sphere {
    centre: Point3,
    radius: F,
    material: Arc<M>,
}

impl Sphere {
    pub fn new(centre: Point3, radius: F, material: Arc<M>) -> Sphere {
        Sphere {
            centre,
            radius,
            material,
        }
    }

    pub fn _centre(&self) -> Point3 {
        self.centre
    }

    pub fn _radius(&self) -> F {
        self.radius
    }

    pub fn tp(p: Point3) -> TexturePoint {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        
        const PI: F = std::f64::consts::PI;

        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        TexturePoint::new(u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: F, t_max: F) -> Option<HitRecord> {
        let oc = ray.origin() - self.centre;
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
        let outward_normal = (p - self.centre) / self.radius;
        let tp = Sphere::tp(outward_normal);
        let mut hit_record =
            HitRecord::new(p, outward_normal, root, tp, true, Arc::clone(&self.material));
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    }

    fn bounding_box(&self, _time0: F, _time1: F) -> Option<AABB> {
        let radius_vector = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(
            self.centre - radius_vector,
            self.centre + radius_vector,
        ))
    }
}
