use collar::CollectArray;

use crate::{
    colour::Colour,
    geometry::Point3,
    hittables::hittable::HittableObject,
    scene_reader::{
        ReadDictionary, WriteDictionary,
        material_parser::{parse_full, parse_glass, parse_light, parse_opaque},
        object_parser::{parse_sphere, parse_triangle},
        parse_f32,
        scene_parser::read_scene,
        texture_parser::{
            parse_checkerboard, parse_gradient, parse_perlin, parse_solid,
            parse_stripe,
        },
    },
    textures::{material::Material, texture::Texture},
};

pub(super) fn parse_camera_data(
    description: &str,
) -> (Point3, Point3, f32, f32, f32, f32, f32) {
    let description = description.replace(['(', ')'], "");
    let Ok(
        [
            from_x,
            from_y,
            from_z,
            at_x,
            at_y,
            at_z,
            camera_tilt,
            fov,
            aspect_ratio,
            focus_distance,
            defocus_angle,
        ],
    ) = description.split(',').collect_array_checked()
    else {
        panic!(
            "{description:?} is not a valid description for the camera; 
        expected (from_x, from_y, from_z), (at_x, at_y, at_z), fov, aspect_ratio, focus_distance, defocus_angle"
        )
    };
    let [
        from_x,
        from_y,
        from_z,
        at_x,
        at_y,
        at_z,
        camera_tilt,
        fov,
        aspect_ratio,
        focus_distance,
        defocus_angle,
    ] = [
        from_x,
        from_y,
        from_z,
        at_x,
        at_y,
        at_z,
        camera_tilt,
        fov,
        aspect_ratio,
        focus_distance,
        defocus_angle,
    ]
    .map(parse_f32);
    (
        Point3::new(from_x, from_y, from_z),
        Point3::new(at_x, at_y, at_z),
        camera_tilt,
        fov,
        aspect_ratio,
        focus_distance,
        defocus_angle,
    )
}

pub(super) fn parse_sky_colour(description: &str) -> (Colour, Colour) {
    let description = description.replace(['(', ')'], "");
    let Ok([r1, g1, b1, r2, g2, b2]) =
        description.split(',').collect_array_checked()
    else {
        panic!(
            "{description:?} is not a valid description for the sky colour; 
        expected (r1, g1, b1), (r2, g2, b2)"
        )
    };
    let [r1, g1, b1, r2, g2, b2] = [r1, g1, b1, r2, g2, b2].map(parse_f32);
    (Colour::new(r1, g1, b1), Colour::new(r2, g2, b2))
}

pub(super) fn parse_row(
    row: &str,
    points: WriteDictionary<Point3>,
    colours: WriteDictionary<Colour>,
    textures: WriteDictionary<Texture>,
    materials: WriteDictionary<Material>,
) -> Option<Vec<HittableObject>> {
    let row = row.split_whitespace().collect::<String>();
    if row.is_empty() || row.starts_with("//") {
        return None;
    }
    let (row_type, row_data) = row
        .split_once(';')
        .unwrap_or_else(|| panic!("{row:?} - row type not properly delimited"));
    if row_type == "object" {
        return Some(parse_object(row_data, materials, points));
    }
    if row_type == "inherit" {
        return Some(parse_scene(row_data));
    }
    let (name, description) = row_data
        .split_once(';')
        .unwrap_or_else(|| panic!("Name not provided for row {row}"));
    let name = name.strip_prefix("name=").unwrap_or(name).to_owned();
    match row_type {
        "point" => parse_point(name, description, points),
        "colour" => parse_colour(name, description, colours),
        "texture" => parse_texture(name, description, textures, colours),
        "material" => parse_material(name, description, materials, textures),
        _ => panic!("{row_type:?} is not a valid row type"),
    }
    None
}

fn parse_scene(scene_name: &str) -> Vec<HittableObject> {
    read_scene(format!("scenes/{}.scene", scene_name.to_ascii_lowercase()))
        .0
        .data
}

fn parse_point(
    name: String,
    description: &str,
    points: WriteDictionary<Point3>,
) {
    let [x, y, z] = description
        .split(',')
        .collect::<Vec<_>>()
        .try_into()
        .unwrap_or_else(|_| {
            panic!("Expected three parameters for colour, got {description:?}")
        });
    let point = Point3::new(parse_f32(x), parse_f32(y), parse_f32(z));
    points.insert(name, point);
}

fn parse_colour(
    name: String,
    description: &str,
    colours: WriteDictionary<Colour>,
) {
    let [red, green, blue] = description
        .split(',')
        .collect::<Vec<_>>()
        .try_into()
        .unwrap_or_else(|_| {
            panic!("Expected three parameters for colour, got {description:?}")
        });
    let colour = Colour::new(parse_f32(red), parse_f32(green), parse_f32(blue));
    colours.insert(name, colour);
}

fn parse_texture(
    name: String,
    description: &str,
    textures: WriteDictionary<Texture>,
    colours: ReadDictionary<Colour>,
) {
    let (texture_type, description) =
        description.split_once(';').unwrap_or_else(|| {
            panic!("Type of texture not properly delimited for {description}")
        });
    let texture_type =
        texture_type.strip_prefix("type=").unwrap_or(texture_type);
    let texture = match texture_type {
        "solid" => parse_solid(description, colours),
        "perlin" => parse_perlin(description, colours),
        "checker" => parse_checkerboard(description, textures),
        "stripe" => parse_stripe(description, textures),
        "gradient" => parse_gradient(description, textures),
        "uv" => Ok(Texture::UV),
        _ => panic!("{texture_type:?} is not a valid texture"),
    }
    .unwrap();
    textures.insert(name, texture);
}

fn parse_material(
    name: String,
    description: &str,
    materials: WriteDictionary<Material>,
    textures: ReadDictionary<Texture>,
) {
    let (mode, description) =
        description.split_once(';').unwrap_or_else(|| {
            panic!("Material mode not provided for {description}")
        });
    let mode = mode.strip_prefix("type=").unwrap_or(mode);
    let material = match mode {
        "full" => parse_full(description, textures),
        "opaque" => parse_opaque(description, textures),
        "light" => parse_light(description, textures),
        "glass" => parse_glass(description, textures),
        _ => panic!("{mode:?} is an invalid mode"),
    }
    .unwrap();
    materials.insert(name, material);
}

fn parse_object(
    description: &str,
    materials: WriteDictionary<Material>,
    points: ReadDictionary<Point3>,
) -> Vec<HittableObject> {
    let (object_type, description) = description
        .split_once(';')
        .unwrap_or_else(|| panic!("Object type not given for {description}"));
    let object_type = object_type.strip_prefix("type=").unwrap_or(object_type);
    vec![
        match object_type {
            "sphere" => parse_sphere(description, materials, points),
            "triangle" => parse_triangle(description, materials, points),
            _ => panic!("{object_type:?} is not a valid object"),
        }
        .unwrap(),
    ]
}
