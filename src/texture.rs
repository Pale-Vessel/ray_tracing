use crate::{
    checker_texture::CheckerTexture, colour::Colour,
    perlin_texture::PerlinTexture, solid_texture::SolidTexture,
};

pub trait GetTexture {
    fn get_colour(&self, u: f32, v: f32) -> Colour;
}

#[derive(Debug, Clone)]
pub enum Texture {
    Solid(SolidTexture),
    Checker(CheckerTexture),
    Perlin(PerlinTexture),
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
        }
    }
}
