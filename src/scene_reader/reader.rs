#![allow(unused_variables)]

use std::{collections::HashMap, fs::File, io::Read};

use collar::CollectArray;

use crate::{
    colour::Colour,
    geometry::vector::Point3,
    hittables::hittable::{HittableList, HittableObject},
    scene_reader::{
        material_parser::{parse_full, parse_glass, parse_light, parse_opaque},
        object_parser::{parse_sphere, parse_triangle},
        texture_parser::{
            parse_checkerboard, parse_f32, parse_gradient, parse_perlin,
            parse_solid, parse_stripe,
        },
    },
    textures::{material::Material, texture::Texture},
};

pub(super) type ReadDictionary<'a, T> = &'a HashMap<String, T>;
type WriteDictionary<'a, T> = &'a mut HashMap<String, T>;

pub fn read_scene(path: String) -> (HittableList, Point3, Point3, f32) {
    let mut file = File::open(path).expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Contents unreadable");
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
            parse_row(row, &mut colours, &mut textures, &mut materials)
        })
        .collect::<HittableList>()
        .optimise();
    (objects, look_from, look_at, fov)
}

fn parse_camera_data(description: String) -> (Point3, Point3, f32) {
    let description = description.replace("(", "").replace(")", "");
    let Ok([from_x, from_y, from_z, at_x, at_y, at_z, fov]) =
        description.split(",").collect_array_checked()
    else {
        panic!("{description} is not a valid description for a sphere")
    };
    let [from_x, from_y, from_z, at_x, at_y, at_z, fov] =
        [from_x, from_y, from_z, at_x, at_y, at_z, fov].map(parse_f32);
    (
        Point3::new(from_x, from_y, from_z),
        Point3::new(at_x, at_y, at_z),
        fov,
    )
}

fn parse_row(
    row: &str,
    colours: WriteDictionary<Colour>,
    textures: WriteDictionary<Texture>,
    materials: WriteDictionary<Material>,
) -> Option<HittableObject> {
    let row = row.split_whitespace().collect::<String>();
    let (row_type, row_data) = row
        .split_once(";")
        .expect("Row type not properly delimited");
    if row_type == "object" {
        return Some(parse_object(row_data, materials));
    }
    let (name, row_data) = row_data.split_once(";").expect("Name not provided");
    let name = name.strip_prefix("name=").unwrap_or(name).to_string();
    match row_type {
        "colour" => parse_colour(name, row_data, colours),
        "texture" => parse_texture(name, row_data, textures, colours),
        "material" => parse_material(name, row_data, materials, textures),
        _ => panic!("{row_type} is not a valid row type"),
    }
    None
}

fn parse_colour(
    name: String,
    description: &str,
    colours: WriteDictionary<Colour>,
) {
    let [red, green, blue] = description
        .split(",")
        .collect::<Vec<_>>()
        .try_into()
        .unwrap_or_else(|_| {
            panic!("Expected three parameters for colour, got {description}")
        });
    let colour = Colour::new(
        red.parse().expect("red was an invalid f32"),
        green.parse().expect("green was an invalid f32"),
        blue.parse().expect("blue was an invalid f32"),
    );
    colours.insert(name, colour);
}

fn parse_texture(
    name: String,
    description: &str,
    textures: WriteDictionary<Texture>,
    colours: ReadDictionary<Colour>,
) {
    let (texture_type, description) = description
        .split_once(";")
        .expect("Type of texture not properly delimited");
    let texture_type = texture_type.strip_prefix("type=").unwrap_or(texture_type);
    let texture = match texture_type {
        "solid" => parse_solid(description, colours),
        "perlin" => parse_perlin(description, colours),
        "checker" => parse_checkerboard(description, textures),
        "stripe" => parse_stripe(description, textures),
        "gradient" => parse_gradient(description, textures),
        "uv" => Texture::UV,
        _ => panic!("{texture_type} is not a valid texture"),
    };
    textures.insert(name, texture);
}

fn parse_material(
    name: String,
    description: &str,
    materials: WriteDictionary<Material>,
    textures: ReadDictionary<Texture>,
) {
    let (mode, description) = description
        .split_once(";")
        .expect("Material mode not provided");
    let mode = mode.strip_prefix("type=").unwrap_or(mode);
    let material = match mode {
        "full" => parse_full(description, textures),
        "opaque" => parse_opaque(description, textures),
        "light" => parse_light(description, textures),
        "glass" => parse_glass(description, textures),
        _ => panic!("{mode} is an invalid mode"),
    };
    materials.insert(name, material);
}

fn parse_object(
    description: &str,
    materials: WriteDictionary<Material>,
) -> HittableObject {
    let (object_type, description) = description
        .split_once(";")
        .unwrap_or_else(|| panic!("Object type not given"));
    let object_type = object_type.strip_prefix("type=").unwrap_or(object_type);
    match object_type {
        "sphere" => parse_sphere(description, materials),
        "triangle" => parse_triangle(description, materials),
        _ => unreachable!(),
    }
}
