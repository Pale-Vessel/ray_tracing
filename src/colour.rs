use crate::{
    interval::Interval,
    textures::{solid_texture::SolidTexture, texture::Texture},
};

use std::ops::MulAssign;

use derive_more::{Add, Deref, Div, Mul, Sum};
use glam::{Vec2, Vec3};

#[derive(Clone, Copy, Debug, Default, Deref, Add, Sum, Mul, Div, PartialEq)]
pub struct Colour(Vec3);

impl From<Vec2> for Colour {
    fn from(value: Vec2) -> Self {
        Self::new(value.x, 0., value.y)
    }
}

impl From<Vec3> for Colour {
    fn from(value: Vec3) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}

impl MulAssign for Colour {
    fn mul_assign(&mut self, rhs: Self) {
        *self = Self(**self * *rhs)
    }
}

impl Colour {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn to_texture(self) -> Texture {
        SolidTexture::new(self).wrap()
    }
}

pub fn map_colours(colour: &Colour) -> (u8, u8, u8) {
    let [r, g, b] = colour.to_array();

    let colour_interval = Interval::new(0., 1.0);
    let (rbyte, gbyte, bbyte) = (
        (colour_interval.clamp(r) * 255.) as u8,
        (colour_interval.clamp(g) * 255.) as u8,
        (colour_interval.clamp(b) * 255.) as u8,
    );
    (rbyte, gbyte, bbyte)
}
