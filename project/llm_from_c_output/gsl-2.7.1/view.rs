// Constants for null vectors and matrices
pub const NULL_VECTOR: [i32; 5] = [0, 0, 0, 0, 0];
pub const NULL_VECTOR_VIEW: [[i32; 5]; 1] = [[0, 0, 0, 0, 0]];

pub const NULL_MATRIX: [i32; 6] = [0, 0, 0, 0, 0, 0];
pub const NULL_MATRIX_VIEW: [[i32; 6]; 1] = [[0, 0, 0, 0, 0, 0]];

// Matrix view functionality using Rust generics
pub mod view {
    use std::marker::PhantomData;

    pub trait NumericType: Clone + Copy {}
    impl NumericType for f32 {}
    impl NumericType for f64 {}
    impl NumericType for i8 {}
    impl NumericType for i16 {}
    impl NumericType for i32 {}
    impl NumericType for i64 {}
    impl NumericType for u8 {}
    impl NumericType for u16 {}
    impl NumericType for u32 {}
    impl NumericType for u64 {}

    pub struct MatrixView<'a, T: NumericType> {
        data: &'a [T],
        rows: usize,
        cols: usize,
        phantom: PhantomData<T>,
    }

    impl<'a, T: NumericType> MatrixView<'a, T> {
        pub fn new(data: &'a [T], rows: usize, cols: usize) -> Result<Self, &'static str> {
            if data.len() < rows * cols {
                return Err("Insufficient data for matrix dimensions");
            }
            Ok(Self {
                data,
                rows,
                cols,
                phantom: PhantomData,
            })
        }

        pub fn get(&self, row: usize, col: usize) -> Option<T> {
            if row >= self.rows || col >= self.cols {
                return None;
            }
            Some(self.data[row * self.cols + col])
        }
    }

    pub struct ConstMatrixView<'a, T: NumericType> {
        data: &'a [T],
        rows: usize,
        cols: usize,
        phantom: PhantomData<T>,
    }

    impl<'a, T: NumericType> ConstMatrixView<'a, T> {
        pub fn new(data: &'a [T], rows: usize, cols: usize) -> Result<Self, &'static str> {
            if data.len() < rows * cols {
                return Err("Insufficient data for matrix dimensions");
            }
            Ok(Self {
                data,
                rows,
                cols,
                phantom: PhantomData,
            })
        }

        pub fn get(&self, row: usize, col: usize) -> Option<T> {
            if row >= self.rows || col >= self.cols {
                return None;
            }
            Some(self.data[row * self.cols + col])
        }
    }
}

// Complex number support could be added using external crates like num-complex
// or implementing custom complex types if needed