use std::marker::PhantomData;

pub type size_t = usize;

pub struct Block<T> {
    size: size_t,
    data: Vec<T>,
}

impl<T> Block<T> {
    pub fn new(size: size_t) -> Self {
        Self {
            size,
            data: Vec::with_capacity(size),
        }
    }

    pub fn size(&self) -> size_t {
        self.size
    }

    pub fn data(&self) -> &[T] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.data
    }
}

pub type BlockComplexLongDouble = Block<f128::f128>;
pub type BlockComplex = Block<f64>;
pub type BlockComplexFloat = Block<f32>;
pub type BlockLongDouble = Block<f128::f128>;
pub type BlockFloat = Block<f32>;
pub type BlockUlong = Block<u64>;
pub type BlockLong = Block<i64>;
pub type BlockUint = Block<u32>;
pub type BlockInt = Block<i32>;
pub type BlockUshort = Block<u16>;
pub type BlockShort = Block<i16>;
pub type BlockUchar = Block<u8>;
pub type BlockChar = Block<i8>;