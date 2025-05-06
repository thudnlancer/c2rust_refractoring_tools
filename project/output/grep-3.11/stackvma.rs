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
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> i32;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: i32,
        __flags: i32,
        __fd: i32,
        __offset: __off_t,
    ) -> *mut libc::c_void;
    fn mincore(__start: *mut libc::c_void, __len: size_t, __vec: *mut u8) -> i32;
}
pub type __off_t = i64;
pub type __ssize_t = i64;
pub type uintptr_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vma_struct {
    pub start: uintptr_t,
    pub end: uintptr_t,
    pub is_near_this: Option<unsafe extern "C" fn(uintptr_t, *mut vma_struct) -> i32>,
    pub prev_end: uintptr_t,
}
pub type pageinfo_t = u8;
pub type MINCORE_ADDR_T = *mut libc::c_void;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct callback_locals {
    pub address: uintptr_t,
    pub vma: *mut vma_struct,
    pub prev: uintptr_t,
    pub retval: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rofile {
    pub position: size_t,
    pub filled: size_t,
    pub eof_seen: i32,
    pub buffer: *mut i8,
    pub auxmap: *mut i8,
    pub auxmap_length: size_t,
    pub auxmap_start: uintptr_t,
    pub auxmap_end: uintptr_t,
    pub stack_allocated_buffer: [i8; 1],
}
pub type ssize_t = __ssize_t;
unsafe extern "C" fn simple_is_near_this(
    mut addr: uintptr_t,
    mut vma: *mut vma_struct,
) -> i32 {
    return (((*vma).start).wrapping_sub(addr)
        <= ((*vma).start).wrapping_sub((*vma).prev_end).wrapping_div(2 as i32 as u64))
        as i32;
}
unsafe extern "C" fn rof_open(mut rof: *mut rofile, mut filename: *const i8) -> i32 {
    let mut current_block: u64;
    let mut fd: i32 = 0;
    let mut pagesize_0: uintptr_t = 0;
    let mut size: size_t = 0;
    fd = open(filename, 0 as i32);
    if fd < 0 as i32 {
        return -(1 as i32);
    }
    (*rof).position = 0 as i32 as size_t;
    (*rof).eof_seen = 0 as i32;
    pagesize_0 = 0 as i32 as uintptr_t;
    (*rof).buffer = ((*rof).stack_allocated_buffer).as_mut_ptr();
    size = ::core::mem::size_of::<[i8; 1]>() as u64;
    (*rof).auxmap = 0 as *mut i8;
    (*rof).auxmap_start = 0 as i32 as uintptr_t;
    (*rof).auxmap_end = 0 as i32 as uintptr_t;
    's_46: loop {
        if size > (73 as i32 + 4096 as i32) as u64 {
            let mut n: i32 = read(fd, (*rof).buffer as *mut libc::c_void, size) as i32;
            if n < 0 as i32 && *__errno_location() == 4 as i32 {
                current_block = 5349275757994369183;
            } else if n <= 0 as i32 {
                current_block = 13680957031993253724;
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
                        current_block = 5349275757994369183;
                        break;
                    }
                    if n < 0 as i32 {
                        current_block = 13680957031993253724;
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
                if pagesize_0 == 0 as i32 as u64 {
                    pagesize_0 = getpagesize() as uintptr_t;
                    size = pagesize_0;
                    while size <= (73 as i32 + 4096 as i32) as u64 {
                        size = (2 as i32 as u64).wrapping_mul(size);
                    }
                } else {
                    size = (2 as i32 as u64).wrapping_mul(size);
                    if size == 0 as i32 as u64 {
                        current_block = 13680957031993253724;
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
                (*rof).auxmap_start = (*rof).auxmap as uintptr_t;
                (*rof).auxmap_end = ((*rof).auxmap_start).wrapping_add(size);
                (*rof).buffer = (*rof).auxmap;
            }
            _ => {}
        }
        if !(lseek(fd, 0 as i32 as __off_t, 0 as i32) < 0 as i32 as i64) {
            continue;
        }
        close(fd);
        fd = open(filename, 0 as i32);
        if fd < 0 as i32 {
            current_block = 4988569561800699799;
            break;
        }
    }
    match current_block {
        13680957031993253724 => {
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
unsafe extern "C" fn rof_scanf_lx(
    mut rof: *mut rofile,
    mut valuep: *mut uintptr_t,
) -> i32 {
    let mut value: uintptr_t = 0 as i32 as uintptr_t;
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
unsafe extern "C" fn vma_iterate_proc(mut locals: *mut callback_locals) -> i32 {
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
        let mut auxmap_start: uintptr_t = rof.auxmap_start;
        let mut auxmap_end: uintptr_t = rof.auxmap_end;
        loop {
            let mut start: uintptr_t = 0;
            let mut end: uintptr_t = 0;
            let mut c: i32 = 0;
            if !(rof_scanf_lx(&mut rof, &mut start) >= 0 as i32
                && rof_getchar(&mut rof) == '-' as i32
                && rof_scanf_lx(&mut rof, &mut end) >= 0 as i32)
            {
                break;
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
                    if callback(locals, start, auxmap_start) != 0 {
                        break;
                    }
                }
                if !(auxmap_end.wrapping_sub(1 as i32 as u64)
                    < end.wrapping_sub(1 as i32 as u64))
                {
                    continue;
                }
                if callback(locals, auxmap_end, end) != 0 {
                    break;
                }
            } else if callback(locals, start, end) != 0 {
                break;
            }
        }
        rof_close(&mut rof);
        return 0 as i32;
    }
    return -(1 as i32);
}
unsafe extern "C" fn vma_iterate(mut locals: *mut callback_locals) -> i32 {
    let mut retval: i32 = vma_iterate_proc(locals);
    if retval == 0 as i32 {
        return 0 as i32;
    }
    return -(1 as i32);
}
static mut pagesize: uintptr_t = 0;
unsafe extern "C" fn init_pagesize() {
    pagesize = getpagesize() as uintptr_t;
}
unsafe extern "C" fn is_mapped(mut addr: uintptr_t) -> i32 {
    let mut vec: [pageinfo_t; 1] = [0; 1];
    return (mincore(addr as MINCORE_ADDR_T, pagesize, vec.as_mut_ptr()) >= 0 as i32)
        as i32;
}
unsafe extern "C" fn mapped_range_start(mut addr: uintptr_t) -> uintptr_t {
    let mut vec: [pageinfo_t; 1024] = [0; 1024];
    let mut stepsize: uintptr_t = ::core::mem::size_of::<[pageinfo_t; 1024]>() as u64;
    loop {
        let mut max_remaining: uintptr_t = 0;
        if addr == 0 as i32 as u64 {
            return addr;
        }
        max_remaining = addr.wrapping_div(pagesize);
        if stepsize > max_remaining {
            stepsize = max_remaining;
        }
        if mincore(
            addr.wrapping_sub(stepsize.wrapping_mul(pagesize)) as MINCORE_ADDR_T,
            stepsize.wrapping_mul(pagesize),
            vec.as_mut_ptr(),
        ) < 0 as i32
        {
            break;
        }
        addr = (addr as u64).wrapping_sub(stepsize.wrapping_mul(pagesize)) as uintptr_t
            as uintptr_t;
    }
    loop {
        let mut halfstepsize1: uintptr_t = 0;
        let mut halfstepsize2: uintptr_t = 0;
        if stepsize == 1 as i32 as u64 {
            return addr;
        }
        halfstepsize1 = stepsize
            .wrapping_add(1 as i32 as u64)
            .wrapping_div(2 as i32 as u64);
        halfstepsize2 = stepsize.wrapping_div(2 as i32 as u64);
        if mincore(
            addr.wrapping_sub(halfstepsize1.wrapping_mul(pagesize)) as MINCORE_ADDR_T,
            halfstepsize1.wrapping_mul(pagesize),
            vec.as_mut_ptr(),
        ) < 0 as i32
        {
            stepsize = halfstepsize1;
        } else {
            addr = (addr as u64).wrapping_sub(halfstepsize1.wrapping_mul(pagesize))
                as uintptr_t as uintptr_t;
            stepsize = halfstepsize2;
        }
    };
}
unsafe extern "C" fn mapped_range_end(mut addr: uintptr_t) -> uintptr_t {
    let mut vec: [pageinfo_t; 1024] = [0; 1024];
    let mut stepsize: uintptr_t = ::core::mem::size_of::<[pageinfo_t; 1024]>() as u64;
    addr = (addr as u64).wrapping_add(pagesize) as uintptr_t as uintptr_t;
    loop {
        let mut max_remaining: uintptr_t = 0;
        if addr == 0 as i32 as u64 {
            return addr;
        }
        max_remaining = addr.wrapping_neg().wrapping_div(pagesize);
        if stepsize > max_remaining {
            stepsize = max_remaining;
        }
        if mincore(
            addr as MINCORE_ADDR_T,
            stepsize.wrapping_mul(pagesize),
            vec.as_mut_ptr(),
        ) < 0 as i32
        {
            break;
        }
        addr = (addr as u64).wrapping_add(stepsize.wrapping_mul(pagesize)) as uintptr_t
            as uintptr_t;
    }
    loop {
        let mut halfstepsize1: uintptr_t = 0;
        let mut halfstepsize2: uintptr_t = 0;
        if stepsize == 1 as i32 as u64 {
            return addr;
        }
        halfstepsize1 = stepsize
            .wrapping_add(1 as i32 as u64)
            .wrapping_div(2 as i32 as u64);
        halfstepsize2 = stepsize.wrapping_div(2 as i32 as u64);
        if mincore(
            addr as MINCORE_ADDR_T,
            halfstepsize1.wrapping_mul(pagesize),
            vec.as_mut_ptr(),
        ) < 0 as i32
        {
            stepsize = halfstepsize1;
        } else {
            addr = (addr as u64).wrapping_add(halfstepsize1.wrapping_mul(pagesize))
                as uintptr_t as uintptr_t;
            stepsize = halfstepsize2;
        }
    };
}
unsafe extern "C" fn is_unmapped(mut addr1: uintptr_t, mut addr2: uintptr_t) -> i32 {
    let mut count: uintptr_t = 0;
    let mut stepsize: uintptr_t = 0;
    addr1 = addr1.wrapping_div(pagesize).wrapping_mul(pagesize);
    addr2 = addr2
        .wrapping_div(pagesize)
        .wrapping_add(1 as i32 as u64)
        .wrapping_mul(pagesize);
    count = addr2.wrapping_sub(addr1).wrapping_div(pagesize);
    stepsize = 1 as i32 as uintptr_t;
    while stepsize < count {
        stepsize = (2 as i32 as u64).wrapping_mul(stepsize);
    }
    loop {
        let mut addr_stepsize: uintptr_t = 0;
        let mut i: uintptr_t = 0;
        let mut addr: uintptr_t = 0;
        stepsize = stepsize.wrapping_div(2 as i32 as u64);
        if stepsize == 0 as i32 as u64 {
            break;
        }
        addr_stepsize = stepsize.wrapping_mul(pagesize);
        i = stepsize;
        addr = addr1.wrapping_add(addr_stepsize);
        while i < count {
            if is_mapped(addr) != 0 {
                return 0 as i32;
            }
            i = (i as u64).wrapping_add((2 as i32 as u64).wrapping_mul(stepsize))
                as uintptr_t as uintptr_t;
            addr = (addr as u64)
                .wrapping_add((2 as i32 as u64).wrapping_mul(addr_stepsize)) as uintptr_t
                as uintptr_t;
        }
    }
    return 1 as i32;
}
unsafe extern "C" fn mincore_is_near_this(
    mut addr: uintptr_t,
    mut vma: *mut vma_struct,
) -> i32 {
    let mut testaddr: uintptr_t = addr.wrapping_sub(((*vma).start).wrapping_sub(addr));
    if testaddr > addr {
        return 0 as i32;
    }
    return is_unmapped(testaddr, ((*vma).start).wrapping_sub(1 as i32 as u64));
}
unsafe extern "C" fn mincore_get_vma(
    mut address: uintptr_t,
    mut vma: *mut vma_struct,
) -> i32 {
    if pagesize == 0 as i32 as u64 {
        init_pagesize();
    }
    address = address.wrapping_div(pagesize).wrapping_mul(pagesize);
    (*vma).start = mapped_range_start(address);
    (*vma).end = mapped_range_end(address);
    (*vma).is_near_this = Some(
        mincore_is_near_this as unsafe extern "C" fn(uintptr_t, *mut vma_struct) -> i32,
    );
    return 0 as i32;
}
unsafe extern "C" fn callback(
    mut locals: *mut callback_locals,
    mut start: uintptr_t,
    mut end: uintptr_t,
) -> i32 {
    if (*locals).address >= start
        && (*locals).address <= end.wrapping_sub(1 as i32 as u64)
    {
        (*(*locals).vma).start = start;
        (*(*locals).vma).end = end;
        (*(*locals).vma).prev_end = (*locals).prev;
        (*locals).retval = 0 as i32;
        return 1 as i32;
    }
    (*locals).prev = end;
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn sigsegv_get_vma(
    mut address: uintptr_t,
    mut vma: *mut vma_struct,
) -> i32 {
    let mut locals: callback_locals = callback_locals {
        address: 0,
        vma: 0 as *mut vma_struct,
        prev: 0,
        retval: 0,
    };
    locals.address = address;
    locals.vma = vma;
    locals.prev = 0 as i32 as uintptr_t;
    locals.retval = -(1 as i32);
    vma_iterate(&mut locals);
    if locals.retval == 0 as i32 {
        (*vma).is_near_this = Some(
            simple_is_near_this
                as unsafe extern "C" fn(uintptr_t, *mut vma_struct) -> i32,
        );
        return 0 as i32;
    }
    return mincore_get_vma(address, vma);
}