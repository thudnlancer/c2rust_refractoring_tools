use num_traits::{Float, Num, NumCast, ToPrimitive};
use std::cmp::min;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

// Define error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Domain = 1,
    Range = 2,
    Fault = 3,
    Invalid = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    NoMemory = 8,
    BadFunc = 9,
    Runaway = 10,
    MaxIter = 11,
    ZeroDiv = 12,
    BadTol = 13,
    Tol = 14,
    Underflow = 15,
    Overflow = 16,
    Loss = 17,
    Round = 18,
    BadLen = 19,
    NotSquare = 20,
    Singular = 21,
    Diverge = 22,
    Unsupported = 23,
    Unimplemented = 24,
    Cache = 25,
    Table = 26,
    NoProgress = 27,
    NoProgressJ = 28,
    TolF = 29,
    TolX = 30,
    TolG = 31,
    Eof = 32,
}

// Complex number type
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex<T> {
    pub re: T,
    pub im: T,
}

impl<T: Float> Complex<T> {
    pub fn new(re: T, im: T) -> Self {
        Self { re, im }
    }

    pub fn scale(&self, factor: T) -> Self {
        Self {
            re: self.re * factor,
            im: self.im * factor,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    pub fn div(&self, other: &Self) -> Self {
        let s = (other.re.powi(2) + (other.im.powi(2)).recip();
        let re = (self.re * other.re + self.im * other.im) * s;
        let im = (self.im * other.re - self.re * other.im) * s;
        Self { re, im }
    }
}

// Matrix type
#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: Vec<T>,
}

impl<T: Copy> Matrix<T> {
    pub fn new(size1: usize, size2: usize) -> Self {
        let tda = size2;
        let data = vec![T::default(); size1 * size2];
        Self {
            size1,
            size2,
            tda,
            data,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> T {
        self.data[i * self.tda + j]
    }

    pub fn set(&mut self, i: usize, j: usize, value: T) {
        self.data[i * self.tda + j] = value;
    }
}

impl<T: Num + Copy> Matrix<T> {
    pub fn add(&mut self, other: &Self) -> Result<(), GslError> {
        if self.size1 != other.size1 || self.size2 != other.size2 {
            return Err(GslError::BadLen);
        }

        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                self.data[idx] = self.data[idx] + other.data[idx];
            }
        }
        Ok(())
    }

    pub fn sub(&mut self, other: &Self) -> Result<(), GslError> {
        if self.size1 != other.size1 || self.size2 != other.size2 {
            return Err(GslError::BadLen);
        }

        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                self.data[idx] = self.data[idx] - other.data[idx];
            }
        }
        Ok(())
    }

    pub fn mul_elements(&mut self, other: &Self) -> Result<(), GslError> {
        if self.size1 != other.size1 || self.size2 != other.size2 {
            return Err(GslError::BadLen);
        }

        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                self.data[idx] = self.data[idx] * other.data[idx];
            }
        }
        Ok(())
    }

    pub fn div_elements(&mut self, other: &Self) -> Result<(), GslError> {
        if self.size1 != other.size1 || self.size2 != other.size2 {
            return Err(GslError::BadLen);
        }

        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                self.data[idx] = self.data[idx] / other.data[idx];
            }
        }
        Ok(())
    }

    pub fn scale(&mut self, x: T) -> Result<(), GslError> {
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                self.data[idx] = self.data[idx] * x;
            }
        }
        Ok(())
    }

    pub fn add_constant(&mut self, x: T) -> Result<(), GslError> {
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                self.data[idx] = self.data[idx] + x;
            }
        }
        Ok(())
    }

    pub fn add_diagonal(&mut self, x: T) -> Result<(), GslError> {
        let loop_lim = min(self.size1, self.size2);
        for i in 0..loop_lim {
            let idx = i * self.tda + i;
            self.data[idx] = self.data[idx] + x;
        }
        Ok(())
    }
}

// Complex matrix type
#[derive(Debug, Clone)]
pub struct ComplexMatrix<T> {
    pub size1: usize,
    pub size2: usize,
    pub tda: usize,
    pub data: Vec<Complex<T>>,
}

impl<T: Float> ComplexMatrix<T> {
    pub fn new(size1: usize, size2: usize) -> Self {
        let tda = size2;
        let data = vec![Complex::new(T::zero(), T::zero()); size1 * size2];
        Self {
            size1,
            size2,
            tda,
            data,
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Complex<T> {
        self.data[i * self.tda + j]
    }

    pub fn set(&mut self, i: usize, j: usize, value: Complex<T>) {
        self.data[i * self.tda + j] = value;
    }

    pub fn add(&mut self, other: &Self) -> Result<(), GslError> {
        if self.size1 != other.size1 || self.size2 != other.size2 {
            return Err(GslError::BadLen);
        }

        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                self.data[idx] = self.data[idx].add(&other.data[idx]);
            }
        }
        Ok(())
    }

    pub fn sub(&mut self, other: &Self) -> Result<(), GslError> {
        if self.size1 != other.size1 || self.size2 != other.size2 {
            return Err(GslError::BadLen);
        }

        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                self.data[idx] = self.data[idx].sub(&other.data[idx]);
            }
        }
        Ok(())
    }

    pub fn mul_elements(&mut self, other: &Self) -> Result<(), GslError> {
        if self.size1 != other.size1 || self.size2 != other.size2 {
            return Err(GslError::BadLen);
        }

        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                self.data[idx] = self.data[idx].mul(&other.data[idx]);
            }
        }
        Ok(())
    }

    pub fn div_elements(&mut self, other: &Self) -> Result<(), GslError> {
        if self.size1 != other.size1 || self.size2 != other.size2 {
            return Err(GslError::BadLen);
        }

        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                self.data[idx] = self.data[idx].div(&other.data[idx]);
            }
        }
        Ok(())
    }

    pub fn scale(&mut self, x: Complex<T>) -> Result<(), GslError> {
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                self.data[idx] = self.data[idx].mul(&x);
            }
        }
        Ok(())
    }

    pub fn add_constant(&mut self, x: Complex<T>) -> Result<(), GslError> {
        for i in 0..self.size1 {
            for j in 0..self.size2 {
                let idx = i * self.tda + j;
                self.data[idx] = self.data[idx].add(&x);
            }
        }
        Ok(())
    }

    pub fn add_diagonal(&mut self, x: Complex<T>) -> Result<(), GslError> {
        let loop_lim = min(self.size1, self.size2);
        for i in 0..loop_lim {
            let idx = i * self.tda + i;
            self.data[idx] = self.data[idx].add(&x);
        }
        Ok(())
    }
}