use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
extern "C" {
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: u64) -> i32;
    fn strlen(_: *const i8) -> u64;
    fn strcspn(_: *const i8, _: *const i8) -> u64;
}
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type size_t = u64;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum kwsub {
    kwsub_b = 5,
    kwsub_o = 4,
    kwsub_v = 3,
    kwsub_k = 2,
    kwsub_kvl = 1,
    kwsub_kv = 0,
}
impl kwsub {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            kwsub::kwsub_b => 5,
            kwsub::kwsub_o => 4,
            kwsub::kwsub_v => 3,
            kwsub::kwsub_k => 2,
            kwsub::kwsub_kvl => 1,
            kwsub::kwsub_kv => 0,
        }
    }
    fn from_libc_c_uint(value: u32) -> kwsub {
        match value {
            5 => kwsub::kwsub_b,
            4 => kwsub::kwsub_o,
            3 => kwsub::kwsub_v,
            2 => kwsub::kwsub_k,
            1 => kwsub::kwsub_kvl,
            0 => kwsub::kwsub_kv,
            _ => panic!("Invalid value for kwsub: {}", value),
        }
    }
}
impl AddAssign<u32> for kwsub {
    fn add_assign(&mut self, rhs: u32) {
        *self = kwsub::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for kwsub {
    fn sub_assign(&mut self, rhs: u32) {
        *self = kwsub::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for kwsub {
    fn mul_assign(&mut self, rhs: u32) {
        *self = kwsub::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for kwsub {
    fn div_assign(&mut self, rhs: u32) {
        *self = kwsub::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for kwsub {
    fn rem_assign(&mut self, rhs: u32) {
        *self = kwsub::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for kwsub {
    type Output = kwsub;
    fn add(self, rhs: u32) -> kwsub {
        kwsub::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for kwsub {
    type Output = kwsub;
    fn sub(self, rhs: u32) -> kwsub {
        kwsub::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for kwsub {
    type Output = kwsub;
    fn mul(self, rhs: u32) -> kwsub {
        kwsub::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for kwsub {
    type Output = kwsub;
    fn div(self, rhs: u32) -> kwsub {
        kwsub::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for kwsub {
    type Output = kwsub;
    fn rem(self, rhs: u32) -> kwsub {
        kwsub::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cbuf {
    pub string: *const i8,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tinysym {
    pub len: uint8_t,
    pub bytes: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pool_found {
    pub i: i32,
    pub sym: *const tinysym,
}
#[no_mangle]
pub static mut ks_revno: [i8; 16] = unsafe {
    *::core::mem::transmute::<&[u8; 16], &[i8; 16]>(b"revision number\0")
};
#[no_mangle]
pub static mut prog_diff: [i8; 14] = unsafe {
    *::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b"/usr/bin/diff\0")
};
#[no_mangle]
pub static mut prog_diff3: [i8; 15] = unsafe {
    *::core::mem::transmute::<&[u8; 15], &[i8; 15]>(b"/usr/bin/diff3\0")
};
#[no_mangle]
pub static mut diff_flags: [i8; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[i8; 4]>(b"-an\0")
};
#[no_mangle]
pub static mut equal_line: [i8; 79] = unsafe {
    *::core::mem::transmute::<
        &[u8; 79],
        &[i8; 79],
    >(
        b"=============================================================================\n\0",
    )
};
#[no_mangle]
pub static mut tiny_ciklog: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_access: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_author: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_branch: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_branches: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_comment: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_commitid: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_date: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_desc: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_expand: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_head: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_integrity: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_locks: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_log: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_next: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_state: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_strict: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_symbols: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub static mut tiny_text: tinysym = tinysym { len: 0, bytes: [] };
#[no_mangle]
pub unsafe extern "C" fn looking_at(
    mut sym: *const tinysym,
    mut start: *const i8,
) -> bool {
    return 0 as i32
        == memcmp(
            start as *const libc::c_void,
            ((*sym).bytes).as_ptr() as *const libc::c_void,
            (*sym).len as u64,
        );
}
static mut kwsub_pool: [uint8_t; 22] = [
    6 as i32 as uint8_t,
    2 as i32 as uint8_t,
    'k' as i32 as uint8_t,
    'v' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    3 as i32 as uint8_t,
    'k' as i32 as uint8_t,
    'v' as i32 as uint8_t,
    'l' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    1 as i32 as uint8_t,
    'k' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    1 as i32 as uint8_t,
    'v' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    1 as i32 as uint8_t,
    'o' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    1 as i32 as uint8_t,
    'b' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
];
static mut keyword_pool: [uint8_t; 80] = [
    11 as i32 as uint8_t,
    6 as i32 as uint8_t,
    'A' as i32 as uint8_t,
    'u' as i32 as uint8_t,
    't' as i32 as uint8_t,
    'h' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    4 as i32 as uint8_t,
    'D' as i32 as uint8_t,
    'a' as i32 as uint8_t,
    't' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    6 as i32 as uint8_t,
    'H' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'a' as i32 as uint8_t,
    'd' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    2 as i32 as uint8_t,
    'I' as i32 as uint8_t,
    'd' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    6 as i32 as uint8_t,
    'L' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    'k' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    3 as i32 as uint8_t,
    'L' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'g' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    4 as i32 as uint8_t,
    'N' as i32 as uint8_t,
    'a' as i32 as uint8_t,
    'm' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    7 as i32 as uint8_t,
    'R' as i32 as uint8_t,
    'C' as i32 as uint8_t,
    'S' as i32 as uint8_t,
    'f' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    'l' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    8 as i32 as uint8_t,
    'R' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'v' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    's' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'n' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    6 as i32 as uint8_t,
    'S' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'u' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    5 as i32 as uint8_t,
    'S' as i32 as uint8_t,
    't' as i32 as uint8_t,
    'a' as i32 as uint8_t,
    't' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
];
unsafe extern "C" fn pool_lookup(
    mut pool: *const uint8_t,
    mut x: *const cbuf,
    mut found: *mut pool_found,
) -> bool {
    let mut p: *const uint8_t = pool.offset(1 as i32 as isize);
    let mut i: size_t = 0 as i32 as size_t;
    while i < *pool.offset(0 as i32 as isize) as u64 {
        let mut symlen: size_t = *p as size_t;
        if (*x).size == symlen
            && 0 as i32
                == memcmp(
                    p.offset(1 as i32 as isize) as *const libc::c_void,
                    (*x).string as *const libc::c_void,
                    symlen,
                )
        {
            (*found).i = i as i32;
            (*found).sym = p as *const tinysym;
            return 1 as i32 != 0;
        }
        p = p
            .offset(
                (1 as i32 as u64).wrapping_add(symlen).wrapping_add(1 as i32 as u64)
                    as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    return 0 as i32 != 0;
}
#[no_mangle]
pub unsafe extern "C" fn recognize_kwsub(mut x: *const cbuf) -> i32 {
    let mut found: pool_found = pool_found {
        i: 0,
        sym: 0 as *const tinysym,
    };
    return if pool_lookup(kwsub_pool.as_ptr(), x, &mut found) as i32 != 0 {
        found.i
    } else {
        -(1 as i32)
    };
}
#[no_mangle]
pub unsafe extern "C" fn str2expmode(mut s: *const i8) -> i32 {
    let x: cbuf = {
        let mut init = cbuf { string: s, size: strlen(s) };
        init
    };
    return recognize_kwsub(&x);
}
#[no_mangle]
pub unsafe extern "C" fn kwsub_string(mut i: kwsub) -> *const i8 {
    let mut count: size_t = kwsub_pool[0 as i32 as usize] as size_t;
    let mut symlen: size_t = 0;
    let mut p: *const uint8_t = kwsub_pool.as_ptr().offset(1 as i32 as isize);
    while i as u32 != 0
        && {
            count = count.wrapping_sub(1);
            count != 0
        }
    {
        symlen = *p as size_t;
        p = p
            .offset(
                (1 as i32 as u64).wrapping_add(symlen).wrapping_add(1 as i32 as u64)
                    as isize,
            );
        i -= 1;
        i;
    }
    return if i as u32 != 0 {
        0 as *const i8
    } else {
        p.offset(1 as i32 as isize) as *const i8
    };
}
#[no_mangle]
pub unsafe extern "C" fn recognize_keyword(
    mut string: *const i8,
    mut found: *mut pool_found,
) -> bool {
    let delims: [i8; 3] = ['$' as i32 as i8, ':' as i32 as i8, '\0' as i32 as i8];
    let mut limit: size_t = strcspn(string, delims.as_ptr());
    let x: cbuf = {
        let mut init = cbuf {
            string: string,
            size: limit,
        };
        init
    };
    return ('$' as i32 == *string.offset(limit as isize) as i32
        || ':' as i32 == *string.offset(limit as isize) as i32)
        && pool_lookup(keyword_pool.as_ptr(), &x, found) as i32 != 0;
}
unsafe extern "C" fn run_static_initializers() {
    tiny_ciklog = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 23]>() as u64)
                .wrapping_sub(1 as i32 as u64) as uint8_t,
            bytes: *::core::mem::transmute::<
                &[u8; 23],
                &mut [uint8_t; 23],
            >(b"checked in with -k by \0"),
        };
        init
    };
    tiny_access = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 7]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 7], &mut [uint8_t; 7]>(b"access\0"),
        };
        init
    };
    tiny_author = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 7]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 7], &mut [uint8_t; 7]>(b"author\0"),
        };
        init
    };
    tiny_branch = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 7]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 7], &mut [uint8_t; 7]>(b"branch\0"),
        };
        init
    };
    tiny_branches = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 9], &mut [uint8_t; 9]>(b"branches\0"),
        };
        init
    };
    tiny_comment = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 8]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 8], &mut [uint8_t; 8]>(b"comment\0"),
        };
        init
    };
    tiny_commitid = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 9]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 9], &mut [uint8_t; 9]>(b"commitid\0"),
        };
        init
    };
    tiny_date = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(b"date\0"),
        };
        init
    };
    tiny_desc = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(b"desc\0"),
        };
        init
    };
    tiny_expand = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 7]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 7], &mut [uint8_t; 7]>(b"expand\0"),
        };
        init
    };
    tiny_head = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(b"head\0"),
        };
        init
    };
    tiny_integrity = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 10]>() as u64)
                .wrapping_sub(1 as i32 as u64) as uint8_t,
            bytes: *::core::mem::transmute::<
                &[u8; 10],
                &mut [uint8_t; 10],
            >(b"integrity\0"),
        };
        init
    };
    tiny_locks = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 6], &mut [uint8_t; 6]>(b"locks\0"),
        };
        init
    };
    tiny_log = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 4]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 4], &mut [uint8_t; 4]>(b"log\0"),
        };
        init
    };
    tiny_next = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(b"next\0"),
        };
        init
    };
    tiny_state = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 6]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 6], &mut [uint8_t; 6]>(b"state\0"),
        };
        init
    };
    tiny_strict = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 7]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 7], &mut [uint8_t; 7]>(b"strict\0"),
        };
        init
    };
    tiny_symbols = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 8]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 8], &mut [uint8_t; 8]>(b"symbols\0"),
        };
        init
    };
    tiny_text = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[i8; 5]>() as u64).wrapping_sub(1 as i32 as u64)
                as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(b"text\0"),
        };
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];