use crate::{
    colour::Colour,
    scene_reader::reader::ReadDictionary,
    textures::{
        checker_texture::CheckerTexture, gradient_texture::GradientTexture,
        material::Material, perlin_texture::PerlinTexture,
        stripe_texture::StripeTexture, texture::Texture,
    },
};

use collar::*;

fn get_colour(colour_name: &str, colours: ReadDictionary<Colour>) -> Colour {
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

pub(super) fn parse_solid(
    description: &str,
    colours: ReadDictionary<Colour>,
) -> Texture {
    get_colour(description, colours).to_texture()
}

pub(super) fn parse_perlin(
    description: &str,
    colours: ReadDictionary<Colour>,
) -> Texture {
    let (scale, colour_name) = description.split_once(",").unwrap_or_else(|| {
        panic!(
            "{description:?} is an insufficient description for perlin texture"
        )
    });
    PerlinTexture::new(parse_f32(scale), get_colour(colour_name, colours))
        .wrap()
}
pub(super) fn parse_checkerboard(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> Texture {
    let Ok([size, texture_one, texture_two]) =
        description.split(",").collect_array_checked()
    else {
        panic!(
            "{description:?} is an invalid description for checkerboard texture"
        )
    };
    CheckerTexture::new(
        get_texture(texture_one, textures),
        get_texture(texture_two, textures),
        parse_f32(size),
    )
    .wrap()
}

pub(super) fn parse_stripe(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> Texture {
    let Ok([size, even_texture, odd_texture, direction]) =
        description.split(",").collect_array_checked()
    else {
        panic!("{description:?} is an invalid description for striped texture")
    };
    StripeTexture::new_with_dir_name(
        get_texture(even_texture, textures),
        get_texture(odd_texture, textures),
        parse_f32(size),
        direction,
    )
    .unwrap_or_else(|| panic!("{direction:?} is not a valid direction"))
    .wrap()
}

pub(super) fn parse_gradient(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> Texture {
    let Ok([bottom_texture, top_texture, direction]) =
        description.split(",").collect_array_checked()
    else {
        panic!("{description:?} is an invalid description for gradient texture")
    };
    GradientTexture::new_with_dir_name(
        get_texture(bottom_texture, textures),
        get_texture(top_texture, textures),
        direction,
    )
    .unwrap_or_else(|| panic!("{direction:?} is not a valid direction"))
    .wrap()
}
