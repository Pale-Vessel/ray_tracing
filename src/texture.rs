use crate::{
    checker_texture::CheckerTexture, colour::Colour,
    gradient_texture::GradientTexture, perlin_texture::PerlinTexture,
    solid_texture::SolidTexture, stripe_texture::StripeTexture,
};

pub trait GetTexture {
    fn get_colour(&self, u: f32, v: f32) -> Colour;
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Texture {
    Solid(SolidTexture),
    Checker(CheckerTexture),
    Perlin(PerlinTexture),
    Stripe(StripeTexture),
    Gradient(GradientTexture),
    UV,
}

impl Default for Texture {
    fn default() -> Self {
        Texture::Solid(SolidTexture::default())
    }
}

impl GetTexture for Texture {
    fn get_colour(&self, u: f32, v: f32) -> Colour {
        match self {
            Texture::Solid(solid_texture) => solid_texture.get_colour(u, v),
            Texture::Checker(checker_texture) => {
                checker_texture.get_colour(u, v)
            }
            Texture::Perlin(perlin_texture) => perlin_texture.get_colour(u, v),
            Texture::Stripe(stripe_texture) => stripe_texture.get_colour(u, v),
            Texture::Gradient(gradient_texture) => {
                gradient_texture.get_colour(u, v)
            }
            Texture::UV => Colour::new(u, v, 0.),
        }
    }
}
