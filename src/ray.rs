use crate::{
    hit::{Hit, Hittables},
    vec3::{unit_vector, Vec3},
};


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

pub fn ray_color<'material, T>(ray: &Ray, world: &Hittables<T>, depth: u16) -> Vec3
where
    T: Hit<'material>,
{
    if depth == 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }
    if let Some(record) = world.hit(ray, 0.001, f64::INFINITY) {
        let mut scattered = Ray::default();
        let mut attenuation = Vec3::default();
        let maybe_attenuation = {
            let material_ref = &mut *record.material.borrow_mut();
            material_ref
                .scatter(ray, &record, &mut attenuation, &mut scattered)
                .then(|| attenuation)
        };
        if let Some(attenuation) = maybe_attenuation {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Vec3::default()
        }
    } else {
        let unit_direction = unit_vector(ray.direction);
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
    }
}
