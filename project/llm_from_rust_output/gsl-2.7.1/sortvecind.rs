use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    EDOM = 1,
    ERANGE = 2,
    EFAULT = 3,
    EINVAL = 4,
    EFAILED = 5,
    EFACTOR = 6,
    ESANITY = 7,
    ENOMEM = 8,
    EBADFUNC = 9,
    ERUNAWAY = 10,
    EMAXITER = 11,
    EZERODIV = 12,
    EBADTOL = 13,
    ETOL = 14,
    EUNDRFLW = 15,
    EOVRFLW = 16,
    ELOSS = 17,
    EROUND = 18,
    EBADLEN = 19,
    ENOTSQR = 20,
    ESING = 21,
    EDIVERGE = 22,
    EUNSUP = 23,
    EUNIMPL = 24,
    ECACHE = 25,
    ETABLE = 26,
    ENOPROG = 27,
    ENOPROGJ = 28,
    ETOLF = 29,
    ETOLX = 30,
    ETOLG = 31,
    EOF = 32,
}

#[derive(Debug, Clone)]
pub struct GslPermutation {
    size: usize,
    data: Vec<usize>,
}

impl GslPermutation {
    pub fn new(size: usize) -> Self {
        Self {
            size,
            data: (0..size).collect(),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn data(&self) -> &[usize] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [usize] {
        &mut self.data
    }
}

pub trait GslVector<T> {
    fn size(&self) -> usize;
    fn stride(&self) -> usize;
    fn data(&self) -> &[T];
}

macro_rules! impl_downheap {
    ($name:ident, $type:ty) => {
        fn $name(p: &mut [usize], data: &[$type], stride: usize, n: usize, mut k: usize) {
            let pki = p[k];
            while k <= n / 2 {
                let mut j = 2 * k;
                if j < n {
                    let left = p[j] * stride;
                    let right = p[j + 1] * stride;
                    if data[left] < data[right] {
                        j += 1;
                    }
                }
                if data[pki * stride] >= data[p[j] * stride] {
                    break;
                }
                p[k] = p[j];
                k = j;
            }
            p[k] = pki;
        }
    };
}

impl_downheap!(index_downheap, f64);
impl_downheap!(index_float_downheap, f32);
impl_downheap!(index_long_downheap, i64);
impl_downheap!(index_ulong_downheap, u64);
impl_downheap!(index_int_downheap, i32);
impl_downheap!(index_uint_downheap, u32);
impl_downheap!(index_short_downheap, i16);
impl_downheap!(index_ushort_downheap, u16);
impl_downheap!(index_char_downheap, i8);
impl_downheap!(index_uchar_downheap, u8);

macro_rules! impl_sort_index {
    ($name:ident, $type:ty, $downheap:ident) => {
        pub fn $name(p: &mut [usize], data: &[$type], stride: usize, n: usize) {
            if n == 0 {
                return;
            }

            for i in 0..n {
                p[i] = i;
            }

            let mut n = n - 1;
            let mut k = n / 2 + 1;

            while k > 0 {
                k -= 1;
                $downheap(p, data, stride, n, k);
            }

            while n > 0 {
                p.swap(0, n);
                n -= 1;
                $downheap(p, data, stride, n, 0);
            }
        }
    };
}

impl_sort_index!(gsl_sort_index, f64, index_downheap);
impl_sort_index!(gsl_sort_float_index, f32, index_float_downheap);
impl_sort_index!(gsl_sort_long_index, i64, index_long_downheap);
impl_sort_index!(gsl_sort_ulong_index, u64, index_ulong_downheap);
impl_sort_index!(gsl_sort_int_index, i32, index_int_downheap);
impl_sort_index!(gsl_sort_uint_index, u32, index_uint_downheap);
impl_sort_index!(gsl_sort_short_index, i16, index_short_downheap);
impl_sort_index!(gsl_sort_ushort_index, u16, index_ushort_downheap);
impl_sort_index!(gsl_sort_char_index, i8, index_char_downheap);
impl_sort_index!(gsl_sort_uchar_index, u8, index_uchar_downheap);

macro_rules! impl_sort_vector_index {
    ($name:ident, $type:ty, $sort_fn:ident) => {
        pub fn $name(
            permutation: &mut GslPermutation,
            v: &impl GslVector<$type>,
        ) -> Result<(), GslError> {
            if permutation.size() != v.size() {
                return Err(GslError::EBADLEN);
            }

            $sort_fn(permutation.data_mut(), v.data(), v.stride(), v.size());
            Ok(())
        }
    };
}

impl_sort_vector_index!(gsl_sort_vector_index, f64, gsl_sort_index);
impl_sort_vector_index!(gsl_sort_vector_float_index, f32, gsl_sort_float_index);
impl_sort_vector_index!(gsl_sort_vector_long_index, i64, gsl_sort_long_index);
impl_sort_vector_index!(gsl_sort_vector_ulong_index, u64, gsl_sort_ulong_index);
impl_sort_vector_index!(gsl_sort_vector_int_index, i32, gsl_sort_int_index);
impl_sort_vector_index!(gsl_sort_vector_uint_index, u32, gsl_sort_uint_index);
impl_sort_vector_index!(gsl_sort_vector_short_index, i16, gsl_sort_short_index);
impl_sort_vector_index!(gsl_sort_vector_ushort_index, u16, gsl_sort_ushort_index);
impl_sort_vector_index!(gsl_sort_vector_char_index, i8, gsl_sort_char_index);
impl_sort_vector_index!(gsl_sort_vector_uchar_index, u8, gsl_sort_uchar_index);