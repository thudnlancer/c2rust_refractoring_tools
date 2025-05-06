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
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn lseek(__fd: i32, __offset: __off_t, __whence: i32) -> __off_t;
    fn close(__fd: i32) -> i32;
    fn read(__fd: i32, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn getpagesize() -> i32;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: i32,
        __flags: i32,
        __fd: i32,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> i32;
}
pub type __off_t = i64;
pub type __ssize_t = i64;
pub type uintptr_t = u64;
pub type vma_iterate_callback_fn = Option<
    unsafe extern "C" fn(*mut libc::c_void, uintptr_t, uintptr_t, u32) -> i32,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rofile {
    pub position: size_t,
    pub filled: size_t,
    pub eof_seen: i32,
    pub buffer: *mut i8,
    pub auxmap: *mut i8,
    pub auxmap_length: size_t,
    pub auxmap_start: u64,
    pub auxmap_end: u64,
    pub stack_allocated_buffer: [i8; 1],
}
pub type size_t = u64;
pub type ssize_t = __ssize_t;
unsafe extern "C" fn rof_open(mut rof: *mut rofile, mut filename: *const i8) -> i32 {
    let mut current_block: u64;
    let mut fd: i32 = 0;
    let mut pagesize: u64 = 0;
    let mut size: size_t = 0;
    fd = open(filename, 0 as i32 | 0o2000000 as i32);
    if fd < 0 as i32 {
        return -(1 as i32);
    }
    (*rof).position = 0 as i32 as size_t;
    (*rof).eof_seen = 0 as i32;
    pagesize = 0 as i32 as u64;
    (*rof).buffer = ((*rof).stack_allocated_buffer).as_mut_ptr();
    size = ::core::mem::size_of::<[i8; 1]>() as u64;
    (*rof).auxmap = 0 as *mut i8;
    (*rof).auxmap_start = 0 as i32 as u64;
    (*rof).auxmap_end = 0 as i32 as u64;
    's_46: loop {
        if size > (73 as i32 + 4096 as i32) as u64 {
            let mut n: i32 = read(fd, (*rof).buffer as *mut libc::c_void, size) as i32;
            if n < 0 as i32 && *__errno_location() == 4 as i32 {
                current_block = 14449773916612594310;
            } else if n <= 0 as i32 {
                current_block = 16343023585595149896;
                break;
            } else if (n + (73 as i32 + 4096 as i32)) as u64 <= size {
                (*rof).filled = n as size_t;
                loop {
                    n = read(
                        fd,
                        ((*rof).buffer).offset((*rof).filled as isize)
                            as *mut libc::c_void,
                        size.wrapping_sub((*rof).filled),
                    ) as i32;
                    if n < 0 as i32 && *__errno_location() == 4 as i32 {
                        current_block = 14449773916612594310;
                        break;
                    }
                    if n < 0 as i32 {
                        current_block = 16343023585595149896;
                        break 's_46;
                    } else if (n + (73 as i32 + 4096 as i32)) as u64
                        > size.wrapping_sub((*rof).filled)
                    {
                        current_block = 10043043949733653460;
                        break;
                    } else {
                        if n == 0 as i32 {
                            close(fd);
                            return 0 as i32;
                        }
                        (*rof).filled = ((*rof).filled as u64).wrapping_add(n as u64)
                            as size_t as size_t;
                    }
                }
            } else {
                current_block = 10043043949733653460;
            }
        } else {
            current_block = 10043043949733653460;
        }
        match current_block {
            10043043949733653460 => {
                if pagesize == 0 as i32 as u64 {
                    pagesize = getpagesize() as u64;
                    size = pagesize;
                    while size <= (73 as i32 + 4096 as i32) as u64 {
                        size = (2 as i32 as u64).wrapping_mul(size);
                    }
                } else {
                    size = (2 as i32 as u64).wrapping_mul(size);
                    if size == 0 as i32 as u64 {
                        current_block = 16343023585595149896;
                        break;
                    }
                    if !((*rof).auxmap).is_null() {
                        munmap((*rof).auxmap as *mut libc::c_void, (*rof).auxmap_length);
                    }
                }
                (*rof).auxmap = mmap(
                    0 as *mut libc::c_void,
                    size,
                    0x1 as i32 | 0x2 as i32,
                    0x20 as i32 | 0x2 as i32,
                    -(1 as i32),
                    0 as i32 as __off_t,
                ) as *mut i8;
                if (*rof).auxmap == -(1 as i32) as *mut libc::c_void as *mut i8 {
                    close(fd);
                    return -(1 as i32);
                }
                (*rof).auxmap_length = size;
                (*rof).auxmap_start = (*rof).auxmap as u64;
                (*rof).auxmap_end = ((*rof).auxmap_start).wrapping_add(size);
                (*rof).buffer = (*rof).auxmap;
            }
            _ => {}
        }
        if !(lseek(fd, 0 as i32 as __off_t, 0 as i32) < 0 as i32 as i64) {
            continue;
        }
        close(fd);
        fd = open(filename, 0 as i32 | 0o2000000 as i32);
        if fd < 0 as i32 {
            current_block = 5826364949951567678;
            break;
        }
    }
    match current_block {
        16343023585595149896 => {
            close(fd);
        }
        _ => {}
    }
    if !((*rof).auxmap).is_null() {
        munmap((*rof).auxmap as *mut libc::c_void, (*rof).auxmap_length);
    }
    return -(1 as i32);
}
unsafe extern "C" fn rof_peekchar(mut rof: *mut rofile) -> i32 {
    if (*rof).position == (*rof).filled {
        (*rof).eof_seen = 1 as i32;
        return -(1 as i32);
    }
    return *((*rof).buffer).offset((*rof).position as isize) as u8 as i32;
}
unsafe extern "C" fn rof_getchar(mut rof: *mut rofile) -> i32 {
    let mut c: i32 = rof_peekchar(rof);
    if c >= 0 as i32 {
        (*rof).position = ((*rof).position).wrapping_add(1);
        (*rof).position;
    }
    return c;
}
unsafe extern "C" fn rof_scanf_lx(mut rof: *mut rofile, mut valuep: *mut u64) -> i32 {
    let mut value: u64 = 0 as i32 as u64;
    let mut numdigits: u32 = 0 as i32 as u32;
    loop {
        let mut c: i32 = rof_peekchar(rof);
        if c >= '0' as i32 && c <= '9' as i32 {
            value = (value << 4 as i32).wrapping_add((c - '0' as i32) as u64);
        } else if c >= 'A' as i32 && c <= 'F' as i32 {
            value = (value << 4 as i32)
                .wrapping_add((c - 'A' as i32 + 10 as i32) as u64);
        } else {
            if !(c >= 'a' as i32 && c <= 'f' as i32) {
                break;
            }
            value = (value << 4 as i32)
                .wrapping_add((c - 'a' as i32 + 10 as i32) as u64);
        }
        rof_getchar(rof);
        numdigits = numdigits.wrapping_add(1);
        numdigits;
    }
    if numdigits == 0 as i32 as u32 {
        return -(1 as i32);
    }
    *valuep = value;
    return 0 as i32;
}
unsafe extern "C" fn rof_close(mut rof: *mut rofile) {
    if !((*rof).auxmap).is_null() {
        munmap((*rof).auxmap as *mut libc::c_void, (*rof).auxmap_length);
    }
}
unsafe extern "C" fn vma_iterate_proc(
    mut callback: vma_iterate_callback_fn,
    mut data: *mut libc::c_void,
) -> i32 {
    let mut rof: rofile = rofile {
        position: 0,
        filled: 0,
        eof_seen: 0,
        buffer: 0 as *mut i8,
        auxmap: 0 as *mut i8,
        auxmap_length: 0,
        auxmap_start: 0,
        auxmap_end: 0,
        stack_allocated_buffer: [0; 1],
    };
    if rof_open(&mut rof, b"/proc/self/maps\0" as *const u8 as *const i8) >= 0 as i32 {
        let mut auxmap_start: u64 = rof.auxmap_start;
        let mut auxmap_end: u64 = rof.auxmap_end;
        loop {
            let mut start: u64 = 0;
            let mut end: u64 = 0;
            let mut flags: u32 = 0;
            let mut c: i32 = 0;
            if !(rof_scanf_lx(&mut rof, &mut start) >= 0 as i32
                && rof_getchar(&mut rof) == '-' as i32
                && rof_scanf_lx(&mut rof, &mut end) >= 0 as i32)
            {
                break;
            }
            loop {
                c = rof_getchar(&mut rof);
                if !(c == ' ' as i32) {
                    break;
                }
            }
            flags = 0 as i32 as u32;
            if c == 'r' as i32 {
                flags |= ((1 as i32) << 0 as i32) as u32;
            }
            c = rof_getchar(&mut rof);
            if c == 'w' as i32 {
                flags |= ((1 as i32) << 1 as i32) as u32;
            }
            c = rof_getchar(&mut rof);
            if c == 'x' as i32 {
                flags |= ((1 as i32) << 2 as i32) as u32;
            }
            loop {
                c = rof_getchar(&mut rof);
                if !(c != -(1 as i32) && c != '\n' as i32) {
                    break;
                }
            }
            if start <= auxmap_start
                && auxmap_end.wrapping_sub(1 as i32 as u64)
                    <= end.wrapping_sub(1 as i32 as u64)
            {
                if start < auxmap_start {
                    if callback
                        .expect(
                            "non-null function pointer",
                        )(data, start, auxmap_start, flags) != 0
                    {
                        break;
                    }
                }
                if !(auxmap_end.wrapping_sub(1 as i32 as u64)
                    < end.wrapping_sub(1 as i32 as u64))
                {
                    continue;
                }
                if callback
                    .expect("non-null function pointer")(data, auxmap_end, end, flags)
                    != 0
                {
                    break;
                }
            } else if callback
                .expect("non-null function pointer")(data, start, end, flags) != 0
            {
                break;
            }
        }
        rof_close(&mut rof);
        return 0 as i32;
    }
    return -(1 as i32);
}
#[inline]
unsafe extern "C" fn vma_iterate_bsd(
    mut callback: vma_iterate_callback_fn,
    mut data: *mut libc::c_void,
) -> i32 {
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn vma_iterate(
    mut callback: vma_iterate_callback_fn,
    mut data: *mut libc::c_void,
) -> i32 {
    let mut retval: i32 = vma_iterate_proc(callback, data);
    if retval == 0 as i32 {
        return 0 as i32;
    }
    return vma_iterate_bsd(callback, data);
}