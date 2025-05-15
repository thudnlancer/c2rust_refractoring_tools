use std::cmp::Ordering;

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

pub trait GslSortable: PartialOrd + Copy {}
impl GslSortable for f64 {}
impl GslSortable for f32 {}
impl GslSortable for i8 {}
impl GslSortable for u8 {}
impl GslSortable for i16 {}
impl GslSortable for u16 {}
impl GslSortable for i32 {}
impl GslSortable for u32 {}
impl GslSortable for i64 {}
impl GslSortable for u64 {}
impl GslSortable for isize {}
impl GslSortable for usize {}

pub fn sort_smallest_indices<T: GslSortable>(
    indices: &mut [usize],
    k: usize,
    src: &[T],
    stride: usize,
) -> Result<(), GslError> {
    let n = src.len();
    if k > n {
        return Err(GslError::Invalid);
    }
    if k == 0 || n == 0 {
        return Ok(());
    }

    let mut j = 1;
    let mut xbound = src[0];
    indices[0] = 0;

    for i in 1..n {
        let xi = src[i * stride];
        if j < k {
            j += 1;
        } else if xi >= xbound {
            continue;
        }

        let mut i1 = j - 1;
        while i1 > 0 {
            let cmp = xi.partial_cmp(&src[indices[i1 - 1] * stride]).unwrap();
            if cmp == Ordering::Greater {
                break;
            }
            indices[i1] = indices[i1 - 1];
            i1 -= 1;
        }
        indices[i1] = i;
        xbound = src[indices[j - 1] * stride];
    }

    Ok(())
}

pub fn sort_largest_indices<T: GslSortable>(
    indices: &mut [usize],
    k: usize,
    src: &[T],
    stride: usize,
) -> Result<(), GslError> {
    let n = src.len();
    if k > n {
        return Err(GslError::Invalid);
    }
    if k == 0 || n == 0 {
        return Ok(());
    }

    let mut j = 1;
    let mut xbound = src[0];
    indices[0] = 0;

    for i in 1..n {
        let xi = src[i * stride];
        if j < k {
            j += 1;
        } else if xi <= xbound {
            continue;
        }

        let mut i1 = j - 1;
        while i1 > 0 {
            let cmp = xi.partial_cmp(&src[indices[i1 - 1] * stride]).unwrap();
            if cmp == Ordering::Less {
                break;
            }
            indices[i1] = indices[i1 - 1];
            i1 -= 1;
        }
        indices[i1] = i;
        xbound = src[indices[j - 1] * stride];
    }

    Ok(())
}

// Generic vector types
pub struct GslVector<T> {
    pub size: usize,
    pub stride: usize,
    pub data: Vec<T>,
}

impl<T: GslSortable> GslVector<T> {
    pub fn sort_smallest_indices(&self, indices: &mut [usize], k: usize) -> Result<(), GslError> {
        sort_smallest_indices(indices, k, &self.data, self.stride)
    }

    pub fn sort_largest_indices(&self, indices: &mut [usize], k: usize) -> Result<(), GslError> {
        sort_largest_indices(indices, k, &self.data, self.stride)
    }
}

// Type aliases for specific vector types
pub type GslVectorF64 = GslVector<f64>;
pub type GslVectorF32 = GslVector<f32>;
pub type GslVectorI8 = GslVector<i8>;
pub type GslVectorU8 = GslVector<u8>;
pub type GslVectorI16 = GslVector<i16>;
pub type GslVectorU16 = GslVector<u16>;
pub type GslVectorI32 = GslVector<i32>;
pub type GslVectorU32 = GslVector<u32>;
pub type GslVectorI64 = GslVector<i64>;
pub type GslVectorU64 = GslVector<u64>;
pub type GslVectorIsize = GslVector<isize>;
pub type GslVectorUsize = GslVector<usize>;