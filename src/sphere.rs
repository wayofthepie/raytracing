use std::collections::VecDeque;

use crate::{
    ray::{self, Ray},
    vec3::{dot, Vec3},
};

#[derive(Default)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = dot(ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hit {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable rande.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        record.t = root;
        record.point = ray.at(record.t);
        let outward_normal = (record.point - self.center) / self.radius;
        record.face_normal(ray, outward_normal);
        true
    }
}

pub struct Hittables<T>(VecDeque<T>)
where
    T: Hit;

impl<T> Hittables<T>
where
    T: Hit,
{
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn add(&mut self, hittable: T) {
        self.0.push_back(hittable)
    }

    pub fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut tmp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_to = t_max;
        for obj in self.0.iter() {
            if obj.hit(ray, t_min, closest_to, &mut tmp_record) {
                hit_anything = true;
                closest_to = tmp_record.t;
                *record = HitRecord { ..tmp_record };
            }
        }
        hit_anything
    }
}
