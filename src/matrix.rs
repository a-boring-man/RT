
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Matrix<T: Copy + Default> {
    data: Vec<T>,
    nbr_row: u8,
    nbr_col: u8,
    nbr_elm: u16,
}

impl<T: Copy + Default> Matrix<T> {

    pub fn default() -> Self {
        Matrix {
            data: Vec::with_capacity(16),
            nbr_col: Default::default(),
            nbr_row: Default::default(),
            nbr_elm: Default::default(),
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

    pub fn set_elm(&mut self, row: u8, col: u8, elm: T) {
        self.data[Matrix::<T>::linear_index_conversion(col, row) as usize] = elm;
    }

    pub fn get_nbr_row(&self) -> u8 {
        self.nbr_row
    }

    pub fn get_nbr_col(&self) -> u8 {
        self.nbr_col
    }

    pub fn linear_index_conversion(col: u8, row: u8) -> u16 {
        col as u16 * row as u16 + col as u16
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
}