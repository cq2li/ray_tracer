pub mod vec3;
use crate::vec3::{ Vec3, Point3, Colour };

pub mod ray;
use crate::ray::{ Ray, ray_colour };

use std::io::{ self, Write };

fn main() -> io::Result<()> {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0_f64, 0_f64, 0_f64);
    let horizontal = Vec3::new(viewport_width, 0_f64, 0_f64);
    let vertical = Vec3::new(0_f64, viewport_height, 0_f64);
    let lower_left_corner = origin - horizontal/2_f64 - vertical/2_f64 - Vec3::new(0_f64, 0_f64, focal_length);
    
    // loading icon 
    let loading_icon = ["-", "\\", "|", "/"];

    let mut out = io::stdout();
    write!(&mut out, "P3\n{image_width} {image_height}\n255\n")?;

    // prints the file from top to bottom
    for j in image_height-1..=0 {
        eprint!("\x1b[2K\r");
        eprint!("{} lines remaining {}", image_height - j, loading_icon[j % loading_icon.len()]);
        // thread::sleep(ten_millis);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_colour = ray_colour(ray);
            
            Vec3::write_colour(&mut out, pixel_colour)?;
        }
    }

    eprint!("\x1b[2K\rDone\n");
    Ok(())
}
