use crate::{matrix::*, tuple::*};

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
}
