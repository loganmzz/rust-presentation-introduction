extern crate rayon;

const MUL_PAR_THREHOLD: usize = 32768;

#[derive(Clone,PartialEq,Eq)]
pub struct Matrix {
    data: Vec<i32>,
    cols: usize,
    rows: usize,
}

impl Matrix {
    pub fn new(cols: usize, rows: usize) -> Matrix {
        let data = vec![0; cols * rows]; 
        Matrix { data, cols, rows }
    }

    pub fn identity(size: usize) -> Matrix {
        let mut m = Matrix::new(size, size);
        for i in 0..size {
            m[(i, i)] = 1;
        }
        m
    }

    pub fn from_vec(data: Vec<Vec<i32>>) -> Matrix {
        let rows = data.len();
        if rows == 0 {
            panic!("Matrix must contain at least one row");
        }
        
        let cols = data.iter().map(|row| row.len()).max().expect("Fatal error");
        if cols == 0 {
            panic!("Matrix must contain at least one column");
        }

        if let Some((row_size, row)) = data.iter().map(|row| row.len()).zip(0..data.len()).filter(|tuple| tuple.0 != cols).next() {
            panic!("Invalid row {}. Contains {} columns instead of {}", row, row_size, cols);
        }

        let data = data.into_iter().flat_map(|row| row).collect();

        Matrix { data, cols, rows }
    }

    fn to_index(&self, (col, row): (usize, usize)) -> usize {
        if col >= self.cols || row >= self.rows {
            panic!("Invalid index ({}, {})", col, row);
        }
        col + self.cols * row
    }

    fn from_index(cols: usize, index: usize) -> (usize,usize) {
        (index % cols, index / cols)
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }
}

impl std::ops::Index<(usize,usize)> for Matrix {
    type Output = i32;

    fn index(&self, coord: (usize,usize)) -> &i32 {
        &self.data[self.to_index(coord)]
    }
}

impl std::ops::IndexMut<(usize,usize)> for Matrix {
    fn index_mut(&mut self, coord: (usize,usize)) -> &mut i32 {
        let index = self.to_index(coord);
        self.data.index_mut(index)
    }
}

impl std::fmt::Debug for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Matrix({}x{}) {:?}", self.cols, self.rows, self.data)
    }
}

impl<'a, 'b> std::ops::Add<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn add(self, rhs: &'b Matrix) -> Matrix {
        if self.cols != rhs.cols || self.rows != rhs.rows {
            panic!("Incompatible matrix ({},{})+({},{})", self.cols, self.rows, rhs.cols, rhs.rows);
        }
        let data = self.data.iter().zip(rhs.data.iter())
                                   .map(|(lh,rh)| lh+rh)
                                   .collect();
        Matrix { data, cols: self.cols, rows: self.rows }
    }
}

impl Matrix {
    fn add_seq(lhs: &[i32], rhs: &[i32], index: usize, data: &mut [i32]) {
        for i in 0..data.len() {
            data[i] = lhs[index+i] + rhs[index+i]
        }
    }

    fn add_par_iter(lhs: &[i32], rhs: &[i32], index: usize, data: &mut [i32]) {
        if data.len() <= 4*1024 {
            Self::add_seq(lhs, rhs, index, data);
        } else {
            let split = data.len()/2;
            let (left, right) = data.split_at_mut(split);
            rayon::join(|| Self::add_par_iter(lhs, rhs, index        , left),
                        || Self::add_par_iter(lhs, rhs, index + split, right)
            );
        }
    }

    pub fn add_par(&self, rhs: &Matrix) -> Matrix {
        if self.cols != rhs.cols || self.rows != rhs.rows {
            panic!("Incompatible matrix ({},{})+({},{})", self.cols, self.rows, rhs.cols, rhs.rows);
        }
        let mut data = vec![0; self.data.len()];
        Self::add_par_iter(&self.data, &rhs.data, 0, &mut data);
        Matrix { data, cols: self.cols, rows: self.rows }
    }
}

impl<'a, 'b> std::ops::Mul<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'b Matrix) -> Matrix {
        if self.cols() != rhs.rows() {
            panic!("Incompatible matrix ({},{})x({},{})", self.cols(), self.rows, rhs.cols(), rhs.rows());
        }
        let cols = rhs.cols();
        let rows = self.rows();
        let mut data = vec![0; cols * rows];
        for i in 0..data.len() {
            data[i] = Matrix::mul_cell(self, rhs, i);
        }
        Matrix { data, cols, rows }
    }

    
}

impl Matrix {

    fn mul_cell(lhs: &Matrix, rhs: &Matrix, index: usize) -> i32 {
        let (col, row) = Matrix::from_index(rhs.cols(), index);
        (0..lhs.cols()).map(|i| lhs[(i, row)] * rhs[(col, i)]).sum()
    }

    fn mul_cells(lhs: &Matrix, rhs: &Matrix, index: usize, data: &mut [i32]) {
        for i in 0..data.len() {
            data[i] = Self::mul_cell(lhs, rhs, index + i);
        }
    }

    fn mul_par_cells(lhs: &Matrix, rhs: &Matrix, index: usize, data: &mut [i32]) {
        if data.len() > MUL_PAR_THREHOLD {
            let split = data.len() / 2;
            let (head, tail) = data.split_at_mut(split);
            rayon::join(|| Self::mul_par_cells(lhs, rhs, index        , head),
                        || Self::mul_par_cells(lhs, rhs, index + split, tail)
            );
        } else {
            Self::mul_cells(lhs, rhs, index, data);
        }
    }

    pub fn mul_par(&self, rhs: &Matrix) -> Matrix {
        if self.cols() != rhs.rows() {
            panic!("Incompatible matrix ({},{})x({},{})", self.cols(), self.rows, rhs.cols(), rhs.rows());
        }
        let cols = rhs.cols();
        let rows = self.rows();
        let mut data = vec![0; cols * rows];
        Self::mul_par_cells(self, rhs, 0, &mut data);
        Matrix { data, cols, rows }
    }
}

#[cfg(test)]
mod tests;
