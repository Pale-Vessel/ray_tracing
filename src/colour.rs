use std::{fmt::Debug, ops::Mul};

use crate::{
    interval::Interval,
    textures::{solid_texture::SolidTexture, texture::Texture},
};

use derive_more::{Add, Div, Mul as MulDerive, MulAssign, Sum};

#[derive(
    Clone, Copy, Debug, Default, Add, Sum, MulDerive, MulAssign, Div, PartialEq,
)]
#[mul(forward)]
pub struct Colour(f32, f32, f32);

impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Colour> for f32 {
    type Output = Colour;

    fn mul(self, rhs: Colour) -> Self::Output {
        rhs * self
    }
}

impl Colour {
    pub const WHITE: Self = Colour::new(1., 1., 1.);
    pub const BLACK: Self = Colour::new(0., 0., 0.);

    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self(r, g, b)
    }

    pub fn to_texture(self) -> Texture {
        SolidTexture::new(self).wrap()
    }

    pub fn lerp(self, rhs: Self, ratio: f32) -> Self {
        (1. - ratio) * self + ratio * rhs
    }
}

pub fn map_colours(colour: &Colour) -> (u8, u8, u8) {
    let (r, g, b) = (colour.0, colour.1, colour.2);
    let colour_interval = Interval::new(0., 1.0);
    let (rbyte, gbyte, bbyte) = (
        (colour_interval.clamp(r) * 255.) as u8,
        (colour_interval.clamp(g) * 255.) as u8,
        (colour_interval.clamp(b) * 255.) as u8,
    );
    (rbyte, gbyte, bbyte)
}
