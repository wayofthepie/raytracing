use rand::Rng;

use crate::{
    hit::HitRecord,
    ray::Ray,
    vec3::{dot, random_in_unit_sphere, random_unit_vector, reflect, unit_vector, Vec3},
};

pub trait Material {
    fn scatter(
        &mut self,
        ray_in: &Ray,
        record: &HitRecord,
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
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = record.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = record.normal;
        }
        *scattered = Ray::new(record.point, scatter_direction);
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
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(unit_vector(ray_in.direction), record.normal);
        *scattered = Ray::new(
            record.point,
            reflected + self.fuzz * random_in_unit_sphere(),
        );
        *attenuation = self.albedo;
        dot(scattered.direction, record.normal) > 0.0
    }
}

pub struct Dialectric {
    refraction_index: f64,
}

impl Dialectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Schlicks approximation
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0.powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dialectric {
    fn scatter(
        &mut self,
        ray_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        // TODO prob not great to get this each call
        let mut rng = rand::thread_rng();
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if record.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = unit_vector(ray_in.direction);
        let cos_theta = f64::min(dot(-unit_direction, record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dialectric::reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0)
        {
            reflect(unit_direction, record.normal)
        } else {
            refract(unit_direction, record.normal, refraction_ratio)
        };
        *scattered = Ray::new(record.point, direction);
        true
    }
}

pub fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
    let cons_theta = f64::min(dot(-uv, normal), 1.0);
    let ray_out_perpendicular = etai_over_etat * (uv + cons_theta * normal);
    let ray_out_parallel = -(1.0 - ray_out_perpendicular.length_squared()).abs().sqrt() * normal;
    ray_out_perpendicular + ray_out_parallel
}
