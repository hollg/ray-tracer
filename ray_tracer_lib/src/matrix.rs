pub struct Matrix {
    data: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f32>>) -> Matrix {
        Matrix { data }
    }

    pub fn get(&self, y: usize, x: usize) -> f32 {
        self.data[y][x]
    }

    pub fn size(&self) -> usize {
        self.data.len() * self.data.len()
    }

    pub fn num_rows(&self) -> usize {
        self.data.len()
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        if !self.size() == other.size() {
            return false;
        };

        for (y, row) in self.data.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell != other.get(y, x) {
                    return false
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn construct_and_inspect_matrix() {
        let m = Matrix {
            data: vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![5.5, 6.5, 7.5, 8.5],
                vec![9.0, 10.0, 11.0, 12.0],
                vec![13.5, 14.5, 15.5, 16.5],
            ],
        };

        assert!(m.get(0, 0) == 1.0);
        assert!(m.get(0, 3) == 4.0);
        assert!(m.get(1, 0) == 5.5);
        assert!(m.get(1, 2) == 7.5);
        assert!(m.get(3, 0) == 13.5);
        assert!(m.get(3, 2) == 15.5);
    }

    #[test]
    fn support_2x2_matrix() {
        let m = Matrix {
            data: vec![vec![-3.0, 5.0], vec![1.0, -2.0]],
        };
        assert!(m.get(0, 0) == -3.0);
        assert!(m.get(0, 1) == 5.0);
        assert!(m.get(1, 0) == 1.0);
        assert!(m.get(1, 1) == -2.0);
    }

    #[test]
    fn support_3x3_matrix() {
        let m = Matrix {
            data: vec![
                vec![-3.0, 5.0, 0.0],
                vec![1.0, -2.0, -7.0],
                vec![0.0, 1.0, 1.0],
            ],
        };
        assert!(m.get(0, 0) == -3.0);
        assert!(m.get(1, 1) == -2.0);
        assert!(m.get(2, 2) == 1.0);
    }

    #[test]
    fn equivalent_matrixes_are_equal() {
        let m1 = Matrix {
            data: vec![
                vec![-3.0, 5.0, 0.0],
                vec![1.0, -2.0, -7.0],
                vec![0.0, 1.0, 1.0],
            ],
        };
        let m2 = Matrix {
            data: vec![
                vec![-3.0, 5.0, 0.0],
                vec![1.0, -2.0, -7.0],
                vec![0.0, 1.0, 1.0],
            ],
        };

        assert!(m1 == m2);
    } 
}
