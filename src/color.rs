use std::ops::{
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Div,
    DivAssign,
};

// r, g, b
// all are 0.0 <= x <= 1.0
#[derive(Clone, Copy)]
pub struct Color(f64, f64, f64);

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color(red, green, blue)
    }

    pub fn black() -> Color {
        Color(0.0, 0.0, 0.0)
    }

    pub fn white() -> Color {
        Color(1.0, 1.0, 1.0)
    }

    pub fn linear_interpolation(a: Color, b: Color, t: f64) -> Color {
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

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        self.0 -= other.0;
        self.1 -= other.1;
        self.2 -= other.2;
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color(self.0 * other, self.1 * other, self.2 * other)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, other: f64) {
        self.0 *= other;
        self.1 *= other;
        self.2 *= other;
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, other: f64) -> Color {
        Color(self.0 / other, self.1 / other, self.2 / other)
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, other: f64) {
        self.0 /= other;
        self.1 /= other;
        self.2 /= other;
    }
}
