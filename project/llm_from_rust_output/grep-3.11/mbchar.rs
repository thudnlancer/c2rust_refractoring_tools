use std::mem;

#[derive(Copy, Clone)]
pub struct MbChar {
    ptr: *const u8,
    bytes: usize,
    wc_valid: bool,
    wc: i32,
    buf: [u8; 24],
}

impl MbChar {
    pub fn is_basic(&self) -> bool {
        const IS_BASIC_TABLE: [u32; 8] = [
            0x1a00,
            0xffffffef,
            0xfffffffe,
            0x7ffffffe,
            0,
            0,
            0,
            0,
        ];

        if self.bytes == 0 {
            return false;
        }

        let c = unsafe { *self.ptr };
        (IS_BASIC_TABLE[(c as usize >> 5)] >> (c & 31) & 1 != 0
    }

    pub fn copy_from(&mut self, other: &MbChar) {
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

    pub fn width(&self) -> i32 {
        if !self.wc_valid {
            return 1;
        }

        let w = unsafe { libc::wcwidth(self.wc) };
        if w >= 0 {
            w
        } else if unsafe { libc::iswcntrl(self.wc as u32) } != 0 {
            0
        } else {
            1
        }
    }
}

// Safe wrapper for MbChar operations
pub struct SafeMbChar {
    inner: MbChar,
}

impl SafeMbChar {
    pub fn new() -> Self {
        SafeMbChar {
            inner: MbChar {
                ptr: std::ptr::null(),
                bytes: 0,
                wc_valid: false,
                wc: 0,
                buf: [0; 24],
            },
        }
    }

    pub fn is_basic(&self) -> bool {
        self.inner.is_basic()
    }

    pub fn copy_from(&mut self, other: &SafeMbChar) {
        self.inner.copy_from(&other.inner)
    }

    pub fn width(&self) -> i32 {
        self.inner.width()
    }
}