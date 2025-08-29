use crate::{colour::Colour, vector::Point3};
use glam::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f32,
    pub collected_light: Colour,
}

impl Ray {
    pub const fn new_white(origin: Point3, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
            collected_light: Colour::new(1., 1., 1.),
        }
    }

    pub const fn new(
        origin: Point3,
        direction: Vec3,
        time: f32,
        collected_light: Colour,
    ) -> Self {
        Self {
            origin,
            direction,
            time,
            collected_light,
        }
    }

    pub const fn new_time_zero(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            time: 0.,
            collected_light: Colour::new(1., 1., 1.),
        }
    }

    pub fn at(&self, time: f32) -> Point3 {
        self.origin + Point3::from_vector(time * self.direction)
    }
}
