use crate::tuple::Tuple;
use float_eq::float_eq;
use std::ops::{Add, Index, IndexMut, Mul, Sub};

#[derive(Clone, Debug)]
pub struct Matrix {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(width: usize, height: usize, data: Vec<f64>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    pub fn size(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height],
        }
    }

    pub fn identity(&self) -> Self {
        let mut data = vec![];
        for x in 0..self.width {
            for y in 0..self.height {
                if x == y {
                    data.push(1.0);
                } else {
                    data.push(0.0);
                }
            }
        }

        Self {
            width: self.width,
            height: self.height,
            data,
        }
    }

    pub fn identity_matrix(size: usize) -> Self {
        let mut data = vec![];
        for x in 0..size {
            for y in 0..size {
                if x == y {
                    data.push(1.0);
                } else {
                    data.push(0.0);
                }
            }
        }

        Self {
            width: size,
            height: size,
            data,
        }
    }

    pub fn transpose(&self) -> Self {
        let mut data = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                data.push(self[(x, y)]);
            }
        }

        Self {
            width: self.width,
            height: self.height,
            data,
        }
    }

    pub fn determinant(&self) -> f64 {
        if self.width != self.height {
            panic!("cannot calculate determinant for non-square matrices");
        }

        if self.width == 2 {
            self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
        } else {
            let mut determinant = 0.0;
            for x in 0..self.width {
                determinant += self[(0, x)] * self.cofactor(0, x)
            }

            determinant
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut data = self.data.clone();

        for (i, x) in (0..self.height).enumerate() {
            data.remove(col + self.width * x - i);
        }

        for (i, y) in (0..self.width - 1).enumerate() {
            data.remove(row * (self.width - 1) + y - i);
        }

        Self {
            width: self.width - 1,
            height: self.height - 1,
            data: data.to_vec(),
        }
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        if (row + col) % 2 != 0 {
            return -self.minor(row, col);
        }

        self.minor(row, col)
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Self {
        if !self.is_invertible() {
            panic!("cannot invert matrices with determinant of 0")
        }

        let mut inverse = Matrix::size(self.width, self.height);
        for row in 0..self.width {
            for col in 0..self.width {
                let cofactor = self.cofactor(row, col);
                inverse[(col, row)] = (cofactor / self.determinant() * 100000.0).round() / 100000.0;
            }
        }
        inverse
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let result = (self.width == other.width) && (self.height == other.height);
        if !result {
            return result;
        }
        self.data
            .iter()
            .zip(other.data.iter())
            .fold(true, |acc, (x, y)| acc && float_eq!(x, y, rmin <= 0.001))
    }
}

impl Eq for Matrix {}

impl Add for Matrix {
    type Output = Matrix;

    fn add(self, other: Self) -> Self {
        if self.width != other.width || self.height != other.height {
            panic!("cannot add two matrices of different dimensions");
        }

        let result: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(x, y)| x + y)
            .collect();

        Self {
            width: self.width,
            height: self.height,
            data: result,
        }
    }
}

impl Sub for Matrix {
    type Output = Matrix;

    fn sub(self, other: Self) -> Self {
        if self.width != other.width || self.height != other.height {
            panic!("cannot subtract two matrices of different dimensions");
        }

        let result: Vec<f64> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(x, y)| x - y)
            .collect();

        Self {
            width: self.width,
            height: self.height,
            data: result,
        }
    }
}

impl Mul<f64> for Matrix {
    type Output = Matrix;

    fn mul(self, other: f64) -> Self {
        Self {
            width: self.width,
            height: self.height,
            data: self.data.iter().map(|x| x * other).collect(),
        }
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;

    fn mul(self, other: Self) -> Self {
        if self.height != other.width {
            panic!("number of columns in the first matrix should be equal to number of rows in the second matrix!");
        }

        let mut result = vec![];

        for i in 0..self.width {
            for j in 0..other.height {
                let mut sum = 0.0;
                for k in 0..self.height {
                    sum += self[(i, k)] * other[(k, j)]
                }
                result.push((sum * 100000.0).round() / 100000.0);
            }
        }

        Self {
            width: self.width,
            height: other.height,
            data: result,
        }
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        if self.height != 4 {
            panic!("cannot multiply this matrix with a tuple!");
        }

        let tuple_matrix = Matrix::new(4, 1, vec![other.x, other.y, other.z, other.w]);

        let result = self * tuple_matrix;

        Tuple::new(
            result[(0, 0)],
            result[(1, 0)],
            result[(2, 0)],
            result[(3, 0)],
        )
    }
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;

    fn index(&self, (row, col): (usize, usize)) -> &f64 {
        match self.data.get(col + row * self.height) {
            Some(t) => t,
            None => panic!(
                "out of bounds! tried to get index of ({}, {}) for matrix size ({} {})",
                row, col, self.width, self.height
            ),
        }
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f64 {
        match self.data.get_mut(col + row * self.height) {
            Some(t) => t,
            None => panic!(
                "out of bounds! tried to get index of ({}, {}) for matrix size ({} {})",
                row, col, self.width, self.height
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_matrix() {
        let matrix = Matrix::size(2, 2);

        assert_eq!(matrix.data, vec![0.0; 4])
    }

    #[test]
    fn test_index_matrix() {
        let matrix = Matrix::new(2, 2, vec![1.0, 2.0, 3.0, 4.0]);

        assert_eq!(matrix[(0, 0)], 1.0);
        assert_eq!(matrix[(0, 1)], 2.0);
        assert_eq!(matrix[(1, 0)], 3.0);
        assert_eq!(matrix[(1, 1)], 4.0);
    }

    #[test]
    fn test_add_matrix_ok() {
        let matrix = Matrix::new(2, 2, vec![1.0; 4]);
        let other = Matrix::new(2, 2, vec![1.0; 4]);
        let result = Matrix::new(2, 2, vec![2.0; 4]);

        assert_eq!(matrix + other, result);
    }

    #[test]
    #[should_panic(expected = "cannot add two matrices of different dimensions")]
    fn test_add_matrix_fail() {
        let matrix = Matrix::size(2, 2);
        let other = Matrix::size(3, 2);

        let _ = matrix + other;
    }

    #[test]
    fn test_sub_matrix_ok() {
        let matrix = Matrix::new(2, 2, vec![1.0; 4]);
        let other = Matrix::new(2, 2, vec![1.0; 4]);
        let result = Matrix::new(2, 2, vec![0.0; 4]);

        assert_eq!(matrix - other, result);
    }

    #[test]
    #[should_panic(expected = "cannot subtract two matrices of different dimensions")]
    fn test_sub_matrix_fail() {
        let matrix = Matrix::size(2, 2);
        let other = Matrix::size(3, 2);

        let _ = matrix - other;
    }

    #[test]
    fn test_mul_matrix_scalar() {
        let matrix = Matrix::new(2, 2, vec![1.0, 1.0, 1.0, 1.0]);
        let scalar = 4.0;
        let result = Matrix::new(2, 2, vec![4.0, 4.0, 4.0, 4.0]);

        assert_eq!(matrix * scalar, result);
    }

    #[test]
    fn test_mul_matrices_ok() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );

        let other = Matrix::new(
            4,
            4,
            vec![
                -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
            ],
        );

        let result = Matrix::new(
            4,
            4,
            vec![
                20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0,
                26.0, 46.0, 42.0,
            ],
        );

        assert_eq!((matrix * other).data, result.data);
    }

    #[test]
    #[should_panic(
        expected = "number of columns in the first matrix should be equal to number of rows in the second matrix!"
    )]
    fn test_mul_matrices_fail() {
        let matrix = Matrix::size(2, 2);
        let other = Matrix::size(3, 2);

        let _ = matrix * other;
    }

    #[test]
    fn test_mul_matrix_tuple_ok() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            ],
        );
        let tuple = Tuple::new(1.0, 2.0, 3.0, 1.0);
        let result = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(matrix * tuple, result);
    }

    #[test]
    fn test_cmp_matrix() {
        let m1 = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );
        let m2 = Matrix::new(
            4,
            4,
            vec![
                1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
            ],
        );
        let m3 = Matrix::new(
            4,
            4,
            vec![
                2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0, 1.0,
            ],
        );

        assert_eq!(m1, m2);
        assert_ne!(m1, m3);
    }

    #[test]
    fn test_mul_identity_matrix() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                0.0, 1.0, 2.0, 4.0, 1.0, 2.0, 4.0, 8.0, 2.0, 4.0, 8.0, 17.0, 4.0, 8.0, 16.0, 32.0,
            ],
        );
        let identity = matrix.identity();

        assert_eq!(matrix.clone() * identity, matrix);
    }

    #[test]
    fn test_mul_identity_matrix_tuple() {
        let matrix = Matrix::size(4, 4).identity();
        let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);

        assert_eq!(matrix * tuple, tuple);
    }

    #[test]
    fn test_transpose_matrix() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
            ],
        );
        let transposed = Matrix::new(
            4,
            4,
            vec![
                0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
            ],
        );

        assert_eq!(matrix.transpose(), transposed);
    }

    #[test]
    fn test_transpose_identity_matrix() {
        let matrix = Matrix::size(2, 2).identity();

        assert_eq!(matrix.transpose(), matrix);
    }

    #[test]
    fn test_det_matrix_2x2() {
        let matrix = Matrix::new(2, 2, vec![1.0, 5.0, -3.0, 2.0]);
        let determinant = 17.0;

        assert_eq!(matrix.determinant(), determinant);
    }

    #[test]
    fn test_det_matrix_3x3() {
        let matrix = Matrix::new(3, 3, vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);

        assert_eq!(matrix.cofactor(0, 0), 56.0);
        assert_eq!(matrix.cofactor(0, 1), 12.0);
        assert_eq!(matrix.cofactor(0, 2), -46.0);
        assert_eq!(matrix.determinant(), -196.0);
    }

    #[test]
    fn test_det_matrix_4x4() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0,
                -9.0,
            ],
        );

        assert_eq!(matrix.cofactor(0, 0), 690.0);
        assert_eq!(matrix.cofactor(0, 1), 447.0);
        assert_eq!(matrix.cofactor(0, 2), 210.0);
        assert_eq!(matrix.cofactor(0, 3), 51.0);
        assert_eq!(matrix.determinant(), -4071.0);
    }

    #[test]
    #[should_panic(expected = "cannot calculate determinant for non-square matrices")]
    fn test_det_matrix_nonsq() {
        let matrix = Matrix::size(3, 4);

        matrix.determinant();
    }

    #[test]
    fn test_submatrix_3x3() {
        let matrix = Matrix::new(3, 3, vec![1.0, 5.0, 0.0, -3.0, 2.0, -7.0, 0.0, 6.0, -3.0]);
        let submatrix = Matrix::new(2, 2, vec![-3.0, 2.0, 0.0, 6.0]);

        assert_eq!(matrix.submatrix(0, 2), submatrix);
    }

    #[test]
    fn test_submatrix_4x4() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                -6.0, 1.0, 1.0, 6.0, -8.0, 5.0, 8.0, 6.0, -1.0, 0.0, 8.0, 2.0, -7.0, 1.0, -1.0, 1.0,
            ],
        );
        let submatrix = Matrix::new(3, 3, vec![-6.0, 1.0, 6.0, -8.0, 8.0, 6.0, -7.0, -1.0, 1.0]);

        assert_eq!(matrix.submatrix(2, 1), submatrix);
    }

    #[test]
    fn test_minor_3x3() {
        let matrix = Matrix::new(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
        let submatrix = matrix.submatrix(1, 0);

        assert_eq!(matrix.minor(1, 0), submatrix.determinant())
    }

    #[test]
    fn test_cofactor() {
        let matrix = Matrix::new(3, 3, vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);

        assert_eq!(matrix.minor(0, 0), matrix.cofactor(0, 0));
        assert_eq!(matrix.minor(1, 0), -matrix.cofactor(1, 0));
    }

    #[test]
    fn test_is_invertible() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                6.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 6.0, 4.0, -9.0, 3.0, -7.0, 9.0, 1.0, 7.0, -6.0,
            ],
        );

        assert_eq!(matrix.determinant(), -2120.0);
        assert!(matrix.is_invertible());
    }

    #[test]
    fn test_invertible_fail() {
        let matrix = Matrix::new(
            4,
            4,
            vec![
                -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
            ],
        );

        assert_eq!(matrix.determinant(), 0.0);
        assert!(!matrix.is_invertible());
    }

    #[test]
    fn test_invert_matrix() {
        let m1 = Matrix::new(
            4,
            4,
            vec![
                -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0,
                4.0,
            ],
        );

        let b1 = m1.inverse();

        assert_eq!(m1.determinant(), 532.0);
        assert_eq!(m1.cofactor(2, 3), -160.0);
        assert_eq!(b1[(3, 2)], -0.30075);
        assert_eq!(m1.cofactor(3, 2), 105.0);
        assert_eq!(b1[(2, 3)], 0.19737);
        assert_eq!(
            b1,
            Matrix::new(
                4,
                4,
                vec![
                    0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068,
                    -0.07895, -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639
                ]
            )
        );

        let m2 = Matrix::new(
            4,
            4,
            vec![
                8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0,
            ],
        );

        let b2 = m2.inverse();

        assert_eq!(
            b2,
            Matrix::new(
                4,
                4,
                vec![
                    -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077,
                    0.35897, 0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308
                ]
            )
        );

        let m3 = Matrix::new(
            4,
            4,
            vec![
                9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0,
                2.0,
            ],
        );

        let b3 = m3.inverse();

        assert_eq!(
            b3,
            Matrix::new(
                4,
                4,
                vec![
                    -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333,
                    -0.02901, -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333
                ]
            )
        )
    }

    #[test]
    fn test_mul_product_with_inverse() {
        let a = Matrix::new(
            4,
            4,
            vec![
                3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0,
                1.0,
            ],
        );

        let b = Matrix::new(
            4,
            4,
            vec![
                8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0,
            ],
        );

        let c = a.clone() * b.clone();
        let result = c * b.inverse();

        assert_eq!(a, result);
    }
}
