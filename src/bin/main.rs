extern crate image;
extern crate rand;
extern crate rayon;

use image::io::Reader as ImageReader;
use rayon::prelude::*;

use std::{error::Error, fs, io::Write, sync::Arc};

use raytracer::{camera::*, hittable::*, material::*, moving_sphere::*, ray::*, sphere::*, vec3::*};

fn main() -> Result<(), Box<dyn Error>> {
    // Image Constants
    const ASPECT_RATIO: F = 16.0 / 9.0;
    const IMAGE_HEIGHT: u32 = 360;
    const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as F * ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // World
    let world = random_scene();

    // Camera
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let v_up = Point3::new(0.0, 1.0, 0.0);
    let distance_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        20.0,
        ASPECT_RATIO,
        aperture,
        distance_to_focus,
        0.0, 1.0,
    );

    // Create and initialise .ppm file
    let name = "image".to_string();
    let ppm_path = name + ".ppm";
    let mut f = fs::File::create(&ppm_path)?;

    write!(f, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rRemaining: {} / {}", j, IMAGE_HEIGHT);
        for i in 0..IMAGE_WIDTH {
            let colour: Colour = (0..SAMPLES_PER_PIXEL)
                .into_par_iter()
                .map(|_| {
                    let u = (i as F + random()) / (IMAGE_WIDTH - 1) as F;
                    let v = (j as F + random()) / (IMAGE_HEIGHT - 1) as F;
                    let ray = camera.get_ray(u, v);
                    let local_colour = ray_colour(&ray, &world, MAX_DEPTH);
                    local_colour
                })
                .sum();
            write!(f, "\n{}", colour.write_colour(SAMPLES_PER_PIXEL))?;
        }
    }

    // Convert to .png
    eprintln!("\rConverting to .png");

    let img = ImageReader::open(&ppm_path)?.decode()?;

    let png_path = ppm_path.replace(".ppm", ".png");
    img.save_with_format(png_path, image::ImageFormat::Png)?;

    eprintln!("\rDone.");

    Ok(())
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random();
            let centre = Point3::new(a as F + 0.9 * random(), 0.2, b as F + 0.9 * random());

            if (centre - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<M>;
                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Colour::random_vector(0.0, 1.0) * Colour::random_vector(0.0, 1.0);
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    
                    let centre1 = centre + Vec3::new(0.0, random_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(MovingSphere::new(centre, centre1, 0.0, 1.0, 0.2, sphere_material)));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Colour::random_vector(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(centre, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(centre, 0.2, sphere_material)));
                }

            }
        }
    }

    let lambertian = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        lambertian,
    )));

    let dielectric = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        dielectric,
    )));

    let metal = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        metal,
    )));

    world
}