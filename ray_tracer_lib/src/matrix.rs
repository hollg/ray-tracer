use crate::tuple::*;
use std::ops::{Index, Mul};
#[derive(Copy, Clone, Debug)]
pub struct Matrix {
    size: usize,
    values: [[f64; 4]; 4],
}

impl Matrix {
    fn build(size: usize, v: [f64; 4 * 4]) -> Matrix {
        let mut values = [[0.0; 4]; 4];
        for r in 0..size {
            for c in 0..size {
                values[r][c] = v[(r * size) + c];
            }
        }
        Matrix { size, values }
    }

    pub fn at(&self, y: usize, x: usize) -> f64 {
        self.values[y][x]
    }

    pub fn transpose(&self) -> Matrix {
        let mut values = [[0.0_f64; 4]; 4];

        for r in 0..self.size {
            for c in 0..self.size {
                values[r][c] = self.values[c][r];
            }
        }

        Matrix {
            size: self.size,
            values,
        }
    }

    pub fn determinant(&self) -> f64 {
        match self.size {
            2 => self[0][0] * self[1][1] - self[0][1] * self[1][0],
            _ => (0..self.size).fold(0.0, |result, num| {
                result + self.at(0, num) * self.cofactor(0, num)
            }),
        }
    }

    pub fn submatrix(&self, row: usize, column: usize) -> Matrix {
        let mut values = [[0.0; 4]; 4];
        let size = self.size - 1;

        for y in 0..size {
            for x in 0..size {
                let y2 = match y < row {
                    true => y,
                    false => y + 1,
                };

                let x2 = match x < column {
                    true => x,
                    false => x + 1,
                };

                values[y][x] = self.values[y2][x2];
            }
        }

        Matrix { size, values }
    }

    pub fn minor(&self, r: usize, c: usize) -> f64 {
        self.submatrix(r, c).determinant()
    }

    pub fn cofactor(&self, r: usize, c: usize) -> f64 {
        let minor = self.minor(r, c);

        match (r + c) % 2 {
            0 => minor,
            _ => -minor,
        }
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Result<Matrix, ()> {
        if !self.is_invertible() {
            return Err(());
        }
        let determinant = self.determinant();
        let mut values = [[0.0; 4]; 4];

        for r in 0..self.size {
            for c in 0..self.size {
                values[c][r] = self.cofactor(r, c) / determinant;
            }
        }

        Ok(Matrix {
            size: self.size,
            values,
        })
    }
}

impl Index<usize> for Matrix {
    type Output = [f64; 4];
    fn index(&self, i: usize) -> &[f64; 4] {
        &self.values[i]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if !self.size == other.size {
            return false;
        };

        let epsilon = 0.0001;

        for (y, row) in self.values.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if (*cell - other.at(y, x)).abs() > epsilon {
                    return false;
                }
            }
        }

        true
    }
}

impl From<[[f64; 4]; 4]> for Matrix {
    fn from(m: [[f64; 4]; 4]) -> Matrix {
        let mut values = [0.0; 16];

        for r in 0..4 {
            for c in 0..4 {
                values[r * 4 + c] = m[r][c]
            }
        }

        Matrix::build(4, values)
    }
}

impl From<[[f64; 3]; 3]> for Matrix {
    fn from(m: [[f64; 3]; 3]) -> Matrix {
        let mut values = [0.0; 16];

        for r in 0..3 {
            for c in 0..3 {
                values[r * 3 + c] = m[r][c]
            }
        }

        Matrix::build(3, values)
    }
}

impl From<[[f64; 2]; 2]> for Matrix {
    fn from(m: [[f64; 2]; 2]) -> Matrix {
        let mut values = [0.0; 16];

        for r in 0..2 {
            for c in 0..2 {
                values[r * 2 + c] = m[r][c]
            }
        }

        Matrix::build(2, values)
    }
}

impl Mul for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Matrix {
        let mut values = self.values.clone();

        for y in 0..self.size {
            for x in 0..self.size {
                // TODO: wrap head around why this works
                values[y][x] =
                    (0..self.size).fold(0.0, |acc, i| acc + self.at(y, i) * rhs.at(i, x));
            }
        }

        Matrix {
            size: self.size,
            values,
        }
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        let mut values = [0.0; 4];
        let Tuple { x, y, z, w } = other;

        for n in 0..self.size {
            values[n] =
                self.at(n, 0) * x + self.at(n, 1) * y + self.at(n, 2) * z + self.at(n, 3) * w;
        }

        Tuple {
            x: values[0],
            y: values[1],
            z: values[2],
            w: values[3],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn construct_and_inspect_matrix() {
        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert!(m.at(0, 0) == 1.0);
        assert!(m.at(0, 3) == 4.0);
        assert!(m.at(1, 0) == 5.5);
        assert!(m.at(1, 2) == 7.5);
        assert!(m.at(3, 0) == 13.5);
        assert!(m.at(3, 2) == 15.5);
    }

    #[test]
    fn support_2x2_matrix() {
        let m = Matrix::from([[-3.0, 5.0], [1.0, -2.0]]);
        assert!(m.at(0, 0) == -3.0);
        assert!(m.at(0, 1) == 5.0);
        assert!(m.at(1, 0) == 1.0);
        assert!(m.at(1, 1) == -2.0);
    }

    #[test]
    fn support_3x3_matrix() {
        let m = Matrix::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        assert!(m.at(0, 0) == -3.0);
        assert!(m.at(1, 1) == -2.0);
        assert!(m.at(2, 2) == 1.0);
    }

    #[test]
    fn equivalent_matrixes_are_equal() {
        let m1 = Matrix::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        let m2 = Matrix::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert!(m1 == m2);
    }

    #[test]
    fn different_matrices_arent_equal() {
        let m1 = Matrix::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);
        let m2 = Matrix::from([[3.0, -5.0, 0.0], [12.0, -2.0, -7.0], [10.0, 1.0, 1.0]]);

        assert!(m1 != m2);
    }

    #[test]
    fn multiplying_matrices() {
        let m1 = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let res = Matrix::from([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert!(m1 * m2 == res);
    }

    #[test]
    fn multiply_matrix_by_tuple() {
        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let t = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };

        assert!(
            m * t
                == Tuple {
                    x: 18.0,
                    y: 24.0,
                    z: 33.0,
                    w: 1.0
                }
        )
    }

    #[test]
    fn multiply_matrix_by_identity_matrix() {
        let identity_matrix = Matrix::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let m = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert!(
            m * identity_matrix
                == Matrix::from([
                    [1.0, 2.0, 3.0, 4.0],
                    [2.0, 4.0, 4.0, 2.0],
                    [8.0, 6.0, 4.0, 1.0],
                    [0.0, 0.0, 0.0, 1.0],
                ])
        );
    }

    #[test]
    fn multiply_identity_matrix_by_tuple() {
        let identity_matrix = Matrix::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let t = Tuple {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 1.0,
        };

        assert!(identity_matrix * t == t);
    }

    #[test]
    fn transpose_matrix() {
        let m = Matrix::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        assert!(
            m.transpose()
                == Matrix::from([
                    [0.0, 9.0, 1.0, 0.0],
                    [9.0, 8.0, 8.0, 0.0],
                    [3.0, 0.0, 5.0, 5.0],
                    [0.0, 8.0, 3.0, 8.0]
                ])
        );
    }

    #[test]
    fn transpose_identity_matrix() {
        let identity_matrix = Matrix::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert!(identity_matrix.transpose() == identity_matrix);
    }

    #[test]
    fn calculate_determinant_of_2x2_matrix() {
        let m = Matrix::from([[1.0, 5.0], [-3.0, 2.0]]);

        assert!(m.determinant() == 17.0);
    }

    #[test]
    fn submatrix_of_3x3_matrix_is_2x2_matrix() {
        let m = Matrix::from([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);

        assert!(m.submatrix(0, 2) == Matrix::from([[-3.0, 2.0], [0.0, 6.0]]));
    }

    #[test]
    fn submatrix_of_4x4_matrix_is_3x3_matrix() {
        let m = Matrix::from([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);

        assert!(
            m.submatrix(2, 1)
                == Matrix::from([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]])
        );
    }

    #[test]
    fn calculate_minor_of_3x3_matrix() {
        let m = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let s = m.submatrix(1, 0);

        assert!(s.determinant() == 25.0);
        assert!(m.minor(1, 0) == 25.0);
    }

    #[test]
    fn cofactor_of_3x3_matrix() {
        let m = Matrix::from([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);

        assert!(m.minor(0, 0) == -12.0);
        assert!(m.cofactor(0, 0) == -12.0);
        assert!(m.minor(1, 0) == 25.0);
        assert!(m.cofactor(1, 0) == -25.0);
    }

    #[test]
    fn determinant_of_3x3_matrix() {
        let m = Matrix::from([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);

        assert!(m.cofactor(0, 0) == 56.0);
        assert!(m.cofactor(0, 1) == 12.0);
        assert!(m.cofactor(0, 2) == -46.0);
        assert!(m.determinant() == -196.0);
    }

    #[test]
    fn determinant_of_4x4_matrix() {
        let m = Matrix::from([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);

        assert!(m.cofactor(0, 0) == 690.0);
        assert!(m.cofactor(0, 1) == 447.0);
        assert!(m.cofactor(0, 2) == 210.0);
        assert!(m.cofactor(0, 3) == 51.0);
        assert!(m.determinant() == -4071.0);
    }

    #[test]
    fn is_invertible() {
        let m_invertible = Matrix::from([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);

        let m_uninvertible = Matrix::from([
            [-4.0, 2.0, -2.0, 3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert!(m_invertible.determinant() == -2120.0 && m_invertible.is_invertible());
        assert!(m_uninvertible.determinant() == -0.0 && !m_uninvertible.is_invertible())
    }

    #[test]
    fn calculate_inverse() {
        let a = Matrix::from([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let b = a.inverse().unwrap();

        assert_eq!(532.0, a.determinant());
        assert_eq!(-160.0, a.cofactor(2, 3));
        assert_eq!(-160.0 / 532.0, b[3][2]);
        assert_eq!(105.0, a.cofactor(3, 2));
        assert_eq!(105.0 / 532.0, b[2][3]);

        let expected = Matrix::from([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        assert_eq!(expected, b);
    }

    #[test]
    fn multiple_matrix_by_its_inverse() {
        let a = Matrix::from([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);

        let b = Matrix::from([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let c = a * b;

        assert!(c * b.inverse().unwrap() == a);
    }
}
