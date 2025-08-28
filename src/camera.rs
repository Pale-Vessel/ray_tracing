use std::sync::{Arc, Mutex};

use crate::{
    colour::{Colour, map_colours},
    hittable::{Hittable, HittableList},
    interval::Interval,
    ray::Ray,
    texture::GetTexture,
    vector::{Point3, VecRand},
};
use glam::Vec3;
use image::{Rgb, RgbImage};
use rand::{Rng, rng};
use rayon::prelude::*;

#[derive(Debug)]
pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel_upper_left: Point3,
    horizontal_pixel_delta: Vec3,
    vertical_pixel_delta: Vec3,
    rays_per_pixel: u16,
    pixel_sample_scale: f32,
    max_ray_bounces: u16,
    defocus_angle: f32,
    defocus_disk_horiz_radius: Vec3,
    defocus_disk_vert_radius: Vec3,
}

impl Camera {
    const IDEAL_ASPECT_RATIO: f32 = 1.;
    const SKY_TOP_COLOUR: Colour = Colour::new(0.5, 0.5, 0.5);
    const SKY_BOTTOM_COLOUR: Colour = Self::SKY_TOP_COLOUR;

    #[allow(clippy::too_many_arguments)]
    pub fn initialise(
        image_width: u32,
        rays_per_pixel: u16,
        max_ray_bounces: u16,
        fov: f32,
        look_from: Point3,
        look_at: Point3,
        up_vector: Vec3,
        focus_distance: f32,
        defocus_angle: f32,
    ) -> Camera {
        let image_height =
            (image_width as f32 / Self::IDEAL_ASPECT_RATIO).floor() as u32;
        let pixel_sample_scale = 1. / f32::from(rays_per_pixel);

        let camera_center = look_from;

        let theta = fov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * focus_distance;
        let viewport_width =
            viewport_height * ((image_width as f32) / (image_height as f32));

        let w = (look_from - look_at).normalize();
        let u = up_vector.cross(w).normalize();
        let v = w.cross(u);

        let viewport_horizontal = viewport_width * u;
        let viewport_vertical = viewport_height * -v;

        let horizontal_pixel_delta = viewport_horizontal / (image_width as f32);
        let vertical_pixel_delta = viewport_vertical / (image_height as f32);

        let viewport_upper_left = camera_center
            - Point3::from_vector(focus_distance * w)
            - Point3::from_vector(viewport_horizontal / 2.)
            - Point3::from_vector(viewport_vertical / 2.);
        let pixel_upper_left = viewport_upper_left
            + Point3::from_vector(
                horizontal_pixel_delta + vertical_pixel_delta / 2.,
            );

        let defocus_radius =
            focus_distance * (defocus_angle / 2.).to_radians().tan();
        let defocus_disk_horiz_radius = defocus_radius * u;
        let defocus_disk_vert_radius = defocus_radius * u;

        Camera {
            image_width,
            image_height,
            center: camera_center,
            pixel_upper_left,
            horizontal_pixel_delta,
            vertical_pixel_delta,
            rays_per_pixel,
            pixel_sample_scale,
            max_ray_bounces,
            defocus_angle,
            defocus_disk_horiz_radius,
            defocus_disk_vert_radius,
        }
    }

    fn ray_colour(&self, mut ray: Ray, world: &HittableList, depth: u16) -> Colour {
        if depth > self.max_ray_bounces {
            return ray.collected_light;
        }
        if let Some(data) =
            world.did_hit(ray, Interval::new(0.001, f32::INFINITY))
        {
            let (u, v) = (data.u, data.v);

            let material = data.clone().material;

            if material.is_light {
                ray.collected_light += material.texture.get_colour(u, v);
            }

            let mut rng = rng();
            if rng.random_bool(material.refraction_chance as f64) {
                let refracted_ray = material.refract(ray, &data);
                return material.texture.get_colour(u, v)
                    * *self.ray_colour(refracted_ray, world, depth + 1);
            }
            let scattered_ray = material.lerp_reflect(ray, &data);
            return material.texture.get_colour(u, v)
                * *self.ray_colour(scattered_ray, world, depth + 1);
        }

        let unit_vector = ray.direction.normalize();
        let vert_ratio = 0.5 * (unit_vector.y + 1.);

        Self::SKY_BOTTOM_COLOUR * (1. - vert_ratio)
            + Self::SKY_TOP_COLOUR * vert_ratio
    }

    pub fn render(&self, world: &HittableList) -> RgbImage {
        let pixel_report = (self.image_width * self.image_height) / 20;
        let done_pixels = Arc::new(Mutex::new(0));
        RgbImage::from_par_fn(self.image_width, self.image_height, |i, j| {
            let colour = (0..self.rays_per_pixel)
                .into_par_iter()
                .map(|_| {
                    let ray = self.get_ray(i, j);
                    self.ray_colour(ray, world, 0)
                })
                .sum::<Colour>()
                * self.pixel_sample_scale;
            let done_pixels = Arc::clone(&done_pixels);
            let mut num = done_pixels.lock().unwrap();
            *num += 1;
            if *num % pixel_report == 0 {
                println!("{}", *num * 5 / pixel_report)
            }
            let (r, g, b) = map_colours(&colour);
            Rgb([r, g, b])
        })
    }

    fn get_ray(&self, horiz_position: u32, vert_position: u32) -> Ray {
        let offset = Self::sample_square(horiz_position, vert_position);
        let pixel_sample = *self.pixel_upper_left
            + offset.x * self.horizontal_pixel_delta
            + offset.y * self.vertical_pixel_delta;
        let ray_origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - *ray_origin;

        let mut rng = rng();
        let ray_time = rng.random();

        Ray::new(ray_origin, ray_direction, ray_time)
    }

    fn sample_square(horiz_position: u32, vert_position: u32) -> Vec3 {
        let mut rng = rng();
        let horiz_offset = rng.random_range(-0.5..0.5);
        let vert_offset = rng.random_range(-0.5..0.5);
        Vec3::new(
            (horiz_position as f32) + horiz_offset,
            (vert_position as f32) + vert_offset,
            0.,
        )
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let point = Vec3::random_on_unit_disk();
        self.center
            + Point3::from_vector(point.x * self.defocus_disk_horiz_radius)
            + Point3::from_vector(point.y * self.defocus_disk_vert_radius)
    }
}
