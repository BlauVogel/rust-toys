use std::ops::AddAssign;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Eq, PartialEq)]
pub struct Matrix<T> {
    row: usize,    // 行
    cloumn: usize, // 列
    data: Vec<T>,
}

impl<T> Matrix<T>
where
    T: Copy,
{
    pub fn new(row: usize, cloumn: usize, data: Vec<T>) -> Matrix<T> {
        Matrix { row, cloumn, data }
    }

    pub fn new_empty(row: usize, cloumn: usize) -> Matrix<T> {
        Matrix {
            row,
            cloumn,
            data: Vec::with_capacity(row * cloumn),
        }
    }

    pub fn slice(&self, (a, b): (usize, usize), (c, d): (usize, usize)) -> Matrix<T> {
        assert!(a < b && c < d);
        let mut new_matrix = Matrix::<T>::new_empty(b - a, d - c);
        for i in 0..b - a {
            for j in 0..d - c {
                new_matrix
                    .data
                    .push(self.data[(a + i) * self.cloumn + c + j]);
            }
        }
        new_matrix
    }

    pub fn combine4(
        m11: &Matrix<T>,
        m12: &Matrix<T>,
        m21: &Matrix<T>,
        m22: &Matrix<T>,
    ) -> Matrix<T> {
        let n = m11.row;
        let mut new_matrix = Matrix::<T>::new_empty(2 * n, 2 * n);
        for i in 0..n {
            for j in 0..n {
                new_matrix.data.push(m11.data[i * n + j]);
            }
            for j in 0..n {
                new_matrix.data.push(m12.data[i * n + j]);
            }
        }
        for i in 0..n {
            for j in 0..n {
                new_matrix.data.push(m21.data[i * n + j]);
            }
            for j in 0..n {
                new_matrix.data.push(m22.data[i * n + j]);
            }
        }
        new_matrix
    }
}

impl<'a, 'b, T> Add<&'a Matrix<T>> for &'b Matrix<T>
where
    T: Copy + Add<Output = T>,
{
    type Output = Matrix<T>;

    fn add(self, rhs: &'a Matrix<T>) -> Matrix<T> {
        assert!(self.row == rhs.row && self.cloumn == rhs.cloumn);
        let mut new_matrix = Matrix::<T>::new_empty(self.row, rhs.cloumn);
        for i in 0..self.row {
            for j in 0..self.cloumn {
                new_matrix
                    .data
                    .push(self.data[i * self.cloumn + j] + rhs.data[i * self.cloumn + j]);
            }
        }
        new_matrix
    }
}

impl<'a, 'b, T> Sub<&'a Matrix<T>> for &'b Matrix<T>
where
    T: Copy + Sub<Output = T>,
{
    type Output = Matrix<T>;

    fn sub(self, rhs: &'a Matrix<T>) -> Matrix<T> {
        assert!(self.row == rhs.row && self.cloumn == rhs.cloumn);
        let mut new_matrix = Matrix::<T>::new_empty(self.row, rhs.cloumn);
        for i in 0..self.row {
            for j in 0..self.cloumn {
                new_matrix
                    .data
                    .push(self.data[i * self.cloumn + j] - rhs.data[i * self.cloumn + j]);
            }
        }
        new_matrix
    }
}

impl<'a, 'b, T> Mul<&'a Matrix<T>> for &'b Matrix<T>
where
    T: From<i32> + Mul<Output = T> + AddAssign + Copy,
    for<'c, 'd> &'c Matrix<T>:
        Add<&'d Matrix<T>, Output = Matrix<T>> + Sub<&'d Matrix<T>, Output = Matrix<T>>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &'a Matrix<T>) -> Matrix<T> {
        assert_eq!(self.cloumn, rhs.row);
        let mut new_matrix = Matrix::<T>::new_empty(self.row, rhs.cloumn);

        if self.row == self.cloumn && self.row >= 4 && (self.row & (self.row - 1)) == 0 {
            // 使用 Strassen 算法
            let a11 = self.slice((0, self.row / 2), (0, self.cloumn / 2));
            let a12 = self.slice((0, self.row / 2), (self.cloumn / 2, self.cloumn));
            let a21 = self.slice((self.row / 2, self.row), (0, self.cloumn / 2));
            let a22 = self.slice((self.row / 2, self.row), (self.cloumn / 2, self.cloumn));
            let b11 = self.slice((0, rhs.row / 2), (0, rhs.cloumn / 2));
            let b12 = self.slice((0, rhs.row / 2), (rhs.cloumn / 2, rhs.cloumn));
            let b21 = self.slice((rhs.row / 2, rhs.row), (0, rhs.cloumn / 2));
            let b22 = self.slice((rhs.row / 2, rhs.row), (rhs.cloumn / 2, rhs.cloumn));

            let s1: Matrix<T> = &(&a12 - &a22) * &(&b21 + &b22);
            let s2: Matrix<T> = &(&a11 + &a22) * &(&b11 + &b22);
            let s3: Matrix<T> = &(&a11 - &a21) * &(&b11 + &b12);
            let s4: Matrix<T> = &(&a11 + &a12) * &b22;
            let s5: Matrix<T> = &a11 * &(&b12 - &b22);
            let s6: Matrix<T> = &a22 * &(&b21 - &b11);
            let s7: Matrix<T> = &(&a21 + &a22) * &b11;

            let m11: Matrix<T> = &(&s1 + &s2) - &(&s4 - &s6);
            let m12: Matrix<T> = &s4 + &s5;
            let m21: Matrix<T> = &s6 + &s7;
            let m22: Matrix<T> = &(&s2 - &s3) + &(&s5 - &s7);

            return Matrix::<T>::combine4(&m11, &m12, &m21, &m22);
        }

        let mut temp: T;
        for i in 0..self.row {
            for j in 0..rhs.cloumn {
                temp = 0.into();
                for k in 0..self.cloumn {
                    temp += self.data[i * self.cloumn + k] * rhs.data[k * rhs.cloumn + j];
                }
                new_matrix.data.push(temp);
            }
        }
        new_matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slice() {
        let m = Matrix::new(3, 4, vec![4, 6, -4, 2, 2, -3, 4, -9, -1, 3, 4, 1]);
        assert_eq!(
            m.slice((0, 2), (1, 3)),
            Matrix::new(2, 2, vec![6, -4, -3, 4])
        );
    }

    #[test]
    fn test_mul() {
        let m1 = Matrix::new(3, 4, vec![4, 6, -4, 2, 2, -3, 4, -9, -1, 3, 4, 1]);
        let m2 = Matrix::new(4, 2, vec![4, -3, -5, -1, 9, 6, 3, 5]);
        assert_eq!(
            &m1 * &m2,
            Matrix::new(3, 2, vec![-44, -32, 32, -24, 20, 29])
        );

        let m = Matrix::new(
            4,
            4,
            vec![3, -1, 4, -1, 5, 1, 4, 2, -1, -4, 2, 5, -2, 3, 1, 9],
        );
        assert_eq!(
            &m * &m,
            Matrix::new(
                4,
                4,
                vec![2, -23, 15, 6, 12, -14, 34, 35, -35, 4, -11, 48, -10, 28, 15, 94],
            )
        );
    }
}
