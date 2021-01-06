use std::sync::Arc;

use crate::{hittable::*, ray::*, texture::*, vec3::*};

pub trait Material {
    fn scatter(&self, _ray_in: &Ray, _hit_record: &mut HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emit(&self, _tp: TexturePoint, _p: Point3) -> Colour {
        Colour::zero()
    }
}

pub struct ScatterRecord {
    ray: Ray,
    colour: Colour,
}

impl ScatterRecord {
    pub fn new(ray: Ray, colour: Colour) -> Self {
        Self { ray, colour } 
    }

    pub fn ray(&self) -> &Ray {
        &self.ray
    }

    pub fn colour(&self) -> Colour {
        self.colour
    }
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
    fn scatter(&self, ray_in: &Ray, hit_record: &mut HitRecord) -> Option<ScatterRecord> {
        let mut scatter_direction = hit_record.n() + Vec3::random_on_unit_sphere();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.n();
        }

        let ray_scattered = Ray::new(hit_record.p(), scatter_direction, ray_in.time());
        let attentuation = self.albedo.value(hit_record.tp(), hit_record.p());

        let scatter_record = ScatterRecord::new(ray_scattered, attentuation);
        Some(scatter_record)
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
    fn scatter(&self, ray_in: &Ray, hit_record: &mut HitRecord) -> Option<ScatterRecord> {
        let reflected = reflect(ray_in.direction(), hit_record.n());

        let ray_scattered = Ray::new(
            hit_record.p(),
            reflected + Vec3::random_on_unit_sphere() * self.fuzz,
            ray_in.time(),
        );
        let attentuation = self.albedo;

        if dot(&ray_scattered.direction(), &hit_record.n()) > 0.0 {
            let scatter_record = ScatterRecord::new(ray_scattered, attentuation);
            Some(scatter_record)
        } else {
            None
        }
            
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
    fn scatter(&self, ray_in: &Ray, hit_record: &mut HitRecord) -> Option<ScatterRecord> {
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
        let attentuation = Colour::one();

        let scatter_record = ScatterRecord::new(ray_scattered, attentuation);
        Some(scatter_record)
    }
}

pub struct DiffuseLight {
    emit: Arc<T>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<T>) -> Self {
        Self { emit }
    }

    pub fn colour(colour: Colour) -> Self {
        Self { emit: Arc::new(SolidColour::new(colour))}
    }
}

impl Material for DiffuseLight {
    fn emit(&self, tp: TexturePoint, p: Point3) -> Colour {
        self.emit.value(tp, p)
    }
}
