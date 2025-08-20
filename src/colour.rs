use image::{Pixel, RgbImage};

use crate::{
    interval::Interval, solid_texture::SolidTexture, texture::Texture,
    vector::Vec3,
};

pub type Colour = Vec3;

impl Colour {
    pub fn to_texture(self) -> Texture {
        Texture::Solid(SolidTexture::new(self))
    }
}

pub fn write_colour(i: u32, j: u32, buffer: &mut RgbImage, colour: &Colour) {
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
    buffer.put_pixel(i, j, *Pixel::from_slice(&[rbyte, gbyte, bbyte]));
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.max(0.).sqrt()
}
