use crate::{colour::Colour, geometry::vector::Point3};
use glam::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub collected_light: Option<Colour>,
}

impl Ray {
    pub const fn new_with_colour(
        origin: Point3,
        direction: Vec3,
        collected_light: Option<Colour>,
    ) -> Ray {
        Ray {
            origin,
            direction,
            collected_light,
        }
    }

    pub const fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            collected_light: None,
        }
    }

    pub fn at(&self, time: f32) -> Point3 {
        self.origin + Point3::from_vector(time * self.direction)
    }
}
