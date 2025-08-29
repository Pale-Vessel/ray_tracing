use derive_more::Constructor;

use crate::{
    colour::Colour,
    textures::texture::{GetTexture, Texture},
};

#[derive(Copy, Clone, Constructor, Debug, Default)]
pub struct SolidTexture {
    colour: Colour,
}

impl GetTexture for SolidTexture {
    fn get_colour(&self, _u: f32, _v: f32) -> Colour {
        self.colour
    }
}

impl SolidTexture {
    pub fn wrap(self) -> Texture {
        Texture::Solid(self)
    }
}
