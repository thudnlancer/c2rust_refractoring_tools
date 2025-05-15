use std::cmp::Ordering;

pub type size_t = usize;

#[derive(Copy, Clone)]
pub struct gsl_block_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_long_double = gsl_block_long_double_struct;

#[derive(Copy, Clone)]
pub struct gsl_vector_long_double {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: i32,
}

#[derive(Copy, Clone)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut f64,
}
pub type gsl_block = gsl_block_struct;

#[derive(Copy, Clone)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut f64,
    pub block: *mut gsl_block,
    pub owner: i32,
}

#[derive(Copy, Clone)]
pub struct gsl_block_float_struct {
    pub size: size_t,
    pub data: *mut f32,
}
pub type gsl_block_float = gsl_block_float_struct;

#[derive(Copy, Clone)]
pub struct gsl_vector_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut f32,
    pub block: *mut gsl_block_float,
    pub owner: i32,
}

#[derive(Copy, Clone)]
pub struct gsl_block_ulong_struct {
    pub size: size_t,
    pub data: *mut u64,
}
pub type gsl_block_ulong = gsl_block_ulong_struct;

#[derive(Copy, Clone)]
pub struct gsl_vector_ulong {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut u64,
    pub block: *mut gsl_block_ulong,
    pub owner: i32,
}

#[derive(Copy, Clone)]
pub struct gsl_block_long_struct {
    pub size: size_t,
    pub data: *mut i64,
}
pub type gsl_block_long = gsl_block_long_struct;

#[derive(Copy, Clone)]
pub struct gsl_vector_long {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut i64,
    pub block: *mut gsl_block_long,
    pub owner: i32,
}

#[derive(Copy, Clone)]
pub struct gsl_block_uint_struct {
    pub size: size_t,
    pub data: *mut u32,
}
pub type gsl_block_uint = gsl_block_uint_struct;

#[derive(Copy, Clone)]
pub struct gsl_vector_uint {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut u32,
    pub block: *mut gsl_block_uint,
    pub owner: i32,
}

#[derive(Copy, Clone)]
pub struct gsl_block_int_struct {
    pub size: size_t,
    pub data: *mut i32,
}
pub type gsl_block_int = gsl_block_int_struct;

#[derive(Copy, Clone)]
pub struct gsl_vector_int {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut i32,
    pub block: *mut gsl_block_int,
    pub owner: i32,
}

#[derive(Copy, Clone)]
pub struct gsl_block_ushort_struct {
    pub size: size_t,
    pub data: *mut u16,
}
pub type gsl_block_ushort = gsl_block_ushort_struct;

#[derive(Copy, Clone)]
pub struct gsl_vector_ushort {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut u16,
    pub block: *mut gsl_block_ushort,
    pub owner: i32,
}

#[derive(Copy, Clone)]
pub struct gsl_block_short_struct {
    pub size: size_t,
    pub data: *mut i16,
}
pub type gsl_block_short = gsl_block_short_struct;

#[derive(Copy, Clone)]
pub struct gsl_vector_short {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut i16,
    pub block: *mut gsl_block_short,
    pub owner: i32,
}

#[derive(Copy, Clone)]
pub struct gsl_block_uchar_struct {
    pub size: size_t,
    pub data: *mut u8,
}
pub type gsl_block_uchar = gsl_block_uchar_struct;

#[derive(Copy, Clone)]
pub struct gsl_vector_uchar {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut u8,
    pub block: *mut gsl_block_uchar,
    pub owner: i32,
}

#[derive(Copy, Clone)]
pub struct gsl_block_char_struct {
    pub size: size_t,
    pub data: *mut i8,
}
pub type gsl_block_char = gsl_block_char_struct;

#[derive(Copy, Clone)]
pub struct gsl_vector_char {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut i8,
    pub block: *mut gsl_block_char,
    pub owner: i32,
}

trait GslSortable: PartialOrd + Copy {}
impl GslSortable for f64 {}
impl GslSortable for f32 {}
impl GslSortable for i8 {}
impl GslSortable for i16 {}
impl GslSortable for i32 {}
impl GslSortable for i64 {}
impl GslSortable for u8 {}
impl GslSortable for u16 {}
impl GslSortable for u32 {}
impl GslSortable for u64 {}
impl GslSortable for f128::f128 {}

fn downheap<T: GslSortable>(data: &mut [T], stride: usize, n: usize, k: usize) {
    let mut k = k;
    let v = data[k * stride];
    
    loop {
        let mut j = 2 * k;
        if j + 1 < n && data[j * stride] < data[(j + 1) * stride] {
            j += 1;
        }
        
        if j >= n || !(v < data[j * stride]) {
            break;
        }
        
        data.swap(k * stride, j * stride);
        k = j;
    }
    
    data[k * stride] = v;
}

fn sort<T: GslSortable>(data: &mut [T], stride: usize) {
    let n = data.len() / stride;
    if n == 0 {
        return;
    }
    
    let mut k = (n - 1) / 2;
    loop {
        downheap(data, stride, n, k);
        if k == 0 {
            break;
        }
        k -= 1;
    }
    
    let mut n = n - 1;
    while n > 0 {
        data.swap(0, n * stride);
        downheap(data, stride, n, 0);
        n -= 1;
    }
}

fn downheap2<T: GslSortable, U: Copy>(data1: &mut [T], data2: &mut [U], stride1: usize, stride2: usize, n: usize, k: usize) {
    let mut k = k;
    let v1 = data1[k * stride1];
    let v2 = data2[k * stride2];
    
    loop {
        let mut j = 2 * k;
        if j + 1 < n && data1[j * stride1] < data1[(j + 1) * stride1] {
            j += 1;
        }
        
        if j >= n || !(v1 < data1[j * stride1]) {
            break;
        }
        
        data1.swap(k * stride1, j * stride1);
        data2.swap(k * stride2, j * stride2);
        k = j;
    }
    
    data1[k * stride1] = v1;
    data2[k * stride2] = v2;
}

fn sort2<T: GslSortable, U: Copy>(data1: &mut [T], data2: &mut [U], stride1: usize, stride2: usize) {
    let n = data1.len() / stride1;
    if n == 0 {
        return;
    }
    
    let mut k = (n - 1) / 2;
    loop {
        downheap2(data1, data2, stride1, stride2, n, k);
        if k == 0 {
            break;
        }
        k -= 1;
    }
    
    let mut n = n - 1;
    while n > 0 {
        data1.swap(0, n * stride1);
        data2.swap(0, n * stride2);
        downheap2(data1, data2, stride1, stride2, n, 0);
        n -= 1;
    }
}

// Implementations for all the specific types would follow the same pattern
// Here's an example for f64:

pub fn gsl_sort(data: &mut [f64], stride: usize) {
    sort(data, stride);
}

pub fn gsl_sort2(data1: &mut [f64], data2: &mut [f64], stride1: usize, stride2: usize) {
    sort2(data1, data2, stride1, stride2);
}

// Similar implementations would be needed for all other types (f32, i8, etc.)
// Vector versions would wrap the slice versions:

pub fn gsl_sort_vector(v: &mut gsl_vector) {
    unsafe {
        let data = std::slice::from_raw_parts_mut(v.data, v.size * v.stride);
        sort(data, v.stride);
    }
}

pub fn gsl_sort_vector2(v1: &mut gsl_vector, v2: &mut gsl_vector) {
    unsafe {
        let data1 = std::slice::from_raw_parts_mut(v1.data, v1.size * v1.stride);
        let data2 = std::slice::from_raw_parts_mut(v2.data, v2.size * v2.stride);
        sort2(data1, data2, v1.stride, v2.stride);
    }
}