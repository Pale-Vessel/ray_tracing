use crate::{
    colour::Colour,
    textures::texture::{GetTexture, Texture},
};

#[derive(Clone, Debug)]
enum Direction {
    U,
    V,
    UV,
}

#[derive(Clone, Debug)]
pub struct GradientTexture {
    bottom_texture: Box<Texture>,
    top_texture: Box<Texture>,
    direction: Direction,
}

impl GetTexture for GradientTexture {
    fn get_colour(&self, u: f32, v: f32) -> Colour {
        let ratio = match self.direction {
            Direction::U => u,
            Direction::V => v,
            Direction::UV => (u + v) / 2.,
        };
        let bottom_colour = self.bottom_texture.get_colour(u, v);
        let top_colour = self.top_texture.get_colour(u, v);
        bottom_colour * ratio + top_colour * (1. - ratio)
    }
}

#[allow(dead_code)]
impl GradientTexture {
    pub fn new_with_dir_name(
        bottom_texture: Texture,
        top_texture: Texture,
        direction: &str,
    ) -> Option<Self> {
        let direction = match direction {
            "u" => Direction::U,
            "v" => Direction::V,
            "uv" => Direction::UV,
            _ => return None,
        };
        Some(Self {
            bottom_texture: Box::new(bottom_texture),
            top_texture: Box::new(top_texture),
            direction,
        })
    }

    pub fn new_u(bottom_texture: Texture, top_texture: Texture) -> Self {
        Self {
            bottom_texture: Box::new(bottom_texture),
            top_texture: Box::new(top_texture),
            direction: Direction::U,
        }
    }

    pub fn new_v(bottom_texture: Texture, top_texture: Texture) -> Self {
        Self {
            bottom_texture: Box::new(bottom_texture),
            top_texture: Box::new(top_texture),
            direction: Direction::V,
        }
    }

    pub fn new_uv(bottom_texture: Texture, top_texture: Texture) -> Self {
        Self {
            bottom_texture: Box::new(bottom_texture),
            top_texture: Box::new(top_texture),
            direction: Direction::UV,
        }
    }

    pub fn wrap(self) -> Texture {
        Texture::Gradient(self)
    }
}
