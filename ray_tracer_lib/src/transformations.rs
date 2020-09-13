use crate::matrix::*;

impl Matrix {
    pub fn translate(self, x: f64, y: f64, z: f64) -> Matrix {
        Matrix::from([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ]) * self
    }
    pub fn scale(self, x: f64, y: f64, z: f64) -> Matrix {
        Matrix::from([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * self
    }

    pub fn rotate_x(self, r: f64) -> Matrix {
        Matrix::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -r.sin(), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * self
    }

    pub fn rotate_y(self, r: f64) -> Matrix {
        Matrix::from([
            [r.cos(), 0.0, r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-r.sin(), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * self
    }

    pub fn rotate_z(self, r: f64) -> Matrix {
        Matrix::from([
            [r.cos(), -r.sin(), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * self
    }

    pub fn shear(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
        Matrix::from([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]) * self
    }
}

pub fn identity() -> Matrix {
    Matrix::from([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

pub fn translate(x: f64, y: f64, z: f64) -> Matrix {
    identity().translate(x, y, z)
}

pub fn scale(x: f64, y: f64, z: f64) -> Matrix {
    identity().scale(x, y, z)
}

pub fn rotate_x(r: f64) -> Matrix {
    identity().rotate_x(r)
}

pub fn rotate_y(r: f64) -> Matrix {
    identity().rotate_y(r)
}

pub fn rotate_z(r: f64) -> Matrix {
    identity().rotate_z(r)
}

pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix {
    identity().shear(xy, xz, yx, yz, zx, zy)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tuple::*;
    use std::f64::consts::PI;
    #[test]
    fn multiply_point_by_translation() {
        let t = translate(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);

        assert!(t * p == point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiply_point_by_inverse_of_translation() {
        let t = translate(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);

        assert!(t.inverse().unwrap() * p == point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_doesnt_affect_vectors() {
        let t = translate(5.0, -3.0, 2.0);
        let v = vector(-3.0, 4.0, 5.0);

        assert!(t * v == v)
    }

    #[test]
    fn apply_scale_to_point() {
        let s = scale(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);

        assert!(s * p == point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn apply_scale_to_vector() {
        let t = scale(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);

        assert!(t * v == vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn apply_inverse_of_scaling_matrix() {
        let t = scale(2.0, 3.0, 4.0);
        let v = vector(-4.0, 6.0, 8.0);

        assert!(t.inverse().unwrap() * v == vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let t = scale(-1.0, 1.0, 1.0);
        let p = point(2.0, 3.0, 4.0);

        assert!(t * p == point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotation_on_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotate_x(PI / 4.0);
        let full_quarter = rotate_x(PI / 2.0);

        assert!(half_quarter * p == point(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0));
        assert!(full_quarter * p == point(0.0, 0.0, 1.0));
    }

    #[test]
    fn opposite_rotation_on_x_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotate_x(PI / 4.0).inverse().unwrap();

        assert!(half_quarter * p == point(0.0, 2_f64.sqrt() / 2.0, -(2_f64.sqrt() / 2.0)));
    }

    #[test]
    fn rotation_on_y_axis() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotate_y(PI / 4.0);

        assert!(half_quarter * p == point(2_f64.sqrt() / 2.0, 0.0, 2_f64.sqrt() / 2.0));
    }

    #[test]
    fn rotation_on_z_axis() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotate_z(PI / 4.0);

        assert!(half_quarter * p == point(-(2_f64.sqrt() / 2.0), 2_f64.sqrt() / 2.0, 0.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let t = shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!(t * p == point(5.0, 3.0, 4.0))
    }
    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let t = shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!(t * p == point(6.0, 3.0, 4.0))
    }
    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let t = shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!(t * p == point(2.0, 5.0, 4.0))
    }

    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let t = shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!(t * p == point(2.0, 7.0, 4.0))
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let t = shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = point(2.0, 3.0, 4.0);

        assert!(t * p == point(2.0, 3.0, 6.0))
    }

    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let t = shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = point(2.0, 3.0, 4.0);

        assert!(t * p == point(2.0, 3.0, 7.0))
    }

    #[test]
    fn individual_transformations_are_applied_in_order() {
        let p = point(1.0, 0.0, 1.0);
        let a = rotate_x(PI / 2.0);
        let b = scale(5.0, 5.0, 5.0);
        let c = translate(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert!(p2 == point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert!(p3 == point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert!(p4 == point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations_are_applied_in_reverse_order() {
        let p = point(1.0, 0.0, 1.0);

        let t = identity()
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);

        assert!(t * p == point(15.0, 0.0, 7.0))
    }
}
