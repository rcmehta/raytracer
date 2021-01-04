use crate::{hittable::*, vec3::*};

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: F,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: F) -> Ray {
        Ray { origin, direction, time }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> F {
        self.time
    }

    pub fn at(&self, t: F) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub fn ray_colour(ray_in: &Ray, world: &HittableList, depth: u32) -> Colour {
    if depth <= 0 {
        Colour::zero()
    } else if let Some(mut hit_record) = world.hit(ray_in, 0.001, f64::INFINITY) {
        if let (true, ray_scattered, attentuation) =
            hit_record.material().scatter(ray_in, &mut hit_record)
        {
            ray_colour(&ray_scattered, world, depth - 1) * attentuation
        } else {
            Colour::zero()
        }
    } else {
        let unit_direction = ray_in.direction().unit();
        let t = (unit_direction.y() + 1.0) * 0.5;
        Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
    }
}
