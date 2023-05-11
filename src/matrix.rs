// dependency
use std::{ops, fmt::Debug};

use crate::vec3::Vec3;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Matrix<T: Copy + Default> {
    data: Vec<T>,
    nbr_row: u8,
    nbr_col: u8,
    nbr_elm: u16,
}

impl Matrix<f32> {
    pub fn new_identity(nbr_col: u8, nbr_row: u8) -> Self {
        let nbr_elm = nbr_col as u16 * nbr_row as u16;
        let mut data = Vec::with_capacity(nbr_elm as usize);
        for nr in 0..nbr_row {
            for nc in 0..nbr_col{
                if nr == nc {
                    data.push(1.0);
                }
                else {
                    data.push(0.0);
                }
            }
        }
        Matrix { data, nbr_row, nbr_col, nbr_elm }
    }

    pub fn new_rot_by_x(nbr_col: u8, nbr_row: u8) -> Self {
        let nbr_elm = nbr_col as u16 * nbr_row as u16;
        let mut data = Vec::with_capacity(nbr_elm as usize);
        data
    }

}

impl Matrix<f64> {
    pub fn new_identity(nbr_col: u8, nbr_row: u8) -> Self {
        let nbr_elm = nbr_col as u16 * nbr_row as u16;
        let mut data = Vec::with_capacity(nbr_elm as usize);
        for nr in 0..nbr_row {
            for nc in 0..nbr_col{
                if nr == nc {
                    data.push(1.0);
                }
                else {
                    data.push(0.0);
                }
            }
        }
        Matrix { data, nbr_row, nbr_col, nbr_elm }
    }
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
        //data.truncate(nbr_elm as usize);
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

impl<T: Copy + Default + std::ops::Add<Output = T>> ops::Add<T> for &Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: T) -> Self::Output {
        let mut tmp_data = Vec::with_capacity(16);
        for i in 0..self.get_nbr_elm() {
            tmp_data.push(self.get_elm_linear(i as u16) + rhs);
        }
        Matrix::new_filled(self.get_nbr_col(), self.get_nbr_row(), tmp_data)
    }
}

impl<T: Copy + Default + std::ops::Add<Output = T>> ops::Add<&T> for Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: &T) -> Self::Output {
        let mut tmp_data = Vec::with_capacity(16);
        for i in 0..self.get_nbr_elm() {
            tmp_data.push(self.get_elm_linear(i as u16) + *rhs);
        }
        Matrix::new_filled(self.get_nbr_col(), self.get_nbr_row(), tmp_data)
    }
}

impl<T: Copy + Default + std::ops::Add<Output = T>> ops::Add<&T> for &Matrix<T> {
    type Output = Matrix<T>;

    fn add(self, rhs: &T) -> Self::Output {
        let mut tmp_data = Vec::with_capacity(16);
        for i in 0..self.get_nbr_elm() {
            tmp_data.push(self.get_elm_linear(i as u16) + *rhs);
        }
        Matrix::new_filled(self.get_nbr_col(), self.get_nbr_row(), tmp_data)
    }
}

impl<T: Copy + Default + Debug + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + std::ops::AddAssign> ops::Mul<Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        let l_col = self.get_nbr_col();
        let l_row = self.get_nbr_row();
        let r_col = rhs.get_nbr_col();
        let r_row = rhs.get_nbr_row();
        let mut tmp_data = Vec::with_capacity(l_row as usize * r_col as usize);
        if l_col == r_row {
            for lr in 0..l_row {
                for rc in 0..r_col {
                    let mut tmp_res: T = T::default();
                    for lc in 0..l_col {
                        let lli = lr * l_col + lc;
                        let rli = lc * r_col + rc;
                        tmp_res += self.get_elm_linear(lli as u16) * rhs.get_elm_linear(rli as u16);
                        eprintln!("{:?}", tmp_res);
                    }
                    tmp_data.push(tmp_res);
                    eprintln!("{:?}", tmp_data);
                }
            }
        }
        let m = Matrix::new_filled(self.get_nbr_col(), self.get_nbr_row(), tmp_data);
        eprintln!("{:?}", m);
        return m;
    }
}

impl<T: Copy + Default + Debug + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + std::ops::AddAssign> ops::Mul<Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        let l_col = self.get_nbr_col();
        let l_row = self.get_nbr_row();
        let r_col = rhs.get_nbr_col();
        let r_row = rhs.get_nbr_row();
        let mut tmp_data = Vec::with_capacity(l_row as usize * r_col as usize);
        if l_col == r_row {
            for lr in 0..l_row {
                for rc in 0..r_col {
                    let mut tmp_res: T = T::default();
                    for lc in 0..l_col {
                        let lli = lr * l_col + lc;
                        let rli = lc * r_col + rc;
                        tmp_res += self.get_elm_linear(lli as u16) * rhs.get_elm_linear(rli as u16);
                        eprintln!("{:?}", tmp_res);
                    }
                    tmp_data.push(tmp_res);
                    eprintln!("{:?}", tmp_data);
                }
            }
        }
        let m = Matrix::new_filled(self.get_nbr_col(), self.get_nbr_row(), tmp_data);
        eprintln!("{:?}", m);
        return m;
    }
}

impl<T: Copy + Default + Debug + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + std::ops::AddAssign> ops::Mul<&Matrix<T>> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        let l_col = self.get_nbr_col();
        let l_row = self.get_nbr_row();
        let r_col = rhs.get_nbr_col();
        let r_row = rhs.get_nbr_row();
        let mut tmp_data = Vec::with_capacity(l_row as usize * r_col as usize);
        if l_col == r_row {
            for lr in 0..l_row {
                for rc in 0..r_col {
                    let mut tmp_res: T = T::default();
                    for lc in 0..l_col {
                        let lli = lr * l_col + lc;
                        let rli = lc * r_col + rc;
                        tmp_res += self.get_elm_linear(lli as u16) * rhs.get_elm_linear(rli as u16);
                        eprintln!("{:?}", tmp_res);
                    }
                    tmp_data.push(tmp_res);
                    eprintln!("{:?}", tmp_data);
                }
            }
        }
        let m = Matrix::new_filled(self.get_nbr_col(), self.get_nbr_row(), tmp_data);
        eprintln!("{:?}", m);
        return m;
    }
}

impl<T: Copy + Default + Debug + std::ops::Add<Output = T> + std::ops::Mul<Output = T> + std::ops::AddAssign> ops::Mul<&Matrix<T>> for &Matrix<T> {
    type Output = Matrix<T>;

    fn mul(self, rhs: &Matrix<T>) -> Self::Output {
        let l_col = self.get_nbr_col();
        let l_row = self.get_nbr_row();
        let r_col = rhs.get_nbr_col();
        let r_row = rhs.get_nbr_row();
        let mut tmp_data = Vec::with_capacity(l_row as usize * r_col as usize);
        if l_col == r_row {
            for lr in 0..l_row {
                for rc in 0..r_col {
                    let mut tmp_res: T = T::default();
                    for lc in 0..l_col {
                        let lli = lr * l_col + lc;
                        let rli = lc * r_col + rc;
                        tmp_res += self.get_elm_linear(lli as u16) * rhs.get_elm_linear(rli as u16);
                        eprintln!("{:?}", tmp_res);
                    }
                    tmp_data.push(tmp_res);
                    eprintln!("{:?}", tmp_data);
                }
            }
        }
        let m = Matrix::new_filled(self.get_nbr_col(), self.get_nbr_row(), tmp_data);
        eprintln!("{:?}", m);
        return m;
    }
}

impl<T: Copy + Default + Debug + std::ops::Add<Output = T> + std::convert::From<T> + std::ops::Mul<f64, Output = T> + std::ops::Mul<Output = T> + std::ops::AddAssign> ops::Mul<Vec3> for Matrix<T> where f64: From<T> {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let l_col = self.get_nbr_col();
        let l_row = self.get_nbr_row();
        let r_row = 3;
        let mut tmp_data = Vec::with_capacity(3);
        if l_col == r_row {
            for lr in 0..l_row {
                let mut tmp_res: T = T::default();
                for lc in 0..l_col {
                    let lli = lr * l_col + lc;
                    eprintln!("multipliyin matrix : {:?} with vec {}, linear index : {}", self.get_elm_linear(lli as u16), rhs.p[lc as usize], lli);
                    tmp_res += self.get_elm_linear(lli as u16) * rhs.p[lc as usize];
                    eprintln!("{:?}", tmp_res);
                }
                tmp_data.push(tmp_res);
                eprintln!("{:?}", tmp_data);
            }
        }
        let v = Vec3::new(f64::from(tmp_data[0]), f64::from(tmp_data[1]), f64::from(tmp_data[2]));
        eprintln!("{:?}", v);
        return v;
    }
}

impl<T: Copy + Default + Debug + std::ops::Add<Output = T> + std::convert::From<T> + std::ops::Mul<f64, Output = T> + std::ops::Mul<Output = T> + std::ops::AddAssign> ops::Mul<Vec3> for &Matrix<T> where f64: From<T> {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        let l_col = self.get_nbr_col();
        let l_row = self.get_nbr_row();
        let r_col = 1;
        let r_row = 3;
        let mut tmp_data = Vec::with_capacity(l_row as usize * r_col as usize);
        if l_col == r_row {
            for lr in 0..l_row {
                let mut tmp_res: T = T::default();
                for lc in 0..l_col {
                    let lli = lr * l_col + lc;
                    tmp_res += self.get_elm_linear(lli as u16) * rhs.p[lc as usize];
                    eprintln!("{:?}", tmp_res);
                }
                tmp_data.push(tmp_res);
                eprintln!("{:?}", tmp_data);
            }
        }
        let v = Vec3::new(f64::from(tmp_data[0]), f64::from(tmp_data[1]), f64::from(tmp_data[2]));
        eprintln!("{:?}", v);
        return v;
    }
}

impl<T: Copy + Default + Debug + std::ops::Add<Output = T> + std::convert::From<T> + std::ops::Mul<f64, Output = T> + std::ops::Mul<Output = T> + std::ops::AddAssign> ops::Mul<&Vec3> for Matrix<T> where f64: From<T> {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        let l_col = self.get_nbr_col();
        let l_row = self.get_nbr_row();
        let r_col = 1;
        let r_row = 3;
        let mut tmp_data = Vec::with_capacity(l_row as usize * r_col as usize);
        if l_col == r_row {
            for lr in 0..l_row {
                let mut tmp_res: T = T::default();
                for lc in 0..l_col {
                    let lli = lr * l_col + lc;
                    tmp_res += self.get_elm_linear(lli as u16) * rhs.p[lc as usize];
                    eprintln!("{:?}", tmp_res);
                }
                tmp_data.push(tmp_res);
                eprintln!("{:?}", tmp_data);
            }
        }
        let v = Vec3::new(f64::from(tmp_data[0]), f64::from(tmp_data[1]), f64::from(tmp_data[2]));
        eprintln!("{:?}", v);
        return v;
    }
}

impl<T: Copy + Default + Debug + std::ops::Add<Output = T> + std::convert::From<T> + std::ops::Mul<f64, Output = T> + std::ops::Mul<Output = T> + std::ops::AddAssign> ops::Mul<&Vec3> for &Matrix<T> where f64: From<T> {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        let l_col = self.get_nbr_col();
        let l_row = self.get_nbr_row();
        let r_col = 1;
        let r_row = 3;
        let mut tmp_data = Vec::with_capacity(l_row as usize * r_col as usize);
        if l_col == r_row {
            for lr in 0..l_row {
                let mut tmp_res: T = T::default();
                for lc in 0..l_col {
                    let lli = lr * l_col + lc;
                    tmp_res += self.get_elm_linear(lli as u16) * rhs.p[lc as usize];
                    eprintln!("{:?}", tmp_res);
                }
                tmp_data.push(tmp_res);
                eprintln!("{:?}", tmp_data);
            }
        }
        let v = Vec3::new(f64::from(tmp_data[0]), f64::from(tmp_data[1]), f64::from(tmp_data[2]));
        eprintln!("{:?}", v);
        return v;
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
    fn test_add_operator5() {
        let t: f64 = 1.0;
        let data = vec![4.0; 16];
        let res = Matrix::new_filled(4, 4, data);
        let data2 = vec![5.0; 16];
        let res2 = Matrix::new_filled(4, 4, data2);
        assert_eq!(res2, res + t);
    }

    #[test]
    fn test_add_operator6() {
        let t: f64 = 1.0;
        let data = vec![4.0; 16];
        let res = Matrix::new_filled(4, 4, data);
        let data2 = vec![5.0; 16];
        let res2 = Matrix::new_filled(4, 4, data2);
        assert_eq!(res2, &res + t);
    }

    #[test]
    fn test_add_operator7() {
        let t: f64 = 1.0;
        let data = vec![4.0; 16];
        let res = Matrix::new_filled(4, 4, data);
        let data2 = vec![5.0; 16];
        let res2 = Matrix::new_filled(4, 4, data2);
        assert_eq!(res2, res + &t);
    }

    #[test]
    fn test_add_operator8() {
        let t: f64 = 1.0;
        let data = vec![4.0; 16];
        let res = Matrix::new_filled(4, 4, data);
        let data2 = vec![5.0; 16];
        let res2 = Matrix::new_filled(4, 4, data2);
        assert_eq!(res2, &res + &t);
    }

    #[test]
    fn test_mul_operator1() {
        let mut data = Vec::with_capacity(4);
        data.push(1.0);
        data.push(0.0);
        data.push(0.0);
        data.push(1.0);
        let res = Matrix::new_filled(2, 2, data);
        let data2 = vec![5.0; 4];
        let res2 = Matrix::new_filled(2, 2, data2);
        let data3 = vec![5.0; 4];
        let res3 = Matrix::new_filled(2, 2, data3);
        assert_eq!(res3, res * res2);
    }
    #[test]
    fn test_mul_operator2() {
        let mut data = Vec::with_capacity(4);
        data.push(1.0);
        data.push(0.0);
        data.push(0.0);
        data.push(1.0);
        let res = Matrix::new_filled(2, 2, data);
        let data2 = vec![5.0; 4];
        let res2 = Matrix::new_filled(2, 2, data2);
        let data3 = vec![5.0; 4];
        let res3 = Matrix::new_filled(2, 2, data3);
        assert_eq!(res3, &res * res2);
    }
    #[test]
    fn test_mul_operator3() {
        let mut data = Vec::with_capacity(4);
        data.push(1.0);
        data.push(0.0);
        data.push(0.0);
        data.push(1.0);
        let res = Matrix::new_filled(2, 2, data);
        let data2 = vec![5.0; 4];
        let res2 = Matrix::new_filled(2, 2, data2);
        let data3 = vec![5.0; 4];
        let res3 = Matrix::new_filled(2, 2, data3);
        assert_eq!(res3, res * &res2);
    }
    #[test]
    fn test_mul_operator5() {
        let mut data = Vec::with_capacity(9);
        data.push(1.0);
        data.push(0.0);
        data.push(0.0);
        data.push(0.0);
        data.push(1.0);
        data.push(0.0);
        data.push(0.0);
        data.push(0.0);
        data.push(1.0);
        let res = Matrix::new_filled(3, 3, data);
        let vec1 = Vec3::new(4.0, 5.0, 2.0);
        let vec2 = Vec3::new(4.0, 5.0, 2.0);
        assert_eq!(vec2, res * vec1);
    }
    #[test]
    fn test_mul_operator6() {
        let mut data = Vec::with_capacity(9);
        data.push(1.0);
        data.push(0.0);
        data.push(0.0);
        data.push(0.0);
        data.push(1.0);
        data.push(0.0);
        data.push(0.0);
        data.push(0.0);
        data.push(1.0);
        let res = Matrix::new_filled(3, 3, data);
        let vec1 = Vec3::new(4.0, 5.0, 2.0);
        let vec2 = Vec3::new(4.0, 5.0, 2.0);
        assert_eq!(vec2, &res * vec1);
    }
    #[test]
    fn test_mul_operator7() {
        let mut data = Vec::with_capacity(9);
        data.push(1.0);
        data.push(0.0);
        data.push(0.0);
        data.push(0.0);
        data.push(1.0);
        data.push(0.0);
        data.push(0.0);
        data.push(0.0);
        data.push(1.0);
        let res = Matrix::new_filled(3, 3, data);
        let vec1 = Vec3::new(4.0, 5.0, 2.0);
        let vec2 = Vec3::new(4.0, 5.0, 2.0);
        assert_eq!(vec2, res * &vec1);
    }
    #[test]
    fn test_mul_operator8() {
        let mut data = Vec::with_capacity(9);
        data.push(1.0);
        data.push(0.0);
        data.push(0.0);
        data.push(0.0);
        data.push(1.0);
        data.push(0.0);
        data.push(0.0);
        data.push(0.0);
        data.push(1.0);
        let res = Matrix::new_filled(3, 3, data);
        let vec1 = Vec3::new(4.0, 5.0, 2.0);
        let vec2 = Vec3::new(4.0, 5.0, 2.0);
        assert_eq!(vec2, &res * &vec1);
    }
}