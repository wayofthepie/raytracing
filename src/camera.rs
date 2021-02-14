use crate::{
    ray::Ray,
    vec3::{cross, unit_vector, Vec3},
};

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vertical_up: Vec3,
        vertical_fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(look_from - look_at);
        let u = unit_vector(cross(vertical_up, w));
        let v = cross(w, u);
        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - (horizontal / 2.0) - (vertical / 2.0) - w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let direction =
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin;
        Ray::new(self.origin, direction)
    }
}
