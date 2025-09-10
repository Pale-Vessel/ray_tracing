mod camera;
mod colour;
mod file_utils;
mod geometry;
mod hittables;
mod interval;
mod scene_reader;
mod textures;

use crate::{
    camera::Camera, file_utils::clean_scenes,
    scene_reader::scene_parser::read_scene,
};
use clap::Parser;
use glam::Vec3;
use image::ImageResult;

/// Program to render images from a `.scene` file
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Profile to render image at
    #[arg(short, long, default_value = "clean_scenes")]
    profile: String,

    /// Which image to render
    #[arg(short, long, default_value = "")]
    scene: String,

    /// Whether to print progress reports
    #[arg(short, long, default_value_t = 10)]
    report_count: u32,
}

fn main() -> ImageResult<()> {
    let args = Args::parse();
    let (profile, scene_name, progress_reports) =
        (args.profile, args.scene, args.report_count);
    let profile_data = match profile.as_str() {
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
    let (world, camera_info) =
        read_scene(format!("scenes/{}.scene", scene_name.to_ascii_lowercase()));
    let camera = Camera::initialise(
        profile_data,
        camera_info,
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
