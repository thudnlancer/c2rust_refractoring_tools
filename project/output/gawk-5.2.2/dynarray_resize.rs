#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn pma_malloc(size: size_t) -> *mut libc::c_void;
    fn pma_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn __errno_location() -> *mut i32;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynarray_header {
    pub used: size_t,
    pub allocated: size_t,
    pub array: *mut libc::c_void,
}
#[no_mangle]
pub unsafe extern "C" fn __libc_dynarray_resize(
    mut list: *mut dynarray_header,
    mut size: size_t,
    mut scratch: *mut libc::c_void,
    mut element_size: size_t,
) -> bool {
    if size <= (*list).allocated {
        (*list).used = size;
        return 1 as i32 != 0;
    }
    let mut new_size_bytes: size_t = 0;
    if if ::core::mem::size_of::<size_t>() as u64
        == ::core::mem::size_of::<libc::c_schar>() as u64
    {
        if !((0 as i32 as size_t) < -(1 as i32) as size_t) {
            if if element_size < 0 as i32 as u64 {
                if size < 0 as i32 as u64 {
                    if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        ((if 1 as i32 != 0 { 0 as i32 } else { 127 as i32 }) as u64)
                            .wrapping_add(element_size)
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        (size < (127 as i32 as u64).wrapping_div(element_size)) as i32
                    } else {
                        ((if (if (if (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            element_size
                        })
                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                        {
                            !((if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                element_size
                            })
                                .wrapping_add(1 as i32 as u64)
                                << (::core::mem::size_of::<size_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(0 as i32 as u64)
                        }) < 0 as i32 as u64
                        {
                            (element_size
                                < (if (if 1 as i32 != 0 {
                                    0 as i32 as u64
                                } else {
                                    element_size
                                })
                                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                                {
                                    ((if 1 as i32 != 0 {
                                        0 as i32 as u64
                                    } else {
                                        element_size
                                    })
                                        .wrapping_add(1 as i32 as u64)
                                        << (::core::mem::size_of::<size_t>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64))
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_mul(2 as i32 as u64)
                                        .wrapping_add(1 as i32 as u64)
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                        .wrapping_sub(1 as i32 as u64)
                                })
                                    .wrapping_neg()) as i32
                        } else {
                            ((0 as i32 as u64) < element_size) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(127 as i32 as u64)
                                >> (::core::mem::size_of::<size_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            (127 as i32 as u64).wrapping_div(element_size.wrapping_neg())
                        }) <= (-(1 as i32) as u64).wrapping_sub(size)) as i32
                    }
                } else if (if (if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    !((if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                    })
                        .wrapping_add(1 as i32 as u64)
                        << (::core::mem::size_of::<u64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                    })
                        .wrapping_add(0 as i32 as u64)
                }) < 0 as i32 as u64
                {
                    ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                        < (if (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                        })
                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                        {
                            ((if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                    .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                            })
                                .wrapping_add(1 as i32 as u64)
                                << (::core::mem::size_of::<u64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                    .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                            })
                                .wrapping_sub(1 as i32 as u64)
                        })
                            .wrapping_neg()) as i32
                } else {
                    ((0 as i32 as u64)
                        < (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add((-(127 as i32) - 1 as i32) as u64)) as i32
                }) != 0 && element_size == -(1 as i32) as u64
                {
                    if (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((0 as i32 as u64)
                            < size.wrapping_add((-(127 as i32) - 1 as i32) as u64))
                            as i32
                    } else {
                        ((0 as i32 as u64) < size
                            && ((-(1 as i32) - (-(127 as i32) - 1 as i32)) as u64)
                                < size.wrapping_sub(1 as i32 as u64)) as i32
                    }
                } else {
                    (((-(127 as i32) - 1 as i32) as u64).wrapping_div(element_size)
                        < size) as i32
                }
            } else if element_size == 0 as i32 as u64 {
                0 as i32
            } else if size < 0 as i32 as u64 {
                if (if (if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    !((if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                    })
                        .wrapping_add(1 as i32 as u64)
                        << (::core::mem::size_of::<u64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                    })
                        .wrapping_add(0 as i32 as u64)
                }) < 0 as i32 as u64
                {
                    ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                        < (if (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                        })
                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                        {
                            ((if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                    .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                            })
                                .wrapping_add(1 as i32 as u64)
                                << (::core::mem::size_of::<u64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                    .wrapping_add((-(127 as i32) - 1 as i32) as u64)
                            })
                                .wrapping_sub(1 as i32 as u64)
                        })
                            .wrapping_neg()) as i32
                } else {
                    ((0 as i32 as u64)
                        < (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add((-(127 as i32) - 1 as i32) as u64)) as i32
                }) != 0 && size == -(1 as i32) as u64
                {
                    if (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((0 as i32 as u64)
                            < element_size
                                .wrapping_add((-(127 as i32) - 1 as i32) as u64)) as i32
                    } else {
                        (((-(1 as i32) - (-(127 as i32) - 1 as i32)) as u64)
                            < element_size.wrapping_sub(1 as i32 as u64)) as i32
                    }
                } else {
                    (((-(127 as i32) - 1 as i32) as u64).wrapping_div(size)
                        < element_size) as i32
                }
            } else {
                ((127 as i32 as u64).wrapping_div(element_size) < size) as i32
            } != 0
            {
                new_size_bytes = (size as u32).wrapping_mul(element_size as u32)
                    as libc::c_schar as size_t;
                1 as i32
            } else {
                new_size_bytes = (size as u32).wrapping_mul(element_size as u32)
                    as libc::c_schar as size_t;
                0 as i32
            }
        } else if if element_size < 0 as i32 as u64 {
            if size < 0 as i32 as u64 {
                if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        127 as i32 * 2 as i32 + 1 as i32
                    }) as u64)
                        .wrapping_add(element_size)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    (size
                        < ((127 as i32 * 2 as i32 + 1 as i32) as u64)
                            .wrapping_div(element_size)) as i32
                } else {
                    ((if (if (if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        !((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(1 as i32 as u64)
                            << (::core::mem::size_of::<size_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(0 as i32 as u64)
                    }) < 0 as i32 as u64
                    {
                        (element_size
                            < (if (if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                element_size
                            })
                                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                            {
                                ((if 1 as i32 != 0 {
                                    0 as i32 as u64
                                } else {
                                    element_size
                                })
                                    .wrapping_add(1 as i32 as u64)
                                    << (::core::mem::size_of::<size_t>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64))
                                    .wrapping_sub(1 as i32 as u64)
                                    .wrapping_mul(2 as i32 as u64)
                                    .wrapping_add(1 as i32 as u64)
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                    .wrapping_sub(1 as i32 as u64)
                            })
                                .wrapping_neg()) as i32
                    } else {
                        ((0 as i32 as u64) < element_size) as i32
                    }) != 0
                    {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add((127 as i32 * 2 as i32 + 1 as i32) as u64)
                            >> (::core::mem::size_of::<size_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        ((127 as i32 * 2 as i32 + 1 as i32) as u64)
                            .wrapping_div(element_size.wrapping_neg())
                    }) <= (-(1 as i32) as u64).wrapping_sub(size)) as i32
                }
            } else if (if (if (if 1 as i32 != 0 {
                0 as i32 as u64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_add(0 as i32 as u64)
            })
                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
            {
                !((if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(1 as i32 as u64)
                    << (::core::mem::size_of::<u64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_mul(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(0 as i32 as u64)
            }) < 0 as i32 as u64
            {
                ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_add(0 as i32 as u64)
                    < (if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(0 as i32 as u64)
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_add(1 as i32 as u64)
                            << (::core::mem::size_of::<u64>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_sub(1 as i32 as u64)
                    })
                        .wrapping_neg()) as i32
            } else {
                ((0 as i32 as u64)
                    < (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)) as i32
            }) != 0 && element_size == -(1 as i32) as u64
            {
                if (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((0 as i32 as u64) < size.wrapping_add(0 as i32 as u64)) as i32
                } else {
                    ((0 as i32 as u64) < size
                        && ((-(1 as i32) - 0 as i32) as u64)
                            < size.wrapping_sub(1 as i32 as u64)) as i32
                }
            } else {
                ((0 as i32 as u64).wrapping_div(element_size) < size) as i32
            }
        } else if element_size == 0 as i32 as u64 {
            0 as i32
        } else if size < 0 as i32 as u64 {
            if (if (if (if 1 as i32 != 0 {
                0 as i32 as u64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_add(0 as i32 as u64)
            })
                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
            {
                !((if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(1 as i32 as u64)
                    << (::core::mem::size_of::<u64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_mul(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(0 as i32 as u64)
            }) < 0 as i32 as u64
            {
                ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_add(0 as i32 as u64)
                    < (if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add(0 as i32 as u64)
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_add(1 as i32 as u64)
                            << (::core::mem::size_of::<u64>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_sub(1 as i32 as u64)
                    })
                        .wrapping_neg()) as i32
            } else {
                ((0 as i32 as u64)
                    < (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add(0 as i32 as u64)) as i32
            }) != 0 && size == -(1 as i32) as u64
            {
                if (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((0 as i32 as u64) < element_size.wrapping_add(0 as i32 as u64))
                        as i32
                } else {
                    (((-(1 as i32) - 0 as i32) as u64)
                        < element_size.wrapping_sub(1 as i32 as u64)) as i32
                }
            } else {
                ((0 as i32 as u64).wrapping_div(size) < element_size) as i32
            }
        } else {
            (((127 as i32 * 2 as i32 + 1 as i32) as u64).wrapping_div(element_size)
                < size) as i32
        } != 0
        {
            new_size_bytes = (size as u32).wrapping_mul(element_size as u32) as u8
                as size_t;
            1 as i32
        } else {
            new_size_bytes = (size as u32).wrapping_mul(element_size as u32) as u8
                as size_t;
            0 as i32
        }
    } else if ::core::mem::size_of::<size_t>() as u64
        == ::core::mem::size_of::<libc::c_short>() as u64
    {
        if !((0 as i32 as size_t) < -(1 as i32) as size_t) {
            if if element_size < 0 as i32 as u64 {
                if size < 0 as i32 as u64 {
                    if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        ((if 1 as i32 != 0 { 0 as i32 } else { 32767 as i32 }) as u64)
                            .wrapping_add(element_size)
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        (size < (32767 as i32 as u64).wrapping_div(element_size)) as i32
                    } else {
                        ((if (if (if (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            element_size
                        })
                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                        {
                            !((if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                element_size
                            })
                                .wrapping_add(1 as i32 as u64)
                                << (::core::mem::size_of::<size_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(0 as i32 as u64)
                        }) < 0 as i32 as u64
                        {
                            (element_size
                                < (if (if 1 as i32 != 0 {
                                    0 as i32 as u64
                                } else {
                                    element_size
                                })
                                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                                {
                                    ((if 1 as i32 != 0 {
                                        0 as i32 as u64
                                    } else {
                                        element_size
                                    })
                                        .wrapping_add(1 as i32 as u64)
                                        << (::core::mem::size_of::<size_t>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64))
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_mul(2 as i32 as u64)
                                        .wrapping_add(1 as i32 as u64)
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                        .wrapping_sub(1 as i32 as u64)
                                })
                                    .wrapping_neg()) as i32
                        } else {
                            ((0 as i32 as u64) < element_size) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(32767 as i32 as u64)
                                >> (::core::mem::size_of::<size_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            (32767 as i32 as u64)
                                .wrapping_div(element_size.wrapping_neg())
                        }) <= (-(1 as i32) as u64).wrapping_sub(size)) as i32
                    }
                } else if (if (if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    !((if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                    })
                        .wrapping_add(1 as i32 as u64)
                        << (::core::mem::size_of::<u64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                    })
                        .wrapping_add(0 as i32 as u64)
                }) < 0 as i32 as u64
                {
                    ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                        < (if (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                        })
                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                        {
                            ((if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                    .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                            })
                                .wrapping_add(1 as i32 as u64)
                                << (::core::mem::size_of::<u64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                    .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                            })
                                .wrapping_sub(1 as i32 as u64)
                        })
                            .wrapping_neg()) as i32
                } else {
                    ((0 as i32 as u64)
                        < (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add((-(32767 as i32) - 1 as i32) as u64)) as i32
                }) != 0 && element_size == -(1 as i32) as u64
                {
                    if (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((0 as i32 as u64)
                            < size.wrapping_add((-(32767 as i32) - 1 as i32) as u64))
                            as i32
                    } else {
                        ((0 as i32 as u64) < size
                            && ((-(1 as i32) - (-(32767 as i32) - 1 as i32)) as u64)
                                < size.wrapping_sub(1 as i32 as u64)) as i32
                    }
                } else {
                    (((-(32767 as i32) - 1 as i32) as u64).wrapping_div(element_size)
                        < size) as i32
                }
            } else if element_size == 0 as i32 as u64 {
                0 as i32
            } else if size < 0 as i32 as u64 {
                if (if (if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    !((if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                    })
                        .wrapping_add(1 as i32 as u64)
                        << (::core::mem::size_of::<u64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                    })
                        .wrapping_add(0 as i32 as u64)
                }) < 0 as i32 as u64
                {
                    ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                        < (if (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                        })
                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                        {
                            ((if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                    .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                            })
                                .wrapping_add(1 as i32 as u64)
                                << (::core::mem::size_of::<u64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                    .wrapping_add((-(32767 as i32) - 1 as i32) as u64)
                            })
                                .wrapping_sub(1 as i32 as u64)
                        })
                            .wrapping_neg()) as i32
                } else {
                    ((0 as i32 as u64)
                        < (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add((-(32767 as i32) - 1 as i32) as u64)) as i32
                }) != 0 && size == -(1 as i32) as u64
                {
                    if (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((0 as i32 as u64)
                            < element_size
                                .wrapping_add((-(32767 as i32) - 1 as i32) as u64)) as i32
                    } else {
                        (((-(1 as i32) - (-(32767 as i32) - 1 as i32)) as u64)
                            < element_size.wrapping_sub(1 as i32 as u64)) as i32
                    }
                } else {
                    (((-(32767 as i32) - 1 as i32) as u64).wrapping_div(size)
                        < element_size) as i32
                }
            } else {
                ((32767 as i32 as u64).wrapping_div(element_size) < size) as i32
            } != 0
            {
                new_size_bytes = (size as u32).wrapping_mul(element_size as u32)
                    as libc::c_short as size_t;
                1 as i32
            } else {
                new_size_bytes = (size as u32).wrapping_mul(element_size as u32)
                    as libc::c_short as size_t;
                0 as i32
            }
        } else if if element_size < 0 as i32 as u64 {
            if size < 0 as i32 as u64 {
                if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        32767 as i32 * 2 as i32 + 1 as i32
                    }) as u64)
                        .wrapping_add(element_size)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    (size
                        < ((32767 as i32 * 2 as i32 + 1 as i32) as u64)
                            .wrapping_div(element_size)) as i32
                } else {
                    ((if (if (if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        !((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(1 as i32 as u64)
                            << (::core::mem::size_of::<size_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(0 as i32 as u64)
                    }) < 0 as i32 as u64
                    {
                        (element_size
                            < (if (if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                element_size
                            })
                                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                            {
                                ((if 1 as i32 != 0 {
                                    0 as i32 as u64
                                } else {
                                    element_size
                                })
                                    .wrapping_add(1 as i32 as u64)
                                    << (::core::mem::size_of::<size_t>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64))
                                    .wrapping_sub(1 as i32 as u64)
                                    .wrapping_mul(2 as i32 as u64)
                                    .wrapping_add(1 as i32 as u64)
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                    .wrapping_sub(1 as i32 as u64)
                            })
                                .wrapping_neg()) as i32
                    } else {
                        ((0 as i32 as u64) < element_size) as i32
                    }) != 0
                    {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add((32767 as i32 * 2 as i32 + 1 as i32) as u64)
                            >> (::core::mem::size_of::<size_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        ((32767 as i32 * 2 as i32 + 1 as i32) as u64)
                            .wrapping_div(element_size.wrapping_neg())
                    }) <= (-(1 as i32) as u64).wrapping_sub(size)) as i32
                }
            } else if (if (if (if 1 as i32 != 0 {
                0 as i32 as u64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_add(0 as i32 as u64)
            })
                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
            {
                !((if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(1 as i32 as u64)
                    << (::core::mem::size_of::<u64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_mul(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(0 as i32 as u64)
            }) < 0 as i32 as u64
            {
                ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_add(0 as i32 as u64)
                    < (if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(0 as i32 as u64)
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_add(1 as i32 as u64)
                            << (::core::mem::size_of::<u64>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_sub(1 as i32 as u64)
                    })
                        .wrapping_neg()) as i32
            } else {
                ((0 as i32 as u64)
                    < (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)) as i32
            }) != 0 && element_size == -(1 as i32) as u64
            {
                if (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((0 as i32 as u64) < size.wrapping_add(0 as i32 as u64)) as i32
                } else {
                    ((0 as i32 as u64) < size
                        && ((-(1 as i32) - 0 as i32) as u64)
                            < size.wrapping_sub(1 as i32 as u64)) as i32
                }
            } else {
                ((0 as i32 as u64).wrapping_div(element_size) < size) as i32
            }
        } else if element_size == 0 as i32 as u64 {
            0 as i32
        } else if size < 0 as i32 as u64 {
            if (if (if (if 1 as i32 != 0 {
                0 as i32 as u64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_add(0 as i32 as u64)
            })
                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
            {
                !((if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(1 as i32 as u64)
                    << (::core::mem::size_of::<u64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_mul(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(0 as i32 as u64)
            }) < 0 as i32 as u64
            {
                ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_add(0 as i32 as u64)
                    < (if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add(0 as i32 as u64)
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_add(1 as i32 as u64)
                            << (::core::mem::size_of::<u64>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_sub(1 as i32 as u64)
                    })
                        .wrapping_neg()) as i32
            } else {
                ((0 as i32 as u64)
                    < (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add(0 as i32 as u64)) as i32
            }) != 0 && size == -(1 as i32) as u64
            {
                if (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((0 as i32 as u64) < element_size.wrapping_add(0 as i32 as u64))
                        as i32
                } else {
                    (((-(1 as i32) - 0 as i32) as u64)
                        < element_size.wrapping_sub(1 as i32 as u64)) as i32
                }
            } else {
                ((0 as i32 as u64).wrapping_div(size) < element_size) as i32
            }
        } else {
            (((32767 as i32 * 2 as i32 + 1 as i32) as u64).wrapping_div(element_size)
                < size) as i32
        } != 0
        {
            new_size_bytes = (size as u32).wrapping_mul(element_size as u32)
                as libc::c_ushort as size_t;
            1 as i32
        } else {
            new_size_bytes = (size as u32).wrapping_mul(element_size as u32)
                as libc::c_ushort as size_t;
            0 as i32
        }
    } else if ::core::mem::size_of::<size_t>() as u64
        == ::core::mem::size_of::<i32>() as u64
    {
        if (if 1 as i32 != 0 { 0 as i32 as u64 } else { new_size_bytes })
            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
        {
            if if element_size < 0 as i32 as u64 {
                if size < 0 as i32 as u64 {
                    if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        ((if 1 as i32 != 0 { 0 as i32 } else { 2147483647 as i32 })
                            as u64)
                            .wrapping_add(element_size)
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        (size < (2147483647 as i32 as u64).wrapping_div(element_size))
                            as i32
                    } else {
                        ((if (if (if (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            element_size
                        })
                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                        {
                            !((if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                element_size
                            })
                                .wrapping_add(1 as i32 as u64)
                                << (::core::mem::size_of::<size_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(0 as i32 as u64)
                        }) < 0 as i32 as u64
                        {
                            (element_size
                                < (if (if 1 as i32 != 0 {
                                    0 as i32 as u64
                                } else {
                                    element_size
                                })
                                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                                {
                                    ((if 1 as i32 != 0 {
                                        0 as i32 as u64
                                    } else {
                                        element_size
                                    })
                                        .wrapping_add(1 as i32 as u64)
                                        << (::core::mem::size_of::<size_t>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64))
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_mul(2 as i32 as u64)
                                        .wrapping_add(1 as i32 as u64)
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                        .wrapping_sub(1 as i32 as u64)
                                })
                                    .wrapping_neg()) as i32
                        } else {
                            ((0 as i32 as u64) < element_size) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(2147483647 as i32 as u64)
                                >> (::core::mem::size_of::<size_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            (2147483647 as i32 as u64)
                                .wrapping_div(element_size.wrapping_neg())
                        }) <= (-(1 as i32) as u64).wrapping_sub(size)) as i32
                    }
                } else if (if (if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    !((if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                    })
                        .wrapping_add(1 as i32 as u64)
                        << (::core::mem::size_of::<u64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                    })
                        .wrapping_add(0 as i32 as u64)
                }) < 0 as i32 as u64
                {
                    ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                        < (if (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                        })
                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                        {
                            ((if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                    .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                            })
                                .wrapping_add(1 as i32 as u64)
                                << (::core::mem::size_of::<u64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                    .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                            })
                                .wrapping_sub(1 as i32 as u64)
                        })
                            .wrapping_neg()) as i32
                } else {
                    ((0 as i32 as u64)
                        < (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64))
                        as i32
                }) != 0 && element_size == -(1 as i32) as u64
                {
                    if (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((0 as i32 as u64)
                            < size
                                .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64))
                            as i32
                    } else {
                        ((0 as i32 as u64) < size
                            && ((-(1 as i32) - (-(2147483647 as i32) - 1 as i32)) as u64)
                                < size.wrapping_sub(1 as i32 as u64)) as i32
                    }
                } else {
                    (((-(2147483647 as i32) - 1 as i32) as u64)
                        .wrapping_div(element_size) < size) as i32
                }
            } else if element_size == 0 as i32 as u64 {
                0 as i32
            } else if size < 0 as i32 as u64 {
                if (if (if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    !((if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                    })
                        .wrapping_add(1 as i32 as u64)
                        << (::core::mem::size_of::<u64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                    })
                        .wrapping_add(0 as i32 as u64)
                }) < 0 as i32 as u64
                {
                    ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                        < (if (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                        })
                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                        {
                            ((if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                    .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                            })
                                .wrapping_add(1 as i32 as u64)
                                << (::core::mem::size_of::<u64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                    .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64)
                            })
                                .wrapping_sub(1 as i32 as u64)
                        })
                            .wrapping_neg()) as i32
                } else {
                    ((0 as i32 as u64)
                        < (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64))
                        as i32
                }) != 0 && size == -(1 as i32) as u64
                {
                    if (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((0 as i32 as u64)
                            < element_size
                                .wrapping_add((-(2147483647 as i32) - 1 as i32) as u64))
                            as i32
                    } else {
                        (((-(1 as i32) - (-(2147483647 as i32) - 1 as i32)) as u64)
                            < element_size.wrapping_sub(1 as i32 as u64)) as i32
                    }
                } else {
                    (((-(2147483647 as i32) - 1 as i32) as u64).wrapping_div(size)
                        < element_size) as i32
                }
            } else {
                ((2147483647 as i32 as u64).wrapping_div(element_size) < size) as i32
            } != 0
            {
                new_size_bytes = (size as u32).wrapping_mul(element_size as u32) as i32
                    as size_t;
                1 as i32
            } else {
                new_size_bytes = (size as u32).wrapping_mul(element_size as u32) as i32
                    as size_t;
                0 as i32
            }
        } else if if element_size < 0 as i32 as u64 {
            if size < 0 as i32 as u64 {
                if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    ((if 1 as i32 != 0 {
                        0 as i32 as u32
                    } else {
                        (2147483647 as i32 as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32)
                    }) as u64)
                        .wrapping_add(element_size)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    (size
                        < ((2147483647 as i32 as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32) as u64)
                            .wrapping_div(element_size)) as i32
                } else {
                    ((if (if (if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        !((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(1 as i32 as u64)
                            << (::core::mem::size_of::<size_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(0 as i32 as u64)
                    }) < 0 as i32 as u64
                    {
                        (element_size
                            < (if (if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                element_size
                            })
                                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                            {
                                ((if 1 as i32 != 0 {
                                    0 as i32 as u64
                                } else {
                                    element_size
                                })
                                    .wrapping_add(1 as i32 as u64)
                                    << (::core::mem::size_of::<size_t>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64))
                                    .wrapping_sub(1 as i32 as u64)
                                    .wrapping_mul(2 as i32 as u64)
                                    .wrapping_add(1 as i32 as u64)
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                    .wrapping_sub(1 as i32 as u64)
                            })
                                .wrapping_neg()) as i32
                    } else {
                        ((0 as i32 as u64) < element_size) as i32
                    }) != 0
                    {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(
                                (2147483647 as i32 as u32)
                                    .wrapping_mul(2 as u32)
                                    .wrapping_add(1 as u32) as u64,
                            )
                            >> (::core::mem::size_of::<size_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        ((2147483647 as i32 as u32)
                            .wrapping_mul(2 as u32)
                            .wrapping_add(1 as u32) as u64)
                            .wrapping_div(element_size.wrapping_neg())
                    }) <= (-(1 as i32) as u64).wrapping_sub(size)) as i32
                }
            } else if (if (if (if 1 as i32 != 0 {
                0 as i32 as u64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_add(0 as i32 as u64)
            })
                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
            {
                !((if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(1 as i32 as u64)
                    << (::core::mem::size_of::<u64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_mul(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(0 as i32 as u64)
            }) < 0 as i32 as u64
            {
                ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_add(0 as i32 as u64)
                    < (if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(0 as i32 as u64)
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_add(1 as i32 as u64)
                            << (::core::mem::size_of::<u64>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_sub(1 as i32 as u64)
                    })
                        .wrapping_neg()) as i32
            } else {
                ((0 as i32 as u64)
                    < (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)) as i32
            }) != 0 && element_size == -(1 as i32) as u64
            {
                if (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((0 as i32 as u64) < size.wrapping_add(0 as i32 as u64)) as i32
                } else {
                    ((0 as i32 as u64) < size
                        && ((-(1 as i32) - 0 as i32) as u64)
                            < size.wrapping_sub(1 as i32 as u64)) as i32
                }
            } else {
                ((0 as i32 as u64).wrapping_div(element_size) < size) as i32
            }
        } else if element_size == 0 as i32 as u64 {
            0 as i32
        } else if size < 0 as i32 as u64 {
            if (if (if (if 1 as i32 != 0 {
                0 as i32 as u64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_add(0 as i32 as u64)
            })
                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
            {
                !((if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(1 as i32 as u64)
                    << (::core::mem::size_of::<u64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_mul(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(0 as i32 as u64)
            }) < 0 as i32 as u64
            {
                ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_add(0 as i32 as u64)
                    < (if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add(0 as i32 as u64)
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_add(1 as i32 as u64)
                            << (::core::mem::size_of::<u64>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_sub(1 as i32 as u64)
                    })
                        .wrapping_neg()) as i32
            } else {
                ((0 as i32 as u64)
                    < (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add(0 as i32 as u64)) as i32
            }) != 0 && size == -(1 as i32) as u64
            {
                if (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((0 as i32 as u64) < element_size.wrapping_add(0 as i32 as u64))
                        as i32
                } else {
                    (((-(1 as i32) - 0 as i32) as u64)
                        < element_size.wrapping_sub(1 as i32 as u64)) as i32
                }
            } else {
                ((0 as i32 as u64).wrapping_div(size) < element_size) as i32
            }
        } else {
            (((2147483647 as i32 as u32).wrapping_mul(2 as u32).wrapping_add(1 as u32)
                as u64)
                .wrapping_div(element_size) < size) as i32
        } != 0
        {
            new_size_bytes = (size as u32).wrapping_mul(element_size as u32) as size_t;
            1 as i32
        } else {
            new_size_bytes = (size as u32).wrapping_mul(element_size as u32) as size_t;
            0 as i32
        }
    } else if ::core::mem::size_of::<size_t>() as u64
        == ::core::mem::size_of::<i64>() as u64
    {
        if (if 1 as i32 != 0 { 0 as i32 as u64 } else { new_size_bytes })
            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
        {
            if if element_size < 0 as i32 as u64 {
                if size < 0 as i32 as u64 {
                    if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            9223372036854775807 as i64
                        }) as u64)
                            .wrapping_add(element_size)
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        (size
                            < (9223372036854775807 as i64 as u64)
                                .wrapping_div(element_size)) as i32
                    } else {
                        ((if (if (if (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            element_size
                        })
                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                        {
                            !((if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                element_size
                            })
                                .wrapping_add(1 as i32 as u64)
                                << (::core::mem::size_of::<size_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(0 as i32 as u64)
                        }) < 0 as i32 as u64
                        {
                            (element_size
                                < (if (if 1 as i32 != 0 {
                                    0 as i32 as u64
                                } else {
                                    element_size
                                })
                                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                                {
                                    ((if 1 as i32 != 0 {
                                        0 as i32 as u64
                                    } else {
                                        element_size
                                    })
                                        .wrapping_add(1 as i32 as u64)
                                        << (::core::mem::size_of::<size_t>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64))
                                        .wrapping_sub(1 as i32 as u64)
                                        .wrapping_mul(2 as i32 as u64)
                                        .wrapping_add(1 as i32 as u64)
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                        .wrapping_sub(1 as i32 as u64)
                                })
                                    .wrapping_neg()) as i32
                        } else {
                            ((0 as i32 as u64) < element_size) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(9223372036854775807 as i64 as u64)
                                >> (::core::mem::size_of::<size_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            (9223372036854775807 as i64 as u64)
                                .wrapping_div(element_size.wrapping_neg())
                        }) <= (-(1 as i32) as u64).wrapping_sub(size)) as i32
                    }
                } else if (if (if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add((-(9223372036854775807 as i64) - 1 as i64) as u64)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    !((if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(
                                (-(9223372036854775807 as i64) - 1 as i64) as u64,
                            )
                    })
                        .wrapping_add(1 as i32 as u64)
                        << (::core::mem::size_of::<u64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(
                                (-(9223372036854775807 as i64) - 1 as i64) as u64,
                            )
                    })
                        .wrapping_add(0 as i32 as u64)
                }) < 0 as i32 as u64
                {
                    ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add((-(9223372036854775807 as i64) - 1 as i64) as u64)
                        < (if (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(
                                    (-(9223372036854775807 as i64) - 1 as i64) as u64,
                                )
                        })
                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                        {
                            ((if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                    .wrapping_add(
                                        (-(9223372036854775807 as i64) - 1 as i64) as u64,
                                    )
                            })
                                .wrapping_add(1 as i32 as u64)
                                << (::core::mem::size_of::<u64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                    .wrapping_add(
                                        (-(9223372036854775807 as i64) - 1 as i64) as u64,
                                    )
                            })
                                .wrapping_sub(1 as i32 as u64)
                        })
                            .wrapping_neg()) as i32
                } else {
                    ((0 as i32 as u64)
                        < (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(
                                (-(9223372036854775807 as i64) - 1 as i64) as u64,
                            )) as i32
                }) != 0 && element_size == -(1 as i32) as u64
                {
                    if (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((0 as i32 as u64)
                            < size
                                .wrapping_add(
                                    (-(9223372036854775807 as i64) - 1 as i64) as u64,
                                )) as i32
                    } else {
                        ((0 as i32 as u64) < size
                            && ((-(1 as i32) as i64
                                - (-(9223372036854775807 as i64) - 1 as i64)) as u64)
                                < size.wrapping_sub(1 as i32 as u64)) as i32
                    }
                } else {
                    (((-(9223372036854775807 as i64) - 1 as i64) as u64)
                        .wrapping_div(element_size) < size) as i32
                }
            } else if element_size == 0 as i32 as u64 {
                0 as i32
            } else if size < 0 as i32 as u64 {
                if (if (if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add((-(9223372036854775807 as i64) - 1 as i64) as u64)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    !((if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add(
                                (-(9223372036854775807 as i64) - 1 as i64) as u64,
                            )
                    })
                        .wrapping_add(1 as i32 as u64)
                        << (::core::mem::size_of::<u64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add(
                                (-(9223372036854775807 as i64) - 1 as i64) as u64,
                            )
                    })
                        .wrapping_add(0 as i32 as u64)
                }) < 0 as i32 as u64
                {
                    ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add((-(9223372036854775807 as i64) - 1 as i64) as u64)
                        < (if (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                .wrapping_add(
                                    (-(9223372036854775807 as i64) - 1 as i64) as u64,
                                )
                        })
                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                        {
                            ((if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                    .wrapping_add(
                                        (-(9223372036854775807 as i64) - 1 as i64) as u64,
                                    )
                            })
                                .wrapping_add(1 as i32 as u64)
                                << (::core::mem::size_of::<u64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                    .wrapping_add(
                                        (-(9223372036854775807 as i64) - 1 as i64) as u64,
                                    )
                            })
                                .wrapping_sub(1 as i32 as u64)
                        })
                            .wrapping_neg()) as i32
                } else {
                    ((0 as i32 as u64)
                        < (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add(
                                (-(9223372036854775807 as i64) - 1 as i64) as u64,
                            )) as i32
                }) != 0 && size == -(1 as i32) as u64
                {
                    if (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((0 as i32 as u64)
                            < element_size
                                .wrapping_add(
                                    (-(9223372036854775807 as i64) - 1 as i64) as u64,
                                )) as i32
                    } else {
                        (((-(1 as i32) as i64
                            - (-(9223372036854775807 as i64) - 1 as i64)) as u64)
                            < element_size.wrapping_sub(1 as i32 as u64)) as i32
                    }
                } else {
                    (((-(9223372036854775807 as i64) - 1 as i64) as u64)
                        .wrapping_div(size) < element_size) as i32
                }
            } else {
                ((9223372036854775807 as i64 as u64).wrapping_div(element_size) < size)
                    as i32
            } != 0
            {
                new_size_bytes = size.wrapping_mul(element_size) as i64 as size_t;
                1 as i32
            } else {
                new_size_bytes = size.wrapping_mul(element_size) as i64 as size_t;
                0 as i32
            }
        } else if if element_size < 0 as i32 as u64 {
            if size < 0 as i32 as u64 {
                if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (9223372036854775807 as i64 as u64)
                            .wrapping_mul(2 as u64)
                            .wrapping_add(1 as u64)
                    })
                        .wrapping_add(element_size)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    (size
                        < (9223372036854775807 as i64 as u64)
                            .wrapping_mul(2 as u64)
                            .wrapping_add(1 as u64)
                            .wrapping_div(element_size)) as i32
                } else {
                    ((if (if (if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        !((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(1 as i32 as u64)
                            << (::core::mem::size_of::<size_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(0 as i32 as u64)
                    }) < 0 as i32 as u64
                    {
                        (element_size
                            < (if (if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                element_size
                            })
                                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                            {
                                ((if 1 as i32 != 0 {
                                    0 as i32 as u64
                                } else {
                                    element_size
                                })
                                    .wrapping_add(1 as i32 as u64)
                                    << (::core::mem::size_of::<size_t>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64))
                                    .wrapping_sub(1 as i32 as u64)
                                    .wrapping_mul(2 as i32 as u64)
                                    .wrapping_add(1 as i32 as u64)
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                    .wrapping_sub(1 as i32 as u64)
                            })
                                .wrapping_neg()) as i32
                    } else {
                        ((0 as i32 as u64) < element_size) as i32
                    }) != 0
                    {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(
                                (9223372036854775807 as i64 as u64)
                                    .wrapping_mul(2 as u64)
                                    .wrapping_add(1 as u64),
                            )
                            >> (::core::mem::size_of::<size_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        (9223372036854775807 as i64 as u64)
                            .wrapping_mul(2 as u64)
                            .wrapping_add(1 as u64)
                            .wrapping_div(element_size.wrapping_neg())
                    }) <= (-(1 as i32) as u64).wrapping_sub(size)) as i32
                }
            } else if (if (if (if 1 as i32 != 0 {
                0 as i32 as u64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_add(0 as i32 as u64)
            })
                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
            {
                !((if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(1 as i32 as u64)
                    << (::core::mem::size_of::<u64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_mul(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(0 as i32 as u64)
            }) < 0 as i32 as u64
            {
                ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_add(0 as i32 as u64)
                    < (if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(0 as i32 as u64)
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_add(1 as i32 as u64)
                            << (::core::mem::size_of::<u64>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_sub(1 as i32 as u64)
                    })
                        .wrapping_neg()) as i32
            } else {
                ((0 as i32 as u64)
                    < (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)) as i32
            }) != 0 && element_size == -(1 as i32) as u64
            {
                if (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((0 as i32 as u64) < size.wrapping_add(0 as i32 as u64)) as i32
                } else {
                    ((0 as i32 as u64) < size
                        && ((-(1 as i32) - 0 as i32) as u64)
                            < size.wrapping_sub(1 as i32 as u64)) as i32
                }
            } else {
                ((0 as i32 as u64).wrapping_div(element_size) < size) as i32
            }
        } else if element_size == 0 as i32 as u64 {
            0 as i32
        } else if size < 0 as i32 as u64 {
            if (if (if (if 1 as i32 != 0 {
                0 as i32 as u64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_add(0 as i32 as u64)
            })
                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
            {
                !((if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(1 as i32 as u64)
                    << (::core::mem::size_of::<u64>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as u64)
                    .wrapping_mul(2 as i32 as u64)
                    .wrapping_add(1 as i32 as u64)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_add(0 as i32 as u64)
            }) < 0 as i32 as u64
            {
                ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_add(0 as i32 as u64)
                    < (if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add(0 as i32 as u64)
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        ((if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_add(1 as i32 as u64)
                            << (::core::mem::size_of::<u64>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                .wrapping_add(0 as i32 as u64)
                        })
                            .wrapping_sub(1 as i32 as u64)
                    })
                        .wrapping_neg()) as i32
            } else {
                ((0 as i32 as u64)
                    < (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add(0 as i32 as u64)) as i32
            }) != 0 && size == -(1 as i32) as u64
            {
                if (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((0 as i32 as u64) < element_size.wrapping_add(0 as i32 as u64))
                        as i32
                } else {
                    (((-(1 as i32) - 0 as i32) as u64)
                        < element_size.wrapping_sub(1 as i32 as u64)) as i32
                }
            } else {
                ((0 as i32 as u64).wrapping_div(size) < element_size) as i32
            }
        } else {
            ((9223372036854775807 as i64 as u64)
                .wrapping_mul(2 as u64)
                .wrapping_add(1 as u64)
                .wrapping_div(element_size) < size) as i32
        } != 0
        {
            new_size_bytes = size.wrapping_mul(element_size);
            1 as i32
        } else {
            new_size_bytes = size.wrapping_mul(element_size);
            0 as i32
        }
    } else if (if 1 as i32 != 0 { 0 as i32 as u64 } else { new_size_bytes })
        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
    {
        if if element_size < 0 as i32 as u64 {
            if size < 0 as i32 as u64 {
                if (if 1 as i32 != 0 {
                    0 as i32 as libc::c_ulonglong
                } else {
                    ((if 1 as i32 != 0 {
                        0 as i32 as libc::c_longlong
                    } else {
                        9223372036854775807 as libc::c_longlong
                    }) as libc::c_ulonglong)
                        .wrapping_add(element_size as libc::c_ulonglong)
                })
                    .wrapping_sub(1 as i32 as libc::c_ulonglong)
                    < 0 as i32 as libc::c_ulonglong
                {
                    ((size as libc::c_ulonglong)
                        < (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                            .wrapping_div(element_size as libc::c_ulonglong)) as i32
                } else {
                    ((if (if (if (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        element_size
                    })
                        .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                    {
                        !((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(1 as i32 as u64)
                            << (::core::mem::size_of::<size_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as u64)
                            .wrapping_mul(2 as i32 as u64)
                            .wrapping_add(1 as i32 as u64)
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(0 as i32 as u64)
                    }) < 0 as i32 as u64
                    {
                        (element_size
                            < (if (if 1 as i32 != 0 {
                                0 as i32 as u64
                            } else {
                                element_size
                            })
                                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                            {
                                ((if 1 as i32 != 0 {
                                    0 as i32 as u64
                                } else {
                                    element_size
                                })
                                    .wrapping_add(1 as i32 as u64)
                                    << (::core::mem::size_of::<size_t>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64))
                                    .wrapping_sub(1 as i32 as u64)
                                    .wrapping_mul(2 as i32 as u64)
                                    .wrapping_add(1 as i32 as u64)
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                    .wrapping_sub(1 as i32 as u64)
                            })
                                .wrapping_neg()) as i32
                    } else {
                        ((0 as i32 as u64) < element_size) as i32
                    }) != 0
                    {
                        ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            as libc::c_ulonglong)
                            .wrapping_add(
                                9223372036854775807 as libc::c_longlong as libc::c_ulonglong,
                            )
                            >> (::core::mem::size_of::<size_t>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(1 as i32 as u64)
                    } else {
                        (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                            .wrapping_div(
                                element_size.wrapping_neg() as libc::c_ulonglong,
                            )
                    }) <= (-(1 as i32) as u64).wrapping_sub(size) as libc::c_ulonglong)
                        as i32
                }
            } else if (if (if (if 1 as i32 != 0 {
                0 as i32 as libc::c_ulonglong
            } else {
                ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    as libc::c_ulonglong)
                    .wrapping_add(
                        (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong) as libc::c_ulonglong,
                    )
            })
                .wrapping_sub(1 as i32 as libc::c_ulonglong)
                < 0 as i32 as libc::c_ulonglong
            {
                !((if 1 as i32 != 0 {
                    0 as i32 as libc::c_ulonglong
                } else {
                    ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        as libc::c_ulonglong)
                        .wrapping_add(
                            (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                        )
                })
                    .wrapping_add(1 as i32 as libc::c_ulonglong)
                    << (::core::mem::size_of::<libc::c_ulonglong>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as libc::c_ulonglong)
                    .wrapping_mul(2 as i32 as libc::c_ulonglong)
                    .wrapping_add(1 as i32 as libc::c_ulonglong)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as libc::c_ulonglong
                } else {
                    ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        as libc::c_ulonglong)
                        .wrapping_add(
                            (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                        )
                })
                    .wrapping_add(0 as i32 as libc::c_ulonglong)
            }) < 0 as i32 as libc::c_ulonglong
            {
                (((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    as libc::c_ulonglong)
                    .wrapping_add(
                        (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong) as libc::c_ulonglong,
                    )
                    < (if (if 1 as i32 != 0 {
                        0 as i32 as libc::c_ulonglong
                    } else {
                        ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            as libc::c_ulonglong)
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                            )
                    })
                        .wrapping_sub(1 as i32 as libc::c_ulonglong)
                        < 0 as i32 as libc::c_ulonglong
                    {
                        ((if 1 as i32 != 0 {
                            0 as i32 as libc::c_ulonglong
                        } else {
                            ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                as libc::c_ulonglong)
                                .wrapping_add(
                                    (-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong) as libc::c_ulonglong,
                                )
                        })
                            .wrapping_add(1 as i32 as libc::c_ulonglong)
                            << (::core::mem::size_of::<libc::c_ulonglong>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as libc::c_ulonglong)
                            .wrapping_mul(2 as i32 as libc::c_ulonglong)
                            .wrapping_add(1 as i32 as libc::c_ulonglong)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as libc::c_ulonglong
                        } else {
                            ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                as libc::c_ulonglong)
                                .wrapping_add(
                                    (-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong) as libc::c_ulonglong,
                                )
                        })
                            .wrapping_sub(1 as i32 as libc::c_ulonglong)
                    })
                        .wrapping_neg()) as i32
            } else {
                ((0 as i32 as libc::c_ulonglong)
                    < ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        as libc::c_ulonglong)
                        .wrapping_add(
                            (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                        )) as i32
            }) != 0 && element_size == -(1 as i32) as u64
            {
                if (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((0 as i32 as libc::c_ulonglong)
                        < (size as libc::c_ulonglong)
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                            )) as i32
                } else {
                    ((0 as i32 as u64) < size
                        && ((-(1 as i32) as libc::c_longlong
                            - (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong)) as libc::c_ulonglong)
                            < size.wrapping_sub(1 as i32 as u64) as libc::c_ulonglong)
                        as i32
                }
            } else {
                (((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                    as libc::c_ulonglong)
                    .wrapping_div(element_size as libc::c_ulonglong)
                    < size as libc::c_ulonglong) as i32
            }
        } else if element_size == 0 as i32 as u64 {
            0 as i32
        } else if size < 0 as i32 as u64 {
            if (if (if (if 1 as i32 != 0 {
                0 as i32 as libc::c_ulonglong
            } else {
                ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    as libc::c_ulonglong)
                    .wrapping_add(
                        (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong) as libc::c_ulonglong,
                    )
            })
                .wrapping_sub(1 as i32 as libc::c_ulonglong)
                < 0 as i32 as libc::c_ulonglong
            {
                !((if 1 as i32 != 0 {
                    0 as i32 as libc::c_ulonglong
                } else {
                    ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        as libc::c_ulonglong)
                        .wrapping_add(
                            (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                        )
                })
                    .wrapping_add(1 as i32 as libc::c_ulonglong)
                    << (::core::mem::size_of::<libc::c_ulonglong>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64))
                    .wrapping_sub(1 as i32 as libc::c_ulonglong)
                    .wrapping_mul(2 as i32 as libc::c_ulonglong)
                    .wrapping_add(1 as i32 as libc::c_ulonglong)
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as libc::c_ulonglong
                } else {
                    ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        as libc::c_ulonglong)
                        .wrapping_add(
                            (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                        )
                })
                    .wrapping_add(0 as i32 as libc::c_ulonglong)
            }) < 0 as i32 as libc::c_ulonglong
            {
                (((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    as libc::c_ulonglong)
                    .wrapping_add(
                        (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong) as libc::c_ulonglong,
                    )
                    < (if (if 1 as i32 != 0 {
                        0 as i32 as libc::c_ulonglong
                    } else {
                        ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            as libc::c_ulonglong)
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                            )
                    })
                        .wrapping_sub(1 as i32 as libc::c_ulonglong)
                        < 0 as i32 as libc::c_ulonglong
                    {
                        ((if 1 as i32 != 0 {
                            0 as i32 as libc::c_ulonglong
                        } else {
                            ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                as libc::c_ulonglong)
                                .wrapping_add(
                                    (-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong) as libc::c_ulonglong,
                                )
                        })
                            .wrapping_add(1 as i32 as libc::c_ulonglong)
                            << (::core::mem::size_of::<libc::c_ulonglong>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64))
                            .wrapping_sub(1 as i32 as libc::c_ulonglong)
                            .wrapping_mul(2 as i32 as libc::c_ulonglong)
                            .wrapping_add(1 as i32 as libc::c_ulonglong)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32 as libc::c_ulonglong
                        } else {
                            ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                                as libc::c_ulonglong)
                                .wrapping_add(
                                    (-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong) as libc::c_ulonglong,
                                )
                        })
                            .wrapping_sub(1 as i32 as libc::c_ulonglong)
                    })
                        .wrapping_neg()) as i32
            } else {
                ((0 as i32 as libc::c_ulonglong)
                    < ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        as libc::c_ulonglong)
                        .wrapping_add(
                            (-(9223372036854775807 as libc::c_longlong)
                                - 1 as libc::c_longlong) as libc::c_ulonglong,
                        )) as i32
            }) != 0 && size == -(1 as i32) as u64
            {
                if (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((0 as i32 as libc::c_ulonglong)
                        < (element_size as libc::c_ulonglong)
                            .wrapping_add(
                                (-(9223372036854775807 as libc::c_longlong)
                                    - 1 as libc::c_longlong) as libc::c_ulonglong,
                            )) as i32
                } else {
                    (((-(1 as i32) as libc::c_longlong
                        - (-(9223372036854775807 as libc::c_longlong)
                            - 1 as libc::c_longlong)) as libc::c_ulonglong)
                        < element_size.wrapping_sub(1 as i32 as u64)
                            as libc::c_ulonglong) as i32
                }
            } else {
                (((-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
                    as libc::c_ulonglong)
                    .wrapping_div(size as libc::c_ulonglong)
                    < element_size as libc::c_ulonglong) as i32
            }
        } else {
            ((9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_div(element_size as libc::c_ulonglong)
                < size as libc::c_ulonglong) as i32
        } != 0
        {
            new_size_bytes = (size as libc::c_ulonglong)
                .wrapping_mul(element_size as libc::c_ulonglong) as libc::c_longlong
                as size_t;
            1 as i32
        } else {
            new_size_bytes = (size as libc::c_ulonglong)
                .wrapping_mul(element_size as libc::c_ulonglong) as libc::c_longlong
                as size_t;
            0 as i32
        }
    } else if if element_size < 0 as i32 as u64 {
        if size < 0 as i32 as u64 {
            if (if 1 as i32 != 0 {
                0 as i32 as libc::c_ulonglong
            } else {
                (if 1 as i32 != 0 {
                    0 as i32 as libc::c_ulonglong
                } else {
                    (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                })
                    .wrapping_add(element_size as libc::c_ulonglong)
            })
                .wrapping_sub(1 as i32 as libc::c_ulonglong)
                < 0 as i32 as libc::c_ulonglong
            {
                ((size as libc::c_ulonglong)
                    < (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                        .wrapping_div(element_size as libc::c_ulonglong)) as i32
            } else {
                ((if (if (if (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    !((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(1 as i32 as u64)
                        << (::core::mem::size_of::<size_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64)
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)
                }) < 0 as i32 as u64
                {
                    (element_size
                        < (if (if 1 as i32 != 0 {
                            0 as i32 as u64
                        } else {
                            element_size
                        })
                            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                        {
                            ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_add(1 as i32 as u64)
                                << (::core::mem::size_of::<size_t>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64))
                                .wrapping_sub(1 as i32 as u64)
                                .wrapping_mul(2 as i32 as u64)
                                .wrapping_add(1 as i32 as u64)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                                .wrapping_sub(1 as i32 as u64)
                        })
                            .wrapping_neg()) as i32
                } else {
                    ((0 as i32 as u64) < element_size) as i32
                }) != 0
                {
                    ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        as libc::c_ulonglong)
                        .wrapping_add(
                            (9223372036854775807 as libc::c_longlong
                                as libc::c_ulonglong)
                                .wrapping_mul(2 as libc::c_ulonglong)
                                .wrapping_add(1 as libc::c_ulonglong),
                        )
                        >> (::core::mem::size_of::<size_t>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(1 as i32 as u64)
                } else {
                    (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                        .wrapping_mul(2 as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_ulonglong)
                        .wrapping_div(element_size.wrapping_neg() as libc::c_ulonglong)
                }) <= (-(1 as i32) as u64).wrapping_sub(size) as libc::c_ulonglong)
                    as i32
            }
        } else if (if (if (if 1 as i32 != 0 {
            0 as i32 as u64
        } else {
            (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                .wrapping_add(0 as i32 as u64)
        })
            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
        {
            !((if 1 as i32 != 0 {
                0 as i32 as u64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_add(0 as i32 as u64)
            })
                .wrapping_add(1 as i32 as u64)
                << (::core::mem::size_of::<u64>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64))
                .wrapping_sub(1 as i32 as u64)
                .wrapping_mul(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64)
        } else {
            (if 1 as i32 != 0 {
                0 as i32 as u64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_add(0 as i32 as u64)
            })
                .wrapping_add(0 as i32 as u64)
        }) < 0 as i32 as u64
        {
            ((if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                .wrapping_add(0 as i32 as u64)
                < (if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(0 as i32 as u64)
                    })
                        .wrapping_add(1 as i32 as u64)
                        << (::core::mem::size_of::<u64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                            .wrapping_add(0 as i32 as u64)
                    })
                        .wrapping_sub(1 as i32 as u64)
                })
                    .wrapping_neg()) as i32
        } else {
            ((0 as i32 as u64)
                < (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                    .wrapping_add(0 as i32 as u64)) as i32
        }) != 0 && element_size == -(1 as i32) as u64
        {
            if (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
            {
                ((0 as i32 as u64) < size.wrapping_add(0 as i32 as u64)) as i32
            } else {
                ((0 as i32 as u64) < size
                    && ((-(1 as i32) - 0 as i32) as u64)
                        < size.wrapping_sub(1 as i32 as u64)) as i32
            }
        } else {
            ((0 as i32 as u64).wrapping_div(element_size) < size) as i32
        }
    } else if element_size == 0 as i32 as u64 {
        0 as i32
    } else if size < 0 as i32 as u64 {
        if (if (if (if 1 as i32 != 0 {
            0 as i32 as u64
        } else {
            (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                .wrapping_add(0 as i32 as u64)
        })
            .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
        {
            !((if 1 as i32 != 0 {
                0 as i32 as u64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_add(0 as i32 as u64)
            })
                .wrapping_add(1 as i32 as u64)
                << (::core::mem::size_of::<u64>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64))
                .wrapping_sub(1 as i32 as u64)
                .wrapping_mul(2 as i32 as u64)
                .wrapping_add(1 as i32 as u64)
        } else {
            (if 1 as i32 != 0 {
                0 as i32 as u64
            } else {
                (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_add(0 as i32 as u64)
            })
                .wrapping_add(0 as i32 as u64)
        }) < 0 as i32 as u64
        {
            ((if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                .wrapping_add(0 as i32 as u64)
                < (if (if 1 as i32 != 0 {
                    0 as i32 as u64
                } else {
                    (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                        .wrapping_add(0 as i32 as u64)
                })
                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                {
                    ((if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add(0 as i32 as u64)
                    })
                        .wrapping_add(1 as i32 as u64)
                        << (::core::mem::size_of::<u64>() as u64)
                            .wrapping_mul(8 as i32 as u64)
                            .wrapping_sub(2 as i32 as u64))
                        .wrapping_sub(1 as i32 as u64)
                        .wrapping_mul(2 as i32 as u64)
                        .wrapping_add(1 as i32 as u64)
                } else {
                    (if 1 as i32 != 0 {
                        0 as i32 as u64
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                            .wrapping_add(0 as i32 as u64)
                    })
                        .wrapping_sub(1 as i32 as u64)
                })
                    .wrapping_neg()) as i32
        } else {
            ((0 as i32 as u64)
                < (if 1 as i32 != 0 { 0 as i32 as u64 } else { size })
                    .wrapping_add(0 as i32 as u64)) as i32
        }) != 0 && size == -(1 as i32) as u64
        {
            if (if 1 as i32 != 0 { 0 as i32 as u64 } else { element_size })
                .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
            {
                ((0 as i32 as u64) < element_size.wrapping_add(0 as i32 as u64)) as i32
            } else {
                (((-(1 as i32) - 0 as i32) as u64)
                    < element_size.wrapping_sub(1 as i32 as u64)) as i32
            }
        } else {
            ((0 as i32 as u64).wrapping_div(size) < element_size) as i32
        }
    } else {
        ((9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_ulonglong)
            .wrapping_div(element_size as libc::c_ulonglong) < size as libc::c_ulonglong)
            as i32
    } != 0
    {
        new_size_bytes = (size as libc::c_ulonglong)
            .wrapping_mul(element_size as libc::c_ulonglong) as size_t;
        1 as i32
    } else {
        new_size_bytes = (size as libc::c_ulonglong)
            .wrapping_mul(element_size as libc::c_ulonglong) as size_t;
        0 as i32
    } != 0
    {
        *__errno_location() = 12 as i32;
        return 0 as i32 != 0;
    }
    let mut new_array: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*list).array == scratch {
        new_array = pma_malloc(new_size_bytes);
        if !new_array.is_null() && !((*list).array).is_null() {
            memcpy(new_array, (*list).array, ((*list).used).wrapping_mul(element_size));
        }
    } else {
        new_array = pma_realloc((*list).array, new_size_bytes);
    }
    if new_array.is_null() {
        return 0 as i32 != 0;
    }
    (*list).array = new_array;
    (*list).allocated = size;
    (*list).used = size;
    return 1 as i32 != 0;
}