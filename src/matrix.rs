use approx::{assert_abs_diff_eq, AbsDiffEq};
use crate::tuple::{tuple, Tuple};
use std::ops::Mul;

#[derive(Debug, Clone, PartialEq)]
struct Matrix {
    data: Vec<Vec<f64>>,
}

impl Matrix {
    fn from_vec(data: Vec<Vec<f64>>) -> Matrix {
        Matrix { data }
    }

    fn identity(size: usize) -> Matrix {
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }
        Matrix::from_vec(data)
    }

    fn from_tuple(t: &Tuple) -> Matrix {
        Matrix::from_vec(vec![
            vec![t.x],
            vec![t.y],
            vec![t.z],
            vec![t.w],
        ])
    }

    fn to_tuple(&self) -> Tuple {
        assert_eq!(self.data.len(), 4, "Matrix must have exactly four rows to convert to Tuple");
        assert_eq!(self.data[0].len(), 1, "Matrix must have exactly one column to convert to Tuple");
        tuple(self.data[0][0], self.data[1][0], self.data[2][0], self.data[3][0])
    }

    fn transpose(&self) -> Matrix {
        let rows = self.data.len();
        let cols = self.data[0].len();
        
        let mut transposed = vec![vec![0.0; rows]; cols];

        for i in 0..rows {
            for j in 0..cols {
                transposed[j][i] = self.data[i][j];
            }
        }
        Matrix::from_vec(transposed)
    }

    fn determinant(&self) -> f64 {
        assert_eq!(self.data.len(), self.data[0].len(), "Determinant is only defined for square matrices");
        if self.data.len() == 2 {
            return self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0];
        }

        let mut result = 0.0;
        for i in 0..self.data.len() {
            result += self.cofactor(0, i) * self.data[0][i];
        }
        result
    }

    fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut submatrix = Vec::new();
        for i in 0..self.data.len() {
            if i != row {
                let mut new_row = Vec::new();
                for j in 0..self.data[i].len() {
                    if j != col {
                        new_row.push(self.data[i][j]);
                    }
                }
                submatrix.push(new_row);
            }
        }
        Matrix::from_vec(submatrix)
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        let submatrix = self.submatrix(row, col);
        submatrix.determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    fn inverse(&self) -> Matrix {
        assert!(self.is_invertible(), "Matrix is not invertible");
        let det = self.determinant();
        let mut result = vec![vec![0.0; self.data.len()]; self.data.len()];

        for i in 0..self.data.len() {
            for j in 0..self.data.len() {
                result[j][i] = self.cofactor(i, j) / det;
            }
        }
        Matrix::from_vec(result)
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        let l = self.data.len();
        let m = self.data[0].len();
        let n = other.data[0].len();
        assert_eq!(m, other.data.len(), "Matrix dimensions do not match for multiplication");

        let mut result = vec![vec![0.0; n]; l];
        
        for i in 0..l {
            for j in 0..m {
                for k in 0..n {
                    result[i][k] += self.data[i][j] * other.data[j][k];
                }
            }
        }

        Matrix::from_vec(result)
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        let result = self * Matrix::from_tuple(&other);
        result.to_tuple()
    }
}

impl AbsDiffEq for Matrix {
    type Epsilon = f64;

    fn default_epsilon() -> Self::Epsilon {
        1e-10
    }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        if self.data.len() != other.data.len() || self.data[0].len() != other.data[0].len() {
            return false;
        }
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                if (self.data[i][j] - other.data[i][j]).abs() > epsilon {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_4x4_matrix() {
        let m = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ]);
        assert_eq!(m.data[0][0], 1.0);
        assert_eq!(m.data[0][3], 4.0);
        assert_eq!(m.data[1][0], 5.5);
        assert_eq!(m.data[1][2], 7.5);
        assert_eq!(m.data[2][2], 11.0);
        assert_eq!(m.data[3][0], 13.5);
        assert_eq!(m.data[3][2], 15.5);
    }

    #[test]
    fn test_a_2x2_matrix() {
        let m = Matrix::from_vec(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]);
        assert_eq!(m.data[0][0], -3.0);
        assert_eq!(m.data[0][1], 5.0);
        assert_eq!(m.data[1][0], 1.0);
        assert_eq!(m.data[1][1], -2.0);
    }

    #[test]
    fn test_a_3x3_matrix() {
        let m = Matrix::from_vec(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ]);
        assert_eq!(m.data[0][0], -3.0);
        assert_eq!(m.data[1][1], -2.0);
        assert_eq!(m.data[2][2], 1.0);
    }

    #[test]
    fn test_matrix_equality() {
        let m1 = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_matrix_inequality() {
        let m1 = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::from_vec(vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 6.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ]);
        assert_ne!(m1, m2);
    }

    #[test]
    fn test_matrix_multiplication() {
        let a = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::from_vec(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ]);
        let expected = Matrix::from_vec(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ]);
        assert_eq!(a * b, expected);
    }

    #[test]
    fn test_matrix_multiplied_by_a_tuple() {
        let m = Matrix::from_vec(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ]);
        let t = tuple(1.0, 2.0, 3.0, 1.0);
        assert_eq!(m * t, tuple(18.0, 24.0, 33.0, 1.0));
    }

    #[test]
    fn test_matrix_multiplied_by_identity() {
        let a = Matrix::from_vec(vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ]);
        let b = Matrix::identity(4);
        let expected = a.clone();
        assert_eq!(a * b, expected);
    }

    #[test]
    fn test_transposing_a_matrix() {
        let m = Matrix::from_vec(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ]);
        let expected = Matrix::from_vec(vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(m.transpose(), expected);
    }

    #[test]
    fn test_calculating_determinant_of_a_2x2_matrix() {
        let m = Matrix::from_vec(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]);
        assert_eq!(m.determinant(), 17.0);
    }

    #[test]
    fn test_submatrix_of_3x3_matrix() {
        let m = Matrix::from_vec(vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ]);
        let submatrix = m.submatrix(0, 2);
        let expected = Matrix::from_vec(vec![
            vec![-3.0, 2.0],
            vec![0.0, 6.0],
        ]);
        assert_eq!(submatrix, expected);
    }

    #[test]
    fn test_submatrix_of_4x4_matrix() {
        let m = Matrix::from_vec(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ]);
        let submatrix = m.submatrix(2, 1);
        let expected = Matrix::from_vec(vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ]);
        assert_eq!(submatrix, expected);
    }

    #[test]
    fn test_minor_of_a_3x3_matrix() {
        let m = Matrix::from_vec(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        let s = m.submatrix(1, 0);
        assert_eq!(s.determinant(), 25.0);
        assert_eq!(m.minor(1, 0), 25.0);
    }

    #[test]
    fn test_cofactor_of_a_3x3_matrix() {
        let m = Matrix::from_vec(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ]);
        assert_eq!(m.minor(0, 0), -12.0);
        assert_eq!(m.cofactor(0, 0), -12.0);
        assert_eq!(m.minor(1, 0), 25.0);
        assert_eq!(m.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_determinant_of_a_3x3_matrix() {
        let m = Matrix::from_vec(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ]);
        assert_eq!(m.cofactor(0, 0), 56.0);
        assert_eq!(m.cofactor(0, 1), 12.0);
        assert_eq!(m.cofactor(0, 2), -46.0);
        assert_eq!(m.determinant(), -196.0);
    }

    #[test]
    fn test_determinant_of_a_4x4_matrix() {
        let m = Matrix::from_vec(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(m.cofactor(0, 0), 690.0);
        assert_eq!(m.cofactor(0, 1), 447.0);
        assert_eq!(m.cofactor(0, 2), 210.0);
        assert_eq!(m.cofactor(0, 3), 51.0);
        assert_eq!(m.determinant(), -4071.0);
    }

    #[test]
    fn test_an_invertible_matrix_for_invertibility() {
        let m = Matrix::from_vec(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ]);
        assert_eq!(m.determinant(), -2120.0);
        assert_eq!(m.is_invertible(), true);
    }

    #[test]
    fn test_a_non_invertible_matrix_for_invertibility() {
        let m = Matrix::from_vec(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![1.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(m.determinant(), 0.0);
        assert_eq!(m.is_invertible(), false);
    }

    #[test]
    fn test_inverse_of_a_matrix() {
        let m = Matrix::from_vec(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ]);
        let i = m.inverse();
        assert_eq!(m.determinant(), 532.0);
        assert_eq!(m.cofactor(2, 3), -160.0);
        assert_eq!(i.data[3][2], -160.0 / 532.0);
        assert_eq!(m.cofactor(3, 2), 105.0);
        assert_eq!(i.data[2][3], 105.0 / 532.0);

        let expected = Matrix::from_vec(vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        assert_abs_diff_eq!(i, expected, epsilon = 0.00001);
    }

    #[test]
    fn test_inverse_of_another_matrix() {
        let m = Matrix::from_vec(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ]);
        let i = m.inverse();
        let expected = Matrix::from_vec(vec![
            vec![-0.15385, -0.15385, -0.28205, -0.53846],
            vec![-0.07692, 0.12308, 0.02564, 0.03077],
            vec![0.35897, 0.35897, 0.43590, 0.92308],
            vec![-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert_abs_diff_eq!(i, expected, epsilon = 0.00001);
    }

    #[test]
    fn test_inverse_of_a_third_matrix() {
        let m = Matrix::from_vec(vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ]);
        let i = m.inverse();
        let expected = Matrix::from_vec(vec![
            vec![-0.04074, -0.07778, 0.14444, -0.22222],
            vec![-0.07778, 0.03333, 0.36667, -0.33333],
            vec![-0.02901, -0.14630, -0.10926, 0.12963],
            vec![0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        assert_abs_diff_eq!(i, expected, epsilon = 0.00001);
    }

    #[test]
    fn test_multiplying_a_matrix_by_its_inverse() {
        let m = Matrix::from_vec(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ]);
        let i = m.inverse();
        assert_abs_diff_eq!(m * i, Matrix::identity(4));
    }
}