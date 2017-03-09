
use std::f64;
use std::cmp;


/// Vector is just Rust Vec
#[derive(Debug, PartialEq, Clone)]
pub struct Vector {
    data: Vec<f64>
}

/// 2 dimensional matrix
#[derive(Debug, PartialEq)]
pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>
}


impl Vector {

    /// Vector constructor
    pub fn new(v: Vec<f64>) -> Vector {
        Vector { data: v }
    }

    /// Vector length
    pub fn len(&self) -> usize { self.data.len() }

    /// Item at given index
    pub fn get(&self, i: usize) -> f64 { self.data[i] }

    /// Positive matrix has all its entries >= 0
    pub fn is_positive(&self) -> bool {
        let has_neg = self.data.iter().any(|&item| item < 0.);
        !has_neg
    }

    /// Ad 2 vectors. If the have different size, they output vector will have minimal size
    pub fn add_vector(&self, u: &Vector) -> Vector {
        let data = self.data.iter().zip(&u.data).map(|(x, y)| x + y).collect();
        Vector::new(data)
    }

    /// Ad scalar to the vector.
    pub fn add_scalar(&self, u: &Vector) -> Vector {
        let data = self.data.iter().zip(&u.data).map(|(x,y)| x+y).collect();
        Vector::new(data)
    }

    /// Apply -log2 for each element
    pub fn minus_log(&self) -> Vector {
        Vector::new(self.data.iter().map(|i| -i.log2()).collect())
    }

    /// get index with minimal value
    pub fn argmin(&self) -> usize {
        let mut min_index = 0;
        let mut min_value = self.data[0];

        for (i,v) in self.data.iter().enumerate() {
            if v < &min_value {
                min_value = v.clone();
                min_index = i.clone();
            }
        }
        min_index
    }
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
        let data = self.data.iter().map(|r| r[index]).collect();
        Some(Vector::new(data))
    }

    /// Add vector to each column
    pub fn add_to_columns(&self, v: &Vector) -> Matrix {
        let n = cmp::min(self.rows, v.len());
        let mut data = self.data.clone();

        for i in 0..n {
            for j in 0..self.cols {
                data[i][j] += v.get(i)
            }
        }
        Matrix { rows: self.rows,
                 cols: self.cols,
                 data: data}
    }

    /// Maximum by column
    pub fn min_by_column(&self) -> Vector {
        let mut v = vec![f64::MAX; self.cols];
        for row in &self.data {
            for (i, &item) in row.iter().enumerate() {
                if item < v[i] { v[i] = item}
            }
        }
        Vector::new(v)
    }

    /// Argmax by column
    pub fn argmin_by_column(&self) -> Vec<usize> {
        let mut v = vec![f64::MAX; self.cols];
        let mut args = vec![0; self.cols];

        for (i, ref row) in self.data.iter().enumerate() {
            for (j, &item) in row.iter().enumerate() {
                if item < v[j] {
                    v[j] = item;
                    args[j] = i;
                }
            }
        }
        args
    }
}



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
        let expected = Vector::new(vec![2., 4.]);
        assert!(mat.column(1) == Some(expected));
    }

    #[test]
    fn test_add_vectors() {
        let v = Vector::new(vec![1., 2.]);
        let u = Vector::new(vec![3., 4.]);
        let expected = Vector::new(vec![4., 6.]);
        assert!(v.add_vector(&u) == expected);
    }

    #[test]
    fn test_add_to_column() {
        let mat = Matrix::new(vec![vec![1., 2.],
                                   vec![3., 4.]]).unwrap();
        let v = Vector::new(vec![3., 4.]);
        let expected = Matrix::new(vec![vec![4., 5.],
                                        vec![7., 8.]]).unwrap();
        assert!(mat.add_to_columns(&v) == expected);
    }

    #[test]
    fn test_min_by_column() {
        let mat = Matrix::new(vec![vec![1., 4.],
                                   vec![3., 2.]]).unwrap();
        let expected = Vector::new(vec![1., 2.]);
        assert!(mat.min_by_column() == expected);
    }

    #[test]
    fn test_argmin_by_column() {
        let mat = Matrix::new(vec![vec![1., 4.],
                                   vec![3., 2.]]).unwrap();
        let expected = vec![0, 1];
        assert!(mat.argmin_by_column() == expected);
    }

    #[test]
    fn test_argmin() {
        let v = Vector::new(vec![1., 4., 0.34, 12.]);
        assert!(v.argmin() == 2);
    }

}