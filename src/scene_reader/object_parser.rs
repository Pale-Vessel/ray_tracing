use thiserror::Error;

use crate::{
    geometry::Point3,
    hittables::{
        hittable::HittableObject::{self},
        sphere::Sphere,
        triangle::Triangle,
    },
    scene_reader::{ReadDictionary, get_material, parse_f32},
    textures::material::Material,
};

#[derive(Debug, Error)]
pub enum ObjectError {
    #[error("{0} is not a valid description of a sphere")]
    Sphere(String),
    #[error("{0} is not a valid description of a triangle")]
    Triangle(String)
}

type ObjectResult = Result<HittableObject, ObjectError>;

fn get_point(point_name: &str, points: ReadDictionary<Point3>) -> Point3 {
    *points
        .get(point_name)
        .unwrap_or_else(|| panic!("{point_name:?} is not a known point name"))
}

pub(super) fn parse_sphere(
    description: &str,
    materials: ReadDictionary<Material>,
    points: ReadDictionary<Point3>,
) -> ObjectResult {
    let description = description.replace(['(', ')'], "");
    let description_parts = description.split(',').collect::<Vec<_>>();
    let (center, radius, material) = match description_parts.len() {
        3 => {
            let [point_name, radius, material_name] =
                description_parts.try_into().unwrap();
            let center = get_point(point_name, points);
            let radius = parse_f32(radius);
            let material = get_material(material_name, materials);
            Ok((center, radius, material))
        }
        5 => {
            let [x, y, z, radius, material_name] =
                description_parts.try_into().unwrap();
            let [x, y, z] = [x, y, z].map(parse_f32);
            let center = Point3::new(x, y, z);
            let radius = parse_f32(radius);
            let material = get_material(material_name, materials);
            Ok((center, radius, material))
        }
        _ => Err(ObjectError::Sphere(description.to_owned()))//panic!("{description:?} is not a valid description of a sphere"),
    }?;
    Ok(Sphere::new(center, radius, material).into())
}

pub(super) fn parse_triangle(
    description: &str,
    materials: ReadDictionary<Material>,
    points: ReadDictionary<Point3>,
) -> ObjectResult {
    let description = description.replace(['(', ')'], "");
    let description_parts = description.split(',').collect::<Vec<_>>();
    let (corner_one, corner_two, corner_three, material) =
        match description_parts.len() {
            4 => {
                let [
                    corner_one_name,
                    corner_two_name,
                    corner_three_name,
                    material_name,
                ] = description_parts.try_into().unwrap();
                let [corner_one, corner_two, corner_three] =
                    [corner_one_name, corner_two_name, corner_three_name]
                        .map(|name| get_point(name, points));
                let material = get_material(material_name, materials);
                Ok((corner_one, corner_two, corner_three, material))
            }
            10 => {
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
                    material_name,
                ] = description_parts.try_into().unwrap();
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
                    x_one, y_one, z_one, x_two, y_two, z_two, x_three, y_three,
                    z_three,
                ]
                .map(parse_f32);
                let material = get_material(material_name, materials);
                let (corner_one, corner_two, corner_three) = (
                    Point3::new(x_one, y_one, z_one),
                    Point3::new(x_two, y_two, z_two),
                    Point3::new(x_three, y_three, z_three),
                );
                Ok((corner_one, corner_two, corner_three, material))
            }
            _ => Err(ObjectError::Triangle(description.to_owned()))
        }?;
    Ok(Triangle::new(corner_one, corner_two, corner_three, material).into())
}
