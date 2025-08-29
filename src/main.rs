#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

mod camera;
mod colour;
mod geometry;
mod hittables;
mod interval;
mod scene_reader;
mod textures;

use crate::camera::Camera;

use derive_more::Display;
use glam::Vec3;
use image::ImageResult;
use std::path::Path;

#[allow(dead_code)]
#[derive(Display)]
enum Profile {
    Debug,
    Release,
    Insane,
    OvernightRender,
    ManyBounces,
}

fn main() -> ImageResult<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let profile = match args[1].to_ascii_lowercase().as_str() {
        "debug" => Profile::Debug,
        "release" => Profile::Release,
        "insane" => Profile::Insane,
        "overnight" => Profile::OvernightRender,
        "bounce" => Profile::ManyBounces,
        _ => panic!("Invalid profile"),
    };
    let (image_width, rays_per_pixel, max_ray_bounces) = match profile {
        Profile::Debug => (800, 10, 10),
        Profile::Release => (800, 100, 10),
        Profile::Insane => (800, 1_000, 10),
        Profile::OvernightRender => (1920, 5_000, 10),
        Profile::ManyBounces => (800, 100, 50),
    };
    let scene_name = &args[2];
    let (world, look_from, look_at, fov) = scene_reader::reader::read_scene(
        format!("scenes/{}.scene", scene_name.to_ascii_lowercase()),
    );
    let camera = Camera::initialise(
        image_width,
        rays_per_pixel,
        max_ray_bounces,
        fov,
        look_from,
        look_at,
        Vec3::new(0., 1., 1e-9),
        10.,
        0.,
    );
    let image = camera.render(&world);
    let path = format!(
        "{scene_name}/{}.png",
        profile.to_string().to_ascii_lowercase()
    );
    let output = Path::new(&path);
    image.save(output)
}
