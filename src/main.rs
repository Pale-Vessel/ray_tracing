#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

mod camera;
mod colour;
mod file_utils;
mod geometry;
mod hittables;
mod interval;
mod scene_reader;
mod textures;

use crate::{camera::Camera, file_utils::clean_scenes::clean_scenes};
use glam::Vec3;
use image::ImageResult;

use clap::Parser;

/// Program to render images from a `.scene` file
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Profile to render image at
    #[arg(short, long)]
    profile: String,

    /// Which image to render
    #[arg(short, long)]
    to_render: String,
}

fn main() -> ImageResult<()> {
    let args = Args::parse();
    let (profile, scene_name) = (args.profile, args.to_render);
    let (image_width, rays_per_pixel, max_ray_bounces) = match profile.as_str()
    {
        "debug" => (800, 10, 10),
        "release" => (800, 100, 10),
        "insane" => (800, 1_000, 10),
        "overnight" => (1920, 5_000, 10),
        "bounce" => (800, 100, 50),
        "clean_scenes" => {
            clean_scenes();
            return Ok(());
        }
        _ => panic!("Invalid profile"),
    };
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
    let dir_path = format!(r"images\{scene_name}");
    let path = format!(
        r"{dir_path}\{}.png",
        profile.to_string().to_ascii_lowercase()
    );
    std::fs::create_dir_all(dir_path).unwrap();
    image.save(path)
}
