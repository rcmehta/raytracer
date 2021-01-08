use std::sync::Arc;

use crate::{aabb::AABB, hittable::*, material::*, ray::Ray, vec3::*};

const INFINITY: f64 = f64::INFINITY;
const DT: f64 = 10e-4;

pub struct ConstantMedium {
    boundary: Arc<H>,
    phase_function: Arc<M>,
    neg_inv_density: F,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<H>, texture: Arc<T>, density: F) -> Self {
        let neg_inv_density = -1.0 / density;
        let phase_function = Arc::new(Isotropic::new(texture));
        Self {
            boundary,
            phase_function,
            neg_inv_density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, t_min: F, t_max: F) -> Option<HitRecord> {
        if let Some(mut hit_record1) = self.boundary.hit(ray, -INFINITY, INFINITY) {
            if let Some(mut hit_record2) = self.boundary.hit(ray, hit_record1.t() + DT, INFINITY) {
                if hit_record1.t() < t_min.max(0.0) {
                    hit_record1.set_t(t_min.max(0.0));
                }
                if hit_record2.t() > t_max {
                    hit_record2.set_t(t_max);
                }

                if hit_record1.t() > hit_record2.t() {
                    None
                } else {
                    let ray_length = ray.direction().length();
                    let distance_in_boundary = (hit_record2.t() - hit_record1.t()) * ray_length;
                    let hit_distance = self.neg_inv_density * random().ln();

                    if hit_distance > distance_in_boundary {
                        None
                    } else {
                        let t = hit_record1.t() + hit_distance / ray_length;
                        let p = ray.at(t);
                        let n = Vec3::new(1.0, 0.0, 0.0); // + front_face arbitrary
                        let tp = hit_record1.tp(); // arbitrary
                        let hit_record =
                            HitRecord::new(p, n, t, tp, true, Arc::clone(&self.phase_function));
                        Some(hit_record)
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self, time0: F, time1: F) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}

pub struct Isotropic {
    albedo: Arc<T>,
}

impl Isotropic {
    pub fn new(albedo: Arc<T>) -> Self {
        Self { albedo }
    }
}

impl Material for Isotropic {
    fn scatter(&self, ray_in: &Ray, hit_record: &mut HitRecord) -> Option<ScatterRecord> {
        let ray_scattered = Ray::new(hit_record.p(), Vec3::random_on_unit_sphere(), ray_in.time());
        let attentuation = self.albedo.value(hit_record.tp(), hit_record.p());

        Some(ScatterRecord::new(ray_scattered, attentuation))
    }
}
