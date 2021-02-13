use rand::{distributions::Uniform, prelude::ThreadRng};

use crate::vec3::{unit_vector, Vec3};
use crate::{
    sphere::{Hit, HitRecord, Hittables},
    vec3::random_in_unit_sphere,
};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + self.direction * t
    }
}

pub fn ray_color<T>(
    ray: &Ray,
    world: &Hittables<T>,
    depth: u16,
    rng: &mut ThreadRng,
    between: &Uniform<f64>,
) -> Vec3
where
    T: Hit,
{
    let mut record = HitRecord::default();
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if world.hit(ray, 0.0, f64::INFINITY, &mut record) {
        let target = record.point + record.normal + random_in_unit_sphere(rng, between);
        let ray = Ray::new(record.point, target - record.point);
        return 0.5 * ray_color(&ray, world, depth - 1, rng, between);
    }
    let unit_direction = unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
