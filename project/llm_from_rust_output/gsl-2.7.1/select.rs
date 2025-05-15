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

fn gsl_error(reason: &str, file: &str, line: i32, errno: GslError) {
    // Implementation would log or handle the error appropriately
    eprintln!("GSL error at {}:{} - {} (code: {:?})", file, line, reason, errno);
}

pub fn gsl_stats_select<T: PartialOrd + Copy>(
    data: &mut [T],
    stride: usize,
    k: usize,
) -> Result<T, GslError> {
    let n = data.len();
    if n == 0 {
        gsl_error(
            "array size must be positive",
            "./select_source.rs",
            43,
            GslError::BadLen,
        );
        return Err(GslError::BadLen);
    }

    let mut left = 0;
    let mut right = n - 1;
    let mut mid;
    let mut i;
    let mut j;
    let mut pivot;
    let mut tmp;

    loop {
        if right <= left + 1 {
            if right == left + 1 && data[right * stride] < data[left * stride] {
                data.swap(right * stride, left * stride);
            }
            return Ok(data[k * stride]);
        } else {
            mid = (left + right) / 2;
            data.swap((left + 1) * stride, mid * stride);

            if data[left * stride] > data[right * stride] {
                data.swap(left * stride, right * stride);
            }
            if data[(left + 1) * stride] > data[right * stride] {
                data.swap((left + 1) * stride, right * stride);
            }
            if data[left * stride] > data[(left + 1) * stride] {
                data.swap(left * stride, (left + 1) * stride);
            }

            i = left + 1;
            j = right;
            pivot = data[(left + 1) * stride];

            loop {
                loop {
                    i += 1;
                    if !(data[i * stride] < pivot) {
                        break;
                    }
                }
                loop {
                    j -= 1;
                    if !(data[j * stride] > pivot) {
                        break;
                    }
                }
                if j < i {
                    break;
                }
                data.swap(i * stride, j * stride);
            }

            data[(left + 1) * stride] = data[j * stride];
            data[j * stride] = pivot;

            match j.cmp(&k) {
                Ordering::Greater => right = j - 1,
                Ordering::Less => left = i,
                Ordering::Equal => (),
            }
        }
    }
}

// Type-specific wrappers
pub fn gsl_stats_ulong_select(
    data: &mut [u64],
    stride: usize,
    k: usize,
) -> Result<u64, GslError> {
    gsl_stats_select(data, stride, k)
}

pub fn gsl_stats_uchar_select(
    data: &mut [u8],
    stride: usize,
    k: usize,
) -> Result<u8, GslError> {
    gsl_stats_select(data, stride, k)
}

pub fn gsl_stats_long_select(
    data: &mut [i64],
    stride: usize,
    k: usize,
) -> Result<i64, GslError> {
    gsl_stats_select(data, stride, k)
}

pub fn gsl_stats_char_select(
    data: &mut [i8],
    stride: usize,
    k: usize,
) -> Result<i8, GslError> {
    gsl_stats_select(data, stride, k)
}

pub fn gsl_stats_uint_select(
    data: &mut [u32],
    stride: usize,
    k: usize,
) -> Result<u32, GslError> {
    gsl_stats_select(data, stride, k)
}

pub fn gsl_stats_int_select(
    data: &mut [i32],
    stride: usize,
    k: usize,
) -> Result<i32, GslError> {
    gsl_stats_select(data, stride, k)
}

pub fn gsl_stats_ushort_select(
    data: &mut [u16],
    stride: usize,
    k: usize,
) -> Result<u16, GslError> {
    gsl_stats_select(data, stride, k)
}

pub fn gsl_stats_short_select(
    data: &mut [i16],
    stride: usize,
    k: usize,
) -> Result<i16, GslError> {
    gsl_stats_select(data, stride, k)
}

pub fn gsl_stats_float_select(
    data: &mut [f32],
    stride: usize,
    k: usize,
) -> Result<f32, GslError> {
    gsl_stats_select(data, stride, k)
}

pub fn gsl_stats_double_select(
    data: &mut [f64],
    stride: usize,
    k: usize,
) -> Result<f64, GslError> {
    gsl_stats_select(data, stride, k)
}

#[cfg(feature = "f128")]
pub fn gsl_stats_long_double_select(
    data: &mut [f128::f128],
    stride: usize,
    k: usize,
) -> Result<f128::f128, GslError> {
    gsl_stats_select(data, stride, k)
}