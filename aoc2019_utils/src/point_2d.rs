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
pub struct Point2d<T: PartialEq + Eq + Clone + Copy> {
    pub x: T,
    pub y: T,
}

impl<T: PartialEq + Eq + Clone + Copy> Point2d<T> {
    pub fn new(x: T, y: T) -> Point2d<T> {
        Point2d { x: x, y: y }
    }
}

impl<T> Neg for Point2d<T>
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
        }
    }
}

impl<T> Add for Point2d<T>
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
        }
    }
}

impl<T> AddAssign for Point2d<T>
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
        };
    }
}

impl<T> Sub for Point2d<T>
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
        }
    }
}

impl<T> SubAssign for Point2d<T>
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
        };
    }
}

impl<T> Mul<T> for Point2d<T>
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
        }
    }
}

impl<T> MulAssign<T> for Point2d<T>
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
        };
    }
}

impl<T> Div<T> for Point2d<T>
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
        }
    }
}

impl<T> DivAssign<T> for Point2d<T>
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
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_point() {
        assert_eq!(Point2d::new(5, 7), Point2d { x: 5, y: 7 });
    }

    #[test]
    fn test_neg() {
        assert_eq!(
            -(Point2d { x: 2, y: 4 }),
            Point2d { x: -2, y: -4 }
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Point2d { x: 2, y: 4 } + Point2d { x: 7, y: 10 },
            Point2d { x: 9, y: 14 }
        );
    }

    #[test]
    fn test_add_assign() {
        let mut p1 = Point2d { x: 2, y: 4 };
        p1 += Point2d { x: 7, y: 10 };
        assert_eq!(p1, Point2d { x: 9, y: 14 });
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Point2d { x: 2, y: 4 } - Point2d { x: 7, y: 10 },
            Point2d { x: -5, y: -6 }
        );
    }

    #[test]
    fn test_sub_assign() {
        let mut p1 = Point2d { x: 2, y: 4 };
        p1 -= Point2d { x: 7, y: 10 };
        assert_eq!(p1, Point2d { x: -5, y: -6 });
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            Point2d { x: 2, y: 4 } * 3,
            Point2d { x: 6, y: 12 }
        );
    }

    #[test]
    fn test_mul_assign() {
        let mut p1 = Point2d { x: 2, y: 4 };
        p1 *= 3;
        assert_eq!(p1, Point2d { x: 6, y: 12 });
    }

    #[test]
    fn test_div() {
        assert_eq!(
            Point2d { x: 2, y: 4 } / 2,
            Point2d { x: 1, y: 2 }
        );
    }

    #[test]
    fn test_div_assign() {
        let mut p1 = Point2d { x: 2, y: 4 };
        p1 /= 2;
        assert_eq!(p1, Point2d { x: 1, y: 2 });
    }
}
