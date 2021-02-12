use std::ops;

#[derive(Debug, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
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

pub fn dot(u: &Vec3, v: &Vec3) -> f32 {
    u.x * v.x + u.y * v.y + u.z * v.z
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let Self { x, y, z } = self;
        Self {
            x: -x,
            y: -y,
            z: -z,
        }
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
            z: (self.z + rhs.z),
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: (self.x - rhs.x),
            y: (self.y - rhs.y),
            z: (self.z - rhs.z),
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: (self.x * rhs.x),
            y: (self.y * rhs.y),
            z: (self.z * rhs.z),
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: (self.x * rhs),
            y: (self.y * rhs),
            z: (self.z * rhs),
        }
    }
}

impl ops::AddAssign<f32> for Vec3 {
    fn add_assign(&mut self, rhs: f32) {
        self.x = self.x + rhs;
        self.y = self.y + rhs;
        self.z = self.z + rhs;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

#[cfg(test)]
mod test {
    use super::{dot, Vec3};

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
        let answer = dot(&one, &two);
        let expected: f32 = (x * x) + (y * y) + (z * z);
        assert!(
            (expected - answer).abs() < f32::EPSILON,
            "got {} expected {}",
            answer,
            expected
        );
    }
}
