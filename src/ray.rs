use crate::vec3::{ Vec3, Point3, Colour };

#[derive(Debug)]
pub struct Ray {
    ori: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { ori: origin, dir: direction }
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

pub fn ray_colour(r: &Ray) -> Vec3 {
    let unit_dir = Vec3::unit_vector(r.direction());
    let t = 0.5*(unit_dir.y() + 1.0); // transforms t to between 0v and 1v
    (1.0-t)*Colour::new(1.0, 1.0, 1.0) + t*Colour::new(0.5, 0.7, 1.0) // blends from white to
                                                                      // something
}

pub fn hits_sphere(sph_center: Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - sph_center; // this is the offset between camera and the hits_sphere
    let a = Vec3::dot(ray.direction(), ray.direction());
    let b = 2_f64 * Vec3::dot(ray.direction(), oc);
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4_f64 * a * c;
    discriminant > 0_f64
}

pub fn sph_ray_colour(r: &Ray) -> Vec3 {
    if hits_sphere(Point3::new(0_f64, 0_f64, -1_f64), 0.5_f64, &r) {
        Colour::new(0_f64, 1_f64, 0_f64)
    } else {
        let unit_dir = Vec3::unit_vector(r.direction());
        let t = 0.5*(unit_dir.y() + 1.0); // transforms t to between 0v and 1v
        (1.0-t)*Colour::new(1.0, 1.0, 1.0) + t*Colour::new(0.5, 0.7, 1.0)
    }
}
