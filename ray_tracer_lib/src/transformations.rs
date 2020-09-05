use crate::{matrix::*, tuple::*};
use std::f64::consts::PI;
pub fn identity() -> Matrix {
    Matrix::from([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn transform(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::from([
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    Matrix::from([
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_x(r: f64) -> Matrix {
    Matrix::from([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, r.cos(), -r.sin(), 0.0],
        [0.0, r.sin(), r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_y(r: f64) -> Matrix {
    Matrix::from([
        [r.cos(), 0.0, r.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-r.sin(), 0.0, r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn rotation_z(r: f64) -> Matrix {
    Matrix::from([
        [r.cos(), -r.sin(), 0.0, 0.0],
        [r.sin(), r.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    Matrix::from([
        [1.0, xy, xz, 0.0],
        [yx, 1.0, yz, 0.0],
        [zx, zy, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multiply_point_by_translation() {
        let t = transform(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert!(t * p == Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiply_point_by_inverse_of_translation() {
        let t = transform(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);

        assert!(t.inverse().unwrap() * p == Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_doesnt_affect_vectors() {
        let t = transform(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);

        assert!(t * v == v)
    }

    #[test]
    fn apply_scale_to_point() {
        let s = scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);

        assert!(s * p == Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn apply_scale_to_vector() {
        let t = scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert!(t * v == Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn apply_inverse_of_scaling_matrix() {
        let t = scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);

        assert!(t.inverse().unwrap() * v == Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let t = scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert!(t * p == Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotation_on_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert!(half_quarter * p == Tuple::point(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0));
        assert!(full_quarter * p == Tuple::point(0.0, 0.0, 1.0));
    }

    #[test]
    fn opposite_rotation_on_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0).inverse().unwrap();

        assert!(half_quarter * p == Tuple::point(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt() / 2.0)));
    }

    #[test]
    fn rotation_on_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);

        assert!(half_quarter * p == Tuple::point(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0));
    }

    #[test]
    fn rotation_on_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);

        assert!(half_quarter * p == Tuple::point(-(2_f64.sqrt() / 2.0), 2_f64.sqrt() / 2.0, 0.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let t = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert!(t * p == Tuple::point(5.0, 3.0, 4.0))
    }
    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let t = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert!(t * p == Tuple::point(6.0, 3.0, 4.0))
    }
    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let t = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert!(t * p == Tuple::point(2.0, 5.0, 4.0))
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let t = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert!(t * p == Tuple::point(2.0, 7.0, 4.0))
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let t = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert!(t * p == Tuple::point(2.0, 3.0, 6.0))
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let t = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);

        assert!(t * p == Tuple::point(2.0, 3.0, 7.0))
    }
}
