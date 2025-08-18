use core::f64::consts::{PI, TAU};

use derive_more::Constructor;

use crate::{
    bounding_box::BoundingBox,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vector::{Point3, Vec3},
};

#[derive(Clone, Debug, Constructor)]
pub struct Sphere {
    center: Ray,
    radius: f64,
    material: Material,
    bounds: BoundingBox,
}

impl Hittable for Sphere {
    fn did_hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = *(current_center - ray.origin);
        // a, b, and c are coefficients in the derived quadratic equation
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0. {
            return None;
        }
        let discriminant_root = discriminant.sqrt();
        //println!("{ray:?} {discriminant_root}\n");
        let mut collision_time = (h - discriminant_root) / a;
        if !interval.surrounds(collision_time) {
            collision_time = (h + discriminant_root) / a;
            if !interval.surrounds(collision_time) {
                return None;
            }
        }

        let collision_point = ray.at(collision_time);
        let outward_normal = *(collision_point - current_center) / self.radius;
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
    pub fn new_still(center: Point3, radius: f64, material: Material) -> Self {
        let radius_vector = Vec3::new(radius, radius, radius);
        let bounds = BoundingBox::new_from_corners(
            Point3::from_vector(*center - radius_vector),
            Point3::from_vector(*center + radius_vector),
        );
        Self::new(
            Ray::new_time_zero(center, Vec3::new(0., 0., 0.)),
            radius,
            material,
            bounds,
        )
    }

    #[allow(dead_code)]
    pub fn new_from_two_points(
        start_center: Point3,
        end_center: Point3,
        radius: f64,
        material: Material,
    ) -> Self {
        let movement =
            Ray::new_time_zero(start_center, *(end_center - start_center));
        let radius_vector = Vec3::new(radius, radius, radius);
        let box_one = BoundingBox::new_from_corners(
            Point3::from_vector(*start_center - radius_vector),
            Point3::from_vector(*start_center + radius_vector),
        );
        let box_two = BoundingBox::new_from_corners(
            Point3::from_vector(*end_center - radius_vector),
            Point3::from_vector(*end_center + radius_vector),
        );
        let bounds = BoundingBox::new_from_boxes(&box_one, &box_two);
        Self::new(movement, radius, material, bounds)
    }

    pub fn get_uv(&self, point: Point3) -> (f64, f64) {
        let vector = (self.center.origin - point).unit();

        (
            self.radius * (0.5 + (vector.x.atan2(vector.z) / TAU)),
            self.radius * (0.5 + vector.y.asin() / PI),
        )
    }
}
