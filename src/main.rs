mod camera;
mod colour;
mod file_utils;
mod geometry;
mod hittables;
mod interval;
mod scene_reader;
mod textures;

use crate::{
    camera::Camera, file_utils::clean_scenes::clean_scenes,
    scene_reader::reader::read_scene,
};
use clap::Parser;
use glam::Vec3;
use image::ImageResult;

/// Program to render images from a `.scene` file
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Profile to render image at
    #[arg(short, long)]
    profile: String,

    /// Which image to render
    #[arg(short, long)]
    scene: String,

    /// Whether to print progress reports
    #[arg(short, long)]
    report_count: String,
}

fn main() -> ImageResult<()> {
    let args = Args::parse();
    let (profile, scene_name, progress_reports) = (
        args.profile,
        args.scene,
        match args.report_count.to_ascii_lowercase().as_str() {
            "false" => 0,
            x => x.parse().unwrap(),
        },
    );
    let (image_width, rays_per_pixel, max_ray_bounces) = match profile.as_str()
    {
        "debug" => (800, 10, 10),
        "release" => (800, 100, 10),
        "insane" => (800, 1_000, 10),
        "overnight" => (1_920, 5_000, 100),
        "bounce" => (800, 100, 50),
        "bounce_with_insane" => (1_920, 500, 50),
        "clean_scenes" => {
            clean_scenes();
            return Ok(());
        }
        _ => panic!("Invalid profile"),
    };
    let (world, look_from, look_at, fov) =
        read_scene(format!("scenes/{}.scene", scene_name.to_ascii_lowercase()));
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
    let image = camera.render(&world, progress_reports);
    let dir_path = format!(r"images\{scene_name}");
    let path = format!(
        r"{dir_path}\{}.png",
        profile.to_string().to_ascii_lowercase()
    );
    std::fs::create_dir_all(dir_path).unwrap();
    image.save(path)
}
