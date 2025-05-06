#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uintmax_t = u64;
pub type uintmax_t = __uintmax_t;
unsafe extern "C" fn count_trailing_zeros(mut n: uintmax_t) -> i32 {
    return (n as libc::c_ulonglong).trailing_zeros() as i32;
}
#[no_mangle]
pub unsafe extern "C" fn adjust_uint(mut n: uintmax_t) -> uintmax_t {
    let mut wordbits: i32 = (8 as i32 as u64)
        .wrapping_mul(::core::mem::size_of::<uintmax_t>() as u64) as i32;
    if (if ::core::mem::size_of::<libc::c_double>() as u64
        == ::core::mem::size_of::<f128::f128>() as u64
    {
        64 as i32
    } else {
        (if ::core::mem::size_of::<libc::c_double>() as u64
            == ::core::mem::size_of::<libc::c_double>() as u64
        {
            53 as i32
        } else {
            24 as i32
        })
    }) * (if 2 as i32 == 2 as i32 { 1 as i32 } else { 4 as i32 }) < wordbits
    {
        let mut one: uintmax_t = 1 as i32 as uintmax_t;
        let mut sentinel: uintmax_t = one
            << wordbits
                - (if ::core::mem::size_of::<libc::c_double>() as u64
                    == ::core::mem::size_of::<f128::f128>() as u64
                {
                    64 as i32
                } else {
                    (if ::core::mem::size_of::<libc::c_double>() as u64
                        == ::core::mem::size_of::<libc::c_double>() as u64
                    {
                        53 as i32
                    } else {
                        24 as i32
                    })
                }) * (if 2 as i32 == 2 as i32 { 1 as i32 } else { 4 as i32 });
        let mut shift: i32 = count_trailing_zeros(n | sentinel);
        let mut mask: uintmax_t = (one
            << (if ::core::mem::size_of::<libc::c_double>() as u64
                == ::core::mem::size_of::<f128::f128>() as u64
            {
                64 as i32
            } else {
                (if ::core::mem::size_of::<libc::c_double>() as u64
                    == ::core::mem::size_of::<libc::c_double>() as u64
                {
                    53 as i32
                } else {
                    24 as i32
                })
            }) * (if 2 as i32 == 2 as i32 { 1 as i32 } else { 4 as i32 }))
            .wrapping_sub(1 as i32 as u64);
        n &= mask << shift;
    }
    return n;
}