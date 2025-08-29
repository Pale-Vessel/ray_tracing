use collar::CollectArray;

use crate::{
    geometry::vector::Point3,
    hittables::{
        hittable::HittableObject::{self},
        sphere::Sphere,
        triangle::Triangle,
    },
    scene_reader::{
        reader::ReadDictionary,
        texture_parser::{get_material, parse_f32},
    },
    textures::material::Material,
};

pub(super) fn parse_sphere(
    description: &str,
    materials: ReadDictionary<Material>,
) -> HittableObject {
    let description = description.replace("(", "").replace(")", "");
    let Ok([x, y, z, radius, material_name]) =
        description.split(",").collect_array_checked()
    else {
        panic!("{description} is not a valid description for a sphere")
    };
    let [x, y, z] = [x, y, z].map(parse_f32);
    let center = Point3::new(x, y, z);
    let radius = parse_f32(radius);
    let material = get_material(material_name, materials);
    Sphere::new(center, radius, material).wrap()
}

pub(super) fn parse_triangle(
    description: &str,
    materials: ReadDictionary<Material>,
) -> HittableObject {
    let description = description.replace("(", "").replace(")", "");
    let Ok(
        [
            x_one,
            y_one,
            z_one,
            x_two,
            y_two,
            z_two,
            x_three,
            y_three,
            z_three,
            material_name,
        ],
    ) = description.split(",").collect_array_checked()
    else {
        panic!("{description} is not a valid description for a sphere")
    };
    let [
        x_one,
        y_one,
        z_one,
        x_two,
        y_two,
        z_two,
        x_three,
        y_three,
        z_three,
    ] = [
        x_one, y_one, z_one, x_two, y_two, z_two, x_three, y_three, z_three,
    ]
    .map(parse_f32);
    let material = get_material(material_name, materials);
    let (corner_one, corner_two, corner_three) = (
        Point3::new(x_one, y_one, z_one),
        Point3::new(x_two, y_two, z_two),
        Point3::new(x_three, y_three, z_three),
    );
    Triangle::new(corner_one, corner_two, corner_three, material).wrap()
}
