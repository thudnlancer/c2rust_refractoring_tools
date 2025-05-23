#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(linkage)]
pub type size_t = u64;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn xsum(mut size1: size_t, mut size2: size_t) -> size_t {
    let mut sum: size_t = size1.wrapping_add(size2);
    return if sum >= size1 { sum } else { 18446744073709551615 as u64 };
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn xsum3(
    mut size1: size_t,
    mut size2: size_t,
    mut size3: size_t,
) -> size_t {
    return xsum(xsum(size1, size2), size3);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn xsum4(
    mut size1: size_t,
    mut size2: size_t,
    mut size3: size_t,
    mut size4: size_t,
) -> size_t {
    return xsum(xsum(xsum(size1, size2), size3), size4);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn xmax(mut size1: size_t, mut size2: size_t) -> size_t {
    return if size1 >= size2 { size1 } else { size2 };
}