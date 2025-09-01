extern crate ray_tracing;
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

fn main() {
    let args = Args::parse();
    let (profile, scene_name) = (args.profile, args.to_render);
    let _ = ray_tracing::render(&profile, &scene_name);
}