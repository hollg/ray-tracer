use crate::consts::EPSILON;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Self {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn w(&self) -> f64 {
        self.w
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let magnitude = self.magnitude();
        Tuple {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        }
    }

    pub fn dot(&self, rhs: Tuple) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    pub fn cross(&self, other: &Self) -> Self {
        Tuple {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0,
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        f64::abs(self.x - other.x) < EPSILON
            && f64::abs(self.y - other.y) < EPSILON
            && f64::abs(self.z - other.z) < EPSILON
            && f64::abs(self.w - other.w) < EPSILON
    }
}

impl Add for Tuple {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Self {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    Tuple { x, y, z, w }
}

pub fn point<A: Into<f64>, B: Into<f64>, C: Into<f64>>(x: A, y: B, z: C) -> Tuple {
    tuple(x.into(), y.into(), z.into(), 1.0)
}

pub fn vector<A: Into<f64>, B: Into<f64>, C: Into<f64>>(x: A, y: B, z: C) -> Tuple {
    tuple(x.into(),y.into(), z.into(), 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn w1_for_point() {
        let tuple = Tuple {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        assert!(tuple.is_point())
    }

    #[test]
    fn w0_for_point() {
        let tuple = Tuple {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 0.0,
        };
        assert!(tuple.is_vector())
    }

    #[test]
    fn point_creates_tuple_with_w1() {
        let tuple = point(1.0, 1.0, 1.0);
        assert!(tuple.w == 1.0)
    }

    #[test]
    fn vector_creates_tuple_with_w0() {
        let tuple = vector(1.0, 1.0, 1.0);
        assert!(tuple.w == 0.0)
    }

    #[test]
    fn equivalent_tuples_are_equal() {
        let tuple_a = vector(1.0, 1.0, 1.0);
        let tuple_b = vector(1.0, 1.0, 1.0);
        assert!(tuple_a == tuple_b)
    }
    #[test]
    fn add_tuples() {
        let tuple_a = Tuple {
            x: 1.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        };
        let tuple_b = Tuple {
            x: 1.0,
            y: 1.0,
            z: 0.0,
            w: 1.0,
        };
        assert!(
            tuple_a + tuple_b
                == Tuple {
                    x: 2.0,
                    y: 2.0,
                    z: 0.0,
                    w: 1.0
                }
        )
    }
    #[test]
    fn subtract_two_points() {
        let point_a = point(1.0, 1.0, 1.0);
        let point_b = point(2.0, 2.0, 2.0);

        assert!(
            point_a - point_b
                == Tuple {
                    x: -1.0,
                    y: -1.0,
                    z: -1.0,
                    w: 0.0
                }
        )
    }

    #[test]
    fn subtract_vector_from_point_is_point() {
        let point = point(1.0, 1.0, 1.0);
        let vector = vector(1.0, 1.0, 1.0);

        assert!((point - vector).is_point())
    }

    #[test]
    fn subtract_vector_from_vector_is_vector() {
        let vector_1 = vector(1.0, 1.0, 1.0);
        let vector_2 = vector(2.0, 2.0, 1.0);

        assert!((vector_1 - vector_2).is_vector())
    }

    #[test]
    fn negate_tuple() {
        let tuple = Tuple {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        assert!(
            -tuple
                == Tuple {
                    x: -1.0,
                    y: -1.0,
                    z: -1.0,
                    w: -1.0
                }
        )
    }

    #[test]
    fn multiply_tuple_by_scalar() {
        let tuple = Tuple {
            x: 1.0,
            y: 1.0,
            z: 1.0,
            w: 1.0,
        };
        assert!(
            tuple * 2.0
                == Tuple {
                    x: 2.0,
                    y: 2.0,
                    z: 2.0,
                    w: 2.0
                }
        )
    }

    #[test]
    fn multiply_tuple_by_fraction() {
        let tuple = Tuple {
            x: 2.0,
            y: 2.0,
            z: 2.0,
            w: 2.0,
        };
        assert!(
            tuple * 0.5
                == Tuple {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                    w: 1.0
                }
        )
    }

    #[test]
    fn divide_tuple_by_scalar() {
        let tuple = Tuple {
            x: 2.0,
            y: 2.0,
            z: 2.0,
            w: 2.0,
        };
        assert!(
            tuple / 2.0
                == Tuple {
                    x: 1.0,
                    y: 1.0,
                    z: 1.0,
                    w: 1.0
                }
        )
    }
    #[test]
    fn magnitude_of_vectors() {
        let v1 = vector(0.0, 1.0, 0.0);
        let v2 = vector(0.0, 0.0, 1.0);
        let v3 = vector(0.0, 1.0, 0.0);
        let v4 = vector(1.0, 2.0, 3.0);
        let v5 = vector(-1.0, -2.0, -3.0);

        assert!(v1.magnitude() == 1.0);
        assert!(v2.magnitude() == 1.0);
        assert!(v3.magnitude() == 1.0);
        assert!(v4.magnitude() == (14.0f64).sqrt());
        assert!(v5.magnitude() == (14.0f64).sqrt());
    }

    #[test]
    fn normalize_vectors() {
        let v1 = vector(4.0, 0.0, 0.0);
        let v2 = vector(1.0, 2.0, 3.0);

        assert!(v1.normalize() == vector(1.0, 0.0, 0.0));
        assert!(
            v2.normalize()
                == vector(
                    1.0 / f64::sqrt(14.0),
                    2.0 / f64::sqrt(14.0),
                    3.0 / f64::sqrt(14.0)
                )
        );
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let t1 = vector(1.0, 2.0, 3.0);
        let t2 = vector(2.0, 3.0, 4.0);
        assert!(t1.dot(t2) == 20.0)
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let t1 = vector(1.0, 2.0, 3.0);
        let t2 = vector(2.0, 3.0, 4.0);

        assert!(t1.cross(&t2) == vector(-1.0, 2.0, -1.0));
        assert!(t2.cross(&t1) == vector(1.0, -2.0, 1.0));
    }
}
