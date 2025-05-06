#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn gsl_sf_legendre_array_index(l: size_t, m: size_t) -> size_t {
    return (l.wrapping_mul(l.wrapping_add(1 as i32 as u64)) >> 1 as i32).wrapping_add(m);
}