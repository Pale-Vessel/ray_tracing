use std::ops::Index;
use enum_dispatch::enum_dispatch;

use crate::{
    geometry::{ray::Ray, vector::Point3},
    hittables::{
        bounding_box::BoundingBox, sphere::Sphere, triangle::Triangle,
    },
    interval::Interval,
    textures::material::Material,
};

use glam::Vec3;

#[derive(Clone, Debug, Default)]
pub struct HitRecord {
    pub collision_point: Point3,
    pub normal_vector: Vec3,
    pub collision_time: f32,
    pub front_face: bool,
    pub material: Material,
    pub u: f32,
    pub v: f32,
}

impl HitRecord {
    pub fn new(
        collision_point: Point3,
        normal_vector: Vec3,
        collision_time: f32,
        front_face: bool,
        material: Material,
        u: f32,
        v: f32,
    ) -> HitRecord {
        HitRecord {
            collision_point,
            normal_vector: normal_vector.normalize(),
            collision_time,
            front_face,
            material,
            u,
            v,
        }
    }
    pub fn calc_front_face(ray: Ray, outward_normal: Vec3) -> (bool, Vec3) {
        let front_face = ray.direction.dot(outward_normal) < 0.;
        (
            front_face,
            if front_face {
                outward_normal
            } else {
                -outward_normal
            },
        )
    }
}

#[enum_dispatch]
pub trait Hittable {
    fn was_hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord>;
    fn get_bounding_box(&self) -> BoundingBox;
}

#[enum_dispatch(Hittable)]
#[derive(Clone, Debug)]
pub enum HittableObject {
    Sphere,
    Triangle
}

#[derive(Debug, Default)]
pub struct HittableList {
    pub data: Vec<HittableObject>,
    bounds: BoundingBox,
}

impl FromIterator<HittableObject> for HittableList {
    fn from_iter<T: IntoIterator<Item = HittableObject>>(iter: T) -> Self {
        let mut bounds = BoundingBox::default();
        let mut data = Vec::new();
        for item in iter {
            data.push(item.clone());
            bounds =
                BoundingBox::new_from_boxes(&bounds, &item.get_bounding_box());
        }
        Self { data, bounds }
    }
}

impl Index<usize> for HittableList {
    type Output = HittableObject;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl Hittable for HittableList {
    fn was_hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
        let mut closest = interval.max;
        let mut out_data = None;
        self.data.iter().for_each(|object| {
            if let Some(data) =
                object.was_hit(ray, Interval::new(interval.min, closest))
            {
                closest = data.collision_time;
                out_data = Some(data);
            }
        });
        out_data
    }

    fn get_bounding_box(&self) -> BoundingBox {
        self.bounds
    }
}
