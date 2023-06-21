use crate::vec3::{ Point3, Vec3 };
use crate::ray::Ray;
use crate::constants::DegToRad;

pub struct Camera {
    pub aspect_ratio: f64,
    pub viewport_height: f64,
    pub viewport_width: f64,
    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3, // horizontal unit vector in lens plane 
    pub v: Vec3, // vertial unit vector in lens plane
    pub w: Vec3, // unit vector from lookat to lookfrom
    pub aperture: f64,
    pub lens_radius: f64,
    pub lower_left_corner: Point3,
}

impl Camera {
    pub fn new(
        lookfrom: Point3, lookat: Point3, vup: Vec3, vfov: f64, aspect_ratio: f64,
        aperture: f64, focus_dist: f64) -> Self {
        let theta = vfov.deg_to_rad();
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(vup, w));
        let v = Vec3::cross(w, u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist*w;

        let lens_radius = aperture/2.0;

        Self {
            aspect_ratio,
            viewport_height,
            viewport_width,
            origin,
            horizontal,
            vertical,
            u,
            v,
            w,
            aperture,
            lens_radius,
            lower_left_corner,
        }
    }
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::rand_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
            Ray::new(
                self.origin + offset, 
                self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset)
        }
}
