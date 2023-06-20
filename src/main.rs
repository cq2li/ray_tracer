pub mod vec3;
use crate::vec3::{Point3, Vec3, Colour };
pub mod ray;
use crate::ray::{ray_colour};
pub mod hit;
use crate::hit::{HittableList, Sphere, Lambertian, Metal};
pub mod camera;
pub mod constants;
use crate::camera::Camera;

use rand::{distributions::Uniform, Rng};
use std::io::{self, Write};
use std::{ time, rc::Rc };

fn main() -> io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 1200;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let samples_per_pix: usize = 100;
    let max_depth: usize = 50;

    // Camera
    // aspect_ratio, viewport height, and focal length
    let cam = Camera::new(aspect_ratio, 2.0, 1.0);

    // World
    let material_ground = Rc::new(Lambertian::new(Colour::new(0.95, 0.8, 0.1)));
    let material_center = Rc::new(Lambertian::new(Colour::new(1.0, 0.25, 0.1)));
    let material_left   = Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 0.0));
    let material_right  = Rc::new(Metal::new(Colour::new(0.3, 0.8, 1.0), 0.6));

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())));
    world.add(Box::new(Sphere::new(Point3::new(1.0, -0.0, -0.8), 0.5, material_right.clone())));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.3), 0.5, material_center.clone())));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground.clone())));

    // Antialias sampling
    let unif = Uniform::from(0.0..1.0);
    let rng = &mut rand::thread_rng();

    // Output
    let mut out = io::stdout();
    write!(&mut out, "P3\n{image_width} {image_height}\n255\n")?;

    // prints the file from top to bottom
    let mut progress: usize = 0;
    let start = time::Instant::now();
    eprint!("\x1b[2J");
    eprint!("\rTracing: {}% Completed", progress);
    for j in (0..image_height).rev() {
        if (100 - 100 * j / (image_height - 1) as usize) > progress {
            progress = 100 - 100 * j / (image_height - 1) as usize;
            eprint!("\rTracing: {}% Completed", progress);
        }

        for i in 0..image_width {
            let samples_u: Vec<f64> = rng
                .sample_iter(&unif)
                .take(samples_per_pix)
                .collect();
            let samples_v: Vec<f64> = rng
                .sample_iter(&unif)
                .take(samples_per_pix)
                .collect();
            
            let mut pixel_colour = Colour::new_z();
            for s in 0..samples_per_pix {
                let u_jitter = samples_u[s];
                let v_jitter = samples_v[s];
                // let u_jitter = 0.0;
                // let v_jitter = 0.0;
                let u = (u_jitter + i as f64) / (image_width - 1) as f64;
                let v = (v_jitter + j as f64) / (image_height - 1) as f64;
                let ray = cam.get_ray(u, v);
                pixel_colour += ray_colour(ray, &world, max_depth);
            }


            Vec3::write_colour(&mut out, pixel_colour, samples_per_pix)?;
        }
    }

    eprint!("\x1b[2K\rDone in {:#?}\n", start.elapsed());
    Ok(())
}
