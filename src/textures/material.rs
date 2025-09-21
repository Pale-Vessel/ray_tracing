use derive_more::Constructor;
use rand::{Rng, rng};

use crate::{
    geometry::{NearZero, Ray, VecRand},
    hittables::hittable::HitRecord,
    textures::texture::Texture,
};

use glam::Vec3;

#[derive(Clone, Debug, Default, Constructor)]
pub struct Material {
    smoothness: f32,
    pub texture: Texture,
    pub is_glass: bool,
    refractive_index: f32,
    pub is_light: bool,
}

impl Material {
    pub const fn new_opaque(smoothness: f32, texture: Texture) -> Self {
        Self::new(smoothness, texture, false, 0., false)
    }

    pub const fn new_glass(refractive_index: f32, texture: Texture) -> Self {
        let refractive_index = f32::max(refractive_index, 0.000_000_1);
        Self::new(0., texture, true, refractive_index, false)
    }

    pub const fn new_light(texture: Texture) -> Self {
        Self::new(0., texture, false, 0., true)
    }

    pub fn diffuse_reflection(record: &HitRecord) -> Ray {
        let scatter_direction = record.normal_vector + Vec3::rand_unit_vector();
        let scatter_direction = if scatter_direction.near_zero() {
            record.normal_vector
        } else {
            scatter_direction
        };

        Ray::new(record.collision_point, scatter_direction)
    }

    pub fn specular_reflection(ray: Ray, record: &HitRecord) -> Ray {
        let reflected = ray.direction.reflect(record.normal_vector);
        Ray::new(record.collision_point, reflected)
    }

    pub fn lerp_reflect(&self, ray: Ray, record: &HitRecord) -> Ray {
        let diffuse_ray = Self::diffuse_reflection(record);
        let specular_ray = Self::specular_reflection(ray, record);
        let direction = self.smoothness * specular_ray.direction
            + (1. - self.smoothness) * diffuse_ray.direction;
        Ray::new(diffuse_ray.origin, direction.normalize())
    }

    pub fn refract(&self, ray: Ray, record: &HitRecord) -> Ray {
        let refractive_index = if record.front_face {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit = ray.direction.normalize();

        let cos_theta = f32::min(1., -unit.dot(record.normal_vector));
        let sin_theta = (1f32 - cos_theta * cos_theta).sqrt();

        let mut rng = rng();
        let direction = if refractive_index * sin_theta < 1.
            || rng.random_bool(
                Self::reflectance(cos_theta, refractive_index).into(),
            ) {
            unit.refract(record.normal_vector, refractive_index)
        } else {
            unit.reflect(record.normal_vector)
        };

        let direction = direction
            .try_normalize()
            .unwrap_or_else(|| unit.reflect(record.normal_vector));

        Ray::new(record.collision_point, direction)
    }

    fn reflectance(cosine: f32, refractive_index: f32) -> f32 {
        match refractive_index {
            0. => 1.,
            1. => 0.,
            _ => {
                // Schlick approximation
                let r0 = ((1. - refractive_index) / (1. + refractive_index))
                    .powf(2.);
                r0 + (1. - r0) * (1. - cosine).powf(5.)
            }
        }
    }
}
