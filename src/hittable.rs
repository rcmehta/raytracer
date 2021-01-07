use std::sync::Arc;

use crate::{aabb::AABB, material::*, ray::*, texture::*, vec3::*};

pub type H = dyn Hittable + Send + Sync;
pub type M = dyn Material + Send + Sync;
pub type T = dyn Texture + Send + Sync;

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: F, t_max: F) -> Option<HitRecord>;
    fn bounding_box(&self, time0: F, time1: F) -> Option<AABB>;
}

pub struct HitRecord {
    p: Point3,
    n: Vec3,
    t: F,
    tp: TexturePoint,
    front_face: bool,
    material: Arc<M>,
}

impl HitRecord {
    pub fn new(p: Point3, n: Vec3, t: F, tp: TexturePoint, front_face: bool, material: Arc<M>) -> HitRecord {
        HitRecord {
            p,
            n,
            t,
            tp,
            front_face,
            material,
        }
    }

    pub fn p(&self) -> Point3 {
        self.p
    }
    pub fn n(&self) -> Vec3 {
        self.n
    }
    pub fn t(&self) -> F {
        self.t
    }

    pub fn tp(&self) -> TexturePoint {
        self.tp
    }
    pub fn front_face(&self) -> bool {
        self.front_face
    }
    pub fn material(&self) -> Arc<M> {
        Arc::clone(&self.material)
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = dot(&ray.direction(), &outward_normal) < 0.0;
        self.n = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn set_p(&mut self, new_p: Point3) {
        self.p = new_p;
    }
}

pub struct HittableList {
    objects: Vec<Arc<H>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<H>) {
        self.objects.push(object)
    }

    pub fn _clear(&mut self) {
        self.objects.clear()
    }

    pub fn ix(&self, index: usize) -> Arc<H> {
        Arc::clone(&self.objects[index])
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: F, t_max: F) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest = t_max;

        for object in self.objects.iter() {
            if let Some(temp_record) = object.hit(ray, t_min, closest) {
                closest = temp_record.t();
                hit_record = Some(temp_record);
            }
        }

        hit_record
    }

    fn bounding_box(&self, time0: F, time1: F) -> Option<AABB> {
        let mut output_box: Option<AABB> = None;

        if self.objects.is_empty() { return output_box; }

        for object in self.objects.iter() {
            if let Some(temp_box) = object.bounding_box(time0, time1) {
                if let Some(current) = output_box {
                    output_box = Some(AABB::surrounding_box(current, temp_box));
                } else {
                    output_box = Some(temp_box);
                }
            } else {
                return None;
            }
        }
        
        output_box        
    }
}
