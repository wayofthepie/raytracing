use std::ops;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
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
impl_op_ex_commutative!(*|l: &Vec3, r: f32| -> Vec3 { Vec3::new(l.x * r, l.y * r, l.z * r) });
impl_op_ex_commutative!(/|l: &Vec3, r: f32| -> Vec3 { Vec3::new(l.x / r, l.y / r, l.z / r) });
impl_op_ex!(+=|l: &mut Vec3, r: f32| {
        l.x = l.x + r;
        l.y = l.y + r;
        l.z = l.z + r;
});
impl_op_ex!(*=|l: &mut Vec3, r: f32| {
        l.x = l.x * r;
        l.y = l.y * r;
        l.z = l.z * r;
});
impl_op_ex!(/=|l: &mut Vec3, r: f32| {
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
    fn mul_with_f32_should_perform_correctly() {
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
    fn div_with_f32_should_perform_correctly() {
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
        let expected: f32 = (x * x) + (y * y) + (z * z);
        assert!(
            (expected - length_squared).abs() < f32::EPSILON,
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
        let expected: f32 = ((x * x) + (y * y) + (z * z)).sqrt();
        assert!(
            (expected - length).abs() < f32::EPSILON,
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
        let expected: f32 = (x * x) + (y * y) + (z * z);
        assert!(
            (expected - answer).abs() < f32::EPSILON,
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
