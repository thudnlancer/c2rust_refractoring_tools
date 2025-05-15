use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MbState {
    pub count: i32,
    pub value: MbStateValue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MbStateValue {
    pub wch: u32,
    pub wchb: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MbChar {
    pub ptr: *const u8,
    pub bytes: usize,
    pub wc_valid: bool,
    pub wc: i32,
    pub buf: [u8; 24],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MbIterMulti {
    pub limit: *const u8,
    pub in_shift: bool,
    pub state: MbState,
    pub next_done: bool,
    pub cur: MbChar,
}

impl MbChar {
    fn copy(&mut self, other: &MbChar) {
        if other.ptr == other.buf.as_ptr() as *const u8 {
            self.buf[..other.bytes].copy_from_slice(&other.buf[..other.bytes]);
            self.ptr = self.buf.as_ptr();
        } else {
            self.ptr = other.ptr;
        }
        self.bytes = other.bytes;
        self.wc_valid = other.wc_valid;
        if self.wc_valid {
            self.wc = other.wc;
        }
    }
}

impl MbIterMulti {
    pub fn next(&mut self) {
        if self.next_done {
            return;
        }

        if self.in_shift {
            self.process_shift();
        } else if is_basic(unsafe { *self.cur.ptr }) {
            self.cur.bytes = 1;
            self.cur.wc = unsafe { *self.cur.ptr } as i32;
            self.cur.wc_valid = true;
        } else {
            assert!(mbsinit(&self.state));
            self.in_shift = true;
            self.process_shift();
        }

        self.next_done = true;
    }

    fn process_shift(&mut self) {
        let remaining_bytes = unsafe { self.limit.offset_from(self.cur.ptr) as usize };
        let bytes = unsafe {
            rpl_mbrtowc(
                &mut self.cur.wc,
                self.cur.ptr,
                remaining_bytes,
                &mut self.state,
            )
        };

        match bytes {
            usize::MAX => {
                self.cur.bytes = 1;
                self.cur.wc_valid = false;
            }
            usize::MAX - 1 => {
                self.cur.bytes = remaining_bytes;
                self.cur.wc_valid = false;
            }
            0 => {
                self.cur.bytes = 1;
                assert_eq!(unsafe { *self.cur.ptr }, 0);
                assert_eq!(self.cur.wc, 0);
                self.cur.wc_valid = true;
            }
            _ => {
                self.cur.bytes = bytes;
                self.cur.wc_valid = true;
                if mbsinit(&self.state) {
                    self.in_shift = false;
                }
            }
        }
    }

    pub fn reloc(&mut self, offset: isize) {
        unsafe {
            self.cur.ptr = self.cur.ptr.offset(offset);
            self.limit = self.limit.offset(offset);
        }
    }

    pub fn copy(&mut self, other: &MbIterMulti) {
        self.limit = other.limit;
        self.in_shift = other.in_shift;
        
        if self.in_shift {
            self.state = other.state;
        } else {
            self.state = unsafe { mem::zeroed() };
        }
        
        self.next_done = other.next_done;
        self.cur.copy(&other.cur);
    }
}

fn is_basic(c: u8) -> bool {
    unsafe {
        (*IS_BASIC_TABLE.as_ptr().offset((c as usize >> 5) as isize) >> (c & 31)) & 1 != 0
    }
}

static IS_BASIC_TABLE: [u32; 0] = [];

extern "C" {
    fn mbsinit(ps: *const MbState) -> i32;
    fn rpl_mbrtowc(pwc: *mut i32, s: *const u8, n: usize, ps: *mut MbState) -> usize;
}