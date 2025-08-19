use std::{
    fmt::Display,
    ops::{Add, AddAssign, Index, Mul, Sub, SubAssign},
};

use derive_more::{
    Add, AddAssign, Constructor, Deref, Div, DivAssign, Mul, MulAssign, Neg,
    Sub, SubAssign, Sum,
};
use rand::{Rng, rng};
use rand_distr::StandardNormal;

#[derive(
    Debug,
    Copy,
    Clone,
    Default,
    Constructor,
    Add,
    Sub,
    Mul,
    AddAssign,
    SubAssign,
    MulAssign,
    Div,
    DivAssign,
    Neg,
    Sum,
)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }
}

impl Vec3 {
    const EPSILON: f64 = 1e-8;

    pub const fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub const fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    pub const fn near_zero(&self) -> bool {
        self.x.abs() < Self::EPSILON
            && self.y.abs() < Self::EPSILON
            && self.z.abs() < Self::EPSILON
    }

    pub const fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub const fn cross(&self, other: Self) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn floor(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }

    pub const fn clamp(&self, min: f64, max: f64) -> Self {
        Self::new(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        )
    }

    pub fn reflect(&self, normal: Self) -> Vec3 {
        (*self - 2. * self.dot(normal) * normal).unit()
    }

    pub fn refract(&self, normal: Self, refractive_index: f64) -> Vec3 {
        let cos_theta = -self.dot(normal).min(1.);
        let perpendicular_ray = refractive_index * (*self + cos_theta * normal);
        let parallel_ray =
            (1. - perpendicular_ray.length_squared()).abs().sqrt() * normal;
        (perpendicular_ray - parallel_ray).unit()
    }

    pub fn rand_unit_vector() -> Self {
        let mut rng = rng();
        let (x, y, z) = (
            rng.sample(StandardNormal),
            rng.sample(StandardNormal),
            rng.sample(StandardNormal),
        );
        if x == 0. && y == 0. && z == 0. {
            Vec3::new(1., 0., 0.)
        } else {
            Vec3::new(x, y, z)
        }
    }

    pub fn random_on_unit_disk() -> Self {
        let mut rng = rng();
        let (x, y) = (rng.sample(StandardNormal), rng.sample(StandardNormal));

        Vec3::new(x, y, 0.)
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    Default,
    Deref,
    Add,
    Sub,
    AddAssign,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Neg,
    Sum,
)]
pub struct Point3(Vec3);

impl Add<f64> for Point3 {
    type Output = Point3;

    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl AddAssign<f64> for Point3 {
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs
    }
}

impl Sub<f64> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: f64) -> Self::Output {
        Self::new(self.x - rhs, self.y - rhs, self.z - rhs)
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, rhs: f64) {
        *self = *self - rhs
    }
}

impl Point3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self(Vec3::new(x, y, z))
    }

    pub const fn from_vector(vec: Vec3) -> Self {
        Self(vec)
    }

    pub fn floor(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor(), self.z.floor())
    }
}
