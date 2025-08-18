use derive_more::Constructor;

use crate::{
    colour::Colour,
    vector::{Point3, Vec3},
};

pub trait GetTexture {
    fn get_colour(&self, u: f64, v: f64, point: Point3) -> Colour;
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
    fn get_colour(&self, u: f64, v: f64, point: Point3) -> Colour {
        match self {
            Texture::Solid(solid_texture) => {
                solid_texture.get_colour(u, v, point)
            }
            Texture::Checker(checker_texture) => {
                checker_texture.get_colour(u, v, point)
            }
            Texture::Perlin(perlin_texture) => {
                perlin_texture.get_colour(u, v, point)
            }
        }
    }
}

#[derive(Copy, Clone, Constructor, Debug, Default)]
pub struct SolidTexture {
    colour: Colour,
}

impl GetTexture for SolidTexture {
    fn get_colour(&self, _: f64, _: f64, _: Point3) -> Colour {
        self.colour
    }
}

#[derive(Clone, Debug)]
pub struct CheckerTexture {
    even_texture: Box<Texture>,
    odd_texture: Box<Texture>,
    pub inv_scale: f64,
}

impl GetTexture for CheckerTexture {
    fn get_colour(&self, u: f64, v: f64, point: Point3) -> Colour {
        let u_pos = (u * self.inv_scale).round() as u16;
        let v_pos = (v * self.inv_scale).round() as u16;

        let sampled_texture = if u_pos.rem_euclid(2) == v_pos.rem_euclid(2) {
            self.even_texture.clone()
        } else {
            self.odd_texture.clone()
        };

        match *sampled_texture {
            Texture::Solid(solid_texture) => {
                solid_texture.get_colour(u, v, point)
            }
            Texture::Checker(checker_texture) => {
                checker_texture.get_colour(u, v, point)
            }
            Texture::Perlin(perlin_texture) => {
                perlin_texture.get_colour(u, v, point)
            }
        }
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

#[derive(Clone, Debug, Constructor)]
pub struct PerlinTexture {
    scale: f64,
    colour: Colour,
}

impl GetTexture for PerlinTexture {
    fn get_colour(&self, _: f64, _: f64, point: Point3) -> Colour {
        let point = Point3::new(point.x, 0., point.z);
        let origin_corner = Point3::new(
            self.floor_to_scale(point.x),
            self.floor_to_scale(point.y),
            self.floor_to_scale(point.z),
        );

        let mut corner_values = Vec::new();
        for x in 0..=1 {
            for y in 0..=1 {
                for z in 0..=1 {
                    let corner_point = Point3::new(
                        origin_corner.x + x as f64 * self.scale,
                        origin_corner.y + y as f64 * self.scale,
                        origin_corner.z + z as f64 * self.scale,
                    );
                    let grid_vector = Self::hash_point_to_vec(corner_point);
                    let offset_vector = point - corner_point;
                    let noise_value = grid_vector.dot(*offset_vector);

                    // Normalise from [-1, 1] to [0, 1]
                    corner_values.push((noise_value / 2.) + 0.5);
                }
            }
        }

        let position_in_cell = (point - origin_corner) / self.scale;

        let x_position = position_in_cell.x;
        let y_position = position_in_cell.y;
        let z_position = position_in_cell.z;

        let interpolated_x = [
            Self::smoothstep(corner_values[0], corner_values[1], z_position),
            Self::smoothstep(corner_values[2], corner_values[3], z_position),
            Self::smoothstep(corner_values[4], corner_values[5], z_position),
            Self::smoothstep(corner_values[6], corner_values[6], z_position),
        ];

        let interpolated_y = [
            Self::smoothstep(interpolated_x[0], interpolated_x[1], y_position),
            Self::smoothstep(interpolated_x[2], interpolated_x[3], y_position),
        ];

        let value =
            Self::smoothstep(interpolated_y[0], interpolated_y[1], x_position);
        self.colour * if value > 0.5 {1.} else {0.}
    }
}

impl PerlinTexture {
    fn hash_point_to_vec(point: Point3) -> Vec3 {
        let point = if point.near_zero() {
            Point3::new(47., 103., 209.)
        } else {
            point
        };
        // https://www.shadertoy.com/view/4djSRW#
        let mut point = Self::fract(point * Vec3::new(0.1031, 0.1030, 0.0973));
        point += point.dot(Vec3::new(point.x, point.y, point.z));
        point = Point3::from_vector(
            (Vec3::new(point.x, point.x, point.y)
                + Vec3::new(point.y, point.x, point.x))
                * Vec3::new(point.z, point.y, point.x),
        );
        let result = Self::fract(point).unit();

        // Normalise from [0, 1] to [-1, 1]
        (result - 0.5) * 2.
    }

    fn fract(point: Point3) -> Point3 {
        point - point.floor()
    }

    fn floor_to_scale(&self, val: f64) -> f64 {
        (val / self.scale).floor() * self.scale
    }

    fn smoothstep(a: f64, b: f64, t: f64) -> f64 {
        Self::lerp(a, b, 3. * t * t - 2. * t * t * t)
    }

    fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a * (1. - t) + b * t
    }
}
