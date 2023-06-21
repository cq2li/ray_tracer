use std::io::{ self, Write };
use std::default::Default;
use std::ops::{ Neg, Index, IndexMut, AddAssign, Add, Sub, Mul, MulAssign, SubAssign, Div };
use rand::distributions::{ Distribution, Uniform };

/* Allow for copy because it reduces the number of references in the code
 *  and makes it nicer to write
 */
#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 { e0: f64, e1: f64, e2: f64 }

pub type Point3 = Vec3;
pub type Colour = Vec3;

impl Vec3 {
    // constructors
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e0, e1, e2 }
    }

    // zero-constructors
    pub fn new_z() -> Self {
        Self::default()
    }

    // random generator
    pub fn rand(dist: &Uniform<f64>) -> Self {
        let gen = &mut rand::thread_rng();
        Self {
            e0: dist.sample(gen),
            e1: dist.sample(gen),
            e2: dist.sample(gen),
        }
    }

    // accessors
    pub fn x(&self) -> f64 {
        self.e0
    }

    pub fn y(&self) -> f64 {
        self.e1
    }
    
    pub fn z(&self) -> f64 {
        self.e2
    }
    
    pub fn length_squared(&self) -> f64 {
        self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    
    // writes a vec3 as a row in ppm format
    pub fn write(out: &mut impl Write, vec: &Vec3) -> io::Result<()> {
        write!(out, "{} {} {}", vec.x(), vec.y(), vec.z())
    }

    pub fn dot(v: Vec3, w: Vec3) -> f64 {
        v.e0 * w.e0 + v.e1 * w.e1 + v.e2 * w.e2
    }
    
    pub fn cross(v: Vec3, w: Vec3) -> Vec3 {
        Vec3::new(
            v.e1 * w.e2 - w.e1 * v.e2,
            v.e2 * w.e0 - w.e2 * v.e0,
            v.e0 * w.e1 - w.e0 * v.e1,
        )
    }
    
    pub fn unit_vector(v: Vec3) -> Vec3 {
        let length = v.length();
        v / length
    }
    
    // writes a colour RGB from a vec3
    pub fn write_colour(out: &mut dyn Write, colour: Colour, samples_per_pix: usize) -> io::Result<()> {
        let scale = 1.0 / samples_per_pix as f64;
        for i in 0..3 {
            write!(out, "{}", (255.999 * clamp((colour[i] * scale).sqrt(), 0.0, 0.999)) as u8)?;
            if i < 2 {
                write!(out, " ")?;

            }
        };
        write!(out, "\n")
    }

    // checks if it's in a unit sphere
    pub fn rand_in_unit_sphere() -> Point3 {
        let dist = Uniform::from(-1.0..1.0);
        loop {
            let p = Self::rand(&dist);
            if p.length_squared() < 1.0 { break p }
        }
    }

    pub fn rand_unit_vector() -> Vec3 {
        return Self::unit_vector(Self::rand_in_unit_sphere())
    }
    
    pub fn rand_in_unit_disk() -> Vec3 {
        let dist = Uniform::from(-1.0..1.0);
        let mut rng = rand::thread_rng();
        loop {
            let p = Vec3::new(dist.sample(&mut rng), dist.sample(&mut rng), 0.0);
            if p.length() < 1.0 { break p }
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s &&
        self.y().abs() < s &&
        self.z().abs() < s
        
    }

    pub fn reflect(v: Vec3, norm: Vec3) -> Vec3 {
        v - 2.0 * Self::dot(v, norm) * norm
    }

    pub fn refract(uv: Vec3, n: Vec3, etap_over_eta: f64) -> Vec3 {
        let cos_theta = Self::dot(-uv, n).min(1.0);
        let r_out_perp = etap_over_eta * (uv + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
        r_out_parallel + r_out_perp
    }
}

//*************Implement Vector operations using standard ops***************


// Unary negation
impl Neg for Vec3 {    
    type Output = Self;
    fn neg(self) -> Self {
        Self { e0: -self.e0, e1: self.e1.neg(), e2: self.e2.neg() }
    }
}


// Indexed element extration
impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.e0,
            1 => &self.e1,
            2 => &self.e2,
            _ => panic!("Invalid Index Into Vec3"),
        }
    }
}


// Index element extration mutable ref
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f64 {
        match i {
            0 => &mut self.e0,
            1 => &mut self.e1,
            2 => &mut self.e2,
            _ => panic!("Invalid IndexMut Into Vec3"),
        }
    }
}


// Vec3 - Vec3
impl Sub<Self> for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            e0: self.e0 - other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
        }
    }
}


// Vec3 - float
impl Sub<f64> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: f64) -> Vec3 {
        Vec3 {
            e0: self.e0 - other,
            e1: self.e1 - other,
            e2: self.e2 - other,
        }
    }
}


// Vec3 -= Vec3
impl SubAssign<Self> for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            e0: self.e0 - rhs.e0,
            e1: self.e1 - rhs.e1,
            e2: self.e2 - rhs.e2,
        };
    }
}


// Vec3 -= float
impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        *self = Self {
            e0: self.e0 - rhs,
            e1: self.e1 - rhs,
            e2: self.e2 - rhs,
        };
    }
}


// Vec3 + Vec3
impl Add<Self> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            e0: self.e0 + other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
        }
    }
}


// Vec3 + float
impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, other: f64) -> Self::Output {
        Self::Output {
            e0: self.e0 + other,
            e1: self.e1 + other,
            e2: self.e2 + other,
        }
    }
}


// float + Vec3
impl Add<Vec3> for f64 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(
            self + other.e0,
            self + other.e1,
            self + other.e2
            )
    }
}


// Vec3 += Vec3
impl AddAssign<Self> for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            e0: self.e0 + rhs.e0,
            e1: self.e1 + rhs.e1,
            e2: self.e2 + rhs.e2,
        };
    }
}


// Vec3 += float
impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        *self = Self {
            e0: self.e0 + rhs,
            e1: self.e1 + rhs,
            e2: self.e2 + rhs,
        };
    }
}


// Vec3 * Vec3 Element wise
//  dot is defined as a method, not a trait
impl Mul<Self> for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            e0: self.e0 * other.e0,
            e1: self.e1 * other.e1,
            e2: self.e2 * other.e2,
        }
    }
}


// Vec3 * float
impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, other: f64) -> Self {
        Self {
            e0: self.e0 * other,
            e1: self.e1 * other,
            e2: self.e2 * other,
        }
    }
}


// float * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self * other.e0,
            self * other.e1,
            self * other.e2
            )
    }
}


// Vec3 *= Vec3
impl MulAssign<Self> for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self {
            e0: self.e0 * rhs.e0,
            e1: self.e1 * rhs.e1,
            e2: self.e2 * rhs.e2,
        };
    }
}


// Vec3 *= float
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            e0: self.e0 * rhs,
            e1: self.e1 * rhs,
            e2: self.e2 * rhs,
        };
    }
}


// Vec3 / float
impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Self::Output {
        Self::Output {
            e0: self.e0 * 1_f64/other,
            e1: self.e1 * 1_f64/other,
            e2: self.e2 * 1_f64/other,
        }
    }
}


// float / Vec3
impl Div<Vec3> for f64 {
    type Output = Vec3;
    fn div(self, other: Vec3) -> Self::Output {
        Vec3::new(
            self * other.e0.recip(),
            self * other.e1.recip(),
            self * other.e2.recip(),
        )
    }
}


fn clamp(x: f64, lo: f64, hi: f64) -> f64 {
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

// testing
#[cfg(test)]
mod second_test {
    use super::Vec3;
    #[test]
    fn basics() {
        // constructor
        let mut v1 = Vec3::new(1f64,2f64,3f64);
        let v2 = Vec3::new(5f64,4f64,3f64);
        // x, y, z
        assert_eq!(v1.x(), 1f64);
        assert_eq!(v1.y(), 2f64);
        assert_eq!(v1.z(), 3f64);
        // negate
        v1 = -v1;
        assert_eq!(v1.x(), -1f64);
        assert_eq!(v1.y(), -2f64);
        assert_eq!(v1.z(), -3f64);
        // AddAssign
        v1 += v2;
        assert_eq!(v1.x(), 4f64);
        // index
        assert_eq!(v1[2], 0f64);
        // index mut
        let e1 = &mut v1[1];
        *e1 = 16f64;
        assert_eq!(v1.y(), 16f64);
        // length
        let mut v3 = Vec3::new(3f64,3f64,3f64);
        assert_eq!(v3.length(), 27f64.sqrt());
        v3 *= 3f64;
        assert_eq!(v3.x(), 9f64);
        let n1 = 2.5f64;
        // mult f64 * vec3
        let v4 = n1 * v3;
        assert_eq!(v4.z(), 22.5f64);

        let v5 = v2 + v3;
        assert_eq!(v5.x(), 14f64);
        
    }
}

