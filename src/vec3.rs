use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
    Neg,
};

use rand::random;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn zero() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn one() -> Vec3 {
        Vec3(1.0, 1.0, 1.0)
    }

    pub fn random() -> Vec3 {
        Vec3(random(), random(), random())
    }

    #[inline]
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    #[inline]
    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn unit(&self) -> Vec3 {
        let length = self.length();
        Vec3(self.0 / length, self.1 / length, self.2 / length)
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3(self.1 * other.2 - self.2 * other.1, self.2 * other.0 - self.0 * other.2, self.0 * other.1 - self.1 * other.0)
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.0
    }
 
    #[inline]
    pub fn y(&self) -> f64 {
        self.1
    }
 
    #[inline]
    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn black() -> Vec3 {
        Vec3(0.0, 0.0, 0.0)
    }

    pub fn white() -> Vec3 {
        Vec3(1.0, 1.0, 1.0)
    }

    pub fn linear_interpolation(a: Vec3, b: Vec3, t: f64) -> Vec3 {
        a * (1.0 - t) + b * t
    }

    #[inline]
    pub fn red(&self) -> f64 {
        self.0
    }

    #[inline]
    pub fn green(&self) -> f64 {
        self.1
    }

    #[inline]
    pub fn blue(&self) -> f64 {
        self.2
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Vec3) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        self.0 *= other.0;
        self.1 *= other.1;
        self.2 *= other.2;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, other: Vec3) -> Vec3 {
        Vec3(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f64) -> Vec3 {
        Vec3(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn dot() {
        let a = Vec3(1.0, 1.0, 0.0);
        let b = Vec3(-1.0, 1.0, 0.0);

        let c = a.dot(b);

        assert_eq!(c, 0.0);
    }

    #[test]
    fn cross() {
        let a = Vec3(1.0, 2.0, 3.0);
        let b = Vec3(4.0, 5.0, 6.0);

        let c = a.cross(b);
        assert_eq!(c, Vec3(-3.0, 6.0, -3.0));
    }

    #[test]
    fn cross_2() {
        let a = Vec3(1.0, 0.0, 0.0);
        let b = Vec3(0.0, 1.0, 0.0);

        assert_eq!(a.cross(b), Vec3(0.0, 0.0, 1.0));
        assert_eq!(b.cross(a), Vec3(0.0, 0.0, -1.0));
    }
}
