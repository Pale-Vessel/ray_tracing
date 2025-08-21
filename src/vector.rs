use std::ops::{Add, AddAssign};

use glam::DVec3 as Vec3;

use derive_more::{
    Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign,
    Sum,
};
use rand::{Rng, rng};
use rand_distr::StandardNormal;

pub trait VecStuff {
    const EPSILON: f64 = 1e-8;
    fn near_zero(&self) -> bool;
    fn rand_unit_vector() -> Self;
    fn random_on_unit_disk() -> Self;
}

impl VecStuff for Vec3 {
    fn near_zero(&self) -> bool {
        self.x.abs() < Self::EPSILON
            && self.y.abs() < Self::EPSILON
            && self.z.abs() < Self::EPSILON
    }

    fn rand_unit_vector() -> Self {
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

impl Add<f64> for Point3 {
    type Output = Point3;

    fn add(self, rhs: f64) -> Self::Output {
        Self::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl AddAssign<f64> for Point3 {
    fn add_assign(&mut self, rhs: f64) {
        *self = *self + rhs;
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
