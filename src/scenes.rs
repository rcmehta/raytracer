use std::sync::Arc;

use crate::{
    aarect::*, bvh::*, camera::*, hittable::*, material::*, medium::*, moving_sphere::*, sphere::*,
    texture::*, transform::*, vec3::*,
};

// Image Constants
pub const ASPECT_RATIO: F = 1.0;
pub const IMAGE_HEIGHT: u32 = 800;
pub const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as F * ASPECT_RATIO) as u32;
pub const SAMPLES_PER_PIXEL: u32 = 1000;
pub const MAX_DEPTH: u32 = 50;

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
        0.0,
        1.0,
    );

    camera
}

pub fn _random_scene() -> (Camera, HittableList) {
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
                    world.add(Arc::new(MovingSphere::new(
                        centre,
                        centre1,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    )));
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

pub fn _two_spheres() -> (Camera, HittableList) {
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

pub fn _two_perlin_spheres() -> (Camera, HittableList) {
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

pub fn _earth() -> (Camera, HittableList) {
    let mut world = HittableList::new();

    let earth = Arc::new(Image::new("textures/earthmap.jpg"));
    let material: Arc<M> = Arc::new(Lambertian::new(earth));

    world.add(Arc::new(Sphere::new(Point3::zero(), 2.0, material)));

    (default_camera(), world)
}

pub fn _simple_light() -> (Camera, HittableList) {
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
        0.0,
        1.0,
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

    world.add(Arc::new(AARect::new(
        Plane::XY,
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        diff_light,
    )));

    (camera, world)
}

pub fn _cornell_box() -> (Camera, HittableList) {
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
        0.0,
        1.0,
    );

    let mut world = HittableList::new();

    let red: Arc<M> = Arc::new(Lambertian::rgb(0.65, 0.05, 0.05));
    let white: Arc<M> = Arc::new(Lambertian::rgb(0.73, 0.73, 0.73));
    let green: Arc<M> = Arc::new(Lambertian::rgb(0.12, 0.45, 0.15));

    let light: Arc<M> = Arc::new(DiffuseLight::rgb(15.0, 15.0, 15.0));

    world.add(Arc::new(AARect::new(
        Plane::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&green),
    )));
    world.add(Arc::new(AARect::new(
        Plane::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&red),
    )));
    world.add(Arc::new(AARect::new(
        Plane::ZX,
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        Arc::clone(&light),
    )));
    world.add(Arc::new(AARect::new(
        Plane::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white),
    )));
    world.add(Arc::new(AARect::new(
        Plane::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));
    world.add(Arc::new(AARect::new(
        Plane::XY,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));

    let box1 = Arc::new(AABox::new(
        Point3::zero(),
        Point3::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    ));
    let box2 = Arc::new(AABox::new(
        Point3::zero(),
        Point3::new(165.0, 165.0, 165.0),
        Arc::clone(&white),
    ));

    let box1 = Translate::new(
        Arc::new(Rotate::new(box1, Plane::ZX, -15.0)),
        Vec3::new(265.0, 0.0, 295.0),
    );
    let box2 = Translate::new(
        Arc::new(Rotate::new(box2, Plane::ZX, 18.0)),
        Vec3::new(130.0, 0.0, 65.0),
    );

    world.add(Arc::new(box1));
    world.add(Arc::new(box2));

    (camera, world)
}

pub fn _cornell_smoke() -> (Camera, HittableList) {
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
        0.0,
        1.0,
    );

    let mut world = HittableList::new();

    let red: Arc<M> = Arc::new(Lambertian::rgb(0.65, 0.05, 0.05));
    let white: Arc<M> = Arc::new(Lambertian::rgb(0.73, 0.73, 0.73));
    let green: Arc<M> = Arc::new(Lambertian::rgb(0.12, 0.45, 0.15));

    let light: Arc<M> = Arc::new(DiffuseLight::rgb(7.0, 7.0, 7.0));

    world.add(Arc::new(AARect::new(
        Plane::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&green),
    )));
    world.add(Arc::new(AARect::new(
        Plane::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&red),
    )));
    world.add(Arc::new(AARect::new(
        Plane::ZX,
        113.0,
        443.0,
        127.0,
        432.0,
        554.0,
        Arc::clone(&light),
    )));
    world.add(Arc::new(AARect::new(
        Plane::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Arc::clone(&white),
    )));
    world.add(Arc::new(AARect::new(
        Plane::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));
    world.add(Arc::new(AARect::new(
        Plane::XY,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Arc::clone(&white),
    )));

    let box1 = Arc::new(AABox::new(
        Point3::zero(),
        Point3::new(165.0, 330.0, 165.0),
        Arc::clone(&white),
    ));
    let box2 = Arc::new(AABox::new(
        Point3::zero(),
        Point3::new(165.0, 165.0, 165.0),
        Arc::clone(&white),
    ));

    let box1 = Translate::new(
        Arc::new(Rotate::new(box1, Plane::ZX, -15.0)),
        Vec3::new(265.0, 0.0, 295.0),
    );
    let box2 = Translate::new(
        Arc::new(Rotate::new(box2, Plane::ZX, 18.0)),
        Vec3::new(130.0, 0.0, 65.0),
    );

    let black_texture: Arc<T> = Arc::new(SolidColour::rgb(0.0, 0.0, 0.0));
    let white_texture: Arc<T> = Arc::new(SolidColour::rgb(1.0, 1.0, 1.0));
    world.add(Arc::new(ConstantMedium::new(
        Arc::new(box1),
        black_texture,
        0.01,
    )));
    world.add(Arc::new(ConstantMedium::new(
        Arc::new(box2),
        white_texture,
        0.01,
    )));

    (camera, world)
}

pub fn _final_scene() -> (Camera, HittableList) {
    let look_from = Point3::new(478.0, 278.0, -600.0);
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
        0.0,
        1.0,
    );

    let mut world = HittableList::new();

    let ground: Arc<M> = Arc::new(Lambertian::rgb(0.48, 0.83, 0.53));

    const BOXES_PER_SIDE: usize = 20;

    let mut ground_boxes = HittableList::new();

    for i in 0..BOXES_PER_SIDE {
        for k in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as F * w;
            let y0 = 0.0;
            let z0 = -1000.0 + k as F * w;
            let x1 = x0 + w;
            let y1 = random_range(1.0, 101.0);
            let z1 = z0 + w;

            ground_boxes.add(Arc::new(AABox::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                Arc::clone(&ground),
            )));
        }
    }

    world.add(Arc::new(BVH::new(
        &ground_boxes,
        0,
        BOXES_PER_SIDE * BOXES_PER_SIDE,
        0.0,
        1.0,
    )));

    let light: Arc<M> = Arc::new(DiffuseLight::rgb(7.0, 7.0, 7.0));
    world.add(Arc::new(AARect::new(
        Plane::ZX,
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        Arc::clone(&light),
    )));

    let centre1 = Point3::new(400.0, 400.0, 400.0);
    let centre2 = centre1 + Point3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::rgb(0.7, 0.3, 0.1));
    world.add(Arc::new(MovingSphere::new(
        centre1,
        centre2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Colour::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary: Arc<H> = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::clone(&boundary));
    world.add(Arc::new(ConstantMedium::new(
        Arc::clone(&boundary),
        Arc::new(SolidColour::rgb(0.2, 0.4, 0.9)),
        0.2,
    )));
    let boundary = Arc::new(Sphere::new(
        Point3::zero(),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new(
        boundary,
        Arc::new(SolidColour::rgb(1.0, 1.0, 1.0)),
        0.0001,
    )));

    let earth_material = Arc::new(Lambertian::new(Arc::new(Image::new(
        "textures/earthmap.jpg",
    ))));
    world.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    )));

    let perlin_material = Arc::new(Lambertian::new(Arc::new(Noise::new(0.1))));
    world.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        perlin_material,
    )));

    let mut boxes = HittableList::new();
    let white: Arc<M> = Arc::new(Lambertian::rgb(0.73, 0.73, 0.73));

    const N: usize = 1000;

    for _ in 0..N {
        boxes.add(Arc::new(Sphere::new(
            Point3::random_vector(0.0, 165.0),
            10.0,
            Arc::clone(&white),
        )));
    }

    let boxes = Arc::new(BVH::new(&boxes, 0, N, 0.0, 1.0));

    world.add(Arc::new(Translate::new(
        Arc::new(Rotate::new(boxes, Plane::ZX, -15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    (camera, world)
}
