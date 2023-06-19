use crate::constants::inf;
use crate::hit::{HitRecord, Hittable};
use crate::vec3::{Colour, Point3, Vec3};
use rand::Rng;

#[derive(Debug)]
pub struct Ray {
    ori: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            ori: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.ori
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.ori + t * self.dir
    }
}

pub fn ray_colour(ray: Ray, world: &impl Hittable, gen: &mut impl Rng, depth: usize) -> Vec3 {
    if depth == 0 {
        return Colour::new(0.0, 0.0, 0.0)
    }
    let mut rec = HitRecord::default();
    if world.hit(&ray, 0.001, inf, &mut rec) {
        let target = rec.point() + rec.norm() + Vec3::rand_unit_vector(gen);
        return 0.5 * ray_colour(Ray::new(rec.point(), target - rec.point()), world, gen, depth - 1)
    }
    let unit_dir = Vec3::unit_vector(ray.direction());
    // transforms t to between 0 and 1
    let t = 0.5 * (unit_dir.y() + 1.0);
    // blends from white to something light blue
    (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
}

pub fn hits_sphere(sph_center: Vec3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - sph_center; // this is the offset between camera and the hits_sphere
    let a = Vec3::dot(ray.direction(), ray.direction());
    let b = Vec3::dot(ray.direction(), oc);
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / a
    }
}

pub fn sph_ray_colour(r: &Ray) -> Vec3 {
    let unit_dir = Vec3::unit_vector(r.direction());
    let t = hits_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let norm = Vec3::unit_vector(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        0.5 * (Colour::new(norm.x(), norm.y(), norm.z()) + 1.0)
    } else {
        let t = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
}
