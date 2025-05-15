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

#[derive(Debug, Clone)]
pub struct GslVector<T> {
    size: usize,
    stride: usize,
    data: Vec<T>,
    owner: bool,
}

impl<T> GslVector<T> {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            stride: 1,
            data: Vec::with_capacity(size),
            owner: true,
        }
    }

    pub fn from_slice(data: &[T]) -> Self 
    where
        T: Clone,
    {
        Self {
            size: data.len(),
            stride: 1,
            data: data.to_vec(),
            owner: true,
        }
    }
}

pub fn sort_smallest<T>(dest: &mut [T], src: &[T], stride: usize) -> Result<(), GslError> 
where
    T: PartialOrd + Clone,
{
    let k = dest.len();
    let n = src.len();

    if k > n {
        return Err(GslError::Invalid);
    }

    if k == 0 || n == 0 {
        return Ok(());
    }

    let mut j = 1;
    dest[0] = src[0].clone();
    let mut xbound = dest[0].clone();

    for i in 1..n {
        let xi = src[i * stride].clone();

        if j < k {
            j += 1;
        } else if xi >= xbound {
            continue;
        }

        let mut i1 = j - 1;
        while i1 > 0 {
            if xi > dest[i1 - 1] {
                break;
            }
            dest[i1] = dest[i1 - 1].clone();
            i1 -= 1;
        }

        dest[i1] = xi;
        xbound = dest[j - 1].clone();
    }

    Ok(())
}

pub fn sort_largest<T>(dest: &mut [T], src: &[T], stride: usize) -> Result<(), GslError> 
where
    T: PartialOrd + Clone,
{
    let k = dest.len();
    let n = src.len();

    if k > n {
        return Err(GslError::Invalid);
    }

    if k == 0 || n == 0 {
        return Ok(());
    }

    let mut j = 1;
    dest[0] = src[0].clone();
    let mut xbound = dest[0].clone();

    for i in 1..n {
        let xi = src[i * stride].clone();

        if j < k {
            j += 1;
        } else if xi <= xbound {
            continue;
        }

        let mut i1 = j - 1;
        while i1 > 0 {
            if xi < dest[i1 - 1] {
                break;
            }
            dest[i1] = dest[i1 - 1].clone();
            i1 -= 1;
        }

        dest[i1] = xi;
        xbound = dest[j - 1].clone();
    }

    Ok(())
}

macro_rules! impl_vector_sort {
    ($type:ty) => {
        impl GslVector<$type> {
            pub fn sort_smallest(&self, dest: &mut [$type]) -> Result<(), GslError> {
                sort_smallest(dest, &self.data, self.stride)
            }

            pub fn sort_largest(&self, dest: &mut [$type]) -> Result<(), GslError> {
                sort_largest(dest, &self.data, self.stride)
            }
        }
    };
}

impl_vector_sort!(i8);
impl_vector_sort!(u8);
impl_vector_sort!(i16);
impl_vector_sort!(u16);
impl_vector_sort!(i32);
impl_vector_sort!(u32);
impl_vector_sort!(i64);
impl_vector_sort!(u64);
impl_vector_sort!(f32);
impl_vector_sort!(f64);