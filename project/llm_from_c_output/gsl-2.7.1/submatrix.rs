// Note: The original C code is using GSL (GNU Scientific Library) templates
// to generate matrix operations for various numeric types. In Rust, we can
// use generics to achieve similar functionality in a type-safe way.

// For complex number support, we'll use the num-complex crate
use num_complex::{Complex32, Complex64};

// We'll define a generic MatrixView struct that can work with different numeric types
pub struct MatrixView<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
    stride: usize,
}

impl<T> MatrixView<T> {
    // Create a new matrix view
    pub fn new(data: Vec<T>, rows: usize, cols: usize, stride: usize) -> Self {
        Self {
            data,
            rows,
            cols,
            stride,
        }
    }

    // Get submatrix view
    pub fn submatrix(&self, row_offset: usize, col_offset: usize, sub_rows: usize, sub_cols: usize) -> Self {
        assert!(row_offset + sub_rows <= self.rows);
        assert!(col_offset + sub_cols <= self.cols);
        
        let start = row_offset * self.stride + col_offset;
        let new_data = self.data[start..].to_vec();
        
        Self {
            data: new_data,
            rows: sub_rows,
            cols: sub_cols,
            stride: self.stride,
        }
    }
}

// Implement specialized versions for common numeric types
impl MatrixView<f32> {
    pub fn new_f32(data: Vec<f32>, rows: usize, cols: usize, stride: usize) -> Self {
        Self::new(data, rows, cols, stride)
    }
}

impl MatrixView<f64> {
    pub fn new_f64(data: Vec<f64>, rows: usize, cols: usize, stride: usize) -> Self {
        Self::new(data, rows, cols, stride)
    }
}

impl MatrixView<Complex32> {
    pub fn new_complex32(data: Vec<Complex32>, rows: usize, cols: usize, stride: usize) -> Self {
        Self::new(data, rows, cols, stride)
    }
}

impl MatrixView<Complex64> {
    pub fn new_complex64(data: Vec<Complex64>, rows: usize, cols: usize, stride: usize) -> Self {
        Self::new(data, rows, cols, stride)
    }
}

// Similarly implement for other numeric types (i8, i16, i32, i64, u8, u16, u32, u64)

// Const versions can be handled by Rust's borrowing system
impl<'a, T> MatrixView<&'a T> {
    pub fn new_const(data: &'a [T], rows: usize, cols: usize, stride: usize) -> Self {
        Self {
            data: data.to_vec(),
            rows,
            cols,
            stride,
        }
    }
}