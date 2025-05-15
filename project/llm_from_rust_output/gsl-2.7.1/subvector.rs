use std::marker::PhantomData;

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
    NoMem = 8,
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

pub type SizeT = usize;

#[derive(Debug, Clone)]
pub struct GslBlock<T> {
    size: SizeT,
    data: Vec<T>,
}

impl<T> GslBlock<T> {
    pub fn new(size: SizeT) -> Self {
        Self {
            size,
            data: Vec::with_capacity(size),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GslVector<T> {
    size: SizeT,
    stride: SizeT,
    data: Vec<T>,
    block: GslBlock<T>,
    owner: bool,
}

impl<T> GslVector<T> {
    pub fn new(size: SizeT) -> Self {
        let block = GslBlock::new(size);
        Self {
            size,
            stride: 1,
            data: Vec::with_capacity(size),
            block,
            owner: true,
        }
    }

    pub fn subvector(&mut self, offset: SizeT, n: SizeT) -> Result<GslVectorView<T>, GslError> {
        if offset + n.saturating_sub(1) >= self.size {
            return Err(GslError::Invalid);
        }

        let data_start = offset * self.stride;
        let data_end = data_start + n * self.stride;
        
        if data_end > self.data.len() {
            return Err(GslError::Invalid);
        }

        Ok(GslVectorView {
            vector: GslVector {
                size: n,
                stride: self.stride,
                data: self.data[data_start..data_end].to_vec(),
                block: self.block.clone(),
                owner: false,
            },
        })
    }

    pub fn subvector_with_stride(
        &mut self,
        offset: SizeT,
        stride: SizeT,
        n: SizeT,
    ) -> Result<GslVectorView<T>, GslError> {
        if stride == 0 {
            return Err(GslError::Invalid);
        }

        if offset + n.saturating_sub(1) * stride >= self.size {
            return Err(GslError::Invalid);
        }

        let data_start = offset * self.stride;
        let data_end = data_start + n * self.stride * stride;
        
        if data_end > self.data.len() {
            return Err(GslError::Invalid);
        }

        Ok(GslVectorView {
            vector: GslVector {
                size: n,
                stride: self.stride * stride,
                data: self.data[data_start..data_end].to_vec(),
                block: self.block.clone(),
                owner: false,
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct GslVectorView<T> {
    vector: GslVector<T>,
}

#[derive(Debug, Clone)]
pub struct GslVectorConstView<T> {
    vector: GslVector<T>,
}

impl<T> GslVector<T> {
    pub fn const_subvector(&self, offset: SizeT, n: SizeT) -> Result<GslVectorConstView<T>, GslError> {
        if offset + n.saturating_sub(1) >= self.size {
            return Err(GslError::Invalid);
        }

        let data_start = offset * self.stride;
        let data_end = data_start + n * self.stride;
        
        if data_end > self.data.len() {
            return Err(GslError::Invalid);
        }

        Ok(GslVectorConstView {
            vector: GslVector {
                size: n,
                stride: self.stride,
                data: self.data[data_start..data_end].to_vec(),
                block: self.block.clone(),
                owner: false,
            },
        })
    }

    pub fn const_subvector_with_stride(
        &self,
        offset: SizeT,
        stride: SizeT,
        n: SizeT,
    ) -> Result<GslVectorConstView<T>, GslError> {
        if stride == 0 {
            return Err(GslError::Invalid);
        }

        if offset + n.saturating_sub(1) * stride >= self.size {
            return Err(GslError::Invalid);
        }

        let data_start = offset * self.stride;
        let data_end = data_start + n * self.stride * stride;
        
        if data_end > self.data.len() {
            return Err(GslError::Invalid);
        }

        Ok(GslVectorConstView {
            vector: GslVector {
                size: n,
                stride: self.stride * stride,
                data: self.data[data_start..data_end].to_vec(),
                block: self.block.clone(),
                owner: false,
            },
        })
    }
}

// Implementations for all specific types
macro_rules! impl_vector_types {
    ($($type:ty),+) => {
        $(
            impl GslVector<$type> {
                pub fn new_specific(size: SizeT) -> Self {
                    Self::new(size)
                }
            }
        )+
    };
}

impl_vector_types!(
    f64, f32, i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, char
);

// Complex number support
#[derive(Debug, Clone, Copy)]
pub struct Complex<T> {
    re: T,
    im: T,
}

impl<T> GslVector<Complex<T>> {
    pub fn subvector_complex(&mut self, offset: SizeT, n: SizeT) -> Result<GslVectorView<Complex<T>>, GslError> {
        if offset + n.saturating_sub(1) >= self.size {
            return Err(GslError::Invalid);
        }

        let data_start = offset * self.stride * 2;
        let data_end = data_start + n * self.stride * 2;
        
        if data_end > self.data.len() {
            return Err(GslError::Invalid);
        }

        Ok(GslVectorView {
            vector: GslVector {
                size: n,
                stride: self.stride,
                data: self.data[data_start..data_end].to_vec(),
                block: self.block.clone(),
                owner: false,
            },
        })
    }

    pub fn subvector_complex_with_stride(
        &mut self,
        offset: SizeT,
        stride: SizeT,
        n: SizeT,
    ) -> Result<GslVectorView<Complex<T>>, GslError> {
        if stride == 0 {
            return Err(GslError::Invalid);
        }

        if offset + n.saturating_sub(1) * stride >= self.size {
            return Err(GslError::Invalid);
        }

        let data_start = offset * self.stride * 2;
        let data_end = data_start + n * self.stride * stride * 2;
        
        if data_end > self.data.len() {
            return Err(GslError::Invalid);
        }

        Ok(GslVectorView {
            vector: GslVector {
                size: n,
                stride: self.stride * stride,
                data: self.data[data_start..data_end].to_vec(),
                block: self.block.clone(),
                owner: false,
            },
        })
    }
}

impl<T> GslVector<Complex<T>> {
    pub fn const_subvector_complex(&self, offset: SizeT, n: SizeT) -> Result<GslVectorConstView<Complex<T>>, GslError> {
        if offset + n.saturating_sub(1) >= self.size {
            return Err(GslError::Invalid);
        }

        let data_start = offset * self.stride * 2;
        let data_end = data_start + n * self.stride * 2;
        
        if data_end > self.data.len() {
            return Err(GslError::Invalid);
        }

        Ok(GslVectorConstView {
            vector: GslVector {
                size: n,
                stride: self.stride,
                data: self.data[data_start..data_end].to_vec(),
                block: self.block.clone(),
                owner: false,
            },
        })
    }

    pub fn const_subvector_complex_with_stride(
        &self,
        offset: SizeT,
        stride: SizeT,
        n: SizeT,
    ) -> Result<GslVectorConstView<Complex<T>>, GslError> {
        if stride == 0 {
            return Err(GslError::Invalid);
        }

        if offset + n.saturating_sub(1) * stride >= self.size {
            return Err(GslError::Invalid);
        }

        let data_start = offset * self.stride * 2;
        let data_end = data_start + n * self.stride * stride * 2;
        
        if data_end > self.data.len() {
            return Err(GslError::Invalid);
        }

        Ok(GslVectorConstView {
            vector: GslVector {
                size: n,
                stride: self.stride * stride,
                data: self.data[data_start..data_end].to_vec(),
                block: self.block.clone(),
                owner: false,
            },
        })
    }
}