use crate::{
    bounding_box::BoundingBox,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vector::{Point3, Vec3},
};

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
    fn did_hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
        // https://en.wikipedia.org/wiki/Möller-Trumbore_intersection_algorithm#Rust_implementation
        let e1 = self.corner_two - self.corner_one;
        let e2 = self.corner_three - self.corner_one;

        let ray_cross_e2 = ray.direction.cross(*e2);
        let det = e1.dot(ray_cross_e2);

        if det.abs() < f64::EPSILON {
            return None;
        }

        let inv_det = 1. / det;
        let s = ray.origin - self.corner_one;
        let u = inv_det * s.dot(ray_cross_e2);
        if !(0. ..=1.).contains(&u) {
            return None;
        }

        let s_cross_e1 = s.cross(*e1);
        let v = inv_det * ray.direction.dot(s_cross_e1);
        if !(0. ..=1.).contains(&v) {
            return None;
        }

        let collision_time = inv_det * e2.dot(s_cross_e1);

        if !interval.surrounds(collision_time) {
            return None;
        }

        let collision_point = ray.at(collision_time);

        // TODO: Calculate front face properly
        let front_face = true;

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
        let normal = a_vector.cross(b_vector);

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
}
