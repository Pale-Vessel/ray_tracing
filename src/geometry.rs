use glam::{Mat3, Vec2, Vec3};

use derive_more::with_trait::{
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
#[repr(transparent)]
pub struct Point3(Vec3);

impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vec3) -> Self::Output {
        self + Point3::from(rhs)
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self - Point3::from(rhs)
    }
}

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

impl From<Vec3> for Point3 {
    fn from(value: Vec3) -> Self {
        Self(value)
    }
}

impl Point3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self(Vec3::new(x, y, z))
    }
}

impl NearZero for Vec2 {
    fn near_zero(&self) -> bool {
        self.x.abs() < Self::EPSILON && self.y.abs() < Self::EPSILON
    }
}
use derive_more::Constructor;

#[derive(Copy, Clone, Debug, Constructor)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, time: f32) -> Point3 {
        self.origin + Point3::from(time * self.direction)
    }
}


fn rotation_between(from: Vec3, to: Vec3) -> Mat3 {
    // https://math.stackexchange.com/questions/180418/calculate-rotation-matrix-to-align-vector-a-to-vector-b-in-3d
    let rotation_axis = from.cross(to);
    let rotation_angle = from.dot(to);
    let cross_product_matrix = Mat3::from_cols_array(&[
        0.,
        rotation_axis.z,
        -rotation_axis.y,
        -rotation_axis.z,
        0.,
        rotation_axis.x,
        rotation_axis.y,
        -rotation_axis.x,
        0.,
    ]);
    Mat3::IDENTITY
        + cross_product_matrix
        + cross_product_matrix * cross_product_matrix / (1. + rotation_angle)
}

pub fn make_basis(from: Point3, to: Point3, theta: f32) -> (Vec3, Vec3, Vec3) {
    let basis_frame_x = (from - to).normalize();
    let basis_rotation = rotation_between(Vec3::X, basis_frame_x) * Mat3::from_rotation_x(theta);
    let basis_frame_y = basis_rotation * Vec3::Y;
    let basis_frame_z = basis_rotation * Vec3::Z;
    (basis_frame_x, basis_frame_y, basis_frame_z)
}