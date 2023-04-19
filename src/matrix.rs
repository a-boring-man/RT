// dependency
use std::ops;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Matrix<T: Copy + Default> {
    data: Vec<T>,
    nbr_row: u8,
    nbr_col: u8,
    nbr_elm: u16,
}

#[allow(dead_code)]
impl<T: Copy + Default> Matrix<T> {

    pub fn default() -> Self {
        Matrix {
            data: vec![T::default(); 16],
            nbr_col: 4,
            nbr_row: 4,
            nbr_elm: 16,
        }
    }

    pub fn new(nbr_col: u8, nbr_row: u8) -> Self {
        let nbr_elm = nbr_col as u16 * nbr_row as u16;
        Matrix {
            nbr_elm,
            nbr_col,
            nbr_row,
            data: vec![T::default(); nbr_elm as usize],
        }
    }

    /**
     * here we asume that data has the rigth size if it's not the case the new function is called instead
     */
    pub fn new_filled(nbr_col: u8, nbr_row: u8, data: Vec<T>) -> Self {
        let nbr_elm: u16 = nbr_col as u16 * nbr_row as u16;
        if nbr_elm as usize != data.len() {
            return Matrix::new(nbr_col, nbr_row);
        }
        Matrix { data, nbr_row, nbr_col, nbr_elm, }
    }

    pub fn destroy(_: Self) {

    }

    pub fn get_elm(&self, row: u8, col:u8) -> T {
        self.data[Matrix::<T>::linear_index_conversion(col, row) as usize]
    }

    pub fn get_elm_linear(&self, index: u16) -> T {
        self.data[index as usize]
    }

    pub fn set_elm(&mut self, row: u8, col: u8, elm: T) {
        self.data[Matrix::<T>::linear_index_conversion(col, row) as usize] = elm;
    }

    pub fn get_nbr_row(&self) -> u8 {
        self.nbr_row
    }

    pub fn get_nbr_col(&self) -> u8 {
        self.nbr_col
    }

    pub fn get_nbr_elm(&self) -> u16 {
        self.nbr_elm
    }

    pub fn linear_index_conversion(col: u8, row: u8) -> u16 {
        col as u16 * row as u16 + col as u16
    }

}

impl<T: Copy + Default + std::ops::Add<Output = T>> ops::Add<&Matrix<T>> for Matrix<T> {
    type Output = Self;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        let nbr_elm = self.get_nbr_elm();
        let mut tmp_data = Vec::with_capacity(16);
        for i in 0..nbr_elm {
            tmp_data.push(self.get_elm_linear(i) + rhs.get_elm_linear(i));
        }
        Matrix::new_filled(self.get_nbr_col(), self.get_nbr_row(), tmp_data)
    }
}

impl<T: Copy + Default + std::ops::Add<Output = T>> ops::Add<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        let nbr_elm = self.get_nbr_elm();
        let mut tmp_data = Vec::with_capacity(16);
        for i in 0..nbr_elm {
            tmp_data.push(self.get_elm_linear(i) + rhs.get_elm_linear(i));
        }
        Matrix::new_filled(self.get_nbr_col(), self.get_nbr_row(), tmp_data)
    }
}

impl<T: Copy + Default + std::ops::Add<Output = T>> ops::Add<Matrix<T>> for Matrix<T> {
    type Output = Self;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        let nbr_elm = self.get_nbr_elm();
        let mut tmp_data = Vec::with_capacity(16);
        for i in 0..nbr_elm {
            tmp_data.push(self.get_elm_linear(i) + rhs.get_elm_linear(i));
        }
        Matrix::new_filled(self.get_nbr_col(), self.get_nbr_row(), tmp_data)
    }
}

impl<T: Copy + Default + std::ops::Add<Output = T>> ops::Add<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        let nbr_elm = self.get_nbr_elm();
        let mut tmp_data = Vec::with_capacity(16);
        for i in 0..nbr_elm {
            tmp_data.push(self.get_elm_linear(i) + rhs.get_elm_linear(i));
        }
        Matrix::new_filled(self.get_nbr_col(), self.get_nbr_row(), tmp_data)
    }
}

impl<T: Copy + Default + std::ops::Add<Output = T>> ops::Add<T> for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: T) -> Self::Output {
        let mut tmp_data = Vec::with_capacity(16);
        for i in 0..self.get_nbr_elm() {
            tmp_data.push(self.get_elm_linear(i as u16) + rhs);
        }
        Matrix::new_filled(self.get_nbr_col(), self.get_nbr_row(), tmp_data)
    }
}


#[cfg(test)]
mod test {

    use super::*;
    use crate::vec3::Vec3;

    #[test]
    fn test_default() {
        let m1 = Matrix::default();
        let data = vec![Vec3::new(0.0, 0.0, 0.0);16];
        let m2 = Matrix {data, nbr_col: 4, nbr_row: 4, nbr_elm: 16};
        assert_eq!(m1, m2);
    }

    #[test]
    fn test_new() {
        let data_ref = vec![0.0; 9];
        let mref = Matrix {data: data_ref, nbr_col: 3, nbr_row: 3, nbr_elm: 9};
        let data_ref2 = vec![Vec3::new(0.0, 0.0, 0.0); 16];
        let mref2 = Matrix {data: data_ref2, nbr_col: 4, nbr_row: 4, nbr_elm: 16};
        let m1 = Matrix::new(3, 3);
        let m2 = Matrix::new(4, 4);
        assert_eq!(m1, mref);
        assert_eq!(m2, mref2);
    }

    #[test]
    fn test_new_filled() {
        let data1 = vec![Vec3::new(0.0, 0.0, 0.0); 16];
        let data2 = vec![0.0; 9];
        assert_eq!(Matrix::new_filled(4, 4, data1), Matrix::new(4, 4));
        assert_eq!(Matrix::new_filled(3, 3, data2), Matrix::new(3, 3));
    }

    #[test]
    fn test_add_operator1() {
        let m1 = Matrix::<f64>::new(4, 4);
        let data = vec![4.0; 16];
        let res = Matrix::new_filled(4, 4, data);
        assert_eq!(res, m1 + &res);
    }

    #[test]
    fn test_add_operator2() {
        let m1 = Matrix::<f64>::new(4, 4);
        let data3 = vec![1.0; 16];
        let res3 = Matrix::new_filled(4, 4, data3);
        let data = vec![4.0; 16];
        let res = Matrix::new_filled(4, 4, data);
        let data2 = vec![5.0; 16];
        let res2 = Matrix::new_filled(4, 4, data2);
        assert_eq!(res2, res3 + res + m1);
    }

    #[test]
    fn test_add_operator3() {
        let m1 = Matrix::<f64>::new(4, 4);
        let data3 = vec![1.0; 16];
        let res3 = Matrix::new_filled(4, 4, data3);
        let data = vec![4.0; 16];
        let res = Matrix::new_filled(4, 4, data);
        let data2 = vec![5.0; 16];
        let res2 = Matrix::new_filled(4, 4, data2);
        assert_eq!(res2, &res3 + res + &m1);
    }

    #[test]
    fn test_add_operator4() {
        let m1 = Matrix::<f64>::new(4, 4);
        let data3 = vec![1.0; 16];
        let res3 = Matrix::new_filled(4, 4, data3);
        let data = vec![4.0; 16];
        let res = Matrix::new_filled(4, 4, data);
        let data2 = vec![5.0; 16];
        let res2 = Matrix::new_filled(4, 4, data2);
        assert_eq!(res2, &res3 + &res + &m1);
    }

    #[test]
    fn test_add_operator() {
        let T: f64 = 1.0;
        let data3 = vec![1.0; 16];
        let res3 = Matrix::new_filled(4, 4, data3);
        let data = vec![4.0; 16];
        let res = Matrix::new_filled(4, 4, data);
        let data2 = vec![5.0; 16];
        let res2 = Matrix::new_filled(4, 4, data2);
        assert_eq!(res2, &res3 + &res + &m1);
    }
}