use std::f32::consts::{PI, TAU};

use glam::Vec3;

use crate::{
    geometry::{ray::Ray, vector::Point3},
    hittables::{
        bounding_box::BoundingBox,
        hittable::{HitRecord, Hittable, HittableObject},
    },
    interval::Interval,
    textures::material::Material,
};

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub bounds: BoundingBox,
    material: Material,
}

impl Hittable for Sphere {
    fn was_hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
        if !self.bounds.was_hit(ray, interval) {
            return None
        }
        let collision_times = self.ray_intersections(ray, interval);
        let collision_time = collision_times.0.or(collision_times.1)?;

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
            bounds,
            material,
        }
    }

    pub fn wrap(self) -> HittableObject {
        HittableObject::Sphere(self)
    }

    pub fn get_uv(&self, point: Point3) -> (f32, f32) {
        let vector = (self.center - point).normalize();

        (
            (0.5 + (vector.x.atan2(vector.z) / TAU)),
            (0.5 + vector.y.asin() / PI),
        )
    }

    pub fn ray_intersections(
        &self,
        ray: Ray,
        interval: Interval,
    ) -> (Option<f32>, Option<f32>) {
        let oc = *(self.center - ray.origin);
        // a, b, and c are coefficients in the derived quadratic equation
        let a_coefficient = ray.direction.length_squared();
        let h_coefficient = ray.direction.dot(oc);
        let c_coefficient = oc.length_squared() - self.radius * self.radius;
        let discriminant =
            h_coefficient * h_coefficient - a_coefficient * c_coefficient;
        if discriminant < 0. {
            return (None, None);
        }
        let discriminant_root = discriminant.sqrt();
        let first_collision =
            (h_coefficient - discriminant_root) / a_coefficient;
        let second_collision =
            (h_coefficient + discriminant_root) / a_coefficient;

        (
            interval
                .surrounds(first_collision)
                .then_some(first_collision),
            interval
                .surrounds(second_collision)
                .then_some(second_collision),
        )
    }
}
