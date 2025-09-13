use std::sync::{Arc, Mutex};

use crate::{
    colour::{Colour, map_colours},
    geometry::{
        ray::Ray,
        vector::{Point3, VecRand},
    },
    hittables::hittable::{Hittable, HittableList},
    interval::Interval,
    textures::texture::GetTexture,
};
use glam::Vec3;
use image::{Rgb, RgbImage};
use rand::{Rng, rng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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
    sky_top_colour: Colour,
    sky_bottom_colour: Colour,
}

pub type CameraInfo = (Point3, Point3, f32, f32, Colour, Colour, f32, f32);

impl Camera {
    pub fn initialise(
        (image_width, rays_per_pixel, max_ray_bounces): (u32, u16, u16),
        (
            look_from,
            look_at,
            fov,
            aspect_ratio,
            sky_top_colour,
            sky_bottom_colour,
            focus_distance,
            defocus_angle,
        ): CameraInfo,
    ) -> Camera {
        let image_height = (image_width as f32 / aspect_ratio).floor() as u32;
        let pixel_sample_scale = 1. / f32::from(rays_per_pixel);

        let camera_center = look_from;

        let theta = fov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * focus_distance;
        let viewport_width =
            viewport_height * ((image_width as f32) / (image_height as f32));

        let w = (look_from - look_at).normalize();
        let u = Vec3::new(0., 1., 1e-10).cross(w).normalize();
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
            sky_top_colour,
            sky_bottom_colour,
        }
    }

    fn ray_colour(&self, mut ray: Ray, world: &HittableList) -> Colour {
        let mut accumulated = Colour::WHITE;
        let mut light_hit = false;
        for _ in 0..self.max_ray_bounces {
            if let Some(data) =
                world.was_hit(ray, Interval::new(0.001, f32::INFINITY))
            {
                let (u, v) = (data.u, data.v);

                let material = data.clone().material;

                if material.is_light {
                    accumulated *= material.texture.get_colour(u, v);
                    light_hit = true;
                    break;
                }

                ray = if material.is_glass {
                    material.refract(ray, &data)
                } else {
                    material.lerp_reflect(ray, &data)
                };

                accumulated *= material.texture.get_colour(u, v);
            } else {
                light_hit = true;
                accumulated *= {
                    let unit_vector = ray.direction.normalize();
                    let vert_ratio = 0.5 * (unit_vector.y + 1.);

                    Colour::lerp(
                        self.sky_bottom_colour,
                        self.sky_top_colour,
                        vert_ratio,
                    )
                };
                break;
            }
        }
        if light_hit {
            accumulated
        } else {
            Colour::BLACK
        }
    }

    pub fn render(&self, world: &HittableList, report_count: u32) -> RgbImage {
        if report_count != 0 {
            let pixel_count = self.image_height * self.image_width;
            println!("0% done (0/{pixel_count})");
            let pixel_report_increment = pixel_count / report_count;
            let done_pixels = Arc::new(Mutex::new(0));
            RgbImage::from_par_fn(
                self.image_width,
                self.image_height,
                |i, j| {
                    let colour = self.get_pixel_colour(i, j, world);
                    let mut done = done_pixels.lock().unwrap();
                    *done += 1;
                    if *done % pixel_report_increment == 0 {
                        println!(
                            "{}% done ({done}/{pixel_count})",
                            100 * *done / pixel_count,
                        );
                    }
                    colour
                },
            )
        } else {
            RgbImage::from_par_fn(
                self.image_width,
                self.image_height,
                |i, j| self.get_pixel_colour(i, j, world),
            )
        }
    }

    fn get_pixel_colour(
        &self,
        i: u32,
        j: u32,
        world: &HittableList,
    ) -> Rgb<u8> {
        let colour = (0..self.rays_per_pixel)
            .into_par_iter()
            .map(|_| {
                let ray = self.get_ray(i, j);
                self.ray_colour(ray, world)
            })
            .sum::<Colour>()
            * self.pixel_sample_scale;
        let (r, g, b) = map_colours(&colour);
        Rgb([r, g, b])
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
        Ray::new(ray_origin, ray_direction)
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
