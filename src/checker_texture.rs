use crate::{
    colour::Colour,
    texture::{GetTexture, Texture},
};

#[derive(Clone, Debug)]
pub struct CheckerTexture {
    even_texture: Box<Texture>,
    odd_texture: Box<Texture>,
    pub inv_scale: f64,
}

impl GetTexture for CheckerTexture {
    fn get_colour(&self, u: f64, v: f64) -> Colour {
        let u_pos = (u * self.inv_scale).round() as u16;
        let v_pos = (v * self.inv_scale).round() as u16;

        let sampled_texture = if u_pos.rem_euclid(2) == v_pos.rem_euclid(2) {
            self.even_texture.clone()
        } else {
            self.odd_texture.clone()
        };

        sampled_texture.get_colour(u, v)
    }
}

impl CheckerTexture {
    pub fn new(even_texture: Texture, odd_texture: Texture, size: f64) -> Self {
        Self {
            even_texture: Box::new(even_texture),
            odd_texture: Box::new(odd_texture),
            inv_scale: 1. / size,
        }
    }
}
