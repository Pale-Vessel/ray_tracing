use std::fmt::Write;

use crate::{
    interval::Interval,
    texture::{SolidTexture, Texture},
    vector::Vec3,
};

pub type Colour = Vec3;

impl Colour {
    pub fn to_texture(self) -> Texture {
        Texture::Solid(SolidTexture::new(self))
    }
}

pub fn write_colour(buffer: &mut String, colour: &Colour) {
    let (r, g, b) = (
        linear_to_gamma(colour.x),
        linear_to_gamma(colour.y),
        linear_to_gamma(colour.z),
    );

    let colour_interval = Interval::new(0., 0.9999);
    let (rbyte, gbyte, bbyte) = (
        (colour_interval.clamp(r) * 255.) as u8,
        (colour_interval.clamp(g) * 255.) as u8,
        (colour_interval.clamp(b) * 255.) as u8,
    );
    _ = buffer.write_str(&format!("{rbyte} {gbyte} {bbyte}\n"));
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    linear_component.max(0.).sqrt()
}
