use std::sync::Arc;

use crate::{hittable::*, ray::*, vec3::*};

pub struct Sphere {
    centre: Point3,
    radius: F,
    material: Arc<M>,
}

impl Sphere {
    pub fn new(centre: Point3, radius: F, material: Arc<M>) -> Sphere {
        Sphere { centre, radius, material }
    }

    pub fn _centre(&self) -> Point3 {
        self.centre
    }

    pub fn _radius(&self) -> F {
        self.radius
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
        let mut hit_record = HitRecord::new(p, outward_normal, root, true, Arc::clone(&self.material));
        hit_record.set_face_normal(ray, outward_normal);

        Some(hit_record)
    } 
}