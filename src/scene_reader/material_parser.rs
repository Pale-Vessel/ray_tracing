use collar::CollectArray;
use thiserror::Error;

use crate::{
    scene_reader::{ReadDictionary, get_texture, parse_bool, parse_f32},
    textures::{material::Material, texture::Texture},
};

#[derive(Error, Debug)]
pub enum MaterialError {
    #[error("{0} is not a valid description for a material")]
    Full(String),
    #[error("{0} is not a valid description for an opaque material")]
    Opaque(String),
    #[error("{0} is not a valid description for a light")]
    Light(String),
    #[error("{0} is not a valid description for a glass")]
    Glass(String),
}

type MaterialResult = Result<Material, MaterialError>;

pub(super) fn parse_full(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> MaterialResult {
    let Ok(
        [
            smoothness,
            texture_name,
            refraction_chance,
            refractive_index,
            is_light,
        ],
    ) = description.split(',').collect_array_checked()
    else {
        return Err(MaterialError::Full(description.to_owned()));
    };
    let smoothness = parse_f32(smoothness);
    let texture = get_texture(texture_name, textures);
    let refraction_chance = parse_bool(refraction_chance);
    let refractive_index = parse_f32(refractive_index);
    let is_light = is_light == "true";
    Ok(Material::new(
        smoothness,
        texture,
        refraction_chance,
        refractive_index,
        is_light,
    ))
}

pub(super) fn parse_opaque(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> MaterialResult {
    let Ok([smoothness, texture_name]) =
        description.split(',').collect_array_checked()
    else {
        return Err(MaterialError::Opaque(description.to_owned()));
    };
    let smoothness = parse_f32(smoothness);
    let texture = get_texture(texture_name, textures);
    Ok(Material::new_opaque(smoothness, texture))
}

pub(super) fn parse_light(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> MaterialResult {
    let Ok([texture_name]) = description.split(',').collect_array_checked()
    else {
        return Err(MaterialError::Light(description.to_owned()));
    };
    let texture = get_texture(texture_name, textures);
    Ok(Material::new_light(texture))
}

pub(super) fn parse_glass(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> MaterialResult {
    let Ok([refractive_index, texture_name]) =
        description.split(',').collect_array_checked()
    else {
        return Err(MaterialError::Glass(description.to_owned()));
    };
    let refractive_index = parse_f32(refractive_index);
    let texture = get_texture(texture_name, textures);
    Ok(Material::new_glass(refractive_index, texture))
}
