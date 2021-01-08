use crate::{hittable::*, vec3::*};

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    time: F,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: F) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
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

pub fn ray_colour(ray_in: &Ray, background: Colour, world: &HittableList, depth: u32) -> Colour {
    if depth <= 0 {
        Colour::zero()
    } else if let Some(mut hit_record) = world.hit(ray_in, 0.001, f64::INFINITY) {
        let emitted = hit_record.material().emit(hit_record.tp(), hit_record.p());

        if let Some(scatter_record) = hit_record.material().scatter(ray_in, &mut hit_record) {
            emitted
                + ray_colour(scatter_record.ray(), background, world, depth - 1)
                    * scatter_record.colour()
        } else {
            emitted
        }
    } else {
        background
    }
}
