use std::ops::Neg;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Sub;
use std::ops::SubAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Vec3d<T: PartialEq + Eq + Clone + Copy> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: PartialEq + Eq + Clone + Copy> Vec3d<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x: x, y: y, z: z }
    }
}

impl<T> Neg for Vec3d<T>
where
    T: Neg<Output = T>
        + PartialEq
        + Eq
        + Clone
        + Copy
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Add for Vec3d<T>
where
    T: Add<Output = T>
        + PartialEq
        + Eq
        + Clone
        + Copy
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> AddAssign for Vec3d<T>
where
    T: Add<Output = T>
        + PartialEq
        + Eq
        + Clone
        + Copy
{
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl<T> Sub for Vec3d<T>
where
    T: Sub<Output = T>
        + PartialEq
        + Eq
        + Clone
        + Copy
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T> SubAssign for Vec3d<T>
where
    T: Sub<Output = T>
        + PartialEq
        + Eq
        + Clone
        + Copy
{
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl<T> Mul<T> for Vec3d<T>
where
    T: Mul<Output = T>
        + PartialEq
        + Eq
        + Clone
        + Copy
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> MulAssign<T> for Vec3d<T>
where
    T: Mul<Output = T>
        + PartialEq
        + Eq
        + Clone
        + Copy
{
    fn mul_assign(&mut self, rhs: T) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
    }
}

impl<T> Div<T> for Vec3d<T>
where
    T: Div<Output = T>
        + PartialEq
        + Eq
        + Clone
        + Copy
{
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T> DivAssign<T> for Vec3d<T>
where
    T: Div<Output = T>
        + PartialEq
        + Eq
        + Clone
        + Copy
{
    fn div_assign(&mut self, rhs: T) {
        *self = Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vec() {
        assert_eq!(Vec3d::new(5, 7, 9), Vec3d { x: 5, y: 7, z: 9 });
    }

    #[test]
    fn test_neg() {
        assert_eq!(
            -(Vec3d { x: 2, y: 4, z: 6 }),
            Vec3d { x: -2, y: -4, z: -6 }
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Vec3d { x: 2, y: 4, z: 6 } + Vec3d { x: 7, y: 10, z: 13 },
            Vec3d { x: 9, y: 14, z: 19 }
        );
    }

    #[test]
    fn test_add_assign() {
        let mut p1 = Vec3d { x: 2, y: 4, z: 6 };
        p1 += Vec3d { x: 7, y: 10, z: 13 };
        assert_eq!(p1, Vec3d { x: 9, y: 14, z: 19 });
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Vec3d { x: 2, y: 4, z: 6 } - Vec3d { x: 7, y: 10, z: 13 },
            Vec3d { x: -5, y: -6, z: -7 }
        );
    }

    #[test]
    fn test_sub_assign() {
        let mut p1 = Vec3d { x: 2, y: 4, z: 6 };
        p1 -= Vec3d { x: 7, y: 10, z: 13 };
        assert_eq!(p1, Vec3d { x: -5, y: -6, z: -7 });
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            Vec3d { x: 2, y: 4, z: 6 } * 3,
            Vec3d { x: 6, y: 12, z: 18 }
        );
    }

    #[test]
    fn test_mul_assign() {
        let mut p1 = Vec3d { x: 2, y: 4, z: 6 };
        p1 *= 3;
        assert_eq!(p1, Vec3d { x: 6, y: 12, z: 18 });
    }

    #[test]
    fn test_div() {
        assert_eq!(
            Vec3d { x: 2, y: 4, z: 6 } / 2,
            Vec3d { x: 1, y: 2, z: 3 }
        );
    }

    #[test]
    fn test_div_assign() {
        let mut p1 = Vec3d { x: 2, y: 4, z: 6 };
        p1 /= 2;
        assert_eq!(p1, Vec3d { x: 1, y: 2, z: 3 });
    }
}
