use derive_more::Constructor;

use crate::{
    colour::Colour,
    geometry::vector::NearZero,
    textures::texture::{GetTexture, Texture},
};

use glam::{Vec2, Vec2Swizzles, Vec3, Vec3Swizzles};

#[derive(Clone, Debug, Constructor)]
pub struct PerlinTexture {
    scale: f32,
    colour: Colour,
}

impl GetTexture for PerlinTexture {
    fn get_colour(&self, u: f32, v: f32) -> Colour {
        let point = Vec2::new(u, v);
        let origin_corner = Vec2::new(
            self.floor_to_scale(point.x),
            self.floor_to_scale(point.y),
        );

        let mut corner_values = Vec::new();
        for x in 0..=1 {
            for y in 0..=1 {
                let corner_point = Vec2::new(
                    origin_corner.x + (x as f32) * self.scale,
                    origin_corner.y + (y as f32) * self.scale,
                );
                let grid_vector = Self::hash_point_to_vec(corner_point);
                let offset_vector = point - corner_point;
                let noise_value = grid_vector.dot(offset_vector).clamp(-1., 1.);

                // Normalise from [-1, 1] to [0, 1]
                corner_values.push((noise_value / 2.) + 0.5);
            }
        }

        let position_in_cell = (point - origin_corner) / self.scale;

        let x_position = position_in_cell.x;
        let y_position = position_in_cell.y;

        let interpolated_y = [
            Self::smoothstep(corner_values[0], corner_values[1], y_position),
            Self::smoothstep(corner_values[2], corner_values[3], y_position),
        ];

        let value =
            Self::smoothstep(interpolated_y[0], interpolated_y[1], x_position);
        self.colour * value
    }
}

impl PerlinTexture {
    fn hash_point_to_vec(point: Vec2) -> Vec2 {
        let point = if point.near_zero() {
            Vec2::new(47., 103.)
        } else {
            point
        };
        // https://www.shadertoy.com/view/4djSRW#
        let mut point =
            (point.xyx() * Vec3::new(0.1031, 0.1030, 0.0973)).fract();
        point += point.dot(point.yzx() + 33.33);
        ((point.xx() + point.yz()) * point.zy()).fract()
    }

    fn floor_to_scale(&self, val: f32) -> f32 {
        (val / self.scale).floor() * self.scale
    }

    fn smoothstep(a: f32, b: f32, t: f32) -> f32 {
        let faded = t.powi(3) * (t * (t * 6. - 15.) + 10.);
        Self::lerp(a, b, faded)
    }

    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a * (1. - t) + b * t
    }

    pub fn wrap(self) -> Texture {
        Texture::Perlin(self)
    }
}
