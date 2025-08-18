mod bounding_box;
mod bvh;
mod camera;
mod colour;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod texture;
mod vector;

use core::f64;
use std::{
    fs::File,
    io::{Result, Write},
};

#[allow(unused_imports)]
use rand::{Rng, rng};

use crate::{
    camera::Camera,
    colour::Colour,
    hittable::{HittableList, HittableObject},
    material::Material,
    sphere::Sphere,
    texture::{CheckerTexture, PerlinTexture, Texture},
    vector::{Point3, Vec3},
};

type SceneInfo = (HittableList, Point3, Point3, f64);

#[allow(dead_code)]
enum Profile {
    Debug,
    Release,
    Insane,
}

const PROFILE: Profile = Profile::Debug;

fn main() -> Result<()> {
    let (image_width, rays_per_pixel, max_ray_bounces) = match PROFILE {
        Profile::Debug => (800, 10, 10),
        Profile::Release => (800, 100, 50),
        Profile::Insane => (1920, 500, 50),
    };
    let (world, look_from, look_at, fov) = perlin_spheres();
    let camera = Camera::initialise(
        image_width,
        rays_per_pixel,
        max_ray_bounces,
        fov,
        look_from,
        look_at,
        Vec3::new(0., 1., 0.0000001),
        10.,
        0.,
    );
    let image = camera.render(world);
    let mut output = File::create("image.ppm")?;
    write!(output, "{}", image)
}

#[allow(dead_code)]
fn checkered_spheres() -> SceneInfo {
    let checkered_texture = Texture::Checker(CheckerTexture::new(
        Colour::new(0.5, 0.5, 0.5).to_texture(),
        Colour::new(0., 0.3, 0.7).to_texture(),
        4.,
    ));

    let material = Material::new_no_refract(1., checkered_texture);

    let world = vec![
        Sphere::new_still(Point3::new(0., -10., 0.), 10., material.clone()),
        Sphere::new_still(Point3::new(0., 10., 0.), 10., material),
    ];

    (
        world
            .iter()
            .map(|sphere| HittableObject::Sphere(sphere.clone()))
            .collect::<HittableList>()
            .optimise(),
        Point3::new(13., 2., 3.),
        Point3::new(0., 1., 0.),
        50.,
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
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let colour = Colour::rand_unit_vector().to_texture();
                let material = Material::new_no_refract(rng.random(), colour);
                world.push(Sphere::new_still(center, 0.2, material));
            }
        }
        for b in 3..5 {
            let center = Point3::new(
                a as f64 + 0.9 * rng.random::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.random::<f64>(),
            );

            if (center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let colour = Colour::rand_unit_vector().to_texture();
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
            .map(|sphere| HittableObject::Sphere(sphere.clone()))
            .collect::<HittableList>()
            .optimise(),
        Point3::new(13., 2., 3.),
        Point3::new(0., 0., 0.),
        20.,
    )
}

fn perlin_spheres() -> SceneInfo {
    let perlin_texture =
        Texture::Perlin(PerlinTexture::new(1., Colour::new(1., 1., 1.)));
    let material = Material::new_no_refract(0., perlin_texture);
    let red =
        Material::new_no_refract(0., Colour::new(1., 0., 0.).to_texture());
        let blue =
        Material::new_no_refract(0., Colour::new(0., 0., 1.).to_texture());
    let world = [
        Sphere::new_still(Point3::new(0., -1000., 0.), 1000., material.clone()),
        Sphere::new_still(Point3::new(10., 0., 0.), 0.5, red),
        Sphere::new_still(Point3::new(0., 0., 10.), 0.5, blue), // Sphere::new_still(Point3::new(0., 2., 0.), 2., material),
    ];

    (
        world
            .iter()
            .map(|sphere| HittableObject::Sphere(sphere.clone()))
            .collect::<HittableList>()
            .optimise(),
        Point3::new(0., 15., 0.),
        Point3::new(0., 0., 0.),
        90.,
    )
}
