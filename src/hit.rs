use crate::vec3::{ Point3, Vec3 };
use crate::ray::Ray;

#[derive(Default, Clone)]
pub struct HitRecord {
    point: Point3,
    // initiall all normas will be outward facing because we took the difference of the point 
    //  from the center
    norm: Vec3,
    // t is the point of ray intersection with sphere
    t: f64,
    // we will always store the normal that is 'against' the ray
    //  as such we'll need to store if the ray is inside/outside the object when it intersects
    //  if true, array hits came from the outside
    front_face: bool, 
}

impl HitRecord {
    pub fn norm(&self) -> Vec3 {
        self.norm
    }
}

impl HitRecord {
    fn set_face_norm(&mut self, ray: &Ray, outward_norm: Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_norm) < 0.0;
        self.norm = if self.front_face {
            outward_norm
        } else {
            -outward_norm
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        false
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {center, radius}
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = Vec3::dot(ray.direction(), ray.direction());
        // b is really half of b, divided everthing by 2 at the start
        let b = Vec3::dot(ray.direction(), oc);
        let c = Vec3::dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();

        // find the nearest root in the range of t_min and max
        let mut root = (-b - sqrtd) / a;
        if root > t_max || root < t_min {
            root = (-b + sqrtd) / a;
            if root > t_max || root < t_min {
                return false
            }
        }
        
        hit_record.t = root;
        hit_record.point = ray.at(root);
        let outward_norm = (hit_record.point - self.center) / self.radius;
        hit_record.set_face_norm(ray, outward_norm);
        true
    }
}

pub struct HittableList<T: Hittable> {
    objects: Vec<T>
}

impl<T: Hittable> HittableList<T> {
    pub fn new() -> Self {
        Self { objects: Vec::<T>::new()}
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
    pub fn add(&mut self, object: T) {
        self.objects.push(object)
    }

}

impl<T: Hittable> Hittable for HittableList<T> {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_record: HitRecord = HitRecord::default();
        let mut hit_any = false;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if obj.hit(ray, t_min, closest_so_far, &mut temp_record) {
                hit_any = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record.clone()
            }
        }
        hit_any
    }
}
