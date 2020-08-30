use crate::tuple::*;
use std::ops::Mul;
pub struct Matrix {
    size: usize,
    values: [[f32; 4]; 4],
}

impl Matrix {
    fn build(size: usize, v: [f32; 4 * 4]) -> Matrix {
        let mut values = [[0.0; 4]; 4];
        for r in 0..size {
            for c in 0..size {
                values[r][c] = v[(r * size) + c];
            }
        }
        Matrix { size, values }
    }

    pub fn at(&self, y: usize, x: usize) -> f32 {
        self.values[y][x]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if !self.size == other.size {
            return false;
        };

        for (y, row) in self.values.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell != other.at(y, x) {
                    return false;
                }
            }
        }

        true
    }
}

impl From<[[f32; 4]; 4]> for Matrix {
    fn from(m: [[f32; 4]; 4]) -> Matrix {
        let mut values = [0.0; 16];

        for r in 0..4 {
            for c in 0..4 {
                values[r * 4 + c] = m[r][c]
            }
        }

        Matrix::build(4, values)
    }
}

impl From<[[f32; 3]; 3]> for Matrix {
    fn from(m: [[f32; 3]; 3]) -> Matrix {
        let mut values = [0.0; 16];

        for r in 0..3 {
            for c in 0..3 {
                values[r * 3 + c] = m[r][c]
            }
        }

        Matrix::build(3, values)
    }
}

impl From<[[f32; 2]; 2]> for Matrix {
    fn from(m: [[f32; 2]; 2]) -> Matrix {
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

impl Mul<&Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: &Tuple) -> Tuple {
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
}
