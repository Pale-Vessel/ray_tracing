use std::ops::Index;

use derive_more::Constructor;

use crate::{
    geometry::{Point3, Ray},
    interval::Interval,
};

#[derive(Debug, Copy, Clone, Constructor, Default)]
pub struct AxisAlignedBoundingBox {
    x: Interval,
    y: Interval,
    z: Interval,
}

pub type BoundingBox = AxisAlignedBoundingBox;

impl Index<usize> for AxisAlignedBoundingBox {
    type Output = Interval;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => unreachable!(),
        }
    }
}

impl AxisAlignedBoundingBox {
    pub fn new_from_corners(corner_one: Point3, corner_two: Point3) -> Self {
        let x = if corner_one.x < corner_two.x {
            Interval::new(corner_one.x, corner_two.x)
        } else {
            Interval::new(corner_two.x, corner_one.x)
        };
        let y = if corner_one.y < corner_two.y {
            Interval::new(corner_one.y, corner_two.y)
        } else {
            Interval::new(corner_two.y, corner_one.y)
        };
        let z = if corner_one.z < corner_two.z {
            Interval::new(corner_one.z, corner_two.z)
        } else {
            Interval::new(corner_two.z, corner_one.z)
        };
        Self::new(x, y, z)
    }

    pub fn new_from_boxes(box_one: &Self, box_two: &Self) -> Self {
        let x = box_one.x.enclose(box_two.x);
        let y = box_one.y.enclose(box_two.y);
        let z = box_one.z.enclose(box_two.z);
        Self::new(x, y, z)
    }

    pub fn was_hit(&self, ray: Ray, interval: Interval) -> bool {
        let origin = ray.origin;
        let direction = ray.direction;

        for axis in 0..3 {
            let axis_interval = self[axis];
            let ray_dir_inv = 1. / direction[axis];

            let time_zero = (axis_interval.min - origin[axis]) * ray_dir_inv;
            let time_one = (axis_interval.max - origin[axis]) * ray_dir_inv;

            let (time_one, time_zero) = if time_one > time_zero {
                (time_one, time_zero)
            } else {
                (time_zero, time_one)
            };

            let minimum = f32::max(time_zero, interval.min);
            let maximum = f32::min(time_one, interval.max);
            if maximum < minimum {
                return false;
            }
        }
        true
    }
}
