use crate::{
    geometry::{ray::Ray, vector::Point3},
    hittables::bounding_box::BoundingBox,
    interval::Interval,
};
use core::f32::consts::{PI, TAU};

use glam::Vec3;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub bounds: BoundingBox,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32) -> Self {
        let radius_vector = Vec3::new(radius, radius, radius);
        let bounds = BoundingBox::new_from_corners(
            Point3::from_vector(*center - radius_vector),
            Point3::from_vector(*center + radius_vector),
        );
        Self {
            center,
            radius,
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
