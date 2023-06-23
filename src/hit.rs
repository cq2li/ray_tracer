use crate::vec3::{ Point3, Vec3, Colour };
use crate::ray::Ray;
use std::sync::Arc;
use rand::Rng;
use rand::distributions::Uniform;
use rand::prelude::Distribution;

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
    pub material: Option<Arc<dyn Material>>,
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            point: Vec3::new_z(),
            norm: Vec3::new_z(),
            t: 0.0,
            front_face: false,
            material: None,
        }
    }

    pub fn norm(&self) -> Vec3 {
        self.norm
    }

    pub fn point(&self) -> Vec3 {
        self.point
    }

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
    fn hit(&self, _ray: &Ray, _t_min: f64, _t_max: f64, _hit_record: &mut HitRecord) -> bool {
        false
    }
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material + Sync + Send>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material + Sync + Send>) -> Self {
        Self {center, radius, material}
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
        hit_record.material = Some(self.material.clone());
        true
    }
}

pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Sync + Send>>
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::<Arc<dyn Hittable + Sync + Send>>::new()}
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
    pub fn add(&mut self, object: Arc<dyn Hittable + Sync + Send>) {
        self.objects.push(object)
    }

}

impl Hittable for HittableList {

    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::default();
        let mut hit_any = false;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if (*obj).hit(ray, t_min, closest_so_far, &mut temp_record) {
                hit_any = true;
                closest_so_far = temp_record.t;
                *hit_record = temp_record.clone()
            }
        }
        hit_any
    }
}

pub trait Material {
    fn scatter(&self, _ray: Ray, _record: &HitRecord, _attenuation: &mut Colour, _scattered: &mut Ray) -> bool {
        false
    }
}

pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self {albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, record: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool {
        let mut scatter_direction = record.norm() + Vec3::rand_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.norm();
        }
        *scattered = Ray::new(record.point(), scatter_direction);
        *attenuation = self.albedo;
        Vec3::dot(scattered.direction(), record.norm()) > 0.0
    }

}

pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Self {
        Self {albedo, fuzz: if fuzz > 1.0 {1.0} else {fuzz}}
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, record: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(ray.direction(), record.norm());
        *scattered = Ray::new(record.point(), reflected + self.fuzz * Vec3::rand_in_unit_sphere());
        *attenuation = self.albedo;
        true
    }

}

pub struct Dielectric {
    ir: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, record: &HitRecord, attenuation: &mut Colour, scattered: &mut Ray) -> bool {
        *attenuation = Colour::new(1.0, 1.0, 1.0);
        let refraction_ratio = if record.front_face { 1.0/self.ir } else { self.ir };
        let unit_direction = Vec3::unit_vector(ray.direction());
        let cos_theta = Vec3::dot(-unit_direction, record.norm()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut rng = rand::thread_rng();
        let direction = if refraction_ratio * sin_theta > 1.0 || Self::reflectance(cos_theta, self.ir) > rng.gen_range(0.0..1.0) {
            // no solution to theta prime, hence no refraction and always reflects
            Vec3::reflect(unit_direction, record.norm())
        } else {
            Vec3::refract(unit_direction, record.norm(), refraction_ratio)
        };
        *scattered = Ray::new(record.point(), direction);
        true
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material.clone())));
    let mut rng = rand::thread_rng();
    let dist_diffuse = Uniform::from(0.0..1.0);
    let dist_metal = Uniform::from(0.5..1.0);
    let dist_fuzz = Uniform::from(0.0..0.5);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = dist_diffuse.sample(&mut rng); // using diffuse dist since its 0 1
            let center = Point3::new(a as f64 + 0.9 * dist_diffuse.sample(&mut rng), 0.2, b as f64 + 0.9 * dist_diffuse.sample(&mut rng));

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material + Sync + Send>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Colour::rand(&dist_diffuse) * Colour::rand(&dist_diffuse);
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Colour::rand(&dist_metal);
                    let fuzz = dist_fuzz.sample(&mut rng);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material.clone())));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1.clone())));

    let material2 = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2.clone())));

    let material3 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3.clone())));

    world
}
