use crate::{
    ray::Ray,
    vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, unit_vector, Vec3},
};

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

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &mut self,
        _: &Ray,
        normal: Vec3,
        point: Vec3,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = normal + random_unit_vector();
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
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
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
        *scattered = Ray::new(point, reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo;
        dot(scattered.direction, normal) > 0.0
    }
}
