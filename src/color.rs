use std::ops::{Add, Mul, Sub};
use crate::consts::EPSILON;
#[derive(Clone, Copy, Debug, Default)]
pub struct Color(pub f32, pub f32, pub f32);

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        f32::abs(self.0 - other.0) < EPSILON
            && f32::abs(self.1 - other.1) < EPSILON
            && f32::abs(self.2 - other.2) < EPSILON
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color(self.0 * other, self.1 * other, self.2 * other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn adding_colors() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);

        assert!(c1 + c2 == Color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);
        let res = c1 - c2;
        println!("{:?}", res);
        assert!(res == Color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiply_color_by_scalar() {
        let c = Color(0.2, 0.3, 0.4);
        assert!(c * 2.0 == Color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiply_two_colors() {
        let c1 = Color(1.0, 0.2, 0.4);
        let c2 = Color(0.9, 1.0, 0.1);

        assert!(c1 * c2 == Color(0.9, 0.2, 0.04));
    }
}
