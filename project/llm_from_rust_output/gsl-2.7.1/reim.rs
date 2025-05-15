use std::marker::PhantomData;

pub type size_t = usize;

#[derive(Debug, Clone)]
pub struct GslBlock<T> {
    size: size_t,
    data: Vec<T>,
}

impl<T> GslBlock<T> {
    pub fn new(size: size_t) -> Self {
        Self {
            size,
            data: Vec::with_capacity(size),
        }
    }
}

#[derive(Debug, Clone)]
pub struct GslVector<T> {
    size: size_t,
    stride: size_t,
    data: Vec<T>,
    block: Option<GslBlock<T>>,
    owner: bool,
}

impl<T> GslVector<T> {
    pub fn new(size: size_t) -> Self {
        Self {
            size,
            stride: 1,
            data: Vec::with_capacity(size),
            block: None,
            owner: true,
        }
    }

    pub fn view(data: &mut [T], stride: size_t) -> GslVectorView<T> {
        GslVectorView {
            size: data.len(),
            stride,
            data,
        }
    }

    pub fn const_view(data: &[T], stride: size_t) -> GslVectorConstView<T> {
        GslVectorConstView {
            size: data.len(),
            stride,
            data,
        }
    }
}

#[derive(Debug)]
pub struct GslVectorView<'a, T> {
    size: size_t,
    stride: size_t,
    data: &'a mut [T],
}

#[derive(Debug)]
pub struct GslVectorConstView<'a, T> {
    size: size_t,
    stride: size_t,
    data: &'a [T],
}

#[derive(Debug, Clone)]
pub struct GslVectorComplex<T> {
    size: size_t,
    stride: size_t,
    data: Vec<T>,
    block: Option<GslBlock<T>>,
    owner: bool,
}

impl<T> GslVectorComplex<T> {
    pub fn new(size: size_t) -> Self {
        Self {
            size,
            stride: 1,
            data: Vec::with_capacity(size * 2),
            block: None,
            owner: true,
        }
    }

    pub fn real(&self) -> GslVectorConstView<T> {
        GslVectorConstView {
            size: self.size,
            stride: self.stride * 2,
            data: &self.data[0..self.size],
        }
    }

    pub fn real_mut(&mut self) -> GslVectorView<T> {
        GslVectorView {
            size: self.size,
            stride: self.stride * 2,
            data: &mut self.data[0..self.size],
        }
    }

    pub fn imag(&self) -> GslVectorConstView<T> {
        GslVectorConstView {
            size: self.size,
            stride: self.stride * 2,
            data: &self.data[1..=self.size],
        }
    }

    pub fn imag_mut(&mut self) -> GslVectorView<T> {
        GslVectorView {
            size: self.size,
            stride: self.stride * 2,
            data: &mut self.data[1..=self.size],
        }
    }
}

// Type aliases for specific numeric types
pub type GslBlockLongDouble = GslBlock<f64>;
pub type GslVectorLongDouble = GslVector<f64>;
pub type GslVectorComplexLongDouble = GslVectorComplex<f64>;

pub type GslBlockFloat = GslBlock<f32>;
pub type GslVectorFloat = GslVector<f32>;
pub type GslVectorComplexFloat = GslVectorComplex<f32>;

pub type GslBlockDouble = GslBlock<f64>;
pub type GslVectorDouble = GslVector<f64>;
pub type GslVectorComplexDouble = GslVectorComplex<f64>;