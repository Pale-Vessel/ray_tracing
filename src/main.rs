#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

mod camera;
mod colour;
mod geometry;
mod hittables;
mod interval;
mod textures;

use core::f32;
use derive_more::Display;
use image::ImageResult;
use std::path::Path;

#[allow(unused_imports)]
use rand::{Rng, rng};

use crate::{
    camera::Camera,
    colour::Colour,
    geometry::vector::Point3,
    hittables::{
        hittable::{
            HittableList,
            HittableObject::{self, Sphere as SpheHit, Triangle as TriHit},
        },
        sphere::Sphere,
        triangle::Triangle,
    },
    textures::{
        checker_texture::CheckerTexture, material::Material,
        perlin_texture::PerlinTexture, texture::Texture,
    },
};

use glam::Vec3;

type SceneInfo = (HittableList, Point3, Point3, f32);

#[allow(dead_code)]
#[derive(Display)]
enum Profile {
    Debug,
    Release,
    Insane,
    OvernightRender,
    ManyBounces,
}

fn main() -> ImageResult<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let profile = match args[1].to_ascii_lowercase().as_str() {
        "debug" => Profile::Debug,
        "release" => Profile::Release,
        "insane" => Profile::Insane,
        "overnight" => Profile::OvernightRender,
        "bounce" => Profile::ManyBounces,
        _ => panic!("Invalid profile"),
    };
    let (image_width, rays_per_pixel, max_ray_bounces) = match profile {
        Profile::Debug => (800, 10, 10),
        Profile::Release => (800, 100, 10),
        Profile::Insane => (800, 1_000, 10),
        Profile::OvernightRender => (1920, 5_000, 10),
        Profile::ManyBounces => (800, 100, 50),
    };
    let (world, look_from, look_at, fov) =
        match args[2].to_ascii_lowercase().as_str() {
            "spheres" => basic_spheres(),
            "big_scene" => many_spheres(),
            "checks" => checkered_spheres(),
            "perlin_sphere" => perlin_spheres(),
            "triangle" => triangle(),
            "tinted_glass" => tinted_glass(),
            "basic_light" => basic_light(),
            "cornell" => cornell_box(),
            "perlin_tri" => perlin_triangle(),
            "glass_box" => glass_box(),
            "glass_square" => glass_square(),
            "empty" => empty_scene(),
            _ => panic!("Invalid scene"),
        };
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
    let path = format!("image{profile}.png");
    let output = Path::new(&path);
    image.save(output)
}

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
        Point3::ORIGIN,
        20.,
    )
}

fn many_spheres() -> SceneInfo {
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
        Point3::ORIGIN,
        20.,
    )
}

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

fn triangle() -> SceneInfo {
    #[allow(unused_variables)]
    let blue: Material =
        Material::new_light(1., Colour::new(0., 0., 1.).to_texture());
    let triangle_texture = Material::new_no_refract(
        1.,
        PerlinTexture::new(1., Colour::new(1., 1., 1.)).wrap(),
    );
    let corner_one = Point3::new(-10., 0., -10.);
    let corner_two = Point3::new(40., 0., -10.);
    let corner_three = Point3::new(-10., 0., 40.);
    let triangle =
        Triangle::new(corner_one, corner_two, corner_three, triangle_texture);
    // let spheres = [
    //     Sphere::new_still(corner_one, 0.5, blue.clone()),
    //     Sphere::new_still(corner_two, 0.5, blue.clone()),
    //     Sphere::new_still(corner_three, 0.5, blue),
    // ];
    let world = [triangle]
        .into_iter()
        .map(TriHit)
        //.chain(spheres.into_iter().map(SpheHit))
        .collect::<HittableList>()
        .optimise();

    (
        world,
        Point3::new(0., 10., 0.000_000_1),
        Point3::ORIGIN,
        90.,
    )
}

fn tinted_glass() -> SceneInfo {
    let ground_material =
        Material::new_no_refract(0., Colour::new(0.2, 0.3, 0.8).to_texture());
    let glass = Material::new_glass(1.5, Colour::new(1., 0., 0.).to_texture());
    let world = [
        Sphere::new_still(Point3::new(0., -1000., 0.), 1000., ground_material),
        Sphere::new_still(Point3::ORIGIN, 1., glass),
    ];
    (
        world
            .into_iter()
            .map(SpheHit)
            .collect::<HittableList>()
            .optimise(),
        Point3::new(0., 3., 2.),
        Point3::ORIGIN,
        90.,
    )
}

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
        Sphere::new_still(Point3::new(4., 1.0, 0.), 1., smooth),
    ];
    (
        world
            .iter()
            .map(|sphere| SpheHit(sphere.clone()))
            .collect::<HittableList>()
            .optimise(),
        Point3::new(13., 2., 3.),
        Point3::ORIGIN,
        30.,
    )
}

fn cornell_box() -> SceneInfo {
    let white_texture = Colour::new(1., 1., 1.).to_texture();
    let glass = Material::new_glass(1.5, white_texture.clone());
    // let ball_one = Sphere::new_still(Point3::ORIGIN, 1. / 3., glass.clone());
    let world = make_cube(1.5, false, false, false, Some(glass), None);
    // world.push(SpheHit(ball_one));

    (
        world.into_iter().collect::<HittableList>().optimise(),
        Point3::new(0., 0., 1.5),
        Point3::ORIGIN,
        90.,
    )
}

fn perlin_triangle() -> SceneInfo {
    let tri_size = 5.;
    let material = Material::new_no_refract(
        0.,
        PerlinTexture::new(0.5, Colour::new(1., 1., 1.)).wrap(),
    );
    let triangle = Triangle::new(
        Point3::new(tri_size, 0., tri_size),
        Point3::new(tri_size, 0., -tri_size),
        Point3::new(-tri_size, 0., -tri_size),
        material,
    );
    let world = [TriHit(triangle)]
        .into_iter()
        .collect::<HittableList>()
        .optimise();
    (world, Point3::new(0., 6., 0.), Point3::default(), 90.)
}

fn glass_box() -> SceneInfo {
    let walls =
        Material::new_no_refract(1., Colour::new(1., 0.3, 0.3).to_texture());
    let world = make_cube(3., true, true, true, Some(walls), None);
    let glass = Material::new_glass(1., Colour::new(1., 1., 1.).to_texture());
    let small_cube = make_cube(0.75, true, false, false, Some(glass), None);
    (
        world
            .into_iter()
            // .chain(small_cube)
            .collect::<HittableList>()
            .optimise(),
        Point3::new(0., 0., 2.5),
        Point3::ORIGIN,
        90.,
    )
}

fn glass_square() -> SceneInfo {
    let size = 3.;
    let glass = Material::new_glass(1.5, Colour::WHITE.to_texture());
    let (square_one, square_two) = Triangle::new_quad(
        (
            Point3::new(0., size, size),
            Point3::new(0., size, -size),
            Point3::new(0., -size, size),
            Point3::new(0., -size, -size),
        ),
        glass,
        None,
    );
    (
        [TriHit(square_one), TriHit(square_two)]
            .into_iter()
            .collect::<HittableList>()
            .optimise(),
        Point3::new(1.5, 0., 0.),
        Point3::ORIGIN,
        90.,
    )
}

fn empty_scene() -> SceneInfo {
    let world = Default::default();
    (world, Point3::new(1., 0., 0.), Point3::default(), 90.)
}

fn make_cube(
    size: f32,
    closed: bool,
    ceiling_light: bool,
    floor_light: bool,
    material: Option<Material>,
    light_size: Option<f32>,
) -> Vec<HittableObject> {
    let material = material.unwrap_or(Material::new_no_refract(
        0.8,
        Colour::new(0.8, 0.8, 0.8).to_texture(),
    ));
    let brightness = 1.;
    let light_size = light_size.unwrap_or(size * 0.8);
    let white_light = Material::new_light(
        1.,
        (Colour::new(1., 1., 1.) * brightness).to_texture(),
    );

    let (floor_one, floor_two) = Triangle::new_quad(
        (
            Point3::new(-size, -size, -size),
            Point3::new(-size, -size, size),
            Point3::new(size, -size, -size),
            Point3::new(size, -size, size),
        ),
        material.clone(),
        None,
    );

    let (back_wall_one, back_wall_two) = Triangle::new_quad(
        (
            Point3::new(-size, -size, -size),
            Point3::new(-size, size, -size),
            Point3::new(size, -size, -size),
            Point3::new(size, size, -size),
        ),
        material.clone(),
        None,
    );

    let (left_wall_one, left_wall_two) = Triangle::new_quad(
        (
            Point3::new(-size, -size, -size),
            Point3::new(-size, -size, size),
            Point3::new(-size, size, -size),
            Point3::new(-size, size, size),
        ),
        material.clone(),
        None,
    );

    let (right_wall_one, right_wall_two) = Triangle::new_quad(
        (
            Point3::new(size, -size, -size),
            Point3::new(size, -size, size),
            Point3::new(size, size, -size),
            Point3::new(size, size, size),
        ),
        material.clone(),
        None,
    );

    let (front_wall_one, front_wall_two) = Triangle::new_quad(
        (
            Point3::new(-size, -size, size),
            Point3::new(-size, size, size),
            Point3::new(size, -size, size),
            Point3::new(size, size, size),
        ),
        material.clone(),
        None,
    );

    let (ceiling_one, ceiling_two) = Triangle::new_quad(
        (
            Point3::new(-size, size, -size),
            Point3::new(-size, size, size),
            Point3::new(size, size, -size),
            Point3::new(size, size, size),
        ),
        material.clone(),
        None,
    );

    let (ceiling_light_one, ceiling_light_two) = Triangle::new_quad(
        (
            Point3::new(-light_size, size - 1e-8, -light_size),
            Point3::new(-light_size, size - 1e-8, light_size),
            Point3::new(light_size, size - 1e-8, -light_size),
            Point3::new(light_size, size - 1e-8, light_size),
        ),
        white_light.clone(),
        None,
    );

    let (floor_light_one, floor_light_two) = Triangle::new_quad(
        (
            Point3::new(-light_size, -size + 1e-8, -light_size),
            Point3::new(-light_size, -size + 1e-8, light_size),
            Point3::new(light_size, -size + 1e-8, -light_size),
            Point3::new(light_size, -size + 1e-8, light_size),
        ),
        white_light.clone(),
        None,
    );

    vec![
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
        TriHit(ceiling_light_one).exist_if(ceiling_light),
        TriHit(ceiling_light_two).exist_if(ceiling_light),
        TriHit(front_wall_one).exist_if(closed),
        TriHit(front_wall_two).exist_if(closed),
        TriHit(floor_light_one).exist_if(floor_light),
        TriHit(floor_light_two).exist_if(floor_light),
    ]
}
