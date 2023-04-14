
#[derive(Copy, Clone, Default, Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    nbr_row: u8,
    nbr_col: u8,
    nbr_elm: u16,
}

impl<T> Matrix<T> {

    pub fn default() -> Self {
        Matrix {
            data: Vec::with_capacity(16),
            nbr_col: Default::default(),
            nbr_row: Default::default(),
            nbr_elm: Default::default(),
        }
    }

    pub fn new(nbr_col: u8, nbr_row: u8) -> Self {
        Matrix {
            nbr_elm: nbr_col * nbr_row,
            data: Vec::with_capacity(nbr_elm),
            ..
        }
    }
}