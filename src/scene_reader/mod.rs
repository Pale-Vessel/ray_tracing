use crate::{
    colour::Colour,
    scene_reader::scene_parser::ReadDictionary,
    textures::{material::Material, texture::Texture},
};

mod material_parser;
mod object_parser;
pub mod scene_parser;
mod texture_parser;

pub(super) fn get_colour(
    colour_name: &str,
    colours: ReadDictionary<Colour>,
) -> Colour {
    *colours
        .get(colour_name)
        .unwrap_or_else(|| panic!("{colour_name:?} is not a known colour name"))
}

pub(super) fn get_texture(
    texture_name: &str,
    textures: ReadDictionary<Texture>,
) -> Texture {
    textures
        .get(texture_name)
        .unwrap_or_else(|| {
            panic!("{texture_name:?} is not a known texture name")
        })
        .clone()
}

pub(super) fn get_material(
    material_name: &str,
    materials: ReadDictionary<Material>,
) -> Material {
    materials
        .get(material_name)
        .unwrap_or_else(|| {
            panic!("{material_name:?} is not a known texture name")
        })
        .clone()
}

pub(super) fn parse_f32(num: &str) -> f32 {
    num.parse()
        .unwrap_or_else(|_| panic!("{num:?} is an invalid f32"))
}

pub(super) fn parse_bool(bool: &str) -> bool {
    bool.parse()
        .unwrap_or_else(|_| panic!("{bool:?} is an invalid boolean"))
}
