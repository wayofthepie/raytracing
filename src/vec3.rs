use rand::{
    distributions::Uniform,
    prelude::{Distribution, ThreadRng},
};
use std::ops;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random(rng: &mut ThreadRng, between: &Uniform<f64>, min: f64, max: f64) -> Self {
        Vec3::new(
            random_bounded(rng, between, min, max),
            random_bounded(rng, between, min, max),
            random_bounded(rng, between, min, max),
        )
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * dot(v, n) * n
}

pub fn random_bounded(rng: &mut ThreadRng, between: &Uniform<f64>, min: f64, max: f64) -> f64 {
    min + (max - min) * between.sample(rng)
}

pub fn random_in_unit_sphere(rng: &mut ThreadRng, between: &Uniform<f64>) -> Vec3 {
    loop {
        let p = Vec3::random(rng, between, -1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_in_hemisphere(normal: Vec3, rng: &mut ThreadRng, between: &Uniform<f64>) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere(rng, between);
    if dot(in_unit_sphere, normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_unit_vector(rng: &mut ThreadRng, between: &Uniform<f64>) -> Vec3 {
    unit_vector(random_in_unit_sphere(rng, between))
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3 {
        x: (u.y * v.z) - (v.z * u.y),
        y: (u.z * v.x) - (u.x * v.z),
        z: (u.x * v.y) - (u.y * v.x),
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let Self { x, y, z } = self;
        Self::new(-x, -y, -z)
    }
}

impl_op_ex!(+|l: &Vec3, r: &Vec3| -> Vec3 { Vec3::new(l.x + r.x, l.y + r.y, l.z + r.z) });
impl_op_ex!(-|l: &Vec3, r: &Vec3| -> Vec3 { Vec3::new(l.x - r.x, l.y - r.y, l.z - r.z) });
impl_op_ex!(/|l: &Vec3, r: &Vec3| -> Vec3 { Vec3::new(l.x / r.x, l.y / r.y, l.z / r.z) });
impl_op_ex!(*|l: &Vec3, r: &Vec3| -> Vec3 { Vec3::new(l.x * r.x, l.y * r.y, l.z * r.z) });
impl_op_ex!(+=|l: &mut Vec3, r: &Vec3| {
        l.x = l.x + r.x;
        l.y = l.y + r.y;
        l.z = l.z + r.z;
});
impl_op_ex!(*=|l: &mut Vec3, r: &Vec3| {
        l.x = l.x * r.x;
        l.y = l.y * r.y;
        l.z = l.z * r.z;
});
impl_op_ex_commutative!(*|l: &Vec3, r: f64| -> Vec3 { Vec3::new(l.x * r, l.y * r, l.z * r) });
impl_op_ex_commutative!(/|l: &Vec3, r: f64| -> Vec3 { Vec3::new(l.x / r, l.y / r, l.z / r) });
impl_op_ex!(+=|l: &mut Vec3, r: f64| {
        l.x = l.x + r;
        l.y = l.y + r;
        l.z = l.z + r;
});
impl_op_ex!(*=|l: &mut Vec3, r: f64| {
        l.x = l.x * r;
        l.y = l.y * r;
        l.z = l.z * r;
});
impl_op_ex!(/=|l: &mut Vec3, r: f64| {
        l.x = l.x / r;
        l.y = l.y / r;
        l.z = l.z / r;
});

#[cfg(test)]
mod test {
    use super::{cross, dot, unit_vector, Vec3};

    #[test]
    fn negation_should_negate_all_fields() {
        let x = 0.1;
        let y = 0.2;
        let z = 0.3;
        let vec = Vec3 { x, y, z };
        assert_eq!(
            -vec,
            Vec3 {
                x: -x,
                y: -y,
                z: -z
            }
        );
    }

    #[test]
    fn mul_assign_should_perform_correctly() {
        let x = 1.0;
        let y = 2.0;
        let z = 3.0;
        let mut vec = Vec3 { x, y, z };
        vec *= 2.0;
        assert_eq!(
            vec,
            Vec3 {
                x: 2.0,
                y: 4.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn add_should_perform_correctly() {
        let x = 1.0;
        let y = 2.0;
        let z = 3.0;
        let one = Vec3 { x, y, z };
        let two = Vec3 { x, y, z };
        let answer = one + two;
        assert_eq!(
            answer,
            Vec3 {
                x: x + x,
                y: y + y,
                z: z + z
            }
        );
    }

    #[test]
    fn add_assign_should_perform_correctly() {
        let x = 1.0;
        let y = 2.0;
        let z = 3.0;
        let mut vec = Vec3 { x, y, z };
        vec += 1.0;
        assert_eq!(
            vec,
            Vec3 {
                x: x + 1.0,
                y: y + 1.0,
                z: z + 1.0
            }
        );
    }

    #[test]
    fn sub_should_perform_correctly() {
        let x = 1.0;
        let y = 2.0;
        let z = 3.0;
        let one = Vec3 { x, y, z };
        let two = Vec3 {
            x: x + 1.0,
            y: y + 1.0,
            z: z + 1.0,
        };
        let vec = two - one;
        assert_eq!(
            vec,
            Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0
            }
        );
    }

    #[test]
    fn mul_should_perform_correctly() {
        let x = 1.0;
        let y = 2.0;
        let z = 3.0;
        let one = Vec3 { x, y, z };
        let two = Vec3 {
            x: x + 1.0,
            y: y + 1.0,
            z: z + 1.0,
        };
        let vec = two * one;
        assert_eq!(
            vec,
            Vec3 {
                x: 2.0,
                y: 6.0,
                z: 12.0
            }
        );
    }

    #[test]
    fn mul_with_f64_should_perform_correctly() {
        let x = 1.0;
        let y = 2.0;
        let z = 3.0;
        let vec = Vec3 { x, y, z };
        let answer = vec * 2.0;
        assert_eq!(
            answer,
            Vec3 {
                x: 2.0,
                y: 4.0,
                z: 6.0
            }
        );
    }

    #[test]
    fn div_with_f64_should_perform_correctly() {
        let x = 2.0;
        let y = 3.0;
        let z = 5.0;
        let vec = Vec3 { x, y, z };
        let answer = vec / 2.0;
        assert_eq!(
            answer,
            Vec3 {
                x: x / 2.0,
                y: y / 2.0,
                z: z / 2.0
            }
        );
    }

    #[test]
    fn div_assign_should_perform_correctly() {
        let x = 2.0;
        let y = 3.0;
        let z = 5.0;
        let mut vec = Vec3 { x, y, z };
        vec /= 2.0;
        assert_eq!(
            vec,
            Vec3 {
                x: x / 2.0,
                y: y / 2.0,
                z: z / 2.0
            }
        );
    }

    #[test]
    fn length_squared_should_compute_correct_value() {
        let x = 2.0;
        let y = 3.0;
        let z = 5.0;
        let vec = Vec3 { x, y, z };
        let length_squared = vec.length_squared();
        let expected: f64 = (x * x) + (y * y) + (z * z);
        assert!(
            (expected - length_squared).abs() < f64::EPSILON,
            "got {} expected {}",
            length_squared,
            expected
        );
    }

    #[test]
    fn length_should_compute_correct_value() {
        let x = 2.0;
        let y = 3.0;
        let z = 5.0;
        let vec = Vec3 { x, y, z };
        let length = vec.length();
        let expected: f64 = ((x * x) + (y * y) + (z * z)).sqrt();
        assert!(
            (expected - length).abs() < f64::EPSILON,
            "got {} expected {}",
            length,
            expected
        );
    }

    #[test]
    fn dot_product_should_be_correct() {
        let x = 2.0;
        let y = 3.0;
        let z = 5.0;
        let one = Vec3 { x, y, z };
        let two = Vec3 { x, y, z };
        let answer = dot(one, two);
        let expected: f64 = (x * x) + (y * y) + (z * z);
        assert!(
            (expected - answer).abs() < f64::EPSILON,
            "got {} expected {}",
            answer,
            expected
        );
    }

    #[test]
    fn cross_product_should_be_correct() {
        let x = 2.0;
        let y = 3.0;
        let z = 5.0;
        let one = Vec3 { x, y, z };
        let two = Vec3 { x, y, z };
        let answer = cross(one, two);
        let expected = Vec3 {
            x: (y * z) - (z * y),
            y: (z * x) - (x * z),
            z: (x * y) - (y * x),
        };
        assert_eq!(answer, expected)
    }

    #[test]
    fn unit_vector_should_be_correct() {
        let x = 2.0;
        let y = 3.0;
        let z = 5.0;
        let one = Vec3 { x, y, z };
        let answer = unit_vector(one);
        let expected = one * (1.0 / one.length());
        assert_eq!(answer, expected);
    }
}
