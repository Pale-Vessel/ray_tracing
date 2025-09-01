use crate::{
    geometry::{ray::Ray, vector::Point3},
    hittables::{
        bounding_box::BoundingBox,
        hittable::{HitRecord, Hittable, HittableObject},
        sphere::Sphere,
    },
    interval::Interval,
    textures::material::Material,
};

use derive_more::Deref;

#[derive(Clone, Debug, Deref)]
pub struct SolidSphere {
    #[deref]
    collision: Sphere,
    material: Material,
}

impl Hittable for SolidSphere {
    fn did_hit(&self, ray: Ray, interval: Interval) -> Option<HitRecord> {
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

impl SolidSphere {
    pub fn new(center: Point3, radius: f32, material: Material) -> Self {
        let collision = Sphere::new(center, radius);
        Self {
            collision,
            material,
        }
    }

    pub fn wrap(self) -> HittableObject {
        HittableObject::Sphere(self)
    }
}
