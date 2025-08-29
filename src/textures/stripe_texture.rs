use crate::{
    colour::Colour,
    textures::texture::{GetTexture, Texture},
};

#[derive(Clone, Debug)]
enum Direction {
    U,
    V,
}

#[derive(Clone, Debug)]
pub struct StripeTexture {
    even_texture: Box<Texture>,
    odd_texture: Box<Texture>,
    inv_scale: f32,
    direction: Direction,
}

impl GetTexture for StripeTexture {
    fn get_colour(&self, u: f32, v: f32) -> Colour {
        let u_pos = (u * self.inv_scale).round() as u16;
        let v_pos = (v * self.inv_scale).round() as u16;
        let to_compare = match self.direction {
            Direction::U => u_pos,
            Direction::V => v_pos,
        };

        let sampled_texture = if to_compare.rem_euclid(2) == 0 {
            self.even_texture.clone()
        } else {
            self.odd_texture.clone()
        };

        sampled_texture.get_colour(u, v)
    }
}

#[allow(dead_code)]
impl StripeTexture {
    pub fn new_u(
        even_texture: Texture,
        odd_texture: Texture,
        size: f32,
    ) -> Self {
        Self {
            even_texture: Box::new(even_texture),
            odd_texture: Box::new(odd_texture),
            inv_scale: 1. / size,
            direction: Direction::U,
        }
    }

    pub fn new_v(
        even_texture: Texture,
        odd_texture: Texture,
        size: f32,
    ) -> Self {
        Self {
            even_texture: Box::new(even_texture),
            odd_texture: Box::new(odd_texture),
            inv_scale: 1. / size,
            direction: Direction::V,
        }
    }

    pub fn wrap(self) -> Texture {
        Texture::Stripe(self)
    }
}
