use std::ops;

#[derive(Debug, PartialEq)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        let Self { x, y, z } = self;
        Self {
            x: -x,
            y: -y,
            z: -z,
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
    use super::Vec3;

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
}
