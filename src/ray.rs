use crate::{
    hit::{Hit, Hittables},
    vec3::{unit_vector, Vec3},
};
use rand::{distributions::Uniform, prelude::ThreadRng};

#[derive(Default, Debug)]
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

pub fn ray_color<'material, T>(
    ray: &Ray,
    world: &Hittables<T>,
    depth: u16,
    rng: &mut ThreadRng,
    between: &Uniform<f64>,
) -> Vec3
where
    T: Hit<'material>,
{
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(mut record) = world.hit(ray, 0.001, f64::INFINITY) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::default();
        let point = record.point;
        let normal = record.normal;
        if let Some(ref mut material) = record.material {
            let maybe_attenuation = {
                let material_ref = &mut *material.borrow_mut();
                if material_ref.scatter(ray, normal, point, &mut attenuation, &mut scattered) {
                    Some(attenuation)
                } else {
                    None
                }
            };
            if let Some(attenuation) = maybe_attenuation {
                return attenuation * ray_color(&scattered, world, depth - 1, rng, between);
            }
        }
        return Vec3::default();
    }
    let unit_direction = unit_vector(ray.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}
