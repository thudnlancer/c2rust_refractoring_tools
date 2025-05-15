use std::num::Wrapping;

pub type size_t = u64;

#[inline]
pub fn xsum(size1: size_t, size2: size_t) -> size_t {
    let sum = Wrapping(size1) + Wrapping(size2);
    if sum.0 >= size1 {
        sum.0
    } else {
        u64::MAX
    }
}

#[inline]
pub fn xsum3(size1: size_t, size2: size_t, size3: size_t) -> size_t {
    xsum(xsum(size1, size2), size3)
}

#[inline]
pub fn xsum4(size1: size_t, size2: size_t, size3: size_t, size4: size_t) -> size_t {
    xsum(xsum(xsum(size1, size2), size3), size4)
}

#[inline]
pub fn xmax(size1: size_t, size2: size_t) -> size_t {
    if size1 >= size2 {
        size1
    } else {
        size2
    }
}