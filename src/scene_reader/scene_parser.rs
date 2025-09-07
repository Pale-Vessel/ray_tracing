#![allow(unused_variables)]

use std::{collections::HashMap, fs::File, io::Read};

use crate::{
    geometry::vector::Point3,
    hittables::hittable::HittableList,
    scene_reader::row_parser::{parse_camera_data, parse_row},
};

pub fn read_scene(path: String) -> (HittableList, Point3, Point3, f32) {
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
    let (look_from, look_at, fov) = parse_camera_data(first_line);
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
        .collect::<HittableList>()
        .optimise();
    (objects, look_from, look_at, fov)
}
