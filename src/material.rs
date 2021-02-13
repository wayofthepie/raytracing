use crate::{
    ray::Ray,
    vec3::{dot, random_unit_vector, reflect, unit_vector, Vec3},
};
use rand::{distributions::Uniform, prelude::ThreadRng};

pub trait Material {
    fn scatter(
        &mut self,
        ray_in: &Ray,
        normal: Vec3,
        point: Vec3,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian<'rng> {
    albedo: Vec3,
    rng: &'rng mut ThreadRng,
    between: &'rng Uniform<f64>,
}

impl<'rng> Lambertian<'rng> {
    pub fn new(albedo: Vec3, rng: &'rng mut ThreadRng, between: &'rng Uniform<f64>) -> Self {
        Self {
            albedo,
            rng,
            between,
        }
    }
}

impl<'rng> Material for Lambertian<'rng> {
    fn scatter(
        &mut self,
        _: &Ray,
        normal: Vec3,
        point: Vec3,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = normal + random_unit_vector(self.rng, self.between);
        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }
        *scattered = Ray::new(point, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &mut self,
        ray_in: &Ray,
        normal: Vec3,
        point: Vec3,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit_vector(ray_in.direction), normal);
        *scattered = Ray::new(point, reflected);
        *attenuation = self.albedo;
        dot(scattered.direction, normal) > 0.0
    }
}
