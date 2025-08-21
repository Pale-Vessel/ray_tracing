use crate::vector::Point3;
use derive_more::Constructor;
use glam::Vec3;

#[derive(Copy, Clone, Debug, Constructor)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub const fn new_time_zero(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            time: 0.,
        }
    }

    pub fn at(&self, time: f32) -> Point3 {
        self.origin + Point3::from_vector(time * self.direction)
    }
}
