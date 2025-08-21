use std::ops::Mul;

use crate::{
    interval::Interval, solid_texture::SolidTexture, texture::Texture, vector::VecStuff,
};

use derive_more::{Add, Deref, Div, Mul, Sum};
use glam::DVec3 as Vec3;

#[derive(Clone, Copy, Debug, Default, Deref, Add, Sum, Mul, Div)]
pub struct Colour(Vec3);

impl Colour {
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Self(Vec3::new(r, g, b))
    }

    pub fn new_random() -> Self {
        let random = Vec3::rand_unit_vector();
        Self(random)
    }

    pub fn to_texture(self) -> Texture {
        SolidTexture::new(self).wrap()
    }
}

impl Mul for Colour {
    type Output = Self;

    fn mul(self, rhs: Colour) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

pub fn map_colours(colour: &Colour) -> (u8, u8, u8) {
    let (r, g, b) = (
        linear_to_gamma(colour.x),
        linear_to_gamma(colour.y),
        linear_to_gamma(colour.z),
    );

    let colour_interval = Interval::new(0., 0.9999);
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    let (rbyte, gbyte, bbyte) = (
        (colour_interval.clamp(r) * 255.) as u8,
        (colour_interval.clamp(g) * 255.) as u8,
        (colour_interval.clamp(b) * 255.) as u8,
    );
    (rbyte, gbyte, bbyte)
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.max(0.).sqrt()
}
