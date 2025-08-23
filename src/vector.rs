use std::ops::{Add, AddAssign};

use glam::{Vec2, Vec3};

use derive_more::{
    Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
    Sum,
};
use rand::{Rng, rng};
use rand_distr::StandardNormal;

pub trait NearZero {
    const EPSILON: f32 = 1e-8;
    fn near_zero(&self) -> bool;
}
pub trait VecRand {
    fn rand_unit_vector() -> Self;
    fn random_on_unit_disk() -> Self;
}

impl NearZero for Vec3 {
    fn near_zero(&self) -> bool {
        self.x.abs() < Self::EPSILON
            && self.y.abs() < Self::EPSILON
            && self.z.abs() < Self::EPSILON
    }
}

impl VecRand for Vec3 {
    fn rand_unit_vector() -> Self {
        let mut rng = rng();
        let (x, y, z) = (
            rng.sample(StandardNormal),
            rng.sample(StandardNormal),
            rng.sample(StandardNormal),
        );
        Vec3::new(x, y, z)
    }

    fn random_on_unit_disk() -> Self {
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

impl Add<f32> for Point3 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl AddAssign<f32> for Point3 {
    fn add_assign(&mut self, rhs: f32) {
        *self = *self + rhs;
    }
}

impl Point3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }

    pub const fn from_vector(vec: Vec3) -> Self {
        Self(vec)
    }
}

impl NearZero for Vec2 {
    fn near_zero(&self) -> bool {
        self.x < Self::EPSILON && self.y < Self::EPSILON
    }
}
