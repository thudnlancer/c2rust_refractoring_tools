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
    fn __errno_location() -> *mut i32;
    fn malloc(_: u64) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn strerror(_: i32) -> *mut i8;
    fn _glp_zlib_read(fd: i32, buf: *mut libc::c_void, nbyte: u64) -> i64;
    fn _glp_zlib_close(fd: i32) -> i32;
    fn _glp_zlib_inflate(strm: z_streamp, flush: i32) -> i32;
    fn _glp_zlib_inflateEnd(strm: z_streamp) -> i32;
    fn _glp_zlib_inflateReset(strm: z_streamp) -> i32;
    fn _glp_zlib_gz_error(_: gz_statep, _: i32, _: *const i8);
    fn _glp_zlib_crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn _glp_zlib_inflateInit2_(
        strm: z_streamp,
        windowBits: i32,
        version: *const i8,
        stream_size: i32,
    ) -> i32;
}
pub type Byte = u8;
pub type uInt = u32;
pub type uLong = u64;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type voidp = *mut libc::c_void;
pub type alloc_func = Option<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct internal_state {
    pub dummy: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut i8,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: i32,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type z_streamp = *mut z_stream;
pub type gzFile = voidp;
pub type gz_statep = *mut gz_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_state {
    pub mode: i32,
    pub fd: i32,
    pub path: *mut i8,
    pub pos: i64,
    pub size: u32,
    pub want: u32,
    pub in_0: *mut u8,
    pub out: *mut u8,
    pub next: *mut u8,
    pub have: u32,
    pub eof: i32,
    pub start: i64,
    pub raw: i64,
    pub how: i32,
    pub direct: i32,
    pub level: i32,
    pub strategy: i32,
    pub skip: i64,
    pub seek: i32,
    pub err: i32,
    pub msg: *mut i8,
    pub strm: z_stream,
}
unsafe extern "C" fn gz_load(
    mut state: gz_statep,
    mut buf: *mut u8,
    mut len: u32,
    mut have: *mut u32,
) -> i32 {
    let mut ret: i32 = 0;
    *have = 0 as i32 as u32;
    loop {
        ret = _glp_zlib_read(
            (*state).fd,
            buf.offset(*have as isize) as *mut libc::c_void,
            len.wrapping_sub(*have) as u64,
        ) as i32;
        if ret <= 0 as i32 {
            break;
        }
        *have = (*have).wrapping_add(ret as u32);
        if !(*have < len) {
            break;
        }
    }
    if ret < 0 as i32 {
        _glp_zlib_gz_error(state, -(1 as i32), strerror(*__errno_location()));
        return -(1 as i32);
    }
    if ret == 0 as i32 {
        (*state).eof = 1 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn gz_avail(mut state: gz_statep) -> i32 {
    let mut strm: z_streamp = &mut (*state).strm;
    if (*state).err != 0 as i32 {
        return -(1 as i32);
    }
    if (*state).eof == 0 as i32 {
        if gz_load(
            state,
            (*state).in_0,
            (*state).size,
            &mut (*strm).avail_in as *mut uInt as *mut u32,
        ) == -(1 as i32)
        {
            return -(1 as i32);
        }
        (*strm).next_in = (*state).in_0;
    }
    return 0 as i32;
}
unsafe extern "C" fn gz_next4(mut state: gz_statep, mut ret: *mut u64) -> i32 {
    let mut ch: i32 = 0;
    let mut val: u64 = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    val = (if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32) {
        -(1 as i32)
    } else if (*strm).avail_in == 0 as i32 as u32 {
        -(1 as i32)
    } else {
        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
        (*strm).avail_in;
        let fresh0 = (*strm).next_in;
        (*strm).next_in = ((*strm).next_in).offset(1);
        *fresh0 as i32
    }) as u64;
    val = val
        .wrapping_add(
            (((if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32) {
                -(1 as i32)
            } else {
                (if (*strm).avail_in == 0 as i32 as u32 {
                    -(1 as i32)
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh1 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh1 as i32
                })
            }) as u32) << 8 as i32) as u64,
        );
    val = val
        .wrapping_add(
            ((if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32) {
                -(1 as i32)
            } else {
                (if (*strm).avail_in == 0 as i32 as u32 {
                    -(1 as i32)
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh2 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh2 as i32
                })
            }) as u64) << 16 as i32,
        );
    ch = if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32) {
        -(1 as i32)
    } else if (*strm).avail_in == 0 as i32 as u32 {
        -(1 as i32)
    } else {
        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
        (*strm).avail_in;
        let fresh3 = (*strm).next_in;
        (*strm).next_in = ((*strm).next_in).offset(1);
        *fresh3 as i32
    };
    if ch == -(1 as i32) {
        return -(1 as i32);
    }
    val = val.wrapping_add((ch as u64) << 24 as i32);
    *ret = val;
    return 0 as i32;
}
unsafe extern "C" fn gz_head(mut state: gz_statep) -> i32 {
    let mut strm: z_streamp = &mut (*state).strm;
    let mut flags: i32 = 0;
    let mut len: u32 = 0;
    if (*state).size == 0 as i32 as u32 {
        (*state).in_0 = malloc((*state).want as u64) as *mut u8;
        (*state).out = malloc(((*state).want << 1 as i32) as u64) as *mut u8;
        if ((*state).in_0).is_null() || ((*state).out).is_null() {
            if !((*state).out).is_null() {
                free((*state).out as *mut libc::c_void);
            }
            if !((*state).in_0).is_null() {
                free((*state).in_0 as *mut libc::c_void);
            }
            _glp_zlib_gz_error(
                state,
                -(4 as i32),
                b"out of memory\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        (*state).size = (*state).want;
        (*state).strm.zalloc = None;
        (*state).strm.zfree = None;
        (*state).strm.opaque = 0 as voidpf;
        (*state).strm.avail_in = 0 as i32 as uInt;
        (*state).strm.next_in = 0 as *mut Bytef;
        if _glp_zlib_inflateInit2_(
            &mut (*state).strm,
            -(15 as i32),
            b"1.2.5\0" as *const u8 as *const i8,
            ::core::mem::size_of::<z_stream>() as u64 as i32,
        ) != 0 as i32
        {
            free((*state).out as *mut libc::c_void);
            free((*state).in_0 as *mut libc::c_void);
            (*state).size = 0 as i32 as u32;
            _glp_zlib_gz_error(
                state,
                -(4 as i32),
                b"out of memory\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
    }
    if (*strm).avail_in == 0 as i32 as u32 {
        if gz_avail(state) == -(1 as i32) {
            return -(1 as i32);
        }
        if (*strm).avail_in == 0 as i32 as u32 {
            return 0 as i32;
        }
    }
    if *((*strm).next_in).offset(0 as i32 as isize) as i32 == 31 as i32 {
        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
        (*strm).avail_in;
        (*strm).next_in = ((*strm).next_in).offset(1);
        (*strm).next_in;
        if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32) {
            return -(1 as i32);
        }
        if (*strm).avail_in != 0
            && *((*strm).next_in).offset(0 as i32 as isize) as i32 == 139 as i32
        {
            (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
            (*strm).avail_in;
            (*strm).next_in = ((*strm).next_in).offset(1);
            (*strm).next_in;
            if (if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32)
            {
                -(1 as i32)
            } else {
                (if (*strm).avail_in == 0 as i32 as u32 {
                    -(1 as i32)
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh4 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh4 as i32
                })
            }) != 8 as i32
            {
                _glp_zlib_gz_error(
                    state,
                    -(3 as i32),
                    b"unknown compression method\0" as *const u8 as *const i8,
                );
                return -(1 as i32);
            }
            flags = if (*strm).avail_in == 0 as i32 as u32
                && gz_avail(state) == -(1 as i32)
            {
                -(1 as i32)
            } else if (*strm).avail_in == 0 as i32 as u32 {
                -(1 as i32)
            } else {
                (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                (*strm).avail_in;
                let fresh5 = (*strm).next_in;
                (*strm).next_in = ((*strm).next_in).offset(1);
                *fresh5 as i32
            };
            if flags & 0xe0 as i32 != 0 {
                _glp_zlib_gz_error(
                    state,
                    -(3 as i32),
                    b"unknown header flags set\0" as *const u8 as *const i8,
                );
                return -(1 as i32);
            }
            if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32) {
                -(1 as i32);
            } else {
                if (*strm).avail_in == 0 as i32 as u32 {
                    -(1 as i32);
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh6 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh6;
                };
            };
            if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32) {
                -(1 as i32);
            } else {
                if (*strm).avail_in == 0 as i32 as u32 {
                    -(1 as i32);
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh7 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh7;
                };
            };
            if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32) {
                -(1 as i32);
            } else {
                if (*strm).avail_in == 0 as i32 as u32 {
                    -(1 as i32);
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh8 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh8;
                };
            };
            if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32) {
                -(1 as i32);
            } else {
                if (*strm).avail_in == 0 as i32 as u32 {
                    -(1 as i32);
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh9 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh9;
                };
            };
            if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32) {
                -(1 as i32);
            } else {
                if (*strm).avail_in == 0 as i32 as u32 {
                    -(1 as i32);
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh10 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh10;
                };
            };
            if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32) {
                -(1 as i32);
            } else {
                if (*strm).avail_in == 0 as i32 as u32 {
                    -(1 as i32);
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh11 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh11;
                };
            };
            if flags & 4 as i32 != 0 {
                len = (if (*strm).avail_in == 0 as i32 as u32
                    && gz_avail(state) == -(1 as i32)
                {
                    -(1 as i32)
                } else if (*strm).avail_in == 0 as i32 as u32 {
                    -(1 as i32)
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh12 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh12 as i32
                }) as u32;
                len = len
                    .wrapping_add(
                        ((if (*strm).avail_in == 0 as i32 as u32
                            && gz_avail(state) == -(1 as i32)
                        {
                            -(1 as i32)
                        } else {
                            (if (*strm).avail_in == 0 as i32 as u32 {
                                -(1 as i32)
                            } else {
                                (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                                (*strm).avail_in;
                                let fresh13 = (*strm).next_in;
                                (*strm).next_in = ((*strm).next_in).offset(1);
                                *fresh13 as i32
                            })
                        }) as u32) << 8 as i32,
                    );
                loop {
                    let fresh14 = len;
                    len = len.wrapping_sub(1);
                    if !(fresh14 != 0) {
                        break;
                    }
                    if (if (*strm).avail_in == 0 as i32 as u32
                        && gz_avail(state) == -(1 as i32)
                    {
                        -(1 as i32)
                    } else {
                        (if (*strm).avail_in == 0 as i32 as u32 {
                            -(1 as i32)
                        } else {
                            (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                            (*strm).avail_in;
                            let fresh15 = (*strm).next_in;
                            (*strm).next_in = ((*strm).next_in).offset(1);
                            *fresh15 as i32
                        })
                    }) < 0 as i32
                    {
                        break;
                    }
                }
            }
            if flags & 8 as i32 != 0 {
                while (if (*strm).avail_in == 0 as i32 as u32
                    && gz_avail(state) == -(1 as i32)
                {
                    -(1 as i32)
                } else {
                    (if (*strm).avail_in == 0 as i32 as u32 {
                        -(1 as i32)
                    } else {
                        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                        (*strm).avail_in;
                        let fresh16 = (*strm).next_in;
                        (*strm).next_in = ((*strm).next_in).offset(1);
                        *fresh16 as i32
                    })
                }) > 0 as i32
                {}
            }
            if flags & 16 as i32 != 0 {
                while (if (*strm).avail_in == 0 as i32 as u32
                    && gz_avail(state) == -(1 as i32)
                {
                    -(1 as i32)
                } else {
                    (if (*strm).avail_in == 0 as i32 as u32 {
                        -(1 as i32)
                    } else {
                        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                        (*strm).avail_in;
                        let fresh17 = (*strm).next_in;
                        (*strm).next_in = ((*strm).next_in).offset(1);
                        *fresh17 as i32
                    })
                }) > 0 as i32
                {}
            }
            if flags & 2 as i32 != 0 {
                if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32)
                {
                    -(1 as i32);
                } else {
                    if (*strm).avail_in == 0 as i32 as u32 {
                        -(1 as i32);
                    } else {
                        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                        (*strm).avail_in;
                        let fresh18 = (*strm).next_in;
                        (*strm).next_in = ((*strm).next_in).offset(1);
                        *fresh18;
                    };
                };
                if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32)
                {
                    -(1 as i32);
                } else {
                    if (*strm).avail_in == 0 as i32 as u32 {
                        -(1 as i32);
                    } else {
                        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                        (*strm).avail_in;
                        let fresh19 = (*strm).next_in;
                        (*strm).next_in = ((*strm).next_in).offset(1);
                        *fresh19;
                    };
                };
            }
            _glp_zlib_inflateReset(strm);
            (*strm).adler = _glp_zlib_crc32(
                0 as i64 as uLong,
                0 as *const Bytef,
                0 as i32 as uInt,
            );
            (*state).how = 2 as i32;
            (*state).direct = 0 as i32;
            return 0 as i32;
        } else {
            *((*state).out).offset(0 as i32 as isize) = 31 as i32 as u8;
            (*state).have = 1 as i32 as u32;
        }
    }
    (*state).raw = (*state).pos;
    (*state).next = (*state).out;
    if (*strm).avail_in != 0 {
        memcpy(
            ((*state).next).offset((*state).have as isize) as *mut libc::c_void,
            (*strm).next_in as *const libc::c_void,
            (*strm).avail_in as u64,
        );
        (*state).have = ((*state).have).wrapping_add((*strm).avail_in);
        (*strm).avail_in = 0 as i32 as uInt;
    }
    (*state).how = 1 as i32;
    (*state).direct = 1 as i32;
    return 0 as i32;
}
unsafe extern "C" fn gz_decomp(mut state: gz_statep) -> i32 {
    let mut ret: i32 = 0;
    let mut had: u32 = 0;
    let mut crc: u64 = 0;
    let mut len: u64 = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    had = (*strm).avail_out;
    loop {
        if (*strm).avail_in == 0 as i32 as u32 && gz_avail(state) == -(1 as i32) {
            return -(1 as i32);
        }
        if (*strm).avail_in == 0 as i32 as u32 {
            _glp_zlib_gz_error(
                state,
                -(3 as i32),
                b"unexpected end of file\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        ret = _glp_zlib_inflate(strm, 0 as i32);
        if ret == -(2 as i32) || ret == 2 as i32 {
            _glp_zlib_gz_error(
                state,
                -(2 as i32),
                b"internal error: inflate stream corrupt\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        if ret == -(4 as i32) {
            _glp_zlib_gz_error(
                state,
                -(4 as i32),
                b"out of memory\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        if ret == -(3 as i32) {
            _glp_zlib_gz_error(
                state,
                -(3 as i32),
                if ((*strm).msg).is_null() {
                    b"compressed data error\0" as *const u8 as *const i8
                } else {
                    (*strm).msg
                },
            );
            return -(1 as i32);
        }
        if !((*strm).avail_out != 0 && ret != 1 as i32) {
            break;
        }
    }
    (*state).have = had.wrapping_sub((*strm).avail_out);
    (*state).next = ((*strm).next_out).offset(-((*state).have as isize));
    (*strm).adler = _glp_zlib_crc32((*strm).adler, (*state).next, (*state).have);
    if ret == 1 as i32 {
        if gz_next4(state, &mut crc) == -(1 as i32)
            || gz_next4(state, &mut len) == -(1 as i32)
        {
            _glp_zlib_gz_error(
                state,
                -(3 as i32),
                b"unexpected end of file\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        if crc != (*strm).adler {
            _glp_zlib_gz_error(
                state,
                -(3 as i32),
                b"incorrect data check\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        if len != (*strm).total_out & 0xffffffff as i64 as u64 {
            _glp_zlib_gz_error(
                state,
                -(3 as i32),
                b"incorrect length check\0" as *const u8 as *const i8,
            );
            return -(1 as i32);
        }
        (*state).how = 0 as i32;
    }
    return 0 as i32;
}
unsafe extern "C" fn gz_make(mut state: gz_statep) -> i32 {
    let mut strm: z_streamp = &mut (*state).strm;
    if (*state).how == 0 as i32 {
        if gz_head(state) == -(1 as i32) {
            return -(1 as i32);
        }
        if (*state).have != 0 {
            return 0 as i32;
        }
    }
    if (*state).how == 1 as i32 {
        if gz_load(state, (*state).out, (*state).size << 1 as i32, &mut (*state).have)
            == -(1 as i32)
        {
            return -(1 as i32);
        }
        (*state).next = (*state).out;
    } else if (*state).how == 2 as i32 {
        (*strm).avail_out = (*state).size << 1 as i32;
        (*strm).next_out = (*state).out;
        if gz_decomp(state) == -(1 as i32) {
            return -(1 as i32);
        }
    }
    return 0 as i32;
}
unsafe extern "C" fn gz_skip(mut state: gz_statep, mut len: i64) -> i32 {
    let mut n: u32 = 0;
    while len != 0 {
        if (*state).have != 0 {
            n = if ::core::mem::size_of::<i32>() as u64
                == ::core::mem::size_of::<i64>() as u64
                && (*state).have > 2147483647 as i32 as u32 || (*state).have as i64 > len
            {
                len as u32
            } else {
                (*state).have
            };
            (*state).have = ((*state).have).wrapping_sub(n);
            (*state).next = ((*state).next).offset(n as isize);
            (*state).pos += n as i64;
            len -= n as i64;
        } else {
            if (*state).eof != 0 && (*state).strm.avail_in == 0 as i32 as u32 {
                break;
            }
            if gz_make(state) == -(1 as i32) {
                return -(1 as i32);
            }
        }
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzread(
    mut file: gzFile,
    mut buf: voidp,
    mut len: u32,
) -> i32 {
    let mut got: u32 = 0;
    let mut n: u32 = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    if file.is_null() {
        return -(1 as i32);
    }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    if (*state).mode != 7247 as i32 || (*state).err != 0 as i32 {
        return -(1 as i32);
    }
    if (len as i32) < 0 as i32 {
        _glp_zlib_gz_error(
            state,
            -(5 as i32),
            b"requested length does not fit in int\0" as *const u8 as *const i8,
        );
        return -(1 as i32);
    }
    if len == 0 as i32 as u32 {
        return 0 as i32;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as i32;
        if gz_skip(state, (*state).skip) == -(1 as i32) {
            return -(1 as i32);
        }
    }
    got = 0 as i32 as u32;
    let mut current_block_39: u64;
    loop {
        if (*state).have != 0 {
            n = if (*state).have > len { len } else { (*state).have };
            memcpy(buf, (*state).next as *const libc::c_void, n as u64);
            (*state).next = ((*state).next).offset(n as isize);
            (*state).have = ((*state).have).wrapping_sub(n);
            current_block_39 = 8693738493027456495;
        } else {
            if (*state).eof != 0 && (*strm).avail_in == 0 as i32 as u32 {
                break;
            }
            if (*state).how == 0 as i32 || len < (*state).size << 1 as i32 {
                if gz_make(state) == -(1 as i32) {
                    return -(1 as i32);
                }
                current_block_39 = 4166486009154926805;
            } else {
                if (*state).how == 1 as i32 {
                    if gz_load(state, buf as *mut u8, len, &mut n) == -(1 as i32) {
                        return -(1 as i32);
                    }
                } else {
                    (*strm).avail_out = len;
                    (*strm).next_out = buf as *mut Bytef;
                    if gz_decomp(state) == -(1 as i32) {
                        return -(1 as i32);
                    }
                    n = (*state).have;
                    (*state).have = 0 as i32 as u32;
                }
                current_block_39 = 8693738493027456495;
            }
        }
        match current_block_39 {
            8693738493027456495 => {
                len = len.wrapping_sub(n);
                buf = (buf as *mut i8).offset(n as isize) as voidp;
                got = got.wrapping_add(n);
                (*state).pos += n as i64;
            }
            _ => {}
        }
        if !(len != 0) {
            break;
        }
    }
    return got as i32;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzgetc(mut file: gzFile) -> i32 {
    let mut ret: i32 = 0;
    let mut buf: [u8; 1] = [0; 1];
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as i32);
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as i32 || (*state).err != 0 as i32 {
        return -(1 as i32);
    }
    if (*state).have != 0 {
        (*state).have = ((*state).have).wrapping_sub(1);
        (*state).have;
        (*state).pos += 1;
        (*state).pos;
        let fresh20 = (*state).next;
        (*state).next = ((*state).next).offset(1);
        return *fresh20 as i32;
    }
    ret = _glp_zlib_gzread(file, buf.as_mut_ptr() as voidp, 1 as i32 as u32);
    return if ret < 1 as i32 { -(1 as i32) } else { buf[0 as i32 as usize] as i32 };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzungetc(mut c: i32, mut file: gzFile) -> i32 {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as i32);
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as i32 || (*state).err != 0 as i32 {
        return -(1 as i32);
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as i32;
        if gz_skip(state, (*state).skip) == -(1 as i32) {
            return -(1 as i32);
        }
    }
    if c < 0 as i32 {
        return -(1 as i32);
    }
    if (*state).have == 0 as i32 as u32 {
        (*state).have = 1 as i32 as u32;
        (*state).next = ((*state).out)
            .offset(((*state).size << 1 as i32) as isize)
            .offset(-(1 as i32 as isize));
        *((*state).next).offset(0 as i32 as isize) = c as u8;
        (*state).pos -= 1;
        (*state).pos;
        return c;
    }
    if (*state).have == (*state).size << 1 as i32 {
        _glp_zlib_gz_error(
            state,
            -(5 as i32),
            b"out of room to push characters\0" as *const u8 as *const i8,
        );
        return -(1 as i32);
    }
    if (*state).next == (*state).out {
        let mut src: *mut u8 = ((*state).out).offset((*state).have as isize);
        let mut dest: *mut u8 = ((*state).out)
            .offset(((*state).size << 1 as i32) as isize);
        while src > (*state).out {
            src = src.offset(-1);
            dest = dest.offset(-1);
            *dest = *src;
        }
        (*state).next = dest;
    }
    (*state).have = ((*state).have).wrapping_add(1);
    (*state).have;
    (*state).next = ((*state).next).offset(-1);
    (*state).next;
    *((*state).next).offset(0 as i32 as isize) = c as u8;
    (*state).pos -= 1;
    (*state).pos;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzgets(
    mut file: gzFile,
    mut buf: *mut i8,
    mut len: i32,
) -> *mut i8 {
    let mut left: u32 = 0;
    let mut n: u32 = 0;
    let mut str: *mut i8 = 0 as *mut i8;
    let mut eol: *mut u8 = 0 as *mut u8;
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() || buf.is_null() || len < 1 as i32 {
        return 0 as *mut i8;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as i32 || (*state).err != 0 as i32 {
        return 0 as *mut i8;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as i32;
        if gz_skip(state, (*state).skip) == -(1 as i32) {
            return 0 as *mut i8;
        }
    }
    str = buf;
    left = (len as u32).wrapping_sub(1 as i32 as u32);
    if left != 0 {
        loop {
            if (*state).have == 0 as i32 as u32 {
                if gz_make(state) == -(1 as i32) {
                    return 0 as *mut i8;
                }
                if (*state).have == 0 as i32 as u32 {
                    if buf == str {
                        return 0 as *mut i8;
                    }
                    break;
                }
            }
            n = if (*state).have > left { left } else { (*state).have };
            eol = memchr((*state).next as *const libc::c_void, '\n' as i32, n as u64)
                as *mut u8;
            if !eol.is_null() {
                n = (eol.offset_from((*state).next) as i64 as u32)
                    .wrapping_add(1 as i32 as u32);
            }
            memcpy(
                buf as *mut libc::c_void,
                (*state).next as *const libc::c_void,
                n as u64,
            );
            (*state).have = ((*state).have).wrapping_sub(n);
            (*state).next = ((*state).next).offset(n as isize);
            (*state).pos += n as i64;
            left = left.wrapping_sub(n);
            buf = buf.offset(n as isize);
            if !(left != 0 && eol.is_null()) {
                break;
            }
        }
    }
    *buf.offset(0 as i32 as isize) = 0 as i32 as i8;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzdirect(mut file: gzFile) -> i32 {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return 0 as i32;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as i32 {
        return 0 as i32;
    }
    if (*state).how == 0 as i32 && (*state).have == 0 as i32 as u32 {
        gz_head(state);
    }
    return (*state).direct;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_zlib_gzclose_r(mut file: gzFile) -> i32 {
    let mut ret: i32 = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(2 as i32);
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as i32 {
        return -(2 as i32);
    }
    if (*state).size != 0 {
        _glp_zlib_inflateEnd(&mut (*state).strm);
        free((*state).out as *mut libc::c_void);
        free((*state).in_0 as *mut libc::c_void);
    }
    _glp_zlib_gz_error(state, 0 as i32, 0 as *const i8);
    free((*state).path as *mut libc::c_void);
    ret = _glp_zlib_close((*state).fd);
    free(state as *mut libc::c_void);
    return if ret != 0 { -(1 as i32) } else { 0 as i32 };
}