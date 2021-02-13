use crate::sphere::{Hit, HitRecord, Hittables};
use crate::vec3::{unit_vector, Vec3};

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
