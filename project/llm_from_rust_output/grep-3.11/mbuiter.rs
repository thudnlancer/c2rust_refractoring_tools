use std::mem;
use std::ptr;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_long};

pub type ptrdiff_t = c_long;
pub type size_t = c_ulong;
pub type wchar_t = c_int;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: c_int,
    pub __value: C2RustUnnamed,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: c_uint,
    pub __wchb: [c_char; 4],
}

pub type mbstate_t = __mbstate_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbchar {
    pub ptr: *const c_char,
    pub bytes: size_t,
    pub wc_valid: bool,
    pub wc: wchar_t,
    pub buf: [c_char; 24],
}

pub type mbchar_t = mbchar;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbuiter_multi {
    pub in_shift: bool,
    pub state: mbstate_t,
    pub next_done: bool,
    pub cur: mbchar,
}

impl mbuiter_multi {
    pub fn copy(&mut self, old_iter: &Self) {
        self.in_shift = old_iter.in_shift;
        if self.in_shift {
            unsafe {
                ptr::copy_nonoverlapping(
                    &old_iter.state as *const mbstate_t,
                    &mut self.state as *mut mbstate_t,
                    1,
                );
            }
        } else {
            unsafe {
                ptr::write_bytes(
                    &mut self.state as *mut mbstate_t,
                    0,
                    1,
                );
            }
        }
        self.next_done = old_iter.next_done;
        mb_copy(&mut self.cur, &old_iter.cur);
    }

    pub fn reloc(&mut self, ptrdiff: ptrdiff_t) {
        unsafe {
            self.cur.ptr = self.cur.ptr.offset(ptrdiff as isize);
        }
    }

    pub fn next(&mut self) {
        if self.next_done {
            return;
        }

        let current_block = if self.in_shift {
            0
        } else if is_basic(unsafe { *self.cur.ptr }) {
            self.cur.bytes = 1;
            self.cur.wc = unsafe { *self.cur.ptr } as wchar_t;
            self.cur.wc_valid = true;
            1
        } else {
            assert!(unsafe { mbsinit(&mut self.state) } != 0);
            self.in_shift = true;
            0
        };

        if current_block == 0 {
            let max_len = unsafe { __ctype_get_mb_cur_max() };
            let len = unsafe { strnlen1(self.cur.ptr, max_len) };
            self.cur.bytes = unsafe {
                rpl_mbrtowc(
                    &mut self.cur.wc,
                    self.cur.ptr,
                    len,
                    &mut self.state,
                )
            };

            if self.cur.bytes == (-1_i32) as size_t {
                self.cur.bytes = 1;
                self.cur.wc_valid = false;
            } else if self.cur.bytes == (-2_i32) as size_t {
                self.cur.bytes = unsafe { strlen(self.cur.ptr) };
                self.cur.wc_valid = false;
            } else {
                if self.cur.bytes == 0 {
                    self.cur.bytes = 1;
                    assert_eq!(unsafe { *self.cur.ptr }, 0);
                    assert_eq!(self.cur.wc, 0);
                }
                self.cur.wc_valid = true;
                if unsafe { mbsinit(&mut self.state) } != 0 {
                    self.in_shift = false;
                }
            }
        }

        self.next_done = true;
    }
}

fn is_basic(c: c_char) -> bool {
    unsafe {
        *is_basic_table
            .as_ptr()
            .offset((c as u8 as c_int >> 5) as isize)
            >> (c as u8 as c_int & 31)
            & 1 as c_uint != 0
    }
}

fn mb_copy(new_mbc: &mut mbchar_t, old_mbc: &mbchar_t) {
    if old_mbc.ptr == unsafe { &*(old_mbc.buf).as_ptr().offset(0) } as *const c_char {
        unsafe {
            ptr::copy_nonoverlapping(
                (old_mbc.buf).as_ptr().offset(0),
                (new_mbc.buf).as_mut_ptr().offset(0),
                old_mbc.bytes,
            );
        }
        new_mbc.ptr = unsafe { &mut *(new_mbc.buf).as_mut_ptr().offset(0) } as *mut c_char;
    } else {
        new_mbc.ptr = old_mbc.ptr;
    }
    new_mbc.bytes = old_mbc.bytes;
    new_mbc.wc_valid = old_mbc.wc_valid;
    if new_mbc.wc_valid {
        new_mbc.wc = old_mbc.wc;
    }
}

extern "C" {
    fn __ctype_get_mb_cur_max() -> size_t;
    fn strnlen1(string: *const c_char, maxlen: size_t) -> size_t;
    static is_basic_table: [c_uint; 0];
    fn mbsinit(__ps: *const mbstate_t) -> c_int;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn strlen(_: *const c_char) -> c_ulong;
}