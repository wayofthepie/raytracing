use std::{cell::RefCell, rc::Rc};

use crate::{
    hit::{face_normal, Hit, HitRecord},
    material::Material,
    ray::Ray,
    vec3::{dot, Vec3},
};

pub struct Sphere<'material> {
    center: Vec3,
    radius: f64,
    material: Rc<RefCell<&'material mut (dyn Material + 'material)>>,
}

impl<'material> Sphere<'material> {
    pub fn new(
        center: Vec3,
        radius: f64,
        material: Rc<RefCell<&'material mut (dyn Material + 'material)>>,
    ) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<'material> Hit<'material> for Sphere<'material> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'material>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        // Find the nearest root that lies in the acceptable rande.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let t = root;
        let point = ray.at(t);
        let outward_normal = (point - self.center) / self.radius;
        let normal = face_normal(ray, outward_normal);
        Some(HitRecord {
            point: ray.at(root),
            normal,
            t: root,
            material: self.material.clone(),
        })
    }
}
