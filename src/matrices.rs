
use std::f64;
use std::cmp;


/// Vector is just Rust Vec
//#[derive(Debug, PartialEq)]
pub type Vector = Vec<f64>;

/// 2 dimensional matrix
#[derive(Debug, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>
}

impl Matrix {
    /// Create new matrix
    pub fn new(data: Vec<Vec<f64>>) -> Option<Matrix> {
        // Validate input
        if data.len() < 1 { return None }
        let cols = data[0].len();
        if data.iter().any(|r| r.len() != cols) { return None }

        Some(Matrix { rows: data.len(), cols: cols, data: data })
    }

//    pub fn rows(&self) -> usize { self.rows }
    pub fn cols(&self) -> usize { self.cols }

    /// Positive matrix has all its entries >= 0
    pub fn is_positive(&self) -> bool {
        let has_neg = self.data.iter().any(|r| r.iter().any(|&item| item < 0.));
        !has_neg
    }

    /// Apply -log2 for each element
    // Maybe it would be better to add function apply() with closure instead?
    pub fn minus_log(&self) -> Matrix {
        Matrix { rows: self.rows,
                 cols: self.cols,
                 data: self.data.iter().map(|r| r.iter().map(|i| -i.log2()).collect()).collect()}
    }

    /// Get copy of a given column
    pub fn column(&self, index: usize) -> Option<Vector> {
        // Validate input
        if index >= self.cols { return None }

        Some(self.data.iter().map(|r| r[index]).collect())
    }

    /// Add vector to each column
    pub fn add_to_columns(&self, v: &Vector) -> Matrix {
        let n = cmp::min(self.rows, v.len());
        Matrix { rows: self.rows,
                 cols: self.cols,
                 data: self.data.iter().map(|r| r.iter().map(|i| -i).collect()).collect()}
    }
}


/// Ad 2 vectors. If the have different size, they output vector will have minimal size
pub fn add_vectors(v: &Vector, u: &Vector) -> Vector {
    v.iter().zip(u).map(|(x,y)| x+y).collect()
}

/// Ad scalar to the vector.
//pub fn add_scalar(v: &Vector, u: &Vector) -> Vector {
//    v.iter().zip(u).map(|(x,y)| x+y).collect()
//}



/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let d = vec![vec![0.75, 0.25],
                     vec![0.25, 0.75]];
        assert!(Matrix::new(d).is_some());
    }

    #[test]
    fn test_new_none1() {
        let d = vec![];
        assert!(Matrix::new(d).is_none());
    }

    #[test]
    fn test_none2() {
        let d = vec![vec![0.75, 0.25],
                     vec![0.25, 0.75, 1.]];
        assert!(Matrix::new(d).is_none());
    }

    #[test]
    fn test_positive() {
        let mat = Matrix::new(vec![vec![0.75, 0.25],
                                   vec![0.25, 0.75]]).unwrap();
        assert!(mat.is_positive());
    }

    #[test]
    fn test_negative() {
        let mat = Matrix::new(vec![vec![0.75, 0.25],
                                   vec![0.25, -0.75]]).unwrap();
        assert!(mat.is_positive() == false);
    }

    #[test]
    fn test_get_column() {
        let mat = Matrix::new(vec![vec![1., 2.],
                                   vec![3., 4.]]).unwrap();
        assert!(mat.column(1) == Some(vec![2., 4.]));
    }

    #[test]
    fn test_add_vectors() {
        let v = vec![1., 2.];
        let u = vec![3., 4.];
        assert!(add_vectors(&v, &u) == vec![4., 6.]);
    }

    #[test]
    fn test_add_to_column() {
        let mat = Matrix::new(vec![vec![1., 2.],
                                   vec![3., 4.]]).unwrap();
        let v = vec![3., 4.];
        let expected = Matrix::new(vec![vec![4., 5.],
                                        vec![7., 8.]]).unwrap();
        assert!(mat.add_to_columns(&v) == expected);
    }

}