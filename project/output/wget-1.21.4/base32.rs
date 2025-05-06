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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
pub type ptrdiff_t = i64;
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base32_decode_context {
    pub i: i32,
    pub buf: [i8; 8],
}
#[inline]
unsafe extern "C" fn imalloc(mut s: idx_t) -> *mut libc::c_void {
    return if s as u64 <= 18446744073709551615 as u64 {
        malloc(s as u64)
    } else {
        _gl_alloc_nomem()
    };
}
#[cold]
#[inline]
unsafe extern "C" fn _gl_alloc_nomem() -> *mut libc::c_void {
    *__errno_location() = 12 as i32;
    return 0 as *mut libc::c_void;
}
unsafe extern "C" fn to_uchar(mut ch: i8) -> u8 {
    return ch as u8;
}
#[no_mangle]
pub unsafe extern "C" fn base32_encode(
    mut in_0: *const i8,
    mut inlen: idx_t,
    mut out: *mut i8,
    mut outlen: idx_t,
) {
    static mut b32str: [i8; 32] = unsafe {
        *::core::mem::transmute::<
            &[u8; 32],
            &[i8; 32],
        >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567")
    };
    while inlen != 0 && outlen != 0 {
        let fresh0 = out;
        out = out.offset(1);
        *fresh0 = b32str[(to_uchar(*in_0.offset(0 as i32 as isize)) as i32 >> 3 as i32
            & 0x1f as i32) as usize];
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        inlen -= 1;
        let fresh1 = out;
        out = out.offset(1);
        *fresh1 = b32str[(((to_uchar(*in_0.offset(0 as i32 as isize)) as i32)
            << 2 as i32)
            + (if inlen != 0 {
                to_uchar(*in_0.offset(1 as i32 as isize)) as i32 >> 6 as i32
            } else {
                0 as i32
            }) & 0x1f as i32) as usize];
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        let fresh2 = out;
        out = out.offset(1);
        *fresh2 = (if inlen != 0 {
            b32str[(to_uchar(*in_0.offset(1 as i32 as isize)) as i32 >> 1 as i32
                & 0x1f as i32) as usize] as i32
        } else {
            '=' as i32
        }) as i8;
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        let fresh3 = out;
        out = out.offset(1);
        *fresh3 = (if inlen != 0 {
            inlen -= 1;
            b32str[(((to_uchar(*in_0.offset(1 as i32 as isize)) as i32) << 4 as i32)
                + (if inlen != 0 {
                    to_uchar(*in_0.offset(2 as i32 as isize)) as i32 >> 4 as i32
                } else {
                    0 as i32
                }) & 0x1f as i32) as usize] as i32
        } else {
            '=' as i32
        }) as i8;
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        let fresh4 = out;
        out = out.offset(1);
        *fresh4 = (if inlen != 0 {
            inlen -= 1;
            b32str[(((to_uchar(*in_0.offset(2 as i32 as isize)) as i32) << 1 as i32)
                + (if inlen != 0 {
                    to_uchar(*in_0.offset(3 as i32 as isize)) as i32 >> 7 as i32
                } else {
                    0 as i32
                }) & 0x1f as i32) as usize] as i32
        } else {
            '=' as i32
        }) as i8;
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        let fresh5 = out;
        out = out.offset(1);
        *fresh5 = (if inlen != 0 {
            b32str[(to_uchar(*in_0.offset(3 as i32 as isize)) as i32 >> 2 as i32
                & 0x1f as i32) as usize] as i32
        } else {
            '=' as i32
        }) as i8;
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        let fresh6 = out;
        out = out.offset(1);
        *fresh6 = (if inlen != 0 {
            inlen -= 1;
            b32str[(((to_uchar(*in_0.offset(3 as i32 as isize)) as i32) << 3 as i32)
                + (if inlen != 0 {
                    to_uchar(*in_0.offset(4 as i32 as isize)) as i32 >> 5 as i32
                } else {
                    0 as i32
                }) & 0x1f as i32) as usize] as i32
        } else {
            '=' as i32
        }) as i8;
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        let fresh7 = out;
        out = out.offset(1);
        *fresh7 = (if inlen != 0 {
            b32str[(to_uchar(*in_0.offset(4 as i32 as isize)) as i32 & 0x1f as i32)
                as usize] as i32
        } else {
            '=' as i32
        }) as i8;
        outlen -= 1;
        if outlen == 0 {
            break;
        }
        if inlen != 0 {
            inlen -= 1;
            inlen;
        }
        if inlen != 0 {
            in_0 = in_0.offset(5 as i32 as isize);
        }
    }
    if outlen != 0 {
        *out = '\0' as i32 as i8;
    }
}
#[no_mangle]
pub unsafe extern "C" fn base32_encode_alloc(
    mut in_0: *const i8,
    mut inlen: idx_t,
    mut out: *mut *mut i8,
) -> idx_t {
    let mut in_over_5: idx_t = inlen / 5 as i32 as i64
        + (inlen % 5 as i32 as i64 != 0 as i32 as i64) as i32 as i64;
    let mut outlen: idx_t = 0;
    if (if ::core::mem::size_of::<idx_t>() as u64
        == ::core::mem::size_of::<libc::c_schar>() as u64
    {
        (if !((0 as i32 as idx_t) < -(1 as i32) as idx_t) {
            (if (if (8 as i32) < 0 as i32 {
                (if in_over_5 < 0 as i32 as i64 {
                    (if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { 127 as i32 }) + 8 as i32
                    }) - 1 as i32) < 0 as i32
                    {
                        (in_over_5 < (127 as i32 / 8 as i32) as i64) as i32
                    } else {
                        ((if (if (if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                            - 1 as i32) < 0 as i32
                        {
                            !(((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                        }) < 0 as i32
                        {
                            ((8 as i32)
                                < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    - 1 as i32) < 0 as i32
                                {
                                    ((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) - 1 as i32
                                })) as i32
                        } else {
                            ((0 as i32) < 8 as i32) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                + 127 as i32
                                >> (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            127 as i32 / -(8 as i32)
                        }) as i64 <= -(1 as i32) as i64 - in_over_5) as i32
                    })
                } else {
                    (if (if (if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                            + (-(127 as i32) - 1 as i32)
                    }) - 1 as i32) < 0 as i32
                    {
                        !(((((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                + (-(127 as i32) - 1 as i32)
                        }) + 1 as i32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                            + 1 as i32)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                + (-(127 as i32) - 1 as i32)
                        }) + 0 as i32
                    }) < 0 as i32
                    {
                        ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                            + (-(127 as i32) - 1 as i32)
                            < -(if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    + (-(127 as i32) - 1 as i32)
                            }) - 1 as i32) < 0 as i32
                            {
                                ((((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + (-(127 as i32) - 1 as i32)
                                }) + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + (-(127 as i32) - 1 as i32)
                                }) - 1 as i32
                            })) as i32
                    } else {
                        ((0 as i32)
                            < (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                + (-(127 as i32) - 1 as i32)) as i32
                    }) != 0 && 8 as i32 == -(1 as i32)
                    {
                        (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                            - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((0 as i32 as i64)
                                < in_over_5 + (-(127 as i32) - 1 as i32) as i64) as i32
                        } else {
                            ((0 as i32 as i64) < in_over_5
                                && ((-(1 as i32) - (-(127 as i32) - 1 as i32)) as i64)
                                    < in_over_5 - 1 as i32 as i64) as i32
                        })
                    } else {
                        ((((-(127 as i32) - 1 as i32) / 8 as i32) as i64) < in_over_5)
                            as i32
                    })
                })
            } else {
                (if 8 as i32 == 0 as i32 {
                    0 as i32
                } else {
                    (if in_over_5 < 0 as i32 as i64 {
                        (if (if (if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                + (-(127 as i32) - 1 as i32) as i64
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            !(((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                    + (-(127 as i32) - 1 as i32) as i64
                            }) + 1 as i32 as i64)
                                << (::core::mem::size_of::<i64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                    + (-(127 as i32) - 1 as i32) as i64
                            }) + 0 as i32 as i64
                        }) < 0 as i32 as i64
                        {
                            (((if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                + (-(127 as i32) - 1 as i32) as i64)
                                < -(if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                        + (-(127 as i32) - 1 as i32) as i64
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    ((((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + (-(127 as i32) - 1 as i32) as i64
                                    }) + 1 as i32 as i64)
                                        << (::core::mem::size_of::<i64>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + (-(127 as i32) - 1 as i32) as i64
                                    }) - 1 as i32 as i64
                                })) as i32
                        } else {
                            ((0 as i32 as i64)
                                < (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                    + (-(127 as i32) - 1 as i32) as i64) as i32
                        }) != 0 && in_over_5 == -(1 as i32) as i64
                        {
                            (if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                - 1 as i32) < 0 as i32
                            {
                                ((0 as i32) < 8 as i32 + (-(127 as i32) - 1 as i32)) as i32
                            } else {
                                (-(1 as i32) - (-(127 as i32) - 1 as i32)
                                    < 8 as i32 - 1 as i32) as i32
                            })
                        } else {
                            ((-(127 as i32) - 1 as i32) as i64 / in_over_5
                                < 8 as i32 as i64) as i32
                        })
                    } else {
                        (((127 as i32 / 8 as i32) as i64) < in_over_5) as i32
                    })
                })
            }) != 0
            {
                outlen = (in_over_5 as u32).wrapping_mul(8 as i32 as u32)
                    as libc::c_schar as idx_t;
                1 as i32
            } else {
                outlen = (in_over_5 as u32).wrapping_mul(8 as i32 as u32)
                    as libc::c_schar as idx_t;
                0 as i32
            })
        } else {
            (if (if (8 as i32) < 0 as i32 {
                (if in_over_5 < 0 as i32 as i64 {
                    (if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            127 as i32 * 2 as i32 + 1 as i32
                        }) + 8 as i32
                    }) - 1 as i32) < 0 as i32
                    {
                        (in_over_5
                            < ((127 as i32 * 2 as i32 + 1 as i32) / 8 as i32) as i64)
                            as i32
                    } else {
                        ((if (if (if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                            - 1 as i32) < 0 as i32
                        {
                            !(((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32)
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                        }) < 0 as i32
                        {
                            ((8 as i32)
                                < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    - 1 as i32) < 0 as i32
                                {
                                    ((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) - 1 as i32
                                })) as i32
                        } else {
                            ((0 as i32) < 8 as i32) as i32
                        }) != 0
                        {
                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                + (127 as i32 * 2 as i32 + 1 as i32)
                                >> (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(1 as i32 as u64)
                        } else {
                            (127 as i32 * 2 as i32 + 1 as i32) / -(8 as i32)
                        }) as i64 <= -(1 as i32) as i64 - in_over_5) as i32
                    })
                } else {
                    (if (if (if ((if 1 as i32 != 0 {
                        0 as i32
                    } else {
                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                    }) - 1 as i32) < 0 as i32
                    {
                        !(((((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                        }) + 1 as i32)
                            << (::core::mem::size_of::<i32>() as u64)
                                .wrapping_mul(8 as i32 as u64)
                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                            + 1 as i32)
                    } else {
                        (if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                        }) + 0 as i32
                    }) < 0 as i32
                    {
                        (((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32)
                            < -(if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                            }) - 1 as i32) < 0 as i32
                            {
                                ((((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                }) + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                }) - 1 as i32
                            })) as i32
                    } else {
                        ((0 as i32)
                            < (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                + 0 as i32) as i32
                    }) != 0 && 8 as i32 == -(1 as i32)
                    {
                        (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                            - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            ((0 as i32 as i64) < in_over_5 + 0 as i32 as i64) as i32
                        } else {
                            ((0 as i32 as i64) < in_over_5
                                && ((-(1 as i32) - 0 as i32) as i64)
                                    < in_over_5 - 1 as i32 as i64) as i32
                        })
                    } else {
                        (((0 as i32 / 8 as i32) as i64) < in_over_5) as i32
                    })
                })
            } else {
                (if 8 as i32 == 0 as i32 {
                    0 as i32
                } else {
                    (if in_over_5 < 0 as i32 as i64 {
                        (if (if (if ((if 1 as i32 != 0 {
                            0 as i32 as i64
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                + 0 as i32 as i64
                        }) - 1 as i32 as i64) < 0 as i32 as i64
                        {
                            !(((((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                    + 0 as i32 as i64
                            }) + 1 as i32 as i64)
                                << (::core::mem::size_of::<i64>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                * 2 as i32 as i64 + 1 as i32 as i64)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                    + 0 as i32 as i64
                            }) + 0 as i32 as i64
                        }) < 0 as i32 as i64
                        {
                            (((if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                + 0 as i32 as i64)
                                < -(if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                        + 0 as i32 as i64
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    ((((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + 0 as i32 as i64
                                    }) + 1 as i32 as i64)
                                        << (::core::mem::size_of::<i64>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + 0 as i32 as i64
                                    }) - 1 as i32 as i64
                                })) as i32
                        } else {
                            ((0 as i32 as i64)
                                < (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                    + 0 as i32 as i64) as i32
                        }) != 0 && in_over_5 == -(1 as i32) as i64
                        {
                            (if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                - 1 as i32) < 0 as i32
                            {
                                ((0 as i32) < 8 as i32 + 0 as i32) as i32
                            } else {
                                ((-(1 as i32) - 0 as i32) < 8 as i32 - 1 as i32) as i32
                            })
                        } else {
                            (0 as i32 as i64 / in_over_5 < 8 as i32 as i64) as i32
                        })
                    } else {
                        ((((127 as i32 * 2 as i32 + 1 as i32) / 8 as i32) as i64)
                            < in_over_5) as i32
                    })
                })
            }) != 0
            {
                outlen = (in_over_5 as u32).wrapping_mul(8 as i32 as u32) as u8 as idx_t;
                1 as i32
            } else {
                outlen = (in_over_5 as u32).wrapping_mul(8 as i32 as u32) as u8 as idx_t;
                0 as i32
            })
        })
    } else {
        (if ::core::mem::size_of::<idx_t>() as u64
            == ::core::mem::size_of::<libc::c_short>() as u64
        {
            (if !((0 as i32 as idx_t) < -(1 as i32) as idx_t) {
                (if (if (8 as i32) < 0 as i32 {
                    (if in_over_5 < 0 as i32 as i64 {
                        (if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 32767 as i32 })
                                + 8 as i32
                        }) - 1 as i32) < 0 as i32
                        {
                            (in_over_5 < (32767 as i32 / 8 as i32) as i64) as i32
                        } else {
                            ((if (if (if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                8 as i32
                            }) - 1 as i32) < 0 as i32
                            {
                                !(((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32)
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                            }) < 0 as i32
                            {
                                ((8 as i32)
                                    < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        - 1 as i32) < 0 as i32
                                    {
                                        ((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) - 1 as i32
                                    })) as i32
                            } else {
                                ((0 as i32) < 8 as i32) as i32
                            }) != 0
                            {
                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    + 32767 as i32
                                    >> (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(1 as i32 as u64)
                            } else {
                                32767 as i32 / -(8 as i32)
                            }) as i64 <= -(1 as i32) as i64 - in_over_5) as i32
                        })
                    } else {
                        (if (if (if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                + (-(32767 as i32) - 1 as i32)
                        }) - 1 as i32) < 0 as i32
                        {
                            !(((((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    + (-(32767 as i32) - 1 as i32)
                            }) + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    + (-(32767 as i32) - 1 as i32)
                            }) + 0 as i32
                        }) < 0 as i32
                        {
                            ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                + (-(32767 as i32) - 1 as i32)
                                < -(if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + (-(32767 as i32) - 1 as i32)
                                }) - 1 as i32) < 0 as i32
                                {
                                    ((((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            + (-(32767 as i32) - 1 as i32)
                                    }) + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            + (-(32767 as i32) - 1 as i32)
                                    }) - 1 as i32
                                })) as i32
                        } else {
                            ((0 as i32)
                                < (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    + (-(32767 as i32) - 1 as i32)) as i32
                        }) != 0 && 8 as i32 == -(1 as i32)
                        {
                            (if ((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                in_over_5
                            }) - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                ((0 as i32 as i64)
                                    < in_over_5 + (-(32767 as i32) - 1 as i32) as i64) as i32
                            } else {
                                ((0 as i32 as i64) < in_over_5
                                    && ((-(1 as i32) - (-(32767 as i32) - 1 as i32)) as i64)
                                        < in_over_5 - 1 as i32 as i64) as i32
                            })
                        } else {
                            ((((-(32767 as i32) - 1 as i32) / 8 as i32) as i64)
                                < in_over_5) as i32
                        })
                    })
                } else {
                    (if 8 as i32 == 0 as i32 {
                        0 as i32
                    } else {
                        (if in_over_5 < 0 as i32 as i64 {
                            (if (if (if ((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                    + (-(32767 as i32) - 1 as i32) as i64
                            }) - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                !(((((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                        + (-(32767 as i32) - 1 as i32) as i64
                                }) + 1 as i32 as i64)
                                    << (::core::mem::size_of::<i64>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                    * 2 as i32 as i64 + 1 as i32 as i64)
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                        + (-(32767 as i32) - 1 as i32) as i64
                                }) + 0 as i32 as i64
                            }) < 0 as i32 as i64
                            {
                                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                    + (-(32767 as i32) - 1 as i32) as i64)
                                    < -(if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + (-(32767 as i32) - 1 as i32) as i64
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + (-(32767 as i32) - 1 as i32) as i64
                                        }) + 1 as i32 as i64)
                                            << (::core::mem::size_of::<i64>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + (-(32767 as i32) - 1 as i32) as i64
                                        }) - 1 as i32 as i64
                                    })) as i32
                            } else {
                                ((0 as i32 as i64)
                                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                        + (-(32767 as i32) - 1 as i32) as i64) as i32
                            }) != 0 && in_over_5 == -(1 as i32) as i64
                            {
                                (if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    - 1 as i32) < 0 as i32
                                {
                                    ((0 as i32) < 8 as i32 + (-(32767 as i32) - 1 as i32))
                                        as i32
                                } else {
                                    (-(1 as i32) - (-(32767 as i32) - 1 as i32)
                                        < 8 as i32 - 1 as i32) as i32
                                })
                            } else {
                                ((-(32767 as i32) - 1 as i32) as i64 / in_over_5
                                    < 8 as i32 as i64) as i32
                            })
                        } else {
                            (((32767 as i32 / 8 as i32) as i64) < in_over_5) as i32
                        })
                    })
                }) != 0
                {
                    outlen = (in_over_5 as u32).wrapping_mul(8 as i32 as u32)
                        as libc::c_short as idx_t;
                    1 as i32
                } else {
                    outlen = (in_over_5 as u32).wrapping_mul(8 as i32 as u32)
                        as libc::c_short as idx_t;
                    0 as i32
                })
            } else {
                (if (if (8 as i32) < 0 as i32 {
                    (if in_over_5 < 0 as i32 as i64 {
                        (if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                32767 as i32 * 2 as i32 + 1 as i32
                            }) + 8 as i32
                        }) - 1 as i32) < 0 as i32
                        {
                            (in_over_5
                                < ((32767 as i32 * 2 as i32 + 1 as i32) / 8 as i32) as i64)
                                as i32
                        } else {
                            ((if (if (if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                8 as i32
                            }) - 1 as i32) < 0 as i32
                            {
                                !(((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32)
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                            }) < 0 as i32
                            {
                                ((8 as i32)
                                    < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        - 1 as i32) < 0 as i32
                                    {
                                        ((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) - 1 as i32
                                    })) as i32
                            } else {
                                ((0 as i32) < 8 as i32) as i32
                            }) != 0
                            {
                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    + (32767 as i32 * 2 as i32 + 1 as i32)
                                    >> (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(1 as i32 as u64)
                            } else {
                                (32767 as i32 * 2 as i32 + 1 as i32) / -(8 as i32)
                            }) as i64 <= -(1 as i32) as i64 - in_over_5) as i32
                        })
                    } else {
                        (if (if (if ((if 1 as i32 != 0 {
                            0 as i32
                        } else {
                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                        }) - 1 as i32) < 0 as i32
                        {
                            !(((((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                            }) + 1 as i32)
                                << (::core::mem::size_of::<i32>() as u64)
                                    .wrapping_mul(8 as i32 as u64)
                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                + 1 as i32)
                        } else {
                            (if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                            }) + 0 as i32
                        }) < 0 as i32
                        {
                            (((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                + 0 as i32)
                                < -(if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                }) - 1 as i32) < 0 as i32
                                {
                                    ((((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                    }) + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                    }) - 1 as i32
                                })) as i32
                        } else {
                            ((0 as i32)
                                < (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    + 0 as i32) as i32
                        }) != 0 && 8 as i32 == -(1 as i32)
                        {
                            (if ((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                in_over_5
                            }) - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                ((0 as i32 as i64) < in_over_5 + 0 as i32 as i64) as i32
                            } else {
                                ((0 as i32 as i64) < in_over_5
                                    && ((-(1 as i32) - 0 as i32) as i64)
                                        < in_over_5 - 1 as i32 as i64) as i32
                            })
                        } else {
                            (((0 as i32 / 8 as i32) as i64) < in_over_5) as i32
                        })
                    })
                } else {
                    (if 8 as i32 == 0 as i32 {
                        0 as i32
                    } else {
                        (if in_over_5 < 0 as i32 as i64 {
                            (if (if (if ((if 1 as i32 != 0 {
                                0 as i32 as i64
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                    + 0 as i32 as i64
                            }) - 1 as i32 as i64) < 0 as i32 as i64
                            {
                                !(((((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                        + 0 as i32 as i64
                                }) + 1 as i32 as i64)
                                    << (::core::mem::size_of::<i64>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                    * 2 as i32 as i64 + 1 as i32 as i64)
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                        + 0 as i32 as i64
                                }) + 0 as i32 as i64
                            }) < 0 as i32 as i64
                            {
                                (((if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                    + 0 as i32 as i64)
                                    < -(if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + 0 as i32 as i64
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + 0 as i32 as i64
                                        }) + 1 as i32 as i64)
                                            << (::core::mem::size_of::<i64>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + 0 as i32 as i64
                                        }) - 1 as i32 as i64
                                    })) as i32
                            } else {
                                ((0 as i32 as i64)
                                    < (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                        + 0 as i32 as i64) as i32
                            }) != 0 && in_over_5 == -(1 as i32) as i64
                            {
                                (if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    - 1 as i32) < 0 as i32
                                {
                                    ((0 as i32) < 8 as i32 + 0 as i32) as i32
                                } else {
                                    ((-(1 as i32) - 0 as i32) < 8 as i32 - 1 as i32) as i32
                                })
                            } else {
                                (0 as i32 as i64 / in_over_5 < 8 as i32 as i64) as i32
                            })
                        } else {
                            ((((32767 as i32 * 2 as i32 + 1 as i32) / 8 as i32) as i64)
                                < in_over_5) as i32
                        })
                    })
                }) != 0
                {
                    outlen = (in_over_5 as u32).wrapping_mul(8 as i32 as u32)
                        as libc::c_ushort as idx_t;
                    1 as i32
                } else {
                    outlen = (in_over_5 as u32).wrapping_mul(8 as i32 as u32)
                        as libc::c_ushort as idx_t;
                    0 as i32
                })
            })
        } else {
            (if ::core::mem::size_of::<idx_t>() as u64
                == ::core::mem::size_of::<i32>() as u64
            {
                (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { outlen })
                    - 1 as i32 as i64) < 0 as i32 as i64
                {
                    (if (if (8 as i32) < 0 as i32 {
                        (if in_over_5 < 0 as i32 as i64 {
                            (if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 2147483647 as i32 })
                                    + 8 as i32
                            }) - 1 as i32) < 0 as i32
                            {
                                (in_over_5 < (2147483647 as i32 / 8 as i32) as i64) as i32
                            } else {
                                ((if (if (if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    8 as i32
                                }) - 1 as i32) < 0 as i32
                                {
                                    !(((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32)
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                }) < 0 as i32
                                {
                                    ((8 as i32)
                                        < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                + 1 as i32)
                                                << (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                + 1 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) - 1 as i32
                                        })) as i32
                                } else {
                                    ((0 as i32) < 8 as i32) as i32
                                }) != 0
                                {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + 2147483647 as i32
                                        >> (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                } else {
                                    2147483647 as i32 / -(8 as i32)
                                }) as i64 <= -(1 as i32) as i64 - in_over_5) as i32
                            })
                        } else {
                            (if (if (if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    + (-(2147483647 as i32) - 1 as i32)
                            }) - 1 as i32) < 0 as i32
                            {
                                !(((((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + (-(2147483647 as i32) - 1 as i32)
                                }) + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32)
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + (-(2147483647 as i32) - 1 as i32)
                                }) + 0 as i32
                            }) < 0 as i32
                            {
                                ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    + (-(2147483647 as i32) - 1 as i32)
                                    < -(if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            + (-(2147483647 as i32) - 1 as i32)
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        ((((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                + (-(2147483647 as i32) - 1 as i32)
                                        }) + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                + (-(2147483647 as i32) - 1 as i32)
                                        }) - 1 as i32
                                    })) as i32
                            } else {
                                ((0 as i32)
                                    < (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + (-(2147483647 as i32) - 1 as i32)) as i32
                            }) != 0 && 8 as i32 == -(1 as i32)
                            {
                                (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    in_over_5
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    ((0 as i32 as i64)
                                        < in_over_5 + (-(2147483647 as i32) - 1 as i32) as i64)
                                        as i32
                                } else {
                                    ((0 as i32 as i64) < in_over_5
                                        && ((-(1 as i32) - (-(2147483647 as i32) - 1 as i32))
                                            as i64) < in_over_5 - 1 as i32 as i64) as i32
                                })
                            } else {
                                ((((-(2147483647 as i32) - 1 as i32) / 8 as i32) as i64)
                                    < in_over_5) as i32
                            })
                        })
                    } else {
                        (if 8 as i32 == 0 as i32 {
                            0 as i32
                        } else {
                            (if in_over_5 < 0 as i32 as i64 {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                        + (-(2147483647 as i32) - 1 as i32) as i64
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + (-(2147483647 as i32) - 1 as i32) as i64
                                    }) + 1 as i32 as i64)
                                        << (::core::mem::size_of::<i64>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64)
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + (-(2147483647 as i32) - 1 as i32) as i64
                                    }) + 0 as i32 as i64
                                }) < 0 as i32 as i64
                                {
                                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                        + (-(2147483647 as i32) - 1 as i32) as i64)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + (-(2147483647 as i32) - 1 as i32) as i64
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                    + (-(2147483647 as i32) - 1 as i32) as i64
                                            }) + 1 as i32 as i64)
                                                << (::core::mem::size_of::<i64>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                * 2 as i32 as i64 + 1 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                    + (-(2147483647 as i32) - 1 as i32) as i64
                                            }) - 1 as i32 as i64
                                        })) as i32
                                } else {
                                    ((0 as i32 as i64)
                                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + (-(2147483647 as i32) - 1 as i32) as i64) as i32
                                }) != 0 && in_over_5 == -(1 as i32) as i64
                                {
                                    (if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        - 1 as i32) < 0 as i32
                                    {
                                        ((0 as i32) < 8 as i32 + (-(2147483647 as i32) - 1 as i32))
                                            as i32
                                    } else {
                                        (-(1 as i32) - (-(2147483647 as i32) - 1 as i32)
                                            < 8 as i32 - 1 as i32) as i32
                                    })
                                } else {
                                    ((-(2147483647 as i32) - 1 as i32) as i64 / in_over_5
                                        < 8 as i32 as i64) as i32
                                })
                            } else {
                                (((2147483647 as i32 / 8 as i32) as i64) < in_over_5) as i32
                            })
                        })
                    }) != 0
                    {
                        outlen = (in_over_5 as u32).wrapping_mul(8 as i32 as u32) as i32
                            as idx_t;
                        1 as i32
                    } else {
                        outlen = (in_over_5 as u32).wrapping_mul(8 as i32 as u32) as i32
                            as idx_t;
                        0 as i32
                    })
                } else {
                    (if (if (8 as i32) < 0 as i32 {
                        (if in_over_5 < 0 as i32 as i64 {
                            (if (if 1 as i32 != 0 {
                                0 as i32 as u32
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32 as u32
                                } else {
                                    (2147483647 as i32 as u32)
                                        .wrapping_mul(2 as u32)
                                        .wrapping_add(1 as u32)
                                })
                                    .wrapping_add(8 as i32 as u32)
                            })
                                .wrapping_sub(1 as i32 as u32) < 0 as i32 as u32
                            {
                                (in_over_5
                                    < (2147483647 as i32 as u32)
                                        .wrapping_mul(2 as u32)
                                        .wrapping_add(1 as u32)
                                        .wrapping_div(8 as i32 as u32) as i64) as i32
                            } else {
                                ((if (if (if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    8 as i32
                                }) - 1 as i32) < 0 as i32
                                {
                                    !(((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32)
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                }) < 0 as i32
                                {
                                    ((8 as i32)
                                        < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                + 1 as i32)
                                                << (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                + 1 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) - 1 as i32
                                        })) as i32
                                } else {
                                    ((0 as i32) < 8 as i32) as i32
                                }) != 0
                                {
                                    ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) as u32)
                                        .wrapping_add(
                                            (2147483647 as i32 as u32)
                                                .wrapping_mul(2 as u32)
                                                .wrapping_add(1 as u32),
                                        )
                                        >> (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(1 as i32 as u64)
                                } else {
                                    (2147483647 as i32 as u32)
                                        .wrapping_mul(2 as u32)
                                        .wrapping_add(1 as u32)
                                        .wrapping_div(-(8 as i32) as u32)
                                }) as i64 <= -(1 as i32) as i64 - in_over_5) as i32
                            })
                        } else {
                            (if (if (if ((if 1 as i32 != 0 {
                                0 as i32
                            } else {
                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                            }) - 1 as i32) < 0 as i32
                            {
                                !(((((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                }) + 1 as i32)
                                    << (::core::mem::size_of::<i32>() as u64)
                                        .wrapping_mul(8 as i32 as u64)
                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                    + 1 as i32)
                            } else {
                                (if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                }) + 0 as i32
                            }) < 0 as i32
                            {
                                (((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                    + 0 as i32)
                                    < -(if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        ((((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                        }) + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                        }) - 1 as i32
                                    })) as i32
                            } else {
                                ((0 as i32)
                                    < (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + 0 as i32) as i32
                            }) != 0 && 8 as i32 == -(1 as i32)
                            {
                                (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    in_over_5
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    ((0 as i32 as i64) < in_over_5 + 0 as i32 as i64) as i32
                                } else {
                                    ((0 as i32 as i64) < in_over_5
                                        && ((-(1 as i32) - 0 as i32) as i64)
                                            < in_over_5 - 1 as i32 as i64) as i32
                                })
                            } else {
                                (((0 as i32 / 8 as i32) as i64) < in_over_5) as i32
                            })
                        })
                    } else {
                        (if 8 as i32 == 0 as i32 {
                            0 as i32
                        } else {
                            (if in_over_5 < 0 as i32 as i64 {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                        + 0 as i32 as i64
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + 0 as i32 as i64
                                    }) + 1 as i32 as i64)
                                        << (::core::mem::size_of::<i64>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64)
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + 0 as i32 as i64
                                    }) + 0 as i32 as i64
                                }) < 0 as i32 as i64
                                {
                                    (((if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                        + 0 as i32 as i64)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + 0 as i32 as i64
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                    + 0 as i32 as i64
                                            }) + 1 as i32 as i64)
                                                << (::core::mem::size_of::<i64>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                * 2 as i32 as i64 + 1 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                    + 0 as i32 as i64
                                            }) - 1 as i32 as i64
                                        })) as i32
                                } else {
                                    ((0 as i32 as i64)
                                        < (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + 0 as i32 as i64) as i32
                                }) != 0 && in_over_5 == -(1 as i32) as i64
                                {
                                    (if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        - 1 as i32) < 0 as i32
                                    {
                                        ((0 as i32) < 8 as i32 + 0 as i32) as i32
                                    } else {
                                        ((-(1 as i32) - 0 as i32) < 8 as i32 - 1 as i32) as i32
                                    })
                                } else {
                                    (0 as i32 as i64 / in_over_5 < 8 as i32 as i64) as i32
                                })
                            } else {
                                (((2147483647 as i32 as u32)
                                    .wrapping_mul(2 as u32)
                                    .wrapping_add(1 as u32)
                                    .wrapping_div(8 as i32 as u32) as i64) < in_over_5) as i32
                            })
                        })
                    }) != 0
                    {
                        outlen = (in_over_5 as u32).wrapping_mul(8 as i32 as u32)
                            as idx_t;
                        1 as i32
                    } else {
                        outlen = (in_over_5 as u32).wrapping_mul(8 as i32 as u32)
                            as idx_t;
                        0 as i32
                    })
                })
            } else {
                (if ::core::mem::size_of::<idx_t>() as u64
                    == ::core::mem::size_of::<i64>() as u64
                {
                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { outlen })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        (if (if (8 as i32) < 0 as i32 {
                            (if in_over_5 < 0 as i32 as i64 {
                                (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        9223372036854775807 as i64
                                    }) + 8 as i32 as i64
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    (in_over_5 < 9223372036854775807 as i64 / 8 as i32 as i64)
                                        as i32
                                } else {
                                    ((if (if (if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        8 as i32
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        !(((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32)
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                    }) < 0 as i32
                                    {
                                        ((8 as i32)
                                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                - 1 as i32) < 0 as i32
                                            {
                                                ((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                    + 1 as i32)
                                                    << (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                    + 1 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) - 1 as i32
                                            })) as i32
                                    } else {
                                        ((0 as i32) < 8 as i32) as i32
                                    }) != 0
                                    {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) as i64
                                            + 9223372036854775807 as i64
                                            >> (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64)
                                    } else {
                                        9223372036854775807 as i64 / -(8 as i32) as i64
                                    }) <= -(1 as i32) as i64 - in_over_5) as i32
                                })
                            } else {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32 as i64
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) as i64
                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                }) - 1 as i32 as i64) < 0 as i32 as i64
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) as i64
                                            + (-(9223372036854775807 as i64) - 1 as i64)
                                    }) + 1 as i32 as i64)
                                        << (::core::mem::size_of::<i64>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                        * 2 as i32 as i64 + 1 as i32 as i64)
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) as i64
                                            + (-(9223372036854775807 as i64) - 1 as i64)
                                    }) + 0 as i32 as i64
                                }) < 0 as i32 as i64
                                {
                                    ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) as i64
                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) as i64
                                                + (-(9223372036854775807 as i64) - 1 as i64)
                                        }) - 1 as i32 as i64) < 0 as i32 as i64
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) as i64
                                                    + (-(9223372036854775807 as i64) - 1 as i64)
                                            }) + 1 as i32 as i64)
                                                << (::core::mem::size_of::<i64>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                * 2 as i32 as i64 + 1 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) as i64
                                                    + (-(9223372036854775807 as i64) - 1 as i64)
                                            }) - 1 as i32 as i64
                                        })) as i32
                                } else {
                                    ((0 as i32 as i64)
                                        < (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) as i64
                                            + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                }) != 0 && 8 as i32 == -(1 as i32)
                                {
                                    (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        in_over_5
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((0 as i32 as i64)
                                            < in_over_5 + (-(9223372036854775807 as i64) - 1 as i64))
                                            as i32
                                    } else {
                                        ((0 as i32 as i64) < in_over_5
                                            && -(1 as i32) as i64
                                                - (-(9223372036854775807 as i64) - 1 as i64)
                                                < in_over_5 - 1 as i32 as i64) as i32
                                    })
                                } else {
                                    (((-(9223372036854775807 as i64) - 1 as i64)
                                        / 8 as i32 as i64) < in_over_5) as i32
                                })
                            })
                        } else {
                            (if 8 as i32 == 0 as i32 {
                                0 as i32
                            } else {
                                (if in_over_5 < 0 as i32 as i64 {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + (-(9223372036854775807 as i64) - 1 as i64)
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + (-(9223372036854775807 as i64) - 1 as i64)
                                        }) + 1 as i32 as i64)
                                            << (::core::mem::size_of::<i64>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64)
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + (-(9223372036854775807 as i64) - 1 as i64)
                                        }) + 0 as i32 as i64
                                    }) < 0 as i32 as i64
                                    {
                                        ((if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + (-(9223372036854775807 as i64) - 1 as i64)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                    + (-(9223372036854775807 as i64) - 1 as i64)
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                                }) + 1 as i32 as i64)
                                                    << (::core::mem::size_of::<i64>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                    * 2 as i32 as i64 + 1 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                        + (-(9223372036854775807 as i64) - 1 as i64)
                                                }) - 1 as i32 as i64
                                            })) as i32
                                    } else {
                                        ((0 as i32 as i64)
                                            < (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                    }) != 0 && in_over_5 == -(1 as i32) as i64
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((0 as i32 as i64)
                                                < 8 as i32 as i64
                                                    + (-(9223372036854775807 as i64) - 1 as i64)) as i32
                                        } else {
                                            (-(1 as i32) as i64
                                                - (-(9223372036854775807 as i64) - 1 as i64)
                                                < (8 as i32 - 1 as i32) as i64) as i32
                                        })
                                    } else {
                                        ((-(9223372036854775807 as i64) - 1 as i64) / in_over_5
                                            < 8 as i32 as i64) as i32
                                    })
                                } else {
                                    ((9223372036854775807 as i64 / 8 as i32 as i64) < in_over_5)
                                        as i32
                                })
                            })
                        }) != 0
                        {
                            outlen = (in_over_5 as u64).wrapping_mul(8 as i32 as u64)
                                as i64;
                            1 as i32
                        } else {
                            outlen = (in_over_5 as u64).wrapping_mul(8 as i32 as u64)
                                as i64;
                            0 as i32
                        })
                    } else {
                        (if (if (8 as i32) < 0 as i32 {
                            (if in_over_5 < 0 as i32 as i64 {
                                (if (if 1 as i32 != 0 {
                                    0 as i32 as u64
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as u64
                                    } else {
                                        (9223372036854775807 as i64 as u64)
                                            .wrapping_mul(2 as u64)
                                            .wrapping_add(1 as u64)
                                    })
                                        .wrapping_add(8 as i32 as u64)
                                })
                                    .wrapping_sub(1 as i32 as u64) < 0 as i32 as u64
                                {
                                    ((in_over_5 as u64)
                                        < (9223372036854775807 as i64 as u64)
                                            .wrapping_mul(2 as u64)
                                            .wrapping_add(1 as u64)
                                            .wrapping_div(8 as i32 as u64)) as i32
                                } else {
                                    ((if (if (if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        8 as i32
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        !(((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32)
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                    }) < 0 as i32
                                    {
                                        ((8 as i32)
                                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                - 1 as i32) < 0 as i32
                                            {
                                                ((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                    + 1 as i32)
                                                    << (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                    + 1 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) - 1 as i32
                                            })) as i32
                                    } else {
                                        ((0 as i32) < 8 as i32) as i32
                                    }) != 0
                                    {
                                        ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) as u64)
                                            .wrapping_add(
                                                (9223372036854775807 as i64 as u64)
                                                    .wrapping_mul(2 as u64)
                                                    .wrapping_add(1 as u64),
                                            )
                                            >> (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64)
                                    } else {
                                        (9223372036854775807 as i64 as u64)
                                            .wrapping_mul(2 as u64)
                                            .wrapping_add(1 as u64)
                                            .wrapping_div(-(8 as i32) as u64)
                                    }) <= (-(1 as i32) as i64 - in_over_5) as u64) as i32
                                })
                            } else {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                }) - 1 as i32) < 0 as i32
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                    }) + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32)
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                    }) + 0 as i32
                                }) < 0 as i32
                                {
                                    (((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + 0 as i32)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                        }) - 1 as i32) < 0 as i32
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                            }) + 1 as i32)
                                                << (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                + 1 as i32
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                            }) - 1 as i32
                                        })) as i32
                                } else {
                                    ((0 as i32)
                                        < (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            + 0 as i32) as i32
                                }) != 0 && 8 as i32 == -(1 as i32)
                                {
                                    (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        in_over_5
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((0 as i32 as i64) < in_over_5 + 0 as i32 as i64) as i32
                                    } else {
                                        ((0 as i32 as i64) < in_over_5
                                            && ((-(1 as i32) - 0 as i32) as i64)
                                                < in_over_5 - 1 as i32 as i64) as i32
                                    })
                                } else {
                                    (((0 as i32 / 8 as i32) as i64) < in_over_5) as i32
                                })
                            })
                        } else {
                            (if 8 as i32 == 0 as i32 {
                                0 as i32
                            } else {
                                (if in_over_5 < 0 as i32 as i64 {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + 0 as i32 as i64
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + 0 as i32 as i64
                                        }) + 1 as i32 as i64)
                                            << (::core::mem::size_of::<i64>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64)
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + 0 as i32 as i64
                                        }) + 0 as i32 as i64
                                    }) < 0 as i32 as i64
                                    {
                                        (((if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + 0 as i32 as i64)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                    + 0 as i32 as i64
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                        + 0 as i32 as i64
                                                }) + 1 as i32 as i64)
                                                    << (::core::mem::size_of::<i64>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                    * 2 as i32 as i64 + 1 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                        + 0 as i32 as i64
                                                }) - 1 as i32 as i64
                                            })) as i32
                                    } else {
                                        ((0 as i32 as i64)
                                            < (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + 0 as i32 as i64) as i32
                                    }) != 0 && in_over_5 == -(1 as i32) as i64
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((0 as i32) < 8 as i32 + 0 as i32) as i32
                                        } else {
                                            ((-(1 as i32) - 0 as i32) < 8 as i32 - 1 as i32) as i32
                                        })
                                    } else {
                                        (0 as i32 as i64 / in_over_5 < 8 as i32 as i64) as i32
                                    })
                                } else {
                                    ((9223372036854775807 as i64 as u64)
                                        .wrapping_mul(2 as u64)
                                        .wrapping_add(1 as u64)
                                        .wrapping_div(8 as i32 as u64) < in_over_5 as u64) as i32
                                })
                            })
                        }) != 0
                        {
                            outlen = (in_over_5 as u64).wrapping_mul(8 as i32 as u64)
                                as idx_t;
                            1 as i32
                        } else {
                            outlen = (in_over_5 as u64).wrapping_mul(8 as i32 as u64)
                                as idx_t;
                            0 as i32
                        })
                    })
                } else {
                    (if ((if 1 as i32 != 0 { 0 as i32 as i64 } else { outlen })
                        - 1 as i32 as i64) < 0 as i32 as i64
                    {
                        (if (if (8 as i32) < 0 as i32 {
                            (if in_over_5 < 0 as i32 as i64 {
                                (if ((if 1 as i32 != 0 {
                                    0 as i32 as libc::c_longlong
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as libc::c_longlong
                                    } else {
                                        9223372036854775807 as libc::c_longlong
                                    }) + 8 as i32 as libc::c_longlong
                                }) - 1 as i32 as libc::c_longlong)
                                    < 0 as i32 as libc::c_longlong
                                {
                                    ((in_over_5 as libc::c_longlong)
                                        < 9223372036854775807 as libc::c_longlong
                                            / 8 as i32 as libc::c_longlong) as i32
                                } else {
                                    ((if (if (if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        8 as i32
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        !(((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32)
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                    }) < 0 as i32
                                    {
                                        ((8 as i32)
                                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                - 1 as i32) < 0 as i32
                                            {
                                                ((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                    + 1 as i32)
                                                    << (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                    + 1 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) - 1 as i32
                                            })) as i32
                                    } else {
                                        ((0 as i32) < 8 as i32) as i32
                                    }) != 0
                                    {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            as libc::c_longlong
                                            + 9223372036854775807 as libc::c_longlong
                                            >> (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64)
                                    } else {
                                        9223372036854775807 as libc::c_longlong
                                            / -(8 as i32) as libc::c_longlong
                                    }) <= (-(1 as i32) as i64 - in_over_5) as libc::c_longlong)
                                        as i32
                                })
                            } else {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32 as libc::c_longlong
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        as libc::c_longlong
                                        + (-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong)
                                }) - 1 as i32 as libc::c_longlong)
                                    < 0 as i32 as libc::c_longlong
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32 as libc::c_longlong
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                    }) + 1 as i32 as libc::c_longlong)
                                        << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64))
                                        - 1 as i32 as libc::c_longlong)
                                        * 2 as i32 as libc::c_longlong
                                        + 1 as i32 as libc::c_longlong)
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as libc::c_longlong
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                    }) + 0 as i32 as libc::c_longlong
                                }) < 0 as i32 as libc::c_longlong
                                {
                                    ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        as libc::c_longlong
                                        + (-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32 as libc::c_longlong
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                        }) - 1 as i32 as libc::c_longlong)
                                            < 0 as i32 as libc::c_longlong
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32 as libc::c_longlong
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                    as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                            }) + 1 as i32 as libc::c_longlong)
                                                << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64))
                                                - 1 as i32 as libc::c_longlong)
                                                * 2 as i32 as libc::c_longlong
                                                + 1 as i32 as libc::c_longlong
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32 as libc::c_longlong
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                    as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                            }) - 1 as i32 as libc::c_longlong
                                        })) as i32
                                } else {
                                    ((0 as i32 as libc::c_longlong)
                                        < (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)) as i32
                                }) != 0 && 8 as i32 == -(1 as i32)
                                {
                                    (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        in_over_5
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((0 as i32 as libc::c_longlong)
                                            < in_over_5 as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)) as i32
                                    } else {
                                        ((0 as i32 as i64) < in_over_5
                                            && -(1 as i32) as libc::c_longlong
                                                - (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                                < (in_over_5 - 1 as i32 as i64) as libc::c_longlong) as i32
                                    })
                                } else {
                                    (((-(9223372036854775807 as libc::c_longlong)
                                        - 1 as libc::c_longlong) / 8 as i32 as libc::c_longlong)
                                        < in_over_5 as libc::c_longlong) as i32
                                })
                            })
                        } else {
                            (if 8 as i32 == 0 as i32 {
                                0 as i32
                            } else {
                                (if in_over_5 < 0 as i32 as i64 {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as libc::c_longlong
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                    }) - 1 as i32 as libc::c_longlong)
                                        < 0 as i32 as libc::c_longlong
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32 as libc::c_longlong
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                        }) + 1 as i32 as libc::c_longlong)
                                            << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64))
                                            - 1 as i32 as libc::c_longlong)
                                            * 2 as i32 as libc::c_longlong
                                            + 1 as i32 as libc::c_longlong)
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as libc::c_longlong
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                        }) + 0 as i32 as libc::c_longlong
                                    }) < 0 as i32 as libc::c_longlong
                                    {
                                        ((if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            as libc::c_longlong
                                            + (-(9223372036854775807 as libc::c_longlong)
                                                - 1 as libc::c_longlong)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32 as libc::c_longlong
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                    as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)
                                            }) - 1 as i32 as libc::c_longlong)
                                                < 0 as i32 as libc::c_longlong
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32 as libc::c_longlong
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                        as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) + 1 as i32 as libc::c_longlong)
                                                    << (::core::mem::size_of::<libc::c_longlong>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64))
                                                    - 1 as i32 as libc::c_longlong)
                                                    * 2 as i32 as libc::c_longlong
                                                    + 1 as i32 as libc::c_longlong
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as libc::c_longlong
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                        as libc::c_longlong
                                                        + (-(9223372036854775807 as libc::c_longlong)
                                                            - 1 as libc::c_longlong)
                                                }) - 1 as i32 as libc::c_longlong
                                            })) as i32
                                    } else {
                                        ((0 as i32 as libc::c_longlong)
                                            < (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                as libc::c_longlong
                                                + (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)) as i32
                                    }) != 0 && in_over_5 == -(1 as i32) as i64
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((0 as i32 as libc::c_longlong)
                                                < 8 as i32 as libc::c_longlong
                                                    + (-(9223372036854775807 as libc::c_longlong)
                                                        - 1 as libc::c_longlong)) as i32
                                        } else {
                                            (-(1 as i32) as libc::c_longlong
                                                - (-(9223372036854775807 as libc::c_longlong)
                                                    - 1 as libc::c_longlong)
                                                < (8 as i32 - 1 as i32) as libc::c_longlong) as i32
                                        })
                                    } else {
                                        (((-(9223372036854775807 as libc::c_longlong)
                                            - 1 as libc::c_longlong) / in_over_5 as libc::c_longlong)
                                            < 8 as i32 as libc::c_longlong) as i32
                                    })
                                } else {
                                    ((9223372036854775807 as libc::c_longlong
                                        / 8 as i32 as libc::c_longlong)
                                        < in_over_5 as libc::c_longlong) as i32
                                })
                            })
                        }) != 0
                        {
                            outlen = (in_over_5 as libc::c_ulonglong)
                                .wrapping_mul(8 as i32 as libc::c_ulonglong)
                                as libc::c_longlong as idx_t;
                            1 as i32
                        } else {
                            outlen = (in_over_5 as libc::c_ulonglong)
                                .wrapping_mul(8 as i32 as libc::c_ulonglong)
                                as libc::c_longlong as idx_t;
                            0 as i32
                        })
                    } else {
                        (if (if (8 as i32) < 0 as i32 {
                            (if in_over_5 < 0 as i32 as i64 {
                                (if (if 1 as i32 != 0 {
                                    0 as i32 as libc::c_ulonglong
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32 as libc::c_ulonglong
                                    } else {
                                        (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_ulonglong)
                                    })
                                        .wrapping_add(8 as i32 as libc::c_ulonglong)
                                })
                                    .wrapping_sub(1 as i32 as libc::c_ulonglong)
                                    < 0 as i32 as libc::c_ulonglong
                                {
                                    ((in_over_5 as libc::c_ulonglong)
                                        < (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_ulonglong)
                                            .wrapping_div(8 as i32 as libc::c_ulonglong)) as i32
                                } else {
                                    ((if (if (if ((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        8 as i32
                                    }) - 1 as i32) < 0 as i32
                                    {
                                        !(((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            + 1 as i32)
                                            << (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                            + 1 as i32)
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                    }) < 0 as i32
                                    {
                                        ((8 as i32)
                                            < -(if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                - 1 as i32) < 0 as i32
                                            {
                                                ((((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                                    + 1 as i32)
                                                    << (::core::mem::size_of::<i32>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                    + 1 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) - 1 as i32
                                            })) as i32
                                    } else {
                                        ((0 as i32) < 8 as i32) as i32
                                    }) != 0
                                    {
                                        ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            as libc::c_ulonglong)
                                            .wrapping_add(
                                                (9223372036854775807 as libc::c_longlong
                                                    as libc::c_ulonglong)
                                                    .wrapping_mul(2 as libc::c_ulonglong)
                                                    .wrapping_add(1 as libc::c_ulonglong),
                                            )
                                            >> (::core::mem::size_of::<i32>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(1 as i32 as u64)
                                    } else {
                                        (9223372036854775807 as libc::c_longlong
                                            as libc::c_ulonglong)
                                            .wrapping_mul(2 as libc::c_ulonglong)
                                            .wrapping_add(1 as libc::c_ulonglong)
                                            .wrapping_div(-(8 as i32) as libc::c_ulonglong)
                                    }) <= (-(1 as i32) as i64 - in_over_5) as libc::c_ulonglong)
                                        as i32
                                })
                            } else {
                                (if (if (if ((if 1 as i32 != 0 {
                                    0 as i32
                                } else {
                                    (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                }) - 1 as i32) < 0 as i32
                                {
                                    !(((((if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                    }) + 1 as i32)
                                        << (::core::mem::size_of::<i32>() as u64)
                                            .wrapping_mul(8 as i32 as u64)
                                            .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                        + 1 as i32)
                                } else {
                                    (if 1 as i32 != 0 {
                                        0 as i32
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                    }) + 0 as i32
                                }) < 0 as i32
                                {
                                    (((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                        + 0 as i32)
                                        < -(if ((if 1 as i32 != 0 {
                                            0 as i32
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                        }) - 1 as i32) < 0 as i32
                                        {
                                            ((((if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                            }) + 1 as i32)
                                                << (::core::mem::size_of::<i32>() as u64)
                                                    .wrapping_mul(8 as i32 as u64)
                                                    .wrapping_sub(2 as i32 as u64)) - 1 as i32) * 2 as i32
                                                + 1 as i32
                                        } else {
                                            (if 1 as i32 != 0 {
                                                0 as i32
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 }) + 0 as i32
                                            }) - 1 as i32
                                        })) as i32
                                } else {
                                    ((0 as i32)
                                        < (if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            + 0 as i32) as i32
                                }) != 0 && 8 as i32 == -(1 as i32)
                                {
                                    (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        in_over_5
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        ((0 as i32 as i64) < in_over_5 + 0 as i32 as i64) as i32
                                    } else {
                                        ((0 as i32 as i64) < in_over_5
                                            && ((-(1 as i32) - 0 as i32) as i64)
                                                < in_over_5 - 1 as i32 as i64) as i32
                                    })
                                } else {
                                    (((0 as i32 / 8 as i32) as i64) < in_over_5) as i32
                                })
                            })
                        } else {
                            (if 8 as i32 == 0 as i32 {
                                0 as i32
                            } else {
                                (if in_over_5 < 0 as i32 as i64 {
                                    (if (if (if ((if 1 as i32 != 0 {
                                        0 as i32 as i64
                                    } else {
                                        (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + 0 as i32 as i64
                                    }) - 1 as i32 as i64) < 0 as i32 as i64
                                    {
                                        !(((((if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + 0 as i32 as i64
                                        }) + 1 as i32 as i64)
                                            << (::core::mem::size_of::<i64>() as u64)
                                                .wrapping_mul(8 as i32 as u64)
                                                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                            * 2 as i32 as i64 + 1 as i32 as i64)
                                    } else {
                                        (if 1 as i32 != 0 {
                                            0 as i32 as i64
                                        } else {
                                            (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + 0 as i32 as i64
                                        }) + 0 as i32 as i64
                                    }) < 0 as i32 as i64
                                    {
                                        (((if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                            + 0 as i32 as i64)
                                            < -(if ((if 1 as i32 != 0 {
                                                0 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                    + 0 as i32 as i64
                                            }) - 1 as i32 as i64) < 0 as i32 as i64
                                            {
                                                ((((if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                        + 0 as i32 as i64
                                                }) + 1 as i32 as i64)
                                                    << (::core::mem::size_of::<i64>() as u64)
                                                        .wrapping_mul(8 as i32 as u64)
                                                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                                                    * 2 as i32 as i64 + 1 as i32 as i64
                                            } else {
                                                (if 1 as i32 != 0 {
                                                    0 as i32 as i64
                                                } else {
                                                    (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                        + 0 as i32 as i64
                                                }) - 1 as i32 as i64
                                            })) as i32
                                    } else {
                                        ((0 as i32 as i64)
                                            < (if 1 as i32 != 0 { 0 as i32 as i64 } else { in_over_5 })
                                                + 0 as i32 as i64) as i32
                                    }) != 0 && in_over_5 == -(1 as i32) as i64
                                    {
                                        (if ((if 1 as i32 != 0 { 0 as i32 } else { 8 as i32 })
                                            - 1 as i32) < 0 as i32
                                        {
                                            ((0 as i32) < 8 as i32 + 0 as i32) as i32
                                        } else {
                                            ((-(1 as i32) - 0 as i32) < 8 as i32 - 1 as i32) as i32
                                        })
                                    } else {
                                        (0 as i32 as i64 / in_over_5 < 8 as i32 as i64) as i32
                                    })
                                } else {
                                    ((9223372036854775807 as libc::c_longlong
                                        as libc::c_ulonglong)
                                        .wrapping_mul(2 as libc::c_ulonglong)
                                        .wrapping_add(1 as libc::c_ulonglong)
                                        .wrapping_div(8 as i32 as libc::c_ulonglong)
                                        < in_over_5 as libc::c_ulonglong) as i32
                                })
                            })
                        }) != 0
                        {
                            outlen = (in_over_5 as libc::c_ulonglong)
                                .wrapping_mul(8 as i32 as libc::c_ulonglong) as idx_t;
                            1 as i32
                        } else {
                            outlen = (in_over_5 as libc::c_ulonglong)
                                .wrapping_mul(8 as i32 as libc::c_ulonglong) as idx_t;
                            0 as i32
                        })
                    })
                })
            })
        })
    }) != 0 || inlen < 0 as i32 as i64
    {
        *out = 0 as *mut i8;
        return 0 as i32 as idx_t;
    }
    outlen += 1;
    outlen;
    *out = imalloc(outlen) as *mut i8;
    if (*out).is_null() {
        return outlen;
    }
    base32_encode(in_0, inlen, *out, outlen);
    return outlen - 1 as i32 as i64;
}
static mut b32: [libc::c_schar; 256] = [0; 256];
#[no_mangle]
pub unsafe extern "C" fn isbase32(mut ch: i8) -> bool {
    return 1 as i32 != 0 && 0 as i32 <= b32[to_uchar(ch) as usize] as i32;
}
#[no_mangle]
pub unsafe extern "C" fn base32_decode_ctx_init(mut ctx: *mut base32_decode_context) {
    (*ctx).i = 0 as i32;
}
unsafe extern "C" fn get_8(
    mut ctx: *mut base32_decode_context,
    mut in_0: *mut *const i8,
    mut in_end: *const i8,
    mut n_non_newline: *mut idx_t,
) -> *mut i8 {
    if (*ctx).i == 8 as i32 {
        (*ctx).i = 0 as i32;
    }
    if (*ctx).i == 0 as i32 {
        let mut t: *const i8 = *in_0;
        if 8 as i32 as i64 <= in_end.offset_from(*in_0) as i64
            && (memchr(t as *const libc::c_void, '\n' as i32, 8 as i32 as u64)).is_null()
        {
            *in_0 = (*in_0).offset(8 as i32 as isize);
            *n_non_newline = 8 as i32 as idx_t;
            return t as *mut i8;
        }
    }
    let mut p: *const i8 = *in_0;
    while p < in_end {
        let fresh8 = p;
        p = p.offset(1);
        let mut c: i8 = *fresh8;
        if !(c as i32 != '\n' as i32) {
            continue;
        }
        let fresh9 = (*ctx).i;
        (*ctx).i = (*ctx).i + 1;
        (*ctx).buf[fresh9 as usize] = c;
        if (*ctx).i == 8 as i32 {
            break;
        }
    }
    *in_0 = p;
    *n_non_newline = (*ctx).i as idx_t;
    return ((*ctx).buf).as_mut_ptr();
}
unsafe extern "C" fn decode_8(
    mut in_0: *const i8,
    mut inlen: idx_t,
    mut outp: *mut *mut i8,
    mut outleft: *mut idx_t,
) -> bool {
    let mut out: *mut i8 = *outp;
    if inlen < 8 as i32 as i64 {
        return 0 as i32 != 0;
    }
    if !isbase32(*in_0.offset(0 as i32 as isize))
        || !isbase32(*in_0.offset(1 as i32 as isize))
    {
        return 0 as i32 != 0;
    }
    if *outleft != 0 {
        let fresh10 = out;
        out = out.offset(1);
        *fresh10 = ((b32[to_uchar(*in_0.offset(0 as i32 as isize)) as usize] as i32)
            << 3 as i32
            | b32[to_uchar(*in_0.offset(1 as i32 as isize)) as usize] as i32 >> 2 as i32)
            as i8;
        *outleft -= 1;
        *outleft;
    }
    if *in_0.offset(2 as i32 as isize) as i32 == '=' as i32 {
        if *in_0.offset(3 as i32 as isize) as i32 != '=' as i32
            || *in_0.offset(4 as i32 as isize) as i32 != '=' as i32
            || *in_0.offset(5 as i32 as isize) as i32 != '=' as i32
            || *in_0.offset(6 as i32 as isize) as i32 != '=' as i32
            || *in_0.offset(7 as i32 as isize) as i32 != '=' as i32
        {
            *outp = out;
            return 0 as i32 != 0;
        }
    } else {
        if !isbase32(*in_0.offset(2 as i32 as isize))
            || !isbase32(*in_0.offset(3 as i32 as isize))
        {
            *outp = out;
            return 0 as i32 != 0;
        }
        if *outleft != 0 {
            let fresh11 = out;
            out = out.offset(1);
            *fresh11 = ((b32[to_uchar(*in_0.offset(1 as i32 as isize)) as usize] as i32)
                << 6 as i32
                | (b32[to_uchar(*in_0.offset(2 as i32 as isize)) as usize] as i32)
                    << 1 as i32
                | b32[to_uchar(*in_0.offset(3 as i32 as isize)) as usize] as i32
                    >> 4 as i32) as i8;
            *outleft -= 1;
            *outleft;
        }
        if *in_0.offset(4 as i32 as isize) as i32 == '=' as i32 {
            if *in_0.offset(5 as i32 as isize) as i32 != '=' as i32
                || *in_0.offset(6 as i32 as isize) as i32 != '=' as i32
                || *in_0.offset(7 as i32 as isize) as i32 != '=' as i32
            {
                *outp = out;
                return 0 as i32 != 0;
            }
        } else {
            if !isbase32(*in_0.offset(4 as i32 as isize)) {
                *outp = out;
                return 0 as i32 != 0;
            }
            if *outleft != 0 {
                let fresh12 = out;
                out = out.offset(1);
                *fresh12 = ((b32[to_uchar(*in_0.offset(3 as i32 as isize)) as usize]
                    as i32) << 4 as i32
                    | b32[to_uchar(*in_0.offset(4 as i32 as isize)) as usize] as i32
                        >> 1 as i32) as i8;
                *outleft -= 1;
                *outleft;
            }
            if *in_0.offset(5 as i32 as isize) as i32 == '=' as i32 {
                if *in_0.offset(6 as i32 as isize) as i32 != '=' as i32
                    || *in_0.offset(7 as i32 as isize) as i32 != '=' as i32
                {
                    *outp = out;
                    return 0 as i32 != 0;
                }
            } else {
                if !isbase32(*in_0.offset(5 as i32 as isize))
                    || !isbase32(*in_0.offset(6 as i32 as isize))
                {
                    *outp = out;
                    return 0 as i32 != 0;
                }
                if *outleft != 0 {
                    let fresh13 = out;
                    out = out.offset(1);
                    *fresh13 = ((b32[to_uchar(*in_0.offset(4 as i32 as isize)) as usize]
                        as i32) << 7 as i32
                        | (b32[to_uchar(*in_0.offset(5 as i32 as isize)) as usize]
                            as i32) << 2 as i32
                        | b32[to_uchar(*in_0.offset(6 as i32 as isize)) as usize] as i32
                            >> 3 as i32) as i8;
                    *outleft -= 1;
                    *outleft;
                }
                if *in_0.offset(7 as i32 as isize) as i32 != '=' as i32 {
                    if !isbase32(*in_0.offset(7 as i32 as isize)) {
                        *outp = out;
                        return 0 as i32 != 0;
                    }
                    if *outleft != 0 {
                        let fresh14 = out;
                        out = out.offset(1);
                        *fresh14 = ((b32[to_uchar(*in_0.offset(6 as i32 as isize))
                            as usize] as i32) << 5 as i32
                            | b32[to_uchar(*in_0.offset(7 as i32 as isize)) as usize]
                                as i32) as i8;
                        *outleft -= 1;
                        *outleft;
                    }
                }
            }
        }
    }
    *outp = out;
    return 1 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn base32_decode_ctx(
    mut ctx: *mut base32_decode_context,
    mut in_0: *const i8,
    mut inlen: idx_t,
    mut out: *mut i8,
    mut outlen: *mut idx_t,
) -> bool {
    let mut outleft: idx_t = *outlen;
    let mut ignore_newlines: bool = !ctx.is_null();
    let mut flush_ctx: bool = 0 as i32 != 0;
    let mut ctx_i: u32 = 0 as i32 as u32;
    if ignore_newlines {
        ctx_i = (*ctx).i as u32;
        flush_ctx = inlen == 0 as i32 as i64;
    }
    loop {
        let mut outleft_save: idx_t = outleft;
        if ctx_i == 0 as i32 as u32 && !flush_ctx {
            loop {
                outleft_save = outleft;
                if !decode_8(in_0, inlen, &mut out, &mut outleft) {
                    break;
                }
                in_0 = in_0.offset(8 as i32 as isize);
                inlen -= 8 as i32 as i64;
            }
        }
        if inlen == 0 as i32 as i64 && !flush_ctx {
            break;
        }
        if inlen != 0 && *in_0 as i32 == '\n' as i32 && ignore_newlines as i32 != 0 {
            in_0 = in_0.offset(1);
            in_0;
            inlen -= 1;
            inlen;
        } else {
            out = out.offset(-((outleft_save - outleft) as isize));
            outleft = outleft_save;
            let mut in_end: *const i8 = in_0.offset(inlen as isize);
            let mut non_nl: *const i8 = 0 as *const i8;
            if ignore_newlines {
                non_nl = get_8(ctx, &mut in_0, in_end, &mut inlen);
            } else {
                non_nl = in_0;
            }
            if inlen == 0 as i32 as i64
                || inlen < 8 as i32 as i64 && !flush_ctx && ignore_newlines as i32 != 0
            {
                inlen = 0 as i32 as idx_t;
                break;
            } else {
                if !decode_8(non_nl, inlen, &mut out, &mut outleft) {
                    break;
                }
                inlen = in_end.offset_from(in_0) as i64;
            }
        }
    }
    *outlen -= outleft;
    return inlen == 0 as i32 as i64;
}
#[no_mangle]
pub unsafe extern "C" fn base32_decode_alloc_ctx(
    mut ctx: *mut base32_decode_context,
    mut in_0: *const i8,
    mut inlen: idx_t,
    mut out: *mut *mut i8,
    mut outlen: *mut idx_t,
) -> bool {
    let mut needlen: idx_t = 5 as i32 as i64 * ((inlen >> 3 as i32) + 1 as i32 as i64);
    *out = imalloc(needlen) as *mut i8;
    if (*out).is_null() {
        return 1 as i32 != 0;
    }
    if !base32_decode_ctx(ctx, in_0, inlen, *out, &mut needlen) {
        rpl_free(*out as *mut libc::c_void);
        *out = 0 as *mut i8;
        return 0 as i32 != 0;
    }
    if !outlen.is_null() {
        *outlen = needlen;
    }
    return 1 as i32 != 0;
}
unsafe extern "C" fn run_static_initializers() {
    b32 = [
        (if 0 as i32 == 'A' as i32 {
            0 as i32
        } else if 0 as i32 == 'B' as i32 {
            1 as i32
        } else if 0 as i32 == 'C' as i32 {
            2 as i32
        } else if 0 as i32 == 'D' as i32 {
            3 as i32
        } else if 0 as i32 == 'E' as i32 {
            4 as i32
        } else if 0 as i32 == 'F' as i32 {
            5 as i32
        } else if 0 as i32 == 'G' as i32 {
            6 as i32
        } else if 0 as i32 == 'H' as i32 {
            7 as i32
        } else if 0 as i32 == 'I' as i32 {
            8 as i32
        } else if 0 as i32 == 'J' as i32 {
            9 as i32
        } else if 0 as i32 == 'K' as i32 {
            10 as i32
        } else if 0 as i32 == 'L' as i32 {
            11 as i32
        } else if 0 as i32 == 'M' as i32 {
            12 as i32
        } else if 0 as i32 == 'N' as i32 {
            13 as i32
        } else if 0 as i32 == 'O' as i32 {
            14 as i32
        } else if 0 as i32 == 'P' as i32 {
            15 as i32
        } else if 0 as i32 == 'Q' as i32 {
            16 as i32
        } else if 0 as i32 == 'R' as i32 {
            17 as i32
        } else if 0 as i32 == 'S' as i32 {
            18 as i32
        } else if 0 as i32 == 'T' as i32 {
            19 as i32
        } else if 0 as i32 == 'U' as i32 {
            20 as i32
        } else if 0 as i32 == 'V' as i32 {
            21 as i32
        } else if 0 as i32 == 'W' as i32 {
            22 as i32
        } else if 0 as i32 == 'X' as i32 {
            23 as i32
        } else if 0 as i32 == 'Y' as i32 {
            24 as i32
        } else if 0 as i32 == 'Z' as i32 {
            25 as i32
        } else if 0 as i32 == '2' as i32 {
            26 as i32
        } else if 0 as i32 == '3' as i32 {
            27 as i32
        } else if 0 as i32 == '4' as i32 {
            28 as i32
        } else if 0 as i32 == '5' as i32 {
            29 as i32
        } else if 0 as i32 == '6' as i32 {
            30 as i32
        } else if 0 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 1 as i32 == 'A' as i32 {
            0 as i32
        } else if 1 as i32 == 'B' as i32 {
            1 as i32
        } else if 1 as i32 == 'C' as i32 {
            2 as i32
        } else if 1 as i32 == 'D' as i32 {
            3 as i32
        } else if 1 as i32 == 'E' as i32 {
            4 as i32
        } else if 1 as i32 == 'F' as i32 {
            5 as i32
        } else if 1 as i32 == 'G' as i32 {
            6 as i32
        } else if 1 as i32 == 'H' as i32 {
            7 as i32
        } else if 1 as i32 == 'I' as i32 {
            8 as i32
        } else if 1 as i32 == 'J' as i32 {
            9 as i32
        } else if 1 as i32 == 'K' as i32 {
            10 as i32
        } else if 1 as i32 == 'L' as i32 {
            11 as i32
        } else if 1 as i32 == 'M' as i32 {
            12 as i32
        } else if 1 as i32 == 'N' as i32 {
            13 as i32
        } else if 1 as i32 == 'O' as i32 {
            14 as i32
        } else if 1 as i32 == 'P' as i32 {
            15 as i32
        } else if 1 as i32 == 'Q' as i32 {
            16 as i32
        } else if 1 as i32 == 'R' as i32 {
            17 as i32
        } else if 1 as i32 == 'S' as i32 {
            18 as i32
        } else if 1 as i32 == 'T' as i32 {
            19 as i32
        } else if 1 as i32 == 'U' as i32 {
            20 as i32
        } else if 1 as i32 == 'V' as i32 {
            21 as i32
        } else if 1 as i32 == 'W' as i32 {
            22 as i32
        } else if 1 as i32 == 'X' as i32 {
            23 as i32
        } else if 1 as i32 == 'Y' as i32 {
            24 as i32
        } else if 1 as i32 == 'Z' as i32 {
            25 as i32
        } else if 1 as i32 == '2' as i32 {
            26 as i32
        } else if 1 as i32 == '3' as i32 {
            27 as i32
        } else if 1 as i32 == '4' as i32 {
            28 as i32
        } else if 1 as i32 == '5' as i32 {
            29 as i32
        } else if 1 as i32 == '6' as i32 {
            30 as i32
        } else if 1 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 2 as i32 == 'A' as i32 {
            0 as i32
        } else if 2 as i32 == 'B' as i32 {
            1 as i32
        } else if 2 as i32 == 'C' as i32 {
            2 as i32
        } else if 2 as i32 == 'D' as i32 {
            3 as i32
        } else if 2 as i32 == 'E' as i32 {
            4 as i32
        } else if 2 as i32 == 'F' as i32 {
            5 as i32
        } else if 2 as i32 == 'G' as i32 {
            6 as i32
        } else if 2 as i32 == 'H' as i32 {
            7 as i32
        } else if 2 as i32 == 'I' as i32 {
            8 as i32
        } else if 2 as i32 == 'J' as i32 {
            9 as i32
        } else if 2 as i32 == 'K' as i32 {
            10 as i32
        } else if 2 as i32 == 'L' as i32 {
            11 as i32
        } else if 2 as i32 == 'M' as i32 {
            12 as i32
        } else if 2 as i32 == 'N' as i32 {
            13 as i32
        } else if 2 as i32 == 'O' as i32 {
            14 as i32
        } else if 2 as i32 == 'P' as i32 {
            15 as i32
        } else if 2 as i32 == 'Q' as i32 {
            16 as i32
        } else if 2 as i32 == 'R' as i32 {
            17 as i32
        } else if 2 as i32 == 'S' as i32 {
            18 as i32
        } else if 2 as i32 == 'T' as i32 {
            19 as i32
        } else if 2 as i32 == 'U' as i32 {
            20 as i32
        } else if 2 as i32 == 'V' as i32 {
            21 as i32
        } else if 2 as i32 == 'W' as i32 {
            22 as i32
        } else if 2 as i32 == 'X' as i32 {
            23 as i32
        } else if 2 as i32 == 'Y' as i32 {
            24 as i32
        } else if 2 as i32 == 'Z' as i32 {
            25 as i32
        } else if 2 as i32 == '2' as i32 {
            26 as i32
        } else if 2 as i32 == '3' as i32 {
            27 as i32
        } else if 2 as i32 == '4' as i32 {
            28 as i32
        } else if 2 as i32 == '5' as i32 {
            29 as i32
        } else if 2 as i32 == '6' as i32 {
            30 as i32
        } else if 2 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 3 as i32 == 'A' as i32 {
            0 as i32
        } else if 3 as i32 == 'B' as i32 {
            1 as i32
        } else if 3 as i32 == 'C' as i32 {
            2 as i32
        } else if 3 as i32 == 'D' as i32 {
            3 as i32
        } else if 3 as i32 == 'E' as i32 {
            4 as i32
        } else if 3 as i32 == 'F' as i32 {
            5 as i32
        } else if 3 as i32 == 'G' as i32 {
            6 as i32
        } else if 3 as i32 == 'H' as i32 {
            7 as i32
        } else if 3 as i32 == 'I' as i32 {
            8 as i32
        } else if 3 as i32 == 'J' as i32 {
            9 as i32
        } else if 3 as i32 == 'K' as i32 {
            10 as i32
        } else if 3 as i32 == 'L' as i32 {
            11 as i32
        } else if 3 as i32 == 'M' as i32 {
            12 as i32
        } else if 3 as i32 == 'N' as i32 {
            13 as i32
        } else if 3 as i32 == 'O' as i32 {
            14 as i32
        } else if 3 as i32 == 'P' as i32 {
            15 as i32
        } else if 3 as i32 == 'Q' as i32 {
            16 as i32
        } else if 3 as i32 == 'R' as i32 {
            17 as i32
        } else if 3 as i32 == 'S' as i32 {
            18 as i32
        } else if 3 as i32 == 'T' as i32 {
            19 as i32
        } else if 3 as i32 == 'U' as i32 {
            20 as i32
        } else if 3 as i32 == 'V' as i32 {
            21 as i32
        } else if 3 as i32 == 'W' as i32 {
            22 as i32
        } else if 3 as i32 == 'X' as i32 {
            23 as i32
        } else if 3 as i32 == 'Y' as i32 {
            24 as i32
        } else if 3 as i32 == 'Z' as i32 {
            25 as i32
        } else if 3 as i32 == '2' as i32 {
            26 as i32
        } else if 3 as i32 == '3' as i32 {
            27 as i32
        } else if 3 as i32 == '4' as i32 {
            28 as i32
        } else if 3 as i32 == '5' as i32 {
            29 as i32
        } else if 3 as i32 == '6' as i32 {
            30 as i32
        } else if 3 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 4 as i32 == 'A' as i32 {
            0 as i32
        } else if 4 as i32 == 'B' as i32 {
            1 as i32
        } else if 4 as i32 == 'C' as i32 {
            2 as i32
        } else if 4 as i32 == 'D' as i32 {
            3 as i32
        } else if 4 as i32 == 'E' as i32 {
            4 as i32
        } else if 4 as i32 == 'F' as i32 {
            5 as i32
        } else if 4 as i32 == 'G' as i32 {
            6 as i32
        } else if 4 as i32 == 'H' as i32 {
            7 as i32
        } else if 4 as i32 == 'I' as i32 {
            8 as i32
        } else if 4 as i32 == 'J' as i32 {
            9 as i32
        } else if 4 as i32 == 'K' as i32 {
            10 as i32
        } else if 4 as i32 == 'L' as i32 {
            11 as i32
        } else if 4 as i32 == 'M' as i32 {
            12 as i32
        } else if 4 as i32 == 'N' as i32 {
            13 as i32
        } else if 4 as i32 == 'O' as i32 {
            14 as i32
        } else if 4 as i32 == 'P' as i32 {
            15 as i32
        } else if 4 as i32 == 'Q' as i32 {
            16 as i32
        } else if 4 as i32 == 'R' as i32 {
            17 as i32
        } else if 4 as i32 == 'S' as i32 {
            18 as i32
        } else if 4 as i32 == 'T' as i32 {
            19 as i32
        } else if 4 as i32 == 'U' as i32 {
            20 as i32
        } else if 4 as i32 == 'V' as i32 {
            21 as i32
        } else if 4 as i32 == 'W' as i32 {
            22 as i32
        } else if 4 as i32 == 'X' as i32 {
            23 as i32
        } else if 4 as i32 == 'Y' as i32 {
            24 as i32
        } else if 4 as i32 == 'Z' as i32 {
            25 as i32
        } else if 4 as i32 == '2' as i32 {
            26 as i32
        } else if 4 as i32 == '3' as i32 {
            27 as i32
        } else if 4 as i32 == '4' as i32 {
            28 as i32
        } else if 4 as i32 == '5' as i32 {
            29 as i32
        } else if 4 as i32 == '6' as i32 {
            30 as i32
        } else if 4 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 5 as i32 == 'A' as i32 {
            0 as i32
        } else if 5 as i32 == 'B' as i32 {
            1 as i32
        } else if 5 as i32 == 'C' as i32 {
            2 as i32
        } else if 5 as i32 == 'D' as i32 {
            3 as i32
        } else if 5 as i32 == 'E' as i32 {
            4 as i32
        } else if 5 as i32 == 'F' as i32 {
            5 as i32
        } else if 5 as i32 == 'G' as i32 {
            6 as i32
        } else if 5 as i32 == 'H' as i32 {
            7 as i32
        } else if 5 as i32 == 'I' as i32 {
            8 as i32
        } else if 5 as i32 == 'J' as i32 {
            9 as i32
        } else if 5 as i32 == 'K' as i32 {
            10 as i32
        } else if 5 as i32 == 'L' as i32 {
            11 as i32
        } else if 5 as i32 == 'M' as i32 {
            12 as i32
        } else if 5 as i32 == 'N' as i32 {
            13 as i32
        } else if 5 as i32 == 'O' as i32 {
            14 as i32
        } else if 5 as i32 == 'P' as i32 {
            15 as i32
        } else if 5 as i32 == 'Q' as i32 {
            16 as i32
        } else if 5 as i32 == 'R' as i32 {
            17 as i32
        } else if 5 as i32 == 'S' as i32 {
            18 as i32
        } else if 5 as i32 == 'T' as i32 {
            19 as i32
        } else if 5 as i32 == 'U' as i32 {
            20 as i32
        } else if 5 as i32 == 'V' as i32 {
            21 as i32
        } else if 5 as i32 == 'W' as i32 {
            22 as i32
        } else if 5 as i32 == 'X' as i32 {
            23 as i32
        } else if 5 as i32 == 'Y' as i32 {
            24 as i32
        } else if 5 as i32 == 'Z' as i32 {
            25 as i32
        } else if 5 as i32 == '2' as i32 {
            26 as i32
        } else if 5 as i32 == '3' as i32 {
            27 as i32
        } else if 5 as i32 == '4' as i32 {
            28 as i32
        } else if 5 as i32 == '5' as i32 {
            29 as i32
        } else if 5 as i32 == '6' as i32 {
            30 as i32
        } else if 5 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 6 as i32 == 'A' as i32 {
            0 as i32
        } else if 6 as i32 == 'B' as i32 {
            1 as i32
        } else if 6 as i32 == 'C' as i32 {
            2 as i32
        } else if 6 as i32 == 'D' as i32 {
            3 as i32
        } else if 6 as i32 == 'E' as i32 {
            4 as i32
        } else if 6 as i32 == 'F' as i32 {
            5 as i32
        } else if 6 as i32 == 'G' as i32 {
            6 as i32
        } else if 6 as i32 == 'H' as i32 {
            7 as i32
        } else if 6 as i32 == 'I' as i32 {
            8 as i32
        } else if 6 as i32 == 'J' as i32 {
            9 as i32
        } else if 6 as i32 == 'K' as i32 {
            10 as i32
        } else if 6 as i32 == 'L' as i32 {
            11 as i32
        } else if 6 as i32 == 'M' as i32 {
            12 as i32
        } else if 6 as i32 == 'N' as i32 {
            13 as i32
        } else if 6 as i32 == 'O' as i32 {
            14 as i32
        } else if 6 as i32 == 'P' as i32 {
            15 as i32
        } else if 6 as i32 == 'Q' as i32 {
            16 as i32
        } else if 6 as i32 == 'R' as i32 {
            17 as i32
        } else if 6 as i32 == 'S' as i32 {
            18 as i32
        } else if 6 as i32 == 'T' as i32 {
            19 as i32
        } else if 6 as i32 == 'U' as i32 {
            20 as i32
        } else if 6 as i32 == 'V' as i32 {
            21 as i32
        } else if 6 as i32 == 'W' as i32 {
            22 as i32
        } else if 6 as i32 == 'X' as i32 {
            23 as i32
        } else if 6 as i32 == 'Y' as i32 {
            24 as i32
        } else if 6 as i32 == 'Z' as i32 {
            25 as i32
        } else if 6 as i32 == '2' as i32 {
            26 as i32
        } else if 6 as i32 == '3' as i32 {
            27 as i32
        } else if 6 as i32 == '4' as i32 {
            28 as i32
        } else if 6 as i32 == '5' as i32 {
            29 as i32
        } else if 6 as i32 == '6' as i32 {
            30 as i32
        } else if 6 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 7 as i32 == 'A' as i32 {
            0 as i32
        } else if 7 as i32 == 'B' as i32 {
            1 as i32
        } else if 7 as i32 == 'C' as i32 {
            2 as i32
        } else if 7 as i32 == 'D' as i32 {
            3 as i32
        } else if 7 as i32 == 'E' as i32 {
            4 as i32
        } else if 7 as i32 == 'F' as i32 {
            5 as i32
        } else if 7 as i32 == 'G' as i32 {
            6 as i32
        } else if 7 as i32 == 'H' as i32 {
            7 as i32
        } else if 7 as i32 == 'I' as i32 {
            8 as i32
        } else if 7 as i32 == 'J' as i32 {
            9 as i32
        } else if 7 as i32 == 'K' as i32 {
            10 as i32
        } else if 7 as i32 == 'L' as i32 {
            11 as i32
        } else if 7 as i32 == 'M' as i32 {
            12 as i32
        } else if 7 as i32 == 'N' as i32 {
            13 as i32
        } else if 7 as i32 == 'O' as i32 {
            14 as i32
        } else if 7 as i32 == 'P' as i32 {
            15 as i32
        } else if 7 as i32 == 'Q' as i32 {
            16 as i32
        } else if 7 as i32 == 'R' as i32 {
            17 as i32
        } else if 7 as i32 == 'S' as i32 {
            18 as i32
        } else if 7 as i32 == 'T' as i32 {
            19 as i32
        } else if 7 as i32 == 'U' as i32 {
            20 as i32
        } else if 7 as i32 == 'V' as i32 {
            21 as i32
        } else if 7 as i32 == 'W' as i32 {
            22 as i32
        } else if 7 as i32 == 'X' as i32 {
            23 as i32
        } else if 7 as i32 == 'Y' as i32 {
            24 as i32
        } else if 7 as i32 == 'Z' as i32 {
            25 as i32
        } else if 7 as i32 == '2' as i32 {
            26 as i32
        } else if 7 as i32 == '3' as i32 {
            27 as i32
        } else if 7 as i32 == '4' as i32 {
            28 as i32
        } else if 7 as i32 == '5' as i32 {
            29 as i32
        } else if 7 as i32 == '6' as i32 {
            30 as i32
        } else if 7 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 8 as i32 == 'A' as i32 {
            0 as i32
        } else if 8 as i32 == 'B' as i32 {
            1 as i32
        } else if 8 as i32 == 'C' as i32 {
            2 as i32
        } else if 8 as i32 == 'D' as i32 {
            3 as i32
        } else if 8 as i32 == 'E' as i32 {
            4 as i32
        } else if 8 as i32 == 'F' as i32 {
            5 as i32
        } else if 8 as i32 == 'G' as i32 {
            6 as i32
        } else if 8 as i32 == 'H' as i32 {
            7 as i32
        } else if 8 as i32 == 'I' as i32 {
            8 as i32
        } else if 8 as i32 == 'J' as i32 {
            9 as i32
        } else if 8 as i32 == 'K' as i32 {
            10 as i32
        } else if 8 as i32 == 'L' as i32 {
            11 as i32
        } else if 8 as i32 == 'M' as i32 {
            12 as i32
        } else if 8 as i32 == 'N' as i32 {
            13 as i32
        } else if 8 as i32 == 'O' as i32 {
            14 as i32
        } else if 8 as i32 == 'P' as i32 {
            15 as i32
        } else if 8 as i32 == 'Q' as i32 {
            16 as i32
        } else if 8 as i32 == 'R' as i32 {
            17 as i32
        } else if 8 as i32 == 'S' as i32 {
            18 as i32
        } else if 8 as i32 == 'T' as i32 {
            19 as i32
        } else if 8 as i32 == 'U' as i32 {
            20 as i32
        } else if 8 as i32 == 'V' as i32 {
            21 as i32
        } else if 8 as i32 == 'W' as i32 {
            22 as i32
        } else if 8 as i32 == 'X' as i32 {
            23 as i32
        } else if 8 as i32 == 'Y' as i32 {
            24 as i32
        } else if 8 as i32 == 'Z' as i32 {
            25 as i32
        } else if 8 as i32 == '2' as i32 {
            26 as i32
        } else if 8 as i32 == '3' as i32 {
            27 as i32
        } else if 8 as i32 == '4' as i32 {
            28 as i32
        } else if 8 as i32 == '5' as i32 {
            29 as i32
        } else if 8 as i32 == '6' as i32 {
            30 as i32
        } else if 8 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 9 as i32 == 'A' as i32 {
            0 as i32
        } else if 9 as i32 == 'B' as i32 {
            1 as i32
        } else if 9 as i32 == 'C' as i32 {
            2 as i32
        } else if 9 as i32 == 'D' as i32 {
            3 as i32
        } else if 9 as i32 == 'E' as i32 {
            4 as i32
        } else if 9 as i32 == 'F' as i32 {
            5 as i32
        } else if 9 as i32 == 'G' as i32 {
            6 as i32
        } else if 9 as i32 == 'H' as i32 {
            7 as i32
        } else if 9 as i32 == 'I' as i32 {
            8 as i32
        } else if 9 as i32 == 'J' as i32 {
            9 as i32
        } else if 9 as i32 == 'K' as i32 {
            10 as i32
        } else if 9 as i32 == 'L' as i32 {
            11 as i32
        } else if 9 as i32 == 'M' as i32 {
            12 as i32
        } else if 9 as i32 == 'N' as i32 {
            13 as i32
        } else if 9 as i32 == 'O' as i32 {
            14 as i32
        } else if 9 as i32 == 'P' as i32 {
            15 as i32
        } else if 9 as i32 == 'Q' as i32 {
            16 as i32
        } else if 9 as i32 == 'R' as i32 {
            17 as i32
        } else if 9 as i32 == 'S' as i32 {
            18 as i32
        } else if 9 as i32 == 'T' as i32 {
            19 as i32
        } else if 9 as i32 == 'U' as i32 {
            20 as i32
        } else if 9 as i32 == 'V' as i32 {
            21 as i32
        } else if 9 as i32 == 'W' as i32 {
            22 as i32
        } else if 9 as i32 == 'X' as i32 {
            23 as i32
        } else if 9 as i32 == 'Y' as i32 {
            24 as i32
        } else if 9 as i32 == 'Z' as i32 {
            25 as i32
        } else if 9 as i32 == '2' as i32 {
            26 as i32
        } else if 9 as i32 == '3' as i32 {
            27 as i32
        } else if 9 as i32 == '4' as i32 {
            28 as i32
        } else if 9 as i32 == '5' as i32 {
            29 as i32
        } else if 9 as i32 == '6' as i32 {
            30 as i32
        } else if 9 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 10 as i32 == 'A' as i32 {
            0 as i32
        } else if 10 as i32 == 'B' as i32 {
            1 as i32
        } else if 10 as i32 == 'C' as i32 {
            2 as i32
        } else if 10 as i32 == 'D' as i32 {
            3 as i32
        } else if 10 as i32 == 'E' as i32 {
            4 as i32
        } else if 10 as i32 == 'F' as i32 {
            5 as i32
        } else if 10 as i32 == 'G' as i32 {
            6 as i32
        } else if 10 as i32 == 'H' as i32 {
            7 as i32
        } else if 10 as i32 == 'I' as i32 {
            8 as i32
        } else if 10 as i32 == 'J' as i32 {
            9 as i32
        } else if 10 as i32 == 'K' as i32 {
            10 as i32
        } else if 10 as i32 == 'L' as i32 {
            11 as i32
        } else if 10 as i32 == 'M' as i32 {
            12 as i32
        } else if 10 as i32 == 'N' as i32 {
            13 as i32
        } else if 10 as i32 == 'O' as i32 {
            14 as i32
        } else if 10 as i32 == 'P' as i32 {
            15 as i32
        } else if 10 as i32 == 'Q' as i32 {
            16 as i32
        } else if 10 as i32 == 'R' as i32 {
            17 as i32
        } else if 10 as i32 == 'S' as i32 {
            18 as i32
        } else if 10 as i32 == 'T' as i32 {
            19 as i32
        } else if 10 as i32 == 'U' as i32 {
            20 as i32
        } else if 10 as i32 == 'V' as i32 {
            21 as i32
        } else if 10 as i32 == 'W' as i32 {
            22 as i32
        } else if 10 as i32 == 'X' as i32 {
            23 as i32
        } else if 10 as i32 == 'Y' as i32 {
            24 as i32
        } else if 10 as i32 == 'Z' as i32 {
            25 as i32
        } else if 10 as i32 == '2' as i32 {
            26 as i32
        } else if 10 as i32 == '3' as i32 {
            27 as i32
        } else if 10 as i32 == '4' as i32 {
            28 as i32
        } else if 10 as i32 == '5' as i32 {
            29 as i32
        } else if 10 as i32 == '6' as i32 {
            30 as i32
        } else if 10 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 11 as i32 == 'A' as i32 {
            0 as i32
        } else if 11 as i32 == 'B' as i32 {
            1 as i32
        } else if 11 as i32 == 'C' as i32 {
            2 as i32
        } else if 11 as i32 == 'D' as i32 {
            3 as i32
        } else if 11 as i32 == 'E' as i32 {
            4 as i32
        } else if 11 as i32 == 'F' as i32 {
            5 as i32
        } else if 11 as i32 == 'G' as i32 {
            6 as i32
        } else if 11 as i32 == 'H' as i32 {
            7 as i32
        } else if 11 as i32 == 'I' as i32 {
            8 as i32
        } else if 11 as i32 == 'J' as i32 {
            9 as i32
        } else if 11 as i32 == 'K' as i32 {
            10 as i32
        } else if 11 as i32 == 'L' as i32 {
            11 as i32
        } else if 11 as i32 == 'M' as i32 {
            12 as i32
        } else if 11 as i32 == 'N' as i32 {
            13 as i32
        } else if 11 as i32 == 'O' as i32 {
            14 as i32
        } else if 11 as i32 == 'P' as i32 {
            15 as i32
        } else if 11 as i32 == 'Q' as i32 {
            16 as i32
        } else if 11 as i32 == 'R' as i32 {
            17 as i32
        } else if 11 as i32 == 'S' as i32 {
            18 as i32
        } else if 11 as i32 == 'T' as i32 {
            19 as i32
        } else if 11 as i32 == 'U' as i32 {
            20 as i32
        } else if 11 as i32 == 'V' as i32 {
            21 as i32
        } else if 11 as i32 == 'W' as i32 {
            22 as i32
        } else if 11 as i32 == 'X' as i32 {
            23 as i32
        } else if 11 as i32 == 'Y' as i32 {
            24 as i32
        } else if 11 as i32 == 'Z' as i32 {
            25 as i32
        } else if 11 as i32 == '2' as i32 {
            26 as i32
        } else if 11 as i32 == '3' as i32 {
            27 as i32
        } else if 11 as i32 == '4' as i32 {
            28 as i32
        } else if 11 as i32 == '5' as i32 {
            29 as i32
        } else if 11 as i32 == '6' as i32 {
            30 as i32
        } else if 11 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 12 as i32 == 'A' as i32 {
            0 as i32
        } else if 12 as i32 == 'B' as i32 {
            1 as i32
        } else if 12 as i32 == 'C' as i32 {
            2 as i32
        } else if 12 as i32 == 'D' as i32 {
            3 as i32
        } else if 12 as i32 == 'E' as i32 {
            4 as i32
        } else if 12 as i32 == 'F' as i32 {
            5 as i32
        } else if 12 as i32 == 'G' as i32 {
            6 as i32
        } else if 12 as i32 == 'H' as i32 {
            7 as i32
        } else if 12 as i32 == 'I' as i32 {
            8 as i32
        } else if 12 as i32 == 'J' as i32 {
            9 as i32
        } else if 12 as i32 == 'K' as i32 {
            10 as i32
        } else if 12 as i32 == 'L' as i32 {
            11 as i32
        } else if 12 as i32 == 'M' as i32 {
            12 as i32
        } else if 12 as i32 == 'N' as i32 {
            13 as i32
        } else if 12 as i32 == 'O' as i32 {
            14 as i32
        } else if 12 as i32 == 'P' as i32 {
            15 as i32
        } else if 12 as i32 == 'Q' as i32 {
            16 as i32
        } else if 12 as i32 == 'R' as i32 {
            17 as i32
        } else if 12 as i32 == 'S' as i32 {
            18 as i32
        } else if 12 as i32 == 'T' as i32 {
            19 as i32
        } else if 12 as i32 == 'U' as i32 {
            20 as i32
        } else if 12 as i32 == 'V' as i32 {
            21 as i32
        } else if 12 as i32 == 'W' as i32 {
            22 as i32
        } else if 12 as i32 == 'X' as i32 {
            23 as i32
        } else if 12 as i32 == 'Y' as i32 {
            24 as i32
        } else if 12 as i32 == 'Z' as i32 {
            25 as i32
        } else if 12 as i32 == '2' as i32 {
            26 as i32
        } else if 12 as i32 == '3' as i32 {
            27 as i32
        } else if 12 as i32 == '4' as i32 {
            28 as i32
        } else if 12 as i32 == '5' as i32 {
            29 as i32
        } else if 12 as i32 == '6' as i32 {
            30 as i32
        } else if 12 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 13 as i32 == 'A' as i32 {
            0 as i32
        } else if 13 as i32 == 'B' as i32 {
            1 as i32
        } else if 13 as i32 == 'C' as i32 {
            2 as i32
        } else if 13 as i32 == 'D' as i32 {
            3 as i32
        } else if 13 as i32 == 'E' as i32 {
            4 as i32
        } else if 13 as i32 == 'F' as i32 {
            5 as i32
        } else if 13 as i32 == 'G' as i32 {
            6 as i32
        } else if 13 as i32 == 'H' as i32 {
            7 as i32
        } else if 13 as i32 == 'I' as i32 {
            8 as i32
        } else if 13 as i32 == 'J' as i32 {
            9 as i32
        } else if 13 as i32 == 'K' as i32 {
            10 as i32
        } else if 13 as i32 == 'L' as i32 {
            11 as i32
        } else if 13 as i32 == 'M' as i32 {
            12 as i32
        } else if 13 as i32 == 'N' as i32 {
            13 as i32
        } else if 13 as i32 == 'O' as i32 {
            14 as i32
        } else if 13 as i32 == 'P' as i32 {
            15 as i32
        } else if 13 as i32 == 'Q' as i32 {
            16 as i32
        } else if 13 as i32 == 'R' as i32 {
            17 as i32
        } else if 13 as i32 == 'S' as i32 {
            18 as i32
        } else if 13 as i32 == 'T' as i32 {
            19 as i32
        } else if 13 as i32 == 'U' as i32 {
            20 as i32
        } else if 13 as i32 == 'V' as i32 {
            21 as i32
        } else if 13 as i32 == 'W' as i32 {
            22 as i32
        } else if 13 as i32 == 'X' as i32 {
            23 as i32
        } else if 13 as i32 == 'Y' as i32 {
            24 as i32
        } else if 13 as i32 == 'Z' as i32 {
            25 as i32
        } else if 13 as i32 == '2' as i32 {
            26 as i32
        } else if 13 as i32 == '3' as i32 {
            27 as i32
        } else if 13 as i32 == '4' as i32 {
            28 as i32
        } else if 13 as i32 == '5' as i32 {
            29 as i32
        } else if 13 as i32 == '6' as i32 {
            30 as i32
        } else if 13 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 14 as i32 == 'A' as i32 {
            0 as i32
        } else if 14 as i32 == 'B' as i32 {
            1 as i32
        } else if 14 as i32 == 'C' as i32 {
            2 as i32
        } else if 14 as i32 == 'D' as i32 {
            3 as i32
        } else if 14 as i32 == 'E' as i32 {
            4 as i32
        } else if 14 as i32 == 'F' as i32 {
            5 as i32
        } else if 14 as i32 == 'G' as i32 {
            6 as i32
        } else if 14 as i32 == 'H' as i32 {
            7 as i32
        } else if 14 as i32 == 'I' as i32 {
            8 as i32
        } else if 14 as i32 == 'J' as i32 {
            9 as i32
        } else if 14 as i32 == 'K' as i32 {
            10 as i32
        } else if 14 as i32 == 'L' as i32 {
            11 as i32
        } else if 14 as i32 == 'M' as i32 {
            12 as i32
        } else if 14 as i32 == 'N' as i32 {
            13 as i32
        } else if 14 as i32 == 'O' as i32 {
            14 as i32
        } else if 14 as i32 == 'P' as i32 {
            15 as i32
        } else if 14 as i32 == 'Q' as i32 {
            16 as i32
        } else if 14 as i32 == 'R' as i32 {
            17 as i32
        } else if 14 as i32 == 'S' as i32 {
            18 as i32
        } else if 14 as i32 == 'T' as i32 {
            19 as i32
        } else if 14 as i32 == 'U' as i32 {
            20 as i32
        } else if 14 as i32 == 'V' as i32 {
            21 as i32
        } else if 14 as i32 == 'W' as i32 {
            22 as i32
        } else if 14 as i32 == 'X' as i32 {
            23 as i32
        } else if 14 as i32 == 'Y' as i32 {
            24 as i32
        } else if 14 as i32 == 'Z' as i32 {
            25 as i32
        } else if 14 as i32 == '2' as i32 {
            26 as i32
        } else if 14 as i32 == '3' as i32 {
            27 as i32
        } else if 14 as i32 == '4' as i32 {
            28 as i32
        } else if 14 as i32 == '5' as i32 {
            29 as i32
        } else if 14 as i32 == '6' as i32 {
            30 as i32
        } else if 14 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 15 as i32 == 'A' as i32 {
            0 as i32
        } else if 15 as i32 == 'B' as i32 {
            1 as i32
        } else if 15 as i32 == 'C' as i32 {
            2 as i32
        } else if 15 as i32 == 'D' as i32 {
            3 as i32
        } else if 15 as i32 == 'E' as i32 {
            4 as i32
        } else if 15 as i32 == 'F' as i32 {
            5 as i32
        } else if 15 as i32 == 'G' as i32 {
            6 as i32
        } else if 15 as i32 == 'H' as i32 {
            7 as i32
        } else if 15 as i32 == 'I' as i32 {
            8 as i32
        } else if 15 as i32 == 'J' as i32 {
            9 as i32
        } else if 15 as i32 == 'K' as i32 {
            10 as i32
        } else if 15 as i32 == 'L' as i32 {
            11 as i32
        } else if 15 as i32 == 'M' as i32 {
            12 as i32
        } else if 15 as i32 == 'N' as i32 {
            13 as i32
        } else if 15 as i32 == 'O' as i32 {
            14 as i32
        } else if 15 as i32 == 'P' as i32 {
            15 as i32
        } else if 15 as i32 == 'Q' as i32 {
            16 as i32
        } else if 15 as i32 == 'R' as i32 {
            17 as i32
        } else if 15 as i32 == 'S' as i32 {
            18 as i32
        } else if 15 as i32 == 'T' as i32 {
            19 as i32
        } else if 15 as i32 == 'U' as i32 {
            20 as i32
        } else if 15 as i32 == 'V' as i32 {
            21 as i32
        } else if 15 as i32 == 'W' as i32 {
            22 as i32
        } else if 15 as i32 == 'X' as i32 {
            23 as i32
        } else if 15 as i32 == 'Y' as i32 {
            24 as i32
        } else if 15 as i32 == 'Z' as i32 {
            25 as i32
        } else if 15 as i32 == '2' as i32 {
            26 as i32
        } else if 15 as i32 == '3' as i32 {
            27 as i32
        } else if 15 as i32 == '4' as i32 {
            28 as i32
        } else if 15 as i32 == '5' as i32 {
            29 as i32
        } else if 15 as i32 == '6' as i32 {
            30 as i32
        } else if 15 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 16 as i32 == 'A' as i32 {
            0 as i32
        } else if 16 as i32 == 'B' as i32 {
            1 as i32
        } else if 16 as i32 == 'C' as i32 {
            2 as i32
        } else if 16 as i32 == 'D' as i32 {
            3 as i32
        } else if 16 as i32 == 'E' as i32 {
            4 as i32
        } else if 16 as i32 == 'F' as i32 {
            5 as i32
        } else if 16 as i32 == 'G' as i32 {
            6 as i32
        } else if 16 as i32 == 'H' as i32 {
            7 as i32
        } else if 16 as i32 == 'I' as i32 {
            8 as i32
        } else if 16 as i32 == 'J' as i32 {
            9 as i32
        } else if 16 as i32 == 'K' as i32 {
            10 as i32
        } else if 16 as i32 == 'L' as i32 {
            11 as i32
        } else if 16 as i32 == 'M' as i32 {
            12 as i32
        } else if 16 as i32 == 'N' as i32 {
            13 as i32
        } else if 16 as i32 == 'O' as i32 {
            14 as i32
        } else if 16 as i32 == 'P' as i32 {
            15 as i32
        } else if 16 as i32 == 'Q' as i32 {
            16 as i32
        } else if 16 as i32 == 'R' as i32 {
            17 as i32
        } else if 16 as i32 == 'S' as i32 {
            18 as i32
        } else if 16 as i32 == 'T' as i32 {
            19 as i32
        } else if 16 as i32 == 'U' as i32 {
            20 as i32
        } else if 16 as i32 == 'V' as i32 {
            21 as i32
        } else if 16 as i32 == 'W' as i32 {
            22 as i32
        } else if 16 as i32 == 'X' as i32 {
            23 as i32
        } else if 16 as i32 == 'Y' as i32 {
            24 as i32
        } else if 16 as i32 == 'Z' as i32 {
            25 as i32
        } else if 16 as i32 == '2' as i32 {
            26 as i32
        } else if 16 as i32 == '3' as i32 {
            27 as i32
        } else if 16 as i32 == '4' as i32 {
            28 as i32
        } else if 16 as i32 == '5' as i32 {
            29 as i32
        } else if 16 as i32 == '6' as i32 {
            30 as i32
        } else if 16 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 17 as i32 == 'A' as i32 {
            0 as i32
        } else if 17 as i32 == 'B' as i32 {
            1 as i32
        } else if 17 as i32 == 'C' as i32 {
            2 as i32
        } else if 17 as i32 == 'D' as i32 {
            3 as i32
        } else if 17 as i32 == 'E' as i32 {
            4 as i32
        } else if 17 as i32 == 'F' as i32 {
            5 as i32
        } else if 17 as i32 == 'G' as i32 {
            6 as i32
        } else if 17 as i32 == 'H' as i32 {
            7 as i32
        } else if 17 as i32 == 'I' as i32 {
            8 as i32
        } else if 17 as i32 == 'J' as i32 {
            9 as i32
        } else if 17 as i32 == 'K' as i32 {
            10 as i32
        } else if 17 as i32 == 'L' as i32 {
            11 as i32
        } else if 17 as i32 == 'M' as i32 {
            12 as i32
        } else if 17 as i32 == 'N' as i32 {
            13 as i32
        } else if 17 as i32 == 'O' as i32 {
            14 as i32
        } else if 17 as i32 == 'P' as i32 {
            15 as i32
        } else if 17 as i32 == 'Q' as i32 {
            16 as i32
        } else if 17 as i32 == 'R' as i32 {
            17 as i32
        } else if 17 as i32 == 'S' as i32 {
            18 as i32
        } else if 17 as i32 == 'T' as i32 {
            19 as i32
        } else if 17 as i32 == 'U' as i32 {
            20 as i32
        } else if 17 as i32 == 'V' as i32 {
            21 as i32
        } else if 17 as i32 == 'W' as i32 {
            22 as i32
        } else if 17 as i32 == 'X' as i32 {
            23 as i32
        } else if 17 as i32 == 'Y' as i32 {
            24 as i32
        } else if 17 as i32 == 'Z' as i32 {
            25 as i32
        } else if 17 as i32 == '2' as i32 {
            26 as i32
        } else if 17 as i32 == '3' as i32 {
            27 as i32
        } else if 17 as i32 == '4' as i32 {
            28 as i32
        } else if 17 as i32 == '5' as i32 {
            29 as i32
        } else if 17 as i32 == '6' as i32 {
            30 as i32
        } else if 17 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 18 as i32 == 'A' as i32 {
            0 as i32
        } else if 18 as i32 == 'B' as i32 {
            1 as i32
        } else if 18 as i32 == 'C' as i32 {
            2 as i32
        } else if 18 as i32 == 'D' as i32 {
            3 as i32
        } else if 18 as i32 == 'E' as i32 {
            4 as i32
        } else if 18 as i32 == 'F' as i32 {
            5 as i32
        } else if 18 as i32 == 'G' as i32 {
            6 as i32
        } else if 18 as i32 == 'H' as i32 {
            7 as i32
        } else if 18 as i32 == 'I' as i32 {
            8 as i32
        } else if 18 as i32 == 'J' as i32 {
            9 as i32
        } else if 18 as i32 == 'K' as i32 {
            10 as i32
        } else if 18 as i32 == 'L' as i32 {
            11 as i32
        } else if 18 as i32 == 'M' as i32 {
            12 as i32
        } else if 18 as i32 == 'N' as i32 {
            13 as i32
        } else if 18 as i32 == 'O' as i32 {
            14 as i32
        } else if 18 as i32 == 'P' as i32 {
            15 as i32
        } else if 18 as i32 == 'Q' as i32 {
            16 as i32
        } else if 18 as i32 == 'R' as i32 {
            17 as i32
        } else if 18 as i32 == 'S' as i32 {
            18 as i32
        } else if 18 as i32 == 'T' as i32 {
            19 as i32
        } else if 18 as i32 == 'U' as i32 {
            20 as i32
        } else if 18 as i32 == 'V' as i32 {
            21 as i32
        } else if 18 as i32 == 'W' as i32 {
            22 as i32
        } else if 18 as i32 == 'X' as i32 {
            23 as i32
        } else if 18 as i32 == 'Y' as i32 {
            24 as i32
        } else if 18 as i32 == 'Z' as i32 {
            25 as i32
        } else if 18 as i32 == '2' as i32 {
            26 as i32
        } else if 18 as i32 == '3' as i32 {
            27 as i32
        } else if 18 as i32 == '4' as i32 {
            28 as i32
        } else if 18 as i32 == '5' as i32 {
            29 as i32
        } else if 18 as i32 == '6' as i32 {
            30 as i32
        } else if 18 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 19 as i32 == 'A' as i32 {
            0 as i32
        } else if 19 as i32 == 'B' as i32 {
            1 as i32
        } else if 19 as i32 == 'C' as i32 {
            2 as i32
        } else if 19 as i32 == 'D' as i32 {
            3 as i32
        } else if 19 as i32 == 'E' as i32 {
            4 as i32
        } else if 19 as i32 == 'F' as i32 {
            5 as i32
        } else if 19 as i32 == 'G' as i32 {
            6 as i32
        } else if 19 as i32 == 'H' as i32 {
            7 as i32
        } else if 19 as i32 == 'I' as i32 {
            8 as i32
        } else if 19 as i32 == 'J' as i32 {
            9 as i32
        } else if 19 as i32 == 'K' as i32 {
            10 as i32
        } else if 19 as i32 == 'L' as i32 {
            11 as i32
        } else if 19 as i32 == 'M' as i32 {
            12 as i32
        } else if 19 as i32 == 'N' as i32 {
            13 as i32
        } else if 19 as i32 == 'O' as i32 {
            14 as i32
        } else if 19 as i32 == 'P' as i32 {
            15 as i32
        } else if 19 as i32 == 'Q' as i32 {
            16 as i32
        } else if 19 as i32 == 'R' as i32 {
            17 as i32
        } else if 19 as i32 == 'S' as i32 {
            18 as i32
        } else if 19 as i32 == 'T' as i32 {
            19 as i32
        } else if 19 as i32 == 'U' as i32 {
            20 as i32
        } else if 19 as i32 == 'V' as i32 {
            21 as i32
        } else if 19 as i32 == 'W' as i32 {
            22 as i32
        } else if 19 as i32 == 'X' as i32 {
            23 as i32
        } else if 19 as i32 == 'Y' as i32 {
            24 as i32
        } else if 19 as i32 == 'Z' as i32 {
            25 as i32
        } else if 19 as i32 == '2' as i32 {
            26 as i32
        } else if 19 as i32 == '3' as i32 {
            27 as i32
        } else if 19 as i32 == '4' as i32 {
            28 as i32
        } else if 19 as i32 == '5' as i32 {
            29 as i32
        } else if 19 as i32 == '6' as i32 {
            30 as i32
        } else if 19 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 20 as i32 == 'A' as i32 {
            0 as i32
        } else if 20 as i32 == 'B' as i32 {
            1 as i32
        } else if 20 as i32 == 'C' as i32 {
            2 as i32
        } else if 20 as i32 == 'D' as i32 {
            3 as i32
        } else if 20 as i32 == 'E' as i32 {
            4 as i32
        } else if 20 as i32 == 'F' as i32 {
            5 as i32
        } else if 20 as i32 == 'G' as i32 {
            6 as i32
        } else if 20 as i32 == 'H' as i32 {
            7 as i32
        } else if 20 as i32 == 'I' as i32 {
            8 as i32
        } else if 20 as i32 == 'J' as i32 {
            9 as i32
        } else if 20 as i32 == 'K' as i32 {
            10 as i32
        } else if 20 as i32 == 'L' as i32 {
            11 as i32
        } else if 20 as i32 == 'M' as i32 {
            12 as i32
        } else if 20 as i32 == 'N' as i32 {
            13 as i32
        } else if 20 as i32 == 'O' as i32 {
            14 as i32
        } else if 20 as i32 == 'P' as i32 {
            15 as i32
        } else if 20 as i32 == 'Q' as i32 {
            16 as i32
        } else if 20 as i32 == 'R' as i32 {
            17 as i32
        } else if 20 as i32 == 'S' as i32 {
            18 as i32
        } else if 20 as i32 == 'T' as i32 {
            19 as i32
        } else if 20 as i32 == 'U' as i32 {
            20 as i32
        } else if 20 as i32 == 'V' as i32 {
            21 as i32
        } else if 20 as i32 == 'W' as i32 {
            22 as i32
        } else if 20 as i32 == 'X' as i32 {
            23 as i32
        } else if 20 as i32 == 'Y' as i32 {
            24 as i32
        } else if 20 as i32 == 'Z' as i32 {
            25 as i32
        } else if 20 as i32 == '2' as i32 {
            26 as i32
        } else if 20 as i32 == '3' as i32 {
            27 as i32
        } else if 20 as i32 == '4' as i32 {
            28 as i32
        } else if 20 as i32 == '5' as i32 {
            29 as i32
        } else if 20 as i32 == '6' as i32 {
            30 as i32
        } else if 20 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 21 as i32 == 'A' as i32 {
            0 as i32
        } else if 21 as i32 == 'B' as i32 {
            1 as i32
        } else if 21 as i32 == 'C' as i32 {
            2 as i32
        } else if 21 as i32 == 'D' as i32 {
            3 as i32
        } else if 21 as i32 == 'E' as i32 {
            4 as i32
        } else if 21 as i32 == 'F' as i32 {
            5 as i32
        } else if 21 as i32 == 'G' as i32 {
            6 as i32
        } else if 21 as i32 == 'H' as i32 {
            7 as i32
        } else if 21 as i32 == 'I' as i32 {
            8 as i32
        } else if 21 as i32 == 'J' as i32 {
            9 as i32
        } else if 21 as i32 == 'K' as i32 {
            10 as i32
        } else if 21 as i32 == 'L' as i32 {
            11 as i32
        } else if 21 as i32 == 'M' as i32 {
            12 as i32
        } else if 21 as i32 == 'N' as i32 {
            13 as i32
        } else if 21 as i32 == 'O' as i32 {
            14 as i32
        } else if 21 as i32 == 'P' as i32 {
            15 as i32
        } else if 21 as i32 == 'Q' as i32 {
            16 as i32
        } else if 21 as i32 == 'R' as i32 {
            17 as i32
        } else if 21 as i32 == 'S' as i32 {
            18 as i32
        } else if 21 as i32 == 'T' as i32 {
            19 as i32
        } else if 21 as i32 == 'U' as i32 {
            20 as i32
        } else if 21 as i32 == 'V' as i32 {
            21 as i32
        } else if 21 as i32 == 'W' as i32 {
            22 as i32
        } else if 21 as i32 == 'X' as i32 {
            23 as i32
        } else if 21 as i32 == 'Y' as i32 {
            24 as i32
        } else if 21 as i32 == 'Z' as i32 {
            25 as i32
        } else if 21 as i32 == '2' as i32 {
            26 as i32
        } else if 21 as i32 == '3' as i32 {
            27 as i32
        } else if 21 as i32 == '4' as i32 {
            28 as i32
        } else if 21 as i32 == '5' as i32 {
            29 as i32
        } else if 21 as i32 == '6' as i32 {
            30 as i32
        } else if 21 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 22 as i32 == 'A' as i32 {
            0 as i32
        } else if 22 as i32 == 'B' as i32 {
            1 as i32
        } else if 22 as i32 == 'C' as i32 {
            2 as i32
        } else if 22 as i32 == 'D' as i32 {
            3 as i32
        } else if 22 as i32 == 'E' as i32 {
            4 as i32
        } else if 22 as i32 == 'F' as i32 {
            5 as i32
        } else if 22 as i32 == 'G' as i32 {
            6 as i32
        } else if 22 as i32 == 'H' as i32 {
            7 as i32
        } else if 22 as i32 == 'I' as i32 {
            8 as i32
        } else if 22 as i32 == 'J' as i32 {
            9 as i32
        } else if 22 as i32 == 'K' as i32 {
            10 as i32
        } else if 22 as i32 == 'L' as i32 {
            11 as i32
        } else if 22 as i32 == 'M' as i32 {
            12 as i32
        } else if 22 as i32 == 'N' as i32 {
            13 as i32
        } else if 22 as i32 == 'O' as i32 {
            14 as i32
        } else if 22 as i32 == 'P' as i32 {
            15 as i32
        } else if 22 as i32 == 'Q' as i32 {
            16 as i32
        } else if 22 as i32 == 'R' as i32 {
            17 as i32
        } else if 22 as i32 == 'S' as i32 {
            18 as i32
        } else if 22 as i32 == 'T' as i32 {
            19 as i32
        } else if 22 as i32 == 'U' as i32 {
            20 as i32
        } else if 22 as i32 == 'V' as i32 {
            21 as i32
        } else if 22 as i32 == 'W' as i32 {
            22 as i32
        } else if 22 as i32 == 'X' as i32 {
            23 as i32
        } else if 22 as i32 == 'Y' as i32 {
            24 as i32
        } else if 22 as i32 == 'Z' as i32 {
            25 as i32
        } else if 22 as i32 == '2' as i32 {
            26 as i32
        } else if 22 as i32 == '3' as i32 {
            27 as i32
        } else if 22 as i32 == '4' as i32 {
            28 as i32
        } else if 22 as i32 == '5' as i32 {
            29 as i32
        } else if 22 as i32 == '6' as i32 {
            30 as i32
        } else if 22 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 23 as i32 == 'A' as i32 {
            0 as i32
        } else if 23 as i32 == 'B' as i32 {
            1 as i32
        } else if 23 as i32 == 'C' as i32 {
            2 as i32
        } else if 23 as i32 == 'D' as i32 {
            3 as i32
        } else if 23 as i32 == 'E' as i32 {
            4 as i32
        } else if 23 as i32 == 'F' as i32 {
            5 as i32
        } else if 23 as i32 == 'G' as i32 {
            6 as i32
        } else if 23 as i32 == 'H' as i32 {
            7 as i32
        } else if 23 as i32 == 'I' as i32 {
            8 as i32
        } else if 23 as i32 == 'J' as i32 {
            9 as i32
        } else if 23 as i32 == 'K' as i32 {
            10 as i32
        } else if 23 as i32 == 'L' as i32 {
            11 as i32
        } else if 23 as i32 == 'M' as i32 {
            12 as i32
        } else if 23 as i32 == 'N' as i32 {
            13 as i32
        } else if 23 as i32 == 'O' as i32 {
            14 as i32
        } else if 23 as i32 == 'P' as i32 {
            15 as i32
        } else if 23 as i32 == 'Q' as i32 {
            16 as i32
        } else if 23 as i32 == 'R' as i32 {
            17 as i32
        } else if 23 as i32 == 'S' as i32 {
            18 as i32
        } else if 23 as i32 == 'T' as i32 {
            19 as i32
        } else if 23 as i32 == 'U' as i32 {
            20 as i32
        } else if 23 as i32 == 'V' as i32 {
            21 as i32
        } else if 23 as i32 == 'W' as i32 {
            22 as i32
        } else if 23 as i32 == 'X' as i32 {
            23 as i32
        } else if 23 as i32 == 'Y' as i32 {
            24 as i32
        } else if 23 as i32 == 'Z' as i32 {
            25 as i32
        } else if 23 as i32 == '2' as i32 {
            26 as i32
        } else if 23 as i32 == '3' as i32 {
            27 as i32
        } else if 23 as i32 == '4' as i32 {
            28 as i32
        } else if 23 as i32 == '5' as i32 {
            29 as i32
        } else if 23 as i32 == '6' as i32 {
            30 as i32
        } else if 23 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 24 as i32 == 'A' as i32 {
            0 as i32
        } else if 24 as i32 == 'B' as i32 {
            1 as i32
        } else if 24 as i32 == 'C' as i32 {
            2 as i32
        } else if 24 as i32 == 'D' as i32 {
            3 as i32
        } else if 24 as i32 == 'E' as i32 {
            4 as i32
        } else if 24 as i32 == 'F' as i32 {
            5 as i32
        } else if 24 as i32 == 'G' as i32 {
            6 as i32
        } else if 24 as i32 == 'H' as i32 {
            7 as i32
        } else if 24 as i32 == 'I' as i32 {
            8 as i32
        } else if 24 as i32 == 'J' as i32 {
            9 as i32
        } else if 24 as i32 == 'K' as i32 {
            10 as i32
        } else if 24 as i32 == 'L' as i32 {
            11 as i32
        } else if 24 as i32 == 'M' as i32 {
            12 as i32
        } else if 24 as i32 == 'N' as i32 {
            13 as i32
        } else if 24 as i32 == 'O' as i32 {
            14 as i32
        } else if 24 as i32 == 'P' as i32 {
            15 as i32
        } else if 24 as i32 == 'Q' as i32 {
            16 as i32
        } else if 24 as i32 == 'R' as i32 {
            17 as i32
        } else if 24 as i32 == 'S' as i32 {
            18 as i32
        } else if 24 as i32 == 'T' as i32 {
            19 as i32
        } else if 24 as i32 == 'U' as i32 {
            20 as i32
        } else if 24 as i32 == 'V' as i32 {
            21 as i32
        } else if 24 as i32 == 'W' as i32 {
            22 as i32
        } else if 24 as i32 == 'X' as i32 {
            23 as i32
        } else if 24 as i32 == 'Y' as i32 {
            24 as i32
        } else if 24 as i32 == 'Z' as i32 {
            25 as i32
        } else if 24 as i32 == '2' as i32 {
            26 as i32
        } else if 24 as i32 == '3' as i32 {
            27 as i32
        } else if 24 as i32 == '4' as i32 {
            28 as i32
        } else if 24 as i32 == '5' as i32 {
            29 as i32
        } else if 24 as i32 == '6' as i32 {
            30 as i32
        } else if 24 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 25 as i32 == 'A' as i32 {
            0 as i32
        } else if 25 as i32 == 'B' as i32 {
            1 as i32
        } else if 25 as i32 == 'C' as i32 {
            2 as i32
        } else if 25 as i32 == 'D' as i32 {
            3 as i32
        } else if 25 as i32 == 'E' as i32 {
            4 as i32
        } else if 25 as i32 == 'F' as i32 {
            5 as i32
        } else if 25 as i32 == 'G' as i32 {
            6 as i32
        } else if 25 as i32 == 'H' as i32 {
            7 as i32
        } else if 25 as i32 == 'I' as i32 {
            8 as i32
        } else if 25 as i32 == 'J' as i32 {
            9 as i32
        } else if 25 as i32 == 'K' as i32 {
            10 as i32
        } else if 25 as i32 == 'L' as i32 {
            11 as i32
        } else if 25 as i32 == 'M' as i32 {
            12 as i32
        } else if 25 as i32 == 'N' as i32 {
            13 as i32
        } else if 25 as i32 == 'O' as i32 {
            14 as i32
        } else if 25 as i32 == 'P' as i32 {
            15 as i32
        } else if 25 as i32 == 'Q' as i32 {
            16 as i32
        } else if 25 as i32 == 'R' as i32 {
            17 as i32
        } else if 25 as i32 == 'S' as i32 {
            18 as i32
        } else if 25 as i32 == 'T' as i32 {
            19 as i32
        } else if 25 as i32 == 'U' as i32 {
            20 as i32
        } else if 25 as i32 == 'V' as i32 {
            21 as i32
        } else if 25 as i32 == 'W' as i32 {
            22 as i32
        } else if 25 as i32 == 'X' as i32 {
            23 as i32
        } else if 25 as i32 == 'Y' as i32 {
            24 as i32
        } else if 25 as i32 == 'Z' as i32 {
            25 as i32
        } else if 25 as i32 == '2' as i32 {
            26 as i32
        } else if 25 as i32 == '3' as i32 {
            27 as i32
        } else if 25 as i32 == '4' as i32 {
            28 as i32
        } else if 25 as i32 == '5' as i32 {
            29 as i32
        } else if 25 as i32 == '6' as i32 {
            30 as i32
        } else if 25 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 26 as i32 == 'A' as i32 {
            0 as i32
        } else if 26 as i32 == 'B' as i32 {
            1 as i32
        } else if 26 as i32 == 'C' as i32 {
            2 as i32
        } else if 26 as i32 == 'D' as i32 {
            3 as i32
        } else if 26 as i32 == 'E' as i32 {
            4 as i32
        } else if 26 as i32 == 'F' as i32 {
            5 as i32
        } else if 26 as i32 == 'G' as i32 {
            6 as i32
        } else if 26 as i32 == 'H' as i32 {
            7 as i32
        } else if 26 as i32 == 'I' as i32 {
            8 as i32
        } else if 26 as i32 == 'J' as i32 {
            9 as i32
        } else if 26 as i32 == 'K' as i32 {
            10 as i32
        } else if 26 as i32 == 'L' as i32 {
            11 as i32
        } else if 26 as i32 == 'M' as i32 {
            12 as i32
        } else if 26 as i32 == 'N' as i32 {
            13 as i32
        } else if 26 as i32 == 'O' as i32 {
            14 as i32
        } else if 26 as i32 == 'P' as i32 {
            15 as i32
        } else if 26 as i32 == 'Q' as i32 {
            16 as i32
        } else if 26 as i32 == 'R' as i32 {
            17 as i32
        } else if 26 as i32 == 'S' as i32 {
            18 as i32
        } else if 26 as i32 == 'T' as i32 {
            19 as i32
        } else if 26 as i32 == 'U' as i32 {
            20 as i32
        } else if 26 as i32 == 'V' as i32 {
            21 as i32
        } else if 26 as i32 == 'W' as i32 {
            22 as i32
        } else if 26 as i32 == 'X' as i32 {
            23 as i32
        } else if 26 as i32 == 'Y' as i32 {
            24 as i32
        } else if 26 as i32 == 'Z' as i32 {
            25 as i32
        } else if 26 as i32 == '2' as i32 {
            26 as i32
        } else if 26 as i32 == '3' as i32 {
            27 as i32
        } else if 26 as i32 == '4' as i32 {
            28 as i32
        } else if 26 as i32 == '5' as i32 {
            29 as i32
        } else if 26 as i32 == '6' as i32 {
            30 as i32
        } else if 26 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 27 as i32 == 'A' as i32 {
            0 as i32
        } else if 27 as i32 == 'B' as i32 {
            1 as i32
        } else if 27 as i32 == 'C' as i32 {
            2 as i32
        } else if 27 as i32 == 'D' as i32 {
            3 as i32
        } else if 27 as i32 == 'E' as i32 {
            4 as i32
        } else if 27 as i32 == 'F' as i32 {
            5 as i32
        } else if 27 as i32 == 'G' as i32 {
            6 as i32
        } else if 27 as i32 == 'H' as i32 {
            7 as i32
        } else if 27 as i32 == 'I' as i32 {
            8 as i32
        } else if 27 as i32 == 'J' as i32 {
            9 as i32
        } else if 27 as i32 == 'K' as i32 {
            10 as i32
        } else if 27 as i32 == 'L' as i32 {
            11 as i32
        } else if 27 as i32 == 'M' as i32 {
            12 as i32
        } else if 27 as i32 == 'N' as i32 {
            13 as i32
        } else if 27 as i32 == 'O' as i32 {
            14 as i32
        } else if 27 as i32 == 'P' as i32 {
            15 as i32
        } else if 27 as i32 == 'Q' as i32 {
            16 as i32
        } else if 27 as i32 == 'R' as i32 {
            17 as i32
        } else if 27 as i32 == 'S' as i32 {
            18 as i32
        } else if 27 as i32 == 'T' as i32 {
            19 as i32
        } else if 27 as i32 == 'U' as i32 {
            20 as i32
        } else if 27 as i32 == 'V' as i32 {
            21 as i32
        } else if 27 as i32 == 'W' as i32 {
            22 as i32
        } else if 27 as i32 == 'X' as i32 {
            23 as i32
        } else if 27 as i32 == 'Y' as i32 {
            24 as i32
        } else if 27 as i32 == 'Z' as i32 {
            25 as i32
        } else if 27 as i32 == '2' as i32 {
            26 as i32
        } else if 27 as i32 == '3' as i32 {
            27 as i32
        } else if 27 as i32 == '4' as i32 {
            28 as i32
        } else if 27 as i32 == '5' as i32 {
            29 as i32
        } else if 27 as i32 == '6' as i32 {
            30 as i32
        } else if 27 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 28 as i32 == 'A' as i32 {
            0 as i32
        } else if 28 as i32 == 'B' as i32 {
            1 as i32
        } else if 28 as i32 == 'C' as i32 {
            2 as i32
        } else if 28 as i32 == 'D' as i32 {
            3 as i32
        } else if 28 as i32 == 'E' as i32 {
            4 as i32
        } else if 28 as i32 == 'F' as i32 {
            5 as i32
        } else if 28 as i32 == 'G' as i32 {
            6 as i32
        } else if 28 as i32 == 'H' as i32 {
            7 as i32
        } else if 28 as i32 == 'I' as i32 {
            8 as i32
        } else if 28 as i32 == 'J' as i32 {
            9 as i32
        } else if 28 as i32 == 'K' as i32 {
            10 as i32
        } else if 28 as i32 == 'L' as i32 {
            11 as i32
        } else if 28 as i32 == 'M' as i32 {
            12 as i32
        } else if 28 as i32 == 'N' as i32 {
            13 as i32
        } else if 28 as i32 == 'O' as i32 {
            14 as i32
        } else if 28 as i32 == 'P' as i32 {
            15 as i32
        } else if 28 as i32 == 'Q' as i32 {
            16 as i32
        } else if 28 as i32 == 'R' as i32 {
            17 as i32
        } else if 28 as i32 == 'S' as i32 {
            18 as i32
        } else if 28 as i32 == 'T' as i32 {
            19 as i32
        } else if 28 as i32 == 'U' as i32 {
            20 as i32
        } else if 28 as i32 == 'V' as i32 {
            21 as i32
        } else if 28 as i32 == 'W' as i32 {
            22 as i32
        } else if 28 as i32 == 'X' as i32 {
            23 as i32
        } else if 28 as i32 == 'Y' as i32 {
            24 as i32
        } else if 28 as i32 == 'Z' as i32 {
            25 as i32
        } else if 28 as i32 == '2' as i32 {
            26 as i32
        } else if 28 as i32 == '3' as i32 {
            27 as i32
        } else if 28 as i32 == '4' as i32 {
            28 as i32
        } else if 28 as i32 == '5' as i32 {
            29 as i32
        } else if 28 as i32 == '6' as i32 {
            30 as i32
        } else if 28 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 29 as i32 == 'A' as i32 {
            0 as i32
        } else if 29 as i32 == 'B' as i32 {
            1 as i32
        } else if 29 as i32 == 'C' as i32 {
            2 as i32
        } else if 29 as i32 == 'D' as i32 {
            3 as i32
        } else if 29 as i32 == 'E' as i32 {
            4 as i32
        } else if 29 as i32 == 'F' as i32 {
            5 as i32
        } else if 29 as i32 == 'G' as i32 {
            6 as i32
        } else if 29 as i32 == 'H' as i32 {
            7 as i32
        } else if 29 as i32 == 'I' as i32 {
            8 as i32
        } else if 29 as i32 == 'J' as i32 {
            9 as i32
        } else if 29 as i32 == 'K' as i32 {
            10 as i32
        } else if 29 as i32 == 'L' as i32 {
            11 as i32
        } else if 29 as i32 == 'M' as i32 {
            12 as i32
        } else if 29 as i32 == 'N' as i32 {
            13 as i32
        } else if 29 as i32 == 'O' as i32 {
            14 as i32
        } else if 29 as i32 == 'P' as i32 {
            15 as i32
        } else if 29 as i32 == 'Q' as i32 {
            16 as i32
        } else if 29 as i32 == 'R' as i32 {
            17 as i32
        } else if 29 as i32 == 'S' as i32 {
            18 as i32
        } else if 29 as i32 == 'T' as i32 {
            19 as i32
        } else if 29 as i32 == 'U' as i32 {
            20 as i32
        } else if 29 as i32 == 'V' as i32 {
            21 as i32
        } else if 29 as i32 == 'W' as i32 {
            22 as i32
        } else if 29 as i32 == 'X' as i32 {
            23 as i32
        } else if 29 as i32 == 'Y' as i32 {
            24 as i32
        } else if 29 as i32 == 'Z' as i32 {
            25 as i32
        } else if 29 as i32 == '2' as i32 {
            26 as i32
        } else if 29 as i32 == '3' as i32 {
            27 as i32
        } else if 29 as i32 == '4' as i32 {
            28 as i32
        } else if 29 as i32 == '5' as i32 {
            29 as i32
        } else if 29 as i32 == '6' as i32 {
            30 as i32
        } else if 29 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 30 as i32 == 'A' as i32 {
            0 as i32
        } else if 30 as i32 == 'B' as i32 {
            1 as i32
        } else if 30 as i32 == 'C' as i32 {
            2 as i32
        } else if 30 as i32 == 'D' as i32 {
            3 as i32
        } else if 30 as i32 == 'E' as i32 {
            4 as i32
        } else if 30 as i32 == 'F' as i32 {
            5 as i32
        } else if 30 as i32 == 'G' as i32 {
            6 as i32
        } else if 30 as i32 == 'H' as i32 {
            7 as i32
        } else if 30 as i32 == 'I' as i32 {
            8 as i32
        } else if 30 as i32 == 'J' as i32 {
            9 as i32
        } else if 30 as i32 == 'K' as i32 {
            10 as i32
        } else if 30 as i32 == 'L' as i32 {
            11 as i32
        } else if 30 as i32 == 'M' as i32 {
            12 as i32
        } else if 30 as i32 == 'N' as i32 {
            13 as i32
        } else if 30 as i32 == 'O' as i32 {
            14 as i32
        } else if 30 as i32 == 'P' as i32 {
            15 as i32
        } else if 30 as i32 == 'Q' as i32 {
            16 as i32
        } else if 30 as i32 == 'R' as i32 {
            17 as i32
        } else if 30 as i32 == 'S' as i32 {
            18 as i32
        } else if 30 as i32 == 'T' as i32 {
            19 as i32
        } else if 30 as i32 == 'U' as i32 {
            20 as i32
        } else if 30 as i32 == 'V' as i32 {
            21 as i32
        } else if 30 as i32 == 'W' as i32 {
            22 as i32
        } else if 30 as i32 == 'X' as i32 {
            23 as i32
        } else if 30 as i32 == 'Y' as i32 {
            24 as i32
        } else if 30 as i32 == 'Z' as i32 {
            25 as i32
        } else if 30 as i32 == '2' as i32 {
            26 as i32
        } else if 30 as i32 == '3' as i32 {
            27 as i32
        } else if 30 as i32 == '4' as i32 {
            28 as i32
        } else if 30 as i32 == '5' as i32 {
            29 as i32
        } else if 30 as i32 == '6' as i32 {
            30 as i32
        } else if 30 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 31 as i32 == 'A' as i32 {
            0 as i32
        } else if 31 as i32 == 'B' as i32 {
            1 as i32
        } else if 31 as i32 == 'C' as i32 {
            2 as i32
        } else if 31 as i32 == 'D' as i32 {
            3 as i32
        } else if 31 as i32 == 'E' as i32 {
            4 as i32
        } else if 31 as i32 == 'F' as i32 {
            5 as i32
        } else if 31 as i32 == 'G' as i32 {
            6 as i32
        } else if 31 as i32 == 'H' as i32 {
            7 as i32
        } else if 31 as i32 == 'I' as i32 {
            8 as i32
        } else if 31 as i32 == 'J' as i32 {
            9 as i32
        } else if 31 as i32 == 'K' as i32 {
            10 as i32
        } else if 31 as i32 == 'L' as i32 {
            11 as i32
        } else if 31 as i32 == 'M' as i32 {
            12 as i32
        } else if 31 as i32 == 'N' as i32 {
            13 as i32
        } else if 31 as i32 == 'O' as i32 {
            14 as i32
        } else if 31 as i32 == 'P' as i32 {
            15 as i32
        } else if 31 as i32 == 'Q' as i32 {
            16 as i32
        } else if 31 as i32 == 'R' as i32 {
            17 as i32
        } else if 31 as i32 == 'S' as i32 {
            18 as i32
        } else if 31 as i32 == 'T' as i32 {
            19 as i32
        } else if 31 as i32 == 'U' as i32 {
            20 as i32
        } else if 31 as i32 == 'V' as i32 {
            21 as i32
        } else if 31 as i32 == 'W' as i32 {
            22 as i32
        } else if 31 as i32 == 'X' as i32 {
            23 as i32
        } else if 31 as i32 == 'Y' as i32 {
            24 as i32
        } else if 31 as i32 == 'Z' as i32 {
            25 as i32
        } else if 31 as i32 == '2' as i32 {
            26 as i32
        } else if 31 as i32 == '3' as i32 {
            27 as i32
        } else if 31 as i32 == '4' as i32 {
            28 as i32
        } else if 31 as i32 == '5' as i32 {
            29 as i32
        } else if 31 as i32 == '6' as i32 {
            30 as i32
        } else if 31 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 32 as i32 == 'A' as i32 {
            0 as i32
        } else if 32 as i32 == 'B' as i32 {
            1 as i32
        } else if 32 as i32 == 'C' as i32 {
            2 as i32
        } else if 32 as i32 == 'D' as i32 {
            3 as i32
        } else if 32 as i32 == 'E' as i32 {
            4 as i32
        } else if 32 as i32 == 'F' as i32 {
            5 as i32
        } else if 32 as i32 == 'G' as i32 {
            6 as i32
        } else if 32 as i32 == 'H' as i32 {
            7 as i32
        } else if 32 as i32 == 'I' as i32 {
            8 as i32
        } else if 32 as i32 == 'J' as i32 {
            9 as i32
        } else if 32 as i32 == 'K' as i32 {
            10 as i32
        } else if 32 as i32 == 'L' as i32 {
            11 as i32
        } else if 32 as i32 == 'M' as i32 {
            12 as i32
        } else if 32 as i32 == 'N' as i32 {
            13 as i32
        } else if 32 as i32 == 'O' as i32 {
            14 as i32
        } else if 32 as i32 == 'P' as i32 {
            15 as i32
        } else if 32 as i32 == 'Q' as i32 {
            16 as i32
        } else if 32 as i32 == 'R' as i32 {
            17 as i32
        } else if 32 as i32 == 'S' as i32 {
            18 as i32
        } else if 32 as i32 == 'T' as i32 {
            19 as i32
        } else if 32 as i32 == 'U' as i32 {
            20 as i32
        } else if 32 as i32 == 'V' as i32 {
            21 as i32
        } else if 32 as i32 == 'W' as i32 {
            22 as i32
        } else if 32 as i32 == 'X' as i32 {
            23 as i32
        } else if 32 as i32 == 'Y' as i32 {
            24 as i32
        } else if 32 as i32 == 'Z' as i32 {
            25 as i32
        } else if 32 as i32 == '2' as i32 {
            26 as i32
        } else if 32 as i32 == '3' as i32 {
            27 as i32
        } else if 32 as i32 == '4' as i32 {
            28 as i32
        } else if 32 as i32 == '5' as i32 {
            29 as i32
        } else if 32 as i32 == '6' as i32 {
            30 as i32
        } else if 32 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 33 as i32 == 'A' as i32 {
            0 as i32
        } else if 33 as i32 == 'B' as i32 {
            1 as i32
        } else if 33 as i32 == 'C' as i32 {
            2 as i32
        } else if 33 as i32 == 'D' as i32 {
            3 as i32
        } else if 33 as i32 == 'E' as i32 {
            4 as i32
        } else if 33 as i32 == 'F' as i32 {
            5 as i32
        } else if 33 as i32 == 'G' as i32 {
            6 as i32
        } else if 33 as i32 == 'H' as i32 {
            7 as i32
        } else if 33 as i32 == 'I' as i32 {
            8 as i32
        } else if 33 as i32 == 'J' as i32 {
            9 as i32
        } else if 33 as i32 == 'K' as i32 {
            10 as i32
        } else if 33 as i32 == 'L' as i32 {
            11 as i32
        } else if 33 as i32 == 'M' as i32 {
            12 as i32
        } else if 33 as i32 == 'N' as i32 {
            13 as i32
        } else if 33 as i32 == 'O' as i32 {
            14 as i32
        } else if 33 as i32 == 'P' as i32 {
            15 as i32
        } else if 33 as i32 == 'Q' as i32 {
            16 as i32
        } else if 33 as i32 == 'R' as i32 {
            17 as i32
        } else if 33 as i32 == 'S' as i32 {
            18 as i32
        } else if 33 as i32 == 'T' as i32 {
            19 as i32
        } else if 33 as i32 == 'U' as i32 {
            20 as i32
        } else if 33 as i32 == 'V' as i32 {
            21 as i32
        } else if 33 as i32 == 'W' as i32 {
            22 as i32
        } else if 33 as i32 == 'X' as i32 {
            23 as i32
        } else if 33 as i32 == 'Y' as i32 {
            24 as i32
        } else if 33 as i32 == 'Z' as i32 {
            25 as i32
        } else if 33 as i32 == '2' as i32 {
            26 as i32
        } else if 33 as i32 == '3' as i32 {
            27 as i32
        } else if 33 as i32 == '4' as i32 {
            28 as i32
        } else if 33 as i32 == '5' as i32 {
            29 as i32
        } else if 33 as i32 == '6' as i32 {
            30 as i32
        } else if 33 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 34 as i32 == 'A' as i32 {
            0 as i32
        } else if 34 as i32 == 'B' as i32 {
            1 as i32
        } else if 34 as i32 == 'C' as i32 {
            2 as i32
        } else if 34 as i32 == 'D' as i32 {
            3 as i32
        } else if 34 as i32 == 'E' as i32 {
            4 as i32
        } else if 34 as i32 == 'F' as i32 {
            5 as i32
        } else if 34 as i32 == 'G' as i32 {
            6 as i32
        } else if 34 as i32 == 'H' as i32 {
            7 as i32
        } else if 34 as i32 == 'I' as i32 {
            8 as i32
        } else if 34 as i32 == 'J' as i32 {
            9 as i32
        } else if 34 as i32 == 'K' as i32 {
            10 as i32
        } else if 34 as i32 == 'L' as i32 {
            11 as i32
        } else if 34 as i32 == 'M' as i32 {
            12 as i32
        } else if 34 as i32 == 'N' as i32 {
            13 as i32
        } else if 34 as i32 == 'O' as i32 {
            14 as i32
        } else if 34 as i32 == 'P' as i32 {
            15 as i32
        } else if 34 as i32 == 'Q' as i32 {
            16 as i32
        } else if 34 as i32 == 'R' as i32 {
            17 as i32
        } else if 34 as i32 == 'S' as i32 {
            18 as i32
        } else if 34 as i32 == 'T' as i32 {
            19 as i32
        } else if 34 as i32 == 'U' as i32 {
            20 as i32
        } else if 34 as i32 == 'V' as i32 {
            21 as i32
        } else if 34 as i32 == 'W' as i32 {
            22 as i32
        } else if 34 as i32 == 'X' as i32 {
            23 as i32
        } else if 34 as i32 == 'Y' as i32 {
            24 as i32
        } else if 34 as i32 == 'Z' as i32 {
            25 as i32
        } else if 34 as i32 == '2' as i32 {
            26 as i32
        } else if 34 as i32 == '3' as i32 {
            27 as i32
        } else if 34 as i32 == '4' as i32 {
            28 as i32
        } else if 34 as i32 == '5' as i32 {
            29 as i32
        } else if 34 as i32 == '6' as i32 {
            30 as i32
        } else if 34 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 35 as i32 == 'A' as i32 {
            0 as i32
        } else if 35 as i32 == 'B' as i32 {
            1 as i32
        } else if 35 as i32 == 'C' as i32 {
            2 as i32
        } else if 35 as i32 == 'D' as i32 {
            3 as i32
        } else if 35 as i32 == 'E' as i32 {
            4 as i32
        } else if 35 as i32 == 'F' as i32 {
            5 as i32
        } else if 35 as i32 == 'G' as i32 {
            6 as i32
        } else if 35 as i32 == 'H' as i32 {
            7 as i32
        } else if 35 as i32 == 'I' as i32 {
            8 as i32
        } else if 35 as i32 == 'J' as i32 {
            9 as i32
        } else if 35 as i32 == 'K' as i32 {
            10 as i32
        } else if 35 as i32 == 'L' as i32 {
            11 as i32
        } else if 35 as i32 == 'M' as i32 {
            12 as i32
        } else if 35 as i32 == 'N' as i32 {
            13 as i32
        } else if 35 as i32 == 'O' as i32 {
            14 as i32
        } else if 35 as i32 == 'P' as i32 {
            15 as i32
        } else if 35 as i32 == 'Q' as i32 {
            16 as i32
        } else if 35 as i32 == 'R' as i32 {
            17 as i32
        } else if 35 as i32 == 'S' as i32 {
            18 as i32
        } else if 35 as i32 == 'T' as i32 {
            19 as i32
        } else if 35 as i32 == 'U' as i32 {
            20 as i32
        } else if 35 as i32 == 'V' as i32 {
            21 as i32
        } else if 35 as i32 == 'W' as i32 {
            22 as i32
        } else if 35 as i32 == 'X' as i32 {
            23 as i32
        } else if 35 as i32 == 'Y' as i32 {
            24 as i32
        } else if 35 as i32 == 'Z' as i32 {
            25 as i32
        } else if 35 as i32 == '2' as i32 {
            26 as i32
        } else if 35 as i32 == '3' as i32 {
            27 as i32
        } else if 35 as i32 == '4' as i32 {
            28 as i32
        } else if 35 as i32 == '5' as i32 {
            29 as i32
        } else if 35 as i32 == '6' as i32 {
            30 as i32
        } else if 35 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 36 as i32 == 'A' as i32 {
            0 as i32
        } else if 36 as i32 == 'B' as i32 {
            1 as i32
        } else if 36 as i32 == 'C' as i32 {
            2 as i32
        } else if 36 as i32 == 'D' as i32 {
            3 as i32
        } else if 36 as i32 == 'E' as i32 {
            4 as i32
        } else if 36 as i32 == 'F' as i32 {
            5 as i32
        } else if 36 as i32 == 'G' as i32 {
            6 as i32
        } else if 36 as i32 == 'H' as i32 {
            7 as i32
        } else if 36 as i32 == 'I' as i32 {
            8 as i32
        } else if 36 as i32 == 'J' as i32 {
            9 as i32
        } else if 36 as i32 == 'K' as i32 {
            10 as i32
        } else if 36 as i32 == 'L' as i32 {
            11 as i32
        } else if 36 as i32 == 'M' as i32 {
            12 as i32
        } else if 36 as i32 == 'N' as i32 {
            13 as i32
        } else if 36 as i32 == 'O' as i32 {
            14 as i32
        } else if 36 as i32 == 'P' as i32 {
            15 as i32
        } else if 36 as i32 == 'Q' as i32 {
            16 as i32
        } else if 36 as i32 == 'R' as i32 {
            17 as i32
        } else if 36 as i32 == 'S' as i32 {
            18 as i32
        } else if 36 as i32 == 'T' as i32 {
            19 as i32
        } else if 36 as i32 == 'U' as i32 {
            20 as i32
        } else if 36 as i32 == 'V' as i32 {
            21 as i32
        } else if 36 as i32 == 'W' as i32 {
            22 as i32
        } else if 36 as i32 == 'X' as i32 {
            23 as i32
        } else if 36 as i32 == 'Y' as i32 {
            24 as i32
        } else if 36 as i32 == 'Z' as i32 {
            25 as i32
        } else if 36 as i32 == '2' as i32 {
            26 as i32
        } else if 36 as i32 == '3' as i32 {
            27 as i32
        } else if 36 as i32 == '4' as i32 {
            28 as i32
        } else if 36 as i32 == '5' as i32 {
            29 as i32
        } else if 36 as i32 == '6' as i32 {
            30 as i32
        } else if 36 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 37 as i32 == 'A' as i32 {
            0 as i32
        } else if 37 as i32 == 'B' as i32 {
            1 as i32
        } else if 37 as i32 == 'C' as i32 {
            2 as i32
        } else if 37 as i32 == 'D' as i32 {
            3 as i32
        } else if 37 as i32 == 'E' as i32 {
            4 as i32
        } else if 37 as i32 == 'F' as i32 {
            5 as i32
        } else if 37 as i32 == 'G' as i32 {
            6 as i32
        } else if 37 as i32 == 'H' as i32 {
            7 as i32
        } else if 37 as i32 == 'I' as i32 {
            8 as i32
        } else if 37 as i32 == 'J' as i32 {
            9 as i32
        } else if 37 as i32 == 'K' as i32 {
            10 as i32
        } else if 37 as i32 == 'L' as i32 {
            11 as i32
        } else if 37 as i32 == 'M' as i32 {
            12 as i32
        } else if 37 as i32 == 'N' as i32 {
            13 as i32
        } else if 37 as i32 == 'O' as i32 {
            14 as i32
        } else if 37 as i32 == 'P' as i32 {
            15 as i32
        } else if 37 as i32 == 'Q' as i32 {
            16 as i32
        } else if 37 as i32 == 'R' as i32 {
            17 as i32
        } else if 37 as i32 == 'S' as i32 {
            18 as i32
        } else if 37 as i32 == 'T' as i32 {
            19 as i32
        } else if 37 as i32 == 'U' as i32 {
            20 as i32
        } else if 37 as i32 == 'V' as i32 {
            21 as i32
        } else if 37 as i32 == 'W' as i32 {
            22 as i32
        } else if 37 as i32 == 'X' as i32 {
            23 as i32
        } else if 37 as i32 == 'Y' as i32 {
            24 as i32
        } else if 37 as i32 == 'Z' as i32 {
            25 as i32
        } else if 37 as i32 == '2' as i32 {
            26 as i32
        } else if 37 as i32 == '3' as i32 {
            27 as i32
        } else if 37 as i32 == '4' as i32 {
            28 as i32
        } else if 37 as i32 == '5' as i32 {
            29 as i32
        } else if 37 as i32 == '6' as i32 {
            30 as i32
        } else if 37 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 38 as i32 == 'A' as i32 {
            0 as i32
        } else if 38 as i32 == 'B' as i32 {
            1 as i32
        } else if 38 as i32 == 'C' as i32 {
            2 as i32
        } else if 38 as i32 == 'D' as i32 {
            3 as i32
        } else if 38 as i32 == 'E' as i32 {
            4 as i32
        } else if 38 as i32 == 'F' as i32 {
            5 as i32
        } else if 38 as i32 == 'G' as i32 {
            6 as i32
        } else if 38 as i32 == 'H' as i32 {
            7 as i32
        } else if 38 as i32 == 'I' as i32 {
            8 as i32
        } else if 38 as i32 == 'J' as i32 {
            9 as i32
        } else if 38 as i32 == 'K' as i32 {
            10 as i32
        } else if 38 as i32 == 'L' as i32 {
            11 as i32
        } else if 38 as i32 == 'M' as i32 {
            12 as i32
        } else if 38 as i32 == 'N' as i32 {
            13 as i32
        } else if 38 as i32 == 'O' as i32 {
            14 as i32
        } else if 38 as i32 == 'P' as i32 {
            15 as i32
        } else if 38 as i32 == 'Q' as i32 {
            16 as i32
        } else if 38 as i32 == 'R' as i32 {
            17 as i32
        } else if 38 as i32 == 'S' as i32 {
            18 as i32
        } else if 38 as i32 == 'T' as i32 {
            19 as i32
        } else if 38 as i32 == 'U' as i32 {
            20 as i32
        } else if 38 as i32 == 'V' as i32 {
            21 as i32
        } else if 38 as i32 == 'W' as i32 {
            22 as i32
        } else if 38 as i32 == 'X' as i32 {
            23 as i32
        } else if 38 as i32 == 'Y' as i32 {
            24 as i32
        } else if 38 as i32 == 'Z' as i32 {
            25 as i32
        } else if 38 as i32 == '2' as i32 {
            26 as i32
        } else if 38 as i32 == '3' as i32 {
            27 as i32
        } else if 38 as i32 == '4' as i32 {
            28 as i32
        } else if 38 as i32 == '5' as i32 {
            29 as i32
        } else if 38 as i32 == '6' as i32 {
            30 as i32
        } else if 38 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 39 as i32 == 'A' as i32 {
            0 as i32
        } else if 39 as i32 == 'B' as i32 {
            1 as i32
        } else if 39 as i32 == 'C' as i32 {
            2 as i32
        } else if 39 as i32 == 'D' as i32 {
            3 as i32
        } else if 39 as i32 == 'E' as i32 {
            4 as i32
        } else if 39 as i32 == 'F' as i32 {
            5 as i32
        } else if 39 as i32 == 'G' as i32 {
            6 as i32
        } else if 39 as i32 == 'H' as i32 {
            7 as i32
        } else if 39 as i32 == 'I' as i32 {
            8 as i32
        } else if 39 as i32 == 'J' as i32 {
            9 as i32
        } else if 39 as i32 == 'K' as i32 {
            10 as i32
        } else if 39 as i32 == 'L' as i32 {
            11 as i32
        } else if 39 as i32 == 'M' as i32 {
            12 as i32
        } else if 39 as i32 == 'N' as i32 {
            13 as i32
        } else if 39 as i32 == 'O' as i32 {
            14 as i32
        } else if 39 as i32 == 'P' as i32 {
            15 as i32
        } else if 39 as i32 == 'Q' as i32 {
            16 as i32
        } else if 39 as i32 == 'R' as i32 {
            17 as i32
        } else if 39 as i32 == 'S' as i32 {
            18 as i32
        } else if 39 as i32 == 'T' as i32 {
            19 as i32
        } else if 39 as i32 == 'U' as i32 {
            20 as i32
        } else if 39 as i32 == 'V' as i32 {
            21 as i32
        } else if 39 as i32 == 'W' as i32 {
            22 as i32
        } else if 39 as i32 == 'X' as i32 {
            23 as i32
        } else if 39 as i32 == 'Y' as i32 {
            24 as i32
        } else if 39 as i32 == 'Z' as i32 {
            25 as i32
        } else if 39 as i32 == '2' as i32 {
            26 as i32
        } else if 39 as i32 == '3' as i32 {
            27 as i32
        } else if 39 as i32 == '4' as i32 {
            28 as i32
        } else if 39 as i32 == '5' as i32 {
            29 as i32
        } else if 39 as i32 == '6' as i32 {
            30 as i32
        } else if 39 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 40 as i32 == 'A' as i32 {
            0 as i32
        } else if 40 as i32 == 'B' as i32 {
            1 as i32
        } else if 40 as i32 == 'C' as i32 {
            2 as i32
        } else if 40 as i32 == 'D' as i32 {
            3 as i32
        } else if 40 as i32 == 'E' as i32 {
            4 as i32
        } else if 40 as i32 == 'F' as i32 {
            5 as i32
        } else if 40 as i32 == 'G' as i32 {
            6 as i32
        } else if 40 as i32 == 'H' as i32 {
            7 as i32
        } else if 40 as i32 == 'I' as i32 {
            8 as i32
        } else if 40 as i32 == 'J' as i32 {
            9 as i32
        } else if 40 as i32 == 'K' as i32 {
            10 as i32
        } else if 40 as i32 == 'L' as i32 {
            11 as i32
        } else if 40 as i32 == 'M' as i32 {
            12 as i32
        } else if 40 as i32 == 'N' as i32 {
            13 as i32
        } else if 40 as i32 == 'O' as i32 {
            14 as i32
        } else if 40 as i32 == 'P' as i32 {
            15 as i32
        } else if 40 as i32 == 'Q' as i32 {
            16 as i32
        } else if 40 as i32 == 'R' as i32 {
            17 as i32
        } else if 40 as i32 == 'S' as i32 {
            18 as i32
        } else if 40 as i32 == 'T' as i32 {
            19 as i32
        } else if 40 as i32 == 'U' as i32 {
            20 as i32
        } else if 40 as i32 == 'V' as i32 {
            21 as i32
        } else if 40 as i32 == 'W' as i32 {
            22 as i32
        } else if 40 as i32 == 'X' as i32 {
            23 as i32
        } else if 40 as i32 == 'Y' as i32 {
            24 as i32
        } else if 40 as i32 == 'Z' as i32 {
            25 as i32
        } else if 40 as i32 == '2' as i32 {
            26 as i32
        } else if 40 as i32 == '3' as i32 {
            27 as i32
        } else if 40 as i32 == '4' as i32 {
            28 as i32
        } else if 40 as i32 == '5' as i32 {
            29 as i32
        } else if 40 as i32 == '6' as i32 {
            30 as i32
        } else if 40 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 41 as i32 == 'A' as i32 {
            0 as i32
        } else if 41 as i32 == 'B' as i32 {
            1 as i32
        } else if 41 as i32 == 'C' as i32 {
            2 as i32
        } else if 41 as i32 == 'D' as i32 {
            3 as i32
        } else if 41 as i32 == 'E' as i32 {
            4 as i32
        } else if 41 as i32 == 'F' as i32 {
            5 as i32
        } else if 41 as i32 == 'G' as i32 {
            6 as i32
        } else if 41 as i32 == 'H' as i32 {
            7 as i32
        } else if 41 as i32 == 'I' as i32 {
            8 as i32
        } else if 41 as i32 == 'J' as i32 {
            9 as i32
        } else if 41 as i32 == 'K' as i32 {
            10 as i32
        } else if 41 as i32 == 'L' as i32 {
            11 as i32
        } else if 41 as i32 == 'M' as i32 {
            12 as i32
        } else if 41 as i32 == 'N' as i32 {
            13 as i32
        } else if 41 as i32 == 'O' as i32 {
            14 as i32
        } else if 41 as i32 == 'P' as i32 {
            15 as i32
        } else if 41 as i32 == 'Q' as i32 {
            16 as i32
        } else if 41 as i32 == 'R' as i32 {
            17 as i32
        } else if 41 as i32 == 'S' as i32 {
            18 as i32
        } else if 41 as i32 == 'T' as i32 {
            19 as i32
        } else if 41 as i32 == 'U' as i32 {
            20 as i32
        } else if 41 as i32 == 'V' as i32 {
            21 as i32
        } else if 41 as i32 == 'W' as i32 {
            22 as i32
        } else if 41 as i32 == 'X' as i32 {
            23 as i32
        } else if 41 as i32 == 'Y' as i32 {
            24 as i32
        } else if 41 as i32 == 'Z' as i32 {
            25 as i32
        } else if 41 as i32 == '2' as i32 {
            26 as i32
        } else if 41 as i32 == '3' as i32 {
            27 as i32
        } else if 41 as i32 == '4' as i32 {
            28 as i32
        } else if 41 as i32 == '5' as i32 {
            29 as i32
        } else if 41 as i32 == '6' as i32 {
            30 as i32
        } else if 41 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 42 as i32 == 'A' as i32 {
            0 as i32
        } else if 42 as i32 == 'B' as i32 {
            1 as i32
        } else if 42 as i32 == 'C' as i32 {
            2 as i32
        } else if 42 as i32 == 'D' as i32 {
            3 as i32
        } else if 42 as i32 == 'E' as i32 {
            4 as i32
        } else if 42 as i32 == 'F' as i32 {
            5 as i32
        } else if 42 as i32 == 'G' as i32 {
            6 as i32
        } else if 42 as i32 == 'H' as i32 {
            7 as i32
        } else if 42 as i32 == 'I' as i32 {
            8 as i32
        } else if 42 as i32 == 'J' as i32 {
            9 as i32
        } else if 42 as i32 == 'K' as i32 {
            10 as i32
        } else if 42 as i32 == 'L' as i32 {
            11 as i32
        } else if 42 as i32 == 'M' as i32 {
            12 as i32
        } else if 42 as i32 == 'N' as i32 {
            13 as i32
        } else if 42 as i32 == 'O' as i32 {
            14 as i32
        } else if 42 as i32 == 'P' as i32 {
            15 as i32
        } else if 42 as i32 == 'Q' as i32 {
            16 as i32
        } else if 42 as i32 == 'R' as i32 {
            17 as i32
        } else if 42 as i32 == 'S' as i32 {
            18 as i32
        } else if 42 as i32 == 'T' as i32 {
            19 as i32
        } else if 42 as i32 == 'U' as i32 {
            20 as i32
        } else if 42 as i32 == 'V' as i32 {
            21 as i32
        } else if 42 as i32 == 'W' as i32 {
            22 as i32
        } else if 42 as i32 == 'X' as i32 {
            23 as i32
        } else if 42 as i32 == 'Y' as i32 {
            24 as i32
        } else if 42 as i32 == 'Z' as i32 {
            25 as i32
        } else if 42 as i32 == '2' as i32 {
            26 as i32
        } else if 42 as i32 == '3' as i32 {
            27 as i32
        } else if 42 as i32 == '4' as i32 {
            28 as i32
        } else if 42 as i32 == '5' as i32 {
            29 as i32
        } else if 42 as i32 == '6' as i32 {
            30 as i32
        } else if 42 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 43 as i32 == 'A' as i32 {
            0 as i32
        } else if 43 as i32 == 'B' as i32 {
            1 as i32
        } else if 43 as i32 == 'C' as i32 {
            2 as i32
        } else if 43 as i32 == 'D' as i32 {
            3 as i32
        } else if 43 as i32 == 'E' as i32 {
            4 as i32
        } else if 43 as i32 == 'F' as i32 {
            5 as i32
        } else if 43 as i32 == 'G' as i32 {
            6 as i32
        } else if 43 as i32 == 'H' as i32 {
            7 as i32
        } else if 43 as i32 == 'I' as i32 {
            8 as i32
        } else if 43 as i32 == 'J' as i32 {
            9 as i32
        } else if 43 as i32 == 'K' as i32 {
            10 as i32
        } else if 43 as i32 == 'L' as i32 {
            11 as i32
        } else if 43 as i32 == 'M' as i32 {
            12 as i32
        } else if 43 as i32 == 'N' as i32 {
            13 as i32
        } else if 43 as i32 == 'O' as i32 {
            14 as i32
        } else if 43 as i32 == 'P' as i32 {
            15 as i32
        } else if 43 as i32 == 'Q' as i32 {
            16 as i32
        } else if 43 as i32 == 'R' as i32 {
            17 as i32
        } else if 43 as i32 == 'S' as i32 {
            18 as i32
        } else if 43 as i32 == 'T' as i32 {
            19 as i32
        } else if 43 as i32 == 'U' as i32 {
            20 as i32
        } else if 43 as i32 == 'V' as i32 {
            21 as i32
        } else if 43 as i32 == 'W' as i32 {
            22 as i32
        } else if 43 as i32 == 'X' as i32 {
            23 as i32
        } else if 43 as i32 == 'Y' as i32 {
            24 as i32
        } else if 43 as i32 == 'Z' as i32 {
            25 as i32
        } else if 43 as i32 == '2' as i32 {
            26 as i32
        } else if 43 as i32 == '3' as i32 {
            27 as i32
        } else if 43 as i32 == '4' as i32 {
            28 as i32
        } else if 43 as i32 == '5' as i32 {
            29 as i32
        } else if 43 as i32 == '6' as i32 {
            30 as i32
        } else if 43 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 44 as i32 == 'A' as i32 {
            0 as i32
        } else if 44 as i32 == 'B' as i32 {
            1 as i32
        } else if 44 as i32 == 'C' as i32 {
            2 as i32
        } else if 44 as i32 == 'D' as i32 {
            3 as i32
        } else if 44 as i32 == 'E' as i32 {
            4 as i32
        } else if 44 as i32 == 'F' as i32 {
            5 as i32
        } else if 44 as i32 == 'G' as i32 {
            6 as i32
        } else if 44 as i32 == 'H' as i32 {
            7 as i32
        } else if 44 as i32 == 'I' as i32 {
            8 as i32
        } else if 44 as i32 == 'J' as i32 {
            9 as i32
        } else if 44 as i32 == 'K' as i32 {
            10 as i32
        } else if 44 as i32 == 'L' as i32 {
            11 as i32
        } else if 44 as i32 == 'M' as i32 {
            12 as i32
        } else if 44 as i32 == 'N' as i32 {
            13 as i32
        } else if 44 as i32 == 'O' as i32 {
            14 as i32
        } else if 44 as i32 == 'P' as i32 {
            15 as i32
        } else if 44 as i32 == 'Q' as i32 {
            16 as i32
        } else if 44 as i32 == 'R' as i32 {
            17 as i32
        } else if 44 as i32 == 'S' as i32 {
            18 as i32
        } else if 44 as i32 == 'T' as i32 {
            19 as i32
        } else if 44 as i32 == 'U' as i32 {
            20 as i32
        } else if 44 as i32 == 'V' as i32 {
            21 as i32
        } else if 44 as i32 == 'W' as i32 {
            22 as i32
        } else if 44 as i32 == 'X' as i32 {
            23 as i32
        } else if 44 as i32 == 'Y' as i32 {
            24 as i32
        } else if 44 as i32 == 'Z' as i32 {
            25 as i32
        } else if 44 as i32 == '2' as i32 {
            26 as i32
        } else if 44 as i32 == '3' as i32 {
            27 as i32
        } else if 44 as i32 == '4' as i32 {
            28 as i32
        } else if 44 as i32 == '5' as i32 {
            29 as i32
        } else if 44 as i32 == '6' as i32 {
            30 as i32
        } else if 44 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 45 as i32 == 'A' as i32 {
            0 as i32
        } else if 45 as i32 == 'B' as i32 {
            1 as i32
        } else if 45 as i32 == 'C' as i32 {
            2 as i32
        } else if 45 as i32 == 'D' as i32 {
            3 as i32
        } else if 45 as i32 == 'E' as i32 {
            4 as i32
        } else if 45 as i32 == 'F' as i32 {
            5 as i32
        } else if 45 as i32 == 'G' as i32 {
            6 as i32
        } else if 45 as i32 == 'H' as i32 {
            7 as i32
        } else if 45 as i32 == 'I' as i32 {
            8 as i32
        } else if 45 as i32 == 'J' as i32 {
            9 as i32
        } else if 45 as i32 == 'K' as i32 {
            10 as i32
        } else if 45 as i32 == 'L' as i32 {
            11 as i32
        } else if 45 as i32 == 'M' as i32 {
            12 as i32
        } else if 45 as i32 == 'N' as i32 {
            13 as i32
        } else if 45 as i32 == 'O' as i32 {
            14 as i32
        } else if 45 as i32 == 'P' as i32 {
            15 as i32
        } else if 45 as i32 == 'Q' as i32 {
            16 as i32
        } else if 45 as i32 == 'R' as i32 {
            17 as i32
        } else if 45 as i32 == 'S' as i32 {
            18 as i32
        } else if 45 as i32 == 'T' as i32 {
            19 as i32
        } else if 45 as i32 == 'U' as i32 {
            20 as i32
        } else if 45 as i32 == 'V' as i32 {
            21 as i32
        } else if 45 as i32 == 'W' as i32 {
            22 as i32
        } else if 45 as i32 == 'X' as i32 {
            23 as i32
        } else if 45 as i32 == 'Y' as i32 {
            24 as i32
        } else if 45 as i32 == 'Z' as i32 {
            25 as i32
        } else if 45 as i32 == '2' as i32 {
            26 as i32
        } else if 45 as i32 == '3' as i32 {
            27 as i32
        } else if 45 as i32 == '4' as i32 {
            28 as i32
        } else if 45 as i32 == '5' as i32 {
            29 as i32
        } else if 45 as i32 == '6' as i32 {
            30 as i32
        } else if 45 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 46 as i32 == 'A' as i32 {
            0 as i32
        } else if 46 as i32 == 'B' as i32 {
            1 as i32
        } else if 46 as i32 == 'C' as i32 {
            2 as i32
        } else if 46 as i32 == 'D' as i32 {
            3 as i32
        } else if 46 as i32 == 'E' as i32 {
            4 as i32
        } else if 46 as i32 == 'F' as i32 {
            5 as i32
        } else if 46 as i32 == 'G' as i32 {
            6 as i32
        } else if 46 as i32 == 'H' as i32 {
            7 as i32
        } else if 46 as i32 == 'I' as i32 {
            8 as i32
        } else if 46 as i32 == 'J' as i32 {
            9 as i32
        } else if 46 as i32 == 'K' as i32 {
            10 as i32
        } else if 46 as i32 == 'L' as i32 {
            11 as i32
        } else if 46 as i32 == 'M' as i32 {
            12 as i32
        } else if 46 as i32 == 'N' as i32 {
            13 as i32
        } else if 46 as i32 == 'O' as i32 {
            14 as i32
        } else if 46 as i32 == 'P' as i32 {
            15 as i32
        } else if 46 as i32 == 'Q' as i32 {
            16 as i32
        } else if 46 as i32 == 'R' as i32 {
            17 as i32
        } else if 46 as i32 == 'S' as i32 {
            18 as i32
        } else if 46 as i32 == 'T' as i32 {
            19 as i32
        } else if 46 as i32 == 'U' as i32 {
            20 as i32
        } else if 46 as i32 == 'V' as i32 {
            21 as i32
        } else if 46 as i32 == 'W' as i32 {
            22 as i32
        } else if 46 as i32 == 'X' as i32 {
            23 as i32
        } else if 46 as i32 == 'Y' as i32 {
            24 as i32
        } else if 46 as i32 == 'Z' as i32 {
            25 as i32
        } else if 46 as i32 == '2' as i32 {
            26 as i32
        } else if 46 as i32 == '3' as i32 {
            27 as i32
        } else if 46 as i32 == '4' as i32 {
            28 as i32
        } else if 46 as i32 == '5' as i32 {
            29 as i32
        } else if 46 as i32 == '6' as i32 {
            30 as i32
        } else if 46 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 47 as i32 == 'A' as i32 {
            0 as i32
        } else if 47 as i32 == 'B' as i32 {
            1 as i32
        } else if 47 as i32 == 'C' as i32 {
            2 as i32
        } else if 47 as i32 == 'D' as i32 {
            3 as i32
        } else if 47 as i32 == 'E' as i32 {
            4 as i32
        } else if 47 as i32 == 'F' as i32 {
            5 as i32
        } else if 47 as i32 == 'G' as i32 {
            6 as i32
        } else if 47 as i32 == 'H' as i32 {
            7 as i32
        } else if 47 as i32 == 'I' as i32 {
            8 as i32
        } else if 47 as i32 == 'J' as i32 {
            9 as i32
        } else if 47 as i32 == 'K' as i32 {
            10 as i32
        } else if 47 as i32 == 'L' as i32 {
            11 as i32
        } else if 47 as i32 == 'M' as i32 {
            12 as i32
        } else if 47 as i32 == 'N' as i32 {
            13 as i32
        } else if 47 as i32 == 'O' as i32 {
            14 as i32
        } else if 47 as i32 == 'P' as i32 {
            15 as i32
        } else if 47 as i32 == 'Q' as i32 {
            16 as i32
        } else if 47 as i32 == 'R' as i32 {
            17 as i32
        } else if 47 as i32 == 'S' as i32 {
            18 as i32
        } else if 47 as i32 == 'T' as i32 {
            19 as i32
        } else if 47 as i32 == 'U' as i32 {
            20 as i32
        } else if 47 as i32 == 'V' as i32 {
            21 as i32
        } else if 47 as i32 == 'W' as i32 {
            22 as i32
        } else if 47 as i32 == 'X' as i32 {
            23 as i32
        } else if 47 as i32 == 'Y' as i32 {
            24 as i32
        } else if 47 as i32 == 'Z' as i32 {
            25 as i32
        } else if 47 as i32 == '2' as i32 {
            26 as i32
        } else if 47 as i32 == '3' as i32 {
            27 as i32
        } else if 47 as i32 == '4' as i32 {
            28 as i32
        } else if 47 as i32 == '5' as i32 {
            29 as i32
        } else if 47 as i32 == '6' as i32 {
            30 as i32
        } else if 47 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 48 as i32 == 'A' as i32 {
            0 as i32
        } else if 48 as i32 == 'B' as i32 {
            1 as i32
        } else if 48 as i32 == 'C' as i32 {
            2 as i32
        } else if 48 as i32 == 'D' as i32 {
            3 as i32
        } else if 48 as i32 == 'E' as i32 {
            4 as i32
        } else if 48 as i32 == 'F' as i32 {
            5 as i32
        } else if 48 as i32 == 'G' as i32 {
            6 as i32
        } else if 48 as i32 == 'H' as i32 {
            7 as i32
        } else if 48 as i32 == 'I' as i32 {
            8 as i32
        } else if 48 as i32 == 'J' as i32 {
            9 as i32
        } else if 48 as i32 == 'K' as i32 {
            10 as i32
        } else if 48 as i32 == 'L' as i32 {
            11 as i32
        } else if 48 as i32 == 'M' as i32 {
            12 as i32
        } else if 48 as i32 == 'N' as i32 {
            13 as i32
        } else if 48 as i32 == 'O' as i32 {
            14 as i32
        } else if 48 as i32 == 'P' as i32 {
            15 as i32
        } else if 48 as i32 == 'Q' as i32 {
            16 as i32
        } else if 48 as i32 == 'R' as i32 {
            17 as i32
        } else if 48 as i32 == 'S' as i32 {
            18 as i32
        } else if 48 as i32 == 'T' as i32 {
            19 as i32
        } else if 48 as i32 == 'U' as i32 {
            20 as i32
        } else if 48 as i32 == 'V' as i32 {
            21 as i32
        } else if 48 as i32 == 'W' as i32 {
            22 as i32
        } else if 48 as i32 == 'X' as i32 {
            23 as i32
        } else if 48 as i32 == 'Y' as i32 {
            24 as i32
        } else if 48 as i32 == 'Z' as i32 {
            25 as i32
        } else if 48 as i32 == '2' as i32 {
            26 as i32
        } else if 48 as i32 == '3' as i32 {
            27 as i32
        } else if 48 as i32 == '4' as i32 {
            28 as i32
        } else if 48 as i32 == '5' as i32 {
            29 as i32
        } else if 48 as i32 == '6' as i32 {
            30 as i32
        } else if 48 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 49 as i32 == 'A' as i32 {
            0 as i32
        } else if 49 as i32 == 'B' as i32 {
            1 as i32
        } else if 49 as i32 == 'C' as i32 {
            2 as i32
        } else if 49 as i32 == 'D' as i32 {
            3 as i32
        } else if 49 as i32 == 'E' as i32 {
            4 as i32
        } else if 49 as i32 == 'F' as i32 {
            5 as i32
        } else if 49 as i32 == 'G' as i32 {
            6 as i32
        } else if 49 as i32 == 'H' as i32 {
            7 as i32
        } else if 49 as i32 == 'I' as i32 {
            8 as i32
        } else if 49 as i32 == 'J' as i32 {
            9 as i32
        } else if 49 as i32 == 'K' as i32 {
            10 as i32
        } else if 49 as i32 == 'L' as i32 {
            11 as i32
        } else if 49 as i32 == 'M' as i32 {
            12 as i32
        } else if 49 as i32 == 'N' as i32 {
            13 as i32
        } else if 49 as i32 == 'O' as i32 {
            14 as i32
        } else if 49 as i32 == 'P' as i32 {
            15 as i32
        } else if 49 as i32 == 'Q' as i32 {
            16 as i32
        } else if 49 as i32 == 'R' as i32 {
            17 as i32
        } else if 49 as i32 == 'S' as i32 {
            18 as i32
        } else if 49 as i32 == 'T' as i32 {
            19 as i32
        } else if 49 as i32 == 'U' as i32 {
            20 as i32
        } else if 49 as i32 == 'V' as i32 {
            21 as i32
        } else if 49 as i32 == 'W' as i32 {
            22 as i32
        } else if 49 as i32 == 'X' as i32 {
            23 as i32
        } else if 49 as i32 == 'Y' as i32 {
            24 as i32
        } else if 49 as i32 == 'Z' as i32 {
            25 as i32
        } else if 49 as i32 == '2' as i32 {
            26 as i32
        } else if 49 as i32 == '3' as i32 {
            27 as i32
        } else if 49 as i32 == '4' as i32 {
            28 as i32
        } else if 49 as i32 == '5' as i32 {
            29 as i32
        } else if 49 as i32 == '6' as i32 {
            30 as i32
        } else if 49 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 50 as i32 == 'A' as i32 {
            0 as i32
        } else if 50 as i32 == 'B' as i32 {
            1 as i32
        } else if 50 as i32 == 'C' as i32 {
            2 as i32
        } else if 50 as i32 == 'D' as i32 {
            3 as i32
        } else if 50 as i32 == 'E' as i32 {
            4 as i32
        } else if 50 as i32 == 'F' as i32 {
            5 as i32
        } else if 50 as i32 == 'G' as i32 {
            6 as i32
        } else if 50 as i32 == 'H' as i32 {
            7 as i32
        } else if 50 as i32 == 'I' as i32 {
            8 as i32
        } else if 50 as i32 == 'J' as i32 {
            9 as i32
        } else if 50 as i32 == 'K' as i32 {
            10 as i32
        } else if 50 as i32 == 'L' as i32 {
            11 as i32
        } else if 50 as i32 == 'M' as i32 {
            12 as i32
        } else if 50 as i32 == 'N' as i32 {
            13 as i32
        } else if 50 as i32 == 'O' as i32 {
            14 as i32
        } else if 50 as i32 == 'P' as i32 {
            15 as i32
        } else if 50 as i32 == 'Q' as i32 {
            16 as i32
        } else if 50 as i32 == 'R' as i32 {
            17 as i32
        } else if 50 as i32 == 'S' as i32 {
            18 as i32
        } else if 50 as i32 == 'T' as i32 {
            19 as i32
        } else if 50 as i32 == 'U' as i32 {
            20 as i32
        } else if 50 as i32 == 'V' as i32 {
            21 as i32
        } else if 50 as i32 == 'W' as i32 {
            22 as i32
        } else if 50 as i32 == 'X' as i32 {
            23 as i32
        } else if 50 as i32 == 'Y' as i32 {
            24 as i32
        } else if 50 as i32 == 'Z' as i32 {
            25 as i32
        } else if 50 as i32 == '2' as i32 {
            26 as i32
        } else if 50 as i32 == '3' as i32 {
            27 as i32
        } else if 50 as i32 == '4' as i32 {
            28 as i32
        } else if 50 as i32 == '5' as i32 {
            29 as i32
        } else if 50 as i32 == '6' as i32 {
            30 as i32
        } else if 50 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 51 as i32 == 'A' as i32 {
            0 as i32
        } else if 51 as i32 == 'B' as i32 {
            1 as i32
        } else if 51 as i32 == 'C' as i32 {
            2 as i32
        } else if 51 as i32 == 'D' as i32 {
            3 as i32
        } else if 51 as i32 == 'E' as i32 {
            4 as i32
        } else if 51 as i32 == 'F' as i32 {
            5 as i32
        } else if 51 as i32 == 'G' as i32 {
            6 as i32
        } else if 51 as i32 == 'H' as i32 {
            7 as i32
        } else if 51 as i32 == 'I' as i32 {
            8 as i32
        } else if 51 as i32 == 'J' as i32 {
            9 as i32
        } else if 51 as i32 == 'K' as i32 {
            10 as i32
        } else if 51 as i32 == 'L' as i32 {
            11 as i32
        } else if 51 as i32 == 'M' as i32 {
            12 as i32
        } else if 51 as i32 == 'N' as i32 {
            13 as i32
        } else if 51 as i32 == 'O' as i32 {
            14 as i32
        } else if 51 as i32 == 'P' as i32 {
            15 as i32
        } else if 51 as i32 == 'Q' as i32 {
            16 as i32
        } else if 51 as i32 == 'R' as i32 {
            17 as i32
        } else if 51 as i32 == 'S' as i32 {
            18 as i32
        } else if 51 as i32 == 'T' as i32 {
            19 as i32
        } else if 51 as i32 == 'U' as i32 {
            20 as i32
        } else if 51 as i32 == 'V' as i32 {
            21 as i32
        } else if 51 as i32 == 'W' as i32 {
            22 as i32
        } else if 51 as i32 == 'X' as i32 {
            23 as i32
        } else if 51 as i32 == 'Y' as i32 {
            24 as i32
        } else if 51 as i32 == 'Z' as i32 {
            25 as i32
        } else if 51 as i32 == '2' as i32 {
            26 as i32
        } else if 51 as i32 == '3' as i32 {
            27 as i32
        } else if 51 as i32 == '4' as i32 {
            28 as i32
        } else if 51 as i32 == '5' as i32 {
            29 as i32
        } else if 51 as i32 == '6' as i32 {
            30 as i32
        } else if 51 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 52 as i32 == 'A' as i32 {
            0 as i32
        } else if 52 as i32 == 'B' as i32 {
            1 as i32
        } else if 52 as i32 == 'C' as i32 {
            2 as i32
        } else if 52 as i32 == 'D' as i32 {
            3 as i32
        } else if 52 as i32 == 'E' as i32 {
            4 as i32
        } else if 52 as i32 == 'F' as i32 {
            5 as i32
        } else if 52 as i32 == 'G' as i32 {
            6 as i32
        } else if 52 as i32 == 'H' as i32 {
            7 as i32
        } else if 52 as i32 == 'I' as i32 {
            8 as i32
        } else if 52 as i32 == 'J' as i32 {
            9 as i32
        } else if 52 as i32 == 'K' as i32 {
            10 as i32
        } else if 52 as i32 == 'L' as i32 {
            11 as i32
        } else if 52 as i32 == 'M' as i32 {
            12 as i32
        } else if 52 as i32 == 'N' as i32 {
            13 as i32
        } else if 52 as i32 == 'O' as i32 {
            14 as i32
        } else if 52 as i32 == 'P' as i32 {
            15 as i32
        } else if 52 as i32 == 'Q' as i32 {
            16 as i32
        } else if 52 as i32 == 'R' as i32 {
            17 as i32
        } else if 52 as i32 == 'S' as i32 {
            18 as i32
        } else if 52 as i32 == 'T' as i32 {
            19 as i32
        } else if 52 as i32 == 'U' as i32 {
            20 as i32
        } else if 52 as i32 == 'V' as i32 {
            21 as i32
        } else if 52 as i32 == 'W' as i32 {
            22 as i32
        } else if 52 as i32 == 'X' as i32 {
            23 as i32
        } else if 52 as i32 == 'Y' as i32 {
            24 as i32
        } else if 52 as i32 == 'Z' as i32 {
            25 as i32
        } else if 52 as i32 == '2' as i32 {
            26 as i32
        } else if 52 as i32 == '3' as i32 {
            27 as i32
        } else if 52 as i32 == '4' as i32 {
            28 as i32
        } else if 52 as i32 == '5' as i32 {
            29 as i32
        } else if 52 as i32 == '6' as i32 {
            30 as i32
        } else if 52 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 53 as i32 == 'A' as i32 {
            0 as i32
        } else if 53 as i32 == 'B' as i32 {
            1 as i32
        } else if 53 as i32 == 'C' as i32 {
            2 as i32
        } else if 53 as i32 == 'D' as i32 {
            3 as i32
        } else if 53 as i32 == 'E' as i32 {
            4 as i32
        } else if 53 as i32 == 'F' as i32 {
            5 as i32
        } else if 53 as i32 == 'G' as i32 {
            6 as i32
        } else if 53 as i32 == 'H' as i32 {
            7 as i32
        } else if 53 as i32 == 'I' as i32 {
            8 as i32
        } else if 53 as i32 == 'J' as i32 {
            9 as i32
        } else if 53 as i32 == 'K' as i32 {
            10 as i32
        } else if 53 as i32 == 'L' as i32 {
            11 as i32
        } else if 53 as i32 == 'M' as i32 {
            12 as i32
        } else if 53 as i32 == 'N' as i32 {
            13 as i32
        } else if 53 as i32 == 'O' as i32 {
            14 as i32
        } else if 53 as i32 == 'P' as i32 {
            15 as i32
        } else if 53 as i32 == 'Q' as i32 {
            16 as i32
        } else if 53 as i32 == 'R' as i32 {
            17 as i32
        } else if 53 as i32 == 'S' as i32 {
            18 as i32
        } else if 53 as i32 == 'T' as i32 {
            19 as i32
        } else if 53 as i32 == 'U' as i32 {
            20 as i32
        } else if 53 as i32 == 'V' as i32 {
            21 as i32
        } else if 53 as i32 == 'W' as i32 {
            22 as i32
        } else if 53 as i32 == 'X' as i32 {
            23 as i32
        } else if 53 as i32 == 'Y' as i32 {
            24 as i32
        } else if 53 as i32 == 'Z' as i32 {
            25 as i32
        } else if 53 as i32 == '2' as i32 {
            26 as i32
        } else if 53 as i32 == '3' as i32 {
            27 as i32
        } else if 53 as i32 == '4' as i32 {
            28 as i32
        } else if 53 as i32 == '5' as i32 {
            29 as i32
        } else if 53 as i32 == '6' as i32 {
            30 as i32
        } else if 53 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 54 as i32 == 'A' as i32 {
            0 as i32
        } else if 54 as i32 == 'B' as i32 {
            1 as i32
        } else if 54 as i32 == 'C' as i32 {
            2 as i32
        } else if 54 as i32 == 'D' as i32 {
            3 as i32
        } else if 54 as i32 == 'E' as i32 {
            4 as i32
        } else if 54 as i32 == 'F' as i32 {
            5 as i32
        } else if 54 as i32 == 'G' as i32 {
            6 as i32
        } else if 54 as i32 == 'H' as i32 {
            7 as i32
        } else if 54 as i32 == 'I' as i32 {
            8 as i32
        } else if 54 as i32 == 'J' as i32 {
            9 as i32
        } else if 54 as i32 == 'K' as i32 {
            10 as i32
        } else if 54 as i32 == 'L' as i32 {
            11 as i32
        } else if 54 as i32 == 'M' as i32 {
            12 as i32
        } else if 54 as i32 == 'N' as i32 {
            13 as i32
        } else if 54 as i32 == 'O' as i32 {
            14 as i32
        } else if 54 as i32 == 'P' as i32 {
            15 as i32
        } else if 54 as i32 == 'Q' as i32 {
            16 as i32
        } else if 54 as i32 == 'R' as i32 {
            17 as i32
        } else if 54 as i32 == 'S' as i32 {
            18 as i32
        } else if 54 as i32 == 'T' as i32 {
            19 as i32
        } else if 54 as i32 == 'U' as i32 {
            20 as i32
        } else if 54 as i32 == 'V' as i32 {
            21 as i32
        } else if 54 as i32 == 'W' as i32 {
            22 as i32
        } else if 54 as i32 == 'X' as i32 {
            23 as i32
        } else if 54 as i32 == 'Y' as i32 {
            24 as i32
        } else if 54 as i32 == 'Z' as i32 {
            25 as i32
        } else if 54 as i32 == '2' as i32 {
            26 as i32
        } else if 54 as i32 == '3' as i32 {
            27 as i32
        } else if 54 as i32 == '4' as i32 {
            28 as i32
        } else if 54 as i32 == '5' as i32 {
            29 as i32
        } else if 54 as i32 == '6' as i32 {
            30 as i32
        } else if 54 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 55 as i32 == 'A' as i32 {
            0 as i32
        } else if 55 as i32 == 'B' as i32 {
            1 as i32
        } else if 55 as i32 == 'C' as i32 {
            2 as i32
        } else if 55 as i32 == 'D' as i32 {
            3 as i32
        } else if 55 as i32 == 'E' as i32 {
            4 as i32
        } else if 55 as i32 == 'F' as i32 {
            5 as i32
        } else if 55 as i32 == 'G' as i32 {
            6 as i32
        } else if 55 as i32 == 'H' as i32 {
            7 as i32
        } else if 55 as i32 == 'I' as i32 {
            8 as i32
        } else if 55 as i32 == 'J' as i32 {
            9 as i32
        } else if 55 as i32 == 'K' as i32 {
            10 as i32
        } else if 55 as i32 == 'L' as i32 {
            11 as i32
        } else if 55 as i32 == 'M' as i32 {
            12 as i32
        } else if 55 as i32 == 'N' as i32 {
            13 as i32
        } else if 55 as i32 == 'O' as i32 {
            14 as i32
        } else if 55 as i32 == 'P' as i32 {
            15 as i32
        } else if 55 as i32 == 'Q' as i32 {
            16 as i32
        } else if 55 as i32 == 'R' as i32 {
            17 as i32
        } else if 55 as i32 == 'S' as i32 {
            18 as i32
        } else if 55 as i32 == 'T' as i32 {
            19 as i32
        } else if 55 as i32 == 'U' as i32 {
            20 as i32
        } else if 55 as i32 == 'V' as i32 {
            21 as i32
        } else if 55 as i32 == 'W' as i32 {
            22 as i32
        } else if 55 as i32 == 'X' as i32 {
            23 as i32
        } else if 55 as i32 == 'Y' as i32 {
            24 as i32
        } else if 55 as i32 == 'Z' as i32 {
            25 as i32
        } else if 55 as i32 == '2' as i32 {
            26 as i32
        } else if 55 as i32 == '3' as i32 {
            27 as i32
        } else if 55 as i32 == '4' as i32 {
            28 as i32
        } else if 55 as i32 == '5' as i32 {
            29 as i32
        } else if 55 as i32 == '6' as i32 {
            30 as i32
        } else if 55 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 56 as i32 == 'A' as i32 {
            0 as i32
        } else if 56 as i32 == 'B' as i32 {
            1 as i32
        } else if 56 as i32 == 'C' as i32 {
            2 as i32
        } else if 56 as i32 == 'D' as i32 {
            3 as i32
        } else if 56 as i32 == 'E' as i32 {
            4 as i32
        } else if 56 as i32 == 'F' as i32 {
            5 as i32
        } else if 56 as i32 == 'G' as i32 {
            6 as i32
        } else if 56 as i32 == 'H' as i32 {
            7 as i32
        } else if 56 as i32 == 'I' as i32 {
            8 as i32
        } else if 56 as i32 == 'J' as i32 {
            9 as i32
        } else if 56 as i32 == 'K' as i32 {
            10 as i32
        } else if 56 as i32 == 'L' as i32 {
            11 as i32
        } else if 56 as i32 == 'M' as i32 {
            12 as i32
        } else if 56 as i32 == 'N' as i32 {
            13 as i32
        } else if 56 as i32 == 'O' as i32 {
            14 as i32
        } else if 56 as i32 == 'P' as i32 {
            15 as i32
        } else if 56 as i32 == 'Q' as i32 {
            16 as i32
        } else if 56 as i32 == 'R' as i32 {
            17 as i32
        } else if 56 as i32 == 'S' as i32 {
            18 as i32
        } else if 56 as i32 == 'T' as i32 {
            19 as i32
        } else if 56 as i32 == 'U' as i32 {
            20 as i32
        } else if 56 as i32 == 'V' as i32 {
            21 as i32
        } else if 56 as i32 == 'W' as i32 {
            22 as i32
        } else if 56 as i32 == 'X' as i32 {
            23 as i32
        } else if 56 as i32 == 'Y' as i32 {
            24 as i32
        } else if 56 as i32 == 'Z' as i32 {
            25 as i32
        } else if 56 as i32 == '2' as i32 {
            26 as i32
        } else if 56 as i32 == '3' as i32 {
            27 as i32
        } else if 56 as i32 == '4' as i32 {
            28 as i32
        } else if 56 as i32 == '5' as i32 {
            29 as i32
        } else if 56 as i32 == '6' as i32 {
            30 as i32
        } else if 56 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 57 as i32 == 'A' as i32 {
            0 as i32
        } else if 57 as i32 == 'B' as i32 {
            1 as i32
        } else if 57 as i32 == 'C' as i32 {
            2 as i32
        } else if 57 as i32 == 'D' as i32 {
            3 as i32
        } else if 57 as i32 == 'E' as i32 {
            4 as i32
        } else if 57 as i32 == 'F' as i32 {
            5 as i32
        } else if 57 as i32 == 'G' as i32 {
            6 as i32
        } else if 57 as i32 == 'H' as i32 {
            7 as i32
        } else if 57 as i32 == 'I' as i32 {
            8 as i32
        } else if 57 as i32 == 'J' as i32 {
            9 as i32
        } else if 57 as i32 == 'K' as i32 {
            10 as i32
        } else if 57 as i32 == 'L' as i32 {
            11 as i32
        } else if 57 as i32 == 'M' as i32 {
            12 as i32
        } else if 57 as i32 == 'N' as i32 {
            13 as i32
        } else if 57 as i32 == 'O' as i32 {
            14 as i32
        } else if 57 as i32 == 'P' as i32 {
            15 as i32
        } else if 57 as i32 == 'Q' as i32 {
            16 as i32
        } else if 57 as i32 == 'R' as i32 {
            17 as i32
        } else if 57 as i32 == 'S' as i32 {
            18 as i32
        } else if 57 as i32 == 'T' as i32 {
            19 as i32
        } else if 57 as i32 == 'U' as i32 {
            20 as i32
        } else if 57 as i32 == 'V' as i32 {
            21 as i32
        } else if 57 as i32 == 'W' as i32 {
            22 as i32
        } else if 57 as i32 == 'X' as i32 {
            23 as i32
        } else if 57 as i32 == 'Y' as i32 {
            24 as i32
        } else if 57 as i32 == 'Z' as i32 {
            25 as i32
        } else if 57 as i32 == '2' as i32 {
            26 as i32
        } else if 57 as i32 == '3' as i32 {
            27 as i32
        } else if 57 as i32 == '4' as i32 {
            28 as i32
        } else if 57 as i32 == '5' as i32 {
            29 as i32
        } else if 57 as i32 == '6' as i32 {
            30 as i32
        } else if 57 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 58 as i32 == 'A' as i32 {
            0 as i32
        } else if 58 as i32 == 'B' as i32 {
            1 as i32
        } else if 58 as i32 == 'C' as i32 {
            2 as i32
        } else if 58 as i32 == 'D' as i32 {
            3 as i32
        } else if 58 as i32 == 'E' as i32 {
            4 as i32
        } else if 58 as i32 == 'F' as i32 {
            5 as i32
        } else if 58 as i32 == 'G' as i32 {
            6 as i32
        } else if 58 as i32 == 'H' as i32 {
            7 as i32
        } else if 58 as i32 == 'I' as i32 {
            8 as i32
        } else if 58 as i32 == 'J' as i32 {
            9 as i32
        } else if 58 as i32 == 'K' as i32 {
            10 as i32
        } else if 58 as i32 == 'L' as i32 {
            11 as i32
        } else if 58 as i32 == 'M' as i32 {
            12 as i32
        } else if 58 as i32 == 'N' as i32 {
            13 as i32
        } else if 58 as i32 == 'O' as i32 {
            14 as i32
        } else if 58 as i32 == 'P' as i32 {
            15 as i32
        } else if 58 as i32 == 'Q' as i32 {
            16 as i32
        } else if 58 as i32 == 'R' as i32 {
            17 as i32
        } else if 58 as i32 == 'S' as i32 {
            18 as i32
        } else if 58 as i32 == 'T' as i32 {
            19 as i32
        } else if 58 as i32 == 'U' as i32 {
            20 as i32
        } else if 58 as i32 == 'V' as i32 {
            21 as i32
        } else if 58 as i32 == 'W' as i32 {
            22 as i32
        } else if 58 as i32 == 'X' as i32 {
            23 as i32
        } else if 58 as i32 == 'Y' as i32 {
            24 as i32
        } else if 58 as i32 == 'Z' as i32 {
            25 as i32
        } else if 58 as i32 == '2' as i32 {
            26 as i32
        } else if 58 as i32 == '3' as i32 {
            27 as i32
        } else if 58 as i32 == '4' as i32 {
            28 as i32
        } else if 58 as i32 == '5' as i32 {
            29 as i32
        } else if 58 as i32 == '6' as i32 {
            30 as i32
        } else if 58 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 59 as i32 == 'A' as i32 {
            0 as i32
        } else if 59 as i32 == 'B' as i32 {
            1 as i32
        } else if 59 as i32 == 'C' as i32 {
            2 as i32
        } else if 59 as i32 == 'D' as i32 {
            3 as i32
        } else if 59 as i32 == 'E' as i32 {
            4 as i32
        } else if 59 as i32 == 'F' as i32 {
            5 as i32
        } else if 59 as i32 == 'G' as i32 {
            6 as i32
        } else if 59 as i32 == 'H' as i32 {
            7 as i32
        } else if 59 as i32 == 'I' as i32 {
            8 as i32
        } else if 59 as i32 == 'J' as i32 {
            9 as i32
        } else if 59 as i32 == 'K' as i32 {
            10 as i32
        } else if 59 as i32 == 'L' as i32 {
            11 as i32
        } else if 59 as i32 == 'M' as i32 {
            12 as i32
        } else if 59 as i32 == 'N' as i32 {
            13 as i32
        } else if 59 as i32 == 'O' as i32 {
            14 as i32
        } else if 59 as i32 == 'P' as i32 {
            15 as i32
        } else if 59 as i32 == 'Q' as i32 {
            16 as i32
        } else if 59 as i32 == 'R' as i32 {
            17 as i32
        } else if 59 as i32 == 'S' as i32 {
            18 as i32
        } else if 59 as i32 == 'T' as i32 {
            19 as i32
        } else if 59 as i32 == 'U' as i32 {
            20 as i32
        } else if 59 as i32 == 'V' as i32 {
            21 as i32
        } else if 59 as i32 == 'W' as i32 {
            22 as i32
        } else if 59 as i32 == 'X' as i32 {
            23 as i32
        } else if 59 as i32 == 'Y' as i32 {
            24 as i32
        } else if 59 as i32 == 'Z' as i32 {
            25 as i32
        } else if 59 as i32 == '2' as i32 {
            26 as i32
        } else if 59 as i32 == '3' as i32 {
            27 as i32
        } else if 59 as i32 == '4' as i32 {
            28 as i32
        } else if 59 as i32 == '5' as i32 {
            29 as i32
        } else if 59 as i32 == '6' as i32 {
            30 as i32
        } else if 59 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 60 as i32 == 'A' as i32 {
            0 as i32
        } else if 60 as i32 == 'B' as i32 {
            1 as i32
        } else if 60 as i32 == 'C' as i32 {
            2 as i32
        } else if 60 as i32 == 'D' as i32 {
            3 as i32
        } else if 60 as i32 == 'E' as i32 {
            4 as i32
        } else if 60 as i32 == 'F' as i32 {
            5 as i32
        } else if 60 as i32 == 'G' as i32 {
            6 as i32
        } else if 60 as i32 == 'H' as i32 {
            7 as i32
        } else if 60 as i32 == 'I' as i32 {
            8 as i32
        } else if 60 as i32 == 'J' as i32 {
            9 as i32
        } else if 60 as i32 == 'K' as i32 {
            10 as i32
        } else if 60 as i32 == 'L' as i32 {
            11 as i32
        } else if 60 as i32 == 'M' as i32 {
            12 as i32
        } else if 60 as i32 == 'N' as i32 {
            13 as i32
        } else if 60 as i32 == 'O' as i32 {
            14 as i32
        } else if 60 as i32 == 'P' as i32 {
            15 as i32
        } else if 60 as i32 == 'Q' as i32 {
            16 as i32
        } else if 60 as i32 == 'R' as i32 {
            17 as i32
        } else if 60 as i32 == 'S' as i32 {
            18 as i32
        } else if 60 as i32 == 'T' as i32 {
            19 as i32
        } else if 60 as i32 == 'U' as i32 {
            20 as i32
        } else if 60 as i32 == 'V' as i32 {
            21 as i32
        } else if 60 as i32 == 'W' as i32 {
            22 as i32
        } else if 60 as i32 == 'X' as i32 {
            23 as i32
        } else if 60 as i32 == 'Y' as i32 {
            24 as i32
        } else if 60 as i32 == 'Z' as i32 {
            25 as i32
        } else if 60 as i32 == '2' as i32 {
            26 as i32
        } else if 60 as i32 == '3' as i32 {
            27 as i32
        } else if 60 as i32 == '4' as i32 {
            28 as i32
        } else if 60 as i32 == '5' as i32 {
            29 as i32
        } else if 60 as i32 == '6' as i32 {
            30 as i32
        } else if 60 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 61 as i32 == 'A' as i32 {
            0 as i32
        } else if 61 as i32 == 'B' as i32 {
            1 as i32
        } else if 61 as i32 == 'C' as i32 {
            2 as i32
        } else if 61 as i32 == 'D' as i32 {
            3 as i32
        } else if 61 as i32 == 'E' as i32 {
            4 as i32
        } else if 61 as i32 == 'F' as i32 {
            5 as i32
        } else if 61 as i32 == 'G' as i32 {
            6 as i32
        } else if 61 as i32 == 'H' as i32 {
            7 as i32
        } else if 61 as i32 == 'I' as i32 {
            8 as i32
        } else if 61 as i32 == 'J' as i32 {
            9 as i32
        } else if 61 as i32 == 'K' as i32 {
            10 as i32
        } else if 61 as i32 == 'L' as i32 {
            11 as i32
        } else if 61 as i32 == 'M' as i32 {
            12 as i32
        } else if 61 as i32 == 'N' as i32 {
            13 as i32
        } else if 61 as i32 == 'O' as i32 {
            14 as i32
        } else if 61 as i32 == 'P' as i32 {
            15 as i32
        } else if 61 as i32 == 'Q' as i32 {
            16 as i32
        } else if 61 as i32 == 'R' as i32 {
            17 as i32
        } else if 61 as i32 == 'S' as i32 {
            18 as i32
        } else if 61 as i32 == 'T' as i32 {
            19 as i32
        } else if 61 as i32 == 'U' as i32 {
            20 as i32
        } else if 61 as i32 == 'V' as i32 {
            21 as i32
        } else if 61 as i32 == 'W' as i32 {
            22 as i32
        } else if 61 as i32 == 'X' as i32 {
            23 as i32
        } else if 61 as i32 == 'Y' as i32 {
            24 as i32
        } else if 61 as i32 == 'Z' as i32 {
            25 as i32
        } else if 61 as i32 == '2' as i32 {
            26 as i32
        } else if 61 as i32 == '3' as i32 {
            27 as i32
        } else if 61 as i32 == '4' as i32 {
            28 as i32
        } else if 61 as i32 == '5' as i32 {
            29 as i32
        } else if 61 as i32 == '6' as i32 {
            30 as i32
        } else if 61 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 62 as i32 == 'A' as i32 {
            0 as i32
        } else if 62 as i32 == 'B' as i32 {
            1 as i32
        } else if 62 as i32 == 'C' as i32 {
            2 as i32
        } else if 62 as i32 == 'D' as i32 {
            3 as i32
        } else if 62 as i32 == 'E' as i32 {
            4 as i32
        } else if 62 as i32 == 'F' as i32 {
            5 as i32
        } else if 62 as i32 == 'G' as i32 {
            6 as i32
        } else if 62 as i32 == 'H' as i32 {
            7 as i32
        } else if 62 as i32 == 'I' as i32 {
            8 as i32
        } else if 62 as i32 == 'J' as i32 {
            9 as i32
        } else if 62 as i32 == 'K' as i32 {
            10 as i32
        } else if 62 as i32 == 'L' as i32 {
            11 as i32
        } else if 62 as i32 == 'M' as i32 {
            12 as i32
        } else if 62 as i32 == 'N' as i32 {
            13 as i32
        } else if 62 as i32 == 'O' as i32 {
            14 as i32
        } else if 62 as i32 == 'P' as i32 {
            15 as i32
        } else if 62 as i32 == 'Q' as i32 {
            16 as i32
        } else if 62 as i32 == 'R' as i32 {
            17 as i32
        } else if 62 as i32 == 'S' as i32 {
            18 as i32
        } else if 62 as i32 == 'T' as i32 {
            19 as i32
        } else if 62 as i32 == 'U' as i32 {
            20 as i32
        } else if 62 as i32 == 'V' as i32 {
            21 as i32
        } else if 62 as i32 == 'W' as i32 {
            22 as i32
        } else if 62 as i32 == 'X' as i32 {
            23 as i32
        } else if 62 as i32 == 'Y' as i32 {
            24 as i32
        } else if 62 as i32 == 'Z' as i32 {
            25 as i32
        } else if 62 as i32 == '2' as i32 {
            26 as i32
        } else if 62 as i32 == '3' as i32 {
            27 as i32
        } else if 62 as i32 == '4' as i32 {
            28 as i32
        } else if 62 as i32 == '5' as i32 {
            29 as i32
        } else if 62 as i32 == '6' as i32 {
            30 as i32
        } else if 62 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 63 as i32 == 'A' as i32 {
            0 as i32
        } else if 63 as i32 == 'B' as i32 {
            1 as i32
        } else if 63 as i32 == 'C' as i32 {
            2 as i32
        } else if 63 as i32 == 'D' as i32 {
            3 as i32
        } else if 63 as i32 == 'E' as i32 {
            4 as i32
        } else if 63 as i32 == 'F' as i32 {
            5 as i32
        } else if 63 as i32 == 'G' as i32 {
            6 as i32
        } else if 63 as i32 == 'H' as i32 {
            7 as i32
        } else if 63 as i32 == 'I' as i32 {
            8 as i32
        } else if 63 as i32 == 'J' as i32 {
            9 as i32
        } else if 63 as i32 == 'K' as i32 {
            10 as i32
        } else if 63 as i32 == 'L' as i32 {
            11 as i32
        } else if 63 as i32 == 'M' as i32 {
            12 as i32
        } else if 63 as i32 == 'N' as i32 {
            13 as i32
        } else if 63 as i32 == 'O' as i32 {
            14 as i32
        } else if 63 as i32 == 'P' as i32 {
            15 as i32
        } else if 63 as i32 == 'Q' as i32 {
            16 as i32
        } else if 63 as i32 == 'R' as i32 {
            17 as i32
        } else if 63 as i32 == 'S' as i32 {
            18 as i32
        } else if 63 as i32 == 'T' as i32 {
            19 as i32
        } else if 63 as i32 == 'U' as i32 {
            20 as i32
        } else if 63 as i32 == 'V' as i32 {
            21 as i32
        } else if 63 as i32 == 'W' as i32 {
            22 as i32
        } else if 63 as i32 == 'X' as i32 {
            23 as i32
        } else if 63 as i32 == 'Y' as i32 {
            24 as i32
        } else if 63 as i32 == 'Z' as i32 {
            25 as i32
        } else if 63 as i32 == '2' as i32 {
            26 as i32
        } else if 63 as i32 == '3' as i32 {
            27 as i32
        } else if 63 as i32 == '4' as i32 {
            28 as i32
        } else if 63 as i32 == '5' as i32 {
            29 as i32
        } else if 63 as i32 == '6' as i32 {
            30 as i32
        } else if 63 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 32 as i32 == 'A' as i32 {
            0 as i32
        } else if 32 as i32 == 'B' as i32 {
            1 as i32
        } else if 32 as i32 == 'C' as i32 {
            2 as i32
        } else if 32 as i32 == 'D' as i32 {
            3 as i32
        } else if 32 as i32 == 'E' as i32 {
            4 as i32
        } else if 32 as i32 == 'F' as i32 {
            5 as i32
        } else if 32 as i32 == 'G' as i32 {
            6 as i32
        } else if 32 as i32 == 'H' as i32 {
            7 as i32
        } else if 32 as i32 == 'I' as i32 {
            8 as i32
        } else if 32 as i32 == 'J' as i32 {
            9 as i32
        } else if 32 as i32 == 'K' as i32 {
            10 as i32
        } else if 32 as i32 == 'L' as i32 {
            11 as i32
        } else if 32 as i32 == 'M' as i32 {
            12 as i32
        } else if 32 as i32 == 'N' as i32 {
            13 as i32
        } else if 32 as i32 == 'O' as i32 {
            14 as i32
        } else if 32 as i32 == 'P' as i32 {
            15 as i32
        } else if 32 as i32 == 'Q' as i32 {
            16 as i32
        } else if 32 as i32 == 'R' as i32 {
            17 as i32
        } else if 32 as i32 == 'S' as i32 {
            18 as i32
        } else if 32 as i32 == 'T' as i32 {
            19 as i32
        } else if 32 as i32 == 'U' as i32 {
            20 as i32
        } else if 32 as i32 == 'V' as i32 {
            21 as i32
        } else if 32 as i32 == 'W' as i32 {
            22 as i32
        } else if 32 as i32 == 'X' as i32 {
            23 as i32
        } else if 32 as i32 == 'Y' as i32 {
            24 as i32
        } else if 32 as i32 == 'Z' as i32 {
            25 as i32
        } else if 32 as i32 == '2' as i32 {
            26 as i32
        } else if 32 as i32 == '3' as i32 {
            27 as i32
        } else if 32 as i32 == '4' as i32 {
            28 as i32
        } else if 32 as i32 == '5' as i32 {
            29 as i32
        } else if 32 as i32 == '6' as i32 {
            30 as i32
        } else if 32 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 65 as i32 == 'A' as i32 {
            0 as i32
        } else if 65 as i32 == 'B' as i32 {
            1 as i32
        } else if 65 as i32 == 'C' as i32 {
            2 as i32
        } else if 65 as i32 == 'D' as i32 {
            3 as i32
        } else if 65 as i32 == 'E' as i32 {
            4 as i32
        } else if 65 as i32 == 'F' as i32 {
            5 as i32
        } else if 65 as i32 == 'G' as i32 {
            6 as i32
        } else if 65 as i32 == 'H' as i32 {
            7 as i32
        } else if 65 as i32 == 'I' as i32 {
            8 as i32
        } else if 65 as i32 == 'J' as i32 {
            9 as i32
        } else if 65 as i32 == 'K' as i32 {
            10 as i32
        } else if 65 as i32 == 'L' as i32 {
            11 as i32
        } else if 65 as i32 == 'M' as i32 {
            12 as i32
        } else if 65 as i32 == 'N' as i32 {
            13 as i32
        } else if 65 as i32 == 'O' as i32 {
            14 as i32
        } else if 65 as i32 == 'P' as i32 {
            15 as i32
        } else if 65 as i32 == 'Q' as i32 {
            16 as i32
        } else if 65 as i32 == 'R' as i32 {
            17 as i32
        } else if 65 as i32 == 'S' as i32 {
            18 as i32
        } else if 65 as i32 == 'T' as i32 {
            19 as i32
        } else if 65 as i32 == 'U' as i32 {
            20 as i32
        } else if 65 as i32 == 'V' as i32 {
            21 as i32
        } else if 65 as i32 == 'W' as i32 {
            22 as i32
        } else if 65 as i32 == 'X' as i32 {
            23 as i32
        } else if 65 as i32 == 'Y' as i32 {
            24 as i32
        } else if 65 as i32 == 'Z' as i32 {
            25 as i32
        } else if 65 as i32 == '2' as i32 {
            26 as i32
        } else if 65 as i32 == '3' as i32 {
            27 as i32
        } else if 65 as i32 == '4' as i32 {
            28 as i32
        } else if 65 as i32 == '5' as i32 {
            29 as i32
        } else if 65 as i32 == '6' as i32 {
            30 as i32
        } else if 65 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 66 as i32 == 'A' as i32 {
            0 as i32
        } else if 66 as i32 == 'B' as i32 {
            1 as i32
        } else if 66 as i32 == 'C' as i32 {
            2 as i32
        } else if 66 as i32 == 'D' as i32 {
            3 as i32
        } else if 66 as i32 == 'E' as i32 {
            4 as i32
        } else if 66 as i32 == 'F' as i32 {
            5 as i32
        } else if 66 as i32 == 'G' as i32 {
            6 as i32
        } else if 66 as i32 == 'H' as i32 {
            7 as i32
        } else if 66 as i32 == 'I' as i32 {
            8 as i32
        } else if 66 as i32 == 'J' as i32 {
            9 as i32
        } else if 66 as i32 == 'K' as i32 {
            10 as i32
        } else if 66 as i32 == 'L' as i32 {
            11 as i32
        } else if 66 as i32 == 'M' as i32 {
            12 as i32
        } else if 66 as i32 == 'N' as i32 {
            13 as i32
        } else if 66 as i32 == 'O' as i32 {
            14 as i32
        } else if 66 as i32 == 'P' as i32 {
            15 as i32
        } else if 66 as i32 == 'Q' as i32 {
            16 as i32
        } else if 66 as i32 == 'R' as i32 {
            17 as i32
        } else if 66 as i32 == 'S' as i32 {
            18 as i32
        } else if 66 as i32 == 'T' as i32 {
            19 as i32
        } else if 66 as i32 == 'U' as i32 {
            20 as i32
        } else if 66 as i32 == 'V' as i32 {
            21 as i32
        } else if 66 as i32 == 'W' as i32 {
            22 as i32
        } else if 66 as i32 == 'X' as i32 {
            23 as i32
        } else if 66 as i32 == 'Y' as i32 {
            24 as i32
        } else if 66 as i32 == 'Z' as i32 {
            25 as i32
        } else if 66 as i32 == '2' as i32 {
            26 as i32
        } else if 66 as i32 == '3' as i32 {
            27 as i32
        } else if 66 as i32 == '4' as i32 {
            28 as i32
        } else if 66 as i32 == '5' as i32 {
            29 as i32
        } else if 66 as i32 == '6' as i32 {
            30 as i32
        } else if 66 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 67 as i32 == 'A' as i32 {
            0 as i32
        } else if 67 as i32 == 'B' as i32 {
            1 as i32
        } else if 67 as i32 == 'C' as i32 {
            2 as i32
        } else if 67 as i32 == 'D' as i32 {
            3 as i32
        } else if 67 as i32 == 'E' as i32 {
            4 as i32
        } else if 67 as i32 == 'F' as i32 {
            5 as i32
        } else if 67 as i32 == 'G' as i32 {
            6 as i32
        } else if 67 as i32 == 'H' as i32 {
            7 as i32
        } else if 67 as i32 == 'I' as i32 {
            8 as i32
        } else if 67 as i32 == 'J' as i32 {
            9 as i32
        } else if 67 as i32 == 'K' as i32 {
            10 as i32
        } else if 67 as i32 == 'L' as i32 {
            11 as i32
        } else if 67 as i32 == 'M' as i32 {
            12 as i32
        } else if 67 as i32 == 'N' as i32 {
            13 as i32
        } else if 67 as i32 == 'O' as i32 {
            14 as i32
        } else if 67 as i32 == 'P' as i32 {
            15 as i32
        } else if 67 as i32 == 'Q' as i32 {
            16 as i32
        } else if 67 as i32 == 'R' as i32 {
            17 as i32
        } else if 67 as i32 == 'S' as i32 {
            18 as i32
        } else if 67 as i32 == 'T' as i32 {
            19 as i32
        } else if 67 as i32 == 'U' as i32 {
            20 as i32
        } else if 67 as i32 == 'V' as i32 {
            21 as i32
        } else if 67 as i32 == 'W' as i32 {
            22 as i32
        } else if 67 as i32 == 'X' as i32 {
            23 as i32
        } else if 67 as i32 == 'Y' as i32 {
            24 as i32
        } else if 67 as i32 == 'Z' as i32 {
            25 as i32
        } else if 67 as i32 == '2' as i32 {
            26 as i32
        } else if 67 as i32 == '3' as i32 {
            27 as i32
        } else if 67 as i32 == '4' as i32 {
            28 as i32
        } else if 67 as i32 == '5' as i32 {
            29 as i32
        } else if 67 as i32 == '6' as i32 {
            30 as i32
        } else if 67 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 68 as i32 == 'A' as i32 {
            0 as i32
        } else if 68 as i32 == 'B' as i32 {
            1 as i32
        } else if 68 as i32 == 'C' as i32 {
            2 as i32
        } else if 68 as i32 == 'D' as i32 {
            3 as i32
        } else if 68 as i32 == 'E' as i32 {
            4 as i32
        } else if 68 as i32 == 'F' as i32 {
            5 as i32
        } else if 68 as i32 == 'G' as i32 {
            6 as i32
        } else if 68 as i32 == 'H' as i32 {
            7 as i32
        } else if 68 as i32 == 'I' as i32 {
            8 as i32
        } else if 68 as i32 == 'J' as i32 {
            9 as i32
        } else if 68 as i32 == 'K' as i32 {
            10 as i32
        } else if 68 as i32 == 'L' as i32 {
            11 as i32
        } else if 68 as i32 == 'M' as i32 {
            12 as i32
        } else if 68 as i32 == 'N' as i32 {
            13 as i32
        } else if 68 as i32 == 'O' as i32 {
            14 as i32
        } else if 68 as i32 == 'P' as i32 {
            15 as i32
        } else if 68 as i32 == 'Q' as i32 {
            16 as i32
        } else if 68 as i32 == 'R' as i32 {
            17 as i32
        } else if 68 as i32 == 'S' as i32 {
            18 as i32
        } else if 68 as i32 == 'T' as i32 {
            19 as i32
        } else if 68 as i32 == 'U' as i32 {
            20 as i32
        } else if 68 as i32 == 'V' as i32 {
            21 as i32
        } else if 68 as i32 == 'W' as i32 {
            22 as i32
        } else if 68 as i32 == 'X' as i32 {
            23 as i32
        } else if 68 as i32 == 'Y' as i32 {
            24 as i32
        } else if 68 as i32 == 'Z' as i32 {
            25 as i32
        } else if 68 as i32 == '2' as i32 {
            26 as i32
        } else if 68 as i32 == '3' as i32 {
            27 as i32
        } else if 68 as i32 == '4' as i32 {
            28 as i32
        } else if 68 as i32 == '5' as i32 {
            29 as i32
        } else if 68 as i32 == '6' as i32 {
            30 as i32
        } else if 68 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 69 as i32 == 'A' as i32 {
            0 as i32
        } else if 69 as i32 == 'B' as i32 {
            1 as i32
        } else if 69 as i32 == 'C' as i32 {
            2 as i32
        } else if 69 as i32 == 'D' as i32 {
            3 as i32
        } else if 69 as i32 == 'E' as i32 {
            4 as i32
        } else if 69 as i32 == 'F' as i32 {
            5 as i32
        } else if 69 as i32 == 'G' as i32 {
            6 as i32
        } else if 69 as i32 == 'H' as i32 {
            7 as i32
        } else if 69 as i32 == 'I' as i32 {
            8 as i32
        } else if 69 as i32 == 'J' as i32 {
            9 as i32
        } else if 69 as i32 == 'K' as i32 {
            10 as i32
        } else if 69 as i32 == 'L' as i32 {
            11 as i32
        } else if 69 as i32 == 'M' as i32 {
            12 as i32
        } else if 69 as i32 == 'N' as i32 {
            13 as i32
        } else if 69 as i32 == 'O' as i32 {
            14 as i32
        } else if 69 as i32 == 'P' as i32 {
            15 as i32
        } else if 69 as i32 == 'Q' as i32 {
            16 as i32
        } else if 69 as i32 == 'R' as i32 {
            17 as i32
        } else if 69 as i32 == 'S' as i32 {
            18 as i32
        } else if 69 as i32 == 'T' as i32 {
            19 as i32
        } else if 69 as i32 == 'U' as i32 {
            20 as i32
        } else if 69 as i32 == 'V' as i32 {
            21 as i32
        } else if 69 as i32 == 'W' as i32 {
            22 as i32
        } else if 69 as i32 == 'X' as i32 {
            23 as i32
        } else if 69 as i32 == 'Y' as i32 {
            24 as i32
        } else if 69 as i32 == 'Z' as i32 {
            25 as i32
        } else if 69 as i32 == '2' as i32 {
            26 as i32
        } else if 69 as i32 == '3' as i32 {
            27 as i32
        } else if 69 as i32 == '4' as i32 {
            28 as i32
        } else if 69 as i32 == '5' as i32 {
            29 as i32
        } else if 69 as i32 == '6' as i32 {
            30 as i32
        } else if 69 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 70 as i32 == 'A' as i32 {
            0 as i32
        } else if 70 as i32 == 'B' as i32 {
            1 as i32
        } else if 70 as i32 == 'C' as i32 {
            2 as i32
        } else if 70 as i32 == 'D' as i32 {
            3 as i32
        } else if 70 as i32 == 'E' as i32 {
            4 as i32
        } else if 70 as i32 == 'F' as i32 {
            5 as i32
        } else if 70 as i32 == 'G' as i32 {
            6 as i32
        } else if 70 as i32 == 'H' as i32 {
            7 as i32
        } else if 70 as i32 == 'I' as i32 {
            8 as i32
        } else if 70 as i32 == 'J' as i32 {
            9 as i32
        } else if 70 as i32 == 'K' as i32 {
            10 as i32
        } else if 70 as i32 == 'L' as i32 {
            11 as i32
        } else if 70 as i32 == 'M' as i32 {
            12 as i32
        } else if 70 as i32 == 'N' as i32 {
            13 as i32
        } else if 70 as i32 == 'O' as i32 {
            14 as i32
        } else if 70 as i32 == 'P' as i32 {
            15 as i32
        } else if 70 as i32 == 'Q' as i32 {
            16 as i32
        } else if 70 as i32 == 'R' as i32 {
            17 as i32
        } else if 70 as i32 == 'S' as i32 {
            18 as i32
        } else if 70 as i32 == 'T' as i32 {
            19 as i32
        } else if 70 as i32 == 'U' as i32 {
            20 as i32
        } else if 70 as i32 == 'V' as i32 {
            21 as i32
        } else if 70 as i32 == 'W' as i32 {
            22 as i32
        } else if 70 as i32 == 'X' as i32 {
            23 as i32
        } else if 70 as i32 == 'Y' as i32 {
            24 as i32
        } else if 70 as i32 == 'Z' as i32 {
            25 as i32
        } else if 70 as i32 == '2' as i32 {
            26 as i32
        } else if 70 as i32 == '3' as i32 {
            27 as i32
        } else if 70 as i32 == '4' as i32 {
            28 as i32
        } else if 70 as i32 == '5' as i32 {
            29 as i32
        } else if 70 as i32 == '6' as i32 {
            30 as i32
        } else if 70 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 71 as i32 == 'A' as i32 {
            0 as i32
        } else if 71 as i32 == 'B' as i32 {
            1 as i32
        } else if 71 as i32 == 'C' as i32 {
            2 as i32
        } else if 71 as i32 == 'D' as i32 {
            3 as i32
        } else if 71 as i32 == 'E' as i32 {
            4 as i32
        } else if 71 as i32 == 'F' as i32 {
            5 as i32
        } else if 71 as i32 == 'G' as i32 {
            6 as i32
        } else if 71 as i32 == 'H' as i32 {
            7 as i32
        } else if 71 as i32 == 'I' as i32 {
            8 as i32
        } else if 71 as i32 == 'J' as i32 {
            9 as i32
        } else if 71 as i32 == 'K' as i32 {
            10 as i32
        } else if 71 as i32 == 'L' as i32 {
            11 as i32
        } else if 71 as i32 == 'M' as i32 {
            12 as i32
        } else if 71 as i32 == 'N' as i32 {
            13 as i32
        } else if 71 as i32 == 'O' as i32 {
            14 as i32
        } else if 71 as i32 == 'P' as i32 {
            15 as i32
        } else if 71 as i32 == 'Q' as i32 {
            16 as i32
        } else if 71 as i32 == 'R' as i32 {
            17 as i32
        } else if 71 as i32 == 'S' as i32 {
            18 as i32
        } else if 71 as i32 == 'T' as i32 {
            19 as i32
        } else if 71 as i32 == 'U' as i32 {
            20 as i32
        } else if 71 as i32 == 'V' as i32 {
            21 as i32
        } else if 71 as i32 == 'W' as i32 {
            22 as i32
        } else if 71 as i32 == 'X' as i32 {
            23 as i32
        } else if 71 as i32 == 'Y' as i32 {
            24 as i32
        } else if 71 as i32 == 'Z' as i32 {
            25 as i32
        } else if 71 as i32 == '2' as i32 {
            26 as i32
        } else if 71 as i32 == '3' as i32 {
            27 as i32
        } else if 71 as i32 == '4' as i32 {
            28 as i32
        } else if 71 as i32 == '5' as i32 {
            29 as i32
        } else if 71 as i32 == '6' as i32 {
            30 as i32
        } else if 71 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 72 as i32 == 'A' as i32 {
            0 as i32
        } else if 72 as i32 == 'B' as i32 {
            1 as i32
        } else if 72 as i32 == 'C' as i32 {
            2 as i32
        } else if 72 as i32 == 'D' as i32 {
            3 as i32
        } else if 72 as i32 == 'E' as i32 {
            4 as i32
        } else if 72 as i32 == 'F' as i32 {
            5 as i32
        } else if 72 as i32 == 'G' as i32 {
            6 as i32
        } else if 72 as i32 == 'H' as i32 {
            7 as i32
        } else if 72 as i32 == 'I' as i32 {
            8 as i32
        } else if 72 as i32 == 'J' as i32 {
            9 as i32
        } else if 72 as i32 == 'K' as i32 {
            10 as i32
        } else if 72 as i32 == 'L' as i32 {
            11 as i32
        } else if 72 as i32 == 'M' as i32 {
            12 as i32
        } else if 72 as i32 == 'N' as i32 {
            13 as i32
        } else if 72 as i32 == 'O' as i32 {
            14 as i32
        } else if 72 as i32 == 'P' as i32 {
            15 as i32
        } else if 72 as i32 == 'Q' as i32 {
            16 as i32
        } else if 72 as i32 == 'R' as i32 {
            17 as i32
        } else if 72 as i32 == 'S' as i32 {
            18 as i32
        } else if 72 as i32 == 'T' as i32 {
            19 as i32
        } else if 72 as i32 == 'U' as i32 {
            20 as i32
        } else if 72 as i32 == 'V' as i32 {
            21 as i32
        } else if 72 as i32 == 'W' as i32 {
            22 as i32
        } else if 72 as i32 == 'X' as i32 {
            23 as i32
        } else if 72 as i32 == 'Y' as i32 {
            24 as i32
        } else if 72 as i32 == 'Z' as i32 {
            25 as i32
        } else if 72 as i32 == '2' as i32 {
            26 as i32
        } else if 72 as i32 == '3' as i32 {
            27 as i32
        } else if 72 as i32 == '4' as i32 {
            28 as i32
        } else if 72 as i32 == '5' as i32 {
            29 as i32
        } else if 72 as i32 == '6' as i32 {
            30 as i32
        } else if 72 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 73 as i32 == 'A' as i32 {
            0 as i32
        } else if 73 as i32 == 'B' as i32 {
            1 as i32
        } else if 73 as i32 == 'C' as i32 {
            2 as i32
        } else if 73 as i32 == 'D' as i32 {
            3 as i32
        } else if 73 as i32 == 'E' as i32 {
            4 as i32
        } else if 73 as i32 == 'F' as i32 {
            5 as i32
        } else if 73 as i32 == 'G' as i32 {
            6 as i32
        } else if 73 as i32 == 'H' as i32 {
            7 as i32
        } else if 73 as i32 == 'I' as i32 {
            8 as i32
        } else if 73 as i32 == 'J' as i32 {
            9 as i32
        } else if 73 as i32 == 'K' as i32 {
            10 as i32
        } else if 73 as i32 == 'L' as i32 {
            11 as i32
        } else if 73 as i32 == 'M' as i32 {
            12 as i32
        } else if 73 as i32 == 'N' as i32 {
            13 as i32
        } else if 73 as i32 == 'O' as i32 {
            14 as i32
        } else if 73 as i32 == 'P' as i32 {
            15 as i32
        } else if 73 as i32 == 'Q' as i32 {
            16 as i32
        } else if 73 as i32 == 'R' as i32 {
            17 as i32
        } else if 73 as i32 == 'S' as i32 {
            18 as i32
        } else if 73 as i32 == 'T' as i32 {
            19 as i32
        } else if 73 as i32 == 'U' as i32 {
            20 as i32
        } else if 73 as i32 == 'V' as i32 {
            21 as i32
        } else if 73 as i32 == 'W' as i32 {
            22 as i32
        } else if 73 as i32 == 'X' as i32 {
            23 as i32
        } else if 73 as i32 == 'Y' as i32 {
            24 as i32
        } else if 73 as i32 == 'Z' as i32 {
            25 as i32
        } else if 73 as i32 == '2' as i32 {
            26 as i32
        } else if 73 as i32 == '3' as i32 {
            27 as i32
        } else if 73 as i32 == '4' as i32 {
            28 as i32
        } else if 73 as i32 == '5' as i32 {
            29 as i32
        } else if 73 as i32 == '6' as i32 {
            30 as i32
        } else if 73 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 74 as i32 == 'A' as i32 {
            0 as i32
        } else if 74 as i32 == 'B' as i32 {
            1 as i32
        } else if 74 as i32 == 'C' as i32 {
            2 as i32
        } else if 74 as i32 == 'D' as i32 {
            3 as i32
        } else if 74 as i32 == 'E' as i32 {
            4 as i32
        } else if 74 as i32 == 'F' as i32 {
            5 as i32
        } else if 74 as i32 == 'G' as i32 {
            6 as i32
        } else if 74 as i32 == 'H' as i32 {
            7 as i32
        } else if 74 as i32 == 'I' as i32 {
            8 as i32
        } else if 74 as i32 == 'J' as i32 {
            9 as i32
        } else if 74 as i32 == 'K' as i32 {
            10 as i32
        } else if 74 as i32 == 'L' as i32 {
            11 as i32
        } else if 74 as i32 == 'M' as i32 {
            12 as i32
        } else if 74 as i32 == 'N' as i32 {
            13 as i32
        } else if 74 as i32 == 'O' as i32 {
            14 as i32
        } else if 74 as i32 == 'P' as i32 {
            15 as i32
        } else if 74 as i32 == 'Q' as i32 {
            16 as i32
        } else if 74 as i32 == 'R' as i32 {
            17 as i32
        } else if 74 as i32 == 'S' as i32 {
            18 as i32
        } else if 74 as i32 == 'T' as i32 {
            19 as i32
        } else if 74 as i32 == 'U' as i32 {
            20 as i32
        } else if 74 as i32 == 'V' as i32 {
            21 as i32
        } else if 74 as i32 == 'W' as i32 {
            22 as i32
        } else if 74 as i32 == 'X' as i32 {
            23 as i32
        } else if 74 as i32 == 'Y' as i32 {
            24 as i32
        } else if 74 as i32 == 'Z' as i32 {
            25 as i32
        } else if 74 as i32 == '2' as i32 {
            26 as i32
        } else if 74 as i32 == '3' as i32 {
            27 as i32
        } else if 74 as i32 == '4' as i32 {
            28 as i32
        } else if 74 as i32 == '5' as i32 {
            29 as i32
        } else if 74 as i32 == '6' as i32 {
            30 as i32
        } else if 74 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 75 as i32 == 'A' as i32 {
            0 as i32
        } else if 75 as i32 == 'B' as i32 {
            1 as i32
        } else if 75 as i32 == 'C' as i32 {
            2 as i32
        } else if 75 as i32 == 'D' as i32 {
            3 as i32
        } else if 75 as i32 == 'E' as i32 {
            4 as i32
        } else if 75 as i32 == 'F' as i32 {
            5 as i32
        } else if 75 as i32 == 'G' as i32 {
            6 as i32
        } else if 75 as i32 == 'H' as i32 {
            7 as i32
        } else if 75 as i32 == 'I' as i32 {
            8 as i32
        } else if 75 as i32 == 'J' as i32 {
            9 as i32
        } else if 75 as i32 == 'K' as i32 {
            10 as i32
        } else if 75 as i32 == 'L' as i32 {
            11 as i32
        } else if 75 as i32 == 'M' as i32 {
            12 as i32
        } else if 75 as i32 == 'N' as i32 {
            13 as i32
        } else if 75 as i32 == 'O' as i32 {
            14 as i32
        } else if 75 as i32 == 'P' as i32 {
            15 as i32
        } else if 75 as i32 == 'Q' as i32 {
            16 as i32
        } else if 75 as i32 == 'R' as i32 {
            17 as i32
        } else if 75 as i32 == 'S' as i32 {
            18 as i32
        } else if 75 as i32 == 'T' as i32 {
            19 as i32
        } else if 75 as i32 == 'U' as i32 {
            20 as i32
        } else if 75 as i32 == 'V' as i32 {
            21 as i32
        } else if 75 as i32 == 'W' as i32 {
            22 as i32
        } else if 75 as i32 == 'X' as i32 {
            23 as i32
        } else if 75 as i32 == 'Y' as i32 {
            24 as i32
        } else if 75 as i32 == 'Z' as i32 {
            25 as i32
        } else if 75 as i32 == '2' as i32 {
            26 as i32
        } else if 75 as i32 == '3' as i32 {
            27 as i32
        } else if 75 as i32 == '4' as i32 {
            28 as i32
        } else if 75 as i32 == '5' as i32 {
            29 as i32
        } else if 75 as i32 == '6' as i32 {
            30 as i32
        } else if 75 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 76 as i32 == 'A' as i32 {
            0 as i32
        } else if 76 as i32 == 'B' as i32 {
            1 as i32
        } else if 76 as i32 == 'C' as i32 {
            2 as i32
        } else if 76 as i32 == 'D' as i32 {
            3 as i32
        } else if 76 as i32 == 'E' as i32 {
            4 as i32
        } else if 76 as i32 == 'F' as i32 {
            5 as i32
        } else if 76 as i32 == 'G' as i32 {
            6 as i32
        } else if 76 as i32 == 'H' as i32 {
            7 as i32
        } else if 76 as i32 == 'I' as i32 {
            8 as i32
        } else if 76 as i32 == 'J' as i32 {
            9 as i32
        } else if 76 as i32 == 'K' as i32 {
            10 as i32
        } else if 76 as i32 == 'L' as i32 {
            11 as i32
        } else if 76 as i32 == 'M' as i32 {
            12 as i32
        } else if 76 as i32 == 'N' as i32 {
            13 as i32
        } else if 76 as i32 == 'O' as i32 {
            14 as i32
        } else if 76 as i32 == 'P' as i32 {
            15 as i32
        } else if 76 as i32 == 'Q' as i32 {
            16 as i32
        } else if 76 as i32 == 'R' as i32 {
            17 as i32
        } else if 76 as i32 == 'S' as i32 {
            18 as i32
        } else if 76 as i32 == 'T' as i32 {
            19 as i32
        } else if 76 as i32 == 'U' as i32 {
            20 as i32
        } else if 76 as i32 == 'V' as i32 {
            21 as i32
        } else if 76 as i32 == 'W' as i32 {
            22 as i32
        } else if 76 as i32 == 'X' as i32 {
            23 as i32
        } else if 76 as i32 == 'Y' as i32 {
            24 as i32
        } else if 76 as i32 == 'Z' as i32 {
            25 as i32
        } else if 76 as i32 == '2' as i32 {
            26 as i32
        } else if 76 as i32 == '3' as i32 {
            27 as i32
        } else if 76 as i32 == '4' as i32 {
            28 as i32
        } else if 76 as i32 == '5' as i32 {
            29 as i32
        } else if 76 as i32 == '6' as i32 {
            30 as i32
        } else if 76 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 77 as i32 == 'A' as i32 {
            0 as i32
        } else if 77 as i32 == 'B' as i32 {
            1 as i32
        } else if 77 as i32 == 'C' as i32 {
            2 as i32
        } else if 77 as i32 == 'D' as i32 {
            3 as i32
        } else if 77 as i32 == 'E' as i32 {
            4 as i32
        } else if 77 as i32 == 'F' as i32 {
            5 as i32
        } else if 77 as i32 == 'G' as i32 {
            6 as i32
        } else if 77 as i32 == 'H' as i32 {
            7 as i32
        } else if 77 as i32 == 'I' as i32 {
            8 as i32
        } else if 77 as i32 == 'J' as i32 {
            9 as i32
        } else if 77 as i32 == 'K' as i32 {
            10 as i32
        } else if 77 as i32 == 'L' as i32 {
            11 as i32
        } else if 77 as i32 == 'M' as i32 {
            12 as i32
        } else if 77 as i32 == 'N' as i32 {
            13 as i32
        } else if 77 as i32 == 'O' as i32 {
            14 as i32
        } else if 77 as i32 == 'P' as i32 {
            15 as i32
        } else if 77 as i32 == 'Q' as i32 {
            16 as i32
        } else if 77 as i32 == 'R' as i32 {
            17 as i32
        } else if 77 as i32 == 'S' as i32 {
            18 as i32
        } else if 77 as i32 == 'T' as i32 {
            19 as i32
        } else if 77 as i32 == 'U' as i32 {
            20 as i32
        } else if 77 as i32 == 'V' as i32 {
            21 as i32
        } else if 77 as i32 == 'W' as i32 {
            22 as i32
        } else if 77 as i32 == 'X' as i32 {
            23 as i32
        } else if 77 as i32 == 'Y' as i32 {
            24 as i32
        } else if 77 as i32 == 'Z' as i32 {
            25 as i32
        } else if 77 as i32 == '2' as i32 {
            26 as i32
        } else if 77 as i32 == '3' as i32 {
            27 as i32
        } else if 77 as i32 == '4' as i32 {
            28 as i32
        } else if 77 as i32 == '5' as i32 {
            29 as i32
        } else if 77 as i32 == '6' as i32 {
            30 as i32
        } else if 77 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 78 as i32 == 'A' as i32 {
            0 as i32
        } else if 78 as i32 == 'B' as i32 {
            1 as i32
        } else if 78 as i32 == 'C' as i32 {
            2 as i32
        } else if 78 as i32 == 'D' as i32 {
            3 as i32
        } else if 78 as i32 == 'E' as i32 {
            4 as i32
        } else if 78 as i32 == 'F' as i32 {
            5 as i32
        } else if 78 as i32 == 'G' as i32 {
            6 as i32
        } else if 78 as i32 == 'H' as i32 {
            7 as i32
        } else if 78 as i32 == 'I' as i32 {
            8 as i32
        } else if 78 as i32 == 'J' as i32 {
            9 as i32
        } else if 78 as i32 == 'K' as i32 {
            10 as i32
        } else if 78 as i32 == 'L' as i32 {
            11 as i32
        } else if 78 as i32 == 'M' as i32 {
            12 as i32
        } else if 78 as i32 == 'N' as i32 {
            13 as i32
        } else if 78 as i32 == 'O' as i32 {
            14 as i32
        } else if 78 as i32 == 'P' as i32 {
            15 as i32
        } else if 78 as i32 == 'Q' as i32 {
            16 as i32
        } else if 78 as i32 == 'R' as i32 {
            17 as i32
        } else if 78 as i32 == 'S' as i32 {
            18 as i32
        } else if 78 as i32 == 'T' as i32 {
            19 as i32
        } else if 78 as i32 == 'U' as i32 {
            20 as i32
        } else if 78 as i32 == 'V' as i32 {
            21 as i32
        } else if 78 as i32 == 'W' as i32 {
            22 as i32
        } else if 78 as i32 == 'X' as i32 {
            23 as i32
        } else if 78 as i32 == 'Y' as i32 {
            24 as i32
        } else if 78 as i32 == 'Z' as i32 {
            25 as i32
        } else if 78 as i32 == '2' as i32 {
            26 as i32
        } else if 78 as i32 == '3' as i32 {
            27 as i32
        } else if 78 as i32 == '4' as i32 {
            28 as i32
        } else if 78 as i32 == '5' as i32 {
            29 as i32
        } else if 78 as i32 == '6' as i32 {
            30 as i32
        } else if 78 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 79 as i32 == 'A' as i32 {
            0 as i32
        } else if 79 as i32 == 'B' as i32 {
            1 as i32
        } else if 79 as i32 == 'C' as i32 {
            2 as i32
        } else if 79 as i32 == 'D' as i32 {
            3 as i32
        } else if 79 as i32 == 'E' as i32 {
            4 as i32
        } else if 79 as i32 == 'F' as i32 {
            5 as i32
        } else if 79 as i32 == 'G' as i32 {
            6 as i32
        } else if 79 as i32 == 'H' as i32 {
            7 as i32
        } else if 79 as i32 == 'I' as i32 {
            8 as i32
        } else if 79 as i32 == 'J' as i32 {
            9 as i32
        } else if 79 as i32 == 'K' as i32 {
            10 as i32
        } else if 79 as i32 == 'L' as i32 {
            11 as i32
        } else if 79 as i32 == 'M' as i32 {
            12 as i32
        } else if 79 as i32 == 'N' as i32 {
            13 as i32
        } else if 79 as i32 == 'O' as i32 {
            14 as i32
        } else if 79 as i32 == 'P' as i32 {
            15 as i32
        } else if 79 as i32 == 'Q' as i32 {
            16 as i32
        } else if 79 as i32 == 'R' as i32 {
            17 as i32
        } else if 79 as i32 == 'S' as i32 {
            18 as i32
        } else if 79 as i32 == 'T' as i32 {
            19 as i32
        } else if 79 as i32 == 'U' as i32 {
            20 as i32
        } else if 79 as i32 == 'V' as i32 {
            21 as i32
        } else if 79 as i32 == 'W' as i32 {
            22 as i32
        } else if 79 as i32 == 'X' as i32 {
            23 as i32
        } else if 79 as i32 == 'Y' as i32 {
            24 as i32
        } else if 79 as i32 == 'Z' as i32 {
            25 as i32
        } else if 79 as i32 == '2' as i32 {
            26 as i32
        } else if 79 as i32 == '3' as i32 {
            27 as i32
        } else if 79 as i32 == '4' as i32 {
            28 as i32
        } else if 79 as i32 == '5' as i32 {
            29 as i32
        } else if 79 as i32 == '6' as i32 {
            30 as i32
        } else if 79 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 80 as i32 == 'A' as i32 {
            0 as i32
        } else if 80 as i32 == 'B' as i32 {
            1 as i32
        } else if 80 as i32 == 'C' as i32 {
            2 as i32
        } else if 80 as i32 == 'D' as i32 {
            3 as i32
        } else if 80 as i32 == 'E' as i32 {
            4 as i32
        } else if 80 as i32 == 'F' as i32 {
            5 as i32
        } else if 80 as i32 == 'G' as i32 {
            6 as i32
        } else if 80 as i32 == 'H' as i32 {
            7 as i32
        } else if 80 as i32 == 'I' as i32 {
            8 as i32
        } else if 80 as i32 == 'J' as i32 {
            9 as i32
        } else if 80 as i32 == 'K' as i32 {
            10 as i32
        } else if 80 as i32 == 'L' as i32 {
            11 as i32
        } else if 80 as i32 == 'M' as i32 {
            12 as i32
        } else if 80 as i32 == 'N' as i32 {
            13 as i32
        } else if 80 as i32 == 'O' as i32 {
            14 as i32
        } else if 80 as i32 == 'P' as i32 {
            15 as i32
        } else if 80 as i32 == 'Q' as i32 {
            16 as i32
        } else if 80 as i32 == 'R' as i32 {
            17 as i32
        } else if 80 as i32 == 'S' as i32 {
            18 as i32
        } else if 80 as i32 == 'T' as i32 {
            19 as i32
        } else if 80 as i32 == 'U' as i32 {
            20 as i32
        } else if 80 as i32 == 'V' as i32 {
            21 as i32
        } else if 80 as i32 == 'W' as i32 {
            22 as i32
        } else if 80 as i32 == 'X' as i32 {
            23 as i32
        } else if 80 as i32 == 'Y' as i32 {
            24 as i32
        } else if 80 as i32 == 'Z' as i32 {
            25 as i32
        } else if 80 as i32 == '2' as i32 {
            26 as i32
        } else if 80 as i32 == '3' as i32 {
            27 as i32
        } else if 80 as i32 == '4' as i32 {
            28 as i32
        } else if 80 as i32 == '5' as i32 {
            29 as i32
        } else if 80 as i32 == '6' as i32 {
            30 as i32
        } else if 80 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 81 as i32 == 'A' as i32 {
            0 as i32
        } else if 81 as i32 == 'B' as i32 {
            1 as i32
        } else if 81 as i32 == 'C' as i32 {
            2 as i32
        } else if 81 as i32 == 'D' as i32 {
            3 as i32
        } else if 81 as i32 == 'E' as i32 {
            4 as i32
        } else if 81 as i32 == 'F' as i32 {
            5 as i32
        } else if 81 as i32 == 'G' as i32 {
            6 as i32
        } else if 81 as i32 == 'H' as i32 {
            7 as i32
        } else if 81 as i32 == 'I' as i32 {
            8 as i32
        } else if 81 as i32 == 'J' as i32 {
            9 as i32
        } else if 81 as i32 == 'K' as i32 {
            10 as i32
        } else if 81 as i32 == 'L' as i32 {
            11 as i32
        } else if 81 as i32 == 'M' as i32 {
            12 as i32
        } else if 81 as i32 == 'N' as i32 {
            13 as i32
        } else if 81 as i32 == 'O' as i32 {
            14 as i32
        } else if 81 as i32 == 'P' as i32 {
            15 as i32
        } else if 81 as i32 == 'Q' as i32 {
            16 as i32
        } else if 81 as i32 == 'R' as i32 {
            17 as i32
        } else if 81 as i32 == 'S' as i32 {
            18 as i32
        } else if 81 as i32 == 'T' as i32 {
            19 as i32
        } else if 81 as i32 == 'U' as i32 {
            20 as i32
        } else if 81 as i32 == 'V' as i32 {
            21 as i32
        } else if 81 as i32 == 'W' as i32 {
            22 as i32
        } else if 81 as i32 == 'X' as i32 {
            23 as i32
        } else if 81 as i32 == 'Y' as i32 {
            24 as i32
        } else if 81 as i32 == 'Z' as i32 {
            25 as i32
        } else if 81 as i32 == '2' as i32 {
            26 as i32
        } else if 81 as i32 == '3' as i32 {
            27 as i32
        } else if 81 as i32 == '4' as i32 {
            28 as i32
        } else if 81 as i32 == '5' as i32 {
            29 as i32
        } else if 81 as i32 == '6' as i32 {
            30 as i32
        } else if 81 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 82 as i32 == 'A' as i32 {
            0 as i32
        } else if 82 as i32 == 'B' as i32 {
            1 as i32
        } else if 82 as i32 == 'C' as i32 {
            2 as i32
        } else if 82 as i32 == 'D' as i32 {
            3 as i32
        } else if 82 as i32 == 'E' as i32 {
            4 as i32
        } else if 82 as i32 == 'F' as i32 {
            5 as i32
        } else if 82 as i32 == 'G' as i32 {
            6 as i32
        } else if 82 as i32 == 'H' as i32 {
            7 as i32
        } else if 82 as i32 == 'I' as i32 {
            8 as i32
        } else if 82 as i32 == 'J' as i32 {
            9 as i32
        } else if 82 as i32 == 'K' as i32 {
            10 as i32
        } else if 82 as i32 == 'L' as i32 {
            11 as i32
        } else if 82 as i32 == 'M' as i32 {
            12 as i32
        } else if 82 as i32 == 'N' as i32 {
            13 as i32
        } else if 82 as i32 == 'O' as i32 {
            14 as i32
        } else if 82 as i32 == 'P' as i32 {
            15 as i32
        } else if 82 as i32 == 'Q' as i32 {
            16 as i32
        } else if 82 as i32 == 'R' as i32 {
            17 as i32
        } else if 82 as i32 == 'S' as i32 {
            18 as i32
        } else if 82 as i32 == 'T' as i32 {
            19 as i32
        } else if 82 as i32 == 'U' as i32 {
            20 as i32
        } else if 82 as i32 == 'V' as i32 {
            21 as i32
        } else if 82 as i32 == 'W' as i32 {
            22 as i32
        } else if 82 as i32 == 'X' as i32 {
            23 as i32
        } else if 82 as i32 == 'Y' as i32 {
            24 as i32
        } else if 82 as i32 == 'Z' as i32 {
            25 as i32
        } else if 82 as i32 == '2' as i32 {
            26 as i32
        } else if 82 as i32 == '3' as i32 {
            27 as i32
        } else if 82 as i32 == '4' as i32 {
            28 as i32
        } else if 82 as i32 == '5' as i32 {
            29 as i32
        } else if 82 as i32 == '6' as i32 {
            30 as i32
        } else if 82 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 83 as i32 == 'A' as i32 {
            0 as i32
        } else if 83 as i32 == 'B' as i32 {
            1 as i32
        } else if 83 as i32 == 'C' as i32 {
            2 as i32
        } else if 83 as i32 == 'D' as i32 {
            3 as i32
        } else if 83 as i32 == 'E' as i32 {
            4 as i32
        } else if 83 as i32 == 'F' as i32 {
            5 as i32
        } else if 83 as i32 == 'G' as i32 {
            6 as i32
        } else if 83 as i32 == 'H' as i32 {
            7 as i32
        } else if 83 as i32 == 'I' as i32 {
            8 as i32
        } else if 83 as i32 == 'J' as i32 {
            9 as i32
        } else if 83 as i32 == 'K' as i32 {
            10 as i32
        } else if 83 as i32 == 'L' as i32 {
            11 as i32
        } else if 83 as i32 == 'M' as i32 {
            12 as i32
        } else if 83 as i32 == 'N' as i32 {
            13 as i32
        } else if 83 as i32 == 'O' as i32 {
            14 as i32
        } else if 83 as i32 == 'P' as i32 {
            15 as i32
        } else if 83 as i32 == 'Q' as i32 {
            16 as i32
        } else if 83 as i32 == 'R' as i32 {
            17 as i32
        } else if 83 as i32 == 'S' as i32 {
            18 as i32
        } else if 83 as i32 == 'T' as i32 {
            19 as i32
        } else if 83 as i32 == 'U' as i32 {
            20 as i32
        } else if 83 as i32 == 'V' as i32 {
            21 as i32
        } else if 83 as i32 == 'W' as i32 {
            22 as i32
        } else if 83 as i32 == 'X' as i32 {
            23 as i32
        } else if 83 as i32 == 'Y' as i32 {
            24 as i32
        } else if 83 as i32 == 'Z' as i32 {
            25 as i32
        } else if 83 as i32 == '2' as i32 {
            26 as i32
        } else if 83 as i32 == '3' as i32 {
            27 as i32
        } else if 83 as i32 == '4' as i32 {
            28 as i32
        } else if 83 as i32 == '5' as i32 {
            29 as i32
        } else if 83 as i32 == '6' as i32 {
            30 as i32
        } else if 83 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 84 as i32 == 'A' as i32 {
            0 as i32
        } else if 84 as i32 == 'B' as i32 {
            1 as i32
        } else if 84 as i32 == 'C' as i32 {
            2 as i32
        } else if 84 as i32 == 'D' as i32 {
            3 as i32
        } else if 84 as i32 == 'E' as i32 {
            4 as i32
        } else if 84 as i32 == 'F' as i32 {
            5 as i32
        } else if 84 as i32 == 'G' as i32 {
            6 as i32
        } else if 84 as i32 == 'H' as i32 {
            7 as i32
        } else if 84 as i32 == 'I' as i32 {
            8 as i32
        } else if 84 as i32 == 'J' as i32 {
            9 as i32
        } else if 84 as i32 == 'K' as i32 {
            10 as i32
        } else if 84 as i32 == 'L' as i32 {
            11 as i32
        } else if 84 as i32 == 'M' as i32 {
            12 as i32
        } else if 84 as i32 == 'N' as i32 {
            13 as i32
        } else if 84 as i32 == 'O' as i32 {
            14 as i32
        } else if 84 as i32 == 'P' as i32 {
            15 as i32
        } else if 84 as i32 == 'Q' as i32 {
            16 as i32
        } else if 84 as i32 == 'R' as i32 {
            17 as i32
        } else if 84 as i32 == 'S' as i32 {
            18 as i32
        } else if 84 as i32 == 'T' as i32 {
            19 as i32
        } else if 84 as i32 == 'U' as i32 {
            20 as i32
        } else if 84 as i32 == 'V' as i32 {
            21 as i32
        } else if 84 as i32 == 'W' as i32 {
            22 as i32
        } else if 84 as i32 == 'X' as i32 {
            23 as i32
        } else if 84 as i32 == 'Y' as i32 {
            24 as i32
        } else if 84 as i32 == 'Z' as i32 {
            25 as i32
        } else if 84 as i32 == '2' as i32 {
            26 as i32
        } else if 84 as i32 == '3' as i32 {
            27 as i32
        } else if 84 as i32 == '4' as i32 {
            28 as i32
        } else if 84 as i32 == '5' as i32 {
            29 as i32
        } else if 84 as i32 == '6' as i32 {
            30 as i32
        } else if 84 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 85 as i32 == 'A' as i32 {
            0 as i32
        } else if 85 as i32 == 'B' as i32 {
            1 as i32
        } else if 85 as i32 == 'C' as i32 {
            2 as i32
        } else if 85 as i32 == 'D' as i32 {
            3 as i32
        } else if 85 as i32 == 'E' as i32 {
            4 as i32
        } else if 85 as i32 == 'F' as i32 {
            5 as i32
        } else if 85 as i32 == 'G' as i32 {
            6 as i32
        } else if 85 as i32 == 'H' as i32 {
            7 as i32
        } else if 85 as i32 == 'I' as i32 {
            8 as i32
        } else if 85 as i32 == 'J' as i32 {
            9 as i32
        } else if 85 as i32 == 'K' as i32 {
            10 as i32
        } else if 85 as i32 == 'L' as i32 {
            11 as i32
        } else if 85 as i32 == 'M' as i32 {
            12 as i32
        } else if 85 as i32 == 'N' as i32 {
            13 as i32
        } else if 85 as i32 == 'O' as i32 {
            14 as i32
        } else if 85 as i32 == 'P' as i32 {
            15 as i32
        } else if 85 as i32 == 'Q' as i32 {
            16 as i32
        } else if 85 as i32 == 'R' as i32 {
            17 as i32
        } else if 85 as i32 == 'S' as i32 {
            18 as i32
        } else if 85 as i32 == 'T' as i32 {
            19 as i32
        } else if 85 as i32 == 'U' as i32 {
            20 as i32
        } else if 85 as i32 == 'V' as i32 {
            21 as i32
        } else if 85 as i32 == 'W' as i32 {
            22 as i32
        } else if 85 as i32 == 'X' as i32 {
            23 as i32
        } else if 85 as i32 == 'Y' as i32 {
            24 as i32
        } else if 85 as i32 == 'Z' as i32 {
            25 as i32
        } else if 85 as i32 == '2' as i32 {
            26 as i32
        } else if 85 as i32 == '3' as i32 {
            27 as i32
        } else if 85 as i32 == '4' as i32 {
            28 as i32
        } else if 85 as i32 == '5' as i32 {
            29 as i32
        } else if 85 as i32 == '6' as i32 {
            30 as i32
        } else if 85 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 86 as i32 == 'A' as i32 {
            0 as i32
        } else if 86 as i32 == 'B' as i32 {
            1 as i32
        } else if 86 as i32 == 'C' as i32 {
            2 as i32
        } else if 86 as i32 == 'D' as i32 {
            3 as i32
        } else if 86 as i32 == 'E' as i32 {
            4 as i32
        } else if 86 as i32 == 'F' as i32 {
            5 as i32
        } else if 86 as i32 == 'G' as i32 {
            6 as i32
        } else if 86 as i32 == 'H' as i32 {
            7 as i32
        } else if 86 as i32 == 'I' as i32 {
            8 as i32
        } else if 86 as i32 == 'J' as i32 {
            9 as i32
        } else if 86 as i32 == 'K' as i32 {
            10 as i32
        } else if 86 as i32 == 'L' as i32 {
            11 as i32
        } else if 86 as i32 == 'M' as i32 {
            12 as i32
        } else if 86 as i32 == 'N' as i32 {
            13 as i32
        } else if 86 as i32 == 'O' as i32 {
            14 as i32
        } else if 86 as i32 == 'P' as i32 {
            15 as i32
        } else if 86 as i32 == 'Q' as i32 {
            16 as i32
        } else if 86 as i32 == 'R' as i32 {
            17 as i32
        } else if 86 as i32 == 'S' as i32 {
            18 as i32
        } else if 86 as i32 == 'T' as i32 {
            19 as i32
        } else if 86 as i32 == 'U' as i32 {
            20 as i32
        } else if 86 as i32 == 'V' as i32 {
            21 as i32
        } else if 86 as i32 == 'W' as i32 {
            22 as i32
        } else if 86 as i32 == 'X' as i32 {
            23 as i32
        } else if 86 as i32 == 'Y' as i32 {
            24 as i32
        } else if 86 as i32 == 'Z' as i32 {
            25 as i32
        } else if 86 as i32 == '2' as i32 {
            26 as i32
        } else if 86 as i32 == '3' as i32 {
            27 as i32
        } else if 86 as i32 == '4' as i32 {
            28 as i32
        } else if 86 as i32 == '5' as i32 {
            29 as i32
        } else if 86 as i32 == '6' as i32 {
            30 as i32
        } else if 86 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 87 as i32 == 'A' as i32 {
            0 as i32
        } else if 87 as i32 == 'B' as i32 {
            1 as i32
        } else if 87 as i32 == 'C' as i32 {
            2 as i32
        } else if 87 as i32 == 'D' as i32 {
            3 as i32
        } else if 87 as i32 == 'E' as i32 {
            4 as i32
        } else if 87 as i32 == 'F' as i32 {
            5 as i32
        } else if 87 as i32 == 'G' as i32 {
            6 as i32
        } else if 87 as i32 == 'H' as i32 {
            7 as i32
        } else if 87 as i32 == 'I' as i32 {
            8 as i32
        } else if 87 as i32 == 'J' as i32 {
            9 as i32
        } else if 87 as i32 == 'K' as i32 {
            10 as i32
        } else if 87 as i32 == 'L' as i32 {
            11 as i32
        } else if 87 as i32 == 'M' as i32 {
            12 as i32
        } else if 87 as i32 == 'N' as i32 {
            13 as i32
        } else if 87 as i32 == 'O' as i32 {
            14 as i32
        } else if 87 as i32 == 'P' as i32 {
            15 as i32
        } else if 87 as i32 == 'Q' as i32 {
            16 as i32
        } else if 87 as i32 == 'R' as i32 {
            17 as i32
        } else if 87 as i32 == 'S' as i32 {
            18 as i32
        } else if 87 as i32 == 'T' as i32 {
            19 as i32
        } else if 87 as i32 == 'U' as i32 {
            20 as i32
        } else if 87 as i32 == 'V' as i32 {
            21 as i32
        } else if 87 as i32 == 'W' as i32 {
            22 as i32
        } else if 87 as i32 == 'X' as i32 {
            23 as i32
        } else if 87 as i32 == 'Y' as i32 {
            24 as i32
        } else if 87 as i32 == 'Z' as i32 {
            25 as i32
        } else if 87 as i32 == '2' as i32 {
            26 as i32
        } else if 87 as i32 == '3' as i32 {
            27 as i32
        } else if 87 as i32 == '4' as i32 {
            28 as i32
        } else if 87 as i32 == '5' as i32 {
            29 as i32
        } else if 87 as i32 == '6' as i32 {
            30 as i32
        } else if 87 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 88 as i32 == 'A' as i32 {
            0 as i32
        } else if 88 as i32 == 'B' as i32 {
            1 as i32
        } else if 88 as i32 == 'C' as i32 {
            2 as i32
        } else if 88 as i32 == 'D' as i32 {
            3 as i32
        } else if 88 as i32 == 'E' as i32 {
            4 as i32
        } else if 88 as i32 == 'F' as i32 {
            5 as i32
        } else if 88 as i32 == 'G' as i32 {
            6 as i32
        } else if 88 as i32 == 'H' as i32 {
            7 as i32
        } else if 88 as i32 == 'I' as i32 {
            8 as i32
        } else if 88 as i32 == 'J' as i32 {
            9 as i32
        } else if 88 as i32 == 'K' as i32 {
            10 as i32
        } else if 88 as i32 == 'L' as i32 {
            11 as i32
        } else if 88 as i32 == 'M' as i32 {
            12 as i32
        } else if 88 as i32 == 'N' as i32 {
            13 as i32
        } else if 88 as i32 == 'O' as i32 {
            14 as i32
        } else if 88 as i32 == 'P' as i32 {
            15 as i32
        } else if 88 as i32 == 'Q' as i32 {
            16 as i32
        } else if 88 as i32 == 'R' as i32 {
            17 as i32
        } else if 88 as i32 == 'S' as i32 {
            18 as i32
        } else if 88 as i32 == 'T' as i32 {
            19 as i32
        } else if 88 as i32 == 'U' as i32 {
            20 as i32
        } else if 88 as i32 == 'V' as i32 {
            21 as i32
        } else if 88 as i32 == 'W' as i32 {
            22 as i32
        } else if 88 as i32 == 'X' as i32 {
            23 as i32
        } else if 88 as i32 == 'Y' as i32 {
            24 as i32
        } else if 88 as i32 == 'Z' as i32 {
            25 as i32
        } else if 88 as i32 == '2' as i32 {
            26 as i32
        } else if 88 as i32 == '3' as i32 {
            27 as i32
        } else if 88 as i32 == '4' as i32 {
            28 as i32
        } else if 88 as i32 == '5' as i32 {
            29 as i32
        } else if 88 as i32 == '6' as i32 {
            30 as i32
        } else if 88 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 89 as i32 == 'A' as i32 {
            0 as i32
        } else if 89 as i32 == 'B' as i32 {
            1 as i32
        } else if 89 as i32 == 'C' as i32 {
            2 as i32
        } else if 89 as i32 == 'D' as i32 {
            3 as i32
        } else if 89 as i32 == 'E' as i32 {
            4 as i32
        } else if 89 as i32 == 'F' as i32 {
            5 as i32
        } else if 89 as i32 == 'G' as i32 {
            6 as i32
        } else if 89 as i32 == 'H' as i32 {
            7 as i32
        } else if 89 as i32 == 'I' as i32 {
            8 as i32
        } else if 89 as i32 == 'J' as i32 {
            9 as i32
        } else if 89 as i32 == 'K' as i32 {
            10 as i32
        } else if 89 as i32 == 'L' as i32 {
            11 as i32
        } else if 89 as i32 == 'M' as i32 {
            12 as i32
        } else if 89 as i32 == 'N' as i32 {
            13 as i32
        } else if 89 as i32 == 'O' as i32 {
            14 as i32
        } else if 89 as i32 == 'P' as i32 {
            15 as i32
        } else if 89 as i32 == 'Q' as i32 {
            16 as i32
        } else if 89 as i32 == 'R' as i32 {
            17 as i32
        } else if 89 as i32 == 'S' as i32 {
            18 as i32
        } else if 89 as i32 == 'T' as i32 {
            19 as i32
        } else if 89 as i32 == 'U' as i32 {
            20 as i32
        } else if 89 as i32 == 'V' as i32 {
            21 as i32
        } else if 89 as i32 == 'W' as i32 {
            22 as i32
        } else if 89 as i32 == 'X' as i32 {
            23 as i32
        } else if 89 as i32 == 'Y' as i32 {
            24 as i32
        } else if 89 as i32 == 'Z' as i32 {
            25 as i32
        } else if 89 as i32 == '2' as i32 {
            26 as i32
        } else if 89 as i32 == '3' as i32 {
            27 as i32
        } else if 89 as i32 == '4' as i32 {
            28 as i32
        } else if 89 as i32 == '5' as i32 {
            29 as i32
        } else if 89 as i32 == '6' as i32 {
            30 as i32
        } else if 89 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 90 as i32 == 'A' as i32 {
            0 as i32
        } else if 90 as i32 == 'B' as i32 {
            1 as i32
        } else if 90 as i32 == 'C' as i32 {
            2 as i32
        } else if 90 as i32 == 'D' as i32 {
            3 as i32
        } else if 90 as i32 == 'E' as i32 {
            4 as i32
        } else if 90 as i32 == 'F' as i32 {
            5 as i32
        } else if 90 as i32 == 'G' as i32 {
            6 as i32
        } else if 90 as i32 == 'H' as i32 {
            7 as i32
        } else if 90 as i32 == 'I' as i32 {
            8 as i32
        } else if 90 as i32 == 'J' as i32 {
            9 as i32
        } else if 90 as i32 == 'K' as i32 {
            10 as i32
        } else if 90 as i32 == 'L' as i32 {
            11 as i32
        } else if 90 as i32 == 'M' as i32 {
            12 as i32
        } else if 90 as i32 == 'N' as i32 {
            13 as i32
        } else if 90 as i32 == 'O' as i32 {
            14 as i32
        } else if 90 as i32 == 'P' as i32 {
            15 as i32
        } else if 90 as i32 == 'Q' as i32 {
            16 as i32
        } else if 90 as i32 == 'R' as i32 {
            17 as i32
        } else if 90 as i32 == 'S' as i32 {
            18 as i32
        } else if 90 as i32 == 'T' as i32 {
            19 as i32
        } else if 90 as i32 == 'U' as i32 {
            20 as i32
        } else if 90 as i32 == 'V' as i32 {
            21 as i32
        } else if 90 as i32 == 'W' as i32 {
            22 as i32
        } else if 90 as i32 == 'X' as i32 {
            23 as i32
        } else if 90 as i32 == 'Y' as i32 {
            24 as i32
        } else if 90 as i32 == 'Z' as i32 {
            25 as i32
        } else if 90 as i32 == '2' as i32 {
            26 as i32
        } else if 90 as i32 == '3' as i32 {
            27 as i32
        } else if 90 as i32 == '4' as i32 {
            28 as i32
        } else if 90 as i32 == '5' as i32 {
            29 as i32
        } else if 90 as i32 == '6' as i32 {
            30 as i32
        } else if 90 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 91 as i32 == 'A' as i32 {
            0 as i32
        } else if 91 as i32 == 'B' as i32 {
            1 as i32
        } else if 91 as i32 == 'C' as i32 {
            2 as i32
        } else if 91 as i32 == 'D' as i32 {
            3 as i32
        } else if 91 as i32 == 'E' as i32 {
            4 as i32
        } else if 91 as i32 == 'F' as i32 {
            5 as i32
        } else if 91 as i32 == 'G' as i32 {
            6 as i32
        } else if 91 as i32 == 'H' as i32 {
            7 as i32
        } else if 91 as i32 == 'I' as i32 {
            8 as i32
        } else if 91 as i32 == 'J' as i32 {
            9 as i32
        } else if 91 as i32 == 'K' as i32 {
            10 as i32
        } else if 91 as i32 == 'L' as i32 {
            11 as i32
        } else if 91 as i32 == 'M' as i32 {
            12 as i32
        } else if 91 as i32 == 'N' as i32 {
            13 as i32
        } else if 91 as i32 == 'O' as i32 {
            14 as i32
        } else if 91 as i32 == 'P' as i32 {
            15 as i32
        } else if 91 as i32 == 'Q' as i32 {
            16 as i32
        } else if 91 as i32 == 'R' as i32 {
            17 as i32
        } else if 91 as i32 == 'S' as i32 {
            18 as i32
        } else if 91 as i32 == 'T' as i32 {
            19 as i32
        } else if 91 as i32 == 'U' as i32 {
            20 as i32
        } else if 91 as i32 == 'V' as i32 {
            21 as i32
        } else if 91 as i32 == 'W' as i32 {
            22 as i32
        } else if 91 as i32 == 'X' as i32 {
            23 as i32
        } else if 91 as i32 == 'Y' as i32 {
            24 as i32
        } else if 91 as i32 == 'Z' as i32 {
            25 as i32
        } else if 91 as i32 == '2' as i32 {
            26 as i32
        } else if 91 as i32 == '3' as i32 {
            27 as i32
        } else if 91 as i32 == '4' as i32 {
            28 as i32
        } else if 91 as i32 == '5' as i32 {
            29 as i32
        } else if 91 as i32 == '6' as i32 {
            30 as i32
        } else if 91 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 92 as i32 == 'A' as i32 {
            0 as i32
        } else if 92 as i32 == 'B' as i32 {
            1 as i32
        } else if 92 as i32 == 'C' as i32 {
            2 as i32
        } else if 92 as i32 == 'D' as i32 {
            3 as i32
        } else if 92 as i32 == 'E' as i32 {
            4 as i32
        } else if 92 as i32 == 'F' as i32 {
            5 as i32
        } else if 92 as i32 == 'G' as i32 {
            6 as i32
        } else if 92 as i32 == 'H' as i32 {
            7 as i32
        } else if 92 as i32 == 'I' as i32 {
            8 as i32
        } else if 92 as i32 == 'J' as i32 {
            9 as i32
        } else if 92 as i32 == 'K' as i32 {
            10 as i32
        } else if 92 as i32 == 'L' as i32 {
            11 as i32
        } else if 92 as i32 == 'M' as i32 {
            12 as i32
        } else if 92 as i32 == 'N' as i32 {
            13 as i32
        } else if 92 as i32 == 'O' as i32 {
            14 as i32
        } else if 92 as i32 == 'P' as i32 {
            15 as i32
        } else if 92 as i32 == 'Q' as i32 {
            16 as i32
        } else if 92 as i32 == 'R' as i32 {
            17 as i32
        } else if 92 as i32 == 'S' as i32 {
            18 as i32
        } else if 92 as i32 == 'T' as i32 {
            19 as i32
        } else if 92 as i32 == 'U' as i32 {
            20 as i32
        } else if 92 as i32 == 'V' as i32 {
            21 as i32
        } else if 92 as i32 == 'W' as i32 {
            22 as i32
        } else if 92 as i32 == 'X' as i32 {
            23 as i32
        } else if 92 as i32 == 'Y' as i32 {
            24 as i32
        } else if 92 as i32 == 'Z' as i32 {
            25 as i32
        } else if 92 as i32 == '2' as i32 {
            26 as i32
        } else if 92 as i32 == '3' as i32 {
            27 as i32
        } else if 92 as i32 == '4' as i32 {
            28 as i32
        } else if 92 as i32 == '5' as i32 {
            29 as i32
        } else if 92 as i32 == '6' as i32 {
            30 as i32
        } else if 92 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 93 as i32 == 'A' as i32 {
            0 as i32
        } else if 93 as i32 == 'B' as i32 {
            1 as i32
        } else if 93 as i32 == 'C' as i32 {
            2 as i32
        } else if 93 as i32 == 'D' as i32 {
            3 as i32
        } else if 93 as i32 == 'E' as i32 {
            4 as i32
        } else if 93 as i32 == 'F' as i32 {
            5 as i32
        } else if 93 as i32 == 'G' as i32 {
            6 as i32
        } else if 93 as i32 == 'H' as i32 {
            7 as i32
        } else if 93 as i32 == 'I' as i32 {
            8 as i32
        } else if 93 as i32 == 'J' as i32 {
            9 as i32
        } else if 93 as i32 == 'K' as i32 {
            10 as i32
        } else if 93 as i32 == 'L' as i32 {
            11 as i32
        } else if 93 as i32 == 'M' as i32 {
            12 as i32
        } else if 93 as i32 == 'N' as i32 {
            13 as i32
        } else if 93 as i32 == 'O' as i32 {
            14 as i32
        } else if 93 as i32 == 'P' as i32 {
            15 as i32
        } else if 93 as i32 == 'Q' as i32 {
            16 as i32
        } else if 93 as i32 == 'R' as i32 {
            17 as i32
        } else if 93 as i32 == 'S' as i32 {
            18 as i32
        } else if 93 as i32 == 'T' as i32 {
            19 as i32
        } else if 93 as i32 == 'U' as i32 {
            20 as i32
        } else if 93 as i32 == 'V' as i32 {
            21 as i32
        } else if 93 as i32 == 'W' as i32 {
            22 as i32
        } else if 93 as i32 == 'X' as i32 {
            23 as i32
        } else if 93 as i32 == 'Y' as i32 {
            24 as i32
        } else if 93 as i32 == 'Z' as i32 {
            25 as i32
        } else if 93 as i32 == '2' as i32 {
            26 as i32
        } else if 93 as i32 == '3' as i32 {
            27 as i32
        } else if 93 as i32 == '4' as i32 {
            28 as i32
        } else if 93 as i32 == '5' as i32 {
            29 as i32
        } else if 93 as i32 == '6' as i32 {
            30 as i32
        } else if 93 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 94 as i32 == 'A' as i32 {
            0 as i32
        } else if 94 as i32 == 'B' as i32 {
            1 as i32
        } else if 94 as i32 == 'C' as i32 {
            2 as i32
        } else if 94 as i32 == 'D' as i32 {
            3 as i32
        } else if 94 as i32 == 'E' as i32 {
            4 as i32
        } else if 94 as i32 == 'F' as i32 {
            5 as i32
        } else if 94 as i32 == 'G' as i32 {
            6 as i32
        } else if 94 as i32 == 'H' as i32 {
            7 as i32
        } else if 94 as i32 == 'I' as i32 {
            8 as i32
        } else if 94 as i32 == 'J' as i32 {
            9 as i32
        } else if 94 as i32 == 'K' as i32 {
            10 as i32
        } else if 94 as i32 == 'L' as i32 {
            11 as i32
        } else if 94 as i32 == 'M' as i32 {
            12 as i32
        } else if 94 as i32 == 'N' as i32 {
            13 as i32
        } else if 94 as i32 == 'O' as i32 {
            14 as i32
        } else if 94 as i32 == 'P' as i32 {
            15 as i32
        } else if 94 as i32 == 'Q' as i32 {
            16 as i32
        } else if 94 as i32 == 'R' as i32 {
            17 as i32
        } else if 94 as i32 == 'S' as i32 {
            18 as i32
        } else if 94 as i32 == 'T' as i32 {
            19 as i32
        } else if 94 as i32 == 'U' as i32 {
            20 as i32
        } else if 94 as i32 == 'V' as i32 {
            21 as i32
        } else if 94 as i32 == 'W' as i32 {
            22 as i32
        } else if 94 as i32 == 'X' as i32 {
            23 as i32
        } else if 94 as i32 == 'Y' as i32 {
            24 as i32
        } else if 94 as i32 == 'Z' as i32 {
            25 as i32
        } else if 94 as i32 == '2' as i32 {
            26 as i32
        } else if 94 as i32 == '3' as i32 {
            27 as i32
        } else if 94 as i32 == '4' as i32 {
            28 as i32
        } else if 94 as i32 == '5' as i32 {
            29 as i32
        } else if 94 as i32 == '6' as i32 {
            30 as i32
        } else if 94 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 95 as i32 == 'A' as i32 {
            0 as i32
        } else if 95 as i32 == 'B' as i32 {
            1 as i32
        } else if 95 as i32 == 'C' as i32 {
            2 as i32
        } else if 95 as i32 == 'D' as i32 {
            3 as i32
        } else if 95 as i32 == 'E' as i32 {
            4 as i32
        } else if 95 as i32 == 'F' as i32 {
            5 as i32
        } else if 95 as i32 == 'G' as i32 {
            6 as i32
        } else if 95 as i32 == 'H' as i32 {
            7 as i32
        } else if 95 as i32 == 'I' as i32 {
            8 as i32
        } else if 95 as i32 == 'J' as i32 {
            9 as i32
        } else if 95 as i32 == 'K' as i32 {
            10 as i32
        } else if 95 as i32 == 'L' as i32 {
            11 as i32
        } else if 95 as i32 == 'M' as i32 {
            12 as i32
        } else if 95 as i32 == 'N' as i32 {
            13 as i32
        } else if 95 as i32 == 'O' as i32 {
            14 as i32
        } else if 95 as i32 == 'P' as i32 {
            15 as i32
        } else if 95 as i32 == 'Q' as i32 {
            16 as i32
        } else if 95 as i32 == 'R' as i32 {
            17 as i32
        } else if 95 as i32 == 'S' as i32 {
            18 as i32
        } else if 95 as i32 == 'T' as i32 {
            19 as i32
        } else if 95 as i32 == 'U' as i32 {
            20 as i32
        } else if 95 as i32 == 'V' as i32 {
            21 as i32
        } else if 95 as i32 == 'W' as i32 {
            22 as i32
        } else if 95 as i32 == 'X' as i32 {
            23 as i32
        } else if 95 as i32 == 'Y' as i32 {
            24 as i32
        } else if 95 as i32 == 'Z' as i32 {
            25 as i32
        } else if 95 as i32 == '2' as i32 {
            26 as i32
        } else if 95 as i32 == '3' as i32 {
            27 as i32
        } else if 95 as i32 == '4' as i32 {
            28 as i32
        } else if 95 as i32 == '5' as i32 {
            29 as i32
        } else if 95 as i32 == '6' as i32 {
            30 as i32
        } else if 95 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 96 as i32 == 'A' as i32 {
            0 as i32
        } else if 96 as i32 == 'B' as i32 {
            1 as i32
        } else if 96 as i32 == 'C' as i32 {
            2 as i32
        } else if 96 as i32 == 'D' as i32 {
            3 as i32
        } else if 96 as i32 == 'E' as i32 {
            4 as i32
        } else if 96 as i32 == 'F' as i32 {
            5 as i32
        } else if 96 as i32 == 'G' as i32 {
            6 as i32
        } else if 96 as i32 == 'H' as i32 {
            7 as i32
        } else if 96 as i32 == 'I' as i32 {
            8 as i32
        } else if 96 as i32 == 'J' as i32 {
            9 as i32
        } else if 96 as i32 == 'K' as i32 {
            10 as i32
        } else if 96 as i32 == 'L' as i32 {
            11 as i32
        } else if 96 as i32 == 'M' as i32 {
            12 as i32
        } else if 96 as i32 == 'N' as i32 {
            13 as i32
        } else if 96 as i32 == 'O' as i32 {
            14 as i32
        } else if 96 as i32 == 'P' as i32 {
            15 as i32
        } else if 96 as i32 == 'Q' as i32 {
            16 as i32
        } else if 96 as i32 == 'R' as i32 {
            17 as i32
        } else if 96 as i32 == 'S' as i32 {
            18 as i32
        } else if 96 as i32 == 'T' as i32 {
            19 as i32
        } else if 96 as i32 == 'U' as i32 {
            20 as i32
        } else if 96 as i32 == 'V' as i32 {
            21 as i32
        } else if 96 as i32 == 'W' as i32 {
            22 as i32
        } else if 96 as i32 == 'X' as i32 {
            23 as i32
        } else if 96 as i32 == 'Y' as i32 {
            24 as i32
        } else if 96 as i32 == 'Z' as i32 {
            25 as i32
        } else if 96 as i32 == '2' as i32 {
            26 as i32
        } else if 96 as i32 == '3' as i32 {
            27 as i32
        } else if 96 as i32 == '4' as i32 {
            28 as i32
        } else if 96 as i32 == '5' as i32 {
            29 as i32
        } else if 96 as i32 == '6' as i32 {
            30 as i32
        } else if 96 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 97 as i32 == 'A' as i32 {
            0 as i32
        } else if 97 as i32 == 'B' as i32 {
            1 as i32
        } else if 97 as i32 == 'C' as i32 {
            2 as i32
        } else if 97 as i32 == 'D' as i32 {
            3 as i32
        } else if 97 as i32 == 'E' as i32 {
            4 as i32
        } else if 97 as i32 == 'F' as i32 {
            5 as i32
        } else if 97 as i32 == 'G' as i32 {
            6 as i32
        } else if 97 as i32 == 'H' as i32 {
            7 as i32
        } else if 97 as i32 == 'I' as i32 {
            8 as i32
        } else if 97 as i32 == 'J' as i32 {
            9 as i32
        } else if 97 as i32 == 'K' as i32 {
            10 as i32
        } else if 97 as i32 == 'L' as i32 {
            11 as i32
        } else if 97 as i32 == 'M' as i32 {
            12 as i32
        } else if 97 as i32 == 'N' as i32 {
            13 as i32
        } else if 97 as i32 == 'O' as i32 {
            14 as i32
        } else if 97 as i32 == 'P' as i32 {
            15 as i32
        } else if 97 as i32 == 'Q' as i32 {
            16 as i32
        } else if 97 as i32 == 'R' as i32 {
            17 as i32
        } else if 97 as i32 == 'S' as i32 {
            18 as i32
        } else if 97 as i32 == 'T' as i32 {
            19 as i32
        } else if 97 as i32 == 'U' as i32 {
            20 as i32
        } else if 97 as i32 == 'V' as i32 {
            21 as i32
        } else if 97 as i32 == 'W' as i32 {
            22 as i32
        } else if 97 as i32 == 'X' as i32 {
            23 as i32
        } else if 97 as i32 == 'Y' as i32 {
            24 as i32
        } else if 97 as i32 == 'Z' as i32 {
            25 as i32
        } else if 97 as i32 == '2' as i32 {
            26 as i32
        } else if 97 as i32 == '3' as i32 {
            27 as i32
        } else if 97 as i32 == '4' as i32 {
            28 as i32
        } else if 97 as i32 == '5' as i32 {
            29 as i32
        } else if 97 as i32 == '6' as i32 {
            30 as i32
        } else if 97 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 98 as i32 == 'A' as i32 {
            0 as i32
        } else if 98 as i32 == 'B' as i32 {
            1 as i32
        } else if 98 as i32 == 'C' as i32 {
            2 as i32
        } else if 98 as i32 == 'D' as i32 {
            3 as i32
        } else if 98 as i32 == 'E' as i32 {
            4 as i32
        } else if 98 as i32 == 'F' as i32 {
            5 as i32
        } else if 98 as i32 == 'G' as i32 {
            6 as i32
        } else if 98 as i32 == 'H' as i32 {
            7 as i32
        } else if 98 as i32 == 'I' as i32 {
            8 as i32
        } else if 98 as i32 == 'J' as i32 {
            9 as i32
        } else if 98 as i32 == 'K' as i32 {
            10 as i32
        } else if 98 as i32 == 'L' as i32 {
            11 as i32
        } else if 98 as i32 == 'M' as i32 {
            12 as i32
        } else if 98 as i32 == 'N' as i32 {
            13 as i32
        } else if 98 as i32 == 'O' as i32 {
            14 as i32
        } else if 98 as i32 == 'P' as i32 {
            15 as i32
        } else if 98 as i32 == 'Q' as i32 {
            16 as i32
        } else if 98 as i32 == 'R' as i32 {
            17 as i32
        } else if 98 as i32 == 'S' as i32 {
            18 as i32
        } else if 98 as i32 == 'T' as i32 {
            19 as i32
        } else if 98 as i32 == 'U' as i32 {
            20 as i32
        } else if 98 as i32 == 'V' as i32 {
            21 as i32
        } else if 98 as i32 == 'W' as i32 {
            22 as i32
        } else if 98 as i32 == 'X' as i32 {
            23 as i32
        } else if 98 as i32 == 'Y' as i32 {
            24 as i32
        } else if 98 as i32 == 'Z' as i32 {
            25 as i32
        } else if 98 as i32 == '2' as i32 {
            26 as i32
        } else if 98 as i32 == '3' as i32 {
            27 as i32
        } else if 98 as i32 == '4' as i32 {
            28 as i32
        } else if 98 as i32 == '5' as i32 {
            29 as i32
        } else if 98 as i32 == '6' as i32 {
            30 as i32
        } else if 98 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 99 as i32 == 'A' as i32 {
            0 as i32
        } else if 99 as i32 == 'B' as i32 {
            1 as i32
        } else if 99 as i32 == 'C' as i32 {
            2 as i32
        } else if 99 as i32 == 'D' as i32 {
            3 as i32
        } else if 99 as i32 == 'E' as i32 {
            4 as i32
        } else if 99 as i32 == 'F' as i32 {
            5 as i32
        } else if 99 as i32 == 'G' as i32 {
            6 as i32
        } else if 99 as i32 == 'H' as i32 {
            7 as i32
        } else if 99 as i32 == 'I' as i32 {
            8 as i32
        } else if 99 as i32 == 'J' as i32 {
            9 as i32
        } else if 99 as i32 == 'K' as i32 {
            10 as i32
        } else if 99 as i32 == 'L' as i32 {
            11 as i32
        } else if 99 as i32 == 'M' as i32 {
            12 as i32
        } else if 99 as i32 == 'N' as i32 {
            13 as i32
        } else if 99 as i32 == 'O' as i32 {
            14 as i32
        } else if 99 as i32 == 'P' as i32 {
            15 as i32
        } else if 99 as i32 == 'Q' as i32 {
            16 as i32
        } else if 99 as i32 == 'R' as i32 {
            17 as i32
        } else if 99 as i32 == 'S' as i32 {
            18 as i32
        } else if 99 as i32 == 'T' as i32 {
            19 as i32
        } else if 99 as i32 == 'U' as i32 {
            20 as i32
        } else if 99 as i32 == 'V' as i32 {
            21 as i32
        } else if 99 as i32 == 'W' as i32 {
            22 as i32
        } else if 99 as i32 == 'X' as i32 {
            23 as i32
        } else if 99 as i32 == 'Y' as i32 {
            24 as i32
        } else if 99 as i32 == 'Z' as i32 {
            25 as i32
        } else if 99 as i32 == '2' as i32 {
            26 as i32
        } else if 99 as i32 == '3' as i32 {
            27 as i32
        } else if 99 as i32 == '4' as i32 {
            28 as i32
        } else if 99 as i32 == '5' as i32 {
            29 as i32
        } else if 99 as i32 == '6' as i32 {
            30 as i32
        } else if 99 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 100 as i32 == 'A' as i32 {
            0 as i32
        } else if 100 as i32 == 'B' as i32 {
            1 as i32
        } else if 100 as i32 == 'C' as i32 {
            2 as i32
        } else if 100 as i32 == 'D' as i32 {
            3 as i32
        } else if 100 as i32 == 'E' as i32 {
            4 as i32
        } else if 100 as i32 == 'F' as i32 {
            5 as i32
        } else if 100 as i32 == 'G' as i32 {
            6 as i32
        } else if 100 as i32 == 'H' as i32 {
            7 as i32
        } else if 100 as i32 == 'I' as i32 {
            8 as i32
        } else if 100 as i32 == 'J' as i32 {
            9 as i32
        } else if 100 as i32 == 'K' as i32 {
            10 as i32
        } else if 100 as i32 == 'L' as i32 {
            11 as i32
        } else if 100 as i32 == 'M' as i32 {
            12 as i32
        } else if 100 as i32 == 'N' as i32 {
            13 as i32
        } else if 100 as i32 == 'O' as i32 {
            14 as i32
        } else if 100 as i32 == 'P' as i32 {
            15 as i32
        } else if 100 as i32 == 'Q' as i32 {
            16 as i32
        } else if 100 as i32 == 'R' as i32 {
            17 as i32
        } else if 100 as i32 == 'S' as i32 {
            18 as i32
        } else if 100 as i32 == 'T' as i32 {
            19 as i32
        } else if 100 as i32 == 'U' as i32 {
            20 as i32
        } else if 100 as i32 == 'V' as i32 {
            21 as i32
        } else if 100 as i32 == 'W' as i32 {
            22 as i32
        } else if 100 as i32 == 'X' as i32 {
            23 as i32
        } else if 100 as i32 == 'Y' as i32 {
            24 as i32
        } else if 100 as i32 == 'Z' as i32 {
            25 as i32
        } else if 100 as i32 == '2' as i32 {
            26 as i32
        } else if 100 as i32 == '3' as i32 {
            27 as i32
        } else if 100 as i32 == '4' as i32 {
            28 as i32
        } else if 100 as i32 == '5' as i32 {
            29 as i32
        } else if 100 as i32 == '6' as i32 {
            30 as i32
        } else if 100 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 101 as i32 == 'A' as i32 {
            0 as i32
        } else if 101 as i32 == 'B' as i32 {
            1 as i32
        } else if 101 as i32 == 'C' as i32 {
            2 as i32
        } else if 101 as i32 == 'D' as i32 {
            3 as i32
        } else if 101 as i32 == 'E' as i32 {
            4 as i32
        } else if 101 as i32 == 'F' as i32 {
            5 as i32
        } else if 101 as i32 == 'G' as i32 {
            6 as i32
        } else if 101 as i32 == 'H' as i32 {
            7 as i32
        } else if 101 as i32 == 'I' as i32 {
            8 as i32
        } else if 101 as i32 == 'J' as i32 {
            9 as i32
        } else if 101 as i32 == 'K' as i32 {
            10 as i32
        } else if 101 as i32 == 'L' as i32 {
            11 as i32
        } else if 101 as i32 == 'M' as i32 {
            12 as i32
        } else if 101 as i32 == 'N' as i32 {
            13 as i32
        } else if 101 as i32 == 'O' as i32 {
            14 as i32
        } else if 101 as i32 == 'P' as i32 {
            15 as i32
        } else if 101 as i32 == 'Q' as i32 {
            16 as i32
        } else if 101 as i32 == 'R' as i32 {
            17 as i32
        } else if 101 as i32 == 'S' as i32 {
            18 as i32
        } else if 101 as i32 == 'T' as i32 {
            19 as i32
        } else if 101 as i32 == 'U' as i32 {
            20 as i32
        } else if 101 as i32 == 'V' as i32 {
            21 as i32
        } else if 101 as i32 == 'W' as i32 {
            22 as i32
        } else if 101 as i32 == 'X' as i32 {
            23 as i32
        } else if 101 as i32 == 'Y' as i32 {
            24 as i32
        } else if 101 as i32 == 'Z' as i32 {
            25 as i32
        } else if 101 as i32 == '2' as i32 {
            26 as i32
        } else if 101 as i32 == '3' as i32 {
            27 as i32
        } else if 101 as i32 == '4' as i32 {
            28 as i32
        } else if 101 as i32 == '5' as i32 {
            29 as i32
        } else if 101 as i32 == '6' as i32 {
            30 as i32
        } else if 101 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 102 as i32 == 'A' as i32 {
            0 as i32
        } else if 102 as i32 == 'B' as i32 {
            1 as i32
        } else if 102 as i32 == 'C' as i32 {
            2 as i32
        } else if 102 as i32 == 'D' as i32 {
            3 as i32
        } else if 102 as i32 == 'E' as i32 {
            4 as i32
        } else if 102 as i32 == 'F' as i32 {
            5 as i32
        } else if 102 as i32 == 'G' as i32 {
            6 as i32
        } else if 102 as i32 == 'H' as i32 {
            7 as i32
        } else if 102 as i32 == 'I' as i32 {
            8 as i32
        } else if 102 as i32 == 'J' as i32 {
            9 as i32
        } else if 102 as i32 == 'K' as i32 {
            10 as i32
        } else if 102 as i32 == 'L' as i32 {
            11 as i32
        } else if 102 as i32 == 'M' as i32 {
            12 as i32
        } else if 102 as i32 == 'N' as i32 {
            13 as i32
        } else if 102 as i32 == 'O' as i32 {
            14 as i32
        } else if 102 as i32 == 'P' as i32 {
            15 as i32
        } else if 102 as i32 == 'Q' as i32 {
            16 as i32
        } else if 102 as i32 == 'R' as i32 {
            17 as i32
        } else if 102 as i32 == 'S' as i32 {
            18 as i32
        } else if 102 as i32 == 'T' as i32 {
            19 as i32
        } else if 102 as i32 == 'U' as i32 {
            20 as i32
        } else if 102 as i32 == 'V' as i32 {
            21 as i32
        } else if 102 as i32 == 'W' as i32 {
            22 as i32
        } else if 102 as i32 == 'X' as i32 {
            23 as i32
        } else if 102 as i32 == 'Y' as i32 {
            24 as i32
        } else if 102 as i32 == 'Z' as i32 {
            25 as i32
        } else if 102 as i32 == '2' as i32 {
            26 as i32
        } else if 102 as i32 == '3' as i32 {
            27 as i32
        } else if 102 as i32 == '4' as i32 {
            28 as i32
        } else if 102 as i32 == '5' as i32 {
            29 as i32
        } else if 102 as i32 == '6' as i32 {
            30 as i32
        } else if 102 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 103 as i32 == 'A' as i32 {
            0 as i32
        } else if 103 as i32 == 'B' as i32 {
            1 as i32
        } else if 103 as i32 == 'C' as i32 {
            2 as i32
        } else if 103 as i32 == 'D' as i32 {
            3 as i32
        } else if 103 as i32 == 'E' as i32 {
            4 as i32
        } else if 103 as i32 == 'F' as i32 {
            5 as i32
        } else if 103 as i32 == 'G' as i32 {
            6 as i32
        } else if 103 as i32 == 'H' as i32 {
            7 as i32
        } else if 103 as i32 == 'I' as i32 {
            8 as i32
        } else if 103 as i32 == 'J' as i32 {
            9 as i32
        } else if 103 as i32 == 'K' as i32 {
            10 as i32
        } else if 103 as i32 == 'L' as i32 {
            11 as i32
        } else if 103 as i32 == 'M' as i32 {
            12 as i32
        } else if 103 as i32 == 'N' as i32 {
            13 as i32
        } else if 103 as i32 == 'O' as i32 {
            14 as i32
        } else if 103 as i32 == 'P' as i32 {
            15 as i32
        } else if 103 as i32 == 'Q' as i32 {
            16 as i32
        } else if 103 as i32 == 'R' as i32 {
            17 as i32
        } else if 103 as i32 == 'S' as i32 {
            18 as i32
        } else if 103 as i32 == 'T' as i32 {
            19 as i32
        } else if 103 as i32 == 'U' as i32 {
            20 as i32
        } else if 103 as i32 == 'V' as i32 {
            21 as i32
        } else if 103 as i32 == 'W' as i32 {
            22 as i32
        } else if 103 as i32 == 'X' as i32 {
            23 as i32
        } else if 103 as i32 == 'Y' as i32 {
            24 as i32
        } else if 103 as i32 == 'Z' as i32 {
            25 as i32
        } else if 103 as i32 == '2' as i32 {
            26 as i32
        } else if 103 as i32 == '3' as i32 {
            27 as i32
        } else if 103 as i32 == '4' as i32 {
            28 as i32
        } else if 103 as i32 == '5' as i32 {
            29 as i32
        } else if 103 as i32 == '6' as i32 {
            30 as i32
        } else if 103 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 104 as i32 == 'A' as i32 {
            0 as i32
        } else if 104 as i32 == 'B' as i32 {
            1 as i32
        } else if 104 as i32 == 'C' as i32 {
            2 as i32
        } else if 104 as i32 == 'D' as i32 {
            3 as i32
        } else if 104 as i32 == 'E' as i32 {
            4 as i32
        } else if 104 as i32 == 'F' as i32 {
            5 as i32
        } else if 104 as i32 == 'G' as i32 {
            6 as i32
        } else if 104 as i32 == 'H' as i32 {
            7 as i32
        } else if 104 as i32 == 'I' as i32 {
            8 as i32
        } else if 104 as i32 == 'J' as i32 {
            9 as i32
        } else if 104 as i32 == 'K' as i32 {
            10 as i32
        } else if 104 as i32 == 'L' as i32 {
            11 as i32
        } else if 104 as i32 == 'M' as i32 {
            12 as i32
        } else if 104 as i32 == 'N' as i32 {
            13 as i32
        } else if 104 as i32 == 'O' as i32 {
            14 as i32
        } else if 104 as i32 == 'P' as i32 {
            15 as i32
        } else if 104 as i32 == 'Q' as i32 {
            16 as i32
        } else if 104 as i32 == 'R' as i32 {
            17 as i32
        } else if 104 as i32 == 'S' as i32 {
            18 as i32
        } else if 104 as i32 == 'T' as i32 {
            19 as i32
        } else if 104 as i32 == 'U' as i32 {
            20 as i32
        } else if 104 as i32 == 'V' as i32 {
            21 as i32
        } else if 104 as i32 == 'W' as i32 {
            22 as i32
        } else if 104 as i32 == 'X' as i32 {
            23 as i32
        } else if 104 as i32 == 'Y' as i32 {
            24 as i32
        } else if 104 as i32 == 'Z' as i32 {
            25 as i32
        } else if 104 as i32 == '2' as i32 {
            26 as i32
        } else if 104 as i32 == '3' as i32 {
            27 as i32
        } else if 104 as i32 == '4' as i32 {
            28 as i32
        } else if 104 as i32 == '5' as i32 {
            29 as i32
        } else if 104 as i32 == '6' as i32 {
            30 as i32
        } else if 104 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 105 as i32 == 'A' as i32 {
            0 as i32
        } else if 105 as i32 == 'B' as i32 {
            1 as i32
        } else if 105 as i32 == 'C' as i32 {
            2 as i32
        } else if 105 as i32 == 'D' as i32 {
            3 as i32
        } else if 105 as i32 == 'E' as i32 {
            4 as i32
        } else if 105 as i32 == 'F' as i32 {
            5 as i32
        } else if 105 as i32 == 'G' as i32 {
            6 as i32
        } else if 105 as i32 == 'H' as i32 {
            7 as i32
        } else if 105 as i32 == 'I' as i32 {
            8 as i32
        } else if 105 as i32 == 'J' as i32 {
            9 as i32
        } else if 105 as i32 == 'K' as i32 {
            10 as i32
        } else if 105 as i32 == 'L' as i32 {
            11 as i32
        } else if 105 as i32 == 'M' as i32 {
            12 as i32
        } else if 105 as i32 == 'N' as i32 {
            13 as i32
        } else if 105 as i32 == 'O' as i32 {
            14 as i32
        } else if 105 as i32 == 'P' as i32 {
            15 as i32
        } else if 105 as i32 == 'Q' as i32 {
            16 as i32
        } else if 105 as i32 == 'R' as i32 {
            17 as i32
        } else if 105 as i32 == 'S' as i32 {
            18 as i32
        } else if 105 as i32 == 'T' as i32 {
            19 as i32
        } else if 105 as i32 == 'U' as i32 {
            20 as i32
        } else if 105 as i32 == 'V' as i32 {
            21 as i32
        } else if 105 as i32 == 'W' as i32 {
            22 as i32
        } else if 105 as i32 == 'X' as i32 {
            23 as i32
        } else if 105 as i32 == 'Y' as i32 {
            24 as i32
        } else if 105 as i32 == 'Z' as i32 {
            25 as i32
        } else if 105 as i32 == '2' as i32 {
            26 as i32
        } else if 105 as i32 == '3' as i32 {
            27 as i32
        } else if 105 as i32 == '4' as i32 {
            28 as i32
        } else if 105 as i32 == '5' as i32 {
            29 as i32
        } else if 105 as i32 == '6' as i32 {
            30 as i32
        } else if 105 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 106 as i32 == 'A' as i32 {
            0 as i32
        } else if 106 as i32 == 'B' as i32 {
            1 as i32
        } else if 106 as i32 == 'C' as i32 {
            2 as i32
        } else if 106 as i32 == 'D' as i32 {
            3 as i32
        } else if 106 as i32 == 'E' as i32 {
            4 as i32
        } else if 106 as i32 == 'F' as i32 {
            5 as i32
        } else if 106 as i32 == 'G' as i32 {
            6 as i32
        } else if 106 as i32 == 'H' as i32 {
            7 as i32
        } else if 106 as i32 == 'I' as i32 {
            8 as i32
        } else if 106 as i32 == 'J' as i32 {
            9 as i32
        } else if 106 as i32 == 'K' as i32 {
            10 as i32
        } else if 106 as i32 == 'L' as i32 {
            11 as i32
        } else if 106 as i32 == 'M' as i32 {
            12 as i32
        } else if 106 as i32 == 'N' as i32 {
            13 as i32
        } else if 106 as i32 == 'O' as i32 {
            14 as i32
        } else if 106 as i32 == 'P' as i32 {
            15 as i32
        } else if 106 as i32 == 'Q' as i32 {
            16 as i32
        } else if 106 as i32 == 'R' as i32 {
            17 as i32
        } else if 106 as i32 == 'S' as i32 {
            18 as i32
        } else if 106 as i32 == 'T' as i32 {
            19 as i32
        } else if 106 as i32 == 'U' as i32 {
            20 as i32
        } else if 106 as i32 == 'V' as i32 {
            21 as i32
        } else if 106 as i32 == 'W' as i32 {
            22 as i32
        } else if 106 as i32 == 'X' as i32 {
            23 as i32
        } else if 106 as i32 == 'Y' as i32 {
            24 as i32
        } else if 106 as i32 == 'Z' as i32 {
            25 as i32
        } else if 106 as i32 == '2' as i32 {
            26 as i32
        } else if 106 as i32 == '3' as i32 {
            27 as i32
        } else if 106 as i32 == '4' as i32 {
            28 as i32
        } else if 106 as i32 == '5' as i32 {
            29 as i32
        } else if 106 as i32 == '6' as i32 {
            30 as i32
        } else if 106 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 107 as i32 == 'A' as i32 {
            0 as i32
        } else if 107 as i32 == 'B' as i32 {
            1 as i32
        } else if 107 as i32 == 'C' as i32 {
            2 as i32
        } else if 107 as i32 == 'D' as i32 {
            3 as i32
        } else if 107 as i32 == 'E' as i32 {
            4 as i32
        } else if 107 as i32 == 'F' as i32 {
            5 as i32
        } else if 107 as i32 == 'G' as i32 {
            6 as i32
        } else if 107 as i32 == 'H' as i32 {
            7 as i32
        } else if 107 as i32 == 'I' as i32 {
            8 as i32
        } else if 107 as i32 == 'J' as i32 {
            9 as i32
        } else if 107 as i32 == 'K' as i32 {
            10 as i32
        } else if 107 as i32 == 'L' as i32 {
            11 as i32
        } else if 107 as i32 == 'M' as i32 {
            12 as i32
        } else if 107 as i32 == 'N' as i32 {
            13 as i32
        } else if 107 as i32 == 'O' as i32 {
            14 as i32
        } else if 107 as i32 == 'P' as i32 {
            15 as i32
        } else if 107 as i32 == 'Q' as i32 {
            16 as i32
        } else if 107 as i32 == 'R' as i32 {
            17 as i32
        } else if 107 as i32 == 'S' as i32 {
            18 as i32
        } else if 107 as i32 == 'T' as i32 {
            19 as i32
        } else if 107 as i32 == 'U' as i32 {
            20 as i32
        } else if 107 as i32 == 'V' as i32 {
            21 as i32
        } else if 107 as i32 == 'W' as i32 {
            22 as i32
        } else if 107 as i32 == 'X' as i32 {
            23 as i32
        } else if 107 as i32 == 'Y' as i32 {
            24 as i32
        } else if 107 as i32 == 'Z' as i32 {
            25 as i32
        } else if 107 as i32 == '2' as i32 {
            26 as i32
        } else if 107 as i32 == '3' as i32 {
            27 as i32
        } else if 107 as i32 == '4' as i32 {
            28 as i32
        } else if 107 as i32 == '5' as i32 {
            29 as i32
        } else if 107 as i32 == '6' as i32 {
            30 as i32
        } else if 107 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 108 as i32 == 'A' as i32 {
            0 as i32
        } else if 108 as i32 == 'B' as i32 {
            1 as i32
        } else if 108 as i32 == 'C' as i32 {
            2 as i32
        } else if 108 as i32 == 'D' as i32 {
            3 as i32
        } else if 108 as i32 == 'E' as i32 {
            4 as i32
        } else if 108 as i32 == 'F' as i32 {
            5 as i32
        } else if 108 as i32 == 'G' as i32 {
            6 as i32
        } else if 108 as i32 == 'H' as i32 {
            7 as i32
        } else if 108 as i32 == 'I' as i32 {
            8 as i32
        } else if 108 as i32 == 'J' as i32 {
            9 as i32
        } else if 108 as i32 == 'K' as i32 {
            10 as i32
        } else if 108 as i32 == 'L' as i32 {
            11 as i32
        } else if 108 as i32 == 'M' as i32 {
            12 as i32
        } else if 108 as i32 == 'N' as i32 {
            13 as i32
        } else if 108 as i32 == 'O' as i32 {
            14 as i32
        } else if 108 as i32 == 'P' as i32 {
            15 as i32
        } else if 108 as i32 == 'Q' as i32 {
            16 as i32
        } else if 108 as i32 == 'R' as i32 {
            17 as i32
        } else if 108 as i32 == 'S' as i32 {
            18 as i32
        } else if 108 as i32 == 'T' as i32 {
            19 as i32
        } else if 108 as i32 == 'U' as i32 {
            20 as i32
        } else if 108 as i32 == 'V' as i32 {
            21 as i32
        } else if 108 as i32 == 'W' as i32 {
            22 as i32
        } else if 108 as i32 == 'X' as i32 {
            23 as i32
        } else if 108 as i32 == 'Y' as i32 {
            24 as i32
        } else if 108 as i32 == 'Z' as i32 {
            25 as i32
        } else if 108 as i32 == '2' as i32 {
            26 as i32
        } else if 108 as i32 == '3' as i32 {
            27 as i32
        } else if 108 as i32 == '4' as i32 {
            28 as i32
        } else if 108 as i32 == '5' as i32 {
            29 as i32
        } else if 108 as i32 == '6' as i32 {
            30 as i32
        } else if 108 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 109 as i32 == 'A' as i32 {
            0 as i32
        } else if 109 as i32 == 'B' as i32 {
            1 as i32
        } else if 109 as i32 == 'C' as i32 {
            2 as i32
        } else if 109 as i32 == 'D' as i32 {
            3 as i32
        } else if 109 as i32 == 'E' as i32 {
            4 as i32
        } else if 109 as i32 == 'F' as i32 {
            5 as i32
        } else if 109 as i32 == 'G' as i32 {
            6 as i32
        } else if 109 as i32 == 'H' as i32 {
            7 as i32
        } else if 109 as i32 == 'I' as i32 {
            8 as i32
        } else if 109 as i32 == 'J' as i32 {
            9 as i32
        } else if 109 as i32 == 'K' as i32 {
            10 as i32
        } else if 109 as i32 == 'L' as i32 {
            11 as i32
        } else if 109 as i32 == 'M' as i32 {
            12 as i32
        } else if 109 as i32 == 'N' as i32 {
            13 as i32
        } else if 109 as i32 == 'O' as i32 {
            14 as i32
        } else if 109 as i32 == 'P' as i32 {
            15 as i32
        } else if 109 as i32 == 'Q' as i32 {
            16 as i32
        } else if 109 as i32 == 'R' as i32 {
            17 as i32
        } else if 109 as i32 == 'S' as i32 {
            18 as i32
        } else if 109 as i32 == 'T' as i32 {
            19 as i32
        } else if 109 as i32 == 'U' as i32 {
            20 as i32
        } else if 109 as i32 == 'V' as i32 {
            21 as i32
        } else if 109 as i32 == 'W' as i32 {
            22 as i32
        } else if 109 as i32 == 'X' as i32 {
            23 as i32
        } else if 109 as i32 == 'Y' as i32 {
            24 as i32
        } else if 109 as i32 == 'Z' as i32 {
            25 as i32
        } else if 109 as i32 == '2' as i32 {
            26 as i32
        } else if 109 as i32 == '3' as i32 {
            27 as i32
        } else if 109 as i32 == '4' as i32 {
            28 as i32
        } else if 109 as i32 == '5' as i32 {
            29 as i32
        } else if 109 as i32 == '6' as i32 {
            30 as i32
        } else if 109 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 110 as i32 == 'A' as i32 {
            0 as i32
        } else if 110 as i32 == 'B' as i32 {
            1 as i32
        } else if 110 as i32 == 'C' as i32 {
            2 as i32
        } else if 110 as i32 == 'D' as i32 {
            3 as i32
        } else if 110 as i32 == 'E' as i32 {
            4 as i32
        } else if 110 as i32 == 'F' as i32 {
            5 as i32
        } else if 110 as i32 == 'G' as i32 {
            6 as i32
        } else if 110 as i32 == 'H' as i32 {
            7 as i32
        } else if 110 as i32 == 'I' as i32 {
            8 as i32
        } else if 110 as i32 == 'J' as i32 {
            9 as i32
        } else if 110 as i32 == 'K' as i32 {
            10 as i32
        } else if 110 as i32 == 'L' as i32 {
            11 as i32
        } else if 110 as i32 == 'M' as i32 {
            12 as i32
        } else if 110 as i32 == 'N' as i32 {
            13 as i32
        } else if 110 as i32 == 'O' as i32 {
            14 as i32
        } else if 110 as i32 == 'P' as i32 {
            15 as i32
        } else if 110 as i32 == 'Q' as i32 {
            16 as i32
        } else if 110 as i32 == 'R' as i32 {
            17 as i32
        } else if 110 as i32 == 'S' as i32 {
            18 as i32
        } else if 110 as i32 == 'T' as i32 {
            19 as i32
        } else if 110 as i32 == 'U' as i32 {
            20 as i32
        } else if 110 as i32 == 'V' as i32 {
            21 as i32
        } else if 110 as i32 == 'W' as i32 {
            22 as i32
        } else if 110 as i32 == 'X' as i32 {
            23 as i32
        } else if 110 as i32 == 'Y' as i32 {
            24 as i32
        } else if 110 as i32 == 'Z' as i32 {
            25 as i32
        } else if 110 as i32 == '2' as i32 {
            26 as i32
        } else if 110 as i32 == '3' as i32 {
            27 as i32
        } else if 110 as i32 == '4' as i32 {
            28 as i32
        } else if 110 as i32 == '5' as i32 {
            29 as i32
        } else if 110 as i32 == '6' as i32 {
            30 as i32
        } else if 110 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 111 as i32 == 'A' as i32 {
            0 as i32
        } else if 111 as i32 == 'B' as i32 {
            1 as i32
        } else if 111 as i32 == 'C' as i32 {
            2 as i32
        } else if 111 as i32 == 'D' as i32 {
            3 as i32
        } else if 111 as i32 == 'E' as i32 {
            4 as i32
        } else if 111 as i32 == 'F' as i32 {
            5 as i32
        } else if 111 as i32 == 'G' as i32 {
            6 as i32
        } else if 111 as i32 == 'H' as i32 {
            7 as i32
        } else if 111 as i32 == 'I' as i32 {
            8 as i32
        } else if 111 as i32 == 'J' as i32 {
            9 as i32
        } else if 111 as i32 == 'K' as i32 {
            10 as i32
        } else if 111 as i32 == 'L' as i32 {
            11 as i32
        } else if 111 as i32 == 'M' as i32 {
            12 as i32
        } else if 111 as i32 == 'N' as i32 {
            13 as i32
        } else if 111 as i32 == 'O' as i32 {
            14 as i32
        } else if 111 as i32 == 'P' as i32 {
            15 as i32
        } else if 111 as i32 == 'Q' as i32 {
            16 as i32
        } else if 111 as i32 == 'R' as i32 {
            17 as i32
        } else if 111 as i32 == 'S' as i32 {
            18 as i32
        } else if 111 as i32 == 'T' as i32 {
            19 as i32
        } else if 111 as i32 == 'U' as i32 {
            20 as i32
        } else if 111 as i32 == 'V' as i32 {
            21 as i32
        } else if 111 as i32 == 'W' as i32 {
            22 as i32
        } else if 111 as i32 == 'X' as i32 {
            23 as i32
        } else if 111 as i32 == 'Y' as i32 {
            24 as i32
        } else if 111 as i32 == 'Z' as i32 {
            25 as i32
        } else if 111 as i32 == '2' as i32 {
            26 as i32
        } else if 111 as i32 == '3' as i32 {
            27 as i32
        } else if 111 as i32 == '4' as i32 {
            28 as i32
        } else if 111 as i32 == '5' as i32 {
            29 as i32
        } else if 111 as i32 == '6' as i32 {
            30 as i32
        } else if 111 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 112 as i32 == 'A' as i32 {
            0 as i32
        } else if 112 as i32 == 'B' as i32 {
            1 as i32
        } else if 112 as i32 == 'C' as i32 {
            2 as i32
        } else if 112 as i32 == 'D' as i32 {
            3 as i32
        } else if 112 as i32 == 'E' as i32 {
            4 as i32
        } else if 112 as i32 == 'F' as i32 {
            5 as i32
        } else if 112 as i32 == 'G' as i32 {
            6 as i32
        } else if 112 as i32 == 'H' as i32 {
            7 as i32
        } else if 112 as i32 == 'I' as i32 {
            8 as i32
        } else if 112 as i32 == 'J' as i32 {
            9 as i32
        } else if 112 as i32 == 'K' as i32 {
            10 as i32
        } else if 112 as i32 == 'L' as i32 {
            11 as i32
        } else if 112 as i32 == 'M' as i32 {
            12 as i32
        } else if 112 as i32 == 'N' as i32 {
            13 as i32
        } else if 112 as i32 == 'O' as i32 {
            14 as i32
        } else if 112 as i32 == 'P' as i32 {
            15 as i32
        } else if 112 as i32 == 'Q' as i32 {
            16 as i32
        } else if 112 as i32 == 'R' as i32 {
            17 as i32
        } else if 112 as i32 == 'S' as i32 {
            18 as i32
        } else if 112 as i32 == 'T' as i32 {
            19 as i32
        } else if 112 as i32 == 'U' as i32 {
            20 as i32
        } else if 112 as i32 == 'V' as i32 {
            21 as i32
        } else if 112 as i32 == 'W' as i32 {
            22 as i32
        } else if 112 as i32 == 'X' as i32 {
            23 as i32
        } else if 112 as i32 == 'Y' as i32 {
            24 as i32
        } else if 112 as i32 == 'Z' as i32 {
            25 as i32
        } else if 112 as i32 == '2' as i32 {
            26 as i32
        } else if 112 as i32 == '3' as i32 {
            27 as i32
        } else if 112 as i32 == '4' as i32 {
            28 as i32
        } else if 112 as i32 == '5' as i32 {
            29 as i32
        } else if 112 as i32 == '6' as i32 {
            30 as i32
        } else if 112 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 113 as i32 == 'A' as i32 {
            0 as i32
        } else if 113 as i32 == 'B' as i32 {
            1 as i32
        } else if 113 as i32 == 'C' as i32 {
            2 as i32
        } else if 113 as i32 == 'D' as i32 {
            3 as i32
        } else if 113 as i32 == 'E' as i32 {
            4 as i32
        } else if 113 as i32 == 'F' as i32 {
            5 as i32
        } else if 113 as i32 == 'G' as i32 {
            6 as i32
        } else if 113 as i32 == 'H' as i32 {
            7 as i32
        } else if 113 as i32 == 'I' as i32 {
            8 as i32
        } else if 113 as i32 == 'J' as i32 {
            9 as i32
        } else if 113 as i32 == 'K' as i32 {
            10 as i32
        } else if 113 as i32 == 'L' as i32 {
            11 as i32
        } else if 113 as i32 == 'M' as i32 {
            12 as i32
        } else if 113 as i32 == 'N' as i32 {
            13 as i32
        } else if 113 as i32 == 'O' as i32 {
            14 as i32
        } else if 113 as i32 == 'P' as i32 {
            15 as i32
        } else if 113 as i32 == 'Q' as i32 {
            16 as i32
        } else if 113 as i32 == 'R' as i32 {
            17 as i32
        } else if 113 as i32 == 'S' as i32 {
            18 as i32
        } else if 113 as i32 == 'T' as i32 {
            19 as i32
        } else if 113 as i32 == 'U' as i32 {
            20 as i32
        } else if 113 as i32 == 'V' as i32 {
            21 as i32
        } else if 113 as i32 == 'W' as i32 {
            22 as i32
        } else if 113 as i32 == 'X' as i32 {
            23 as i32
        } else if 113 as i32 == 'Y' as i32 {
            24 as i32
        } else if 113 as i32 == 'Z' as i32 {
            25 as i32
        } else if 113 as i32 == '2' as i32 {
            26 as i32
        } else if 113 as i32 == '3' as i32 {
            27 as i32
        } else if 113 as i32 == '4' as i32 {
            28 as i32
        } else if 113 as i32 == '5' as i32 {
            29 as i32
        } else if 113 as i32 == '6' as i32 {
            30 as i32
        } else if 113 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 114 as i32 == 'A' as i32 {
            0 as i32
        } else if 114 as i32 == 'B' as i32 {
            1 as i32
        } else if 114 as i32 == 'C' as i32 {
            2 as i32
        } else if 114 as i32 == 'D' as i32 {
            3 as i32
        } else if 114 as i32 == 'E' as i32 {
            4 as i32
        } else if 114 as i32 == 'F' as i32 {
            5 as i32
        } else if 114 as i32 == 'G' as i32 {
            6 as i32
        } else if 114 as i32 == 'H' as i32 {
            7 as i32
        } else if 114 as i32 == 'I' as i32 {
            8 as i32
        } else if 114 as i32 == 'J' as i32 {
            9 as i32
        } else if 114 as i32 == 'K' as i32 {
            10 as i32
        } else if 114 as i32 == 'L' as i32 {
            11 as i32
        } else if 114 as i32 == 'M' as i32 {
            12 as i32
        } else if 114 as i32 == 'N' as i32 {
            13 as i32
        } else if 114 as i32 == 'O' as i32 {
            14 as i32
        } else if 114 as i32 == 'P' as i32 {
            15 as i32
        } else if 114 as i32 == 'Q' as i32 {
            16 as i32
        } else if 114 as i32 == 'R' as i32 {
            17 as i32
        } else if 114 as i32 == 'S' as i32 {
            18 as i32
        } else if 114 as i32 == 'T' as i32 {
            19 as i32
        } else if 114 as i32 == 'U' as i32 {
            20 as i32
        } else if 114 as i32 == 'V' as i32 {
            21 as i32
        } else if 114 as i32 == 'W' as i32 {
            22 as i32
        } else if 114 as i32 == 'X' as i32 {
            23 as i32
        } else if 114 as i32 == 'Y' as i32 {
            24 as i32
        } else if 114 as i32 == 'Z' as i32 {
            25 as i32
        } else if 114 as i32 == '2' as i32 {
            26 as i32
        } else if 114 as i32 == '3' as i32 {
            27 as i32
        } else if 114 as i32 == '4' as i32 {
            28 as i32
        } else if 114 as i32 == '5' as i32 {
            29 as i32
        } else if 114 as i32 == '6' as i32 {
            30 as i32
        } else if 114 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 115 as i32 == 'A' as i32 {
            0 as i32
        } else if 115 as i32 == 'B' as i32 {
            1 as i32
        } else if 115 as i32 == 'C' as i32 {
            2 as i32
        } else if 115 as i32 == 'D' as i32 {
            3 as i32
        } else if 115 as i32 == 'E' as i32 {
            4 as i32
        } else if 115 as i32 == 'F' as i32 {
            5 as i32
        } else if 115 as i32 == 'G' as i32 {
            6 as i32
        } else if 115 as i32 == 'H' as i32 {
            7 as i32
        } else if 115 as i32 == 'I' as i32 {
            8 as i32
        } else if 115 as i32 == 'J' as i32 {
            9 as i32
        } else if 115 as i32 == 'K' as i32 {
            10 as i32
        } else if 115 as i32 == 'L' as i32 {
            11 as i32
        } else if 115 as i32 == 'M' as i32 {
            12 as i32
        } else if 115 as i32 == 'N' as i32 {
            13 as i32
        } else if 115 as i32 == 'O' as i32 {
            14 as i32
        } else if 115 as i32 == 'P' as i32 {
            15 as i32
        } else if 115 as i32 == 'Q' as i32 {
            16 as i32
        } else if 115 as i32 == 'R' as i32 {
            17 as i32
        } else if 115 as i32 == 'S' as i32 {
            18 as i32
        } else if 115 as i32 == 'T' as i32 {
            19 as i32
        } else if 115 as i32 == 'U' as i32 {
            20 as i32
        } else if 115 as i32 == 'V' as i32 {
            21 as i32
        } else if 115 as i32 == 'W' as i32 {
            22 as i32
        } else if 115 as i32 == 'X' as i32 {
            23 as i32
        } else if 115 as i32 == 'Y' as i32 {
            24 as i32
        } else if 115 as i32 == 'Z' as i32 {
            25 as i32
        } else if 115 as i32 == '2' as i32 {
            26 as i32
        } else if 115 as i32 == '3' as i32 {
            27 as i32
        } else if 115 as i32 == '4' as i32 {
            28 as i32
        } else if 115 as i32 == '5' as i32 {
            29 as i32
        } else if 115 as i32 == '6' as i32 {
            30 as i32
        } else if 115 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 116 as i32 == 'A' as i32 {
            0 as i32
        } else if 116 as i32 == 'B' as i32 {
            1 as i32
        } else if 116 as i32 == 'C' as i32 {
            2 as i32
        } else if 116 as i32 == 'D' as i32 {
            3 as i32
        } else if 116 as i32 == 'E' as i32 {
            4 as i32
        } else if 116 as i32 == 'F' as i32 {
            5 as i32
        } else if 116 as i32 == 'G' as i32 {
            6 as i32
        } else if 116 as i32 == 'H' as i32 {
            7 as i32
        } else if 116 as i32 == 'I' as i32 {
            8 as i32
        } else if 116 as i32 == 'J' as i32 {
            9 as i32
        } else if 116 as i32 == 'K' as i32 {
            10 as i32
        } else if 116 as i32 == 'L' as i32 {
            11 as i32
        } else if 116 as i32 == 'M' as i32 {
            12 as i32
        } else if 116 as i32 == 'N' as i32 {
            13 as i32
        } else if 116 as i32 == 'O' as i32 {
            14 as i32
        } else if 116 as i32 == 'P' as i32 {
            15 as i32
        } else if 116 as i32 == 'Q' as i32 {
            16 as i32
        } else if 116 as i32 == 'R' as i32 {
            17 as i32
        } else if 116 as i32 == 'S' as i32 {
            18 as i32
        } else if 116 as i32 == 'T' as i32 {
            19 as i32
        } else if 116 as i32 == 'U' as i32 {
            20 as i32
        } else if 116 as i32 == 'V' as i32 {
            21 as i32
        } else if 116 as i32 == 'W' as i32 {
            22 as i32
        } else if 116 as i32 == 'X' as i32 {
            23 as i32
        } else if 116 as i32 == 'Y' as i32 {
            24 as i32
        } else if 116 as i32 == 'Z' as i32 {
            25 as i32
        } else if 116 as i32 == '2' as i32 {
            26 as i32
        } else if 116 as i32 == '3' as i32 {
            27 as i32
        } else if 116 as i32 == '4' as i32 {
            28 as i32
        } else if 116 as i32 == '5' as i32 {
            29 as i32
        } else if 116 as i32 == '6' as i32 {
            30 as i32
        } else if 116 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 117 as i32 == 'A' as i32 {
            0 as i32
        } else if 117 as i32 == 'B' as i32 {
            1 as i32
        } else if 117 as i32 == 'C' as i32 {
            2 as i32
        } else if 117 as i32 == 'D' as i32 {
            3 as i32
        } else if 117 as i32 == 'E' as i32 {
            4 as i32
        } else if 117 as i32 == 'F' as i32 {
            5 as i32
        } else if 117 as i32 == 'G' as i32 {
            6 as i32
        } else if 117 as i32 == 'H' as i32 {
            7 as i32
        } else if 117 as i32 == 'I' as i32 {
            8 as i32
        } else if 117 as i32 == 'J' as i32 {
            9 as i32
        } else if 117 as i32 == 'K' as i32 {
            10 as i32
        } else if 117 as i32 == 'L' as i32 {
            11 as i32
        } else if 117 as i32 == 'M' as i32 {
            12 as i32
        } else if 117 as i32 == 'N' as i32 {
            13 as i32
        } else if 117 as i32 == 'O' as i32 {
            14 as i32
        } else if 117 as i32 == 'P' as i32 {
            15 as i32
        } else if 117 as i32 == 'Q' as i32 {
            16 as i32
        } else if 117 as i32 == 'R' as i32 {
            17 as i32
        } else if 117 as i32 == 'S' as i32 {
            18 as i32
        } else if 117 as i32 == 'T' as i32 {
            19 as i32
        } else if 117 as i32 == 'U' as i32 {
            20 as i32
        } else if 117 as i32 == 'V' as i32 {
            21 as i32
        } else if 117 as i32 == 'W' as i32 {
            22 as i32
        } else if 117 as i32 == 'X' as i32 {
            23 as i32
        } else if 117 as i32 == 'Y' as i32 {
            24 as i32
        } else if 117 as i32 == 'Z' as i32 {
            25 as i32
        } else if 117 as i32 == '2' as i32 {
            26 as i32
        } else if 117 as i32 == '3' as i32 {
            27 as i32
        } else if 117 as i32 == '4' as i32 {
            28 as i32
        } else if 117 as i32 == '5' as i32 {
            29 as i32
        } else if 117 as i32 == '6' as i32 {
            30 as i32
        } else if 117 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 118 as i32 == 'A' as i32 {
            0 as i32
        } else if 118 as i32 == 'B' as i32 {
            1 as i32
        } else if 118 as i32 == 'C' as i32 {
            2 as i32
        } else if 118 as i32 == 'D' as i32 {
            3 as i32
        } else if 118 as i32 == 'E' as i32 {
            4 as i32
        } else if 118 as i32 == 'F' as i32 {
            5 as i32
        } else if 118 as i32 == 'G' as i32 {
            6 as i32
        } else if 118 as i32 == 'H' as i32 {
            7 as i32
        } else if 118 as i32 == 'I' as i32 {
            8 as i32
        } else if 118 as i32 == 'J' as i32 {
            9 as i32
        } else if 118 as i32 == 'K' as i32 {
            10 as i32
        } else if 118 as i32 == 'L' as i32 {
            11 as i32
        } else if 118 as i32 == 'M' as i32 {
            12 as i32
        } else if 118 as i32 == 'N' as i32 {
            13 as i32
        } else if 118 as i32 == 'O' as i32 {
            14 as i32
        } else if 118 as i32 == 'P' as i32 {
            15 as i32
        } else if 118 as i32 == 'Q' as i32 {
            16 as i32
        } else if 118 as i32 == 'R' as i32 {
            17 as i32
        } else if 118 as i32 == 'S' as i32 {
            18 as i32
        } else if 118 as i32 == 'T' as i32 {
            19 as i32
        } else if 118 as i32 == 'U' as i32 {
            20 as i32
        } else if 118 as i32 == 'V' as i32 {
            21 as i32
        } else if 118 as i32 == 'W' as i32 {
            22 as i32
        } else if 118 as i32 == 'X' as i32 {
            23 as i32
        } else if 118 as i32 == 'Y' as i32 {
            24 as i32
        } else if 118 as i32 == 'Z' as i32 {
            25 as i32
        } else if 118 as i32 == '2' as i32 {
            26 as i32
        } else if 118 as i32 == '3' as i32 {
            27 as i32
        } else if 118 as i32 == '4' as i32 {
            28 as i32
        } else if 118 as i32 == '5' as i32 {
            29 as i32
        } else if 118 as i32 == '6' as i32 {
            30 as i32
        } else if 118 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 119 as i32 == 'A' as i32 {
            0 as i32
        } else if 119 as i32 == 'B' as i32 {
            1 as i32
        } else if 119 as i32 == 'C' as i32 {
            2 as i32
        } else if 119 as i32 == 'D' as i32 {
            3 as i32
        } else if 119 as i32 == 'E' as i32 {
            4 as i32
        } else if 119 as i32 == 'F' as i32 {
            5 as i32
        } else if 119 as i32 == 'G' as i32 {
            6 as i32
        } else if 119 as i32 == 'H' as i32 {
            7 as i32
        } else if 119 as i32 == 'I' as i32 {
            8 as i32
        } else if 119 as i32 == 'J' as i32 {
            9 as i32
        } else if 119 as i32 == 'K' as i32 {
            10 as i32
        } else if 119 as i32 == 'L' as i32 {
            11 as i32
        } else if 119 as i32 == 'M' as i32 {
            12 as i32
        } else if 119 as i32 == 'N' as i32 {
            13 as i32
        } else if 119 as i32 == 'O' as i32 {
            14 as i32
        } else if 119 as i32 == 'P' as i32 {
            15 as i32
        } else if 119 as i32 == 'Q' as i32 {
            16 as i32
        } else if 119 as i32 == 'R' as i32 {
            17 as i32
        } else if 119 as i32 == 'S' as i32 {
            18 as i32
        } else if 119 as i32 == 'T' as i32 {
            19 as i32
        } else if 119 as i32 == 'U' as i32 {
            20 as i32
        } else if 119 as i32 == 'V' as i32 {
            21 as i32
        } else if 119 as i32 == 'W' as i32 {
            22 as i32
        } else if 119 as i32 == 'X' as i32 {
            23 as i32
        } else if 119 as i32 == 'Y' as i32 {
            24 as i32
        } else if 119 as i32 == 'Z' as i32 {
            25 as i32
        } else if 119 as i32 == '2' as i32 {
            26 as i32
        } else if 119 as i32 == '3' as i32 {
            27 as i32
        } else if 119 as i32 == '4' as i32 {
            28 as i32
        } else if 119 as i32 == '5' as i32 {
            29 as i32
        } else if 119 as i32 == '6' as i32 {
            30 as i32
        } else if 119 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 120 as i32 == 'A' as i32 {
            0 as i32
        } else if 120 as i32 == 'B' as i32 {
            1 as i32
        } else if 120 as i32 == 'C' as i32 {
            2 as i32
        } else if 120 as i32 == 'D' as i32 {
            3 as i32
        } else if 120 as i32 == 'E' as i32 {
            4 as i32
        } else if 120 as i32 == 'F' as i32 {
            5 as i32
        } else if 120 as i32 == 'G' as i32 {
            6 as i32
        } else if 120 as i32 == 'H' as i32 {
            7 as i32
        } else if 120 as i32 == 'I' as i32 {
            8 as i32
        } else if 120 as i32 == 'J' as i32 {
            9 as i32
        } else if 120 as i32 == 'K' as i32 {
            10 as i32
        } else if 120 as i32 == 'L' as i32 {
            11 as i32
        } else if 120 as i32 == 'M' as i32 {
            12 as i32
        } else if 120 as i32 == 'N' as i32 {
            13 as i32
        } else if 120 as i32 == 'O' as i32 {
            14 as i32
        } else if 120 as i32 == 'P' as i32 {
            15 as i32
        } else if 120 as i32 == 'Q' as i32 {
            16 as i32
        } else if 120 as i32 == 'R' as i32 {
            17 as i32
        } else if 120 as i32 == 'S' as i32 {
            18 as i32
        } else if 120 as i32 == 'T' as i32 {
            19 as i32
        } else if 120 as i32 == 'U' as i32 {
            20 as i32
        } else if 120 as i32 == 'V' as i32 {
            21 as i32
        } else if 120 as i32 == 'W' as i32 {
            22 as i32
        } else if 120 as i32 == 'X' as i32 {
            23 as i32
        } else if 120 as i32 == 'Y' as i32 {
            24 as i32
        } else if 120 as i32 == 'Z' as i32 {
            25 as i32
        } else if 120 as i32 == '2' as i32 {
            26 as i32
        } else if 120 as i32 == '3' as i32 {
            27 as i32
        } else if 120 as i32 == '4' as i32 {
            28 as i32
        } else if 120 as i32 == '5' as i32 {
            29 as i32
        } else if 120 as i32 == '6' as i32 {
            30 as i32
        } else if 120 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 121 as i32 == 'A' as i32 {
            0 as i32
        } else if 121 as i32 == 'B' as i32 {
            1 as i32
        } else if 121 as i32 == 'C' as i32 {
            2 as i32
        } else if 121 as i32 == 'D' as i32 {
            3 as i32
        } else if 121 as i32 == 'E' as i32 {
            4 as i32
        } else if 121 as i32 == 'F' as i32 {
            5 as i32
        } else if 121 as i32 == 'G' as i32 {
            6 as i32
        } else if 121 as i32 == 'H' as i32 {
            7 as i32
        } else if 121 as i32 == 'I' as i32 {
            8 as i32
        } else if 121 as i32 == 'J' as i32 {
            9 as i32
        } else if 121 as i32 == 'K' as i32 {
            10 as i32
        } else if 121 as i32 == 'L' as i32 {
            11 as i32
        } else if 121 as i32 == 'M' as i32 {
            12 as i32
        } else if 121 as i32 == 'N' as i32 {
            13 as i32
        } else if 121 as i32 == 'O' as i32 {
            14 as i32
        } else if 121 as i32 == 'P' as i32 {
            15 as i32
        } else if 121 as i32 == 'Q' as i32 {
            16 as i32
        } else if 121 as i32 == 'R' as i32 {
            17 as i32
        } else if 121 as i32 == 'S' as i32 {
            18 as i32
        } else if 121 as i32 == 'T' as i32 {
            19 as i32
        } else if 121 as i32 == 'U' as i32 {
            20 as i32
        } else if 121 as i32 == 'V' as i32 {
            21 as i32
        } else if 121 as i32 == 'W' as i32 {
            22 as i32
        } else if 121 as i32 == 'X' as i32 {
            23 as i32
        } else if 121 as i32 == 'Y' as i32 {
            24 as i32
        } else if 121 as i32 == 'Z' as i32 {
            25 as i32
        } else if 121 as i32 == '2' as i32 {
            26 as i32
        } else if 121 as i32 == '3' as i32 {
            27 as i32
        } else if 121 as i32 == '4' as i32 {
            28 as i32
        } else if 121 as i32 == '5' as i32 {
            29 as i32
        } else if 121 as i32 == '6' as i32 {
            30 as i32
        } else if 121 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 122 as i32 == 'A' as i32 {
            0 as i32
        } else if 122 as i32 == 'B' as i32 {
            1 as i32
        } else if 122 as i32 == 'C' as i32 {
            2 as i32
        } else if 122 as i32 == 'D' as i32 {
            3 as i32
        } else if 122 as i32 == 'E' as i32 {
            4 as i32
        } else if 122 as i32 == 'F' as i32 {
            5 as i32
        } else if 122 as i32 == 'G' as i32 {
            6 as i32
        } else if 122 as i32 == 'H' as i32 {
            7 as i32
        } else if 122 as i32 == 'I' as i32 {
            8 as i32
        } else if 122 as i32 == 'J' as i32 {
            9 as i32
        } else if 122 as i32 == 'K' as i32 {
            10 as i32
        } else if 122 as i32 == 'L' as i32 {
            11 as i32
        } else if 122 as i32 == 'M' as i32 {
            12 as i32
        } else if 122 as i32 == 'N' as i32 {
            13 as i32
        } else if 122 as i32 == 'O' as i32 {
            14 as i32
        } else if 122 as i32 == 'P' as i32 {
            15 as i32
        } else if 122 as i32 == 'Q' as i32 {
            16 as i32
        } else if 122 as i32 == 'R' as i32 {
            17 as i32
        } else if 122 as i32 == 'S' as i32 {
            18 as i32
        } else if 122 as i32 == 'T' as i32 {
            19 as i32
        } else if 122 as i32 == 'U' as i32 {
            20 as i32
        } else if 122 as i32 == 'V' as i32 {
            21 as i32
        } else if 122 as i32 == 'W' as i32 {
            22 as i32
        } else if 122 as i32 == 'X' as i32 {
            23 as i32
        } else if 122 as i32 == 'Y' as i32 {
            24 as i32
        } else if 122 as i32 == 'Z' as i32 {
            25 as i32
        } else if 122 as i32 == '2' as i32 {
            26 as i32
        } else if 122 as i32 == '3' as i32 {
            27 as i32
        } else if 122 as i32 == '4' as i32 {
            28 as i32
        } else if 122 as i32 == '5' as i32 {
            29 as i32
        } else if 122 as i32 == '6' as i32 {
            30 as i32
        } else if 122 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 123 as i32 == 'A' as i32 {
            0 as i32
        } else if 123 as i32 == 'B' as i32 {
            1 as i32
        } else if 123 as i32 == 'C' as i32 {
            2 as i32
        } else if 123 as i32 == 'D' as i32 {
            3 as i32
        } else if 123 as i32 == 'E' as i32 {
            4 as i32
        } else if 123 as i32 == 'F' as i32 {
            5 as i32
        } else if 123 as i32 == 'G' as i32 {
            6 as i32
        } else if 123 as i32 == 'H' as i32 {
            7 as i32
        } else if 123 as i32 == 'I' as i32 {
            8 as i32
        } else if 123 as i32 == 'J' as i32 {
            9 as i32
        } else if 123 as i32 == 'K' as i32 {
            10 as i32
        } else if 123 as i32 == 'L' as i32 {
            11 as i32
        } else if 123 as i32 == 'M' as i32 {
            12 as i32
        } else if 123 as i32 == 'N' as i32 {
            13 as i32
        } else if 123 as i32 == 'O' as i32 {
            14 as i32
        } else if 123 as i32 == 'P' as i32 {
            15 as i32
        } else if 123 as i32 == 'Q' as i32 {
            16 as i32
        } else if 123 as i32 == 'R' as i32 {
            17 as i32
        } else if 123 as i32 == 'S' as i32 {
            18 as i32
        } else if 123 as i32 == 'T' as i32 {
            19 as i32
        } else if 123 as i32 == 'U' as i32 {
            20 as i32
        } else if 123 as i32 == 'V' as i32 {
            21 as i32
        } else if 123 as i32 == 'W' as i32 {
            22 as i32
        } else if 123 as i32 == 'X' as i32 {
            23 as i32
        } else if 123 as i32 == 'Y' as i32 {
            24 as i32
        } else if 123 as i32 == 'Z' as i32 {
            25 as i32
        } else if 123 as i32 == '2' as i32 {
            26 as i32
        } else if 123 as i32 == '3' as i32 {
            27 as i32
        } else if 123 as i32 == '4' as i32 {
            28 as i32
        } else if 123 as i32 == '5' as i32 {
            29 as i32
        } else if 123 as i32 == '6' as i32 {
            30 as i32
        } else if 123 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 124 as i32 == 'A' as i32 {
            0 as i32
        } else if 124 as i32 == 'B' as i32 {
            1 as i32
        } else if 124 as i32 == 'C' as i32 {
            2 as i32
        } else if 124 as i32 == 'D' as i32 {
            3 as i32
        } else if 124 as i32 == 'E' as i32 {
            4 as i32
        } else if 124 as i32 == 'F' as i32 {
            5 as i32
        } else if 124 as i32 == 'G' as i32 {
            6 as i32
        } else if 124 as i32 == 'H' as i32 {
            7 as i32
        } else if 124 as i32 == 'I' as i32 {
            8 as i32
        } else if 124 as i32 == 'J' as i32 {
            9 as i32
        } else if 124 as i32 == 'K' as i32 {
            10 as i32
        } else if 124 as i32 == 'L' as i32 {
            11 as i32
        } else if 124 as i32 == 'M' as i32 {
            12 as i32
        } else if 124 as i32 == 'N' as i32 {
            13 as i32
        } else if 124 as i32 == 'O' as i32 {
            14 as i32
        } else if 124 as i32 == 'P' as i32 {
            15 as i32
        } else if 124 as i32 == 'Q' as i32 {
            16 as i32
        } else if 124 as i32 == 'R' as i32 {
            17 as i32
        } else if 124 as i32 == 'S' as i32 {
            18 as i32
        } else if 124 as i32 == 'T' as i32 {
            19 as i32
        } else if 124 as i32 == 'U' as i32 {
            20 as i32
        } else if 124 as i32 == 'V' as i32 {
            21 as i32
        } else if 124 as i32 == 'W' as i32 {
            22 as i32
        } else if 124 as i32 == 'X' as i32 {
            23 as i32
        } else if 124 as i32 == 'Y' as i32 {
            24 as i32
        } else if 124 as i32 == 'Z' as i32 {
            25 as i32
        } else if 124 as i32 == '2' as i32 {
            26 as i32
        } else if 124 as i32 == '3' as i32 {
            27 as i32
        } else if 124 as i32 == '4' as i32 {
            28 as i32
        } else if 124 as i32 == '5' as i32 {
            29 as i32
        } else if 124 as i32 == '6' as i32 {
            30 as i32
        } else if 124 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 125 as i32 == 'A' as i32 {
            0 as i32
        } else if 125 as i32 == 'B' as i32 {
            1 as i32
        } else if 125 as i32 == 'C' as i32 {
            2 as i32
        } else if 125 as i32 == 'D' as i32 {
            3 as i32
        } else if 125 as i32 == 'E' as i32 {
            4 as i32
        } else if 125 as i32 == 'F' as i32 {
            5 as i32
        } else if 125 as i32 == 'G' as i32 {
            6 as i32
        } else if 125 as i32 == 'H' as i32 {
            7 as i32
        } else if 125 as i32 == 'I' as i32 {
            8 as i32
        } else if 125 as i32 == 'J' as i32 {
            9 as i32
        } else if 125 as i32 == 'K' as i32 {
            10 as i32
        } else if 125 as i32 == 'L' as i32 {
            11 as i32
        } else if 125 as i32 == 'M' as i32 {
            12 as i32
        } else if 125 as i32 == 'N' as i32 {
            13 as i32
        } else if 125 as i32 == 'O' as i32 {
            14 as i32
        } else if 125 as i32 == 'P' as i32 {
            15 as i32
        } else if 125 as i32 == 'Q' as i32 {
            16 as i32
        } else if 125 as i32 == 'R' as i32 {
            17 as i32
        } else if 125 as i32 == 'S' as i32 {
            18 as i32
        } else if 125 as i32 == 'T' as i32 {
            19 as i32
        } else if 125 as i32 == 'U' as i32 {
            20 as i32
        } else if 125 as i32 == 'V' as i32 {
            21 as i32
        } else if 125 as i32 == 'W' as i32 {
            22 as i32
        } else if 125 as i32 == 'X' as i32 {
            23 as i32
        } else if 125 as i32 == 'Y' as i32 {
            24 as i32
        } else if 125 as i32 == 'Z' as i32 {
            25 as i32
        } else if 125 as i32 == '2' as i32 {
            26 as i32
        } else if 125 as i32 == '3' as i32 {
            27 as i32
        } else if 125 as i32 == '4' as i32 {
            28 as i32
        } else if 125 as i32 == '5' as i32 {
            29 as i32
        } else if 125 as i32 == '6' as i32 {
            30 as i32
        } else if 125 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 126 as i32 == 'A' as i32 {
            0 as i32
        } else if 126 as i32 == 'B' as i32 {
            1 as i32
        } else if 126 as i32 == 'C' as i32 {
            2 as i32
        } else if 126 as i32 == 'D' as i32 {
            3 as i32
        } else if 126 as i32 == 'E' as i32 {
            4 as i32
        } else if 126 as i32 == 'F' as i32 {
            5 as i32
        } else if 126 as i32 == 'G' as i32 {
            6 as i32
        } else if 126 as i32 == 'H' as i32 {
            7 as i32
        } else if 126 as i32 == 'I' as i32 {
            8 as i32
        } else if 126 as i32 == 'J' as i32 {
            9 as i32
        } else if 126 as i32 == 'K' as i32 {
            10 as i32
        } else if 126 as i32 == 'L' as i32 {
            11 as i32
        } else if 126 as i32 == 'M' as i32 {
            12 as i32
        } else if 126 as i32 == 'N' as i32 {
            13 as i32
        } else if 126 as i32 == 'O' as i32 {
            14 as i32
        } else if 126 as i32 == 'P' as i32 {
            15 as i32
        } else if 126 as i32 == 'Q' as i32 {
            16 as i32
        } else if 126 as i32 == 'R' as i32 {
            17 as i32
        } else if 126 as i32 == 'S' as i32 {
            18 as i32
        } else if 126 as i32 == 'T' as i32 {
            19 as i32
        } else if 126 as i32 == 'U' as i32 {
            20 as i32
        } else if 126 as i32 == 'V' as i32 {
            21 as i32
        } else if 126 as i32 == 'W' as i32 {
            22 as i32
        } else if 126 as i32 == 'X' as i32 {
            23 as i32
        } else if 126 as i32 == 'Y' as i32 {
            24 as i32
        } else if 126 as i32 == 'Z' as i32 {
            25 as i32
        } else if 126 as i32 == '2' as i32 {
            26 as i32
        } else if 126 as i32 == '3' as i32 {
            27 as i32
        } else if 126 as i32 == '4' as i32 {
            28 as i32
        } else if 126 as i32 == '5' as i32 {
            29 as i32
        } else if 126 as i32 == '6' as i32 {
            30 as i32
        } else if 126 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 127 as i32 == 'A' as i32 {
            0 as i32
        } else if 127 as i32 == 'B' as i32 {
            1 as i32
        } else if 127 as i32 == 'C' as i32 {
            2 as i32
        } else if 127 as i32 == 'D' as i32 {
            3 as i32
        } else if 127 as i32 == 'E' as i32 {
            4 as i32
        } else if 127 as i32 == 'F' as i32 {
            5 as i32
        } else if 127 as i32 == 'G' as i32 {
            6 as i32
        } else if 127 as i32 == 'H' as i32 {
            7 as i32
        } else if 127 as i32 == 'I' as i32 {
            8 as i32
        } else if 127 as i32 == 'J' as i32 {
            9 as i32
        } else if 127 as i32 == 'K' as i32 {
            10 as i32
        } else if 127 as i32 == 'L' as i32 {
            11 as i32
        } else if 127 as i32 == 'M' as i32 {
            12 as i32
        } else if 127 as i32 == 'N' as i32 {
            13 as i32
        } else if 127 as i32 == 'O' as i32 {
            14 as i32
        } else if 127 as i32 == 'P' as i32 {
            15 as i32
        } else if 127 as i32 == 'Q' as i32 {
            16 as i32
        } else if 127 as i32 == 'R' as i32 {
            17 as i32
        } else if 127 as i32 == 'S' as i32 {
            18 as i32
        } else if 127 as i32 == 'T' as i32 {
            19 as i32
        } else if 127 as i32 == 'U' as i32 {
            20 as i32
        } else if 127 as i32 == 'V' as i32 {
            21 as i32
        } else if 127 as i32 == 'W' as i32 {
            22 as i32
        } else if 127 as i32 == 'X' as i32 {
            23 as i32
        } else if 127 as i32 == 'Y' as i32 {
            24 as i32
        } else if 127 as i32 == 'Z' as i32 {
            25 as i32
        } else if 127 as i32 == '2' as i32 {
            26 as i32
        } else if 127 as i32 == '3' as i32 {
            27 as i32
        } else if 127 as i32 == '4' as i32 {
            28 as i32
        } else if 127 as i32 == '5' as i32 {
            29 as i32
        } else if 127 as i32 == '6' as i32 {
            30 as i32
        } else if 127 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 128 as i32 == 'A' as i32 {
            0 as i32
        } else if 128 as i32 == 'B' as i32 {
            1 as i32
        } else if 128 as i32 == 'C' as i32 {
            2 as i32
        } else if 128 as i32 == 'D' as i32 {
            3 as i32
        } else if 128 as i32 == 'E' as i32 {
            4 as i32
        } else if 128 as i32 == 'F' as i32 {
            5 as i32
        } else if 128 as i32 == 'G' as i32 {
            6 as i32
        } else if 128 as i32 == 'H' as i32 {
            7 as i32
        } else if 128 as i32 == 'I' as i32 {
            8 as i32
        } else if 128 as i32 == 'J' as i32 {
            9 as i32
        } else if 128 as i32 == 'K' as i32 {
            10 as i32
        } else if 128 as i32 == 'L' as i32 {
            11 as i32
        } else if 128 as i32 == 'M' as i32 {
            12 as i32
        } else if 128 as i32 == 'N' as i32 {
            13 as i32
        } else if 128 as i32 == 'O' as i32 {
            14 as i32
        } else if 128 as i32 == 'P' as i32 {
            15 as i32
        } else if 128 as i32 == 'Q' as i32 {
            16 as i32
        } else if 128 as i32 == 'R' as i32 {
            17 as i32
        } else if 128 as i32 == 'S' as i32 {
            18 as i32
        } else if 128 as i32 == 'T' as i32 {
            19 as i32
        } else if 128 as i32 == 'U' as i32 {
            20 as i32
        } else if 128 as i32 == 'V' as i32 {
            21 as i32
        } else if 128 as i32 == 'W' as i32 {
            22 as i32
        } else if 128 as i32 == 'X' as i32 {
            23 as i32
        } else if 128 as i32 == 'Y' as i32 {
            24 as i32
        } else if 128 as i32 == 'Z' as i32 {
            25 as i32
        } else if 128 as i32 == '2' as i32 {
            26 as i32
        } else if 128 as i32 == '3' as i32 {
            27 as i32
        } else if 128 as i32 == '4' as i32 {
            28 as i32
        } else if 128 as i32 == '5' as i32 {
            29 as i32
        } else if 128 as i32 == '6' as i32 {
            30 as i32
        } else if 128 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 129 as i32 == 'A' as i32 {
            0 as i32
        } else if 129 as i32 == 'B' as i32 {
            1 as i32
        } else if 129 as i32 == 'C' as i32 {
            2 as i32
        } else if 129 as i32 == 'D' as i32 {
            3 as i32
        } else if 129 as i32 == 'E' as i32 {
            4 as i32
        } else if 129 as i32 == 'F' as i32 {
            5 as i32
        } else if 129 as i32 == 'G' as i32 {
            6 as i32
        } else if 129 as i32 == 'H' as i32 {
            7 as i32
        } else if 129 as i32 == 'I' as i32 {
            8 as i32
        } else if 129 as i32 == 'J' as i32 {
            9 as i32
        } else if 129 as i32 == 'K' as i32 {
            10 as i32
        } else if 129 as i32 == 'L' as i32 {
            11 as i32
        } else if 129 as i32 == 'M' as i32 {
            12 as i32
        } else if 129 as i32 == 'N' as i32 {
            13 as i32
        } else if 129 as i32 == 'O' as i32 {
            14 as i32
        } else if 129 as i32 == 'P' as i32 {
            15 as i32
        } else if 129 as i32 == 'Q' as i32 {
            16 as i32
        } else if 129 as i32 == 'R' as i32 {
            17 as i32
        } else if 129 as i32 == 'S' as i32 {
            18 as i32
        } else if 129 as i32 == 'T' as i32 {
            19 as i32
        } else if 129 as i32 == 'U' as i32 {
            20 as i32
        } else if 129 as i32 == 'V' as i32 {
            21 as i32
        } else if 129 as i32 == 'W' as i32 {
            22 as i32
        } else if 129 as i32 == 'X' as i32 {
            23 as i32
        } else if 129 as i32 == 'Y' as i32 {
            24 as i32
        } else if 129 as i32 == 'Z' as i32 {
            25 as i32
        } else if 129 as i32 == '2' as i32 {
            26 as i32
        } else if 129 as i32 == '3' as i32 {
            27 as i32
        } else if 129 as i32 == '4' as i32 {
            28 as i32
        } else if 129 as i32 == '5' as i32 {
            29 as i32
        } else if 129 as i32 == '6' as i32 {
            30 as i32
        } else if 129 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 130 as i32 == 'A' as i32 {
            0 as i32
        } else if 130 as i32 == 'B' as i32 {
            1 as i32
        } else if 130 as i32 == 'C' as i32 {
            2 as i32
        } else if 130 as i32 == 'D' as i32 {
            3 as i32
        } else if 130 as i32 == 'E' as i32 {
            4 as i32
        } else if 130 as i32 == 'F' as i32 {
            5 as i32
        } else if 130 as i32 == 'G' as i32 {
            6 as i32
        } else if 130 as i32 == 'H' as i32 {
            7 as i32
        } else if 130 as i32 == 'I' as i32 {
            8 as i32
        } else if 130 as i32 == 'J' as i32 {
            9 as i32
        } else if 130 as i32 == 'K' as i32 {
            10 as i32
        } else if 130 as i32 == 'L' as i32 {
            11 as i32
        } else if 130 as i32 == 'M' as i32 {
            12 as i32
        } else if 130 as i32 == 'N' as i32 {
            13 as i32
        } else if 130 as i32 == 'O' as i32 {
            14 as i32
        } else if 130 as i32 == 'P' as i32 {
            15 as i32
        } else if 130 as i32 == 'Q' as i32 {
            16 as i32
        } else if 130 as i32 == 'R' as i32 {
            17 as i32
        } else if 130 as i32 == 'S' as i32 {
            18 as i32
        } else if 130 as i32 == 'T' as i32 {
            19 as i32
        } else if 130 as i32 == 'U' as i32 {
            20 as i32
        } else if 130 as i32 == 'V' as i32 {
            21 as i32
        } else if 130 as i32 == 'W' as i32 {
            22 as i32
        } else if 130 as i32 == 'X' as i32 {
            23 as i32
        } else if 130 as i32 == 'Y' as i32 {
            24 as i32
        } else if 130 as i32 == 'Z' as i32 {
            25 as i32
        } else if 130 as i32 == '2' as i32 {
            26 as i32
        } else if 130 as i32 == '3' as i32 {
            27 as i32
        } else if 130 as i32 == '4' as i32 {
            28 as i32
        } else if 130 as i32 == '5' as i32 {
            29 as i32
        } else if 130 as i32 == '6' as i32 {
            30 as i32
        } else if 130 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 131 as i32 == 'A' as i32 {
            0 as i32
        } else if 131 as i32 == 'B' as i32 {
            1 as i32
        } else if 131 as i32 == 'C' as i32 {
            2 as i32
        } else if 131 as i32 == 'D' as i32 {
            3 as i32
        } else if 131 as i32 == 'E' as i32 {
            4 as i32
        } else if 131 as i32 == 'F' as i32 {
            5 as i32
        } else if 131 as i32 == 'G' as i32 {
            6 as i32
        } else if 131 as i32 == 'H' as i32 {
            7 as i32
        } else if 131 as i32 == 'I' as i32 {
            8 as i32
        } else if 131 as i32 == 'J' as i32 {
            9 as i32
        } else if 131 as i32 == 'K' as i32 {
            10 as i32
        } else if 131 as i32 == 'L' as i32 {
            11 as i32
        } else if 131 as i32 == 'M' as i32 {
            12 as i32
        } else if 131 as i32 == 'N' as i32 {
            13 as i32
        } else if 131 as i32 == 'O' as i32 {
            14 as i32
        } else if 131 as i32 == 'P' as i32 {
            15 as i32
        } else if 131 as i32 == 'Q' as i32 {
            16 as i32
        } else if 131 as i32 == 'R' as i32 {
            17 as i32
        } else if 131 as i32 == 'S' as i32 {
            18 as i32
        } else if 131 as i32 == 'T' as i32 {
            19 as i32
        } else if 131 as i32 == 'U' as i32 {
            20 as i32
        } else if 131 as i32 == 'V' as i32 {
            21 as i32
        } else if 131 as i32 == 'W' as i32 {
            22 as i32
        } else if 131 as i32 == 'X' as i32 {
            23 as i32
        } else if 131 as i32 == 'Y' as i32 {
            24 as i32
        } else if 131 as i32 == 'Z' as i32 {
            25 as i32
        } else if 131 as i32 == '2' as i32 {
            26 as i32
        } else if 131 as i32 == '3' as i32 {
            27 as i32
        } else if 131 as i32 == '4' as i32 {
            28 as i32
        } else if 131 as i32 == '5' as i32 {
            29 as i32
        } else if 131 as i32 == '6' as i32 {
            30 as i32
        } else if 131 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 132 as i32 == 'A' as i32 {
            0 as i32
        } else if 132 as i32 == 'B' as i32 {
            1 as i32
        } else if 132 as i32 == 'C' as i32 {
            2 as i32
        } else if 132 as i32 == 'D' as i32 {
            3 as i32
        } else if 132 as i32 == 'E' as i32 {
            4 as i32
        } else if 132 as i32 == 'F' as i32 {
            5 as i32
        } else if 132 as i32 == 'G' as i32 {
            6 as i32
        } else if 132 as i32 == 'H' as i32 {
            7 as i32
        } else if 132 as i32 == 'I' as i32 {
            8 as i32
        } else if 132 as i32 == 'J' as i32 {
            9 as i32
        } else if 132 as i32 == 'K' as i32 {
            10 as i32
        } else if 132 as i32 == 'L' as i32 {
            11 as i32
        } else if 132 as i32 == 'M' as i32 {
            12 as i32
        } else if 132 as i32 == 'N' as i32 {
            13 as i32
        } else if 132 as i32 == 'O' as i32 {
            14 as i32
        } else if 132 as i32 == 'P' as i32 {
            15 as i32
        } else if 132 as i32 == 'Q' as i32 {
            16 as i32
        } else if 132 as i32 == 'R' as i32 {
            17 as i32
        } else if 132 as i32 == 'S' as i32 {
            18 as i32
        } else if 132 as i32 == 'T' as i32 {
            19 as i32
        } else if 132 as i32 == 'U' as i32 {
            20 as i32
        } else if 132 as i32 == 'V' as i32 {
            21 as i32
        } else if 132 as i32 == 'W' as i32 {
            22 as i32
        } else if 132 as i32 == 'X' as i32 {
            23 as i32
        } else if 132 as i32 == 'Y' as i32 {
            24 as i32
        } else if 132 as i32 == 'Z' as i32 {
            25 as i32
        } else if 132 as i32 == '2' as i32 {
            26 as i32
        } else if 132 as i32 == '3' as i32 {
            27 as i32
        } else if 132 as i32 == '4' as i32 {
            28 as i32
        } else if 132 as i32 == '5' as i32 {
            29 as i32
        } else if 132 as i32 == '6' as i32 {
            30 as i32
        } else if 132 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 133 as i32 == 'A' as i32 {
            0 as i32
        } else if 133 as i32 == 'B' as i32 {
            1 as i32
        } else if 133 as i32 == 'C' as i32 {
            2 as i32
        } else if 133 as i32 == 'D' as i32 {
            3 as i32
        } else if 133 as i32 == 'E' as i32 {
            4 as i32
        } else if 133 as i32 == 'F' as i32 {
            5 as i32
        } else if 133 as i32 == 'G' as i32 {
            6 as i32
        } else if 133 as i32 == 'H' as i32 {
            7 as i32
        } else if 133 as i32 == 'I' as i32 {
            8 as i32
        } else if 133 as i32 == 'J' as i32 {
            9 as i32
        } else if 133 as i32 == 'K' as i32 {
            10 as i32
        } else if 133 as i32 == 'L' as i32 {
            11 as i32
        } else if 133 as i32 == 'M' as i32 {
            12 as i32
        } else if 133 as i32 == 'N' as i32 {
            13 as i32
        } else if 133 as i32 == 'O' as i32 {
            14 as i32
        } else if 133 as i32 == 'P' as i32 {
            15 as i32
        } else if 133 as i32 == 'Q' as i32 {
            16 as i32
        } else if 133 as i32 == 'R' as i32 {
            17 as i32
        } else if 133 as i32 == 'S' as i32 {
            18 as i32
        } else if 133 as i32 == 'T' as i32 {
            19 as i32
        } else if 133 as i32 == 'U' as i32 {
            20 as i32
        } else if 133 as i32 == 'V' as i32 {
            21 as i32
        } else if 133 as i32 == 'W' as i32 {
            22 as i32
        } else if 133 as i32 == 'X' as i32 {
            23 as i32
        } else if 133 as i32 == 'Y' as i32 {
            24 as i32
        } else if 133 as i32 == 'Z' as i32 {
            25 as i32
        } else if 133 as i32 == '2' as i32 {
            26 as i32
        } else if 133 as i32 == '3' as i32 {
            27 as i32
        } else if 133 as i32 == '4' as i32 {
            28 as i32
        } else if 133 as i32 == '5' as i32 {
            29 as i32
        } else if 133 as i32 == '6' as i32 {
            30 as i32
        } else if 133 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 134 as i32 == 'A' as i32 {
            0 as i32
        } else if 134 as i32 == 'B' as i32 {
            1 as i32
        } else if 134 as i32 == 'C' as i32 {
            2 as i32
        } else if 134 as i32 == 'D' as i32 {
            3 as i32
        } else if 134 as i32 == 'E' as i32 {
            4 as i32
        } else if 134 as i32 == 'F' as i32 {
            5 as i32
        } else if 134 as i32 == 'G' as i32 {
            6 as i32
        } else if 134 as i32 == 'H' as i32 {
            7 as i32
        } else if 134 as i32 == 'I' as i32 {
            8 as i32
        } else if 134 as i32 == 'J' as i32 {
            9 as i32
        } else if 134 as i32 == 'K' as i32 {
            10 as i32
        } else if 134 as i32 == 'L' as i32 {
            11 as i32
        } else if 134 as i32 == 'M' as i32 {
            12 as i32
        } else if 134 as i32 == 'N' as i32 {
            13 as i32
        } else if 134 as i32 == 'O' as i32 {
            14 as i32
        } else if 134 as i32 == 'P' as i32 {
            15 as i32
        } else if 134 as i32 == 'Q' as i32 {
            16 as i32
        } else if 134 as i32 == 'R' as i32 {
            17 as i32
        } else if 134 as i32 == 'S' as i32 {
            18 as i32
        } else if 134 as i32 == 'T' as i32 {
            19 as i32
        } else if 134 as i32 == 'U' as i32 {
            20 as i32
        } else if 134 as i32 == 'V' as i32 {
            21 as i32
        } else if 134 as i32 == 'W' as i32 {
            22 as i32
        } else if 134 as i32 == 'X' as i32 {
            23 as i32
        } else if 134 as i32 == 'Y' as i32 {
            24 as i32
        } else if 134 as i32 == 'Z' as i32 {
            25 as i32
        } else if 134 as i32 == '2' as i32 {
            26 as i32
        } else if 134 as i32 == '3' as i32 {
            27 as i32
        } else if 134 as i32 == '4' as i32 {
            28 as i32
        } else if 134 as i32 == '5' as i32 {
            29 as i32
        } else if 134 as i32 == '6' as i32 {
            30 as i32
        } else if 134 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 135 as i32 == 'A' as i32 {
            0 as i32
        } else if 135 as i32 == 'B' as i32 {
            1 as i32
        } else if 135 as i32 == 'C' as i32 {
            2 as i32
        } else if 135 as i32 == 'D' as i32 {
            3 as i32
        } else if 135 as i32 == 'E' as i32 {
            4 as i32
        } else if 135 as i32 == 'F' as i32 {
            5 as i32
        } else if 135 as i32 == 'G' as i32 {
            6 as i32
        } else if 135 as i32 == 'H' as i32 {
            7 as i32
        } else if 135 as i32 == 'I' as i32 {
            8 as i32
        } else if 135 as i32 == 'J' as i32 {
            9 as i32
        } else if 135 as i32 == 'K' as i32 {
            10 as i32
        } else if 135 as i32 == 'L' as i32 {
            11 as i32
        } else if 135 as i32 == 'M' as i32 {
            12 as i32
        } else if 135 as i32 == 'N' as i32 {
            13 as i32
        } else if 135 as i32 == 'O' as i32 {
            14 as i32
        } else if 135 as i32 == 'P' as i32 {
            15 as i32
        } else if 135 as i32 == 'Q' as i32 {
            16 as i32
        } else if 135 as i32 == 'R' as i32 {
            17 as i32
        } else if 135 as i32 == 'S' as i32 {
            18 as i32
        } else if 135 as i32 == 'T' as i32 {
            19 as i32
        } else if 135 as i32 == 'U' as i32 {
            20 as i32
        } else if 135 as i32 == 'V' as i32 {
            21 as i32
        } else if 135 as i32 == 'W' as i32 {
            22 as i32
        } else if 135 as i32 == 'X' as i32 {
            23 as i32
        } else if 135 as i32 == 'Y' as i32 {
            24 as i32
        } else if 135 as i32 == 'Z' as i32 {
            25 as i32
        } else if 135 as i32 == '2' as i32 {
            26 as i32
        } else if 135 as i32 == '3' as i32 {
            27 as i32
        } else if 135 as i32 == '4' as i32 {
            28 as i32
        } else if 135 as i32 == '5' as i32 {
            29 as i32
        } else if 135 as i32 == '6' as i32 {
            30 as i32
        } else if 135 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 136 as i32 == 'A' as i32 {
            0 as i32
        } else if 136 as i32 == 'B' as i32 {
            1 as i32
        } else if 136 as i32 == 'C' as i32 {
            2 as i32
        } else if 136 as i32 == 'D' as i32 {
            3 as i32
        } else if 136 as i32 == 'E' as i32 {
            4 as i32
        } else if 136 as i32 == 'F' as i32 {
            5 as i32
        } else if 136 as i32 == 'G' as i32 {
            6 as i32
        } else if 136 as i32 == 'H' as i32 {
            7 as i32
        } else if 136 as i32 == 'I' as i32 {
            8 as i32
        } else if 136 as i32 == 'J' as i32 {
            9 as i32
        } else if 136 as i32 == 'K' as i32 {
            10 as i32
        } else if 136 as i32 == 'L' as i32 {
            11 as i32
        } else if 136 as i32 == 'M' as i32 {
            12 as i32
        } else if 136 as i32 == 'N' as i32 {
            13 as i32
        } else if 136 as i32 == 'O' as i32 {
            14 as i32
        } else if 136 as i32 == 'P' as i32 {
            15 as i32
        } else if 136 as i32 == 'Q' as i32 {
            16 as i32
        } else if 136 as i32 == 'R' as i32 {
            17 as i32
        } else if 136 as i32 == 'S' as i32 {
            18 as i32
        } else if 136 as i32 == 'T' as i32 {
            19 as i32
        } else if 136 as i32 == 'U' as i32 {
            20 as i32
        } else if 136 as i32 == 'V' as i32 {
            21 as i32
        } else if 136 as i32 == 'W' as i32 {
            22 as i32
        } else if 136 as i32 == 'X' as i32 {
            23 as i32
        } else if 136 as i32 == 'Y' as i32 {
            24 as i32
        } else if 136 as i32 == 'Z' as i32 {
            25 as i32
        } else if 136 as i32 == '2' as i32 {
            26 as i32
        } else if 136 as i32 == '3' as i32 {
            27 as i32
        } else if 136 as i32 == '4' as i32 {
            28 as i32
        } else if 136 as i32 == '5' as i32 {
            29 as i32
        } else if 136 as i32 == '6' as i32 {
            30 as i32
        } else if 136 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 137 as i32 == 'A' as i32 {
            0 as i32
        } else if 137 as i32 == 'B' as i32 {
            1 as i32
        } else if 137 as i32 == 'C' as i32 {
            2 as i32
        } else if 137 as i32 == 'D' as i32 {
            3 as i32
        } else if 137 as i32 == 'E' as i32 {
            4 as i32
        } else if 137 as i32 == 'F' as i32 {
            5 as i32
        } else if 137 as i32 == 'G' as i32 {
            6 as i32
        } else if 137 as i32 == 'H' as i32 {
            7 as i32
        } else if 137 as i32 == 'I' as i32 {
            8 as i32
        } else if 137 as i32 == 'J' as i32 {
            9 as i32
        } else if 137 as i32 == 'K' as i32 {
            10 as i32
        } else if 137 as i32 == 'L' as i32 {
            11 as i32
        } else if 137 as i32 == 'M' as i32 {
            12 as i32
        } else if 137 as i32 == 'N' as i32 {
            13 as i32
        } else if 137 as i32 == 'O' as i32 {
            14 as i32
        } else if 137 as i32 == 'P' as i32 {
            15 as i32
        } else if 137 as i32 == 'Q' as i32 {
            16 as i32
        } else if 137 as i32 == 'R' as i32 {
            17 as i32
        } else if 137 as i32 == 'S' as i32 {
            18 as i32
        } else if 137 as i32 == 'T' as i32 {
            19 as i32
        } else if 137 as i32 == 'U' as i32 {
            20 as i32
        } else if 137 as i32 == 'V' as i32 {
            21 as i32
        } else if 137 as i32 == 'W' as i32 {
            22 as i32
        } else if 137 as i32 == 'X' as i32 {
            23 as i32
        } else if 137 as i32 == 'Y' as i32 {
            24 as i32
        } else if 137 as i32 == 'Z' as i32 {
            25 as i32
        } else if 137 as i32 == '2' as i32 {
            26 as i32
        } else if 137 as i32 == '3' as i32 {
            27 as i32
        } else if 137 as i32 == '4' as i32 {
            28 as i32
        } else if 137 as i32 == '5' as i32 {
            29 as i32
        } else if 137 as i32 == '6' as i32 {
            30 as i32
        } else if 137 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 138 as i32 == 'A' as i32 {
            0 as i32
        } else if 138 as i32 == 'B' as i32 {
            1 as i32
        } else if 138 as i32 == 'C' as i32 {
            2 as i32
        } else if 138 as i32 == 'D' as i32 {
            3 as i32
        } else if 138 as i32 == 'E' as i32 {
            4 as i32
        } else if 138 as i32 == 'F' as i32 {
            5 as i32
        } else if 138 as i32 == 'G' as i32 {
            6 as i32
        } else if 138 as i32 == 'H' as i32 {
            7 as i32
        } else if 138 as i32 == 'I' as i32 {
            8 as i32
        } else if 138 as i32 == 'J' as i32 {
            9 as i32
        } else if 138 as i32 == 'K' as i32 {
            10 as i32
        } else if 138 as i32 == 'L' as i32 {
            11 as i32
        } else if 138 as i32 == 'M' as i32 {
            12 as i32
        } else if 138 as i32 == 'N' as i32 {
            13 as i32
        } else if 138 as i32 == 'O' as i32 {
            14 as i32
        } else if 138 as i32 == 'P' as i32 {
            15 as i32
        } else if 138 as i32 == 'Q' as i32 {
            16 as i32
        } else if 138 as i32 == 'R' as i32 {
            17 as i32
        } else if 138 as i32 == 'S' as i32 {
            18 as i32
        } else if 138 as i32 == 'T' as i32 {
            19 as i32
        } else if 138 as i32 == 'U' as i32 {
            20 as i32
        } else if 138 as i32 == 'V' as i32 {
            21 as i32
        } else if 138 as i32 == 'W' as i32 {
            22 as i32
        } else if 138 as i32 == 'X' as i32 {
            23 as i32
        } else if 138 as i32 == 'Y' as i32 {
            24 as i32
        } else if 138 as i32 == 'Z' as i32 {
            25 as i32
        } else if 138 as i32 == '2' as i32 {
            26 as i32
        } else if 138 as i32 == '3' as i32 {
            27 as i32
        } else if 138 as i32 == '4' as i32 {
            28 as i32
        } else if 138 as i32 == '5' as i32 {
            29 as i32
        } else if 138 as i32 == '6' as i32 {
            30 as i32
        } else if 138 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 139 as i32 == 'A' as i32 {
            0 as i32
        } else if 139 as i32 == 'B' as i32 {
            1 as i32
        } else if 139 as i32 == 'C' as i32 {
            2 as i32
        } else if 139 as i32 == 'D' as i32 {
            3 as i32
        } else if 139 as i32 == 'E' as i32 {
            4 as i32
        } else if 139 as i32 == 'F' as i32 {
            5 as i32
        } else if 139 as i32 == 'G' as i32 {
            6 as i32
        } else if 139 as i32 == 'H' as i32 {
            7 as i32
        } else if 139 as i32 == 'I' as i32 {
            8 as i32
        } else if 139 as i32 == 'J' as i32 {
            9 as i32
        } else if 139 as i32 == 'K' as i32 {
            10 as i32
        } else if 139 as i32 == 'L' as i32 {
            11 as i32
        } else if 139 as i32 == 'M' as i32 {
            12 as i32
        } else if 139 as i32 == 'N' as i32 {
            13 as i32
        } else if 139 as i32 == 'O' as i32 {
            14 as i32
        } else if 139 as i32 == 'P' as i32 {
            15 as i32
        } else if 139 as i32 == 'Q' as i32 {
            16 as i32
        } else if 139 as i32 == 'R' as i32 {
            17 as i32
        } else if 139 as i32 == 'S' as i32 {
            18 as i32
        } else if 139 as i32 == 'T' as i32 {
            19 as i32
        } else if 139 as i32 == 'U' as i32 {
            20 as i32
        } else if 139 as i32 == 'V' as i32 {
            21 as i32
        } else if 139 as i32 == 'W' as i32 {
            22 as i32
        } else if 139 as i32 == 'X' as i32 {
            23 as i32
        } else if 139 as i32 == 'Y' as i32 {
            24 as i32
        } else if 139 as i32 == 'Z' as i32 {
            25 as i32
        } else if 139 as i32 == '2' as i32 {
            26 as i32
        } else if 139 as i32 == '3' as i32 {
            27 as i32
        } else if 139 as i32 == '4' as i32 {
            28 as i32
        } else if 139 as i32 == '5' as i32 {
            29 as i32
        } else if 139 as i32 == '6' as i32 {
            30 as i32
        } else if 139 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 140 as i32 == 'A' as i32 {
            0 as i32
        } else if 140 as i32 == 'B' as i32 {
            1 as i32
        } else if 140 as i32 == 'C' as i32 {
            2 as i32
        } else if 140 as i32 == 'D' as i32 {
            3 as i32
        } else if 140 as i32 == 'E' as i32 {
            4 as i32
        } else if 140 as i32 == 'F' as i32 {
            5 as i32
        } else if 140 as i32 == 'G' as i32 {
            6 as i32
        } else if 140 as i32 == 'H' as i32 {
            7 as i32
        } else if 140 as i32 == 'I' as i32 {
            8 as i32
        } else if 140 as i32 == 'J' as i32 {
            9 as i32
        } else if 140 as i32 == 'K' as i32 {
            10 as i32
        } else if 140 as i32 == 'L' as i32 {
            11 as i32
        } else if 140 as i32 == 'M' as i32 {
            12 as i32
        } else if 140 as i32 == 'N' as i32 {
            13 as i32
        } else if 140 as i32 == 'O' as i32 {
            14 as i32
        } else if 140 as i32 == 'P' as i32 {
            15 as i32
        } else if 140 as i32 == 'Q' as i32 {
            16 as i32
        } else if 140 as i32 == 'R' as i32 {
            17 as i32
        } else if 140 as i32 == 'S' as i32 {
            18 as i32
        } else if 140 as i32 == 'T' as i32 {
            19 as i32
        } else if 140 as i32 == 'U' as i32 {
            20 as i32
        } else if 140 as i32 == 'V' as i32 {
            21 as i32
        } else if 140 as i32 == 'W' as i32 {
            22 as i32
        } else if 140 as i32 == 'X' as i32 {
            23 as i32
        } else if 140 as i32 == 'Y' as i32 {
            24 as i32
        } else if 140 as i32 == 'Z' as i32 {
            25 as i32
        } else if 140 as i32 == '2' as i32 {
            26 as i32
        } else if 140 as i32 == '3' as i32 {
            27 as i32
        } else if 140 as i32 == '4' as i32 {
            28 as i32
        } else if 140 as i32 == '5' as i32 {
            29 as i32
        } else if 140 as i32 == '6' as i32 {
            30 as i32
        } else if 140 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 141 as i32 == 'A' as i32 {
            0 as i32
        } else if 141 as i32 == 'B' as i32 {
            1 as i32
        } else if 141 as i32 == 'C' as i32 {
            2 as i32
        } else if 141 as i32 == 'D' as i32 {
            3 as i32
        } else if 141 as i32 == 'E' as i32 {
            4 as i32
        } else if 141 as i32 == 'F' as i32 {
            5 as i32
        } else if 141 as i32 == 'G' as i32 {
            6 as i32
        } else if 141 as i32 == 'H' as i32 {
            7 as i32
        } else if 141 as i32 == 'I' as i32 {
            8 as i32
        } else if 141 as i32 == 'J' as i32 {
            9 as i32
        } else if 141 as i32 == 'K' as i32 {
            10 as i32
        } else if 141 as i32 == 'L' as i32 {
            11 as i32
        } else if 141 as i32 == 'M' as i32 {
            12 as i32
        } else if 141 as i32 == 'N' as i32 {
            13 as i32
        } else if 141 as i32 == 'O' as i32 {
            14 as i32
        } else if 141 as i32 == 'P' as i32 {
            15 as i32
        } else if 141 as i32 == 'Q' as i32 {
            16 as i32
        } else if 141 as i32 == 'R' as i32 {
            17 as i32
        } else if 141 as i32 == 'S' as i32 {
            18 as i32
        } else if 141 as i32 == 'T' as i32 {
            19 as i32
        } else if 141 as i32 == 'U' as i32 {
            20 as i32
        } else if 141 as i32 == 'V' as i32 {
            21 as i32
        } else if 141 as i32 == 'W' as i32 {
            22 as i32
        } else if 141 as i32 == 'X' as i32 {
            23 as i32
        } else if 141 as i32 == 'Y' as i32 {
            24 as i32
        } else if 141 as i32 == 'Z' as i32 {
            25 as i32
        } else if 141 as i32 == '2' as i32 {
            26 as i32
        } else if 141 as i32 == '3' as i32 {
            27 as i32
        } else if 141 as i32 == '4' as i32 {
            28 as i32
        } else if 141 as i32 == '5' as i32 {
            29 as i32
        } else if 141 as i32 == '6' as i32 {
            30 as i32
        } else if 141 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 142 as i32 == 'A' as i32 {
            0 as i32
        } else if 142 as i32 == 'B' as i32 {
            1 as i32
        } else if 142 as i32 == 'C' as i32 {
            2 as i32
        } else if 142 as i32 == 'D' as i32 {
            3 as i32
        } else if 142 as i32 == 'E' as i32 {
            4 as i32
        } else if 142 as i32 == 'F' as i32 {
            5 as i32
        } else if 142 as i32 == 'G' as i32 {
            6 as i32
        } else if 142 as i32 == 'H' as i32 {
            7 as i32
        } else if 142 as i32 == 'I' as i32 {
            8 as i32
        } else if 142 as i32 == 'J' as i32 {
            9 as i32
        } else if 142 as i32 == 'K' as i32 {
            10 as i32
        } else if 142 as i32 == 'L' as i32 {
            11 as i32
        } else if 142 as i32 == 'M' as i32 {
            12 as i32
        } else if 142 as i32 == 'N' as i32 {
            13 as i32
        } else if 142 as i32 == 'O' as i32 {
            14 as i32
        } else if 142 as i32 == 'P' as i32 {
            15 as i32
        } else if 142 as i32 == 'Q' as i32 {
            16 as i32
        } else if 142 as i32 == 'R' as i32 {
            17 as i32
        } else if 142 as i32 == 'S' as i32 {
            18 as i32
        } else if 142 as i32 == 'T' as i32 {
            19 as i32
        } else if 142 as i32 == 'U' as i32 {
            20 as i32
        } else if 142 as i32 == 'V' as i32 {
            21 as i32
        } else if 142 as i32 == 'W' as i32 {
            22 as i32
        } else if 142 as i32 == 'X' as i32 {
            23 as i32
        } else if 142 as i32 == 'Y' as i32 {
            24 as i32
        } else if 142 as i32 == 'Z' as i32 {
            25 as i32
        } else if 142 as i32 == '2' as i32 {
            26 as i32
        } else if 142 as i32 == '3' as i32 {
            27 as i32
        } else if 142 as i32 == '4' as i32 {
            28 as i32
        } else if 142 as i32 == '5' as i32 {
            29 as i32
        } else if 142 as i32 == '6' as i32 {
            30 as i32
        } else if 142 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 143 as i32 == 'A' as i32 {
            0 as i32
        } else if 143 as i32 == 'B' as i32 {
            1 as i32
        } else if 143 as i32 == 'C' as i32 {
            2 as i32
        } else if 143 as i32 == 'D' as i32 {
            3 as i32
        } else if 143 as i32 == 'E' as i32 {
            4 as i32
        } else if 143 as i32 == 'F' as i32 {
            5 as i32
        } else if 143 as i32 == 'G' as i32 {
            6 as i32
        } else if 143 as i32 == 'H' as i32 {
            7 as i32
        } else if 143 as i32 == 'I' as i32 {
            8 as i32
        } else if 143 as i32 == 'J' as i32 {
            9 as i32
        } else if 143 as i32 == 'K' as i32 {
            10 as i32
        } else if 143 as i32 == 'L' as i32 {
            11 as i32
        } else if 143 as i32 == 'M' as i32 {
            12 as i32
        } else if 143 as i32 == 'N' as i32 {
            13 as i32
        } else if 143 as i32 == 'O' as i32 {
            14 as i32
        } else if 143 as i32 == 'P' as i32 {
            15 as i32
        } else if 143 as i32 == 'Q' as i32 {
            16 as i32
        } else if 143 as i32 == 'R' as i32 {
            17 as i32
        } else if 143 as i32 == 'S' as i32 {
            18 as i32
        } else if 143 as i32 == 'T' as i32 {
            19 as i32
        } else if 143 as i32 == 'U' as i32 {
            20 as i32
        } else if 143 as i32 == 'V' as i32 {
            21 as i32
        } else if 143 as i32 == 'W' as i32 {
            22 as i32
        } else if 143 as i32 == 'X' as i32 {
            23 as i32
        } else if 143 as i32 == 'Y' as i32 {
            24 as i32
        } else if 143 as i32 == 'Z' as i32 {
            25 as i32
        } else if 143 as i32 == '2' as i32 {
            26 as i32
        } else if 143 as i32 == '3' as i32 {
            27 as i32
        } else if 143 as i32 == '4' as i32 {
            28 as i32
        } else if 143 as i32 == '5' as i32 {
            29 as i32
        } else if 143 as i32 == '6' as i32 {
            30 as i32
        } else if 143 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 144 as i32 == 'A' as i32 {
            0 as i32
        } else if 144 as i32 == 'B' as i32 {
            1 as i32
        } else if 144 as i32 == 'C' as i32 {
            2 as i32
        } else if 144 as i32 == 'D' as i32 {
            3 as i32
        } else if 144 as i32 == 'E' as i32 {
            4 as i32
        } else if 144 as i32 == 'F' as i32 {
            5 as i32
        } else if 144 as i32 == 'G' as i32 {
            6 as i32
        } else if 144 as i32 == 'H' as i32 {
            7 as i32
        } else if 144 as i32 == 'I' as i32 {
            8 as i32
        } else if 144 as i32 == 'J' as i32 {
            9 as i32
        } else if 144 as i32 == 'K' as i32 {
            10 as i32
        } else if 144 as i32 == 'L' as i32 {
            11 as i32
        } else if 144 as i32 == 'M' as i32 {
            12 as i32
        } else if 144 as i32 == 'N' as i32 {
            13 as i32
        } else if 144 as i32 == 'O' as i32 {
            14 as i32
        } else if 144 as i32 == 'P' as i32 {
            15 as i32
        } else if 144 as i32 == 'Q' as i32 {
            16 as i32
        } else if 144 as i32 == 'R' as i32 {
            17 as i32
        } else if 144 as i32 == 'S' as i32 {
            18 as i32
        } else if 144 as i32 == 'T' as i32 {
            19 as i32
        } else if 144 as i32 == 'U' as i32 {
            20 as i32
        } else if 144 as i32 == 'V' as i32 {
            21 as i32
        } else if 144 as i32 == 'W' as i32 {
            22 as i32
        } else if 144 as i32 == 'X' as i32 {
            23 as i32
        } else if 144 as i32 == 'Y' as i32 {
            24 as i32
        } else if 144 as i32 == 'Z' as i32 {
            25 as i32
        } else if 144 as i32 == '2' as i32 {
            26 as i32
        } else if 144 as i32 == '3' as i32 {
            27 as i32
        } else if 144 as i32 == '4' as i32 {
            28 as i32
        } else if 144 as i32 == '5' as i32 {
            29 as i32
        } else if 144 as i32 == '6' as i32 {
            30 as i32
        } else if 144 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 145 as i32 == 'A' as i32 {
            0 as i32
        } else if 145 as i32 == 'B' as i32 {
            1 as i32
        } else if 145 as i32 == 'C' as i32 {
            2 as i32
        } else if 145 as i32 == 'D' as i32 {
            3 as i32
        } else if 145 as i32 == 'E' as i32 {
            4 as i32
        } else if 145 as i32 == 'F' as i32 {
            5 as i32
        } else if 145 as i32 == 'G' as i32 {
            6 as i32
        } else if 145 as i32 == 'H' as i32 {
            7 as i32
        } else if 145 as i32 == 'I' as i32 {
            8 as i32
        } else if 145 as i32 == 'J' as i32 {
            9 as i32
        } else if 145 as i32 == 'K' as i32 {
            10 as i32
        } else if 145 as i32 == 'L' as i32 {
            11 as i32
        } else if 145 as i32 == 'M' as i32 {
            12 as i32
        } else if 145 as i32 == 'N' as i32 {
            13 as i32
        } else if 145 as i32 == 'O' as i32 {
            14 as i32
        } else if 145 as i32 == 'P' as i32 {
            15 as i32
        } else if 145 as i32 == 'Q' as i32 {
            16 as i32
        } else if 145 as i32 == 'R' as i32 {
            17 as i32
        } else if 145 as i32 == 'S' as i32 {
            18 as i32
        } else if 145 as i32 == 'T' as i32 {
            19 as i32
        } else if 145 as i32 == 'U' as i32 {
            20 as i32
        } else if 145 as i32 == 'V' as i32 {
            21 as i32
        } else if 145 as i32 == 'W' as i32 {
            22 as i32
        } else if 145 as i32 == 'X' as i32 {
            23 as i32
        } else if 145 as i32 == 'Y' as i32 {
            24 as i32
        } else if 145 as i32 == 'Z' as i32 {
            25 as i32
        } else if 145 as i32 == '2' as i32 {
            26 as i32
        } else if 145 as i32 == '3' as i32 {
            27 as i32
        } else if 145 as i32 == '4' as i32 {
            28 as i32
        } else if 145 as i32 == '5' as i32 {
            29 as i32
        } else if 145 as i32 == '6' as i32 {
            30 as i32
        } else if 145 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 146 as i32 == 'A' as i32 {
            0 as i32
        } else if 146 as i32 == 'B' as i32 {
            1 as i32
        } else if 146 as i32 == 'C' as i32 {
            2 as i32
        } else if 146 as i32 == 'D' as i32 {
            3 as i32
        } else if 146 as i32 == 'E' as i32 {
            4 as i32
        } else if 146 as i32 == 'F' as i32 {
            5 as i32
        } else if 146 as i32 == 'G' as i32 {
            6 as i32
        } else if 146 as i32 == 'H' as i32 {
            7 as i32
        } else if 146 as i32 == 'I' as i32 {
            8 as i32
        } else if 146 as i32 == 'J' as i32 {
            9 as i32
        } else if 146 as i32 == 'K' as i32 {
            10 as i32
        } else if 146 as i32 == 'L' as i32 {
            11 as i32
        } else if 146 as i32 == 'M' as i32 {
            12 as i32
        } else if 146 as i32 == 'N' as i32 {
            13 as i32
        } else if 146 as i32 == 'O' as i32 {
            14 as i32
        } else if 146 as i32 == 'P' as i32 {
            15 as i32
        } else if 146 as i32 == 'Q' as i32 {
            16 as i32
        } else if 146 as i32 == 'R' as i32 {
            17 as i32
        } else if 146 as i32 == 'S' as i32 {
            18 as i32
        } else if 146 as i32 == 'T' as i32 {
            19 as i32
        } else if 146 as i32 == 'U' as i32 {
            20 as i32
        } else if 146 as i32 == 'V' as i32 {
            21 as i32
        } else if 146 as i32 == 'W' as i32 {
            22 as i32
        } else if 146 as i32 == 'X' as i32 {
            23 as i32
        } else if 146 as i32 == 'Y' as i32 {
            24 as i32
        } else if 146 as i32 == 'Z' as i32 {
            25 as i32
        } else if 146 as i32 == '2' as i32 {
            26 as i32
        } else if 146 as i32 == '3' as i32 {
            27 as i32
        } else if 146 as i32 == '4' as i32 {
            28 as i32
        } else if 146 as i32 == '5' as i32 {
            29 as i32
        } else if 146 as i32 == '6' as i32 {
            30 as i32
        } else if 146 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 147 as i32 == 'A' as i32 {
            0 as i32
        } else if 147 as i32 == 'B' as i32 {
            1 as i32
        } else if 147 as i32 == 'C' as i32 {
            2 as i32
        } else if 147 as i32 == 'D' as i32 {
            3 as i32
        } else if 147 as i32 == 'E' as i32 {
            4 as i32
        } else if 147 as i32 == 'F' as i32 {
            5 as i32
        } else if 147 as i32 == 'G' as i32 {
            6 as i32
        } else if 147 as i32 == 'H' as i32 {
            7 as i32
        } else if 147 as i32 == 'I' as i32 {
            8 as i32
        } else if 147 as i32 == 'J' as i32 {
            9 as i32
        } else if 147 as i32 == 'K' as i32 {
            10 as i32
        } else if 147 as i32 == 'L' as i32 {
            11 as i32
        } else if 147 as i32 == 'M' as i32 {
            12 as i32
        } else if 147 as i32 == 'N' as i32 {
            13 as i32
        } else if 147 as i32 == 'O' as i32 {
            14 as i32
        } else if 147 as i32 == 'P' as i32 {
            15 as i32
        } else if 147 as i32 == 'Q' as i32 {
            16 as i32
        } else if 147 as i32 == 'R' as i32 {
            17 as i32
        } else if 147 as i32 == 'S' as i32 {
            18 as i32
        } else if 147 as i32 == 'T' as i32 {
            19 as i32
        } else if 147 as i32 == 'U' as i32 {
            20 as i32
        } else if 147 as i32 == 'V' as i32 {
            21 as i32
        } else if 147 as i32 == 'W' as i32 {
            22 as i32
        } else if 147 as i32 == 'X' as i32 {
            23 as i32
        } else if 147 as i32 == 'Y' as i32 {
            24 as i32
        } else if 147 as i32 == 'Z' as i32 {
            25 as i32
        } else if 147 as i32 == '2' as i32 {
            26 as i32
        } else if 147 as i32 == '3' as i32 {
            27 as i32
        } else if 147 as i32 == '4' as i32 {
            28 as i32
        } else if 147 as i32 == '5' as i32 {
            29 as i32
        } else if 147 as i32 == '6' as i32 {
            30 as i32
        } else if 147 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 148 as i32 == 'A' as i32 {
            0 as i32
        } else if 148 as i32 == 'B' as i32 {
            1 as i32
        } else if 148 as i32 == 'C' as i32 {
            2 as i32
        } else if 148 as i32 == 'D' as i32 {
            3 as i32
        } else if 148 as i32 == 'E' as i32 {
            4 as i32
        } else if 148 as i32 == 'F' as i32 {
            5 as i32
        } else if 148 as i32 == 'G' as i32 {
            6 as i32
        } else if 148 as i32 == 'H' as i32 {
            7 as i32
        } else if 148 as i32 == 'I' as i32 {
            8 as i32
        } else if 148 as i32 == 'J' as i32 {
            9 as i32
        } else if 148 as i32 == 'K' as i32 {
            10 as i32
        } else if 148 as i32 == 'L' as i32 {
            11 as i32
        } else if 148 as i32 == 'M' as i32 {
            12 as i32
        } else if 148 as i32 == 'N' as i32 {
            13 as i32
        } else if 148 as i32 == 'O' as i32 {
            14 as i32
        } else if 148 as i32 == 'P' as i32 {
            15 as i32
        } else if 148 as i32 == 'Q' as i32 {
            16 as i32
        } else if 148 as i32 == 'R' as i32 {
            17 as i32
        } else if 148 as i32 == 'S' as i32 {
            18 as i32
        } else if 148 as i32 == 'T' as i32 {
            19 as i32
        } else if 148 as i32 == 'U' as i32 {
            20 as i32
        } else if 148 as i32 == 'V' as i32 {
            21 as i32
        } else if 148 as i32 == 'W' as i32 {
            22 as i32
        } else if 148 as i32 == 'X' as i32 {
            23 as i32
        } else if 148 as i32 == 'Y' as i32 {
            24 as i32
        } else if 148 as i32 == 'Z' as i32 {
            25 as i32
        } else if 148 as i32 == '2' as i32 {
            26 as i32
        } else if 148 as i32 == '3' as i32 {
            27 as i32
        } else if 148 as i32 == '4' as i32 {
            28 as i32
        } else if 148 as i32 == '5' as i32 {
            29 as i32
        } else if 148 as i32 == '6' as i32 {
            30 as i32
        } else if 148 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 149 as i32 == 'A' as i32 {
            0 as i32
        } else if 149 as i32 == 'B' as i32 {
            1 as i32
        } else if 149 as i32 == 'C' as i32 {
            2 as i32
        } else if 149 as i32 == 'D' as i32 {
            3 as i32
        } else if 149 as i32 == 'E' as i32 {
            4 as i32
        } else if 149 as i32 == 'F' as i32 {
            5 as i32
        } else if 149 as i32 == 'G' as i32 {
            6 as i32
        } else if 149 as i32 == 'H' as i32 {
            7 as i32
        } else if 149 as i32 == 'I' as i32 {
            8 as i32
        } else if 149 as i32 == 'J' as i32 {
            9 as i32
        } else if 149 as i32 == 'K' as i32 {
            10 as i32
        } else if 149 as i32 == 'L' as i32 {
            11 as i32
        } else if 149 as i32 == 'M' as i32 {
            12 as i32
        } else if 149 as i32 == 'N' as i32 {
            13 as i32
        } else if 149 as i32 == 'O' as i32 {
            14 as i32
        } else if 149 as i32 == 'P' as i32 {
            15 as i32
        } else if 149 as i32 == 'Q' as i32 {
            16 as i32
        } else if 149 as i32 == 'R' as i32 {
            17 as i32
        } else if 149 as i32 == 'S' as i32 {
            18 as i32
        } else if 149 as i32 == 'T' as i32 {
            19 as i32
        } else if 149 as i32 == 'U' as i32 {
            20 as i32
        } else if 149 as i32 == 'V' as i32 {
            21 as i32
        } else if 149 as i32 == 'W' as i32 {
            22 as i32
        } else if 149 as i32 == 'X' as i32 {
            23 as i32
        } else if 149 as i32 == 'Y' as i32 {
            24 as i32
        } else if 149 as i32 == 'Z' as i32 {
            25 as i32
        } else if 149 as i32 == '2' as i32 {
            26 as i32
        } else if 149 as i32 == '3' as i32 {
            27 as i32
        } else if 149 as i32 == '4' as i32 {
            28 as i32
        } else if 149 as i32 == '5' as i32 {
            29 as i32
        } else if 149 as i32 == '6' as i32 {
            30 as i32
        } else if 149 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 150 as i32 == 'A' as i32 {
            0 as i32
        } else if 150 as i32 == 'B' as i32 {
            1 as i32
        } else if 150 as i32 == 'C' as i32 {
            2 as i32
        } else if 150 as i32 == 'D' as i32 {
            3 as i32
        } else if 150 as i32 == 'E' as i32 {
            4 as i32
        } else if 150 as i32 == 'F' as i32 {
            5 as i32
        } else if 150 as i32 == 'G' as i32 {
            6 as i32
        } else if 150 as i32 == 'H' as i32 {
            7 as i32
        } else if 150 as i32 == 'I' as i32 {
            8 as i32
        } else if 150 as i32 == 'J' as i32 {
            9 as i32
        } else if 150 as i32 == 'K' as i32 {
            10 as i32
        } else if 150 as i32 == 'L' as i32 {
            11 as i32
        } else if 150 as i32 == 'M' as i32 {
            12 as i32
        } else if 150 as i32 == 'N' as i32 {
            13 as i32
        } else if 150 as i32 == 'O' as i32 {
            14 as i32
        } else if 150 as i32 == 'P' as i32 {
            15 as i32
        } else if 150 as i32 == 'Q' as i32 {
            16 as i32
        } else if 150 as i32 == 'R' as i32 {
            17 as i32
        } else if 150 as i32 == 'S' as i32 {
            18 as i32
        } else if 150 as i32 == 'T' as i32 {
            19 as i32
        } else if 150 as i32 == 'U' as i32 {
            20 as i32
        } else if 150 as i32 == 'V' as i32 {
            21 as i32
        } else if 150 as i32 == 'W' as i32 {
            22 as i32
        } else if 150 as i32 == 'X' as i32 {
            23 as i32
        } else if 150 as i32 == 'Y' as i32 {
            24 as i32
        } else if 150 as i32 == 'Z' as i32 {
            25 as i32
        } else if 150 as i32 == '2' as i32 {
            26 as i32
        } else if 150 as i32 == '3' as i32 {
            27 as i32
        } else if 150 as i32 == '4' as i32 {
            28 as i32
        } else if 150 as i32 == '5' as i32 {
            29 as i32
        } else if 150 as i32 == '6' as i32 {
            30 as i32
        } else if 150 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 151 as i32 == 'A' as i32 {
            0 as i32
        } else if 151 as i32 == 'B' as i32 {
            1 as i32
        } else if 151 as i32 == 'C' as i32 {
            2 as i32
        } else if 151 as i32 == 'D' as i32 {
            3 as i32
        } else if 151 as i32 == 'E' as i32 {
            4 as i32
        } else if 151 as i32 == 'F' as i32 {
            5 as i32
        } else if 151 as i32 == 'G' as i32 {
            6 as i32
        } else if 151 as i32 == 'H' as i32 {
            7 as i32
        } else if 151 as i32 == 'I' as i32 {
            8 as i32
        } else if 151 as i32 == 'J' as i32 {
            9 as i32
        } else if 151 as i32 == 'K' as i32 {
            10 as i32
        } else if 151 as i32 == 'L' as i32 {
            11 as i32
        } else if 151 as i32 == 'M' as i32 {
            12 as i32
        } else if 151 as i32 == 'N' as i32 {
            13 as i32
        } else if 151 as i32 == 'O' as i32 {
            14 as i32
        } else if 151 as i32 == 'P' as i32 {
            15 as i32
        } else if 151 as i32 == 'Q' as i32 {
            16 as i32
        } else if 151 as i32 == 'R' as i32 {
            17 as i32
        } else if 151 as i32 == 'S' as i32 {
            18 as i32
        } else if 151 as i32 == 'T' as i32 {
            19 as i32
        } else if 151 as i32 == 'U' as i32 {
            20 as i32
        } else if 151 as i32 == 'V' as i32 {
            21 as i32
        } else if 151 as i32 == 'W' as i32 {
            22 as i32
        } else if 151 as i32 == 'X' as i32 {
            23 as i32
        } else if 151 as i32 == 'Y' as i32 {
            24 as i32
        } else if 151 as i32 == 'Z' as i32 {
            25 as i32
        } else if 151 as i32 == '2' as i32 {
            26 as i32
        } else if 151 as i32 == '3' as i32 {
            27 as i32
        } else if 151 as i32 == '4' as i32 {
            28 as i32
        } else if 151 as i32 == '5' as i32 {
            29 as i32
        } else if 151 as i32 == '6' as i32 {
            30 as i32
        } else if 151 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 152 as i32 == 'A' as i32 {
            0 as i32
        } else if 152 as i32 == 'B' as i32 {
            1 as i32
        } else if 152 as i32 == 'C' as i32 {
            2 as i32
        } else if 152 as i32 == 'D' as i32 {
            3 as i32
        } else if 152 as i32 == 'E' as i32 {
            4 as i32
        } else if 152 as i32 == 'F' as i32 {
            5 as i32
        } else if 152 as i32 == 'G' as i32 {
            6 as i32
        } else if 152 as i32 == 'H' as i32 {
            7 as i32
        } else if 152 as i32 == 'I' as i32 {
            8 as i32
        } else if 152 as i32 == 'J' as i32 {
            9 as i32
        } else if 152 as i32 == 'K' as i32 {
            10 as i32
        } else if 152 as i32 == 'L' as i32 {
            11 as i32
        } else if 152 as i32 == 'M' as i32 {
            12 as i32
        } else if 152 as i32 == 'N' as i32 {
            13 as i32
        } else if 152 as i32 == 'O' as i32 {
            14 as i32
        } else if 152 as i32 == 'P' as i32 {
            15 as i32
        } else if 152 as i32 == 'Q' as i32 {
            16 as i32
        } else if 152 as i32 == 'R' as i32 {
            17 as i32
        } else if 152 as i32 == 'S' as i32 {
            18 as i32
        } else if 152 as i32 == 'T' as i32 {
            19 as i32
        } else if 152 as i32 == 'U' as i32 {
            20 as i32
        } else if 152 as i32 == 'V' as i32 {
            21 as i32
        } else if 152 as i32 == 'W' as i32 {
            22 as i32
        } else if 152 as i32 == 'X' as i32 {
            23 as i32
        } else if 152 as i32 == 'Y' as i32 {
            24 as i32
        } else if 152 as i32 == 'Z' as i32 {
            25 as i32
        } else if 152 as i32 == '2' as i32 {
            26 as i32
        } else if 152 as i32 == '3' as i32 {
            27 as i32
        } else if 152 as i32 == '4' as i32 {
            28 as i32
        } else if 152 as i32 == '5' as i32 {
            29 as i32
        } else if 152 as i32 == '6' as i32 {
            30 as i32
        } else if 152 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 153 as i32 == 'A' as i32 {
            0 as i32
        } else if 153 as i32 == 'B' as i32 {
            1 as i32
        } else if 153 as i32 == 'C' as i32 {
            2 as i32
        } else if 153 as i32 == 'D' as i32 {
            3 as i32
        } else if 153 as i32 == 'E' as i32 {
            4 as i32
        } else if 153 as i32 == 'F' as i32 {
            5 as i32
        } else if 153 as i32 == 'G' as i32 {
            6 as i32
        } else if 153 as i32 == 'H' as i32 {
            7 as i32
        } else if 153 as i32 == 'I' as i32 {
            8 as i32
        } else if 153 as i32 == 'J' as i32 {
            9 as i32
        } else if 153 as i32 == 'K' as i32 {
            10 as i32
        } else if 153 as i32 == 'L' as i32 {
            11 as i32
        } else if 153 as i32 == 'M' as i32 {
            12 as i32
        } else if 153 as i32 == 'N' as i32 {
            13 as i32
        } else if 153 as i32 == 'O' as i32 {
            14 as i32
        } else if 153 as i32 == 'P' as i32 {
            15 as i32
        } else if 153 as i32 == 'Q' as i32 {
            16 as i32
        } else if 153 as i32 == 'R' as i32 {
            17 as i32
        } else if 153 as i32 == 'S' as i32 {
            18 as i32
        } else if 153 as i32 == 'T' as i32 {
            19 as i32
        } else if 153 as i32 == 'U' as i32 {
            20 as i32
        } else if 153 as i32 == 'V' as i32 {
            21 as i32
        } else if 153 as i32 == 'W' as i32 {
            22 as i32
        } else if 153 as i32 == 'X' as i32 {
            23 as i32
        } else if 153 as i32 == 'Y' as i32 {
            24 as i32
        } else if 153 as i32 == 'Z' as i32 {
            25 as i32
        } else if 153 as i32 == '2' as i32 {
            26 as i32
        } else if 153 as i32 == '3' as i32 {
            27 as i32
        } else if 153 as i32 == '4' as i32 {
            28 as i32
        } else if 153 as i32 == '5' as i32 {
            29 as i32
        } else if 153 as i32 == '6' as i32 {
            30 as i32
        } else if 153 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 154 as i32 == 'A' as i32 {
            0 as i32
        } else if 154 as i32 == 'B' as i32 {
            1 as i32
        } else if 154 as i32 == 'C' as i32 {
            2 as i32
        } else if 154 as i32 == 'D' as i32 {
            3 as i32
        } else if 154 as i32 == 'E' as i32 {
            4 as i32
        } else if 154 as i32 == 'F' as i32 {
            5 as i32
        } else if 154 as i32 == 'G' as i32 {
            6 as i32
        } else if 154 as i32 == 'H' as i32 {
            7 as i32
        } else if 154 as i32 == 'I' as i32 {
            8 as i32
        } else if 154 as i32 == 'J' as i32 {
            9 as i32
        } else if 154 as i32 == 'K' as i32 {
            10 as i32
        } else if 154 as i32 == 'L' as i32 {
            11 as i32
        } else if 154 as i32 == 'M' as i32 {
            12 as i32
        } else if 154 as i32 == 'N' as i32 {
            13 as i32
        } else if 154 as i32 == 'O' as i32 {
            14 as i32
        } else if 154 as i32 == 'P' as i32 {
            15 as i32
        } else if 154 as i32 == 'Q' as i32 {
            16 as i32
        } else if 154 as i32 == 'R' as i32 {
            17 as i32
        } else if 154 as i32 == 'S' as i32 {
            18 as i32
        } else if 154 as i32 == 'T' as i32 {
            19 as i32
        } else if 154 as i32 == 'U' as i32 {
            20 as i32
        } else if 154 as i32 == 'V' as i32 {
            21 as i32
        } else if 154 as i32 == 'W' as i32 {
            22 as i32
        } else if 154 as i32 == 'X' as i32 {
            23 as i32
        } else if 154 as i32 == 'Y' as i32 {
            24 as i32
        } else if 154 as i32 == 'Z' as i32 {
            25 as i32
        } else if 154 as i32 == '2' as i32 {
            26 as i32
        } else if 154 as i32 == '3' as i32 {
            27 as i32
        } else if 154 as i32 == '4' as i32 {
            28 as i32
        } else if 154 as i32 == '5' as i32 {
            29 as i32
        } else if 154 as i32 == '6' as i32 {
            30 as i32
        } else if 154 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 155 as i32 == 'A' as i32 {
            0 as i32
        } else if 155 as i32 == 'B' as i32 {
            1 as i32
        } else if 155 as i32 == 'C' as i32 {
            2 as i32
        } else if 155 as i32 == 'D' as i32 {
            3 as i32
        } else if 155 as i32 == 'E' as i32 {
            4 as i32
        } else if 155 as i32 == 'F' as i32 {
            5 as i32
        } else if 155 as i32 == 'G' as i32 {
            6 as i32
        } else if 155 as i32 == 'H' as i32 {
            7 as i32
        } else if 155 as i32 == 'I' as i32 {
            8 as i32
        } else if 155 as i32 == 'J' as i32 {
            9 as i32
        } else if 155 as i32 == 'K' as i32 {
            10 as i32
        } else if 155 as i32 == 'L' as i32 {
            11 as i32
        } else if 155 as i32 == 'M' as i32 {
            12 as i32
        } else if 155 as i32 == 'N' as i32 {
            13 as i32
        } else if 155 as i32 == 'O' as i32 {
            14 as i32
        } else if 155 as i32 == 'P' as i32 {
            15 as i32
        } else if 155 as i32 == 'Q' as i32 {
            16 as i32
        } else if 155 as i32 == 'R' as i32 {
            17 as i32
        } else if 155 as i32 == 'S' as i32 {
            18 as i32
        } else if 155 as i32 == 'T' as i32 {
            19 as i32
        } else if 155 as i32 == 'U' as i32 {
            20 as i32
        } else if 155 as i32 == 'V' as i32 {
            21 as i32
        } else if 155 as i32 == 'W' as i32 {
            22 as i32
        } else if 155 as i32 == 'X' as i32 {
            23 as i32
        } else if 155 as i32 == 'Y' as i32 {
            24 as i32
        } else if 155 as i32 == 'Z' as i32 {
            25 as i32
        } else if 155 as i32 == '2' as i32 {
            26 as i32
        } else if 155 as i32 == '3' as i32 {
            27 as i32
        } else if 155 as i32 == '4' as i32 {
            28 as i32
        } else if 155 as i32 == '5' as i32 {
            29 as i32
        } else if 155 as i32 == '6' as i32 {
            30 as i32
        } else if 155 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 156 as i32 == 'A' as i32 {
            0 as i32
        } else if 156 as i32 == 'B' as i32 {
            1 as i32
        } else if 156 as i32 == 'C' as i32 {
            2 as i32
        } else if 156 as i32 == 'D' as i32 {
            3 as i32
        } else if 156 as i32 == 'E' as i32 {
            4 as i32
        } else if 156 as i32 == 'F' as i32 {
            5 as i32
        } else if 156 as i32 == 'G' as i32 {
            6 as i32
        } else if 156 as i32 == 'H' as i32 {
            7 as i32
        } else if 156 as i32 == 'I' as i32 {
            8 as i32
        } else if 156 as i32 == 'J' as i32 {
            9 as i32
        } else if 156 as i32 == 'K' as i32 {
            10 as i32
        } else if 156 as i32 == 'L' as i32 {
            11 as i32
        } else if 156 as i32 == 'M' as i32 {
            12 as i32
        } else if 156 as i32 == 'N' as i32 {
            13 as i32
        } else if 156 as i32 == 'O' as i32 {
            14 as i32
        } else if 156 as i32 == 'P' as i32 {
            15 as i32
        } else if 156 as i32 == 'Q' as i32 {
            16 as i32
        } else if 156 as i32 == 'R' as i32 {
            17 as i32
        } else if 156 as i32 == 'S' as i32 {
            18 as i32
        } else if 156 as i32 == 'T' as i32 {
            19 as i32
        } else if 156 as i32 == 'U' as i32 {
            20 as i32
        } else if 156 as i32 == 'V' as i32 {
            21 as i32
        } else if 156 as i32 == 'W' as i32 {
            22 as i32
        } else if 156 as i32 == 'X' as i32 {
            23 as i32
        } else if 156 as i32 == 'Y' as i32 {
            24 as i32
        } else if 156 as i32 == 'Z' as i32 {
            25 as i32
        } else if 156 as i32 == '2' as i32 {
            26 as i32
        } else if 156 as i32 == '3' as i32 {
            27 as i32
        } else if 156 as i32 == '4' as i32 {
            28 as i32
        } else if 156 as i32 == '5' as i32 {
            29 as i32
        } else if 156 as i32 == '6' as i32 {
            30 as i32
        } else if 156 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 157 as i32 == 'A' as i32 {
            0 as i32
        } else if 157 as i32 == 'B' as i32 {
            1 as i32
        } else if 157 as i32 == 'C' as i32 {
            2 as i32
        } else if 157 as i32 == 'D' as i32 {
            3 as i32
        } else if 157 as i32 == 'E' as i32 {
            4 as i32
        } else if 157 as i32 == 'F' as i32 {
            5 as i32
        } else if 157 as i32 == 'G' as i32 {
            6 as i32
        } else if 157 as i32 == 'H' as i32 {
            7 as i32
        } else if 157 as i32 == 'I' as i32 {
            8 as i32
        } else if 157 as i32 == 'J' as i32 {
            9 as i32
        } else if 157 as i32 == 'K' as i32 {
            10 as i32
        } else if 157 as i32 == 'L' as i32 {
            11 as i32
        } else if 157 as i32 == 'M' as i32 {
            12 as i32
        } else if 157 as i32 == 'N' as i32 {
            13 as i32
        } else if 157 as i32 == 'O' as i32 {
            14 as i32
        } else if 157 as i32 == 'P' as i32 {
            15 as i32
        } else if 157 as i32 == 'Q' as i32 {
            16 as i32
        } else if 157 as i32 == 'R' as i32 {
            17 as i32
        } else if 157 as i32 == 'S' as i32 {
            18 as i32
        } else if 157 as i32 == 'T' as i32 {
            19 as i32
        } else if 157 as i32 == 'U' as i32 {
            20 as i32
        } else if 157 as i32 == 'V' as i32 {
            21 as i32
        } else if 157 as i32 == 'W' as i32 {
            22 as i32
        } else if 157 as i32 == 'X' as i32 {
            23 as i32
        } else if 157 as i32 == 'Y' as i32 {
            24 as i32
        } else if 157 as i32 == 'Z' as i32 {
            25 as i32
        } else if 157 as i32 == '2' as i32 {
            26 as i32
        } else if 157 as i32 == '3' as i32 {
            27 as i32
        } else if 157 as i32 == '4' as i32 {
            28 as i32
        } else if 157 as i32 == '5' as i32 {
            29 as i32
        } else if 157 as i32 == '6' as i32 {
            30 as i32
        } else if 157 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 158 as i32 == 'A' as i32 {
            0 as i32
        } else if 158 as i32 == 'B' as i32 {
            1 as i32
        } else if 158 as i32 == 'C' as i32 {
            2 as i32
        } else if 158 as i32 == 'D' as i32 {
            3 as i32
        } else if 158 as i32 == 'E' as i32 {
            4 as i32
        } else if 158 as i32 == 'F' as i32 {
            5 as i32
        } else if 158 as i32 == 'G' as i32 {
            6 as i32
        } else if 158 as i32 == 'H' as i32 {
            7 as i32
        } else if 158 as i32 == 'I' as i32 {
            8 as i32
        } else if 158 as i32 == 'J' as i32 {
            9 as i32
        } else if 158 as i32 == 'K' as i32 {
            10 as i32
        } else if 158 as i32 == 'L' as i32 {
            11 as i32
        } else if 158 as i32 == 'M' as i32 {
            12 as i32
        } else if 158 as i32 == 'N' as i32 {
            13 as i32
        } else if 158 as i32 == 'O' as i32 {
            14 as i32
        } else if 158 as i32 == 'P' as i32 {
            15 as i32
        } else if 158 as i32 == 'Q' as i32 {
            16 as i32
        } else if 158 as i32 == 'R' as i32 {
            17 as i32
        } else if 158 as i32 == 'S' as i32 {
            18 as i32
        } else if 158 as i32 == 'T' as i32 {
            19 as i32
        } else if 158 as i32 == 'U' as i32 {
            20 as i32
        } else if 158 as i32 == 'V' as i32 {
            21 as i32
        } else if 158 as i32 == 'W' as i32 {
            22 as i32
        } else if 158 as i32 == 'X' as i32 {
            23 as i32
        } else if 158 as i32 == 'Y' as i32 {
            24 as i32
        } else if 158 as i32 == 'Z' as i32 {
            25 as i32
        } else if 158 as i32 == '2' as i32 {
            26 as i32
        } else if 158 as i32 == '3' as i32 {
            27 as i32
        } else if 158 as i32 == '4' as i32 {
            28 as i32
        } else if 158 as i32 == '5' as i32 {
            29 as i32
        } else if 158 as i32 == '6' as i32 {
            30 as i32
        } else if 158 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 159 as i32 == 'A' as i32 {
            0 as i32
        } else if 159 as i32 == 'B' as i32 {
            1 as i32
        } else if 159 as i32 == 'C' as i32 {
            2 as i32
        } else if 159 as i32 == 'D' as i32 {
            3 as i32
        } else if 159 as i32 == 'E' as i32 {
            4 as i32
        } else if 159 as i32 == 'F' as i32 {
            5 as i32
        } else if 159 as i32 == 'G' as i32 {
            6 as i32
        } else if 159 as i32 == 'H' as i32 {
            7 as i32
        } else if 159 as i32 == 'I' as i32 {
            8 as i32
        } else if 159 as i32 == 'J' as i32 {
            9 as i32
        } else if 159 as i32 == 'K' as i32 {
            10 as i32
        } else if 159 as i32 == 'L' as i32 {
            11 as i32
        } else if 159 as i32 == 'M' as i32 {
            12 as i32
        } else if 159 as i32 == 'N' as i32 {
            13 as i32
        } else if 159 as i32 == 'O' as i32 {
            14 as i32
        } else if 159 as i32 == 'P' as i32 {
            15 as i32
        } else if 159 as i32 == 'Q' as i32 {
            16 as i32
        } else if 159 as i32 == 'R' as i32 {
            17 as i32
        } else if 159 as i32 == 'S' as i32 {
            18 as i32
        } else if 159 as i32 == 'T' as i32 {
            19 as i32
        } else if 159 as i32 == 'U' as i32 {
            20 as i32
        } else if 159 as i32 == 'V' as i32 {
            21 as i32
        } else if 159 as i32 == 'W' as i32 {
            22 as i32
        } else if 159 as i32 == 'X' as i32 {
            23 as i32
        } else if 159 as i32 == 'Y' as i32 {
            24 as i32
        } else if 159 as i32 == 'Z' as i32 {
            25 as i32
        } else if 159 as i32 == '2' as i32 {
            26 as i32
        } else if 159 as i32 == '3' as i32 {
            27 as i32
        } else if 159 as i32 == '4' as i32 {
            28 as i32
        } else if 159 as i32 == '5' as i32 {
            29 as i32
        } else if 159 as i32 == '6' as i32 {
            30 as i32
        } else if 159 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 160 as i32 == 'A' as i32 {
            0 as i32
        } else if 160 as i32 == 'B' as i32 {
            1 as i32
        } else if 160 as i32 == 'C' as i32 {
            2 as i32
        } else if 160 as i32 == 'D' as i32 {
            3 as i32
        } else if 160 as i32 == 'E' as i32 {
            4 as i32
        } else if 160 as i32 == 'F' as i32 {
            5 as i32
        } else if 160 as i32 == 'G' as i32 {
            6 as i32
        } else if 160 as i32 == 'H' as i32 {
            7 as i32
        } else if 160 as i32 == 'I' as i32 {
            8 as i32
        } else if 160 as i32 == 'J' as i32 {
            9 as i32
        } else if 160 as i32 == 'K' as i32 {
            10 as i32
        } else if 160 as i32 == 'L' as i32 {
            11 as i32
        } else if 160 as i32 == 'M' as i32 {
            12 as i32
        } else if 160 as i32 == 'N' as i32 {
            13 as i32
        } else if 160 as i32 == 'O' as i32 {
            14 as i32
        } else if 160 as i32 == 'P' as i32 {
            15 as i32
        } else if 160 as i32 == 'Q' as i32 {
            16 as i32
        } else if 160 as i32 == 'R' as i32 {
            17 as i32
        } else if 160 as i32 == 'S' as i32 {
            18 as i32
        } else if 160 as i32 == 'T' as i32 {
            19 as i32
        } else if 160 as i32 == 'U' as i32 {
            20 as i32
        } else if 160 as i32 == 'V' as i32 {
            21 as i32
        } else if 160 as i32 == 'W' as i32 {
            22 as i32
        } else if 160 as i32 == 'X' as i32 {
            23 as i32
        } else if 160 as i32 == 'Y' as i32 {
            24 as i32
        } else if 160 as i32 == 'Z' as i32 {
            25 as i32
        } else if 160 as i32 == '2' as i32 {
            26 as i32
        } else if 160 as i32 == '3' as i32 {
            27 as i32
        } else if 160 as i32 == '4' as i32 {
            28 as i32
        } else if 160 as i32 == '5' as i32 {
            29 as i32
        } else if 160 as i32 == '6' as i32 {
            30 as i32
        } else if 160 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 161 as i32 == 'A' as i32 {
            0 as i32
        } else if 161 as i32 == 'B' as i32 {
            1 as i32
        } else if 161 as i32 == 'C' as i32 {
            2 as i32
        } else if 161 as i32 == 'D' as i32 {
            3 as i32
        } else if 161 as i32 == 'E' as i32 {
            4 as i32
        } else if 161 as i32 == 'F' as i32 {
            5 as i32
        } else if 161 as i32 == 'G' as i32 {
            6 as i32
        } else if 161 as i32 == 'H' as i32 {
            7 as i32
        } else if 161 as i32 == 'I' as i32 {
            8 as i32
        } else if 161 as i32 == 'J' as i32 {
            9 as i32
        } else if 161 as i32 == 'K' as i32 {
            10 as i32
        } else if 161 as i32 == 'L' as i32 {
            11 as i32
        } else if 161 as i32 == 'M' as i32 {
            12 as i32
        } else if 161 as i32 == 'N' as i32 {
            13 as i32
        } else if 161 as i32 == 'O' as i32 {
            14 as i32
        } else if 161 as i32 == 'P' as i32 {
            15 as i32
        } else if 161 as i32 == 'Q' as i32 {
            16 as i32
        } else if 161 as i32 == 'R' as i32 {
            17 as i32
        } else if 161 as i32 == 'S' as i32 {
            18 as i32
        } else if 161 as i32 == 'T' as i32 {
            19 as i32
        } else if 161 as i32 == 'U' as i32 {
            20 as i32
        } else if 161 as i32 == 'V' as i32 {
            21 as i32
        } else if 161 as i32 == 'W' as i32 {
            22 as i32
        } else if 161 as i32 == 'X' as i32 {
            23 as i32
        } else if 161 as i32 == 'Y' as i32 {
            24 as i32
        } else if 161 as i32 == 'Z' as i32 {
            25 as i32
        } else if 161 as i32 == '2' as i32 {
            26 as i32
        } else if 161 as i32 == '3' as i32 {
            27 as i32
        } else if 161 as i32 == '4' as i32 {
            28 as i32
        } else if 161 as i32 == '5' as i32 {
            29 as i32
        } else if 161 as i32 == '6' as i32 {
            30 as i32
        } else if 161 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 162 as i32 == 'A' as i32 {
            0 as i32
        } else if 162 as i32 == 'B' as i32 {
            1 as i32
        } else if 162 as i32 == 'C' as i32 {
            2 as i32
        } else if 162 as i32 == 'D' as i32 {
            3 as i32
        } else if 162 as i32 == 'E' as i32 {
            4 as i32
        } else if 162 as i32 == 'F' as i32 {
            5 as i32
        } else if 162 as i32 == 'G' as i32 {
            6 as i32
        } else if 162 as i32 == 'H' as i32 {
            7 as i32
        } else if 162 as i32 == 'I' as i32 {
            8 as i32
        } else if 162 as i32 == 'J' as i32 {
            9 as i32
        } else if 162 as i32 == 'K' as i32 {
            10 as i32
        } else if 162 as i32 == 'L' as i32 {
            11 as i32
        } else if 162 as i32 == 'M' as i32 {
            12 as i32
        } else if 162 as i32 == 'N' as i32 {
            13 as i32
        } else if 162 as i32 == 'O' as i32 {
            14 as i32
        } else if 162 as i32 == 'P' as i32 {
            15 as i32
        } else if 162 as i32 == 'Q' as i32 {
            16 as i32
        } else if 162 as i32 == 'R' as i32 {
            17 as i32
        } else if 162 as i32 == 'S' as i32 {
            18 as i32
        } else if 162 as i32 == 'T' as i32 {
            19 as i32
        } else if 162 as i32 == 'U' as i32 {
            20 as i32
        } else if 162 as i32 == 'V' as i32 {
            21 as i32
        } else if 162 as i32 == 'W' as i32 {
            22 as i32
        } else if 162 as i32 == 'X' as i32 {
            23 as i32
        } else if 162 as i32 == 'Y' as i32 {
            24 as i32
        } else if 162 as i32 == 'Z' as i32 {
            25 as i32
        } else if 162 as i32 == '2' as i32 {
            26 as i32
        } else if 162 as i32 == '3' as i32 {
            27 as i32
        } else if 162 as i32 == '4' as i32 {
            28 as i32
        } else if 162 as i32 == '5' as i32 {
            29 as i32
        } else if 162 as i32 == '6' as i32 {
            30 as i32
        } else if 162 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 163 as i32 == 'A' as i32 {
            0 as i32
        } else if 163 as i32 == 'B' as i32 {
            1 as i32
        } else if 163 as i32 == 'C' as i32 {
            2 as i32
        } else if 163 as i32 == 'D' as i32 {
            3 as i32
        } else if 163 as i32 == 'E' as i32 {
            4 as i32
        } else if 163 as i32 == 'F' as i32 {
            5 as i32
        } else if 163 as i32 == 'G' as i32 {
            6 as i32
        } else if 163 as i32 == 'H' as i32 {
            7 as i32
        } else if 163 as i32 == 'I' as i32 {
            8 as i32
        } else if 163 as i32 == 'J' as i32 {
            9 as i32
        } else if 163 as i32 == 'K' as i32 {
            10 as i32
        } else if 163 as i32 == 'L' as i32 {
            11 as i32
        } else if 163 as i32 == 'M' as i32 {
            12 as i32
        } else if 163 as i32 == 'N' as i32 {
            13 as i32
        } else if 163 as i32 == 'O' as i32 {
            14 as i32
        } else if 163 as i32 == 'P' as i32 {
            15 as i32
        } else if 163 as i32 == 'Q' as i32 {
            16 as i32
        } else if 163 as i32 == 'R' as i32 {
            17 as i32
        } else if 163 as i32 == 'S' as i32 {
            18 as i32
        } else if 163 as i32 == 'T' as i32 {
            19 as i32
        } else if 163 as i32 == 'U' as i32 {
            20 as i32
        } else if 163 as i32 == 'V' as i32 {
            21 as i32
        } else if 163 as i32 == 'W' as i32 {
            22 as i32
        } else if 163 as i32 == 'X' as i32 {
            23 as i32
        } else if 163 as i32 == 'Y' as i32 {
            24 as i32
        } else if 163 as i32 == 'Z' as i32 {
            25 as i32
        } else if 163 as i32 == '2' as i32 {
            26 as i32
        } else if 163 as i32 == '3' as i32 {
            27 as i32
        } else if 163 as i32 == '4' as i32 {
            28 as i32
        } else if 163 as i32 == '5' as i32 {
            29 as i32
        } else if 163 as i32 == '6' as i32 {
            30 as i32
        } else if 163 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 132 as i32 == 'A' as i32 {
            0 as i32
        } else if 132 as i32 == 'B' as i32 {
            1 as i32
        } else if 132 as i32 == 'C' as i32 {
            2 as i32
        } else if 132 as i32 == 'D' as i32 {
            3 as i32
        } else if 132 as i32 == 'E' as i32 {
            4 as i32
        } else if 132 as i32 == 'F' as i32 {
            5 as i32
        } else if 132 as i32 == 'G' as i32 {
            6 as i32
        } else if 132 as i32 == 'H' as i32 {
            7 as i32
        } else if 132 as i32 == 'I' as i32 {
            8 as i32
        } else if 132 as i32 == 'J' as i32 {
            9 as i32
        } else if 132 as i32 == 'K' as i32 {
            10 as i32
        } else if 132 as i32 == 'L' as i32 {
            11 as i32
        } else if 132 as i32 == 'M' as i32 {
            12 as i32
        } else if 132 as i32 == 'N' as i32 {
            13 as i32
        } else if 132 as i32 == 'O' as i32 {
            14 as i32
        } else if 132 as i32 == 'P' as i32 {
            15 as i32
        } else if 132 as i32 == 'Q' as i32 {
            16 as i32
        } else if 132 as i32 == 'R' as i32 {
            17 as i32
        } else if 132 as i32 == 'S' as i32 {
            18 as i32
        } else if 132 as i32 == 'T' as i32 {
            19 as i32
        } else if 132 as i32 == 'U' as i32 {
            20 as i32
        } else if 132 as i32 == 'V' as i32 {
            21 as i32
        } else if 132 as i32 == 'W' as i32 {
            22 as i32
        } else if 132 as i32 == 'X' as i32 {
            23 as i32
        } else if 132 as i32 == 'Y' as i32 {
            24 as i32
        } else if 132 as i32 == 'Z' as i32 {
            25 as i32
        } else if 132 as i32 == '2' as i32 {
            26 as i32
        } else if 132 as i32 == '3' as i32 {
            27 as i32
        } else if 132 as i32 == '4' as i32 {
            28 as i32
        } else if 132 as i32 == '5' as i32 {
            29 as i32
        } else if 132 as i32 == '6' as i32 {
            30 as i32
        } else if 132 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 165 as i32 == 'A' as i32 {
            0 as i32
        } else if 165 as i32 == 'B' as i32 {
            1 as i32
        } else if 165 as i32 == 'C' as i32 {
            2 as i32
        } else if 165 as i32 == 'D' as i32 {
            3 as i32
        } else if 165 as i32 == 'E' as i32 {
            4 as i32
        } else if 165 as i32 == 'F' as i32 {
            5 as i32
        } else if 165 as i32 == 'G' as i32 {
            6 as i32
        } else if 165 as i32 == 'H' as i32 {
            7 as i32
        } else if 165 as i32 == 'I' as i32 {
            8 as i32
        } else if 165 as i32 == 'J' as i32 {
            9 as i32
        } else if 165 as i32 == 'K' as i32 {
            10 as i32
        } else if 165 as i32 == 'L' as i32 {
            11 as i32
        } else if 165 as i32 == 'M' as i32 {
            12 as i32
        } else if 165 as i32 == 'N' as i32 {
            13 as i32
        } else if 165 as i32 == 'O' as i32 {
            14 as i32
        } else if 165 as i32 == 'P' as i32 {
            15 as i32
        } else if 165 as i32 == 'Q' as i32 {
            16 as i32
        } else if 165 as i32 == 'R' as i32 {
            17 as i32
        } else if 165 as i32 == 'S' as i32 {
            18 as i32
        } else if 165 as i32 == 'T' as i32 {
            19 as i32
        } else if 165 as i32 == 'U' as i32 {
            20 as i32
        } else if 165 as i32 == 'V' as i32 {
            21 as i32
        } else if 165 as i32 == 'W' as i32 {
            22 as i32
        } else if 165 as i32 == 'X' as i32 {
            23 as i32
        } else if 165 as i32 == 'Y' as i32 {
            24 as i32
        } else if 165 as i32 == 'Z' as i32 {
            25 as i32
        } else if 165 as i32 == '2' as i32 {
            26 as i32
        } else if 165 as i32 == '3' as i32 {
            27 as i32
        } else if 165 as i32 == '4' as i32 {
            28 as i32
        } else if 165 as i32 == '5' as i32 {
            29 as i32
        } else if 165 as i32 == '6' as i32 {
            30 as i32
        } else if 165 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 166 as i32 == 'A' as i32 {
            0 as i32
        } else if 166 as i32 == 'B' as i32 {
            1 as i32
        } else if 166 as i32 == 'C' as i32 {
            2 as i32
        } else if 166 as i32 == 'D' as i32 {
            3 as i32
        } else if 166 as i32 == 'E' as i32 {
            4 as i32
        } else if 166 as i32 == 'F' as i32 {
            5 as i32
        } else if 166 as i32 == 'G' as i32 {
            6 as i32
        } else if 166 as i32 == 'H' as i32 {
            7 as i32
        } else if 166 as i32 == 'I' as i32 {
            8 as i32
        } else if 166 as i32 == 'J' as i32 {
            9 as i32
        } else if 166 as i32 == 'K' as i32 {
            10 as i32
        } else if 166 as i32 == 'L' as i32 {
            11 as i32
        } else if 166 as i32 == 'M' as i32 {
            12 as i32
        } else if 166 as i32 == 'N' as i32 {
            13 as i32
        } else if 166 as i32 == 'O' as i32 {
            14 as i32
        } else if 166 as i32 == 'P' as i32 {
            15 as i32
        } else if 166 as i32 == 'Q' as i32 {
            16 as i32
        } else if 166 as i32 == 'R' as i32 {
            17 as i32
        } else if 166 as i32 == 'S' as i32 {
            18 as i32
        } else if 166 as i32 == 'T' as i32 {
            19 as i32
        } else if 166 as i32 == 'U' as i32 {
            20 as i32
        } else if 166 as i32 == 'V' as i32 {
            21 as i32
        } else if 166 as i32 == 'W' as i32 {
            22 as i32
        } else if 166 as i32 == 'X' as i32 {
            23 as i32
        } else if 166 as i32 == 'Y' as i32 {
            24 as i32
        } else if 166 as i32 == 'Z' as i32 {
            25 as i32
        } else if 166 as i32 == '2' as i32 {
            26 as i32
        } else if 166 as i32 == '3' as i32 {
            27 as i32
        } else if 166 as i32 == '4' as i32 {
            28 as i32
        } else if 166 as i32 == '5' as i32 {
            29 as i32
        } else if 166 as i32 == '6' as i32 {
            30 as i32
        } else if 166 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 167 as i32 == 'A' as i32 {
            0 as i32
        } else if 167 as i32 == 'B' as i32 {
            1 as i32
        } else if 167 as i32 == 'C' as i32 {
            2 as i32
        } else if 167 as i32 == 'D' as i32 {
            3 as i32
        } else if 167 as i32 == 'E' as i32 {
            4 as i32
        } else if 167 as i32 == 'F' as i32 {
            5 as i32
        } else if 167 as i32 == 'G' as i32 {
            6 as i32
        } else if 167 as i32 == 'H' as i32 {
            7 as i32
        } else if 167 as i32 == 'I' as i32 {
            8 as i32
        } else if 167 as i32 == 'J' as i32 {
            9 as i32
        } else if 167 as i32 == 'K' as i32 {
            10 as i32
        } else if 167 as i32 == 'L' as i32 {
            11 as i32
        } else if 167 as i32 == 'M' as i32 {
            12 as i32
        } else if 167 as i32 == 'N' as i32 {
            13 as i32
        } else if 167 as i32 == 'O' as i32 {
            14 as i32
        } else if 167 as i32 == 'P' as i32 {
            15 as i32
        } else if 167 as i32 == 'Q' as i32 {
            16 as i32
        } else if 167 as i32 == 'R' as i32 {
            17 as i32
        } else if 167 as i32 == 'S' as i32 {
            18 as i32
        } else if 167 as i32 == 'T' as i32 {
            19 as i32
        } else if 167 as i32 == 'U' as i32 {
            20 as i32
        } else if 167 as i32 == 'V' as i32 {
            21 as i32
        } else if 167 as i32 == 'W' as i32 {
            22 as i32
        } else if 167 as i32 == 'X' as i32 {
            23 as i32
        } else if 167 as i32 == 'Y' as i32 {
            24 as i32
        } else if 167 as i32 == 'Z' as i32 {
            25 as i32
        } else if 167 as i32 == '2' as i32 {
            26 as i32
        } else if 167 as i32 == '3' as i32 {
            27 as i32
        } else if 167 as i32 == '4' as i32 {
            28 as i32
        } else if 167 as i32 == '5' as i32 {
            29 as i32
        } else if 167 as i32 == '6' as i32 {
            30 as i32
        } else if 167 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 168 as i32 == 'A' as i32 {
            0 as i32
        } else if 168 as i32 == 'B' as i32 {
            1 as i32
        } else if 168 as i32 == 'C' as i32 {
            2 as i32
        } else if 168 as i32 == 'D' as i32 {
            3 as i32
        } else if 168 as i32 == 'E' as i32 {
            4 as i32
        } else if 168 as i32 == 'F' as i32 {
            5 as i32
        } else if 168 as i32 == 'G' as i32 {
            6 as i32
        } else if 168 as i32 == 'H' as i32 {
            7 as i32
        } else if 168 as i32 == 'I' as i32 {
            8 as i32
        } else if 168 as i32 == 'J' as i32 {
            9 as i32
        } else if 168 as i32 == 'K' as i32 {
            10 as i32
        } else if 168 as i32 == 'L' as i32 {
            11 as i32
        } else if 168 as i32 == 'M' as i32 {
            12 as i32
        } else if 168 as i32 == 'N' as i32 {
            13 as i32
        } else if 168 as i32 == 'O' as i32 {
            14 as i32
        } else if 168 as i32 == 'P' as i32 {
            15 as i32
        } else if 168 as i32 == 'Q' as i32 {
            16 as i32
        } else if 168 as i32 == 'R' as i32 {
            17 as i32
        } else if 168 as i32 == 'S' as i32 {
            18 as i32
        } else if 168 as i32 == 'T' as i32 {
            19 as i32
        } else if 168 as i32 == 'U' as i32 {
            20 as i32
        } else if 168 as i32 == 'V' as i32 {
            21 as i32
        } else if 168 as i32 == 'W' as i32 {
            22 as i32
        } else if 168 as i32 == 'X' as i32 {
            23 as i32
        } else if 168 as i32 == 'Y' as i32 {
            24 as i32
        } else if 168 as i32 == 'Z' as i32 {
            25 as i32
        } else if 168 as i32 == '2' as i32 {
            26 as i32
        } else if 168 as i32 == '3' as i32 {
            27 as i32
        } else if 168 as i32 == '4' as i32 {
            28 as i32
        } else if 168 as i32 == '5' as i32 {
            29 as i32
        } else if 168 as i32 == '6' as i32 {
            30 as i32
        } else if 168 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 169 as i32 == 'A' as i32 {
            0 as i32
        } else if 169 as i32 == 'B' as i32 {
            1 as i32
        } else if 169 as i32 == 'C' as i32 {
            2 as i32
        } else if 169 as i32 == 'D' as i32 {
            3 as i32
        } else if 169 as i32 == 'E' as i32 {
            4 as i32
        } else if 169 as i32 == 'F' as i32 {
            5 as i32
        } else if 169 as i32 == 'G' as i32 {
            6 as i32
        } else if 169 as i32 == 'H' as i32 {
            7 as i32
        } else if 169 as i32 == 'I' as i32 {
            8 as i32
        } else if 169 as i32 == 'J' as i32 {
            9 as i32
        } else if 169 as i32 == 'K' as i32 {
            10 as i32
        } else if 169 as i32 == 'L' as i32 {
            11 as i32
        } else if 169 as i32 == 'M' as i32 {
            12 as i32
        } else if 169 as i32 == 'N' as i32 {
            13 as i32
        } else if 169 as i32 == 'O' as i32 {
            14 as i32
        } else if 169 as i32 == 'P' as i32 {
            15 as i32
        } else if 169 as i32 == 'Q' as i32 {
            16 as i32
        } else if 169 as i32 == 'R' as i32 {
            17 as i32
        } else if 169 as i32 == 'S' as i32 {
            18 as i32
        } else if 169 as i32 == 'T' as i32 {
            19 as i32
        } else if 169 as i32 == 'U' as i32 {
            20 as i32
        } else if 169 as i32 == 'V' as i32 {
            21 as i32
        } else if 169 as i32 == 'W' as i32 {
            22 as i32
        } else if 169 as i32 == 'X' as i32 {
            23 as i32
        } else if 169 as i32 == 'Y' as i32 {
            24 as i32
        } else if 169 as i32 == 'Z' as i32 {
            25 as i32
        } else if 169 as i32 == '2' as i32 {
            26 as i32
        } else if 169 as i32 == '3' as i32 {
            27 as i32
        } else if 169 as i32 == '4' as i32 {
            28 as i32
        } else if 169 as i32 == '5' as i32 {
            29 as i32
        } else if 169 as i32 == '6' as i32 {
            30 as i32
        } else if 169 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 170 as i32 == 'A' as i32 {
            0 as i32
        } else if 170 as i32 == 'B' as i32 {
            1 as i32
        } else if 170 as i32 == 'C' as i32 {
            2 as i32
        } else if 170 as i32 == 'D' as i32 {
            3 as i32
        } else if 170 as i32 == 'E' as i32 {
            4 as i32
        } else if 170 as i32 == 'F' as i32 {
            5 as i32
        } else if 170 as i32 == 'G' as i32 {
            6 as i32
        } else if 170 as i32 == 'H' as i32 {
            7 as i32
        } else if 170 as i32 == 'I' as i32 {
            8 as i32
        } else if 170 as i32 == 'J' as i32 {
            9 as i32
        } else if 170 as i32 == 'K' as i32 {
            10 as i32
        } else if 170 as i32 == 'L' as i32 {
            11 as i32
        } else if 170 as i32 == 'M' as i32 {
            12 as i32
        } else if 170 as i32 == 'N' as i32 {
            13 as i32
        } else if 170 as i32 == 'O' as i32 {
            14 as i32
        } else if 170 as i32 == 'P' as i32 {
            15 as i32
        } else if 170 as i32 == 'Q' as i32 {
            16 as i32
        } else if 170 as i32 == 'R' as i32 {
            17 as i32
        } else if 170 as i32 == 'S' as i32 {
            18 as i32
        } else if 170 as i32 == 'T' as i32 {
            19 as i32
        } else if 170 as i32 == 'U' as i32 {
            20 as i32
        } else if 170 as i32 == 'V' as i32 {
            21 as i32
        } else if 170 as i32 == 'W' as i32 {
            22 as i32
        } else if 170 as i32 == 'X' as i32 {
            23 as i32
        } else if 170 as i32 == 'Y' as i32 {
            24 as i32
        } else if 170 as i32 == 'Z' as i32 {
            25 as i32
        } else if 170 as i32 == '2' as i32 {
            26 as i32
        } else if 170 as i32 == '3' as i32 {
            27 as i32
        } else if 170 as i32 == '4' as i32 {
            28 as i32
        } else if 170 as i32 == '5' as i32 {
            29 as i32
        } else if 170 as i32 == '6' as i32 {
            30 as i32
        } else if 170 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 171 as i32 == 'A' as i32 {
            0 as i32
        } else if 171 as i32 == 'B' as i32 {
            1 as i32
        } else if 171 as i32 == 'C' as i32 {
            2 as i32
        } else if 171 as i32 == 'D' as i32 {
            3 as i32
        } else if 171 as i32 == 'E' as i32 {
            4 as i32
        } else if 171 as i32 == 'F' as i32 {
            5 as i32
        } else if 171 as i32 == 'G' as i32 {
            6 as i32
        } else if 171 as i32 == 'H' as i32 {
            7 as i32
        } else if 171 as i32 == 'I' as i32 {
            8 as i32
        } else if 171 as i32 == 'J' as i32 {
            9 as i32
        } else if 171 as i32 == 'K' as i32 {
            10 as i32
        } else if 171 as i32 == 'L' as i32 {
            11 as i32
        } else if 171 as i32 == 'M' as i32 {
            12 as i32
        } else if 171 as i32 == 'N' as i32 {
            13 as i32
        } else if 171 as i32 == 'O' as i32 {
            14 as i32
        } else if 171 as i32 == 'P' as i32 {
            15 as i32
        } else if 171 as i32 == 'Q' as i32 {
            16 as i32
        } else if 171 as i32 == 'R' as i32 {
            17 as i32
        } else if 171 as i32 == 'S' as i32 {
            18 as i32
        } else if 171 as i32 == 'T' as i32 {
            19 as i32
        } else if 171 as i32 == 'U' as i32 {
            20 as i32
        } else if 171 as i32 == 'V' as i32 {
            21 as i32
        } else if 171 as i32 == 'W' as i32 {
            22 as i32
        } else if 171 as i32 == 'X' as i32 {
            23 as i32
        } else if 171 as i32 == 'Y' as i32 {
            24 as i32
        } else if 171 as i32 == 'Z' as i32 {
            25 as i32
        } else if 171 as i32 == '2' as i32 {
            26 as i32
        } else if 171 as i32 == '3' as i32 {
            27 as i32
        } else if 171 as i32 == '4' as i32 {
            28 as i32
        } else if 171 as i32 == '5' as i32 {
            29 as i32
        } else if 171 as i32 == '6' as i32 {
            30 as i32
        } else if 171 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 172 as i32 == 'A' as i32 {
            0 as i32
        } else if 172 as i32 == 'B' as i32 {
            1 as i32
        } else if 172 as i32 == 'C' as i32 {
            2 as i32
        } else if 172 as i32 == 'D' as i32 {
            3 as i32
        } else if 172 as i32 == 'E' as i32 {
            4 as i32
        } else if 172 as i32 == 'F' as i32 {
            5 as i32
        } else if 172 as i32 == 'G' as i32 {
            6 as i32
        } else if 172 as i32 == 'H' as i32 {
            7 as i32
        } else if 172 as i32 == 'I' as i32 {
            8 as i32
        } else if 172 as i32 == 'J' as i32 {
            9 as i32
        } else if 172 as i32 == 'K' as i32 {
            10 as i32
        } else if 172 as i32 == 'L' as i32 {
            11 as i32
        } else if 172 as i32 == 'M' as i32 {
            12 as i32
        } else if 172 as i32 == 'N' as i32 {
            13 as i32
        } else if 172 as i32 == 'O' as i32 {
            14 as i32
        } else if 172 as i32 == 'P' as i32 {
            15 as i32
        } else if 172 as i32 == 'Q' as i32 {
            16 as i32
        } else if 172 as i32 == 'R' as i32 {
            17 as i32
        } else if 172 as i32 == 'S' as i32 {
            18 as i32
        } else if 172 as i32 == 'T' as i32 {
            19 as i32
        } else if 172 as i32 == 'U' as i32 {
            20 as i32
        } else if 172 as i32 == 'V' as i32 {
            21 as i32
        } else if 172 as i32 == 'W' as i32 {
            22 as i32
        } else if 172 as i32 == 'X' as i32 {
            23 as i32
        } else if 172 as i32 == 'Y' as i32 {
            24 as i32
        } else if 172 as i32 == 'Z' as i32 {
            25 as i32
        } else if 172 as i32 == '2' as i32 {
            26 as i32
        } else if 172 as i32 == '3' as i32 {
            27 as i32
        } else if 172 as i32 == '4' as i32 {
            28 as i32
        } else if 172 as i32 == '5' as i32 {
            29 as i32
        } else if 172 as i32 == '6' as i32 {
            30 as i32
        } else if 172 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 173 as i32 == 'A' as i32 {
            0 as i32
        } else if 173 as i32 == 'B' as i32 {
            1 as i32
        } else if 173 as i32 == 'C' as i32 {
            2 as i32
        } else if 173 as i32 == 'D' as i32 {
            3 as i32
        } else if 173 as i32 == 'E' as i32 {
            4 as i32
        } else if 173 as i32 == 'F' as i32 {
            5 as i32
        } else if 173 as i32 == 'G' as i32 {
            6 as i32
        } else if 173 as i32 == 'H' as i32 {
            7 as i32
        } else if 173 as i32 == 'I' as i32 {
            8 as i32
        } else if 173 as i32 == 'J' as i32 {
            9 as i32
        } else if 173 as i32 == 'K' as i32 {
            10 as i32
        } else if 173 as i32 == 'L' as i32 {
            11 as i32
        } else if 173 as i32 == 'M' as i32 {
            12 as i32
        } else if 173 as i32 == 'N' as i32 {
            13 as i32
        } else if 173 as i32 == 'O' as i32 {
            14 as i32
        } else if 173 as i32 == 'P' as i32 {
            15 as i32
        } else if 173 as i32 == 'Q' as i32 {
            16 as i32
        } else if 173 as i32 == 'R' as i32 {
            17 as i32
        } else if 173 as i32 == 'S' as i32 {
            18 as i32
        } else if 173 as i32 == 'T' as i32 {
            19 as i32
        } else if 173 as i32 == 'U' as i32 {
            20 as i32
        } else if 173 as i32 == 'V' as i32 {
            21 as i32
        } else if 173 as i32 == 'W' as i32 {
            22 as i32
        } else if 173 as i32 == 'X' as i32 {
            23 as i32
        } else if 173 as i32 == 'Y' as i32 {
            24 as i32
        } else if 173 as i32 == 'Z' as i32 {
            25 as i32
        } else if 173 as i32 == '2' as i32 {
            26 as i32
        } else if 173 as i32 == '3' as i32 {
            27 as i32
        } else if 173 as i32 == '4' as i32 {
            28 as i32
        } else if 173 as i32 == '5' as i32 {
            29 as i32
        } else if 173 as i32 == '6' as i32 {
            30 as i32
        } else if 173 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 174 as i32 == 'A' as i32 {
            0 as i32
        } else if 174 as i32 == 'B' as i32 {
            1 as i32
        } else if 174 as i32 == 'C' as i32 {
            2 as i32
        } else if 174 as i32 == 'D' as i32 {
            3 as i32
        } else if 174 as i32 == 'E' as i32 {
            4 as i32
        } else if 174 as i32 == 'F' as i32 {
            5 as i32
        } else if 174 as i32 == 'G' as i32 {
            6 as i32
        } else if 174 as i32 == 'H' as i32 {
            7 as i32
        } else if 174 as i32 == 'I' as i32 {
            8 as i32
        } else if 174 as i32 == 'J' as i32 {
            9 as i32
        } else if 174 as i32 == 'K' as i32 {
            10 as i32
        } else if 174 as i32 == 'L' as i32 {
            11 as i32
        } else if 174 as i32 == 'M' as i32 {
            12 as i32
        } else if 174 as i32 == 'N' as i32 {
            13 as i32
        } else if 174 as i32 == 'O' as i32 {
            14 as i32
        } else if 174 as i32 == 'P' as i32 {
            15 as i32
        } else if 174 as i32 == 'Q' as i32 {
            16 as i32
        } else if 174 as i32 == 'R' as i32 {
            17 as i32
        } else if 174 as i32 == 'S' as i32 {
            18 as i32
        } else if 174 as i32 == 'T' as i32 {
            19 as i32
        } else if 174 as i32 == 'U' as i32 {
            20 as i32
        } else if 174 as i32 == 'V' as i32 {
            21 as i32
        } else if 174 as i32 == 'W' as i32 {
            22 as i32
        } else if 174 as i32 == 'X' as i32 {
            23 as i32
        } else if 174 as i32 == 'Y' as i32 {
            24 as i32
        } else if 174 as i32 == 'Z' as i32 {
            25 as i32
        } else if 174 as i32 == '2' as i32 {
            26 as i32
        } else if 174 as i32 == '3' as i32 {
            27 as i32
        } else if 174 as i32 == '4' as i32 {
            28 as i32
        } else if 174 as i32 == '5' as i32 {
            29 as i32
        } else if 174 as i32 == '6' as i32 {
            30 as i32
        } else if 174 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 175 as i32 == 'A' as i32 {
            0 as i32
        } else if 175 as i32 == 'B' as i32 {
            1 as i32
        } else if 175 as i32 == 'C' as i32 {
            2 as i32
        } else if 175 as i32 == 'D' as i32 {
            3 as i32
        } else if 175 as i32 == 'E' as i32 {
            4 as i32
        } else if 175 as i32 == 'F' as i32 {
            5 as i32
        } else if 175 as i32 == 'G' as i32 {
            6 as i32
        } else if 175 as i32 == 'H' as i32 {
            7 as i32
        } else if 175 as i32 == 'I' as i32 {
            8 as i32
        } else if 175 as i32 == 'J' as i32 {
            9 as i32
        } else if 175 as i32 == 'K' as i32 {
            10 as i32
        } else if 175 as i32 == 'L' as i32 {
            11 as i32
        } else if 175 as i32 == 'M' as i32 {
            12 as i32
        } else if 175 as i32 == 'N' as i32 {
            13 as i32
        } else if 175 as i32 == 'O' as i32 {
            14 as i32
        } else if 175 as i32 == 'P' as i32 {
            15 as i32
        } else if 175 as i32 == 'Q' as i32 {
            16 as i32
        } else if 175 as i32 == 'R' as i32 {
            17 as i32
        } else if 175 as i32 == 'S' as i32 {
            18 as i32
        } else if 175 as i32 == 'T' as i32 {
            19 as i32
        } else if 175 as i32 == 'U' as i32 {
            20 as i32
        } else if 175 as i32 == 'V' as i32 {
            21 as i32
        } else if 175 as i32 == 'W' as i32 {
            22 as i32
        } else if 175 as i32 == 'X' as i32 {
            23 as i32
        } else if 175 as i32 == 'Y' as i32 {
            24 as i32
        } else if 175 as i32 == 'Z' as i32 {
            25 as i32
        } else if 175 as i32 == '2' as i32 {
            26 as i32
        } else if 175 as i32 == '3' as i32 {
            27 as i32
        } else if 175 as i32 == '4' as i32 {
            28 as i32
        } else if 175 as i32 == '5' as i32 {
            29 as i32
        } else if 175 as i32 == '6' as i32 {
            30 as i32
        } else if 175 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 176 as i32 == 'A' as i32 {
            0 as i32
        } else if 176 as i32 == 'B' as i32 {
            1 as i32
        } else if 176 as i32 == 'C' as i32 {
            2 as i32
        } else if 176 as i32 == 'D' as i32 {
            3 as i32
        } else if 176 as i32 == 'E' as i32 {
            4 as i32
        } else if 176 as i32 == 'F' as i32 {
            5 as i32
        } else if 176 as i32 == 'G' as i32 {
            6 as i32
        } else if 176 as i32 == 'H' as i32 {
            7 as i32
        } else if 176 as i32 == 'I' as i32 {
            8 as i32
        } else if 176 as i32 == 'J' as i32 {
            9 as i32
        } else if 176 as i32 == 'K' as i32 {
            10 as i32
        } else if 176 as i32 == 'L' as i32 {
            11 as i32
        } else if 176 as i32 == 'M' as i32 {
            12 as i32
        } else if 176 as i32 == 'N' as i32 {
            13 as i32
        } else if 176 as i32 == 'O' as i32 {
            14 as i32
        } else if 176 as i32 == 'P' as i32 {
            15 as i32
        } else if 176 as i32 == 'Q' as i32 {
            16 as i32
        } else if 176 as i32 == 'R' as i32 {
            17 as i32
        } else if 176 as i32 == 'S' as i32 {
            18 as i32
        } else if 176 as i32 == 'T' as i32 {
            19 as i32
        } else if 176 as i32 == 'U' as i32 {
            20 as i32
        } else if 176 as i32 == 'V' as i32 {
            21 as i32
        } else if 176 as i32 == 'W' as i32 {
            22 as i32
        } else if 176 as i32 == 'X' as i32 {
            23 as i32
        } else if 176 as i32 == 'Y' as i32 {
            24 as i32
        } else if 176 as i32 == 'Z' as i32 {
            25 as i32
        } else if 176 as i32 == '2' as i32 {
            26 as i32
        } else if 176 as i32 == '3' as i32 {
            27 as i32
        } else if 176 as i32 == '4' as i32 {
            28 as i32
        } else if 176 as i32 == '5' as i32 {
            29 as i32
        } else if 176 as i32 == '6' as i32 {
            30 as i32
        } else if 176 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 177 as i32 == 'A' as i32 {
            0 as i32
        } else if 177 as i32 == 'B' as i32 {
            1 as i32
        } else if 177 as i32 == 'C' as i32 {
            2 as i32
        } else if 177 as i32 == 'D' as i32 {
            3 as i32
        } else if 177 as i32 == 'E' as i32 {
            4 as i32
        } else if 177 as i32 == 'F' as i32 {
            5 as i32
        } else if 177 as i32 == 'G' as i32 {
            6 as i32
        } else if 177 as i32 == 'H' as i32 {
            7 as i32
        } else if 177 as i32 == 'I' as i32 {
            8 as i32
        } else if 177 as i32 == 'J' as i32 {
            9 as i32
        } else if 177 as i32 == 'K' as i32 {
            10 as i32
        } else if 177 as i32 == 'L' as i32 {
            11 as i32
        } else if 177 as i32 == 'M' as i32 {
            12 as i32
        } else if 177 as i32 == 'N' as i32 {
            13 as i32
        } else if 177 as i32 == 'O' as i32 {
            14 as i32
        } else if 177 as i32 == 'P' as i32 {
            15 as i32
        } else if 177 as i32 == 'Q' as i32 {
            16 as i32
        } else if 177 as i32 == 'R' as i32 {
            17 as i32
        } else if 177 as i32 == 'S' as i32 {
            18 as i32
        } else if 177 as i32 == 'T' as i32 {
            19 as i32
        } else if 177 as i32 == 'U' as i32 {
            20 as i32
        } else if 177 as i32 == 'V' as i32 {
            21 as i32
        } else if 177 as i32 == 'W' as i32 {
            22 as i32
        } else if 177 as i32 == 'X' as i32 {
            23 as i32
        } else if 177 as i32 == 'Y' as i32 {
            24 as i32
        } else if 177 as i32 == 'Z' as i32 {
            25 as i32
        } else if 177 as i32 == '2' as i32 {
            26 as i32
        } else if 177 as i32 == '3' as i32 {
            27 as i32
        } else if 177 as i32 == '4' as i32 {
            28 as i32
        } else if 177 as i32 == '5' as i32 {
            29 as i32
        } else if 177 as i32 == '6' as i32 {
            30 as i32
        } else if 177 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 178 as i32 == 'A' as i32 {
            0 as i32
        } else if 178 as i32 == 'B' as i32 {
            1 as i32
        } else if 178 as i32 == 'C' as i32 {
            2 as i32
        } else if 178 as i32 == 'D' as i32 {
            3 as i32
        } else if 178 as i32 == 'E' as i32 {
            4 as i32
        } else if 178 as i32 == 'F' as i32 {
            5 as i32
        } else if 178 as i32 == 'G' as i32 {
            6 as i32
        } else if 178 as i32 == 'H' as i32 {
            7 as i32
        } else if 178 as i32 == 'I' as i32 {
            8 as i32
        } else if 178 as i32 == 'J' as i32 {
            9 as i32
        } else if 178 as i32 == 'K' as i32 {
            10 as i32
        } else if 178 as i32 == 'L' as i32 {
            11 as i32
        } else if 178 as i32 == 'M' as i32 {
            12 as i32
        } else if 178 as i32 == 'N' as i32 {
            13 as i32
        } else if 178 as i32 == 'O' as i32 {
            14 as i32
        } else if 178 as i32 == 'P' as i32 {
            15 as i32
        } else if 178 as i32 == 'Q' as i32 {
            16 as i32
        } else if 178 as i32 == 'R' as i32 {
            17 as i32
        } else if 178 as i32 == 'S' as i32 {
            18 as i32
        } else if 178 as i32 == 'T' as i32 {
            19 as i32
        } else if 178 as i32 == 'U' as i32 {
            20 as i32
        } else if 178 as i32 == 'V' as i32 {
            21 as i32
        } else if 178 as i32 == 'W' as i32 {
            22 as i32
        } else if 178 as i32 == 'X' as i32 {
            23 as i32
        } else if 178 as i32 == 'Y' as i32 {
            24 as i32
        } else if 178 as i32 == 'Z' as i32 {
            25 as i32
        } else if 178 as i32 == '2' as i32 {
            26 as i32
        } else if 178 as i32 == '3' as i32 {
            27 as i32
        } else if 178 as i32 == '4' as i32 {
            28 as i32
        } else if 178 as i32 == '5' as i32 {
            29 as i32
        } else if 178 as i32 == '6' as i32 {
            30 as i32
        } else if 178 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 179 as i32 == 'A' as i32 {
            0 as i32
        } else if 179 as i32 == 'B' as i32 {
            1 as i32
        } else if 179 as i32 == 'C' as i32 {
            2 as i32
        } else if 179 as i32 == 'D' as i32 {
            3 as i32
        } else if 179 as i32 == 'E' as i32 {
            4 as i32
        } else if 179 as i32 == 'F' as i32 {
            5 as i32
        } else if 179 as i32 == 'G' as i32 {
            6 as i32
        } else if 179 as i32 == 'H' as i32 {
            7 as i32
        } else if 179 as i32 == 'I' as i32 {
            8 as i32
        } else if 179 as i32 == 'J' as i32 {
            9 as i32
        } else if 179 as i32 == 'K' as i32 {
            10 as i32
        } else if 179 as i32 == 'L' as i32 {
            11 as i32
        } else if 179 as i32 == 'M' as i32 {
            12 as i32
        } else if 179 as i32 == 'N' as i32 {
            13 as i32
        } else if 179 as i32 == 'O' as i32 {
            14 as i32
        } else if 179 as i32 == 'P' as i32 {
            15 as i32
        } else if 179 as i32 == 'Q' as i32 {
            16 as i32
        } else if 179 as i32 == 'R' as i32 {
            17 as i32
        } else if 179 as i32 == 'S' as i32 {
            18 as i32
        } else if 179 as i32 == 'T' as i32 {
            19 as i32
        } else if 179 as i32 == 'U' as i32 {
            20 as i32
        } else if 179 as i32 == 'V' as i32 {
            21 as i32
        } else if 179 as i32 == 'W' as i32 {
            22 as i32
        } else if 179 as i32 == 'X' as i32 {
            23 as i32
        } else if 179 as i32 == 'Y' as i32 {
            24 as i32
        } else if 179 as i32 == 'Z' as i32 {
            25 as i32
        } else if 179 as i32 == '2' as i32 {
            26 as i32
        } else if 179 as i32 == '3' as i32 {
            27 as i32
        } else if 179 as i32 == '4' as i32 {
            28 as i32
        } else if 179 as i32 == '5' as i32 {
            29 as i32
        } else if 179 as i32 == '6' as i32 {
            30 as i32
        } else if 179 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 180 as i32 == 'A' as i32 {
            0 as i32
        } else if 180 as i32 == 'B' as i32 {
            1 as i32
        } else if 180 as i32 == 'C' as i32 {
            2 as i32
        } else if 180 as i32 == 'D' as i32 {
            3 as i32
        } else if 180 as i32 == 'E' as i32 {
            4 as i32
        } else if 180 as i32 == 'F' as i32 {
            5 as i32
        } else if 180 as i32 == 'G' as i32 {
            6 as i32
        } else if 180 as i32 == 'H' as i32 {
            7 as i32
        } else if 180 as i32 == 'I' as i32 {
            8 as i32
        } else if 180 as i32 == 'J' as i32 {
            9 as i32
        } else if 180 as i32 == 'K' as i32 {
            10 as i32
        } else if 180 as i32 == 'L' as i32 {
            11 as i32
        } else if 180 as i32 == 'M' as i32 {
            12 as i32
        } else if 180 as i32 == 'N' as i32 {
            13 as i32
        } else if 180 as i32 == 'O' as i32 {
            14 as i32
        } else if 180 as i32 == 'P' as i32 {
            15 as i32
        } else if 180 as i32 == 'Q' as i32 {
            16 as i32
        } else if 180 as i32 == 'R' as i32 {
            17 as i32
        } else if 180 as i32 == 'S' as i32 {
            18 as i32
        } else if 180 as i32 == 'T' as i32 {
            19 as i32
        } else if 180 as i32 == 'U' as i32 {
            20 as i32
        } else if 180 as i32 == 'V' as i32 {
            21 as i32
        } else if 180 as i32 == 'W' as i32 {
            22 as i32
        } else if 180 as i32 == 'X' as i32 {
            23 as i32
        } else if 180 as i32 == 'Y' as i32 {
            24 as i32
        } else if 180 as i32 == 'Z' as i32 {
            25 as i32
        } else if 180 as i32 == '2' as i32 {
            26 as i32
        } else if 180 as i32 == '3' as i32 {
            27 as i32
        } else if 180 as i32 == '4' as i32 {
            28 as i32
        } else if 180 as i32 == '5' as i32 {
            29 as i32
        } else if 180 as i32 == '6' as i32 {
            30 as i32
        } else if 180 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 181 as i32 == 'A' as i32 {
            0 as i32
        } else if 181 as i32 == 'B' as i32 {
            1 as i32
        } else if 181 as i32 == 'C' as i32 {
            2 as i32
        } else if 181 as i32 == 'D' as i32 {
            3 as i32
        } else if 181 as i32 == 'E' as i32 {
            4 as i32
        } else if 181 as i32 == 'F' as i32 {
            5 as i32
        } else if 181 as i32 == 'G' as i32 {
            6 as i32
        } else if 181 as i32 == 'H' as i32 {
            7 as i32
        } else if 181 as i32 == 'I' as i32 {
            8 as i32
        } else if 181 as i32 == 'J' as i32 {
            9 as i32
        } else if 181 as i32 == 'K' as i32 {
            10 as i32
        } else if 181 as i32 == 'L' as i32 {
            11 as i32
        } else if 181 as i32 == 'M' as i32 {
            12 as i32
        } else if 181 as i32 == 'N' as i32 {
            13 as i32
        } else if 181 as i32 == 'O' as i32 {
            14 as i32
        } else if 181 as i32 == 'P' as i32 {
            15 as i32
        } else if 181 as i32 == 'Q' as i32 {
            16 as i32
        } else if 181 as i32 == 'R' as i32 {
            17 as i32
        } else if 181 as i32 == 'S' as i32 {
            18 as i32
        } else if 181 as i32 == 'T' as i32 {
            19 as i32
        } else if 181 as i32 == 'U' as i32 {
            20 as i32
        } else if 181 as i32 == 'V' as i32 {
            21 as i32
        } else if 181 as i32 == 'W' as i32 {
            22 as i32
        } else if 181 as i32 == 'X' as i32 {
            23 as i32
        } else if 181 as i32 == 'Y' as i32 {
            24 as i32
        } else if 181 as i32 == 'Z' as i32 {
            25 as i32
        } else if 181 as i32 == '2' as i32 {
            26 as i32
        } else if 181 as i32 == '3' as i32 {
            27 as i32
        } else if 181 as i32 == '4' as i32 {
            28 as i32
        } else if 181 as i32 == '5' as i32 {
            29 as i32
        } else if 181 as i32 == '6' as i32 {
            30 as i32
        } else if 181 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 182 as i32 == 'A' as i32 {
            0 as i32
        } else if 182 as i32 == 'B' as i32 {
            1 as i32
        } else if 182 as i32 == 'C' as i32 {
            2 as i32
        } else if 182 as i32 == 'D' as i32 {
            3 as i32
        } else if 182 as i32 == 'E' as i32 {
            4 as i32
        } else if 182 as i32 == 'F' as i32 {
            5 as i32
        } else if 182 as i32 == 'G' as i32 {
            6 as i32
        } else if 182 as i32 == 'H' as i32 {
            7 as i32
        } else if 182 as i32 == 'I' as i32 {
            8 as i32
        } else if 182 as i32 == 'J' as i32 {
            9 as i32
        } else if 182 as i32 == 'K' as i32 {
            10 as i32
        } else if 182 as i32 == 'L' as i32 {
            11 as i32
        } else if 182 as i32 == 'M' as i32 {
            12 as i32
        } else if 182 as i32 == 'N' as i32 {
            13 as i32
        } else if 182 as i32 == 'O' as i32 {
            14 as i32
        } else if 182 as i32 == 'P' as i32 {
            15 as i32
        } else if 182 as i32 == 'Q' as i32 {
            16 as i32
        } else if 182 as i32 == 'R' as i32 {
            17 as i32
        } else if 182 as i32 == 'S' as i32 {
            18 as i32
        } else if 182 as i32 == 'T' as i32 {
            19 as i32
        } else if 182 as i32 == 'U' as i32 {
            20 as i32
        } else if 182 as i32 == 'V' as i32 {
            21 as i32
        } else if 182 as i32 == 'W' as i32 {
            22 as i32
        } else if 182 as i32 == 'X' as i32 {
            23 as i32
        } else if 182 as i32 == 'Y' as i32 {
            24 as i32
        } else if 182 as i32 == 'Z' as i32 {
            25 as i32
        } else if 182 as i32 == '2' as i32 {
            26 as i32
        } else if 182 as i32 == '3' as i32 {
            27 as i32
        } else if 182 as i32 == '4' as i32 {
            28 as i32
        } else if 182 as i32 == '5' as i32 {
            29 as i32
        } else if 182 as i32 == '6' as i32 {
            30 as i32
        } else if 182 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 183 as i32 == 'A' as i32 {
            0 as i32
        } else if 183 as i32 == 'B' as i32 {
            1 as i32
        } else if 183 as i32 == 'C' as i32 {
            2 as i32
        } else if 183 as i32 == 'D' as i32 {
            3 as i32
        } else if 183 as i32 == 'E' as i32 {
            4 as i32
        } else if 183 as i32 == 'F' as i32 {
            5 as i32
        } else if 183 as i32 == 'G' as i32 {
            6 as i32
        } else if 183 as i32 == 'H' as i32 {
            7 as i32
        } else if 183 as i32 == 'I' as i32 {
            8 as i32
        } else if 183 as i32 == 'J' as i32 {
            9 as i32
        } else if 183 as i32 == 'K' as i32 {
            10 as i32
        } else if 183 as i32 == 'L' as i32 {
            11 as i32
        } else if 183 as i32 == 'M' as i32 {
            12 as i32
        } else if 183 as i32 == 'N' as i32 {
            13 as i32
        } else if 183 as i32 == 'O' as i32 {
            14 as i32
        } else if 183 as i32 == 'P' as i32 {
            15 as i32
        } else if 183 as i32 == 'Q' as i32 {
            16 as i32
        } else if 183 as i32 == 'R' as i32 {
            17 as i32
        } else if 183 as i32 == 'S' as i32 {
            18 as i32
        } else if 183 as i32 == 'T' as i32 {
            19 as i32
        } else if 183 as i32 == 'U' as i32 {
            20 as i32
        } else if 183 as i32 == 'V' as i32 {
            21 as i32
        } else if 183 as i32 == 'W' as i32 {
            22 as i32
        } else if 183 as i32 == 'X' as i32 {
            23 as i32
        } else if 183 as i32 == 'Y' as i32 {
            24 as i32
        } else if 183 as i32 == 'Z' as i32 {
            25 as i32
        } else if 183 as i32 == '2' as i32 {
            26 as i32
        } else if 183 as i32 == '3' as i32 {
            27 as i32
        } else if 183 as i32 == '4' as i32 {
            28 as i32
        } else if 183 as i32 == '5' as i32 {
            29 as i32
        } else if 183 as i32 == '6' as i32 {
            30 as i32
        } else if 183 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 184 as i32 == 'A' as i32 {
            0 as i32
        } else if 184 as i32 == 'B' as i32 {
            1 as i32
        } else if 184 as i32 == 'C' as i32 {
            2 as i32
        } else if 184 as i32 == 'D' as i32 {
            3 as i32
        } else if 184 as i32 == 'E' as i32 {
            4 as i32
        } else if 184 as i32 == 'F' as i32 {
            5 as i32
        } else if 184 as i32 == 'G' as i32 {
            6 as i32
        } else if 184 as i32 == 'H' as i32 {
            7 as i32
        } else if 184 as i32 == 'I' as i32 {
            8 as i32
        } else if 184 as i32 == 'J' as i32 {
            9 as i32
        } else if 184 as i32 == 'K' as i32 {
            10 as i32
        } else if 184 as i32 == 'L' as i32 {
            11 as i32
        } else if 184 as i32 == 'M' as i32 {
            12 as i32
        } else if 184 as i32 == 'N' as i32 {
            13 as i32
        } else if 184 as i32 == 'O' as i32 {
            14 as i32
        } else if 184 as i32 == 'P' as i32 {
            15 as i32
        } else if 184 as i32 == 'Q' as i32 {
            16 as i32
        } else if 184 as i32 == 'R' as i32 {
            17 as i32
        } else if 184 as i32 == 'S' as i32 {
            18 as i32
        } else if 184 as i32 == 'T' as i32 {
            19 as i32
        } else if 184 as i32 == 'U' as i32 {
            20 as i32
        } else if 184 as i32 == 'V' as i32 {
            21 as i32
        } else if 184 as i32 == 'W' as i32 {
            22 as i32
        } else if 184 as i32 == 'X' as i32 {
            23 as i32
        } else if 184 as i32 == 'Y' as i32 {
            24 as i32
        } else if 184 as i32 == 'Z' as i32 {
            25 as i32
        } else if 184 as i32 == '2' as i32 {
            26 as i32
        } else if 184 as i32 == '3' as i32 {
            27 as i32
        } else if 184 as i32 == '4' as i32 {
            28 as i32
        } else if 184 as i32 == '5' as i32 {
            29 as i32
        } else if 184 as i32 == '6' as i32 {
            30 as i32
        } else if 184 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 185 as i32 == 'A' as i32 {
            0 as i32
        } else if 185 as i32 == 'B' as i32 {
            1 as i32
        } else if 185 as i32 == 'C' as i32 {
            2 as i32
        } else if 185 as i32 == 'D' as i32 {
            3 as i32
        } else if 185 as i32 == 'E' as i32 {
            4 as i32
        } else if 185 as i32 == 'F' as i32 {
            5 as i32
        } else if 185 as i32 == 'G' as i32 {
            6 as i32
        } else if 185 as i32 == 'H' as i32 {
            7 as i32
        } else if 185 as i32 == 'I' as i32 {
            8 as i32
        } else if 185 as i32 == 'J' as i32 {
            9 as i32
        } else if 185 as i32 == 'K' as i32 {
            10 as i32
        } else if 185 as i32 == 'L' as i32 {
            11 as i32
        } else if 185 as i32 == 'M' as i32 {
            12 as i32
        } else if 185 as i32 == 'N' as i32 {
            13 as i32
        } else if 185 as i32 == 'O' as i32 {
            14 as i32
        } else if 185 as i32 == 'P' as i32 {
            15 as i32
        } else if 185 as i32 == 'Q' as i32 {
            16 as i32
        } else if 185 as i32 == 'R' as i32 {
            17 as i32
        } else if 185 as i32 == 'S' as i32 {
            18 as i32
        } else if 185 as i32 == 'T' as i32 {
            19 as i32
        } else if 185 as i32 == 'U' as i32 {
            20 as i32
        } else if 185 as i32 == 'V' as i32 {
            21 as i32
        } else if 185 as i32 == 'W' as i32 {
            22 as i32
        } else if 185 as i32 == 'X' as i32 {
            23 as i32
        } else if 185 as i32 == 'Y' as i32 {
            24 as i32
        } else if 185 as i32 == 'Z' as i32 {
            25 as i32
        } else if 185 as i32 == '2' as i32 {
            26 as i32
        } else if 185 as i32 == '3' as i32 {
            27 as i32
        } else if 185 as i32 == '4' as i32 {
            28 as i32
        } else if 185 as i32 == '5' as i32 {
            29 as i32
        } else if 185 as i32 == '6' as i32 {
            30 as i32
        } else if 185 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 186 as i32 == 'A' as i32 {
            0 as i32
        } else if 186 as i32 == 'B' as i32 {
            1 as i32
        } else if 186 as i32 == 'C' as i32 {
            2 as i32
        } else if 186 as i32 == 'D' as i32 {
            3 as i32
        } else if 186 as i32 == 'E' as i32 {
            4 as i32
        } else if 186 as i32 == 'F' as i32 {
            5 as i32
        } else if 186 as i32 == 'G' as i32 {
            6 as i32
        } else if 186 as i32 == 'H' as i32 {
            7 as i32
        } else if 186 as i32 == 'I' as i32 {
            8 as i32
        } else if 186 as i32 == 'J' as i32 {
            9 as i32
        } else if 186 as i32 == 'K' as i32 {
            10 as i32
        } else if 186 as i32 == 'L' as i32 {
            11 as i32
        } else if 186 as i32 == 'M' as i32 {
            12 as i32
        } else if 186 as i32 == 'N' as i32 {
            13 as i32
        } else if 186 as i32 == 'O' as i32 {
            14 as i32
        } else if 186 as i32 == 'P' as i32 {
            15 as i32
        } else if 186 as i32 == 'Q' as i32 {
            16 as i32
        } else if 186 as i32 == 'R' as i32 {
            17 as i32
        } else if 186 as i32 == 'S' as i32 {
            18 as i32
        } else if 186 as i32 == 'T' as i32 {
            19 as i32
        } else if 186 as i32 == 'U' as i32 {
            20 as i32
        } else if 186 as i32 == 'V' as i32 {
            21 as i32
        } else if 186 as i32 == 'W' as i32 {
            22 as i32
        } else if 186 as i32 == 'X' as i32 {
            23 as i32
        } else if 186 as i32 == 'Y' as i32 {
            24 as i32
        } else if 186 as i32 == 'Z' as i32 {
            25 as i32
        } else if 186 as i32 == '2' as i32 {
            26 as i32
        } else if 186 as i32 == '3' as i32 {
            27 as i32
        } else if 186 as i32 == '4' as i32 {
            28 as i32
        } else if 186 as i32 == '5' as i32 {
            29 as i32
        } else if 186 as i32 == '6' as i32 {
            30 as i32
        } else if 186 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 187 as i32 == 'A' as i32 {
            0 as i32
        } else if 187 as i32 == 'B' as i32 {
            1 as i32
        } else if 187 as i32 == 'C' as i32 {
            2 as i32
        } else if 187 as i32 == 'D' as i32 {
            3 as i32
        } else if 187 as i32 == 'E' as i32 {
            4 as i32
        } else if 187 as i32 == 'F' as i32 {
            5 as i32
        } else if 187 as i32 == 'G' as i32 {
            6 as i32
        } else if 187 as i32 == 'H' as i32 {
            7 as i32
        } else if 187 as i32 == 'I' as i32 {
            8 as i32
        } else if 187 as i32 == 'J' as i32 {
            9 as i32
        } else if 187 as i32 == 'K' as i32 {
            10 as i32
        } else if 187 as i32 == 'L' as i32 {
            11 as i32
        } else if 187 as i32 == 'M' as i32 {
            12 as i32
        } else if 187 as i32 == 'N' as i32 {
            13 as i32
        } else if 187 as i32 == 'O' as i32 {
            14 as i32
        } else if 187 as i32 == 'P' as i32 {
            15 as i32
        } else if 187 as i32 == 'Q' as i32 {
            16 as i32
        } else if 187 as i32 == 'R' as i32 {
            17 as i32
        } else if 187 as i32 == 'S' as i32 {
            18 as i32
        } else if 187 as i32 == 'T' as i32 {
            19 as i32
        } else if 187 as i32 == 'U' as i32 {
            20 as i32
        } else if 187 as i32 == 'V' as i32 {
            21 as i32
        } else if 187 as i32 == 'W' as i32 {
            22 as i32
        } else if 187 as i32 == 'X' as i32 {
            23 as i32
        } else if 187 as i32 == 'Y' as i32 {
            24 as i32
        } else if 187 as i32 == 'Z' as i32 {
            25 as i32
        } else if 187 as i32 == '2' as i32 {
            26 as i32
        } else if 187 as i32 == '3' as i32 {
            27 as i32
        } else if 187 as i32 == '4' as i32 {
            28 as i32
        } else if 187 as i32 == '5' as i32 {
            29 as i32
        } else if 187 as i32 == '6' as i32 {
            30 as i32
        } else if 187 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 188 as i32 == 'A' as i32 {
            0 as i32
        } else if 188 as i32 == 'B' as i32 {
            1 as i32
        } else if 188 as i32 == 'C' as i32 {
            2 as i32
        } else if 188 as i32 == 'D' as i32 {
            3 as i32
        } else if 188 as i32 == 'E' as i32 {
            4 as i32
        } else if 188 as i32 == 'F' as i32 {
            5 as i32
        } else if 188 as i32 == 'G' as i32 {
            6 as i32
        } else if 188 as i32 == 'H' as i32 {
            7 as i32
        } else if 188 as i32 == 'I' as i32 {
            8 as i32
        } else if 188 as i32 == 'J' as i32 {
            9 as i32
        } else if 188 as i32 == 'K' as i32 {
            10 as i32
        } else if 188 as i32 == 'L' as i32 {
            11 as i32
        } else if 188 as i32 == 'M' as i32 {
            12 as i32
        } else if 188 as i32 == 'N' as i32 {
            13 as i32
        } else if 188 as i32 == 'O' as i32 {
            14 as i32
        } else if 188 as i32 == 'P' as i32 {
            15 as i32
        } else if 188 as i32 == 'Q' as i32 {
            16 as i32
        } else if 188 as i32 == 'R' as i32 {
            17 as i32
        } else if 188 as i32 == 'S' as i32 {
            18 as i32
        } else if 188 as i32 == 'T' as i32 {
            19 as i32
        } else if 188 as i32 == 'U' as i32 {
            20 as i32
        } else if 188 as i32 == 'V' as i32 {
            21 as i32
        } else if 188 as i32 == 'W' as i32 {
            22 as i32
        } else if 188 as i32 == 'X' as i32 {
            23 as i32
        } else if 188 as i32 == 'Y' as i32 {
            24 as i32
        } else if 188 as i32 == 'Z' as i32 {
            25 as i32
        } else if 188 as i32 == '2' as i32 {
            26 as i32
        } else if 188 as i32 == '3' as i32 {
            27 as i32
        } else if 188 as i32 == '4' as i32 {
            28 as i32
        } else if 188 as i32 == '5' as i32 {
            29 as i32
        } else if 188 as i32 == '6' as i32 {
            30 as i32
        } else if 188 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 189 as i32 == 'A' as i32 {
            0 as i32
        } else if 189 as i32 == 'B' as i32 {
            1 as i32
        } else if 189 as i32 == 'C' as i32 {
            2 as i32
        } else if 189 as i32 == 'D' as i32 {
            3 as i32
        } else if 189 as i32 == 'E' as i32 {
            4 as i32
        } else if 189 as i32 == 'F' as i32 {
            5 as i32
        } else if 189 as i32 == 'G' as i32 {
            6 as i32
        } else if 189 as i32 == 'H' as i32 {
            7 as i32
        } else if 189 as i32 == 'I' as i32 {
            8 as i32
        } else if 189 as i32 == 'J' as i32 {
            9 as i32
        } else if 189 as i32 == 'K' as i32 {
            10 as i32
        } else if 189 as i32 == 'L' as i32 {
            11 as i32
        } else if 189 as i32 == 'M' as i32 {
            12 as i32
        } else if 189 as i32 == 'N' as i32 {
            13 as i32
        } else if 189 as i32 == 'O' as i32 {
            14 as i32
        } else if 189 as i32 == 'P' as i32 {
            15 as i32
        } else if 189 as i32 == 'Q' as i32 {
            16 as i32
        } else if 189 as i32 == 'R' as i32 {
            17 as i32
        } else if 189 as i32 == 'S' as i32 {
            18 as i32
        } else if 189 as i32 == 'T' as i32 {
            19 as i32
        } else if 189 as i32 == 'U' as i32 {
            20 as i32
        } else if 189 as i32 == 'V' as i32 {
            21 as i32
        } else if 189 as i32 == 'W' as i32 {
            22 as i32
        } else if 189 as i32 == 'X' as i32 {
            23 as i32
        } else if 189 as i32 == 'Y' as i32 {
            24 as i32
        } else if 189 as i32 == 'Z' as i32 {
            25 as i32
        } else if 189 as i32 == '2' as i32 {
            26 as i32
        } else if 189 as i32 == '3' as i32 {
            27 as i32
        } else if 189 as i32 == '4' as i32 {
            28 as i32
        } else if 189 as i32 == '5' as i32 {
            29 as i32
        } else if 189 as i32 == '6' as i32 {
            30 as i32
        } else if 189 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 190 as i32 == 'A' as i32 {
            0 as i32
        } else if 190 as i32 == 'B' as i32 {
            1 as i32
        } else if 190 as i32 == 'C' as i32 {
            2 as i32
        } else if 190 as i32 == 'D' as i32 {
            3 as i32
        } else if 190 as i32 == 'E' as i32 {
            4 as i32
        } else if 190 as i32 == 'F' as i32 {
            5 as i32
        } else if 190 as i32 == 'G' as i32 {
            6 as i32
        } else if 190 as i32 == 'H' as i32 {
            7 as i32
        } else if 190 as i32 == 'I' as i32 {
            8 as i32
        } else if 190 as i32 == 'J' as i32 {
            9 as i32
        } else if 190 as i32 == 'K' as i32 {
            10 as i32
        } else if 190 as i32 == 'L' as i32 {
            11 as i32
        } else if 190 as i32 == 'M' as i32 {
            12 as i32
        } else if 190 as i32 == 'N' as i32 {
            13 as i32
        } else if 190 as i32 == 'O' as i32 {
            14 as i32
        } else if 190 as i32 == 'P' as i32 {
            15 as i32
        } else if 190 as i32 == 'Q' as i32 {
            16 as i32
        } else if 190 as i32 == 'R' as i32 {
            17 as i32
        } else if 190 as i32 == 'S' as i32 {
            18 as i32
        } else if 190 as i32 == 'T' as i32 {
            19 as i32
        } else if 190 as i32 == 'U' as i32 {
            20 as i32
        } else if 190 as i32 == 'V' as i32 {
            21 as i32
        } else if 190 as i32 == 'W' as i32 {
            22 as i32
        } else if 190 as i32 == 'X' as i32 {
            23 as i32
        } else if 190 as i32 == 'Y' as i32 {
            24 as i32
        } else if 190 as i32 == 'Z' as i32 {
            25 as i32
        } else if 190 as i32 == '2' as i32 {
            26 as i32
        } else if 190 as i32 == '3' as i32 {
            27 as i32
        } else if 190 as i32 == '4' as i32 {
            28 as i32
        } else if 190 as i32 == '5' as i32 {
            29 as i32
        } else if 190 as i32 == '6' as i32 {
            30 as i32
        } else if 190 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 191 as i32 == 'A' as i32 {
            0 as i32
        } else if 191 as i32 == 'B' as i32 {
            1 as i32
        } else if 191 as i32 == 'C' as i32 {
            2 as i32
        } else if 191 as i32 == 'D' as i32 {
            3 as i32
        } else if 191 as i32 == 'E' as i32 {
            4 as i32
        } else if 191 as i32 == 'F' as i32 {
            5 as i32
        } else if 191 as i32 == 'G' as i32 {
            6 as i32
        } else if 191 as i32 == 'H' as i32 {
            7 as i32
        } else if 191 as i32 == 'I' as i32 {
            8 as i32
        } else if 191 as i32 == 'J' as i32 {
            9 as i32
        } else if 191 as i32 == 'K' as i32 {
            10 as i32
        } else if 191 as i32 == 'L' as i32 {
            11 as i32
        } else if 191 as i32 == 'M' as i32 {
            12 as i32
        } else if 191 as i32 == 'N' as i32 {
            13 as i32
        } else if 191 as i32 == 'O' as i32 {
            14 as i32
        } else if 191 as i32 == 'P' as i32 {
            15 as i32
        } else if 191 as i32 == 'Q' as i32 {
            16 as i32
        } else if 191 as i32 == 'R' as i32 {
            17 as i32
        } else if 191 as i32 == 'S' as i32 {
            18 as i32
        } else if 191 as i32 == 'T' as i32 {
            19 as i32
        } else if 191 as i32 == 'U' as i32 {
            20 as i32
        } else if 191 as i32 == 'V' as i32 {
            21 as i32
        } else if 191 as i32 == 'W' as i32 {
            22 as i32
        } else if 191 as i32 == 'X' as i32 {
            23 as i32
        } else if 191 as i32 == 'Y' as i32 {
            24 as i32
        } else if 191 as i32 == 'Z' as i32 {
            25 as i32
        } else if 191 as i32 == '2' as i32 {
            26 as i32
        } else if 191 as i32 == '3' as i32 {
            27 as i32
        } else if 191 as i32 == '4' as i32 {
            28 as i32
        } else if 191 as i32 == '5' as i32 {
            29 as i32
        } else if 191 as i32 == '6' as i32 {
            30 as i32
        } else if 191 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 192 as i32 == 'A' as i32 {
            0 as i32
        } else if 192 as i32 == 'B' as i32 {
            1 as i32
        } else if 192 as i32 == 'C' as i32 {
            2 as i32
        } else if 192 as i32 == 'D' as i32 {
            3 as i32
        } else if 192 as i32 == 'E' as i32 {
            4 as i32
        } else if 192 as i32 == 'F' as i32 {
            5 as i32
        } else if 192 as i32 == 'G' as i32 {
            6 as i32
        } else if 192 as i32 == 'H' as i32 {
            7 as i32
        } else if 192 as i32 == 'I' as i32 {
            8 as i32
        } else if 192 as i32 == 'J' as i32 {
            9 as i32
        } else if 192 as i32 == 'K' as i32 {
            10 as i32
        } else if 192 as i32 == 'L' as i32 {
            11 as i32
        } else if 192 as i32 == 'M' as i32 {
            12 as i32
        } else if 192 as i32 == 'N' as i32 {
            13 as i32
        } else if 192 as i32 == 'O' as i32 {
            14 as i32
        } else if 192 as i32 == 'P' as i32 {
            15 as i32
        } else if 192 as i32 == 'Q' as i32 {
            16 as i32
        } else if 192 as i32 == 'R' as i32 {
            17 as i32
        } else if 192 as i32 == 'S' as i32 {
            18 as i32
        } else if 192 as i32 == 'T' as i32 {
            19 as i32
        } else if 192 as i32 == 'U' as i32 {
            20 as i32
        } else if 192 as i32 == 'V' as i32 {
            21 as i32
        } else if 192 as i32 == 'W' as i32 {
            22 as i32
        } else if 192 as i32 == 'X' as i32 {
            23 as i32
        } else if 192 as i32 == 'Y' as i32 {
            24 as i32
        } else if 192 as i32 == 'Z' as i32 {
            25 as i32
        } else if 192 as i32 == '2' as i32 {
            26 as i32
        } else if 192 as i32 == '3' as i32 {
            27 as i32
        } else if 192 as i32 == '4' as i32 {
            28 as i32
        } else if 192 as i32 == '5' as i32 {
            29 as i32
        } else if 192 as i32 == '6' as i32 {
            30 as i32
        } else if 192 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 193 as i32 == 'A' as i32 {
            0 as i32
        } else if 193 as i32 == 'B' as i32 {
            1 as i32
        } else if 193 as i32 == 'C' as i32 {
            2 as i32
        } else if 193 as i32 == 'D' as i32 {
            3 as i32
        } else if 193 as i32 == 'E' as i32 {
            4 as i32
        } else if 193 as i32 == 'F' as i32 {
            5 as i32
        } else if 193 as i32 == 'G' as i32 {
            6 as i32
        } else if 193 as i32 == 'H' as i32 {
            7 as i32
        } else if 193 as i32 == 'I' as i32 {
            8 as i32
        } else if 193 as i32 == 'J' as i32 {
            9 as i32
        } else if 193 as i32 == 'K' as i32 {
            10 as i32
        } else if 193 as i32 == 'L' as i32 {
            11 as i32
        } else if 193 as i32 == 'M' as i32 {
            12 as i32
        } else if 193 as i32 == 'N' as i32 {
            13 as i32
        } else if 193 as i32 == 'O' as i32 {
            14 as i32
        } else if 193 as i32 == 'P' as i32 {
            15 as i32
        } else if 193 as i32 == 'Q' as i32 {
            16 as i32
        } else if 193 as i32 == 'R' as i32 {
            17 as i32
        } else if 193 as i32 == 'S' as i32 {
            18 as i32
        } else if 193 as i32 == 'T' as i32 {
            19 as i32
        } else if 193 as i32 == 'U' as i32 {
            20 as i32
        } else if 193 as i32 == 'V' as i32 {
            21 as i32
        } else if 193 as i32 == 'W' as i32 {
            22 as i32
        } else if 193 as i32 == 'X' as i32 {
            23 as i32
        } else if 193 as i32 == 'Y' as i32 {
            24 as i32
        } else if 193 as i32 == 'Z' as i32 {
            25 as i32
        } else if 193 as i32 == '2' as i32 {
            26 as i32
        } else if 193 as i32 == '3' as i32 {
            27 as i32
        } else if 193 as i32 == '4' as i32 {
            28 as i32
        } else if 193 as i32 == '5' as i32 {
            29 as i32
        } else if 193 as i32 == '6' as i32 {
            30 as i32
        } else if 193 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 194 as i32 == 'A' as i32 {
            0 as i32
        } else if 194 as i32 == 'B' as i32 {
            1 as i32
        } else if 194 as i32 == 'C' as i32 {
            2 as i32
        } else if 194 as i32 == 'D' as i32 {
            3 as i32
        } else if 194 as i32 == 'E' as i32 {
            4 as i32
        } else if 194 as i32 == 'F' as i32 {
            5 as i32
        } else if 194 as i32 == 'G' as i32 {
            6 as i32
        } else if 194 as i32 == 'H' as i32 {
            7 as i32
        } else if 194 as i32 == 'I' as i32 {
            8 as i32
        } else if 194 as i32 == 'J' as i32 {
            9 as i32
        } else if 194 as i32 == 'K' as i32 {
            10 as i32
        } else if 194 as i32 == 'L' as i32 {
            11 as i32
        } else if 194 as i32 == 'M' as i32 {
            12 as i32
        } else if 194 as i32 == 'N' as i32 {
            13 as i32
        } else if 194 as i32 == 'O' as i32 {
            14 as i32
        } else if 194 as i32 == 'P' as i32 {
            15 as i32
        } else if 194 as i32 == 'Q' as i32 {
            16 as i32
        } else if 194 as i32 == 'R' as i32 {
            17 as i32
        } else if 194 as i32 == 'S' as i32 {
            18 as i32
        } else if 194 as i32 == 'T' as i32 {
            19 as i32
        } else if 194 as i32 == 'U' as i32 {
            20 as i32
        } else if 194 as i32 == 'V' as i32 {
            21 as i32
        } else if 194 as i32 == 'W' as i32 {
            22 as i32
        } else if 194 as i32 == 'X' as i32 {
            23 as i32
        } else if 194 as i32 == 'Y' as i32 {
            24 as i32
        } else if 194 as i32 == 'Z' as i32 {
            25 as i32
        } else if 194 as i32 == '2' as i32 {
            26 as i32
        } else if 194 as i32 == '3' as i32 {
            27 as i32
        } else if 194 as i32 == '4' as i32 {
            28 as i32
        } else if 194 as i32 == '5' as i32 {
            29 as i32
        } else if 194 as i32 == '6' as i32 {
            30 as i32
        } else if 194 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 195 as i32 == 'A' as i32 {
            0 as i32
        } else if 195 as i32 == 'B' as i32 {
            1 as i32
        } else if 195 as i32 == 'C' as i32 {
            2 as i32
        } else if 195 as i32 == 'D' as i32 {
            3 as i32
        } else if 195 as i32 == 'E' as i32 {
            4 as i32
        } else if 195 as i32 == 'F' as i32 {
            5 as i32
        } else if 195 as i32 == 'G' as i32 {
            6 as i32
        } else if 195 as i32 == 'H' as i32 {
            7 as i32
        } else if 195 as i32 == 'I' as i32 {
            8 as i32
        } else if 195 as i32 == 'J' as i32 {
            9 as i32
        } else if 195 as i32 == 'K' as i32 {
            10 as i32
        } else if 195 as i32 == 'L' as i32 {
            11 as i32
        } else if 195 as i32 == 'M' as i32 {
            12 as i32
        } else if 195 as i32 == 'N' as i32 {
            13 as i32
        } else if 195 as i32 == 'O' as i32 {
            14 as i32
        } else if 195 as i32 == 'P' as i32 {
            15 as i32
        } else if 195 as i32 == 'Q' as i32 {
            16 as i32
        } else if 195 as i32 == 'R' as i32 {
            17 as i32
        } else if 195 as i32 == 'S' as i32 {
            18 as i32
        } else if 195 as i32 == 'T' as i32 {
            19 as i32
        } else if 195 as i32 == 'U' as i32 {
            20 as i32
        } else if 195 as i32 == 'V' as i32 {
            21 as i32
        } else if 195 as i32 == 'W' as i32 {
            22 as i32
        } else if 195 as i32 == 'X' as i32 {
            23 as i32
        } else if 195 as i32 == 'Y' as i32 {
            24 as i32
        } else if 195 as i32 == 'Z' as i32 {
            25 as i32
        } else if 195 as i32 == '2' as i32 {
            26 as i32
        } else if 195 as i32 == '3' as i32 {
            27 as i32
        } else if 195 as i32 == '4' as i32 {
            28 as i32
        } else if 195 as i32 == '5' as i32 {
            29 as i32
        } else if 195 as i32 == '6' as i32 {
            30 as i32
        } else if 195 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 196 as i32 == 'A' as i32 {
            0 as i32
        } else if 196 as i32 == 'B' as i32 {
            1 as i32
        } else if 196 as i32 == 'C' as i32 {
            2 as i32
        } else if 196 as i32 == 'D' as i32 {
            3 as i32
        } else if 196 as i32 == 'E' as i32 {
            4 as i32
        } else if 196 as i32 == 'F' as i32 {
            5 as i32
        } else if 196 as i32 == 'G' as i32 {
            6 as i32
        } else if 196 as i32 == 'H' as i32 {
            7 as i32
        } else if 196 as i32 == 'I' as i32 {
            8 as i32
        } else if 196 as i32 == 'J' as i32 {
            9 as i32
        } else if 196 as i32 == 'K' as i32 {
            10 as i32
        } else if 196 as i32 == 'L' as i32 {
            11 as i32
        } else if 196 as i32 == 'M' as i32 {
            12 as i32
        } else if 196 as i32 == 'N' as i32 {
            13 as i32
        } else if 196 as i32 == 'O' as i32 {
            14 as i32
        } else if 196 as i32 == 'P' as i32 {
            15 as i32
        } else if 196 as i32 == 'Q' as i32 {
            16 as i32
        } else if 196 as i32 == 'R' as i32 {
            17 as i32
        } else if 196 as i32 == 'S' as i32 {
            18 as i32
        } else if 196 as i32 == 'T' as i32 {
            19 as i32
        } else if 196 as i32 == 'U' as i32 {
            20 as i32
        } else if 196 as i32 == 'V' as i32 {
            21 as i32
        } else if 196 as i32 == 'W' as i32 {
            22 as i32
        } else if 196 as i32 == 'X' as i32 {
            23 as i32
        } else if 196 as i32 == 'Y' as i32 {
            24 as i32
        } else if 196 as i32 == 'Z' as i32 {
            25 as i32
        } else if 196 as i32 == '2' as i32 {
            26 as i32
        } else if 196 as i32 == '3' as i32 {
            27 as i32
        } else if 196 as i32 == '4' as i32 {
            28 as i32
        } else if 196 as i32 == '5' as i32 {
            29 as i32
        } else if 196 as i32 == '6' as i32 {
            30 as i32
        } else if 196 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 197 as i32 == 'A' as i32 {
            0 as i32
        } else if 197 as i32 == 'B' as i32 {
            1 as i32
        } else if 197 as i32 == 'C' as i32 {
            2 as i32
        } else if 197 as i32 == 'D' as i32 {
            3 as i32
        } else if 197 as i32 == 'E' as i32 {
            4 as i32
        } else if 197 as i32 == 'F' as i32 {
            5 as i32
        } else if 197 as i32 == 'G' as i32 {
            6 as i32
        } else if 197 as i32 == 'H' as i32 {
            7 as i32
        } else if 197 as i32 == 'I' as i32 {
            8 as i32
        } else if 197 as i32 == 'J' as i32 {
            9 as i32
        } else if 197 as i32 == 'K' as i32 {
            10 as i32
        } else if 197 as i32 == 'L' as i32 {
            11 as i32
        } else if 197 as i32 == 'M' as i32 {
            12 as i32
        } else if 197 as i32 == 'N' as i32 {
            13 as i32
        } else if 197 as i32 == 'O' as i32 {
            14 as i32
        } else if 197 as i32 == 'P' as i32 {
            15 as i32
        } else if 197 as i32 == 'Q' as i32 {
            16 as i32
        } else if 197 as i32 == 'R' as i32 {
            17 as i32
        } else if 197 as i32 == 'S' as i32 {
            18 as i32
        } else if 197 as i32 == 'T' as i32 {
            19 as i32
        } else if 197 as i32 == 'U' as i32 {
            20 as i32
        } else if 197 as i32 == 'V' as i32 {
            21 as i32
        } else if 197 as i32 == 'W' as i32 {
            22 as i32
        } else if 197 as i32 == 'X' as i32 {
            23 as i32
        } else if 197 as i32 == 'Y' as i32 {
            24 as i32
        } else if 197 as i32 == 'Z' as i32 {
            25 as i32
        } else if 197 as i32 == '2' as i32 {
            26 as i32
        } else if 197 as i32 == '3' as i32 {
            27 as i32
        } else if 197 as i32 == '4' as i32 {
            28 as i32
        } else if 197 as i32 == '5' as i32 {
            29 as i32
        } else if 197 as i32 == '6' as i32 {
            30 as i32
        } else if 197 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 198 as i32 == 'A' as i32 {
            0 as i32
        } else if 198 as i32 == 'B' as i32 {
            1 as i32
        } else if 198 as i32 == 'C' as i32 {
            2 as i32
        } else if 198 as i32 == 'D' as i32 {
            3 as i32
        } else if 198 as i32 == 'E' as i32 {
            4 as i32
        } else if 198 as i32 == 'F' as i32 {
            5 as i32
        } else if 198 as i32 == 'G' as i32 {
            6 as i32
        } else if 198 as i32 == 'H' as i32 {
            7 as i32
        } else if 198 as i32 == 'I' as i32 {
            8 as i32
        } else if 198 as i32 == 'J' as i32 {
            9 as i32
        } else if 198 as i32 == 'K' as i32 {
            10 as i32
        } else if 198 as i32 == 'L' as i32 {
            11 as i32
        } else if 198 as i32 == 'M' as i32 {
            12 as i32
        } else if 198 as i32 == 'N' as i32 {
            13 as i32
        } else if 198 as i32 == 'O' as i32 {
            14 as i32
        } else if 198 as i32 == 'P' as i32 {
            15 as i32
        } else if 198 as i32 == 'Q' as i32 {
            16 as i32
        } else if 198 as i32 == 'R' as i32 {
            17 as i32
        } else if 198 as i32 == 'S' as i32 {
            18 as i32
        } else if 198 as i32 == 'T' as i32 {
            19 as i32
        } else if 198 as i32 == 'U' as i32 {
            20 as i32
        } else if 198 as i32 == 'V' as i32 {
            21 as i32
        } else if 198 as i32 == 'W' as i32 {
            22 as i32
        } else if 198 as i32 == 'X' as i32 {
            23 as i32
        } else if 198 as i32 == 'Y' as i32 {
            24 as i32
        } else if 198 as i32 == 'Z' as i32 {
            25 as i32
        } else if 198 as i32 == '2' as i32 {
            26 as i32
        } else if 198 as i32 == '3' as i32 {
            27 as i32
        } else if 198 as i32 == '4' as i32 {
            28 as i32
        } else if 198 as i32 == '5' as i32 {
            29 as i32
        } else if 198 as i32 == '6' as i32 {
            30 as i32
        } else if 198 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 199 as i32 == 'A' as i32 {
            0 as i32
        } else if 199 as i32 == 'B' as i32 {
            1 as i32
        } else if 199 as i32 == 'C' as i32 {
            2 as i32
        } else if 199 as i32 == 'D' as i32 {
            3 as i32
        } else if 199 as i32 == 'E' as i32 {
            4 as i32
        } else if 199 as i32 == 'F' as i32 {
            5 as i32
        } else if 199 as i32 == 'G' as i32 {
            6 as i32
        } else if 199 as i32 == 'H' as i32 {
            7 as i32
        } else if 199 as i32 == 'I' as i32 {
            8 as i32
        } else if 199 as i32 == 'J' as i32 {
            9 as i32
        } else if 199 as i32 == 'K' as i32 {
            10 as i32
        } else if 199 as i32 == 'L' as i32 {
            11 as i32
        } else if 199 as i32 == 'M' as i32 {
            12 as i32
        } else if 199 as i32 == 'N' as i32 {
            13 as i32
        } else if 199 as i32 == 'O' as i32 {
            14 as i32
        } else if 199 as i32 == 'P' as i32 {
            15 as i32
        } else if 199 as i32 == 'Q' as i32 {
            16 as i32
        } else if 199 as i32 == 'R' as i32 {
            17 as i32
        } else if 199 as i32 == 'S' as i32 {
            18 as i32
        } else if 199 as i32 == 'T' as i32 {
            19 as i32
        } else if 199 as i32 == 'U' as i32 {
            20 as i32
        } else if 199 as i32 == 'V' as i32 {
            21 as i32
        } else if 199 as i32 == 'W' as i32 {
            22 as i32
        } else if 199 as i32 == 'X' as i32 {
            23 as i32
        } else if 199 as i32 == 'Y' as i32 {
            24 as i32
        } else if 199 as i32 == 'Z' as i32 {
            25 as i32
        } else if 199 as i32 == '2' as i32 {
            26 as i32
        } else if 199 as i32 == '3' as i32 {
            27 as i32
        } else if 199 as i32 == '4' as i32 {
            28 as i32
        } else if 199 as i32 == '5' as i32 {
            29 as i32
        } else if 199 as i32 == '6' as i32 {
            30 as i32
        } else if 199 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 200 as i32 == 'A' as i32 {
            0 as i32
        } else if 200 as i32 == 'B' as i32 {
            1 as i32
        } else if 200 as i32 == 'C' as i32 {
            2 as i32
        } else if 200 as i32 == 'D' as i32 {
            3 as i32
        } else if 200 as i32 == 'E' as i32 {
            4 as i32
        } else if 200 as i32 == 'F' as i32 {
            5 as i32
        } else if 200 as i32 == 'G' as i32 {
            6 as i32
        } else if 200 as i32 == 'H' as i32 {
            7 as i32
        } else if 200 as i32 == 'I' as i32 {
            8 as i32
        } else if 200 as i32 == 'J' as i32 {
            9 as i32
        } else if 200 as i32 == 'K' as i32 {
            10 as i32
        } else if 200 as i32 == 'L' as i32 {
            11 as i32
        } else if 200 as i32 == 'M' as i32 {
            12 as i32
        } else if 200 as i32 == 'N' as i32 {
            13 as i32
        } else if 200 as i32 == 'O' as i32 {
            14 as i32
        } else if 200 as i32 == 'P' as i32 {
            15 as i32
        } else if 200 as i32 == 'Q' as i32 {
            16 as i32
        } else if 200 as i32 == 'R' as i32 {
            17 as i32
        } else if 200 as i32 == 'S' as i32 {
            18 as i32
        } else if 200 as i32 == 'T' as i32 {
            19 as i32
        } else if 200 as i32 == 'U' as i32 {
            20 as i32
        } else if 200 as i32 == 'V' as i32 {
            21 as i32
        } else if 200 as i32 == 'W' as i32 {
            22 as i32
        } else if 200 as i32 == 'X' as i32 {
            23 as i32
        } else if 200 as i32 == 'Y' as i32 {
            24 as i32
        } else if 200 as i32 == 'Z' as i32 {
            25 as i32
        } else if 200 as i32 == '2' as i32 {
            26 as i32
        } else if 200 as i32 == '3' as i32 {
            27 as i32
        } else if 200 as i32 == '4' as i32 {
            28 as i32
        } else if 200 as i32 == '5' as i32 {
            29 as i32
        } else if 200 as i32 == '6' as i32 {
            30 as i32
        } else if 200 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 201 as i32 == 'A' as i32 {
            0 as i32
        } else if 201 as i32 == 'B' as i32 {
            1 as i32
        } else if 201 as i32 == 'C' as i32 {
            2 as i32
        } else if 201 as i32 == 'D' as i32 {
            3 as i32
        } else if 201 as i32 == 'E' as i32 {
            4 as i32
        } else if 201 as i32 == 'F' as i32 {
            5 as i32
        } else if 201 as i32 == 'G' as i32 {
            6 as i32
        } else if 201 as i32 == 'H' as i32 {
            7 as i32
        } else if 201 as i32 == 'I' as i32 {
            8 as i32
        } else if 201 as i32 == 'J' as i32 {
            9 as i32
        } else if 201 as i32 == 'K' as i32 {
            10 as i32
        } else if 201 as i32 == 'L' as i32 {
            11 as i32
        } else if 201 as i32 == 'M' as i32 {
            12 as i32
        } else if 201 as i32 == 'N' as i32 {
            13 as i32
        } else if 201 as i32 == 'O' as i32 {
            14 as i32
        } else if 201 as i32 == 'P' as i32 {
            15 as i32
        } else if 201 as i32 == 'Q' as i32 {
            16 as i32
        } else if 201 as i32 == 'R' as i32 {
            17 as i32
        } else if 201 as i32 == 'S' as i32 {
            18 as i32
        } else if 201 as i32 == 'T' as i32 {
            19 as i32
        } else if 201 as i32 == 'U' as i32 {
            20 as i32
        } else if 201 as i32 == 'V' as i32 {
            21 as i32
        } else if 201 as i32 == 'W' as i32 {
            22 as i32
        } else if 201 as i32 == 'X' as i32 {
            23 as i32
        } else if 201 as i32 == 'Y' as i32 {
            24 as i32
        } else if 201 as i32 == 'Z' as i32 {
            25 as i32
        } else if 201 as i32 == '2' as i32 {
            26 as i32
        } else if 201 as i32 == '3' as i32 {
            27 as i32
        } else if 201 as i32 == '4' as i32 {
            28 as i32
        } else if 201 as i32 == '5' as i32 {
            29 as i32
        } else if 201 as i32 == '6' as i32 {
            30 as i32
        } else if 201 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 202 as i32 == 'A' as i32 {
            0 as i32
        } else if 202 as i32 == 'B' as i32 {
            1 as i32
        } else if 202 as i32 == 'C' as i32 {
            2 as i32
        } else if 202 as i32 == 'D' as i32 {
            3 as i32
        } else if 202 as i32 == 'E' as i32 {
            4 as i32
        } else if 202 as i32 == 'F' as i32 {
            5 as i32
        } else if 202 as i32 == 'G' as i32 {
            6 as i32
        } else if 202 as i32 == 'H' as i32 {
            7 as i32
        } else if 202 as i32 == 'I' as i32 {
            8 as i32
        } else if 202 as i32 == 'J' as i32 {
            9 as i32
        } else if 202 as i32 == 'K' as i32 {
            10 as i32
        } else if 202 as i32 == 'L' as i32 {
            11 as i32
        } else if 202 as i32 == 'M' as i32 {
            12 as i32
        } else if 202 as i32 == 'N' as i32 {
            13 as i32
        } else if 202 as i32 == 'O' as i32 {
            14 as i32
        } else if 202 as i32 == 'P' as i32 {
            15 as i32
        } else if 202 as i32 == 'Q' as i32 {
            16 as i32
        } else if 202 as i32 == 'R' as i32 {
            17 as i32
        } else if 202 as i32 == 'S' as i32 {
            18 as i32
        } else if 202 as i32 == 'T' as i32 {
            19 as i32
        } else if 202 as i32 == 'U' as i32 {
            20 as i32
        } else if 202 as i32 == 'V' as i32 {
            21 as i32
        } else if 202 as i32 == 'W' as i32 {
            22 as i32
        } else if 202 as i32 == 'X' as i32 {
            23 as i32
        } else if 202 as i32 == 'Y' as i32 {
            24 as i32
        } else if 202 as i32 == 'Z' as i32 {
            25 as i32
        } else if 202 as i32 == '2' as i32 {
            26 as i32
        } else if 202 as i32 == '3' as i32 {
            27 as i32
        } else if 202 as i32 == '4' as i32 {
            28 as i32
        } else if 202 as i32 == '5' as i32 {
            29 as i32
        } else if 202 as i32 == '6' as i32 {
            30 as i32
        } else if 202 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 203 as i32 == 'A' as i32 {
            0 as i32
        } else if 203 as i32 == 'B' as i32 {
            1 as i32
        } else if 203 as i32 == 'C' as i32 {
            2 as i32
        } else if 203 as i32 == 'D' as i32 {
            3 as i32
        } else if 203 as i32 == 'E' as i32 {
            4 as i32
        } else if 203 as i32 == 'F' as i32 {
            5 as i32
        } else if 203 as i32 == 'G' as i32 {
            6 as i32
        } else if 203 as i32 == 'H' as i32 {
            7 as i32
        } else if 203 as i32 == 'I' as i32 {
            8 as i32
        } else if 203 as i32 == 'J' as i32 {
            9 as i32
        } else if 203 as i32 == 'K' as i32 {
            10 as i32
        } else if 203 as i32 == 'L' as i32 {
            11 as i32
        } else if 203 as i32 == 'M' as i32 {
            12 as i32
        } else if 203 as i32 == 'N' as i32 {
            13 as i32
        } else if 203 as i32 == 'O' as i32 {
            14 as i32
        } else if 203 as i32 == 'P' as i32 {
            15 as i32
        } else if 203 as i32 == 'Q' as i32 {
            16 as i32
        } else if 203 as i32 == 'R' as i32 {
            17 as i32
        } else if 203 as i32 == 'S' as i32 {
            18 as i32
        } else if 203 as i32 == 'T' as i32 {
            19 as i32
        } else if 203 as i32 == 'U' as i32 {
            20 as i32
        } else if 203 as i32 == 'V' as i32 {
            21 as i32
        } else if 203 as i32 == 'W' as i32 {
            22 as i32
        } else if 203 as i32 == 'X' as i32 {
            23 as i32
        } else if 203 as i32 == 'Y' as i32 {
            24 as i32
        } else if 203 as i32 == 'Z' as i32 {
            25 as i32
        } else if 203 as i32 == '2' as i32 {
            26 as i32
        } else if 203 as i32 == '3' as i32 {
            27 as i32
        } else if 203 as i32 == '4' as i32 {
            28 as i32
        } else if 203 as i32 == '5' as i32 {
            29 as i32
        } else if 203 as i32 == '6' as i32 {
            30 as i32
        } else if 203 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 204 as i32 == 'A' as i32 {
            0 as i32
        } else if 204 as i32 == 'B' as i32 {
            1 as i32
        } else if 204 as i32 == 'C' as i32 {
            2 as i32
        } else if 204 as i32 == 'D' as i32 {
            3 as i32
        } else if 204 as i32 == 'E' as i32 {
            4 as i32
        } else if 204 as i32 == 'F' as i32 {
            5 as i32
        } else if 204 as i32 == 'G' as i32 {
            6 as i32
        } else if 204 as i32 == 'H' as i32 {
            7 as i32
        } else if 204 as i32 == 'I' as i32 {
            8 as i32
        } else if 204 as i32 == 'J' as i32 {
            9 as i32
        } else if 204 as i32 == 'K' as i32 {
            10 as i32
        } else if 204 as i32 == 'L' as i32 {
            11 as i32
        } else if 204 as i32 == 'M' as i32 {
            12 as i32
        } else if 204 as i32 == 'N' as i32 {
            13 as i32
        } else if 204 as i32 == 'O' as i32 {
            14 as i32
        } else if 204 as i32 == 'P' as i32 {
            15 as i32
        } else if 204 as i32 == 'Q' as i32 {
            16 as i32
        } else if 204 as i32 == 'R' as i32 {
            17 as i32
        } else if 204 as i32 == 'S' as i32 {
            18 as i32
        } else if 204 as i32 == 'T' as i32 {
            19 as i32
        } else if 204 as i32 == 'U' as i32 {
            20 as i32
        } else if 204 as i32 == 'V' as i32 {
            21 as i32
        } else if 204 as i32 == 'W' as i32 {
            22 as i32
        } else if 204 as i32 == 'X' as i32 {
            23 as i32
        } else if 204 as i32 == 'Y' as i32 {
            24 as i32
        } else if 204 as i32 == 'Z' as i32 {
            25 as i32
        } else if 204 as i32 == '2' as i32 {
            26 as i32
        } else if 204 as i32 == '3' as i32 {
            27 as i32
        } else if 204 as i32 == '4' as i32 {
            28 as i32
        } else if 204 as i32 == '5' as i32 {
            29 as i32
        } else if 204 as i32 == '6' as i32 {
            30 as i32
        } else if 204 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 205 as i32 == 'A' as i32 {
            0 as i32
        } else if 205 as i32 == 'B' as i32 {
            1 as i32
        } else if 205 as i32 == 'C' as i32 {
            2 as i32
        } else if 205 as i32 == 'D' as i32 {
            3 as i32
        } else if 205 as i32 == 'E' as i32 {
            4 as i32
        } else if 205 as i32 == 'F' as i32 {
            5 as i32
        } else if 205 as i32 == 'G' as i32 {
            6 as i32
        } else if 205 as i32 == 'H' as i32 {
            7 as i32
        } else if 205 as i32 == 'I' as i32 {
            8 as i32
        } else if 205 as i32 == 'J' as i32 {
            9 as i32
        } else if 205 as i32 == 'K' as i32 {
            10 as i32
        } else if 205 as i32 == 'L' as i32 {
            11 as i32
        } else if 205 as i32 == 'M' as i32 {
            12 as i32
        } else if 205 as i32 == 'N' as i32 {
            13 as i32
        } else if 205 as i32 == 'O' as i32 {
            14 as i32
        } else if 205 as i32 == 'P' as i32 {
            15 as i32
        } else if 205 as i32 == 'Q' as i32 {
            16 as i32
        } else if 205 as i32 == 'R' as i32 {
            17 as i32
        } else if 205 as i32 == 'S' as i32 {
            18 as i32
        } else if 205 as i32 == 'T' as i32 {
            19 as i32
        } else if 205 as i32 == 'U' as i32 {
            20 as i32
        } else if 205 as i32 == 'V' as i32 {
            21 as i32
        } else if 205 as i32 == 'W' as i32 {
            22 as i32
        } else if 205 as i32 == 'X' as i32 {
            23 as i32
        } else if 205 as i32 == 'Y' as i32 {
            24 as i32
        } else if 205 as i32 == 'Z' as i32 {
            25 as i32
        } else if 205 as i32 == '2' as i32 {
            26 as i32
        } else if 205 as i32 == '3' as i32 {
            27 as i32
        } else if 205 as i32 == '4' as i32 {
            28 as i32
        } else if 205 as i32 == '5' as i32 {
            29 as i32
        } else if 205 as i32 == '6' as i32 {
            30 as i32
        } else if 205 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 206 as i32 == 'A' as i32 {
            0 as i32
        } else if 206 as i32 == 'B' as i32 {
            1 as i32
        } else if 206 as i32 == 'C' as i32 {
            2 as i32
        } else if 206 as i32 == 'D' as i32 {
            3 as i32
        } else if 206 as i32 == 'E' as i32 {
            4 as i32
        } else if 206 as i32 == 'F' as i32 {
            5 as i32
        } else if 206 as i32 == 'G' as i32 {
            6 as i32
        } else if 206 as i32 == 'H' as i32 {
            7 as i32
        } else if 206 as i32 == 'I' as i32 {
            8 as i32
        } else if 206 as i32 == 'J' as i32 {
            9 as i32
        } else if 206 as i32 == 'K' as i32 {
            10 as i32
        } else if 206 as i32 == 'L' as i32 {
            11 as i32
        } else if 206 as i32 == 'M' as i32 {
            12 as i32
        } else if 206 as i32 == 'N' as i32 {
            13 as i32
        } else if 206 as i32 == 'O' as i32 {
            14 as i32
        } else if 206 as i32 == 'P' as i32 {
            15 as i32
        } else if 206 as i32 == 'Q' as i32 {
            16 as i32
        } else if 206 as i32 == 'R' as i32 {
            17 as i32
        } else if 206 as i32 == 'S' as i32 {
            18 as i32
        } else if 206 as i32 == 'T' as i32 {
            19 as i32
        } else if 206 as i32 == 'U' as i32 {
            20 as i32
        } else if 206 as i32 == 'V' as i32 {
            21 as i32
        } else if 206 as i32 == 'W' as i32 {
            22 as i32
        } else if 206 as i32 == 'X' as i32 {
            23 as i32
        } else if 206 as i32 == 'Y' as i32 {
            24 as i32
        } else if 206 as i32 == 'Z' as i32 {
            25 as i32
        } else if 206 as i32 == '2' as i32 {
            26 as i32
        } else if 206 as i32 == '3' as i32 {
            27 as i32
        } else if 206 as i32 == '4' as i32 {
            28 as i32
        } else if 206 as i32 == '5' as i32 {
            29 as i32
        } else if 206 as i32 == '6' as i32 {
            30 as i32
        } else if 206 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 207 as i32 == 'A' as i32 {
            0 as i32
        } else if 207 as i32 == 'B' as i32 {
            1 as i32
        } else if 207 as i32 == 'C' as i32 {
            2 as i32
        } else if 207 as i32 == 'D' as i32 {
            3 as i32
        } else if 207 as i32 == 'E' as i32 {
            4 as i32
        } else if 207 as i32 == 'F' as i32 {
            5 as i32
        } else if 207 as i32 == 'G' as i32 {
            6 as i32
        } else if 207 as i32 == 'H' as i32 {
            7 as i32
        } else if 207 as i32 == 'I' as i32 {
            8 as i32
        } else if 207 as i32 == 'J' as i32 {
            9 as i32
        } else if 207 as i32 == 'K' as i32 {
            10 as i32
        } else if 207 as i32 == 'L' as i32 {
            11 as i32
        } else if 207 as i32 == 'M' as i32 {
            12 as i32
        } else if 207 as i32 == 'N' as i32 {
            13 as i32
        } else if 207 as i32 == 'O' as i32 {
            14 as i32
        } else if 207 as i32 == 'P' as i32 {
            15 as i32
        } else if 207 as i32 == 'Q' as i32 {
            16 as i32
        } else if 207 as i32 == 'R' as i32 {
            17 as i32
        } else if 207 as i32 == 'S' as i32 {
            18 as i32
        } else if 207 as i32 == 'T' as i32 {
            19 as i32
        } else if 207 as i32 == 'U' as i32 {
            20 as i32
        } else if 207 as i32 == 'V' as i32 {
            21 as i32
        } else if 207 as i32 == 'W' as i32 {
            22 as i32
        } else if 207 as i32 == 'X' as i32 {
            23 as i32
        } else if 207 as i32 == 'Y' as i32 {
            24 as i32
        } else if 207 as i32 == 'Z' as i32 {
            25 as i32
        } else if 207 as i32 == '2' as i32 {
            26 as i32
        } else if 207 as i32 == '3' as i32 {
            27 as i32
        } else if 207 as i32 == '4' as i32 {
            28 as i32
        } else if 207 as i32 == '5' as i32 {
            29 as i32
        } else if 207 as i32 == '6' as i32 {
            30 as i32
        } else if 207 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 208 as i32 == 'A' as i32 {
            0 as i32
        } else if 208 as i32 == 'B' as i32 {
            1 as i32
        } else if 208 as i32 == 'C' as i32 {
            2 as i32
        } else if 208 as i32 == 'D' as i32 {
            3 as i32
        } else if 208 as i32 == 'E' as i32 {
            4 as i32
        } else if 208 as i32 == 'F' as i32 {
            5 as i32
        } else if 208 as i32 == 'G' as i32 {
            6 as i32
        } else if 208 as i32 == 'H' as i32 {
            7 as i32
        } else if 208 as i32 == 'I' as i32 {
            8 as i32
        } else if 208 as i32 == 'J' as i32 {
            9 as i32
        } else if 208 as i32 == 'K' as i32 {
            10 as i32
        } else if 208 as i32 == 'L' as i32 {
            11 as i32
        } else if 208 as i32 == 'M' as i32 {
            12 as i32
        } else if 208 as i32 == 'N' as i32 {
            13 as i32
        } else if 208 as i32 == 'O' as i32 {
            14 as i32
        } else if 208 as i32 == 'P' as i32 {
            15 as i32
        } else if 208 as i32 == 'Q' as i32 {
            16 as i32
        } else if 208 as i32 == 'R' as i32 {
            17 as i32
        } else if 208 as i32 == 'S' as i32 {
            18 as i32
        } else if 208 as i32 == 'T' as i32 {
            19 as i32
        } else if 208 as i32 == 'U' as i32 {
            20 as i32
        } else if 208 as i32 == 'V' as i32 {
            21 as i32
        } else if 208 as i32 == 'W' as i32 {
            22 as i32
        } else if 208 as i32 == 'X' as i32 {
            23 as i32
        } else if 208 as i32 == 'Y' as i32 {
            24 as i32
        } else if 208 as i32 == 'Z' as i32 {
            25 as i32
        } else if 208 as i32 == '2' as i32 {
            26 as i32
        } else if 208 as i32 == '3' as i32 {
            27 as i32
        } else if 208 as i32 == '4' as i32 {
            28 as i32
        } else if 208 as i32 == '5' as i32 {
            29 as i32
        } else if 208 as i32 == '6' as i32 {
            30 as i32
        } else if 208 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 209 as i32 == 'A' as i32 {
            0 as i32
        } else if 209 as i32 == 'B' as i32 {
            1 as i32
        } else if 209 as i32 == 'C' as i32 {
            2 as i32
        } else if 209 as i32 == 'D' as i32 {
            3 as i32
        } else if 209 as i32 == 'E' as i32 {
            4 as i32
        } else if 209 as i32 == 'F' as i32 {
            5 as i32
        } else if 209 as i32 == 'G' as i32 {
            6 as i32
        } else if 209 as i32 == 'H' as i32 {
            7 as i32
        } else if 209 as i32 == 'I' as i32 {
            8 as i32
        } else if 209 as i32 == 'J' as i32 {
            9 as i32
        } else if 209 as i32 == 'K' as i32 {
            10 as i32
        } else if 209 as i32 == 'L' as i32 {
            11 as i32
        } else if 209 as i32 == 'M' as i32 {
            12 as i32
        } else if 209 as i32 == 'N' as i32 {
            13 as i32
        } else if 209 as i32 == 'O' as i32 {
            14 as i32
        } else if 209 as i32 == 'P' as i32 {
            15 as i32
        } else if 209 as i32 == 'Q' as i32 {
            16 as i32
        } else if 209 as i32 == 'R' as i32 {
            17 as i32
        } else if 209 as i32 == 'S' as i32 {
            18 as i32
        } else if 209 as i32 == 'T' as i32 {
            19 as i32
        } else if 209 as i32 == 'U' as i32 {
            20 as i32
        } else if 209 as i32 == 'V' as i32 {
            21 as i32
        } else if 209 as i32 == 'W' as i32 {
            22 as i32
        } else if 209 as i32 == 'X' as i32 {
            23 as i32
        } else if 209 as i32 == 'Y' as i32 {
            24 as i32
        } else if 209 as i32 == 'Z' as i32 {
            25 as i32
        } else if 209 as i32 == '2' as i32 {
            26 as i32
        } else if 209 as i32 == '3' as i32 {
            27 as i32
        } else if 209 as i32 == '4' as i32 {
            28 as i32
        } else if 209 as i32 == '5' as i32 {
            29 as i32
        } else if 209 as i32 == '6' as i32 {
            30 as i32
        } else if 209 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 210 as i32 == 'A' as i32 {
            0 as i32
        } else if 210 as i32 == 'B' as i32 {
            1 as i32
        } else if 210 as i32 == 'C' as i32 {
            2 as i32
        } else if 210 as i32 == 'D' as i32 {
            3 as i32
        } else if 210 as i32 == 'E' as i32 {
            4 as i32
        } else if 210 as i32 == 'F' as i32 {
            5 as i32
        } else if 210 as i32 == 'G' as i32 {
            6 as i32
        } else if 210 as i32 == 'H' as i32 {
            7 as i32
        } else if 210 as i32 == 'I' as i32 {
            8 as i32
        } else if 210 as i32 == 'J' as i32 {
            9 as i32
        } else if 210 as i32 == 'K' as i32 {
            10 as i32
        } else if 210 as i32 == 'L' as i32 {
            11 as i32
        } else if 210 as i32 == 'M' as i32 {
            12 as i32
        } else if 210 as i32 == 'N' as i32 {
            13 as i32
        } else if 210 as i32 == 'O' as i32 {
            14 as i32
        } else if 210 as i32 == 'P' as i32 {
            15 as i32
        } else if 210 as i32 == 'Q' as i32 {
            16 as i32
        } else if 210 as i32 == 'R' as i32 {
            17 as i32
        } else if 210 as i32 == 'S' as i32 {
            18 as i32
        } else if 210 as i32 == 'T' as i32 {
            19 as i32
        } else if 210 as i32 == 'U' as i32 {
            20 as i32
        } else if 210 as i32 == 'V' as i32 {
            21 as i32
        } else if 210 as i32 == 'W' as i32 {
            22 as i32
        } else if 210 as i32 == 'X' as i32 {
            23 as i32
        } else if 210 as i32 == 'Y' as i32 {
            24 as i32
        } else if 210 as i32 == 'Z' as i32 {
            25 as i32
        } else if 210 as i32 == '2' as i32 {
            26 as i32
        } else if 210 as i32 == '3' as i32 {
            27 as i32
        } else if 210 as i32 == '4' as i32 {
            28 as i32
        } else if 210 as i32 == '5' as i32 {
            29 as i32
        } else if 210 as i32 == '6' as i32 {
            30 as i32
        } else if 210 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 211 as i32 == 'A' as i32 {
            0 as i32
        } else if 211 as i32 == 'B' as i32 {
            1 as i32
        } else if 211 as i32 == 'C' as i32 {
            2 as i32
        } else if 211 as i32 == 'D' as i32 {
            3 as i32
        } else if 211 as i32 == 'E' as i32 {
            4 as i32
        } else if 211 as i32 == 'F' as i32 {
            5 as i32
        } else if 211 as i32 == 'G' as i32 {
            6 as i32
        } else if 211 as i32 == 'H' as i32 {
            7 as i32
        } else if 211 as i32 == 'I' as i32 {
            8 as i32
        } else if 211 as i32 == 'J' as i32 {
            9 as i32
        } else if 211 as i32 == 'K' as i32 {
            10 as i32
        } else if 211 as i32 == 'L' as i32 {
            11 as i32
        } else if 211 as i32 == 'M' as i32 {
            12 as i32
        } else if 211 as i32 == 'N' as i32 {
            13 as i32
        } else if 211 as i32 == 'O' as i32 {
            14 as i32
        } else if 211 as i32 == 'P' as i32 {
            15 as i32
        } else if 211 as i32 == 'Q' as i32 {
            16 as i32
        } else if 211 as i32 == 'R' as i32 {
            17 as i32
        } else if 211 as i32 == 'S' as i32 {
            18 as i32
        } else if 211 as i32 == 'T' as i32 {
            19 as i32
        } else if 211 as i32 == 'U' as i32 {
            20 as i32
        } else if 211 as i32 == 'V' as i32 {
            21 as i32
        } else if 211 as i32 == 'W' as i32 {
            22 as i32
        } else if 211 as i32 == 'X' as i32 {
            23 as i32
        } else if 211 as i32 == 'Y' as i32 {
            24 as i32
        } else if 211 as i32 == 'Z' as i32 {
            25 as i32
        } else if 211 as i32 == '2' as i32 {
            26 as i32
        } else if 211 as i32 == '3' as i32 {
            27 as i32
        } else if 211 as i32 == '4' as i32 {
            28 as i32
        } else if 211 as i32 == '5' as i32 {
            29 as i32
        } else if 211 as i32 == '6' as i32 {
            30 as i32
        } else if 211 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 212 as i32 == 'A' as i32 {
            0 as i32
        } else if 212 as i32 == 'B' as i32 {
            1 as i32
        } else if 212 as i32 == 'C' as i32 {
            2 as i32
        } else if 212 as i32 == 'D' as i32 {
            3 as i32
        } else if 212 as i32 == 'E' as i32 {
            4 as i32
        } else if 212 as i32 == 'F' as i32 {
            5 as i32
        } else if 212 as i32 == 'G' as i32 {
            6 as i32
        } else if 212 as i32 == 'H' as i32 {
            7 as i32
        } else if 212 as i32 == 'I' as i32 {
            8 as i32
        } else if 212 as i32 == 'J' as i32 {
            9 as i32
        } else if 212 as i32 == 'K' as i32 {
            10 as i32
        } else if 212 as i32 == 'L' as i32 {
            11 as i32
        } else if 212 as i32 == 'M' as i32 {
            12 as i32
        } else if 212 as i32 == 'N' as i32 {
            13 as i32
        } else if 212 as i32 == 'O' as i32 {
            14 as i32
        } else if 212 as i32 == 'P' as i32 {
            15 as i32
        } else if 212 as i32 == 'Q' as i32 {
            16 as i32
        } else if 212 as i32 == 'R' as i32 {
            17 as i32
        } else if 212 as i32 == 'S' as i32 {
            18 as i32
        } else if 212 as i32 == 'T' as i32 {
            19 as i32
        } else if 212 as i32 == 'U' as i32 {
            20 as i32
        } else if 212 as i32 == 'V' as i32 {
            21 as i32
        } else if 212 as i32 == 'W' as i32 {
            22 as i32
        } else if 212 as i32 == 'X' as i32 {
            23 as i32
        } else if 212 as i32 == 'Y' as i32 {
            24 as i32
        } else if 212 as i32 == 'Z' as i32 {
            25 as i32
        } else if 212 as i32 == '2' as i32 {
            26 as i32
        } else if 212 as i32 == '3' as i32 {
            27 as i32
        } else if 212 as i32 == '4' as i32 {
            28 as i32
        } else if 212 as i32 == '5' as i32 {
            29 as i32
        } else if 212 as i32 == '6' as i32 {
            30 as i32
        } else if 212 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 213 as i32 == 'A' as i32 {
            0 as i32
        } else if 213 as i32 == 'B' as i32 {
            1 as i32
        } else if 213 as i32 == 'C' as i32 {
            2 as i32
        } else if 213 as i32 == 'D' as i32 {
            3 as i32
        } else if 213 as i32 == 'E' as i32 {
            4 as i32
        } else if 213 as i32 == 'F' as i32 {
            5 as i32
        } else if 213 as i32 == 'G' as i32 {
            6 as i32
        } else if 213 as i32 == 'H' as i32 {
            7 as i32
        } else if 213 as i32 == 'I' as i32 {
            8 as i32
        } else if 213 as i32 == 'J' as i32 {
            9 as i32
        } else if 213 as i32 == 'K' as i32 {
            10 as i32
        } else if 213 as i32 == 'L' as i32 {
            11 as i32
        } else if 213 as i32 == 'M' as i32 {
            12 as i32
        } else if 213 as i32 == 'N' as i32 {
            13 as i32
        } else if 213 as i32 == 'O' as i32 {
            14 as i32
        } else if 213 as i32 == 'P' as i32 {
            15 as i32
        } else if 213 as i32 == 'Q' as i32 {
            16 as i32
        } else if 213 as i32 == 'R' as i32 {
            17 as i32
        } else if 213 as i32 == 'S' as i32 {
            18 as i32
        } else if 213 as i32 == 'T' as i32 {
            19 as i32
        } else if 213 as i32 == 'U' as i32 {
            20 as i32
        } else if 213 as i32 == 'V' as i32 {
            21 as i32
        } else if 213 as i32 == 'W' as i32 {
            22 as i32
        } else if 213 as i32 == 'X' as i32 {
            23 as i32
        } else if 213 as i32 == 'Y' as i32 {
            24 as i32
        } else if 213 as i32 == 'Z' as i32 {
            25 as i32
        } else if 213 as i32 == '2' as i32 {
            26 as i32
        } else if 213 as i32 == '3' as i32 {
            27 as i32
        } else if 213 as i32 == '4' as i32 {
            28 as i32
        } else if 213 as i32 == '5' as i32 {
            29 as i32
        } else if 213 as i32 == '6' as i32 {
            30 as i32
        } else if 213 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 214 as i32 == 'A' as i32 {
            0 as i32
        } else if 214 as i32 == 'B' as i32 {
            1 as i32
        } else if 214 as i32 == 'C' as i32 {
            2 as i32
        } else if 214 as i32 == 'D' as i32 {
            3 as i32
        } else if 214 as i32 == 'E' as i32 {
            4 as i32
        } else if 214 as i32 == 'F' as i32 {
            5 as i32
        } else if 214 as i32 == 'G' as i32 {
            6 as i32
        } else if 214 as i32 == 'H' as i32 {
            7 as i32
        } else if 214 as i32 == 'I' as i32 {
            8 as i32
        } else if 214 as i32 == 'J' as i32 {
            9 as i32
        } else if 214 as i32 == 'K' as i32 {
            10 as i32
        } else if 214 as i32 == 'L' as i32 {
            11 as i32
        } else if 214 as i32 == 'M' as i32 {
            12 as i32
        } else if 214 as i32 == 'N' as i32 {
            13 as i32
        } else if 214 as i32 == 'O' as i32 {
            14 as i32
        } else if 214 as i32 == 'P' as i32 {
            15 as i32
        } else if 214 as i32 == 'Q' as i32 {
            16 as i32
        } else if 214 as i32 == 'R' as i32 {
            17 as i32
        } else if 214 as i32 == 'S' as i32 {
            18 as i32
        } else if 214 as i32 == 'T' as i32 {
            19 as i32
        } else if 214 as i32 == 'U' as i32 {
            20 as i32
        } else if 214 as i32 == 'V' as i32 {
            21 as i32
        } else if 214 as i32 == 'W' as i32 {
            22 as i32
        } else if 214 as i32 == 'X' as i32 {
            23 as i32
        } else if 214 as i32 == 'Y' as i32 {
            24 as i32
        } else if 214 as i32 == 'Z' as i32 {
            25 as i32
        } else if 214 as i32 == '2' as i32 {
            26 as i32
        } else if 214 as i32 == '3' as i32 {
            27 as i32
        } else if 214 as i32 == '4' as i32 {
            28 as i32
        } else if 214 as i32 == '5' as i32 {
            29 as i32
        } else if 214 as i32 == '6' as i32 {
            30 as i32
        } else if 214 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 215 as i32 == 'A' as i32 {
            0 as i32
        } else if 215 as i32 == 'B' as i32 {
            1 as i32
        } else if 215 as i32 == 'C' as i32 {
            2 as i32
        } else if 215 as i32 == 'D' as i32 {
            3 as i32
        } else if 215 as i32 == 'E' as i32 {
            4 as i32
        } else if 215 as i32 == 'F' as i32 {
            5 as i32
        } else if 215 as i32 == 'G' as i32 {
            6 as i32
        } else if 215 as i32 == 'H' as i32 {
            7 as i32
        } else if 215 as i32 == 'I' as i32 {
            8 as i32
        } else if 215 as i32 == 'J' as i32 {
            9 as i32
        } else if 215 as i32 == 'K' as i32 {
            10 as i32
        } else if 215 as i32 == 'L' as i32 {
            11 as i32
        } else if 215 as i32 == 'M' as i32 {
            12 as i32
        } else if 215 as i32 == 'N' as i32 {
            13 as i32
        } else if 215 as i32 == 'O' as i32 {
            14 as i32
        } else if 215 as i32 == 'P' as i32 {
            15 as i32
        } else if 215 as i32 == 'Q' as i32 {
            16 as i32
        } else if 215 as i32 == 'R' as i32 {
            17 as i32
        } else if 215 as i32 == 'S' as i32 {
            18 as i32
        } else if 215 as i32 == 'T' as i32 {
            19 as i32
        } else if 215 as i32 == 'U' as i32 {
            20 as i32
        } else if 215 as i32 == 'V' as i32 {
            21 as i32
        } else if 215 as i32 == 'W' as i32 {
            22 as i32
        } else if 215 as i32 == 'X' as i32 {
            23 as i32
        } else if 215 as i32 == 'Y' as i32 {
            24 as i32
        } else if 215 as i32 == 'Z' as i32 {
            25 as i32
        } else if 215 as i32 == '2' as i32 {
            26 as i32
        } else if 215 as i32 == '3' as i32 {
            27 as i32
        } else if 215 as i32 == '4' as i32 {
            28 as i32
        } else if 215 as i32 == '5' as i32 {
            29 as i32
        } else if 215 as i32 == '6' as i32 {
            30 as i32
        } else if 215 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 216 as i32 == 'A' as i32 {
            0 as i32
        } else if 216 as i32 == 'B' as i32 {
            1 as i32
        } else if 216 as i32 == 'C' as i32 {
            2 as i32
        } else if 216 as i32 == 'D' as i32 {
            3 as i32
        } else if 216 as i32 == 'E' as i32 {
            4 as i32
        } else if 216 as i32 == 'F' as i32 {
            5 as i32
        } else if 216 as i32 == 'G' as i32 {
            6 as i32
        } else if 216 as i32 == 'H' as i32 {
            7 as i32
        } else if 216 as i32 == 'I' as i32 {
            8 as i32
        } else if 216 as i32 == 'J' as i32 {
            9 as i32
        } else if 216 as i32 == 'K' as i32 {
            10 as i32
        } else if 216 as i32 == 'L' as i32 {
            11 as i32
        } else if 216 as i32 == 'M' as i32 {
            12 as i32
        } else if 216 as i32 == 'N' as i32 {
            13 as i32
        } else if 216 as i32 == 'O' as i32 {
            14 as i32
        } else if 216 as i32 == 'P' as i32 {
            15 as i32
        } else if 216 as i32 == 'Q' as i32 {
            16 as i32
        } else if 216 as i32 == 'R' as i32 {
            17 as i32
        } else if 216 as i32 == 'S' as i32 {
            18 as i32
        } else if 216 as i32 == 'T' as i32 {
            19 as i32
        } else if 216 as i32 == 'U' as i32 {
            20 as i32
        } else if 216 as i32 == 'V' as i32 {
            21 as i32
        } else if 216 as i32 == 'W' as i32 {
            22 as i32
        } else if 216 as i32 == 'X' as i32 {
            23 as i32
        } else if 216 as i32 == 'Y' as i32 {
            24 as i32
        } else if 216 as i32 == 'Z' as i32 {
            25 as i32
        } else if 216 as i32 == '2' as i32 {
            26 as i32
        } else if 216 as i32 == '3' as i32 {
            27 as i32
        } else if 216 as i32 == '4' as i32 {
            28 as i32
        } else if 216 as i32 == '5' as i32 {
            29 as i32
        } else if 216 as i32 == '6' as i32 {
            30 as i32
        } else if 216 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 217 as i32 == 'A' as i32 {
            0 as i32
        } else if 217 as i32 == 'B' as i32 {
            1 as i32
        } else if 217 as i32 == 'C' as i32 {
            2 as i32
        } else if 217 as i32 == 'D' as i32 {
            3 as i32
        } else if 217 as i32 == 'E' as i32 {
            4 as i32
        } else if 217 as i32 == 'F' as i32 {
            5 as i32
        } else if 217 as i32 == 'G' as i32 {
            6 as i32
        } else if 217 as i32 == 'H' as i32 {
            7 as i32
        } else if 217 as i32 == 'I' as i32 {
            8 as i32
        } else if 217 as i32 == 'J' as i32 {
            9 as i32
        } else if 217 as i32 == 'K' as i32 {
            10 as i32
        } else if 217 as i32 == 'L' as i32 {
            11 as i32
        } else if 217 as i32 == 'M' as i32 {
            12 as i32
        } else if 217 as i32 == 'N' as i32 {
            13 as i32
        } else if 217 as i32 == 'O' as i32 {
            14 as i32
        } else if 217 as i32 == 'P' as i32 {
            15 as i32
        } else if 217 as i32 == 'Q' as i32 {
            16 as i32
        } else if 217 as i32 == 'R' as i32 {
            17 as i32
        } else if 217 as i32 == 'S' as i32 {
            18 as i32
        } else if 217 as i32 == 'T' as i32 {
            19 as i32
        } else if 217 as i32 == 'U' as i32 {
            20 as i32
        } else if 217 as i32 == 'V' as i32 {
            21 as i32
        } else if 217 as i32 == 'W' as i32 {
            22 as i32
        } else if 217 as i32 == 'X' as i32 {
            23 as i32
        } else if 217 as i32 == 'Y' as i32 {
            24 as i32
        } else if 217 as i32 == 'Z' as i32 {
            25 as i32
        } else if 217 as i32 == '2' as i32 {
            26 as i32
        } else if 217 as i32 == '3' as i32 {
            27 as i32
        } else if 217 as i32 == '4' as i32 {
            28 as i32
        } else if 217 as i32 == '5' as i32 {
            29 as i32
        } else if 217 as i32 == '6' as i32 {
            30 as i32
        } else if 217 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 218 as i32 == 'A' as i32 {
            0 as i32
        } else if 218 as i32 == 'B' as i32 {
            1 as i32
        } else if 218 as i32 == 'C' as i32 {
            2 as i32
        } else if 218 as i32 == 'D' as i32 {
            3 as i32
        } else if 218 as i32 == 'E' as i32 {
            4 as i32
        } else if 218 as i32 == 'F' as i32 {
            5 as i32
        } else if 218 as i32 == 'G' as i32 {
            6 as i32
        } else if 218 as i32 == 'H' as i32 {
            7 as i32
        } else if 218 as i32 == 'I' as i32 {
            8 as i32
        } else if 218 as i32 == 'J' as i32 {
            9 as i32
        } else if 218 as i32 == 'K' as i32 {
            10 as i32
        } else if 218 as i32 == 'L' as i32 {
            11 as i32
        } else if 218 as i32 == 'M' as i32 {
            12 as i32
        } else if 218 as i32 == 'N' as i32 {
            13 as i32
        } else if 218 as i32 == 'O' as i32 {
            14 as i32
        } else if 218 as i32 == 'P' as i32 {
            15 as i32
        } else if 218 as i32 == 'Q' as i32 {
            16 as i32
        } else if 218 as i32 == 'R' as i32 {
            17 as i32
        } else if 218 as i32 == 'S' as i32 {
            18 as i32
        } else if 218 as i32 == 'T' as i32 {
            19 as i32
        } else if 218 as i32 == 'U' as i32 {
            20 as i32
        } else if 218 as i32 == 'V' as i32 {
            21 as i32
        } else if 218 as i32 == 'W' as i32 {
            22 as i32
        } else if 218 as i32 == 'X' as i32 {
            23 as i32
        } else if 218 as i32 == 'Y' as i32 {
            24 as i32
        } else if 218 as i32 == 'Z' as i32 {
            25 as i32
        } else if 218 as i32 == '2' as i32 {
            26 as i32
        } else if 218 as i32 == '3' as i32 {
            27 as i32
        } else if 218 as i32 == '4' as i32 {
            28 as i32
        } else if 218 as i32 == '5' as i32 {
            29 as i32
        } else if 218 as i32 == '6' as i32 {
            30 as i32
        } else if 218 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 219 as i32 == 'A' as i32 {
            0 as i32
        } else if 219 as i32 == 'B' as i32 {
            1 as i32
        } else if 219 as i32 == 'C' as i32 {
            2 as i32
        } else if 219 as i32 == 'D' as i32 {
            3 as i32
        } else if 219 as i32 == 'E' as i32 {
            4 as i32
        } else if 219 as i32 == 'F' as i32 {
            5 as i32
        } else if 219 as i32 == 'G' as i32 {
            6 as i32
        } else if 219 as i32 == 'H' as i32 {
            7 as i32
        } else if 219 as i32 == 'I' as i32 {
            8 as i32
        } else if 219 as i32 == 'J' as i32 {
            9 as i32
        } else if 219 as i32 == 'K' as i32 {
            10 as i32
        } else if 219 as i32 == 'L' as i32 {
            11 as i32
        } else if 219 as i32 == 'M' as i32 {
            12 as i32
        } else if 219 as i32 == 'N' as i32 {
            13 as i32
        } else if 219 as i32 == 'O' as i32 {
            14 as i32
        } else if 219 as i32 == 'P' as i32 {
            15 as i32
        } else if 219 as i32 == 'Q' as i32 {
            16 as i32
        } else if 219 as i32 == 'R' as i32 {
            17 as i32
        } else if 219 as i32 == 'S' as i32 {
            18 as i32
        } else if 219 as i32 == 'T' as i32 {
            19 as i32
        } else if 219 as i32 == 'U' as i32 {
            20 as i32
        } else if 219 as i32 == 'V' as i32 {
            21 as i32
        } else if 219 as i32 == 'W' as i32 {
            22 as i32
        } else if 219 as i32 == 'X' as i32 {
            23 as i32
        } else if 219 as i32 == 'Y' as i32 {
            24 as i32
        } else if 219 as i32 == 'Z' as i32 {
            25 as i32
        } else if 219 as i32 == '2' as i32 {
            26 as i32
        } else if 219 as i32 == '3' as i32 {
            27 as i32
        } else if 219 as i32 == '4' as i32 {
            28 as i32
        } else if 219 as i32 == '5' as i32 {
            29 as i32
        } else if 219 as i32 == '6' as i32 {
            30 as i32
        } else if 219 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 220 as i32 == 'A' as i32 {
            0 as i32
        } else if 220 as i32 == 'B' as i32 {
            1 as i32
        } else if 220 as i32 == 'C' as i32 {
            2 as i32
        } else if 220 as i32 == 'D' as i32 {
            3 as i32
        } else if 220 as i32 == 'E' as i32 {
            4 as i32
        } else if 220 as i32 == 'F' as i32 {
            5 as i32
        } else if 220 as i32 == 'G' as i32 {
            6 as i32
        } else if 220 as i32 == 'H' as i32 {
            7 as i32
        } else if 220 as i32 == 'I' as i32 {
            8 as i32
        } else if 220 as i32 == 'J' as i32 {
            9 as i32
        } else if 220 as i32 == 'K' as i32 {
            10 as i32
        } else if 220 as i32 == 'L' as i32 {
            11 as i32
        } else if 220 as i32 == 'M' as i32 {
            12 as i32
        } else if 220 as i32 == 'N' as i32 {
            13 as i32
        } else if 220 as i32 == 'O' as i32 {
            14 as i32
        } else if 220 as i32 == 'P' as i32 {
            15 as i32
        } else if 220 as i32 == 'Q' as i32 {
            16 as i32
        } else if 220 as i32 == 'R' as i32 {
            17 as i32
        } else if 220 as i32 == 'S' as i32 {
            18 as i32
        } else if 220 as i32 == 'T' as i32 {
            19 as i32
        } else if 220 as i32 == 'U' as i32 {
            20 as i32
        } else if 220 as i32 == 'V' as i32 {
            21 as i32
        } else if 220 as i32 == 'W' as i32 {
            22 as i32
        } else if 220 as i32 == 'X' as i32 {
            23 as i32
        } else if 220 as i32 == 'Y' as i32 {
            24 as i32
        } else if 220 as i32 == 'Z' as i32 {
            25 as i32
        } else if 220 as i32 == '2' as i32 {
            26 as i32
        } else if 220 as i32 == '3' as i32 {
            27 as i32
        } else if 220 as i32 == '4' as i32 {
            28 as i32
        } else if 220 as i32 == '5' as i32 {
            29 as i32
        } else if 220 as i32 == '6' as i32 {
            30 as i32
        } else if 220 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 221 as i32 == 'A' as i32 {
            0 as i32
        } else if 221 as i32 == 'B' as i32 {
            1 as i32
        } else if 221 as i32 == 'C' as i32 {
            2 as i32
        } else if 221 as i32 == 'D' as i32 {
            3 as i32
        } else if 221 as i32 == 'E' as i32 {
            4 as i32
        } else if 221 as i32 == 'F' as i32 {
            5 as i32
        } else if 221 as i32 == 'G' as i32 {
            6 as i32
        } else if 221 as i32 == 'H' as i32 {
            7 as i32
        } else if 221 as i32 == 'I' as i32 {
            8 as i32
        } else if 221 as i32 == 'J' as i32 {
            9 as i32
        } else if 221 as i32 == 'K' as i32 {
            10 as i32
        } else if 221 as i32 == 'L' as i32 {
            11 as i32
        } else if 221 as i32 == 'M' as i32 {
            12 as i32
        } else if 221 as i32 == 'N' as i32 {
            13 as i32
        } else if 221 as i32 == 'O' as i32 {
            14 as i32
        } else if 221 as i32 == 'P' as i32 {
            15 as i32
        } else if 221 as i32 == 'Q' as i32 {
            16 as i32
        } else if 221 as i32 == 'R' as i32 {
            17 as i32
        } else if 221 as i32 == 'S' as i32 {
            18 as i32
        } else if 221 as i32 == 'T' as i32 {
            19 as i32
        } else if 221 as i32 == 'U' as i32 {
            20 as i32
        } else if 221 as i32 == 'V' as i32 {
            21 as i32
        } else if 221 as i32 == 'W' as i32 {
            22 as i32
        } else if 221 as i32 == 'X' as i32 {
            23 as i32
        } else if 221 as i32 == 'Y' as i32 {
            24 as i32
        } else if 221 as i32 == 'Z' as i32 {
            25 as i32
        } else if 221 as i32 == '2' as i32 {
            26 as i32
        } else if 221 as i32 == '3' as i32 {
            27 as i32
        } else if 221 as i32 == '4' as i32 {
            28 as i32
        } else if 221 as i32 == '5' as i32 {
            29 as i32
        } else if 221 as i32 == '6' as i32 {
            30 as i32
        } else if 221 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 222 as i32 == 'A' as i32 {
            0 as i32
        } else if 222 as i32 == 'B' as i32 {
            1 as i32
        } else if 222 as i32 == 'C' as i32 {
            2 as i32
        } else if 222 as i32 == 'D' as i32 {
            3 as i32
        } else if 222 as i32 == 'E' as i32 {
            4 as i32
        } else if 222 as i32 == 'F' as i32 {
            5 as i32
        } else if 222 as i32 == 'G' as i32 {
            6 as i32
        } else if 222 as i32 == 'H' as i32 {
            7 as i32
        } else if 222 as i32 == 'I' as i32 {
            8 as i32
        } else if 222 as i32 == 'J' as i32 {
            9 as i32
        } else if 222 as i32 == 'K' as i32 {
            10 as i32
        } else if 222 as i32 == 'L' as i32 {
            11 as i32
        } else if 222 as i32 == 'M' as i32 {
            12 as i32
        } else if 222 as i32 == 'N' as i32 {
            13 as i32
        } else if 222 as i32 == 'O' as i32 {
            14 as i32
        } else if 222 as i32 == 'P' as i32 {
            15 as i32
        } else if 222 as i32 == 'Q' as i32 {
            16 as i32
        } else if 222 as i32 == 'R' as i32 {
            17 as i32
        } else if 222 as i32 == 'S' as i32 {
            18 as i32
        } else if 222 as i32 == 'T' as i32 {
            19 as i32
        } else if 222 as i32 == 'U' as i32 {
            20 as i32
        } else if 222 as i32 == 'V' as i32 {
            21 as i32
        } else if 222 as i32 == 'W' as i32 {
            22 as i32
        } else if 222 as i32 == 'X' as i32 {
            23 as i32
        } else if 222 as i32 == 'Y' as i32 {
            24 as i32
        } else if 222 as i32 == 'Z' as i32 {
            25 as i32
        } else if 222 as i32 == '2' as i32 {
            26 as i32
        } else if 222 as i32 == '3' as i32 {
            27 as i32
        } else if 222 as i32 == '4' as i32 {
            28 as i32
        } else if 222 as i32 == '5' as i32 {
            29 as i32
        } else if 222 as i32 == '6' as i32 {
            30 as i32
        } else if 222 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 223 as i32 == 'A' as i32 {
            0 as i32
        } else if 223 as i32 == 'B' as i32 {
            1 as i32
        } else if 223 as i32 == 'C' as i32 {
            2 as i32
        } else if 223 as i32 == 'D' as i32 {
            3 as i32
        } else if 223 as i32 == 'E' as i32 {
            4 as i32
        } else if 223 as i32 == 'F' as i32 {
            5 as i32
        } else if 223 as i32 == 'G' as i32 {
            6 as i32
        } else if 223 as i32 == 'H' as i32 {
            7 as i32
        } else if 223 as i32 == 'I' as i32 {
            8 as i32
        } else if 223 as i32 == 'J' as i32 {
            9 as i32
        } else if 223 as i32 == 'K' as i32 {
            10 as i32
        } else if 223 as i32 == 'L' as i32 {
            11 as i32
        } else if 223 as i32 == 'M' as i32 {
            12 as i32
        } else if 223 as i32 == 'N' as i32 {
            13 as i32
        } else if 223 as i32 == 'O' as i32 {
            14 as i32
        } else if 223 as i32 == 'P' as i32 {
            15 as i32
        } else if 223 as i32 == 'Q' as i32 {
            16 as i32
        } else if 223 as i32 == 'R' as i32 {
            17 as i32
        } else if 223 as i32 == 'S' as i32 {
            18 as i32
        } else if 223 as i32 == 'T' as i32 {
            19 as i32
        } else if 223 as i32 == 'U' as i32 {
            20 as i32
        } else if 223 as i32 == 'V' as i32 {
            21 as i32
        } else if 223 as i32 == 'W' as i32 {
            22 as i32
        } else if 223 as i32 == 'X' as i32 {
            23 as i32
        } else if 223 as i32 == 'Y' as i32 {
            24 as i32
        } else if 223 as i32 == 'Z' as i32 {
            25 as i32
        } else if 223 as i32 == '2' as i32 {
            26 as i32
        } else if 223 as i32 == '3' as i32 {
            27 as i32
        } else if 223 as i32 == '4' as i32 {
            28 as i32
        } else if 223 as i32 == '5' as i32 {
            29 as i32
        } else if 223 as i32 == '6' as i32 {
            30 as i32
        } else if 223 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 224 as i32 == 'A' as i32 {
            0 as i32
        } else if 224 as i32 == 'B' as i32 {
            1 as i32
        } else if 224 as i32 == 'C' as i32 {
            2 as i32
        } else if 224 as i32 == 'D' as i32 {
            3 as i32
        } else if 224 as i32 == 'E' as i32 {
            4 as i32
        } else if 224 as i32 == 'F' as i32 {
            5 as i32
        } else if 224 as i32 == 'G' as i32 {
            6 as i32
        } else if 224 as i32 == 'H' as i32 {
            7 as i32
        } else if 224 as i32 == 'I' as i32 {
            8 as i32
        } else if 224 as i32 == 'J' as i32 {
            9 as i32
        } else if 224 as i32 == 'K' as i32 {
            10 as i32
        } else if 224 as i32 == 'L' as i32 {
            11 as i32
        } else if 224 as i32 == 'M' as i32 {
            12 as i32
        } else if 224 as i32 == 'N' as i32 {
            13 as i32
        } else if 224 as i32 == 'O' as i32 {
            14 as i32
        } else if 224 as i32 == 'P' as i32 {
            15 as i32
        } else if 224 as i32 == 'Q' as i32 {
            16 as i32
        } else if 224 as i32 == 'R' as i32 {
            17 as i32
        } else if 224 as i32 == 'S' as i32 {
            18 as i32
        } else if 224 as i32 == 'T' as i32 {
            19 as i32
        } else if 224 as i32 == 'U' as i32 {
            20 as i32
        } else if 224 as i32 == 'V' as i32 {
            21 as i32
        } else if 224 as i32 == 'W' as i32 {
            22 as i32
        } else if 224 as i32 == 'X' as i32 {
            23 as i32
        } else if 224 as i32 == 'Y' as i32 {
            24 as i32
        } else if 224 as i32 == 'Z' as i32 {
            25 as i32
        } else if 224 as i32 == '2' as i32 {
            26 as i32
        } else if 224 as i32 == '3' as i32 {
            27 as i32
        } else if 224 as i32 == '4' as i32 {
            28 as i32
        } else if 224 as i32 == '5' as i32 {
            29 as i32
        } else if 224 as i32 == '6' as i32 {
            30 as i32
        } else if 224 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 225 as i32 == 'A' as i32 {
            0 as i32
        } else if 225 as i32 == 'B' as i32 {
            1 as i32
        } else if 225 as i32 == 'C' as i32 {
            2 as i32
        } else if 225 as i32 == 'D' as i32 {
            3 as i32
        } else if 225 as i32 == 'E' as i32 {
            4 as i32
        } else if 225 as i32 == 'F' as i32 {
            5 as i32
        } else if 225 as i32 == 'G' as i32 {
            6 as i32
        } else if 225 as i32 == 'H' as i32 {
            7 as i32
        } else if 225 as i32 == 'I' as i32 {
            8 as i32
        } else if 225 as i32 == 'J' as i32 {
            9 as i32
        } else if 225 as i32 == 'K' as i32 {
            10 as i32
        } else if 225 as i32 == 'L' as i32 {
            11 as i32
        } else if 225 as i32 == 'M' as i32 {
            12 as i32
        } else if 225 as i32 == 'N' as i32 {
            13 as i32
        } else if 225 as i32 == 'O' as i32 {
            14 as i32
        } else if 225 as i32 == 'P' as i32 {
            15 as i32
        } else if 225 as i32 == 'Q' as i32 {
            16 as i32
        } else if 225 as i32 == 'R' as i32 {
            17 as i32
        } else if 225 as i32 == 'S' as i32 {
            18 as i32
        } else if 225 as i32 == 'T' as i32 {
            19 as i32
        } else if 225 as i32 == 'U' as i32 {
            20 as i32
        } else if 225 as i32 == 'V' as i32 {
            21 as i32
        } else if 225 as i32 == 'W' as i32 {
            22 as i32
        } else if 225 as i32 == 'X' as i32 {
            23 as i32
        } else if 225 as i32 == 'Y' as i32 {
            24 as i32
        } else if 225 as i32 == 'Z' as i32 {
            25 as i32
        } else if 225 as i32 == '2' as i32 {
            26 as i32
        } else if 225 as i32 == '3' as i32 {
            27 as i32
        } else if 225 as i32 == '4' as i32 {
            28 as i32
        } else if 225 as i32 == '5' as i32 {
            29 as i32
        } else if 225 as i32 == '6' as i32 {
            30 as i32
        } else if 225 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 226 as i32 == 'A' as i32 {
            0 as i32
        } else if 226 as i32 == 'B' as i32 {
            1 as i32
        } else if 226 as i32 == 'C' as i32 {
            2 as i32
        } else if 226 as i32 == 'D' as i32 {
            3 as i32
        } else if 226 as i32 == 'E' as i32 {
            4 as i32
        } else if 226 as i32 == 'F' as i32 {
            5 as i32
        } else if 226 as i32 == 'G' as i32 {
            6 as i32
        } else if 226 as i32 == 'H' as i32 {
            7 as i32
        } else if 226 as i32 == 'I' as i32 {
            8 as i32
        } else if 226 as i32 == 'J' as i32 {
            9 as i32
        } else if 226 as i32 == 'K' as i32 {
            10 as i32
        } else if 226 as i32 == 'L' as i32 {
            11 as i32
        } else if 226 as i32 == 'M' as i32 {
            12 as i32
        } else if 226 as i32 == 'N' as i32 {
            13 as i32
        } else if 226 as i32 == 'O' as i32 {
            14 as i32
        } else if 226 as i32 == 'P' as i32 {
            15 as i32
        } else if 226 as i32 == 'Q' as i32 {
            16 as i32
        } else if 226 as i32 == 'R' as i32 {
            17 as i32
        } else if 226 as i32 == 'S' as i32 {
            18 as i32
        } else if 226 as i32 == 'T' as i32 {
            19 as i32
        } else if 226 as i32 == 'U' as i32 {
            20 as i32
        } else if 226 as i32 == 'V' as i32 {
            21 as i32
        } else if 226 as i32 == 'W' as i32 {
            22 as i32
        } else if 226 as i32 == 'X' as i32 {
            23 as i32
        } else if 226 as i32 == 'Y' as i32 {
            24 as i32
        } else if 226 as i32 == 'Z' as i32 {
            25 as i32
        } else if 226 as i32 == '2' as i32 {
            26 as i32
        } else if 226 as i32 == '3' as i32 {
            27 as i32
        } else if 226 as i32 == '4' as i32 {
            28 as i32
        } else if 226 as i32 == '5' as i32 {
            29 as i32
        } else if 226 as i32 == '6' as i32 {
            30 as i32
        } else if 226 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 227 as i32 == 'A' as i32 {
            0 as i32
        } else if 227 as i32 == 'B' as i32 {
            1 as i32
        } else if 227 as i32 == 'C' as i32 {
            2 as i32
        } else if 227 as i32 == 'D' as i32 {
            3 as i32
        } else if 227 as i32 == 'E' as i32 {
            4 as i32
        } else if 227 as i32 == 'F' as i32 {
            5 as i32
        } else if 227 as i32 == 'G' as i32 {
            6 as i32
        } else if 227 as i32 == 'H' as i32 {
            7 as i32
        } else if 227 as i32 == 'I' as i32 {
            8 as i32
        } else if 227 as i32 == 'J' as i32 {
            9 as i32
        } else if 227 as i32 == 'K' as i32 {
            10 as i32
        } else if 227 as i32 == 'L' as i32 {
            11 as i32
        } else if 227 as i32 == 'M' as i32 {
            12 as i32
        } else if 227 as i32 == 'N' as i32 {
            13 as i32
        } else if 227 as i32 == 'O' as i32 {
            14 as i32
        } else if 227 as i32 == 'P' as i32 {
            15 as i32
        } else if 227 as i32 == 'Q' as i32 {
            16 as i32
        } else if 227 as i32 == 'R' as i32 {
            17 as i32
        } else if 227 as i32 == 'S' as i32 {
            18 as i32
        } else if 227 as i32 == 'T' as i32 {
            19 as i32
        } else if 227 as i32 == 'U' as i32 {
            20 as i32
        } else if 227 as i32 == 'V' as i32 {
            21 as i32
        } else if 227 as i32 == 'W' as i32 {
            22 as i32
        } else if 227 as i32 == 'X' as i32 {
            23 as i32
        } else if 227 as i32 == 'Y' as i32 {
            24 as i32
        } else if 227 as i32 == 'Z' as i32 {
            25 as i32
        } else if 227 as i32 == '2' as i32 {
            26 as i32
        } else if 227 as i32 == '3' as i32 {
            27 as i32
        } else if 227 as i32 == '4' as i32 {
            28 as i32
        } else if 227 as i32 == '5' as i32 {
            29 as i32
        } else if 227 as i32 == '6' as i32 {
            30 as i32
        } else if 227 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 228 as i32 == 'A' as i32 {
            0 as i32
        } else if 228 as i32 == 'B' as i32 {
            1 as i32
        } else if 228 as i32 == 'C' as i32 {
            2 as i32
        } else if 228 as i32 == 'D' as i32 {
            3 as i32
        } else if 228 as i32 == 'E' as i32 {
            4 as i32
        } else if 228 as i32 == 'F' as i32 {
            5 as i32
        } else if 228 as i32 == 'G' as i32 {
            6 as i32
        } else if 228 as i32 == 'H' as i32 {
            7 as i32
        } else if 228 as i32 == 'I' as i32 {
            8 as i32
        } else if 228 as i32 == 'J' as i32 {
            9 as i32
        } else if 228 as i32 == 'K' as i32 {
            10 as i32
        } else if 228 as i32 == 'L' as i32 {
            11 as i32
        } else if 228 as i32 == 'M' as i32 {
            12 as i32
        } else if 228 as i32 == 'N' as i32 {
            13 as i32
        } else if 228 as i32 == 'O' as i32 {
            14 as i32
        } else if 228 as i32 == 'P' as i32 {
            15 as i32
        } else if 228 as i32 == 'Q' as i32 {
            16 as i32
        } else if 228 as i32 == 'R' as i32 {
            17 as i32
        } else if 228 as i32 == 'S' as i32 {
            18 as i32
        } else if 228 as i32 == 'T' as i32 {
            19 as i32
        } else if 228 as i32 == 'U' as i32 {
            20 as i32
        } else if 228 as i32 == 'V' as i32 {
            21 as i32
        } else if 228 as i32 == 'W' as i32 {
            22 as i32
        } else if 228 as i32 == 'X' as i32 {
            23 as i32
        } else if 228 as i32 == 'Y' as i32 {
            24 as i32
        } else if 228 as i32 == 'Z' as i32 {
            25 as i32
        } else if 228 as i32 == '2' as i32 {
            26 as i32
        } else if 228 as i32 == '3' as i32 {
            27 as i32
        } else if 228 as i32 == '4' as i32 {
            28 as i32
        } else if 228 as i32 == '5' as i32 {
            29 as i32
        } else if 228 as i32 == '6' as i32 {
            30 as i32
        } else if 228 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 229 as i32 == 'A' as i32 {
            0 as i32
        } else if 229 as i32 == 'B' as i32 {
            1 as i32
        } else if 229 as i32 == 'C' as i32 {
            2 as i32
        } else if 229 as i32 == 'D' as i32 {
            3 as i32
        } else if 229 as i32 == 'E' as i32 {
            4 as i32
        } else if 229 as i32 == 'F' as i32 {
            5 as i32
        } else if 229 as i32 == 'G' as i32 {
            6 as i32
        } else if 229 as i32 == 'H' as i32 {
            7 as i32
        } else if 229 as i32 == 'I' as i32 {
            8 as i32
        } else if 229 as i32 == 'J' as i32 {
            9 as i32
        } else if 229 as i32 == 'K' as i32 {
            10 as i32
        } else if 229 as i32 == 'L' as i32 {
            11 as i32
        } else if 229 as i32 == 'M' as i32 {
            12 as i32
        } else if 229 as i32 == 'N' as i32 {
            13 as i32
        } else if 229 as i32 == 'O' as i32 {
            14 as i32
        } else if 229 as i32 == 'P' as i32 {
            15 as i32
        } else if 229 as i32 == 'Q' as i32 {
            16 as i32
        } else if 229 as i32 == 'R' as i32 {
            17 as i32
        } else if 229 as i32 == 'S' as i32 {
            18 as i32
        } else if 229 as i32 == 'T' as i32 {
            19 as i32
        } else if 229 as i32 == 'U' as i32 {
            20 as i32
        } else if 229 as i32 == 'V' as i32 {
            21 as i32
        } else if 229 as i32 == 'W' as i32 {
            22 as i32
        } else if 229 as i32 == 'X' as i32 {
            23 as i32
        } else if 229 as i32 == 'Y' as i32 {
            24 as i32
        } else if 229 as i32 == 'Z' as i32 {
            25 as i32
        } else if 229 as i32 == '2' as i32 {
            26 as i32
        } else if 229 as i32 == '3' as i32 {
            27 as i32
        } else if 229 as i32 == '4' as i32 {
            28 as i32
        } else if 229 as i32 == '5' as i32 {
            29 as i32
        } else if 229 as i32 == '6' as i32 {
            30 as i32
        } else if 229 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 230 as i32 == 'A' as i32 {
            0 as i32
        } else if 230 as i32 == 'B' as i32 {
            1 as i32
        } else if 230 as i32 == 'C' as i32 {
            2 as i32
        } else if 230 as i32 == 'D' as i32 {
            3 as i32
        } else if 230 as i32 == 'E' as i32 {
            4 as i32
        } else if 230 as i32 == 'F' as i32 {
            5 as i32
        } else if 230 as i32 == 'G' as i32 {
            6 as i32
        } else if 230 as i32 == 'H' as i32 {
            7 as i32
        } else if 230 as i32 == 'I' as i32 {
            8 as i32
        } else if 230 as i32 == 'J' as i32 {
            9 as i32
        } else if 230 as i32 == 'K' as i32 {
            10 as i32
        } else if 230 as i32 == 'L' as i32 {
            11 as i32
        } else if 230 as i32 == 'M' as i32 {
            12 as i32
        } else if 230 as i32 == 'N' as i32 {
            13 as i32
        } else if 230 as i32 == 'O' as i32 {
            14 as i32
        } else if 230 as i32 == 'P' as i32 {
            15 as i32
        } else if 230 as i32 == 'Q' as i32 {
            16 as i32
        } else if 230 as i32 == 'R' as i32 {
            17 as i32
        } else if 230 as i32 == 'S' as i32 {
            18 as i32
        } else if 230 as i32 == 'T' as i32 {
            19 as i32
        } else if 230 as i32 == 'U' as i32 {
            20 as i32
        } else if 230 as i32 == 'V' as i32 {
            21 as i32
        } else if 230 as i32 == 'W' as i32 {
            22 as i32
        } else if 230 as i32 == 'X' as i32 {
            23 as i32
        } else if 230 as i32 == 'Y' as i32 {
            24 as i32
        } else if 230 as i32 == 'Z' as i32 {
            25 as i32
        } else if 230 as i32 == '2' as i32 {
            26 as i32
        } else if 230 as i32 == '3' as i32 {
            27 as i32
        } else if 230 as i32 == '4' as i32 {
            28 as i32
        } else if 230 as i32 == '5' as i32 {
            29 as i32
        } else if 230 as i32 == '6' as i32 {
            30 as i32
        } else if 230 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 231 as i32 == 'A' as i32 {
            0 as i32
        } else if 231 as i32 == 'B' as i32 {
            1 as i32
        } else if 231 as i32 == 'C' as i32 {
            2 as i32
        } else if 231 as i32 == 'D' as i32 {
            3 as i32
        } else if 231 as i32 == 'E' as i32 {
            4 as i32
        } else if 231 as i32 == 'F' as i32 {
            5 as i32
        } else if 231 as i32 == 'G' as i32 {
            6 as i32
        } else if 231 as i32 == 'H' as i32 {
            7 as i32
        } else if 231 as i32 == 'I' as i32 {
            8 as i32
        } else if 231 as i32 == 'J' as i32 {
            9 as i32
        } else if 231 as i32 == 'K' as i32 {
            10 as i32
        } else if 231 as i32 == 'L' as i32 {
            11 as i32
        } else if 231 as i32 == 'M' as i32 {
            12 as i32
        } else if 231 as i32 == 'N' as i32 {
            13 as i32
        } else if 231 as i32 == 'O' as i32 {
            14 as i32
        } else if 231 as i32 == 'P' as i32 {
            15 as i32
        } else if 231 as i32 == 'Q' as i32 {
            16 as i32
        } else if 231 as i32 == 'R' as i32 {
            17 as i32
        } else if 231 as i32 == 'S' as i32 {
            18 as i32
        } else if 231 as i32 == 'T' as i32 {
            19 as i32
        } else if 231 as i32 == 'U' as i32 {
            20 as i32
        } else if 231 as i32 == 'V' as i32 {
            21 as i32
        } else if 231 as i32 == 'W' as i32 {
            22 as i32
        } else if 231 as i32 == 'X' as i32 {
            23 as i32
        } else if 231 as i32 == 'Y' as i32 {
            24 as i32
        } else if 231 as i32 == 'Z' as i32 {
            25 as i32
        } else if 231 as i32 == '2' as i32 {
            26 as i32
        } else if 231 as i32 == '3' as i32 {
            27 as i32
        } else if 231 as i32 == '4' as i32 {
            28 as i32
        } else if 231 as i32 == '5' as i32 {
            29 as i32
        } else if 231 as i32 == '6' as i32 {
            30 as i32
        } else if 231 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 232 as i32 == 'A' as i32 {
            0 as i32
        } else if 232 as i32 == 'B' as i32 {
            1 as i32
        } else if 232 as i32 == 'C' as i32 {
            2 as i32
        } else if 232 as i32 == 'D' as i32 {
            3 as i32
        } else if 232 as i32 == 'E' as i32 {
            4 as i32
        } else if 232 as i32 == 'F' as i32 {
            5 as i32
        } else if 232 as i32 == 'G' as i32 {
            6 as i32
        } else if 232 as i32 == 'H' as i32 {
            7 as i32
        } else if 232 as i32 == 'I' as i32 {
            8 as i32
        } else if 232 as i32 == 'J' as i32 {
            9 as i32
        } else if 232 as i32 == 'K' as i32 {
            10 as i32
        } else if 232 as i32 == 'L' as i32 {
            11 as i32
        } else if 232 as i32 == 'M' as i32 {
            12 as i32
        } else if 232 as i32 == 'N' as i32 {
            13 as i32
        } else if 232 as i32 == 'O' as i32 {
            14 as i32
        } else if 232 as i32 == 'P' as i32 {
            15 as i32
        } else if 232 as i32 == 'Q' as i32 {
            16 as i32
        } else if 232 as i32 == 'R' as i32 {
            17 as i32
        } else if 232 as i32 == 'S' as i32 {
            18 as i32
        } else if 232 as i32 == 'T' as i32 {
            19 as i32
        } else if 232 as i32 == 'U' as i32 {
            20 as i32
        } else if 232 as i32 == 'V' as i32 {
            21 as i32
        } else if 232 as i32 == 'W' as i32 {
            22 as i32
        } else if 232 as i32 == 'X' as i32 {
            23 as i32
        } else if 232 as i32 == 'Y' as i32 {
            24 as i32
        } else if 232 as i32 == 'Z' as i32 {
            25 as i32
        } else if 232 as i32 == '2' as i32 {
            26 as i32
        } else if 232 as i32 == '3' as i32 {
            27 as i32
        } else if 232 as i32 == '4' as i32 {
            28 as i32
        } else if 232 as i32 == '5' as i32 {
            29 as i32
        } else if 232 as i32 == '6' as i32 {
            30 as i32
        } else if 232 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 233 as i32 == 'A' as i32 {
            0 as i32
        } else if 233 as i32 == 'B' as i32 {
            1 as i32
        } else if 233 as i32 == 'C' as i32 {
            2 as i32
        } else if 233 as i32 == 'D' as i32 {
            3 as i32
        } else if 233 as i32 == 'E' as i32 {
            4 as i32
        } else if 233 as i32 == 'F' as i32 {
            5 as i32
        } else if 233 as i32 == 'G' as i32 {
            6 as i32
        } else if 233 as i32 == 'H' as i32 {
            7 as i32
        } else if 233 as i32 == 'I' as i32 {
            8 as i32
        } else if 233 as i32 == 'J' as i32 {
            9 as i32
        } else if 233 as i32 == 'K' as i32 {
            10 as i32
        } else if 233 as i32 == 'L' as i32 {
            11 as i32
        } else if 233 as i32 == 'M' as i32 {
            12 as i32
        } else if 233 as i32 == 'N' as i32 {
            13 as i32
        } else if 233 as i32 == 'O' as i32 {
            14 as i32
        } else if 233 as i32 == 'P' as i32 {
            15 as i32
        } else if 233 as i32 == 'Q' as i32 {
            16 as i32
        } else if 233 as i32 == 'R' as i32 {
            17 as i32
        } else if 233 as i32 == 'S' as i32 {
            18 as i32
        } else if 233 as i32 == 'T' as i32 {
            19 as i32
        } else if 233 as i32 == 'U' as i32 {
            20 as i32
        } else if 233 as i32 == 'V' as i32 {
            21 as i32
        } else if 233 as i32 == 'W' as i32 {
            22 as i32
        } else if 233 as i32 == 'X' as i32 {
            23 as i32
        } else if 233 as i32 == 'Y' as i32 {
            24 as i32
        } else if 233 as i32 == 'Z' as i32 {
            25 as i32
        } else if 233 as i32 == '2' as i32 {
            26 as i32
        } else if 233 as i32 == '3' as i32 {
            27 as i32
        } else if 233 as i32 == '4' as i32 {
            28 as i32
        } else if 233 as i32 == '5' as i32 {
            29 as i32
        } else if 233 as i32 == '6' as i32 {
            30 as i32
        } else if 233 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 234 as i32 == 'A' as i32 {
            0 as i32
        } else if 234 as i32 == 'B' as i32 {
            1 as i32
        } else if 234 as i32 == 'C' as i32 {
            2 as i32
        } else if 234 as i32 == 'D' as i32 {
            3 as i32
        } else if 234 as i32 == 'E' as i32 {
            4 as i32
        } else if 234 as i32 == 'F' as i32 {
            5 as i32
        } else if 234 as i32 == 'G' as i32 {
            6 as i32
        } else if 234 as i32 == 'H' as i32 {
            7 as i32
        } else if 234 as i32 == 'I' as i32 {
            8 as i32
        } else if 234 as i32 == 'J' as i32 {
            9 as i32
        } else if 234 as i32 == 'K' as i32 {
            10 as i32
        } else if 234 as i32 == 'L' as i32 {
            11 as i32
        } else if 234 as i32 == 'M' as i32 {
            12 as i32
        } else if 234 as i32 == 'N' as i32 {
            13 as i32
        } else if 234 as i32 == 'O' as i32 {
            14 as i32
        } else if 234 as i32 == 'P' as i32 {
            15 as i32
        } else if 234 as i32 == 'Q' as i32 {
            16 as i32
        } else if 234 as i32 == 'R' as i32 {
            17 as i32
        } else if 234 as i32 == 'S' as i32 {
            18 as i32
        } else if 234 as i32 == 'T' as i32 {
            19 as i32
        } else if 234 as i32 == 'U' as i32 {
            20 as i32
        } else if 234 as i32 == 'V' as i32 {
            21 as i32
        } else if 234 as i32 == 'W' as i32 {
            22 as i32
        } else if 234 as i32 == 'X' as i32 {
            23 as i32
        } else if 234 as i32 == 'Y' as i32 {
            24 as i32
        } else if 234 as i32 == 'Z' as i32 {
            25 as i32
        } else if 234 as i32 == '2' as i32 {
            26 as i32
        } else if 234 as i32 == '3' as i32 {
            27 as i32
        } else if 234 as i32 == '4' as i32 {
            28 as i32
        } else if 234 as i32 == '5' as i32 {
            29 as i32
        } else if 234 as i32 == '6' as i32 {
            30 as i32
        } else if 234 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 235 as i32 == 'A' as i32 {
            0 as i32
        } else if 235 as i32 == 'B' as i32 {
            1 as i32
        } else if 235 as i32 == 'C' as i32 {
            2 as i32
        } else if 235 as i32 == 'D' as i32 {
            3 as i32
        } else if 235 as i32 == 'E' as i32 {
            4 as i32
        } else if 235 as i32 == 'F' as i32 {
            5 as i32
        } else if 235 as i32 == 'G' as i32 {
            6 as i32
        } else if 235 as i32 == 'H' as i32 {
            7 as i32
        } else if 235 as i32 == 'I' as i32 {
            8 as i32
        } else if 235 as i32 == 'J' as i32 {
            9 as i32
        } else if 235 as i32 == 'K' as i32 {
            10 as i32
        } else if 235 as i32 == 'L' as i32 {
            11 as i32
        } else if 235 as i32 == 'M' as i32 {
            12 as i32
        } else if 235 as i32 == 'N' as i32 {
            13 as i32
        } else if 235 as i32 == 'O' as i32 {
            14 as i32
        } else if 235 as i32 == 'P' as i32 {
            15 as i32
        } else if 235 as i32 == 'Q' as i32 {
            16 as i32
        } else if 235 as i32 == 'R' as i32 {
            17 as i32
        } else if 235 as i32 == 'S' as i32 {
            18 as i32
        } else if 235 as i32 == 'T' as i32 {
            19 as i32
        } else if 235 as i32 == 'U' as i32 {
            20 as i32
        } else if 235 as i32 == 'V' as i32 {
            21 as i32
        } else if 235 as i32 == 'W' as i32 {
            22 as i32
        } else if 235 as i32 == 'X' as i32 {
            23 as i32
        } else if 235 as i32 == 'Y' as i32 {
            24 as i32
        } else if 235 as i32 == 'Z' as i32 {
            25 as i32
        } else if 235 as i32 == '2' as i32 {
            26 as i32
        } else if 235 as i32 == '3' as i32 {
            27 as i32
        } else if 235 as i32 == '4' as i32 {
            28 as i32
        } else if 235 as i32 == '5' as i32 {
            29 as i32
        } else if 235 as i32 == '6' as i32 {
            30 as i32
        } else if 235 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 236 as i32 == 'A' as i32 {
            0 as i32
        } else if 236 as i32 == 'B' as i32 {
            1 as i32
        } else if 236 as i32 == 'C' as i32 {
            2 as i32
        } else if 236 as i32 == 'D' as i32 {
            3 as i32
        } else if 236 as i32 == 'E' as i32 {
            4 as i32
        } else if 236 as i32 == 'F' as i32 {
            5 as i32
        } else if 236 as i32 == 'G' as i32 {
            6 as i32
        } else if 236 as i32 == 'H' as i32 {
            7 as i32
        } else if 236 as i32 == 'I' as i32 {
            8 as i32
        } else if 236 as i32 == 'J' as i32 {
            9 as i32
        } else if 236 as i32 == 'K' as i32 {
            10 as i32
        } else if 236 as i32 == 'L' as i32 {
            11 as i32
        } else if 236 as i32 == 'M' as i32 {
            12 as i32
        } else if 236 as i32 == 'N' as i32 {
            13 as i32
        } else if 236 as i32 == 'O' as i32 {
            14 as i32
        } else if 236 as i32 == 'P' as i32 {
            15 as i32
        } else if 236 as i32 == 'Q' as i32 {
            16 as i32
        } else if 236 as i32 == 'R' as i32 {
            17 as i32
        } else if 236 as i32 == 'S' as i32 {
            18 as i32
        } else if 236 as i32 == 'T' as i32 {
            19 as i32
        } else if 236 as i32 == 'U' as i32 {
            20 as i32
        } else if 236 as i32 == 'V' as i32 {
            21 as i32
        } else if 236 as i32 == 'W' as i32 {
            22 as i32
        } else if 236 as i32 == 'X' as i32 {
            23 as i32
        } else if 236 as i32 == 'Y' as i32 {
            24 as i32
        } else if 236 as i32 == 'Z' as i32 {
            25 as i32
        } else if 236 as i32 == '2' as i32 {
            26 as i32
        } else if 236 as i32 == '3' as i32 {
            27 as i32
        } else if 236 as i32 == '4' as i32 {
            28 as i32
        } else if 236 as i32 == '5' as i32 {
            29 as i32
        } else if 236 as i32 == '6' as i32 {
            30 as i32
        } else if 236 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 237 as i32 == 'A' as i32 {
            0 as i32
        } else if 237 as i32 == 'B' as i32 {
            1 as i32
        } else if 237 as i32 == 'C' as i32 {
            2 as i32
        } else if 237 as i32 == 'D' as i32 {
            3 as i32
        } else if 237 as i32 == 'E' as i32 {
            4 as i32
        } else if 237 as i32 == 'F' as i32 {
            5 as i32
        } else if 237 as i32 == 'G' as i32 {
            6 as i32
        } else if 237 as i32 == 'H' as i32 {
            7 as i32
        } else if 237 as i32 == 'I' as i32 {
            8 as i32
        } else if 237 as i32 == 'J' as i32 {
            9 as i32
        } else if 237 as i32 == 'K' as i32 {
            10 as i32
        } else if 237 as i32 == 'L' as i32 {
            11 as i32
        } else if 237 as i32 == 'M' as i32 {
            12 as i32
        } else if 237 as i32 == 'N' as i32 {
            13 as i32
        } else if 237 as i32 == 'O' as i32 {
            14 as i32
        } else if 237 as i32 == 'P' as i32 {
            15 as i32
        } else if 237 as i32 == 'Q' as i32 {
            16 as i32
        } else if 237 as i32 == 'R' as i32 {
            17 as i32
        } else if 237 as i32 == 'S' as i32 {
            18 as i32
        } else if 237 as i32 == 'T' as i32 {
            19 as i32
        } else if 237 as i32 == 'U' as i32 {
            20 as i32
        } else if 237 as i32 == 'V' as i32 {
            21 as i32
        } else if 237 as i32 == 'W' as i32 {
            22 as i32
        } else if 237 as i32 == 'X' as i32 {
            23 as i32
        } else if 237 as i32 == 'Y' as i32 {
            24 as i32
        } else if 237 as i32 == 'Z' as i32 {
            25 as i32
        } else if 237 as i32 == '2' as i32 {
            26 as i32
        } else if 237 as i32 == '3' as i32 {
            27 as i32
        } else if 237 as i32 == '4' as i32 {
            28 as i32
        } else if 237 as i32 == '5' as i32 {
            29 as i32
        } else if 237 as i32 == '6' as i32 {
            30 as i32
        } else if 237 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 238 as i32 == 'A' as i32 {
            0 as i32
        } else if 238 as i32 == 'B' as i32 {
            1 as i32
        } else if 238 as i32 == 'C' as i32 {
            2 as i32
        } else if 238 as i32 == 'D' as i32 {
            3 as i32
        } else if 238 as i32 == 'E' as i32 {
            4 as i32
        } else if 238 as i32 == 'F' as i32 {
            5 as i32
        } else if 238 as i32 == 'G' as i32 {
            6 as i32
        } else if 238 as i32 == 'H' as i32 {
            7 as i32
        } else if 238 as i32 == 'I' as i32 {
            8 as i32
        } else if 238 as i32 == 'J' as i32 {
            9 as i32
        } else if 238 as i32 == 'K' as i32 {
            10 as i32
        } else if 238 as i32 == 'L' as i32 {
            11 as i32
        } else if 238 as i32 == 'M' as i32 {
            12 as i32
        } else if 238 as i32 == 'N' as i32 {
            13 as i32
        } else if 238 as i32 == 'O' as i32 {
            14 as i32
        } else if 238 as i32 == 'P' as i32 {
            15 as i32
        } else if 238 as i32 == 'Q' as i32 {
            16 as i32
        } else if 238 as i32 == 'R' as i32 {
            17 as i32
        } else if 238 as i32 == 'S' as i32 {
            18 as i32
        } else if 238 as i32 == 'T' as i32 {
            19 as i32
        } else if 238 as i32 == 'U' as i32 {
            20 as i32
        } else if 238 as i32 == 'V' as i32 {
            21 as i32
        } else if 238 as i32 == 'W' as i32 {
            22 as i32
        } else if 238 as i32 == 'X' as i32 {
            23 as i32
        } else if 238 as i32 == 'Y' as i32 {
            24 as i32
        } else if 238 as i32 == 'Z' as i32 {
            25 as i32
        } else if 238 as i32 == '2' as i32 {
            26 as i32
        } else if 238 as i32 == '3' as i32 {
            27 as i32
        } else if 238 as i32 == '4' as i32 {
            28 as i32
        } else if 238 as i32 == '5' as i32 {
            29 as i32
        } else if 238 as i32 == '6' as i32 {
            30 as i32
        } else if 238 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 239 as i32 == 'A' as i32 {
            0 as i32
        } else if 239 as i32 == 'B' as i32 {
            1 as i32
        } else if 239 as i32 == 'C' as i32 {
            2 as i32
        } else if 239 as i32 == 'D' as i32 {
            3 as i32
        } else if 239 as i32 == 'E' as i32 {
            4 as i32
        } else if 239 as i32 == 'F' as i32 {
            5 as i32
        } else if 239 as i32 == 'G' as i32 {
            6 as i32
        } else if 239 as i32 == 'H' as i32 {
            7 as i32
        } else if 239 as i32 == 'I' as i32 {
            8 as i32
        } else if 239 as i32 == 'J' as i32 {
            9 as i32
        } else if 239 as i32 == 'K' as i32 {
            10 as i32
        } else if 239 as i32 == 'L' as i32 {
            11 as i32
        } else if 239 as i32 == 'M' as i32 {
            12 as i32
        } else if 239 as i32 == 'N' as i32 {
            13 as i32
        } else if 239 as i32 == 'O' as i32 {
            14 as i32
        } else if 239 as i32 == 'P' as i32 {
            15 as i32
        } else if 239 as i32 == 'Q' as i32 {
            16 as i32
        } else if 239 as i32 == 'R' as i32 {
            17 as i32
        } else if 239 as i32 == 'S' as i32 {
            18 as i32
        } else if 239 as i32 == 'T' as i32 {
            19 as i32
        } else if 239 as i32 == 'U' as i32 {
            20 as i32
        } else if 239 as i32 == 'V' as i32 {
            21 as i32
        } else if 239 as i32 == 'W' as i32 {
            22 as i32
        } else if 239 as i32 == 'X' as i32 {
            23 as i32
        } else if 239 as i32 == 'Y' as i32 {
            24 as i32
        } else if 239 as i32 == 'Z' as i32 {
            25 as i32
        } else if 239 as i32 == '2' as i32 {
            26 as i32
        } else if 239 as i32 == '3' as i32 {
            27 as i32
        } else if 239 as i32 == '4' as i32 {
            28 as i32
        } else if 239 as i32 == '5' as i32 {
            29 as i32
        } else if 239 as i32 == '6' as i32 {
            30 as i32
        } else if 239 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 240 as i32 == 'A' as i32 {
            0 as i32
        } else if 240 as i32 == 'B' as i32 {
            1 as i32
        } else if 240 as i32 == 'C' as i32 {
            2 as i32
        } else if 240 as i32 == 'D' as i32 {
            3 as i32
        } else if 240 as i32 == 'E' as i32 {
            4 as i32
        } else if 240 as i32 == 'F' as i32 {
            5 as i32
        } else if 240 as i32 == 'G' as i32 {
            6 as i32
        } else if 240 as i32 == 'H' as i32 {
            7 as i32
        } else if 240 as i32 == 'I' as i32 {
            8 as i32
        } else if 240 as i32 == 'J' as i32 {
            9 as i32
        } else if 240 as i32 == 'K' as i32 {
            10 as i32
        } else if 240 as i32 == 'L' as i32 {
            11 as i32
        } else if 240 as i32 == 'M' as i32 {
            12 as i32
        } else if 240 as i32 == 'N' as i32 {
            13 as i32
        } else if 240 as i32 == 'O' as i32 {
            14 as i32
        } else if 240 as i32 == 'P' as i32 {
            15 as i32
        } else if 240 as i32 == 'Q' as i32 {
            16 as i32
        } else if 240 as i32 == 'R' as i32 {
            17 as i32
        } else if 240 as i32 == 'S' as i32 {
            18 as i32
        } else if 240 as i32 == 'T' as i32 {
            19 as i32
        } else if 240 as i32 == 'U' as i32 {
            20 as i32
        } else if 240 as i32 == 'V' as i32 {
            21 as i32
        } else if 240 as i32 == 'W' as i32 {
            22 as i32
        } else if 240 as i32 == 'X' as i32 {
            23 as i32
        } else if 240 as i32 == 'Y' as i32 {
            24 as i32
        } else if 240 as i32 == 'Z' as i32 {
            25 as i32
        } else if 240 as i32 == '2' as i32 {
            26 as i32
        } else if 240 as i32 == '3' as i32 {
            27 as i32
        } else if 240 as i32 == '4' as i32 {
            28 as i32
        } else if 240 as i32 == '5' as i32 {
            29 as i32
        } else if 240 as i32 == '6' as i32 {
            30 as i32
        } else if 240 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 241 as i32 == 'A' as i32 {
            0 as i32
        } else if 241 as i32 == 'B' as i32 {
            1 as i32
        } else if 241 as i32 == 'C' as i32 {
            2 as i32
        } else if 241 as i32 == 'D' as i32 {
            3 as i32
        } else if 241 as i32 == 'E' as i32 {
            4 as i32
        } else if 241 as i32 == 'F' as i32 {
            5 as i32
        } else if 241 as i32 == 'G' as i32 {
            6 as i32
        } else if 241 as i32 == 'H' as i32 {
            7 as i32
        } else if 241 as i32 == 'I' as i32 {
            8 as i32
        } else if 241 as i32 == 'J' as i32 {
            9 as i32
        } else if 241 as i32 == 'K' as i32 {
            10 as i32
        } else if 241 as i32 == 'L' as i32 {
            11 as i32
        } else if 241 as i32 == 'M' as i32 {
            12 as i32
        } else if 241 as i32 == 'N' as i32 {
            13 as i32
        } else if 241 as i32 == 'O' as i32 {
            14 as i32
        } else if 241 as i32 == 'P' as i32 {
            15 as i32
        } else if 241 as i32 == 'Q' as i32 {
            16 as i32
        } else if 241 as i32 == 'R' as i32 {
            17 as i32
        } else if 241 as i32 == 'S' as i32 {
            18 as i32
        } else if 241 as i32 == 'T' as i32 {
            19 as i32
        } else if 241 as i32 == 'U' as i32 {
            20 as i32
        } else if 241 as i32 == 'V' as i32 {
            21 as i32
        } else if 241 as i32 == 'W' as i32 {
            22 as i32
        } else if 241 as i32 == 'X' as i32 {
            23 as i32
        } else if 241 as i32 == 'Y' as i32 {
            24 as i32
        } else if 241 as i32 == 'Z' as i32 {
            25 as i32
        } else if 241 as i32 == '2' as i32 {
            26 as i32
        } else if 241 as i32 == '3' as i32 {
            27 as i32
        } else if 241 as i32 == '4' as i32 {
            28 as i32
        } else if 241 as i32 == '5' as i32 {
            29 as i32
        } else if 241 as i32 == '6' as i32 {
            30 as i32
        } else if 241 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 242 as i32 == 'A' as i32 {
            0 as i32
        } else if 242 as i32 == 'B' as i32 {
            1 as i32
        } else if 242 as i32 == 'C' as i32 {
            2 as i32
        } else if 242 as i32 == 'D' as i32 {
            3 as i32
        } else if 242 as i32 == 'E' as i32 {
            4 as i32
        } else if 242 as i32 == 'F' as i32 {
            5 as i32
        } else if 242 as i32 == 'G' as i32 {
            6 as i32
        } else if 242 as i32 == 'H' as i32 {
            7 as i32
        } else if 242 as i32 == 'I' as i32 {
            8 as i32
        } else if 242 as i32 == 'J' as i32 {
            9 as i32
        } else if 242 as i32 == 'K' as i32 {
            10 as i32
        } else if 242 as i32 == 'L' as i32 {
            11 as i32
        } else if 242 as i32 == 'M' as i32 {
            12 as i32
        } else if 242 as i32 == 'N' as i32 {
            13 as i32
        } else if 242 as i32 == 'O' as i32 {
            14 as i32
        } else if 242 as i32 == 'P' as i32 {
            15 as i32
        } else if 242 as i32 == 'Q' as i32 {
            16 as i32
        } else if 242 as i32 == 'R' as i32 {
            17 as i32
        } else if 242 as i32 == 'S' as i32 {
            18 as i32
        } else if 242 as i32 == 'T' as i32 {
            19 as i32
        } else if 242 as i32 == 'U' as i32 {
            20 as i32
        } else if 242 as i32 == 'V' as i32 {
            21 as i32
        } else if 242 as i32 == 'W' as i32 {
            22 as i32
        } else if 242 as i32 == 'X' as i32 {
            23 as i32
        } else if 242 as i32 == 'Y' as i32 {
            24 as i32
        } else if 242 as i32 == 'Z' as i32 {
            25 as i32
        } else if 242 as i32 == '2' as i32 {
            26 as i32
        } else if 242 as i32 == '3' as i32 {
            27 as i32
        } else if 242 as i32 == '4' as i32 {
            28 as i32
        } else if 242 as i32 == '5' as i32 {
            29 as i32
        } else if 242 as i32 == '6' as i32 {
            30 as i32
        } else if 242 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 243 as i32 == 'A' as i32 {
            0 as i32
        } else if 243 as i32 == 'B' as i32 {
            1 as i32
        } else if 243 as i32 == 'C' as i32 {
            2 as i32
        } else if 243 as i32 == 'D' as i32 {
            3 as i32
        } else if 243 as i32 == 'E' as i32 {
            4 as i32
        } else if 243 as i32 == 'F' as i32 {
            5 as i32
        } else if 243 as i32 == 'G' as i32 {
            6 as i32
        } else if 243 as i32 == 'H' as i32 {
            7 as i32
        } else if 243 as i32 == 'I' as i32 {
            8 as i32
        } else if 243 as i32 == 'J' as i32 {
            9 as i32
        } else if 243 as i32 == 'K' as i32 {
            10 as i32
        } else if 243 as i32 == 'L' as i32 {
            11 as i32
        } else if 243 as i32 == 'M' as i32 {
            12 as i32
        } else if 243 as i32 == 'N' as i32 {
            13 as i32
        } else if 243 as i32 == 'O' as i32 {
            14 as i32
        } else if 243 as i32 == 'P' as i32 {
            15 as i32
        } else if 243 as i32 == 'Q' as i32 {
            16 as i32
        } else if 243 as i32 == 'R' as i32 {
            17 as i32
        } else if 243 as i32 == 'S' as i32 {
            18 as i32
        } else if 243 as i32 == 'T' as i32 {
            19 as i32
        } else if 243 as i32 == 'U' as i32 {
            20 as i32
        } else if 243 as i32 == 'V' as i32 {
            21 as i32
        } else if 243 as i32 == 'W' as i32 {
            22 as i32
        } else if 243 as i32 == 'X' as i32 {
            23 as i32
        } else if 243 as i32 == 'Y' as i32 {
            24 as i32
        } else if 243 as i32 == 'Z' as i32 {
            25 as i32
        } else if 243 as i32 == '2' as i32 {
            26 as i32
        } else if 243 as i32 == '3' as i32 {
            27 as i32
        } else if 243 as i32 == '4' as i32 {
            28 as i32
        } else if 243 as i32 == '5' as i32 {
            29 as i32
        } else if 243 as i32 == '6' as i32 {
            30 as i32
        } else if 243 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 244 as i32 == 'A' as i32 {
            0 as i32
        } else if 244 as i32 == 'B' as i32 {
            1 as i32
        } else if 244 as i32 == 'C' as i32 {
            2 as i32
        } else if 244 as i32 == 'D' as i32 {
            3 as i32
        } else if 244 as i32 == 'E' as i32 {
            4 as i32
        } else if 244 as i32 == 'F' as i32 {
            5 as i32
        } else if 244 as i32 == 'G' as i32 {
            6 as i32
        } else if 244 as i32 == 'H' as i32 {
            7 as i32
        } else if 244 as i32 == 'I' as i32 {
            8 as i32
        } else if 244 as i32 == 'J' as i32 {
            9 as i32
        } else if 244 as i32 == 'K' as i32 {
            10 as i32
        } else if 244 as i32 == 'L' as i32 {
            11 as i32
        } else if 244 as i32 == 'M' as i32 {
            12 as i32
        } else if 244 as i32 == 'N' as i32 {
            13 as i32
        } else if 244 as i32 == 'O' as i32 {
            14 as i32
        } else if 244 as i32 == 'P' as i32 {
            15 as i32
        } else if 244 as i32 == 'Q' as i32 {
            16 as i32
        } else if 244 as i32 == 'R' as i32 {
            17 as i32
        } else if 244 as i32 == 'S' as i32 {
            18 as i32
        } else if 244 as i32 == 'T' as i32 {
            19 as i32
        } else if 244 as i32 == 'U' as i32 {
            20 as i32
        } else if 244 as i32 == 'V' as i32 {
            21 as i32
        } else if 244 as i32 == 'W' as i32 {
            22 as i32
        } else if 244 as i32 == 'X' as i32 {
            23 as i32
        } else if 244 as i32 == 'Y' as i32 {
            24 as i32
        } else if 244 as i32 == 'Z' as i32 {
            25 as i32
        } else if 244 as i32 == '2' as i32 {
            26 as i32
        } else if 244 as i32 == '3' as i32 {
            27 as i32
        } else if 244 as i32 == '4' as i32 {
            28 as i32
        } else if 244 as i32 == '5' as i32 {
            29 as i32
        } else if 244 as i32 == '6' as i32 {
            30 as i32
        } else if 244 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 245 as i32 == 'A' as i32 {
            0 as i32
        } else if 245 as i32 == 'B' as i32 {
            1 as i32
        } else if 245 as i32 == 'C' as i32 {
            2 as i32
        } else if 245 as i32 == 'D' as i32 {
            3 as i32
        } else if 245 as i32 == 'E' as i32 {
            4 as i32
        } else if 245 as i32 == 'F' as i32 {
            5 as i32
        } else if 245 as i32 == 'G' as i32 {
            6 as i32
        } else if 245 as i32 == 'H' as i32 {
            7 as i32
        } else if 245 as i32 == 'I' as i32 {
            8 as i32
        } else if 245 as i32 == 'J' as i32 {
            9 as i32
        } else if 245 as i32 == 'K' as i32 {
            10 as i32
        } else if 245 as i32 == 'L' as i32 {
            11 as i32
        } else if 245 as i32 == 'M' as i32 {
            12 as i32
        } else if 245 as i32 == 'N' as i32 {
            13 as i32
        } else if 245 as i32 == 'O' as i32 {
            14 as i32
        } else if 245 as i32 == 'P' as i32 {
            15 as i32
        } else if 245 as i32 == 'Q' as i32 {
            16 as i32
        } else if 245 as i32 == 'R' as i32 {
            17 as i32
        } else if 245 as i32 == 'S' as i32 {
            18 as i32
        } else if 245 as i32 == 'T' as i32 {
            19 as i32
        } else if 245 as i32 == 'U' as i32 {
            20 as i32
        } else if 245 as i32 == 'V' as i32 {
            21 as i32
        } else if 245 as i32 == 'W' as i32 {
            22 as i32
        } else if 245 as i32 == 'X' as i32 {
            23 as i32
        } else if 245 as i32 == 'Y' as i32 {
            24 as i32
        } else if 245 as i32 == 'Z' as i32 {
            25 as i32
        } else if 245 as i32 == '2' as i32 {
            26 as i32
        } else if 245 as i32 == '3' as i32 {
            27 as i32
        } else if 245 as i32 == '4' as i32 {
            28 as i32
        } else if 245 as i32 == '5' as i32 {
            29 as i32
        } else if 245 as i32 == '6' as i32 {
            30 as i32
        } else if 245 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 246 as i32 == 'A' as i32 {
            0 as i32
        } else if 246 as i32 == 'B' as i32 {
            1 as i32
        } else if 246 as i32 == 'C' as i32 {
            2 as i32
        } else if 246 as i32 == 'D' as i32 {
            3 as i32
        } else if 246 as i32 == 'E' as i32 {
            4 as i32
        } else if 246 as i32 == 'F' as i32 {
            5 as i32
        } else if 246 as i32 == 'G' as i32 {
            6 as i32
        } else if 246 as i32 == 'H' as i32 {
            7 as i32
        } else if 246 as i32 == 'I' as i32 {
            8 as i32
        } else if 246 as i32 == 'J' as i32 {
            9 as i32
        } else if 246 as i32 == 'K' as i32 {
            10 as i32
        } else if 246 as i32 == 'L' as i32 {
            11 as i32
        } else if 246 as i32 == 'M' as i32 {
            12 as i32
        } else if 246 as i32 == 'N' as i32 {
            13 as i32
        } else if 246 as i32 == 'O' as i32 {
            14 as i32
        } else if 246 as i32 == 'P' as i32 {
            15 as i32
        } else if 246 as i32 == 'Q' as i32 {
            16 as i32
        } else if 246 as i32 == 'R' as i32 {
            17 as i32
        } else if 246 as i32 == 'S' as i32 {
            18 as i32
        } else if 246 as i32 == 'T' as i32 {
            19 as i32
        } else if 246 as i32 == 'U' as i32 {
            20 as i32
        } else if 246 as i32 == 'V' as i32 {
            21 as i32
        } else if 246 as i32 == 'W' as i32 {
            22 as i32
        } else if 246 as i32 == 'X' as i32 {
            23 as i32
        } else if 246 as i32 == 'Y' as i32 {
            24 as i32
        } else if 246 as i32 == 'Z' as i32 {
            25 as i32
        } else if 246 as i32 == '2' as i32 {
            26 as i32
        } else if 246 as i32 == '3' as i32 {
            27 as i32
        } else if 246 as i32 == '4' as i32 {
            28 as i32
        } else if 246 as i32 == '5' as i32 {
            29 as i32
        } else if 246 as i32 == '6' as i32 {
            30 as i32
        } else if 246 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 247 as i32 == 'A' as i32 {
            0 as i32
        } else if 247 as i32 == 'B' as i32 {
            1 as i32
        } else if 247 as i32 == 'C' as i32 {
            2 as i32
        } else if 247 as i32 == 'D' as i32 {
            3 as i32
        } else if 247 as i32 == 'E' as i32 {
            4 as i32
        } else if 247 as i32 == 'F' as i32 {
            5 as i32
        } else if 247 as i32 == 'G' as i32 {
            6 as i32
        } else if 247 as i32 == 'H' as i32 {
            7 as i32
        } else if 247 as i32 == 'I' as i32 {
            8 as i32
        } else if 247 as i32 == 'J' as i32 {
            9 as i32
        } else if 247 as i32 == 'K' as i32 {
            10 as i32
        } else if 247 as i32 == 'L' as i32 {
            11 as i32
        } else if 247 as i32 == 'M' as i32 {
            12 as i32
        } else if 247 as i32 == 'N' as i32 {
            13 as i32
        } else if 247 as i32 == 'O' as i32 {
            14 as i32
        } else if 247 as i32 == 'P' as i32 {
            15 as i32
        } else if 247 as i32 == 'Q' as i32 {
            16 as i32
        } else if 247 as i32 == 'R' as i32 {
            17 as i32
        } else if 247 as i32 == 'S' as i32 {
            18 as i32
        } else if 247 as i32 == 'T' as i32 {
            19 as i32
        } else if 247 as i32 == 'U' as i32 {
            20 as i32
        } else if 247 as i32 == 'V' as i32 {
            21 as i32
        } else if 247 as i32 == 'W' as i32 {
            22 as i32
        } else if 247 as i32 == 'X' as i32 {
            23 as i32
        } else if 247 as i32 == 'Y' as i32 {
            24 as i32
        } else if 247 as i32 == 'Z' as i32 {
            25 as i32
        } else if 247 as i32 == '2' as i32 {
            26 as i32
        } else if 247 as i32 == '3' as i32 {
            27 as i32
        } else if 247 as i32 == '4' as i32 {
            28 as i32
        } else if 247 as i32 == '5' as i32 {
            29 as i32
        } else if 247 as i32 == '6' as i32 {
            30 as i32
        } else if 247 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 248 as i32 == 'A' as i32 {
            0 as i32
        } else if 248 as i32 == 'B' as i32 {
            1 as i32
        } else if 248 as i32 == 'C' as i32 {
            2 as i32
        } else if 248 as i32 == 'D' as i32 {
            3 as i32
        } else if 248 as i32 == 'E' as i32 {
            4 as i32
        } else if 248 as i32 == 'F' as i32 {
            5 as i32
        } else if 248 as i32 == 'G' as i32 {
            6 as i32
        } else if 248 as i32 == 'H' as i32 {
            7 as i32
        } else if 248 as i32 == 'I' as i32 {
            8 as i32
        } else if 248 as i32 == 'J' as i32 {
            9 as i32
        } else if 248 as i32 == 'K' as i32 {
            10 as i32
        } else if 248 as i32 == 'L' as i32 {
            11 as i32
        } else if 248 as i32 == 'M' as i32 {
            12 as i32
        } else if 248 as i32 == 'N' as i32 {
            13 as i32
        } else if 248 as i32 == 'O' as i32 {
            14 as i32
        } else if 248 as i32 == 'P' as i32 {
            15 as i32
        } else if 248 as i32 == 'Q' as i32 {
            16 as i32
        } else if 248 as i32 == 'R' as i32 {
            17 as i32
        } else if 248 as i32 == 'S' as i32 {
            18 as i32
        } else if 248 as i32 == 'T' as i32 {
            19 as i32
        } else if 248 as i32 == 'U' as i32 {
            20 as i32
        } else if 248 as i32 == 'V' as i32 {
            21 as i32
        } else if 248 as i32 == 'W' as i32 {
            22 as i32
        } else if 248 as i32 == 'X' as i32 {
            23 as i32
        } else if 248 as i32 == 'Y' as i32 {
            24 as i32
        } else if 248 as i32 == 'Z' as i32 {
            25 as i32
        } else if 248 as i32 == '2' as i32 {
            26 as i32
        } else if 248 as i32 == '3' as i32 {
            27 as i32
        } else if 248 as i32 == '4' as i32 {
            28 as i32
        } else if 248 as i32 == '5' as i32 {
            29 as i32
        } else if 248 as i32 == '6' as i32 {
            30 as i32
        } else if 248 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 249 as i32 == 'A' as i32 {
            0 as i32
        } else if 249 as i32 == 'B' as i32 {
            1 as i32
        } else if 249 as i32 == 'C' as i32 {
            2 as i32
        } else if 249 as i32 == 'D' as i32 {
            3 as i32
        } else if 249 as i32 == 'E' as i32 {
            4 as i32
        } else if 249 as i32 == 'F' as i32 {
            5 as i32
        } else if 249 as i32 == 'G' as i32 {
            6 as i32
        } else if 249 as i32 == 'H' as i32 {
            7 as i32
        } else if 249 as i32 == 'I' as i32 {
            8 as i32
        } else if 249 as i32 == 'J' as i32 {
            9 as i32
        } else if 249 as i32 == 'K' as i32 {
            10 as i32
        } else if 249 as i32 == 'L' as i32 {
            11 as i32
        } else if 249 as i32 == 'M' as i32 {
            12 as i32
        } else if 249 as i32 == 'N' as i32 {
            13 as i32
        } else if 249 as i32 == 'O' as i32 {
            14 as i32
        } else if 249 as i32 == 'P' as i32 {
            15 as i32
        } else if 249 as i32 == 'Q' as i32 {
            16 as i32
        } else if 249 as i32 == 'R' as i32 {
            17 as i32
        } else if 249 as i32 == 'S' as i32 {
            18 as i32
        } else if 249 as i32 == 'T' as i32 {
            19 as i32
        } else if 249 as i32 == 'U' as i32 {
            20 as i32
        } else if 249 as i32 == 'V' as i32 {
            21 as i32
        } else if 249 as i32 == 'W' as i32 {
            22 as i32
        } else if 249 as i32 == 'X' as i32 {
            23 as i32
        } else if 249 as i32 == 'Y' as i32 {
            24 as i32
        } else if 249 as i32 == 'Z' as i32 {
            25 as i32
        } else if 249 as i32 == '2' as i32 {
            26 as i32
        } else if 249 as i32 == '3' as i32 {
            27 as i32
        } else if 249 as i32 == '4' as i32 {
            28 as i32
        } else if 249 as i32 == '5' as i32 {
            29 as i32
        } else if 249 as i32 == '6' as i32 {
            30 as i32
        } else if 249 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 250 as i32 == 'A' as i32 {
            0 as i32
        } else if 250 as i32 == 'B' as i32 {
            1 as i32
        } else if 250 as i32 == 'C' as i32 {
            2 as i32
        } else if 250 as i32 == 'D' as i32 {
            3 as i32
        } else if 250 as i32 == 'E' as i32 {
            4 as i32
        } else if 250 as i32 == 'F' as i32 {
            5 as i32
        } else if 250 as i32 == 'G' as i32 {
            6 as i32
        } else if 250 as i32 == 'H' as i32 {
            7 as i32
        } else if 250 as i32 == 'I' as i32 {
            8 as i32
        } else if 250 as i32 == 'J' as i32 {
            9 as i32
        } else if 250 as i32 == 'K' as i32 {
            10 as i32
        } else if 250 as i32 == 'L' as i32 {
            11 as i32
        } else if 250 as i32 == 'M' as i32 {
            12 as i32
        } else if 250 as i32 == 'N' as i32 {
            13 as i32
        } else if 250 as i32 == 'O' as i32 {
            14 as i32
        } else if 250 as i32 == 'P' as i32 {
            15 as i32
        } else if 250 as i32 == 'Q' as i32 {
            16 as i32
        } else if 250 as i32 == 'R' as i32 {
            17 as i32
        } else if 250 as i32 == 'S' as i32 {
            18 as i32
        } else if 250 as i32 == 'T' as i32 {
            19 as i32
        } else if 250 as i32 == 'U' as i32 {
            20 as i32
        } else if 250 as i32 == 'V' as i32 {
            21 as i32
        } else if 250 as i32 == 'W' as i32 {
            22 as i32
        } else if 250 as i32 == 'X' as i32 {
            23 as i32
        } else if 250 as i32 == 'Y' as i32 {
            24 as i32
        } else if 250 as i32 == 'Z' as i32 {
            25 as i32
        } else if 250 as i32 == '2' as i32 {
            26 as i32
        } else if 250 as i32 == '3' as i32 {
            27 as i32
        } else if 250 as i32 == '4' as i32 {
            28 as i32
        } else if 250 as i32 == '5' as i32 {
            29 as i32
        } else if 250 as i32 == '6' as i32 {
            30 as i32
        } else if 250 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 251 as i32 == 'A' as i32 {
            0 as i32
        } else if 251 as i32 == 'B' as i32 {
            1 as i32
        } else if 251 as i32 == 'C' as i32 {
            2 as i32
        } else if 251 as i32 == 'D' as i32 {
            3 as i32
        } else if 251 as i32 == 'E' as i32 {
            4 as i32
        } else if 251 as i32 == 'F' as i32 {
            5 as i32
        } else if 251 as i32 == 'G' as i32 {
            6 as i32
        } else if 251 as i32 == 'H' as i32 {
            7 as i32
        } else if 251 as i32 == 'I' as i32 {
            8 as i32
        } else if 251 as i32 == 'J' as i32 {
            9 as i32
        } else if 251 as i32 == 'K' as i32 {
            10 as i32
        } else if 251 as i32 == 'L' as i32 {
            11 as i32
        } else if 251 as i32 == 'M' as i32 {
            12 as i32
        } else if 251 as i32 == 'N' as i32 {
            13 as i32
        } else if 251 as i32 == 'O' as i32 {
            14 as i32
        } else if 251 as i32 == 'P' as i32 {
            15 as i32
        } else if 251 as i32 == 'Q' as i32 {
            16 as i32
        } else if 251 as i32 == 'R' as i32 {
            17 as i32
        } else if 251 as i32 == 'S' as i32 {
            18 as i32
        } else if 251 as i32 == 'T' as i32 {
            19 as i32
        } else if 251 as i32 == 'U' as i32 {
            20 as i32
        } else if 251 as i32 == 'V' as i32 {
            21 as i32
        } else if 251 as i32 == 'W' as i32 {
            22 as i32
        } else if 251 as i32 == 'X' as i32 {
            23 as i32
        } else if 251 as i32 == 'Y' as i32 {
            24 as i32
        } else if 251 as i32 == 'Z' as i32 {
            25 as i32
        } else if 251 as i32 == '2' as i32 {
            26 as i32
        } else if 251 as i32 == '3' as i32 {
            27 as i32
        } else if 251 as i32 == '4' as i32 {
            28 as i32
        } else if 251 as i32 == '5' as i32 {
            29 as i32
        } else if 251 as i32 == '6' as i32 {
            30 as i32
        } else if 251 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 252 as i32 == 'A' as i32 {
            0 as i32
        } else if 252 as i32 == 'B' as i32 {
            1 as i32
        } else if 252 as i32 == 'C' as i32 {
            2 as i32
        } else if 252 as i32 == 'D' as i32 {
            3 as i32
        } else if 252 as i32 == 'E' as i32 {
            4 as i32
        } else if 252 as i32 == 'F' as i32 {
            5 as i32
        } else if 252 as i32 == 'G' as i32 {
            6 as i32
        } else if 252 as i32 == 'H' as i32 {
            7 as i32
        } else if 252 as i32 == 'I' as i32 {
            8 as i32
        } else if 252 as i32 == 'J' as i32 {
            9 as i32
        } else if 252 as i32 == 'K' as i32 {
            10 as i32
        } else if 252 as i32 == 'L' as i32 {
            11 as i32
        } else if 252 as i32 == 'M' as i32 {
            12 as i32
        } else if 252 as i32 == 'N' as i32 {
            13 as i32
        } else if 252 as i32 == 'O' as i32 {
            14 as i32
        } else if 252 as i32 == 'P' as i32 {
            15 as i32
        } else if 252 as i32 == 'Q' as i32 {
            16 as i32
        } else if 252 as i32 == 'R' as i32 {
            17 as i32
        } else if 252 as i32 == 'S' as i32 {
            18 as i32
        } else if 252 as i32 == 'T' as i32 {
            19 as i32
        } else if 252 as i32 == 'U' as i32 {
            20 as i32
        } else if 252 as i32 == 'V' as i32 {
            21 as i32
        } else if 252 as i32 == 'W' as i32 {
            22 as i32
        } else if 252 as i32 == 'X' as i32 {
            23 as i32
        } else if 252 as i32 == 'Y' as i32 {
            24 as i32
        } else if 252 as i32 == 'Z' as i32 {
            25 as i32
        } else if 252 as i32 == '2' as i32 {
            26 as i32
        } else if 252 as i32 == '3' as i32 {
            27 as i32
        } else if 252 as i32 == '4' as i32 {
            28 as i32
        } else if 252 as i32 == '5' as i32 {
            29 as i32
        } else if 252 as i32 == '6' as i32 {
            30 as i32
        } else if 252 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 253 as i32 == 'A' as i32 {
            0 as i32
        } else if 253 as i32 == 'B' as i32 {
            1 as i32
        } else if 253 as i32 == 'C' as i32 {
            2 as i32
        } else if 253 as i32 == 'D' as i32 {
            3 as i32
        } else if 253 as i32 == 'E' as i32 {
            4 as i32
        } else if 253 as i32 == 'F' as i32 {
            5 as i32
        } else if 253 as i32 == 'G' as i32 {
            6 as i32
        } else if 253 as i32 == 'H' as i32 {
            7 as i32
        } else if 253 as i32 == 'I' as i32 {
            8 as i32
        } else if 253 as i32 == 'J' as i32 {
            9 as i32
        } else if 253 as i32 == 'K' as i32 {
            10 as i32
        } else if 253 as i32 == 'L' as i32 {
            11 as i32
        } else if 253 as i32 == 'M' as i32 {
            12 as i32
        } else if 253 as i32 == 'N' as i32 {
            13 as i32
        } else if 253 as i32 == 'O' as i32 {
            14 as i32
        } else if 253 as i32 == 'P' as i32 {
            15 as i32
        } else if 253 as i32 == 'Q' as i32 {
            16 as i32
        } else if 253 as i32 == 'R' as i32 {
            17 as i32
        } else if 253 as i32 == 'S' as i32 {
            18 as i32
        } else if 253 as i32 == 'T' as i32 {
            19 as i32
        } else if 253 as i32 == 'U' as i32 {
            20 as i32
        } else if 253 as i32 == 'V' as i32 {
            21 as i32
        } else if 253 as i32 == 'W' as i32 {
            22 as i32
        } else if 253 as i32 == 'X' as i32 {
            23 as i32
        } else if 253 as i32 == 'Y' as i32 {
            24 as i32
        } else if 253 as i32 == 'Z' as i32 {
            25 as i32
        } else if 253 as i32 == '2' as i32 {
            26 as i32
        } else if 253 as i32 == '3' as i32 {
            27 as i32
        } else if 253 as i32 == '4' as i32 {
            28 as i32
        } else if 253 as i32 == '5' as i32 {
            29 as i32
        } else if 253 as i32 == '6' as i32 {
            30 as i32
        } else if 253 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 254 as i32 == 'A' as i32 {
            0 as i32
        } else if 254 as i32 == 'B' as i32 {
            1 as i32
        } else if 254 as i32 == 'C' as i32 {
            2 as i32
        } else if 254 as i32 == 'D' as i32 {
            3 as i32
        } else if 254 as i32 == 'E' as i32 {
            4 as i32
        } else if 254 as i32 == 'F' as i32 {
            5 as i32
        } else if 254 as i32 == 'G' as i32 {
            6 as i32
        } else if 254 as i32 == 'H' as i32 {
            7 as i32
        } else if 254 as i32 == 'I' as i32 {
            8 as i32
        } else if 254 as i32 == 'J' as i32 {
            9 as i32
        } else if 254 as i32 == 'K' as i32 {
            10 as i32
        } else if 254 as i32 == 'L' as i32 {
            11 as i32
        } else if 254 as i32 == 'M' as i32 {
            12 as i32
        } else if 254 as i32 == 'N' as i32 {
            13 as i32
        } else if 254 as i32 == 'O' as i32 {
            14 as i32
        } else if 254 as i32 == 'P' as i32 {
            15 as i32
        } else if 254 as i32 == 'Q' as i32 {
            16 as i32
        } else if 254 as i32 == 'R' as i32 {
            17 as i32
        } else if 254 as i32 == 'S' as i32 {
            18 as i32
        } else if 254 as i32 == 'T' as i32 {
            19 as i32
        } else if 254 as i32 == 'U' as i32 {
            20 as i32
        } else if 254 as i32 == 'V' as i32 {
            21 as i32
        } else if 254 as i32 == 'W' as i32 {
            22 as i32
        } else if 254 as i32 == 'X' as i32 {
            23 as i32
        } else if 254 as i32 == 'Y' as i32 {
            24 as i32
        } else if 254 as i32 == 'Z' as i32 {
            25 as i32
        } else if 254 as i32 == '2' as i32 {
            26 as i32
        } else if 254 as i32 == '3' as i32 {
            27 as i32
        } else if 254 as i32 == '4' as i32 {
            28 as i32
        } else if 254 as i32 == '5' as i32 {
            29 as i32
        } else if 254 as i32 == '6' as i32 {
            30 as i32
        } else if 254 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
        (if 255 as i32 == 'A' as i32 {
            0 as i32
        } else if 255 as i32 == 'B' as i32 {
            1 as i32
        } else if 255 as i32 == 'C' as i32 {
            2 as i32
        } else if 255 as i32 == 'D' as i32 {
            3 as i32
        } else if 255 as i32 == 'E' as i32 {
            4 as i32
        } else if 255 as i32 == 'F' as i32 {
            5 as i32
        } else if 255 as i32 == 'G' as i32 {
            6 as i32
        } else if 255 as i32 == 'H' as i32 {
            7 as i32
        } else if 255 as i32 == 'I' as i32 {
            8 as i32
        } else if 255 as i32 == 'J' as i32 {
            9 as i32
        } else if 255 as i32 == 'K' as i32 {
            10 as i32
        } else if 255 as i32 == 'L' as i32 {
            11 as i32
        } else if 255 as i32 == 'M' as i32 {
            12 as i32
        } else if 255 as i32 == 'N' as i32 {
            13 as i32
        } else if 255 as i32 == 'O' as i32 {
            14 as i32
        } else if 255 as i32 == 'P' as i32 {
            15 as i32
        } else if 255 as i32 == 'Q' as i32 {
            16 as i32
        } else if 255 as i32 == 'R' as i32 {
            17 as i32
        } else if 255 as i32 == 'S' as i32 {
            18 as i32
        } else if 255 as i32 == 'T' as i32 {
            19 as i32
        } else if 255 as i32 == 'U' as i32 {
            20 as i32
        } else if 255 as i32 == 'V' as i32 {
            21 as i32
        } else if 255 as i32 == 'W' as i32 {
            22 as i32
        } else if 255 as i32 == 'X' as i32 {
            23 as i32
        } else if 255 as i32 == 'Y' as i32 {
            24 as i32
        } else if 255 as i32 == 'Z' as i32 {
            25 as i32
        } else if 255 as i32 == '2' as i32 {
            26 as i32
        } else if 255 as i32 == '3' as i32 {
            27 as i32
        } else if 255 as i32 == '4' as i32 {
            28 as i32
        } else if 255 as i32 == '5' as i32 {
            29 as i32
        } else if 255 as i32 == '6' as i32 {
            30 as i32
        } else if 255 as i32 == '7' as i32 {
            31 as i32
        } else {
            -(1 as i32)
        }) as libc::c_schar,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];