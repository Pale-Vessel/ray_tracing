mod bounding_box;
mod bvh;
mod camera;
mod checker_texture;
mod colour;
mod hittable;
mod interval;
mod material;
mod perlin_texture;
mod ray;
mod solid_texture;
mod sphere;
mod texture;
mod triangle;
mod vector;

use core::f32;
use derive_more::Display;
use image::ImageResult;
use std::path::Path;

#[allow(unused_imports)]
use rand::{Rng, rng};

use crate::{
    camera::Camera,
    checker_texture::CheckerTexture,
    colour::Colour,
    hittable::{
        HittableList,
        HittableObject::{Sphere as SpheHit, Triangle as TriHit},
    },
    material::Material,
    perlin_texture::PerlinTexture,
    sphere::Sphere,
    texture::Texture,
    triangle::Triangle,
    vector::Point3,
};

use glam::Vec3;

type SceneInfo = (HittableList, Point3, Point3, f32);

#[allow(dead_code)]
#[derive(Display)]
enum Profile {
    Debug,
    Release,
    InsaneRays,
    Insane,
    OvernightRender,
}

const PROFILE: Profile = Profile::InsaneRays;

fn main() -> ImageResult<()> {
    let (image_width, rays_per_pixel, max_ray_bounces) = match PROFILE {
        Profile::Debug => (800, 10, 10),
        Profile::Release => (800, 100, 50),
        Profile::InsaneRays => (800, 1_000, 10),
        Profile::Insane => (1920, 500, 100),
        Profile::OvernightRender => (1920, 5_000, 10),
    };
    let (world, look_from, look_at, fov) = cornell_box();
    let camera = Camera::initialise(
        image_width,
        rays_per_pixel,
        max_ray_bounces,
        fov,
        look_from,
        look_at,
        Vec3::new(0., 1., 1e-9),
        10.,
        0.,
    );
    let image = camera.render(&world);
    let path = format!("image{PROFILE}.png");
    let output = Path::new(&path);
    image.save(output)
}

#[allow(dead_code)]
fn basic_spheres() -> SceneInfo {
    let checkered_texture = Texture::Checker(CheckerTexture::new(
        Colour::new(0.2, 0.3, 0.1).to_texture(),
        Colour::new(0.9, 0.9, 0.9).to_texture(),
        0.32,
    ));

    let ground_material = Material::new_no_refract(0., checkered_texture);

    let mut world = vec![Sphere::new_still(
        Point3::new(0., -1000., -1.),
        1000.,
        ground_material,
    )];

    let glass = Material::new_glass(1.5, Colour::new(1., 1., 1.).to_texture());
    let air =
        Material::new_glass(1. / 1.5, Colour::new(1., 1., 1.).to_texture());
    let rough =
        Material::new_no_refract(1., Colour::new(0.4, 0.2, 0.1).to_texture());
    let smooth =
        Material::new_no_refract(1., Colour::new(1., 0.6, 0.5).to_texture());

    world.push(Sphere::new_still(Point3::new(0., 2., 1.), 1., glass));
    world.push(Sphere::new_still(Point3::new(0., 2., 1.), 0.5, air));
    world.push(Sphere::new_still(Point3::new(-4., 1., 0.), 1., rough));
    world.push(Sphere::new_still(
        // Point3::new(4., 0.5, 0.),
        Point3::new(4., 1.0, 0.),
        1.,
        smooth,
    ));

    (
        world
            .iter()
            .map(|sphere| SpheHit(sphere.clone()))
            .collect::<HittableList>()
            .optimise(),
        Point3::new(13., 2., 3.),
        Point3::new(0., 0., 0.),
        20.,
    )
}

#[allow(dead_code)]
fn bouncing_spheres() -> SceneInfo {
    let checkered_texture = Texture::Checker(CheckerTexture::new(
        Colour::new(0.2, 0.3, 0.1).to_texture(),
        Colour::new(0.9, 0.9, 0.9).to_texture(),
        0.32,
    ));

    let ground_material = Material::new_no_refract(0., checkered_texture);

    let mut world = vec![Sphere::new_still(
        Point3::new(0., -1000., -1.),
        1000.,
        ground_material,
    )];

    let mut rng = rng();
    for a in -5..5 {
        for b in -5..3 {
            let center = Point3::new(
                (a as f32) + 0.9 * rng.random::<f32>(),
                0.2,
                (b as f32) + 0.9 * rng.random::<f32>(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let colour = Colour::new_random().to_texture();
                let material = Material::new_no_refract(rng.random(), colour);
                world.push(Sphere::new_still(center, 0.2, material));
            }
        }
        for b in 3..5 {
            let center = Point3::new(
                (a as f32) + 0.9 * rng.random::<f32>(),
                0.2,
                (b as f32) + 0.9 * rng.random::<f32>(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let colour = Colour::new_random().to_texture();
                let material = Material::new_glass(1.5, colour);
                world.push(Sphere::new_still(center, 0.2, material));
            }
        }
    }

    let glass = Material::new_glass(1.5, Colour::new(1., 1., 1.).to_texture());
    let air =
        Material::new_glass(1. / 1.5, Colour::new(1., 1., 1.).to_texture());
    let rough =
        Material::new_no_refract(1., Colour::new(0.4, 0.2, 0.1).to_texture());
    let smooth =
        Material::new_no_refract(1., Colour::new(1., 0.6, 0.5).to_texture());

    world.push(Sphere::new_still(Point3::new(0., 2., 1.), 1., glass));
    world.push(Sphere::new_still(Point3::new(0., 2., 1.), 0.5, air));
    world.push(Sphere::new_still(Point3::new(-4., 1., 0.), 1., rough));
    world.push(Sphere::new_still(
        // Point3::new(4., 0.5, 0.),
        Point3::new(4., 1.0, 0.),
        1.,
        smooth,
    ));

    (
        world
            .iter()
            .map(|sphere| SpheHit(sphere.clone()))
            .collect::<HittableList>()
            .optimise(),
        Point3::new(13., 2., 3.),
        Point3::new(0., 0., 0.),
        20.,
    )
}

#[allow(dead_code)]
fn checkered_spheres() -> SceneInfo {
    let checkered_texture = Texture::Checker(CheckerTexture::new(
        Colour::new(0.5, 0.5, 0.5).to_texture(),
        Colour::new(0., 0.3, 0.7).to_texture(),
        4.,
    ));

    let material = Material::new_no_refract(1., checkered_texture);

    let world = [
        Sphere::new_still(Point3::new(0., -10., 0.), 10., material.clone()),
        Sphere::new_still(Point3::new(0., 10., 0.), 10., material),
    ];

    (
        world
            .iter()
            .map(|sphere| SpheHit(sphere.clone()))
            .collect::<HittableList>()
            .optimise(),
        Point3::new(13., 2., 3.),
        Point3::new(0., 1., 0.),
        50.,
    )
}

#[allow(dead_code)]
fn perlin_spheres() -> SceneInfo {
    let perlin_texture =
        Texture::Perlin(PerlinTexture::new(5., Colour::new(1., 1., 1.)));
    let smooth_material =
        Material::new_no_refract(0.5, Colour::new(0.7, 0.7, 0.5).to_texture());
    let perlin_material = Material::new_no_refract(0., perlin_texture);
    let world = [
        Sphere::new_still(
            Point3::new(0., -1000.0, 0.),
            1000.0,
            smooth_material,
        ),
        Sphere::new_still(Point3::new(5., 3., 0.), 3., perlin_material),
    ];

    (
        world
            .iter()
            .map(|sphere| SpheHit(sphere.clone()))
            .collect::<HittableList>()
            .optimise(),
        Point3::new(13., 2., 3.),
        Point3::new(0., 1., 0.),
        50.,
    )
}

#[allow(dead_code)]
fn triangle() -> SceneInfo {
    let blue =
        Material::new_no_refract(0., Colour::new(0., 0., 1.).to_texture());
    let triangle_texture = Material::new_no_refract(
        1.,
        PerlinTexture::new(10., Colour::new(1., 1., 1.)).wrap(),
    );
    let corner_one = Point3::new(0., 0., 0.);
    let corner_two = Point3::new(5., 0., 0.);
    let corner_three = Point3::new(0., 0., 5.);
    let triangle =
        Triangle::new(corner_one, corner_two, corner_three, triangle_texture);
    let spheres = [
        Sphere::new_still(corner_one, 0.5, blue.clone()),
        Sphere::new_still(corner_two, 0.5, blue.clone()),
        Sphere::new_still(corner_three, 0.5, blue),
    ];
    let world = [triangle]
        .into_iter()
        .map(TriHit)
        .chain(spheres.into_iter().map(SpheHit))
        .collect::<HittableList>()
        .optimise();

    (
        world,
        Point3::new(0., 10., 0.000_000_1),
        Point3::new(0., 0., 0.),
        90.,
    )
}

#[allow(dead_code)]
fn tinted_glass() -> SceneInfo {
    let noise = PerlinTexture::new(1., Colour::new(0.4, 0.4, 0.4));
    let glass = Material::new_glass(1., Texture::Perlin(noise));
    let world = [
        // Sphere::new_still(Point3::new(0., -1000., 0.), 1000., ground_material),
        Sphere::new_still(Point3::new(0., 0., 0.), 1., glass),
    ];
    (
        world
            .into_iter()
            .map(SpheHit)
            .collect::<HittableList>()
            .optimise(),
        Point3::new(0., 10., 0.),
        Point3::new(0., 0., 0.),
        20.,
    )
}

#[allow(dead_code)]
fn basic_light() -> SceneInfo {
    let checkered_texture = Texture::Checker(CheckerTexture::new(
        Colour::new(0.2, 0.3, 0.1).to_texture(),
        Colour::new(0.9, 0.9, 0.9).to_texture(),
        0.32,
    ));

    let ground_material = Material::new_no_refract(0., checkered_texture);

    let glass = Material::new_glass(1.5, Colour::new(1., 1., 1.).to_texture());
    let air =
        Material::new_glass(1. / 1.5, Colour::new(1., 1., 1.).to_texture());
    let lamp =
        Material::new_no_refract(0., Colour::new(1., 1., 1.).to_texture());
    let smooth =
        Material::new_no_refract(1., Colour::new(1., 0.6, 0.5).to_texture());

    let world = vec![
        Sphere::new_still(Point3::new(0., -1000., -1.), 1000., ground_material),
        Sphere::new_still(Point3::new(0., 2., 1.), 1., glass),
        Sphere::new_still(Point3::new(0., 2., 1.), 0.5, air),
        Sphere::new_still(Point3::new(0., 1000., 0.), 900., lamp),
        Sphere::new_still(
            Point3::new(4., 1.0, 0.),
            1.,
            smooth,
        ),
    ];
    (
        world
            .iter()
            .map(|sphere| SpheHit(sphere.clone()))
            .collect::<HittableList>()
            .optimise(),
        Point3::new(13., 2., 3.),
        Point3::new(0., 0., 0.),
        30.,
    )
}

#[allow(dead_code)]
fn cornell_box() -> SceneInfo {
    let brightness = 1. / 2.;
    let light_size = 0.1;
    let white_texture = Colour::new(1., 1., 1.).to_texture();
    let glass = Material::new_glass(1.5, white_texture.clone());
    let white_walls = Material::new_no_refract(0.5, white_texture.clone());
    let white_light = Material::new_light(
        (Colour::new(1., 1., 1.) * brightness).to_texture(),
    );
    let red_walls =
        Material::new_no_refract(0.5, Colour::new(1., 0., 0.).to_texture());
    let green_walls =
        Material::new_no_refract(0.5, Colour::new(0., 1., 0.).to_texture());
    let back_walls = Material::new_no_refract(
        0.5,
        CheckerTexture::new(
            Colour::new(1., 0., 0.).to_texture(),
            Colour::new(0., 0., 1.).to_texture(),
            0.1,
        )
        .wrap(),
    );

    let (floor_one, floor_two) = Triangle::new_quad(
        (
            Point3::new(-1., -1., -1.),
            Point3::new(-1., -1., 1.),
            Point3::new(1., -1., -1.),
            Point3::new(1., -1., 1.),
        ),
        white_walls.clone(),
        None,
    );

    let (back_wall_one, back_wall_two) = Triangle::new_quad(
        (
            Point3::new(-1., -1., -1.),
            Point3::new(-1., 1., -1.),
            Point3::new(1., -1., -1.),
            Point3::new(1., 1., -1.),
        ),
        back_walls,
        None,
    );

    let (left_wall_one, left_wall_two) = Triangle::new_quad(
        (
            Point3::new(-1., -1., -1.),
            Point3::new(-1., -1., 1.),
            Point3::new(-1., 1., -1.),
            Point3::new(-1., 1., 1.),
        ),
        red_walls,
        None,
    );

    let (right_wall_one, right_wall_two) = Triangle::new_quad(
        (
            Point3::new(1., -1., -1.),
            Point3::new(1., -1., 1.),
            Point3::new(1., 1., -1.),
            Point3::new(1., 1., 1.),
        ),
        green_walls,
        None,
    );

    let (ceiling_one, ceiling_two) = Triangle::new_quad(
        (
            Point3::new(-1., 1., -1.),
            Point3::new(-1., 1., 1.),
            Point3::new(1., 1., -1.),
            Point3::new(1., 1., 1.),
        ),
        white_walls,
        None,
    );

    let (ceiling_light_one, ceiling_light_two) = Triangle::new_quad(
        (
            Point3::new(-light_size, 0.99, -light_size),
            Point3::new(-light_size, 0.99, light_size),
            Point3::new(light_size, 0.99, -light_size),
            Point3::new(light_size, 0.99, light_size),
        ),
        white_light,
        None,
    );

    let ball = Sphere::new_still(Point3::new(0., 0., 0.), 1. / 2., glass);

    let world = [
        TriHit(floor_one),
        TriHit(floor_two),
        TriHit(back_wall_one),
        TriHit(back_wall_two),
        TriHit(left_wall_one),
        TriHit(left_wall_two),
        TriHit(right_wall_one),
        TriHit(right_wall_two),
        TriHit(ceiling_one),
        TriHit(ceiling_two),
        TriHit(ceiling_light_one),
        TriHit(ceiling_light_two),
        SpheHit(ball),
    ];
    (
        world.into_iter().collect::<HittableList>().optimise(),
        Point3::new(0., 0., 2.),
        Point3::new(0., 0., 0.),
        90.,
    )
}
