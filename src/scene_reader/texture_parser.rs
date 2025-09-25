use collar::CollectArray;
use thiserror::Error;

use crate::{
    colour::Colour,
    scene_reader::{ReadDictionary, get_colour, get_texture, parse_f32},
    textures::{
        checker_texture::CheckerTexture, gradient_texture::GradientTexture,
        perlin_texture::PerlinTexture, stripe_texture::StripeTexture,
        texture::Texture,
    },
};

#[derive(Debug, Error)]
pub enum TextureError {
    #[error(
        "{0} is an insufficient description for perlin texture - expected `scale, colour_name`"
    )]
    Perlin(String),
    #[error(
        "{0} is an invalid description for checkerboard texture - expected `size, texture_one, texture, two`"
    )]
    Checker(String),
    #[error(
        "{0} is an invalid description for striped texture - expected `size, even_texture, odd_texture, direction`"
    )]
    Stripe(String),
    #[error(
        "{0}  is an invalid description for gradient texture - expected `bottom_texture, top_texture, direction`"
    )]
    Gradient(String),
}

type TextureResult = Result<Texture, TextureError>;

pub(super) fn parse_solid(
    description: &str,
    colours: ReadDictionary<Colour>,
) -> TextureResult {
    Ok(get_colour(description, colours).to_texture())
}

pub(super) fn parse_perlin(
    description: &str,
    colours: ReadDictionary<Colour>,
) -> TextureResult {
    let (scale, colour_name) = description
        .split_once(',')
        .ok_or(TextureError::Perlin(description.to_owned()))?;
    Ok(
        PerlinTexture::new(parse_f32(scale), get_colour(colour_name, colours))
            .wrap(),
    )
}
pub(super) fn parse_checkerboard(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> TextureResult {
    let [size, texture_one, texture_two] = description
        .split(',')
        .collect_array_checked()
        .map_err(|_| TextureError::Checker(description.to_owned()))?;
    Ok(CheckerTexture::new(
        get_texture(texture_one, textures),
        get_texture(texture_two, textures),
        parse_f32(size),
    )
    .wrap())
}

pub(super) fn parse_stripe(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> TextureResult {
    let [size, even_texture, odd_texture, direction] = description
        .split(",")
        .collect_array_checked()
        .map_err(|_| TextureError::Stripe(description.to_owned()))?;
    Ok(StripeTexture::new_with_dir_name(
        get_texture(even_texture, textures),
        get_texture(odd_texture, textures),
        parse_f32(size),
        direction,
    )
    .unwrap_or_else(|| panic!("{direction:?} is not a valid direction"))
    .wrap())
}

pub(super) fn parse_gradient(
    description: &str,
    textures: ReadDictionary<Texture>,
) -> TextureResult {
    let [bottom_texture, top_texture, direction] = description
        .split(",")
        .collect_array_checked()
        .map_err(|_| TextureError::Gradient(description.to_owned()))?;
    Ok(GradientTexture::new_with_dir_name(
        get_texture(bottom_texture, textures),
        get_texture(top_texture, textures),
        direction,
    )
    .unwrap_or_else(|| panic!("{direction:?} is not a valid direction"))
    .wrap())
}
