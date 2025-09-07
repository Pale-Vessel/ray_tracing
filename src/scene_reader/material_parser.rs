use collar::CollectArray;

use crate::{
    scene_reader::{ReadDictionary, get_texture, parse_bool, parse_f32},
    textures::{material::Material, texture::Texture},
};

pub(super) fn parse_full(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> Material {
    let Ok(
        [
            smoothness,
            texture_name,
            refraction_chance,
            refractive_index,
            is_light,
        ],
    ) = description.split(",").collect_array_checked()
    else {
        panic!("{description:?} is not a valid description for a material")
    };
    let smoothness = parse_f32(smoothness);
    let texture = get_texture(texture_name, textures);
    let refraction_chance = parse_bool(refraction_chance);
    let refractive_index = parse_f32(refractive_index);
    let is_light = is_light == "true";
    Material::new(
        smoothness,
        texture,
        refraction_chance,
        refractive_index,
        is_light,
    )
}

pub(super) fn parse_opaque(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> Material {
    let Ok([smoothness, texture_name]) =
        description.split(",").collect_array_checked()
    else {
        panic!(
            "{description:?} is not a valid description for an opaque material"
        )
    };
    let smoothness = parse_f32(smoothness);
    let texture = get_texture(texture_name, textures);
    Material::new_opaque(smoothness, texture)
}

pub(super) fn parse_light(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> Material {
    let Ok([smoothness, texture_name]) =
        description.split(",").collect_array_checked()
    else {
        panic!(
            "{description:?} is not a valid description for a light material"
        )
    };
    let smoothness = parse_f32(smoothness);
    let texture = get_texture(texture_name, textures);
    Material::new_light(smoothness, texture)
}

pub(super) fn parse_glass(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> Material {
    let Ok([refractive_index, texture_name]) =
        description.split(",").collect_array_checked()
    else {
        panic!(
            "{description:?} is not a valid description for a glass material"
        )
    };
    let refractive_index = parse_f32(refractive_index);
    let texture = get_texture(texture_name, textures);
    Material::new_glass(refractive_index, texture)
}
