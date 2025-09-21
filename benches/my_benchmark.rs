#![allow(unused)]

use criterion::{Criterion, criterion_group, criterion_main};
use glam::Vec3;
use ray_tracing::{
    colour::Colour,
    geometry::{Point3, Ray},
    hittables::{hittable::Hittable, sphere::Sphere},
    interval::Interval,
    textures::material::Material,
};

    fn sphere_hit(n: f32) {
        let ray = Ray::new(Point3::new(-n, 0., 0.), Vec3::new(1., 0., 0.));
        let mat =
            Material::new_opaque(0., Colour::new(1., 1., 1.).to_texture());
        let sphere = Sphere::new(Point3::new(0., 0., 0.), 1.0, mat);
        sphere.was_hit(ray, Interval::new(f32::NEG_INFINITY, f32::INFINITY));
    }

    pub fn criterion_benchmark(c: &mut Criterion) {
        c.bench_function("sphere hit 20", |b| {
            b.iter(|| sphere_hit(std::hint::black_box(5.0)))
        });
    }

    criterion_group!(benches, criterion_benchmark);
    criterion_main!(benches);

