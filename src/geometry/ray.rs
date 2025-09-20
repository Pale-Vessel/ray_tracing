use crate::geometry::vector::Point3;
use derive_more::Constructor;
use glam::Vec3;

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
