use std::{fmt::Debug, ops::Mul};

use crate::{
    interval::Interval,
    textures::{solid_texture::SolidTexture, texture::Texture},
};

use derive_more::{Add, Div, Mul as MulDerive, MulAssign, Sum};
use glam::{Vec2, Vec3};

#[derive(
    Clone, Copy, Debug, Default, Add, Sum, MulDerive, MulAssign, Div, PartialEq,
)]
#[mul(forward)]
pub struct Colour {
    r: f32,
    g: f32,
    b: f32
}

impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul<Colour> for f32 {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        rhs * self
    }
}

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

impl Colour {
    pub const WHITE: Self = Colour::new(1., 1., 1.);
    pub const BLACK: Self = Colour::new(0., 0., 0.);

    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self {r, g, b}
    }

    pub fn to_texture(self) -> Texture {
        SolidTexture::new(self).wrap()
    }

    pub fn lerp(self, rhs: Self, ratio: f32) -> Self {
        (1. - ratio) * self + ratio * rhs
    }
}

pub fn map_colours(colour: &Colour) -> (u8, u8, u8) {
    let colour_interval = Interval::new(0., 1.0);
    let (rbyte, gbyte, bbyte) = (
        (colour_interval.clamp(colour.r) * 255.) as u8,
        (colour_interval.clamp(colour.g) * 255.) as u8,
        (colour_interval.clamp(colour.b) * 255.) as u8,
    );
    (rbyte, gbyte, bbyte)
}
