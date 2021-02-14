use crate::{
    material::Material,
    ray::Ray,
    vec3::{dot, Vec3},
};
use std::{cell::RefCell, collections::VecDeque, rc::Rc};

pub trait Hit<'material> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'material>>;
}

pub struct HitRecord<'material> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<RefCell<&'material mut (dyn Material + 'material)>>,
}

pub fn face_normal(ray: &Ray, outward_normal: Vec3) -> (bool, Vec3) {
    let front_face = dot(ray.direction, outward_normal) < 0.0;
    if front_face {
        (front_face, outward_normal)
    } else {
        (front_face, -outward_normal)
    }
}

pub struct Hittables<T>(VecDeque<T>);

impl<'material, T> Hittables<T>
where
    T: Hit<'material>,
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

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'material>> {
        let mut hit_record = None;
        let mut closest_to = t_max;
        for obj in self.0.iter() {
            if let Some(new_record) = obj.hit(ray, t_min, closest_to) {
                closest_to = new_record.t;
                hit_record = Some(new_record);
            }
        }
        hit_record
    }
}
