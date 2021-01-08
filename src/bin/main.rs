extern crate image;
extern crate rand;
extern crate rayon;

use image::io::Reader as ImageReader;
use rayon::prelude::*;

use std::{error::Error, fs, io::Write};

use raytracer::{ray::*, scenes::*, vec3::*};

fn main() -> Result<(), Box<dyn Error>> {
    // Camera, World
    let (camera, world) = _final_scene();

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
