use crate::vec3::{ Vec3, Point3, Colour };

pub struct Ray {
    ori: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { ori: origin.clone(), dir: direction.clone() }
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

pub fn ray_colour(r: Ray) -> Vec3 {
    let unit_dir = Vec3::unit_vector(r.direction());
    let t = 0.5*(unit_dir.y() + 1.0); // transforms t to between 0v and 1v
    return (1.0-t)*Colour::new(1.0, 1.0, 1.0) + t*Colour::new(0.5, 0.7, 1.0);
}
