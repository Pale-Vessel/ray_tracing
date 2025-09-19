use crate::{
    geometry::{ray::Ray, vector::Point3},
    hittables::{
        bounding_box::BoundingBox,
        hittable::{HitRecord, Hittable},
    },
    interval::Interval,
    textures::material::Material,
};

use glam::Vec3;

#[derive(Clone, Debug)]
pub struct Triangle {
    corner_one: Point3,
    corner_two: Point3,
    corner_three: Point3,
    material: Material,
    normal: Vec3,
    bounds: BoundingBox,
}

impl Hittable for Triangle {
    fn was_hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
        if !self.bounds.was_hit(ray, interval) {
            return None;
        }
        let (collision_time, u, v) =
            self.moller_trumbore_intersection(ray.origin, ray.direction)?;

        if !interval.surrounds(collision_time) {
            return None;
        }

        let collision_point = ray.at(collision_time);

        let front_face = self.normal.dot(ray.direction.normalize()) > 0.;

        Some(HitRecord::new(
            collision_point,
            self.normal,
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

impl Triangle {
    pub fn new(
        corner_one: Point3,
        corner_two: Point3,
        corner_three: Point3,
        material: Material,
    ) -> Self {
        // https://stackoverflow.com/a/23709352/23247074
        let a_vector = *(corner_two - corner_one);
        let b_vector = *(corner_three - corner_one);
        let normal = a_vector.cross(b_vector).normalize();

        let mut intervals = [Interval::default(); 3];
        for axis in 0..=2 {
            let axis_min = corner_one[axis]
                .min(corner_two[axis])
                .min(corner_three[axis]);
            let axis_max = corner_one[axis]
                .max(corner_two[axis])
                .max(corner_three[axis]);
            intervals[axis] = Interval::new(axis_min, axis_max);
        }
        let bounds = BoundingBox::new(intervals[0], intervals[1], intervals[2]);
        Self {
            corner_one,
            corner_two,
            corner_three,
            material,
            normal,
            bounds,
        }
    }

    //https://en.wikipedia.org/wiki/Möller-Trumbore_intersection_algorithm#Rust_implementation
    fn moller_trumbore_intersection(
        &self,
        origin: Point3,
        direction: Vec3,
    ) -> Option<(f32, f32, f32)> {
        let e1 = self.corner_two - self.corner_one;
        let e2 = self.corner_three - self.corner_one;

        let ray_cross_e2 = direction.cross(*e2);
        let det = e1.dot(ray_cross_e2);

        if det.abs() < f32::EPSILON {
            return None; // This ray is parallel to this self.
        }

        let inv_det = 1.0 / det;
        let s = origin - self.corner_one;
        let u = inv_det * s.dot(ray_cross_e2);
        if !(0.0..=1.0).contains(&u) {
            return None;
        }

        let s_cross_e1 = s.cross(*e1);
        let v = inv_det * direction.dot(s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        // At this stage we can compute t to find out where the intersection point is on the line.
        let collision_time = inv_det * e2.dot(s_cross_e1);

        if collision_time > f32::EPSILON {
            Some((collision_time, u, v))
        } else {
            // This means that there is a line intersection but not a ray intersection.
            None
        }
    }
}
