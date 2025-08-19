use std::ops::Index;

use derive_more::Constructor;

use crate::{
    bounding_box::BoundingBox, bvh::BVHNode, interval::Interval, material::Material, ray::Ray, sphere::Sphere, triangle::Triangle, vector::{Point3, Vec3}
};

#[derive(Clone, Debug, Default, Constructor)]
pub struct HitRecord {
    pub collision_point: Point3,
    pub normal_vector: Vec3,
    pub collision_time: f64,
    pub front_face: bool,
    pub material: Material,
    pub u: f64,
    pub v: f64,
}

impl HitRecord {
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

pub trait Hittable {
    fn did_hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord>;
    fn get_bounding_box(&self) -> BoundingBox;
}

#[derive(Clone, Debug)]
pub enum HittableObject {
    Sphere(Sphere),
    BVHNode(BVHNode),
    Triangle(Triangle)
}

impl Hittable for HittableObject {
    fn did_hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
        match self {
            HittableObject::Sphere(sphere) => sphere.did_hit(ray, interval),
            HittableObject::BVHNode(bvhnode) => bvhnode.did_hit(ray, interval),
            HittableObject::Triangle(triangle) => triangle.did_hit(ray, interval)
        }
    }

    fn get_bounding_box(&self) -> BoundingBox {
        match self {
            HittableObject::Sphere(sphere) => sphere.get_bounding_box(),
            HittableObject::BVHNode(bvhnode) => bvhnode.get_bounding_box(),
            HittableObject::Triangle(triangle) => triangle.get_bounding_box(),
        }
    }
}

#[derive(Debug)]
pub struct HittableList {
    pub data: Vec<HittableObject>,
    bounds: BoundingBox,
}

impl FromIterator<HittableObject> for HittableList {
    fn from_iter<T: IntoIterator<Item = HittableObject>>(iter: T) -> Self {
        let mut bounds = BoundingBox::default();
        let mut data = Vec::new();
        for item in iter.into_iter() {
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
    fn did_hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
        let mut closest = interval.max;
        let mut out_data = None;
        self.data.iter().for_each(|object| {
            if let Some(data) =
                object.did_hit(ray, Interval::new(interval.min, closest))
            {
                closest = data.collision_time;
                out_data = Some(data)
            }
        });
        out_data
    }

    fn get_bounding_box(&self) -> BoundingBox {
        self.bounds
    }
}

impl HittableList {
    pub fn optimise(self) -> Self {
        vec![HittableObject::BVHNode(BVHNode::new(self))]
            .into_iter()
            .collect()
    }
}
