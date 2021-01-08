extern crate image;
extern crate rand;
extern crate rayon;

use image::io::Reader as ImageReader;
use rayon::prelude::*;

use std::{error::Error, fs, io::Write, sync::Arc};

use raytracer::{aarect::*, camera::*, hittable::*, material::*, medium::*, moving_sphere::*, ray::*, sphere::*, texture::*, transform::*, vec3::*};

// Image Constants
const ASPECT_RATIO: F = 1.0;
const IMAGE_HEIGHT: u32 = 600;
const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as F * ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 200;
const MAX_DEPTH: u32 = 50;

fn main() -> Result<(), Box<dyn Error>> {
    // Camera, World
    let (camera, world) = _cornell_smoke();

    // Background
    let background = Colour::new(0.0, 0.0, 0.0);

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
                    let local_colour = ray_colour(&ray, background, &world, MAX_DEPTH);
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

pub fn default_camera() -> Camera {
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::zero();
    let vfov = 20.0;
    let v_up = Point3::new(0.0, 1.0, 0.0);
    let distance_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        vfov,
        ASPECT_RATIO,
        aperture,
        distance_to_focus,
        0.0, 1.0,
    );

    camera
}

fn _random_scene() -> (Camera, HittableList) {
    let mut world = HittableList::new();

    let ground_texture = Arc::new(Checkered::new(
        Arc::new(SolidColour::rgb(0.2, 0.3, 0.1)),
        Arc::new(SolidColour::rgb(0.9, 0.9, 0.9)),
    ));
    let ground_material = Arc::new(Lambertian::new(ground_texture));
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
                    sphere_material = Arc::new(Lambertian::colour(albedo));
                    
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

    let lambertian = Arc::new(Lambertian::rgb(0.4, 0.2, 0.1));
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

    (default_camera(), world)
}

fn _two_spheres() -> (Camera, HittableList) {
    let mut world = HittableList::new();

    let checkered = Arc::new(Checkered::colour(
        Colour::new(0.2, 0.3, 0.1),
        Colour::new(0.9, 0.9, 0.9),
    ));

    let material: Arc<M> = Arc::new(Lambertian::new(checkered));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::clone(&material),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::clone(&material),
    )));

    (default_camera(), world)
}

fn _two_perlin_spheres() -> (Camera, HittableList) {
    let mut world = HittableList::new();

    let perlin = Arc::new(Noise::new(4.0));
    let material: Arc<M> = Arc::new(Lambertian::new(perlin));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&material),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::clone(&material),
    )));

    (default_camera(), world)
}

fn _earth() -> (Camera, HittableList) {
    let mut world = HittableList::new();

    let earth = Arc::new(Image::new("textures/earthmap.jpg"));
    let material: Arc<M> = Arc::new(Lambertian::new(earth));

    world.add(Arc::new(Sphere::new(
        Point3::zero(),
        2.0,
        material,
    )));

    (default_camera(), world)
}

fn _simple_light() -> (Camera, HittableList) {
    let look_from = Point3::new(26.0, 3.0, 6.0);
    let look_at = Point3::new(0.0, 2.0, 0.0);
    let vfov = 20.0;
    let v_up = Point3::new(0.0, 1.0, 0.0);
    let distance_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        vfov,
        ASPECT_RATIO,
        aperture,
        distance_to_focus,
        0.0, 1.0,
    );

    let mut world = HittableList::new();

    let perlin = Arc::new(Noise::new(4.0));
    let material: Arc<M> = Arc::new(Lambertian::new(perlin));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&material),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::clone(&material),
    )));

    let diff_light: Arc<M> = Arc::new(DiffuseLight::rgb(4.0, 4.0, 4.0));

    world.add(Arc::new(AARect::new(Plane::XY, 3.0, 5.0, 1.0, 3.0, -2.0, diff_light)));

    (camera, world)
}

fn _cornell_box() -> (Camera, HittableList) {
    let look_from = Point3::new(278.0, 278.0, -800.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let v_up = Point3::new(0.0, 1.0, 0.0);
    let vfov = 40.0;
    let distance_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        vfov,
        ASPECT_RATIO,
        aperture,
        distance_to_focus,
        0.0, 1.0,
    );

    let mut world = HittableList::new();

    let red: Arc<M> = Arc::new(Lambertian::rgb(0.65, 0.05, 0.05));
    let white: Arc<M> = Arc::new(Lambertian::rgb(0.73, 0.73, 0.73));
    let green: Arc<M> = Arc::new(Lambertian::rgb(0.12, 0.45, 0.15));

    let light: Arc<M> = Arc::new(DiffuseLight::rgb(15.0, 15.0, 15.0));

    world.add(Arc::new(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&green))));
    world.add(Arc::new(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, Arc::clone(&red))));
    world.add(Arc::new(AARect::new(Plane::ZX, 213.0, 343.0, 227.0, 332.0, 554.0, Arc::clone(&light))));
    world.add(Arc::new(AARect::new(Plane::ZX, 0.0, 555.0, 0.0, 555.0, 0.0, Arc::clone(&white))));
    world.add(Arc::new(AARect::new(Plane::ZX, 0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&white))));
    world.add(Arc::new(AARect::new(Plane::XY, 0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&white))));

    let box1 = Arc::new(AABox::new(
        Point3::zero(),
        Point3::new(165.0, 330.0, 165.0), 
        Arc::clone(&white)));
    let box2 = Arc::new(AABox::new(
        Point3::zero(),
        Point3::new(165.0, 165.0, 165.0), 
        Arc::clone(&white)));

    let box1 = Translate::new(
        Arc::new(Rotate::new(box1, Plane::ZX, -15.0)),
        Vec3::new(265.0, 0.0, 295.0));
    let box2 = Translate::new(
        Arc::new(Rotate::new(box2, Plane::ZX, 18.0)),
        Vec3::new(130.0, 0.0, 65.0));

    world.add(Arc::new(box1));
    world.add(Arc::new(box2));

    (camera, world)
}

fn _cornell_smoke() -> (Camera, HittableList) {
    let look_from = Point3::new(278.0, 278.0, -800.0);
    let look_at = Point3::new(278.0, 278.0, 0.0);
    let v_up = Point3::new(0.0, 1.0, 0.0);
    let vfov = 40.0;
    let distance_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(
        look_from,
        look_at,
        v_up,
        vfov,
        ASPECT_RATIO,
        aperture,
        distance_to_focus,
        0.0, 1.0,
    );

    let mut world = HittableList::new();

    let red: Arc<M> = Arc::new(Lambertian::rgb(0.65, 0.05, 0.05));
    let white: Arc<M> = Arc::new(Lambertian::rgb(0.73, 0.73, 0.73));
    let green: Arc<M> = Arc::new(Lambertian::rgb(0.12, 0.45, 0.15));

    let light: Arc<M> = Arc::new(DiffuseLight::rgb(7.0, 7.0, 7.0));

    world.add(Arc::new(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&green))));
    world.add(Arc::new(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, Arc::clone(&red))));
    world.add(Arc::new(AARect::new(Plane::ZX, 113.0, 443.0, 127.0, 432.0, 554.0, Arc::clone(&light))));
    world.add(Arc::new(AARect::new(Plane::ZX, 0.0, 555.0, 0.0, 555.0, 0.0, Arc::clone(&white))));
    world.add(Arc::new(AARect::new(Plane::ZX, 0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&white))));
    world.add(Arc::new(AARect::new(Plane::XY, 0.0, 555.0, 0.0, 555.0, 555.0, Arc::clone(&white))));

    let box1 = Arc::new(AABox::new(
        Point3::zero(),
        Point3::new(165.0, 330.0, 165.0), 
        Arc::clone(&white)));
    let box2 = Arc::new(AABox::new(
        Point3::zero(),
        Point3::new(165.0, 165.0, 165.0), 
        Arc::clone(&white)));

    let box1 = Translate::new(
        Arc::new(Rotate::new(box1, Plane::ZX, -15.0)),
        Vec3::new(265.0, 0.0, 295.0));
    let box2 = Translate::new(
        Arc::new(Rotate::new(box2, Plane::ZX, 18.0)),
        Vec3::new(130.0, 0.0, 65.0));

    let black_texture: Arc<T> = Arc::new(SolidColour::rgb(0.0, 0.0, 0.0));
    let white_texture: Arc<T> = Arc::new(SolidColour::rgb(1.0, 1.0, 1.0));
    world.add(Arc::new(ConstantMedium::new(
        Arc::new(box1),
        black_texture,
        0.01)));
    world.add(Arc::new(ConstantMedium::new(
        Arc::new(box2),
        white_texture,
        0.01)));

    (camera, world)
}
