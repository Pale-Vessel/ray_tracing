#![allow(unused_variables)]

use std::{collections::HashMap, fs::File, io::Read};

use crate::{
    colour::Colour,
    geometry::vector::Point3,
    hittables::hittable::HittableList,
    scene_reader::row_parser::{
        parse_camera_data, parse_row, parse_sky_colour,
    },
};

pub fn read_scene(
    path: String,
) -> (HittableList, (Point3, Point3, f32, f32, Colour, Colour)) {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Contents unreadable");

    let mut points = HashMap::new();
    let mut textures = HashMap::new();
    let mut materials = HashMap::new();
    let mut colours = HashMap::new();
    let lowered = contents.to_ascii_lowercase();
    let mut lines = lowered.lines();

    let first_line = lines
        .next()
        .expect("camera data not given")
        .split_whitespace()
        .collect::<String>();
    let (look_from, look_at, fov, aspect_ratio) = parse_camera_data(first_line);

    let second_line = lines
        .next()
        .expect("sky colour data not given")
        .split_whitespace()
        .collect::<String>();
    let (sky_top_colour, sky_bottom_colour) = parse_sky_colour(second_line);

    let objects = lines
        .flat_map(|row| {
            parse_row(
                row,
                &mut points,
                &mut colours,
                &mut textures,
                &mut materials,
            )
        })
        .flatten()
        .collect::<HittableList>()
        .optimise();
    (
        objects,
        (look_from,
        look_at,
        fov,
        aspect_ratio,
        sky_top_colour,
        sky_bottom_colour,)
    )
}
