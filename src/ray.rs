use std::f32::INFINITY;

use crate::{
    sphere::{Hit, HitRecord, Hittables},
    vec3::{dot, unit_vector, Vec3},
};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub fn ray_color<T>(ray: &Ray, world: &Hittables<T>) -> Vec3
where
    T: Hit,
{
    let mut record = HitRecord::default();
    if world.hit(ray, 0.0, f32::INFINITY, &mut record) {
        return (record.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
    }
    let unit_direction = unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}

pub fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> f32 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = dot(oc, ray.direction);
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}
