use std::cmp::Ordering;

use crate::{
    geometry::ray::Ray,
    hittables::{
        bounding_box::BoundingBox,
        hittable::{HitRecord, Hittable, HittableList, HittableObject},
    },
    interval::Interval,
};

#[derive(Clone, Debug)]
pub struct BVHNode {
    left: Box<HittableObject>,
    right: Box<HittableObject>,
    bounds: BoundingBox,
}

impl Hittable for BVHNode {
    fn did_hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
        if !self.bounds.did_hit(ray, interval) {
            return None;
        }
        let hit_left = self.left.did_hit(ray, interval);
        let hit_right = self.right.did_hit(
            ray,
            Interval::new(
                interval.min,
                if let Some(data) = hit_left.clone() {
                    data.collision_time
                } else {
                    interval.max
                },
            ),
        );
        if hit_right.is_some() {
            hit_right
        } else {
            hit_left
        }
    }

    fn get_bounding_box(&self) -> BoundingBox {
        self.bounds
    }
}

impl BVHNode {
    pub fn new(objects: HittableList) -> Self {
        let mut bounds = BoundingBox::EMPTY;
        for object in &objects.data {
            bounds.grow_to_box(&object.get_bounding_box());
        }

        let comparator = bounds.longest_axis();

        let [left, right] = match objects.data.len() {
            1 => [objects[0].clone(), objects[0].clone()],
            2 => [objects[0].clone(), objects[1].clone()],
            _ => {
                let mut data = objects.data;
                data.sort_by(|a, b| Self::box_compare(a, b, comparator));
                let mid = data.len() / 2;
                let left = BVHNode::new(data[..mid].iter().cloned().collect());
                let right = BVHNode::new(data[mid..].iter().cloned().collect());
                [
                    HittableObject::BVHNode(left),
                    HittableObject::BVHNode(right),
                ]
            }
        }
        .map(Box::new);

        Self {
            left,
            right,
            bounds,
        }
    }

    fn box_compare(
        a: &HittableObject,
        b: &HittableObject,
        axis_index: usize,
    ) -> Ordering {
        let a_axis_interval = a.get_bounding_box()[axis_index];
        let b_axis_interval = b.get_bounding_box()[axis_index];
        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }
}
