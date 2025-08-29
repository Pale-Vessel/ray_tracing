use crate::{colour::Colour, geometry::vector::Point3};
use derive_more::Constructor;
use glam::Vec3;

#[derive(Copy, Clone, Debug, Constructor)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub collected_light: Colour,
}

impl Ray {
    pub const fn new_white(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            collected_light: Colour::new(1., 1., 1.),
        }
    }

    pub fn at(&self, time: f32) -> Point3 {
        self.origin + Point3::from_vector(time * self.direction)
    }
}
