use crate::{
    interval::Interval, solid_texture::SolidTexture, texture::Texture,
    vector::VecRand,
};

use derive_more::{Add, AddAssign, Deref, Div, Mul, Sum};
use glam::{Vec2, Vec3};

#[derive(Clone, Copy, Debug, Default, Deref, Add, AddAssign, Sum, Mul, Div)]
pub struct Colour(Vec3);

impl From<Vec2> for  Colour {
    fn from(value: Vec2) -> Self {
        Self::new(value.x, 0., value.y)
    }
}

impl Colour {
    #[allow(dead_code)]
    pub const WHITE: Self = Self::new(1., 1., 1.);
    #[allow(dead_code)]
    pub const BLACK: Self = Self::new(0. ,0. ,0.);
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
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

pub fn map_colours(colour: &Colour) -> (u8, u8, u8) {
    let (r, g, b) = (colour.x, colour.y, colour.z);

    let colour_interval = Interval::new(0., 1.0);
    let (rbyte, gbyte, bbyte) = (
        (colour_interval.clamp(r) * 255.) as u8,
        (colour_interval.clamp(g) * 255.) as u8,
        (colour_interval.clamp(b) * 255.) as u8,
    );
    (rbyte, gbyte, bbyte)
}
