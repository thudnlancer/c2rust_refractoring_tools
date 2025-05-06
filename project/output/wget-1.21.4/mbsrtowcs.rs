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
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const i8,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn __errno_location() -> *mut i32;
    fn abort() -> !;
    fn strnlen1(string: *const i8, maxlen: size_t) -> size_t;
    static mut _gl_mbsrtowcs_state: mbstate_t;
}
pub type size_t = u64;
pub type wchar_t = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: i32,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: u32,
    pub __wchb: [i8; 4],
}
pub type mbstate_t = __mbstate_t;
#[no_mangle]
pub unsafe extern "C" fn rpl_mbsrtowcs(
    mut dest: *mut wchar_t,
    mut srcp: *mut *const i8,
    mut len: size_t,
    mut ps: *mut mbstate_t,
) -> size_t {
    if ps.is_null() {
        ps = &mut _gl_mbsrtowcs_state;
    }
    let mut current_block_34: u64;
    let mut src: *const i8 = *srcp;
    if !dest.is_null() {
        let mut destptr: *mut wchar_t = dest;
        loop {
            if !(len > 0 as i32 as u64) {
                current_block_34 = 7175849428784450219;
                break;
            }
            let mut src_avail: size_t = 0;
            let mut ret: size_t = 0;
            if *src.offset(0 as i32 as isize) as i32 == '\0' as i32 {
                src_avail = 1 as i32 as size_t;
            } else if *src.offset(1 as i32 as isize) as i32 == '\0' as i32 {
                src_avail = 2 as i32 as size_t;
            } else if *src.offset(2 as i32 as isize) as i32 == '\0' as i32 {
                src_avail = 3 as i32 as size_t;
            } else if 16 as i32 <= 4 as i32
                || *src.offset(3 as i32 as isize) as i32 == '\0' as i32
            {
                src_avail = 4 as i32 as size_t;
            } else {
                src_avail = (4 as i32 as u64)
                    .wrapping_add(
                        strnlen1(
                            src.offset(4 as i32 as isize),
                            (16 as i32 - 4 as i32) as size_t,
                        ),
                    );
            }
            ret = rpl_mbrtowc(destptr, src, src_avail, ps);
            if ret == -(2 as i32) as size_t {
                abort();
            }
            if ret == -(1 as i32) as size_t {
                current_block_34 = 4902359605313298855;
                break;
            }
            if ret == 0 as i32 as u64 {
                src = 0 as *const i8;
                current_block_34 = 7175849428784450219;
                break;
            } else {
                src = src.offset(ret as isize);
                destptr = destptr.offset(1);
                destptr;
                len = len.wrapping_sub(1);
                len;
            }
        }
        match current_block_34 {
            4902359605313298855 => {
                *srcp = src;
            }
            _ => {
                *srcp = src;
                return destptr.offset_from(dest) as i64 as size_t;
            }
        }
    } else {
        let mut state: mbstate_t = *ps;
        let mut totalcount: size_t = 0 as i32 as size_t;
        loop {
            let mut src_avail_0: size_t = 0;
            let mut ret_0: size_t = 0;
            if *src.offset(0 as i32 as isize) as i32 == '\0' as i32 {
                src_avail_0 = 1 as i32 as size_t;
            } else if *src.offset(1 as i32 as isize) as i32 == '\0' as i32 {
                src_avail_0 = 2 as i32 as size_t;
            } else if *src.offset(2 as i32 as isize) as i32 == '\0' as i32 {
                src_avail_0 = 3 as i32 as size_t;
            } else if 16 as i32 <= 4 as i32
                || *src.offset(3 as i32 as isize) as i32 == '\0' as i32
            {
                src_avail_0 = 4 as i32 as size_t;
            } else {
                src_avail_0 = (4 as i32 as u64)
                    .wrapping_add(
                        strnlen1(
                            src.offset(4 as i32 as isize),
                            (16 as i32 - 4 as i32) as size_t,
                        ),
                    );
            }
            ret_0 = rpl_mbrtowc(0 as *mut wchar_t, src, src_avail_0, &mut state);
            if ret_0 == -(2 as i32) as size_t {
                abort();
            }
            if ret_0 == -(1 as i32) as size_t {
                current_block_34 = 14573528950570904290;
                break;
            }
            if ret_0 == 0 as i32 as u64 {
                current_block_34 = 6417057564578538666;
                break;
            }
            src = src.offset(ret_0 as isize);
            totalcount = totalcount.wrapping_add(1);
            totalcount;
        }
        match current_block_34 {
            14573528950570904290 => {}
            _ => return totalcount,
        }
    }
    *__errno_location() = 84 as i32;
    return -(1 as i32) as size_t;
}