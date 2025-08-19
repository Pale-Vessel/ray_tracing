use derive_more::Constructor;

use crate::{colour::Colour, texture::GetTexture};

#[derive(Copy, Clone, Constructor, Debug, Default)]
pub struct SolidTexture {
    colour: Colour,
}

impl GetTexture for SolidTexture {
    fn get_colour(&self, _u: f64, _v: f64) -> Colour {
        self.colour
    }
}
