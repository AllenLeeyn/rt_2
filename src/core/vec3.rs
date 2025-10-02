use std::fmt::{Display, Formatter, Result};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use rand::Rng;

use crate::sq;

#[derive(Debug, Copy, Clone, Default)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    
    pub const ONE: Vec3 = Self { x: 1.0, y: 1.0, z: 1.0 };
    pub const ZERO: Vec3 = Self { x: 0.0, y: 0.0, z: 0.0 };
    pub const X: Vec3 = Self { x: 1.0, y: 0.0, z: 0.0 };
    pub const Y: Vec3 = Self { x: 0.0, y: 1.0, z: 0.0 };
    pub const Z: Vec3 = Self { x: 0.0, y: 0.0, z: 1.0 };

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
    
    pub fn z(&self) -> f32 {
        self.z
    }

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn splat(val: f32) -> Self {
        Self { x: val, y: val, z: val }
    }

    pub fn length_squared(&self) -> f32 {
        sq(self.x) + sq(self.y) + sq(self.z)
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&self) -> Self {
        match self.length() {
            0.0 => *self,
            len => *self / len,
        }
    }

    pub fn distance(&self, other: Vec3) -> f32 {
        (*self - other).length()
    }

    pub fn cross(&self, v: Vec3) -> Self {
        Vec3::new(
            self.y() * v.z() - self.z() * v.y(),
            self.z() * v.x() - self.x() * v.z(),
            self.x() * v.y() - self.y() * v.x(),
        )
    }
    
    pub fn dot(&self, v: Vec3) -> f32 {
        self.x() * v.x() + self.y() * v.y() + self.z() * v.z()
    }
    
    pub fn random_unit_vector() -> Vec3 {
        let mut rng = rand::rng();
        let a = rng.random_range(0.0..2.0 * std::f32::consts::PI);
        let z = rng.random_range(-1.0..1.0) as f32;
        let r = (1.0 - z * z).sqrt();
        Vec3::new(r * a.cos(), r * a.sin(), z)
    }
    
    pub fn random_in_unit_sphere() -> Vec3 {
        let mut rng = rand::rng();

        loop {
            let x = rng.random_range(-1.0..1.0);
            let y = rng.random_range(-1.0..1.0);
            let z = rng.random_range(-1.0..1.0);

            let p = Vec3::new(x, y, z);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
    
    pub fn reflect(&self, n: Vec3) -> Vec3 {
        self.clone() - 2.0 * self.dot(n) * n
    }
    
    pub fn refract(&self, n: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-*self).dot(n).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;
        r_out_perp + r_out_parallel
    }

    pub fn near_zero(&self) -> bool {
        const S: f32 = 1e-8;
        self.x.abs() < S && self.y.abs() < S && self.z.abs() < S
    }
}

pub type Point3 = Vec3;

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Self::Output {
        Vec3::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        *self = *self + v;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Self::Output {
        Vec3::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, v: Vec3) {
        *self = *self - v;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(self.x * v.x, self.y * v.y, self.z * v.z)
    }

}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, t: f32) {
        *self = *self * t;
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, v: Vec3) -> Self::Output {
        Vec3::new(self * v.x, self * v.y, self * v.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, t: f32) -> Self::Output {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, t: f32) -> Self::Output {
        Vec3::new(self.x / t, self.y / t, self.z / t)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, t: f32) {
        *self = *self / t;
    }
}

use std::ops::{Index, IndexMut};

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        match i {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Vec3 index out of range"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut f32 {
        match i {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Vec3 index out of range"),
        }
    }
}
