use crate::{ray::*, vec3::*};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: F,
    time0: F,   // shutter open
    time1: F,   // shutter close
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
        vertical_fov: F,
        aspect_ratio: F,
        aperture: F,
        focus_distance: F,
        time0: F,
        time1: F,
    ) -> Camera {
        let theta = deg_to_rad(vertical_fov);
        let h = (theta / 2.0).tan();
        let viewport_height: F = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit();
        let u = cross(&v_up, &w);
        let v = cross(&w, &u);

        let origin = look_from;
        let horizontal = u * viewport_width * focus_distance;
        let vertical = v * viewport_height * focus_distance;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w * focus_distance;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            lens_radius,
            time0,
            time1,
        }
    }

    pub fn get_ray(&self, s: F, t: F) -> Ray {
        let rd = Vec3::random_in_unit_disc() * self.lens_radius;
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * s + self.vertical * t
                - (self.origin + offset),
            random_range(self.time0, self.time1),
        )
    }
}
