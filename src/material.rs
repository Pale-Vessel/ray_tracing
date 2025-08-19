use rand::{Rng, rng};

use crate::{hittable::HitRecord, ray::Ray, texture::Texture, vector};

#[derive(Clone, Debug, Default)]
pub struct Material {
    smoothness: f64,
    pub texture: Texture,
    pub refraction_chance: f64,
    refractive_index: f64,
}

impl Material {
    pub const fn new(
        smoothness: f64,
        texture: Texture,
        refraction_chance: f64,
        refractive_index: f64,
    ) -> Self {
        let refractive_index = refractive_index.max(0.000_000_1);
        Material {
            smoothness,
            texture,
            refraction_chance,
            refractive_index,
        }
    }

    pub const fn new_no_refract(smoothness: f64, texture: Texture) -> Self {
        Self::new(smoothness, texture, 0., 0.)
    }

    pub const fn new_glass(refractive_index: f64, texture: Texture) -> Self {
        Self::new(0., texture, 1., refractive_index)
    }

    pub fn diffuse_reflection(ray: Ray, record: &HitRecord) -> Ray {
        let scatter_direction =
            record.normal_vector + vector::Vec3::rand_unit_vector();
        let scatter_direction = if scatter_direction.near_zero() {
            record.normal_vector
        } else {
            scatter_direction
        };

        Ray::new(record.collision_point, scatter_direction, ray.time)
    }

    pub fn specular_reflection(ray: Ray, record: &HitRecord) -> Ray {
        let reflected = ray.direction.reflect(record.normal_vector);
        Ray::new(record.collision_point, reflected, ray.time)
    }

    pub fn lerp_reflect(&self, ray: Ray, record: &HitRecord) -> Ray {
        let diffuse_ray = Self::diffuse_reflection(ray, record);
        let specular_ray = Self::specular_reflection(ray, record);
        let direction = self.smoothness * specular_ray.direction
            + (1. - self.smoothness) * diffuse_ray.direction;
        Ray::new(diffuse_ray.origin, direction.unit(), ray.time)
    }

    pub fn refract(&self, ray: Ray, record: HitRecord) -> Ray {
        let refractive_index = if record.front_face {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit = ray.direction.unit();

        let cos_theta = -unit.dot(record.normal_vector).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();

        let mut rng = rng();
        let direction = if refractive_index * sin_theta < 1.
            || rng.random_bool(Self::reflectance(cos_theta, refractive_index))
        {
            unit.refract(record.normal_vector, refractive_index)
        } else {
            unit.reflect(record.normal_vector)
        }
        .unit();

        Ray::new(record.collision_point, direction, ray.time)
    }

    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
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
