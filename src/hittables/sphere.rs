use crate::{
    geometry::{ray::Ray, vector::Point3},
    hittables::{
        bounding_box::BoundingBox,
        hittable::{HitRecord, Hittable, HittableObject},
    },
    interval::Interval,
    textures::material::Material,
};
use core::f32::consts::{PI, TAU};

use glam::Vec3;

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Material,
    bounds: BoundingBox,
}

impl Hittable for Sphere {
    fn did_hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
        let oc = *(self.center - ray.origin);
        // a, b, and c are coefficients in the derived quadratic equation
        let a_coefficient = ray.direction.length_squared();
        let h_coefficient = ray.direction.dot(oc);
        let c_coefficient = oc.length_squared() - self.radius * self.radius;
        let discriminant =
            h_coefficient * h_coefficient - a_coefficient * c_coefficient;
        if discriminant < 0. {
            return None;
        }
        let discriminant_root = discriminant.sqrt();
        let mut collision_time =
            (h_coefficient - discriminant_root) / a_coefficient;
        if !interval.surrounds(collision_time) {
            collision_time =
                (h_coefficient + discriminant_root) / a_coefficient;
            if !interval.surrounds(collision_time) {
                return None;
            }
        }

        let collision_point = ray.at(collision_time);
        let outward_normal = *(collision_point - self.center) / self.radius;
        let (front_face, normal_vector) =
            HitRecord::calc_front_face(ray, outward_normal);
        let (u, v) = self.get_uv(collision_point);
        Some(HitRecord::new(
            collision_point,
            normal_vector,
            collision_time,
            front_face,
            self.material.clone(),
            u,
            v,
        ))
    }

    fn get_bounding_box(&self) -> BoundingBox {
        self.bounds
    }
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Material) -> Self {
        let radius_vector = Vec3::new(radius, radius, radius);
        let bounds = BoundingBox::new_from_corners(
            Point3::from_vector(*center - radius_vector),
            Point3::from_vector(*center + radius_vector),
        );
        Self {
            center,
            radius,
            material,
            bounds,
        }
    }

    pub fn get_uv(&self, point: Point3) -> (f32, f32) {
        let vector = (self.center - point).normalize();

        (
            self.radius * (0.5 + (vector.x.atan2(vector.z) / TAU)),
            self.radius * (0.5 + vector.y.asin() / PI),
        )
    }

    pub fn wrap(self) -> HittableObject {
        HittableObject::Sphere(self)
    }
}
