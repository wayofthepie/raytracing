use crate::{
    ray::Ray,
    vec3::{dot, Vec3},
};

struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f32,
}

trait Hit {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}

struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant > 0.0 {
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
        record.normal = (record.point - self.center) / self.radius;
        true
    }
}
