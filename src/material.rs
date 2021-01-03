use std::sync::Arc;

use crate::{hittable::*, ray::*, texture::*, vec3::*};

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &mut HitRecord) -> (bool, Ray, Colour);
}

pub struct Lambertian {
    albedo: Arc<T>,
}

impl Lambertian {
    pub fn new(albedo: Arc<T>) -> Self {
        Lambertian { albedo }
    }

    pub fn colour(colour: Colour) -> Self {
        Lambertian { albedo: Arc::new(SolidColour::new(colour)) }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, hit_record: &mut HitRecord) -> (bool, Ray, Colour) {
        let mut scatter_direction = hit_record.n() + Vec3::random_on_unit_sphere();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.n();
        }

        let ray_scattered = Ray::new(hit_record.p(), scatter_direction, ray_in.time());
        let attentuation = self.albedo.value(hit_record.tp(), hit_record.p());

        (true, ray_scattered, attentuation)
    }
}

pub struct Metal {
    albedo: Colour,
    fuzz: F,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: F) -> Metal {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &mut HitRecord) -> (bool, Ray, Colour) {
        let reflected = reflect(ray_in.direction(), hit_record.n());

        let ray_scattered = Ray::new(
            hit_record.p(),
            reflected + Vec3::random_on_unit_sphere() * self.fuzz,
            ray_in.time(),
        );
        let attentuation = self.albedo;

        (
            dot(&ray_scattered.direction(), &hit_record.n()) > 0.0,
            ray_scattered,
            attentuation,
        )
    }
}

pub struct Dielectric {
    refractive_index: F,
}

impl Dielectric {
    pub fn new(refractive_index: F) -> Dielectric {
        Dielectric { refractive_index }
    }

    fn reflectance(&self, cosine: F, refractive_ratio: F) -> F {
        let r0 = ((1.0 - refractive_ratio) / (1.0 + refractive_ratio)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &mut HitRecord) -> (bool, Ray, Colour) {
        let refractive_ratio = if hit_record.front_face() {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = ray_in.direction().unit();
        let cos_theta = dot(&(-unit_direction), &hit_record.n()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refractive_ratio * sin_theta > 1.0;
        let direction: Vec3;

        if cannot_refract || self.reflectance(cos_theta, refractive_ratio) > random() {
            direction = reflect(unit_direction, hit_record.n());
        } else {
            direction = refract(unit_direction, hit_record.n(), refractive_ratio);
        }

        let ray_scattered = Ray::new(hit_record.p(), direction, ray_in.time());
        let attentuation = Colour::new(1.0, 1.0, 1.0);

        (true, ray_scattered, attentuation)
    }
}
