use collar::CollectArray;

use crate::{
    colour::Colour,
    scene_reader::{ReadDictionary, get_colour, get_texture, parse_f32},
    textures::{
        checker_texture::CheckerTexture, gradient_texture::GradientTexture,
        perlin_texture::PerlinTexture, stripe_texture::StripeTexture,
        texture::Texture,
    },
};

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
            "{description:?} is an insufficient description for perlin texture - expected `scale, colour_name`"
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
            "{description:?} is an invalid description for checkerboard texture - expected `size, texture_one, texture, two`"
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
        panic!(
            "{description:?} is an invalid description for striped texture - expected `size, even_texture, odd_texture, direction`"
        )
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
        panic!(
            "{description:?} is an invalid description for gradient texture - expected `bottom_texture, top_texture, direction`"
        )
    };
    GradientTexture::new_with_dir_name(
        get_texture(bottom_texture, textures),
        get_texture(top_texture, textures),
        direction,
    )
    .unwrap_or_else(|| panic!("{direction:?} is not a valid direction"))
    .wrap()
}
