pub mod vec3;
use crate::vec3::{Colour, Point3, Vec3};
pub mod ray;
use crate::ray::ray_colour;
pub mod hit;
use crate::hit::random_scene;
pub mod camera;
pub mod constants;
use crate::camera::Camera;

use indicatif::{style::ProgressStyle, ParallelProgressIterator};
use rand::distributions::{Distribution, Uniform};
use rayon::prelude::*;
use std::io::{self, Write};
use std::time;

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width: usize = 1200;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pix: usize = 200;
    let max_depth: usize = 50;

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    // camera vertical orientation not necessarily aligned with
    //  the actual lens plan, points straight up
    let vup = Vec3::new(0.0, 1.0, 0.0); 
    // in degrees vertical field of view
    let vfov = 20.0; 
    let focus_dist = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        focus_dist,
    );

    // World
    let world = random_scene();

    // Output
    let mut out = io::stdout();
    write!(&mut out, "P3\n{image_width} {image_height}\n255\n")?;

    // setup
    let start = time::Instant::now();
    let render = (0..(image_height * image_width))
        .rev()
        .collect::<Vec<usize>>();
    let mut result = Vec::<Box<str>>::with_capacity(image_height * image_width);
    let unif = Uniform::from(0.0..1.0);
    let pb = ProgressStyle::with_template(
        "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos:>7}/{len:7}rays traced",
    )
    .unwrap()
    .progress_chars("#>-");

    // trace the rays
    render
        .par_iter()
        .progress_with_style(pb)
        .map_init(
            || rand::thread_rng(),
            |mut rng, s| {
                // height rendering from top to bottom + -> -
                let j = s / image_width; 
                // going from left to right - -> +
                let i = image_width - s % image_width - 1; 

                let samples_u: Vec<f64> =
                    unif.sample_iter(&mut rng).take(samples_per_pix).collect();
                let samples_v: Vec<f64> =
                    unif.sample_iter(&mut rng).take(samples_per_pix).collect();

                let mut pixel_colour = Colour::new_z();
                for s in 0..samples_per_pix {
                    // anti alias sampling
                    let u_jitter = samples_u[s];
                    let v_jitter = samples_v[s];

                    //offset origin by jitter
                    let u = (u_jitter + i as f64) / (image_width - 1) as f64;
                    let v = (v_jitter + j as f64) / (image_height - 1) as f64;
                    let ray = cam.get_ray(u, v);
                    pixel_colour += ray_colour(ray, &world, max_depth);
                }
                Vec3::colour_to_str(pixel_colour, samples_per_pix)
            },
        )
        .collect_into_vec(&mut result);

    // prints the file from top to bottom
    result.iter().for_each(|box_str| {
        write!(out, "{}", *box_str).unwrap();
        write!(out, "\n").unwrap();
    });

    eprint!("\x1b[2K\rDone in {:#?}\n", start.elapsed());
    Ok(())
}
