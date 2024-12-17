#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
}
pub type __uint8_t = libc::c_uchar;
pub type uint8_t = __uint8_t;
pub type size_t = libc::c_ulong;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum kwsub {
    kwsub_b = 5,
    kwsub_o = 4,
    kwsub_v = 3,
    kwsub_k = 2,
    kwsub_kvl = 1,
    kwsub_kv = 0,
}  // end of enum

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cbuf {
    pub string: *const libc::c_char,
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
    pub i: libc::c_int,
    pub sym: *const tinysym,
}
#[no_mangle]
pub static mut ks_revno: [libc::c_char; 16] = unsafe {
    *::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"revision number\0")
};
#[no_mangle]
pub static mut prog_diff: [libc::c_char; 14] = unsafe {
    *::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"/usr/bin/diff\0")
};
#[no_mangle]
pub static mut prog_diff3: [libc::c_char; 15] = unsafe {
    *::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"/usr/bin/diff3\0")
};
#[no_mangle]
pub static mut diff_flags: [libc::c_char; 4] = unsafe {
    *::core::mem::transmute::<&[u8; 4], &[libc::c_char; 4]>(b"-an\0")
};
#[no_mangle]
pub static mut equal_line: [libc::c_char; 79] = unsafe {
    *::core::mem::transmute::<
        &[u8; 79],
        &[libc::c_char; 79],
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
    mut start: *const libc::c_char,
) -> bool {
    return 0 as libc::c_int
        == memcmp(
            start as *const libc::c_void,
            ((*sym).bytes).as_ptr() as *const libc::c_void,
            (*sym).len as libc::c_ulong,
        );
}
static mut kwsub_pool: [uint8_t; 22] = [
    6 as libc::c_int as uint8_t,
    2 as libc::c_int as uint8_t,
    'k' as i32 as uint8_t,
    'v' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    3 as libc::c_int as uint8_t,
    'k' as i32 as uint8_t,
    'v' as i32 as uint8_t,
    'l' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    1 as libc::c_int as uint8_t,
    'k' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    1 as libc::c_int as uint8_t,
    'v' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    1 as libc::c_int as uint8_t,
    'o' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    1 as libc::c_int as uint8_t,
    'b' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
];
static mut keyword_pool: [uint8_t; 80] = [
    11 as libc::c_int as uint8_t,
    6 as libc::c_int as uint8_t,
    'A' as i32 as uint8_t,
    'u' as i32 as uint8_t,
    't' as i32 as uint8_t,
    'h' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    4 as libc::c_int as uint8_t,
    'D' as i32 as uint8_t,
    'a' as i32 as uint8_t,
    't' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    6 as libc::c_int as uint8_t,
    'H' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'a' as i32 as uint8_t,
    'd' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    2 as libc::c_int as uint8_t,
    'I' as i32 as uint8_t,
    'd' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    6 as libc::c_int as uint8_t,
    'L' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    'k' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    3 as libc::c_int as uint8_t,
    'L' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'g' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    4 as libc::c_int as uint8_t,
    'N' as i32 as uint8_t,
    'a' as i32 as uint8_t,
    'm' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    7 as libc::c_int as uint8_t,
    'R' as i32 as uint8_t,
    'C' as i32 as uint8_t,
    'S' as i32 as uint8_t,
    'f' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    'l' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    8 as libc::c_int as uint8_t,
    'R' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    'v' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    's' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'n' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    6 as libc::c_int as uint8_t,
    'S' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'u' as i32 as uint8_t,
    'r' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    'e' as i32 as uint8_t,
    '\0' as i32 as uint8_t,
    5 as libc::c_int as uint8_t,
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
    let mut p: *const uint8_t = pool.offset(1 as libc::c_int as isize);
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < *pool.offset(0 as libc::c_int as isize) as libc::c_ulong {
        let mut symlen: size_t = *p as size_t;
        if (*x).size == symlen
            && 0 as libc::c_int
                == memcmp(
                    p.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    (*x).string as *const libc::c_void,
                    symlen,
                )
        {
            (*found).i = i as libc::c_int;
            (*found).sym = p as *const tinysym;
            return 1 as libc::c_int != 0;
        }
        p = p
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(symlen)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    return 0 as libc::c_int != 0;
}
#[no_mangle]
pub unsafe extern "C" fn recognize_kwsub(mut x: *const cbuf) -> libc::c_int {
    let mut found: pool_found = pool_found {
        i: 0,
        sym: 0 as *const tinysym,
    };
    return if pool_lookup(kwsub_pool.as_ptr(), x, &mut found) as libc::c_int != 0 {
        found.i
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn str2expmode(mut s: *const libc::c_char) -> libc::c_int {
    let x: cbuf = {
        let mut init = cbuf { string: s, size: strlen(s) };
        init
    };
    return recognize_kwsub(&x);
}
#[no_mangle]
pub unsafe extern "C" fn kwsub_string(mut i: kwsub) -> *const libc::c_char {
    let mut count: size_t = kwsub_pool[0 as libc::c_int as usize] as size_t;
    let mut symlen: size_t = 0;
    let mut p: *const uint8_t = kwsub_pool.as_ptr().offset(1 as libc::c_int as isize);
    while i as libc::c_uint != 0
        && {
            count = count.wrapping_sub(1);
            count != 0
        }
    {
        symlen = *p as size_t;
        p = p
            .offset(
                (1 as libc::c_int as libc::c_ulong)
                    .wrapping_add(symlen)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        i -= 1;
        i;
    }
    return if i as libc::c_uint != 0 {
        0 as *const libc::c_char
    } else {
        p.offset(1 as libc::c_int as isize) as *const libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn recognize_keyword(
    mut string: *const libc::c_char,
    mut found: *mut pool_found,
) -> bool {
    let delims: [libc::c_char; 3] = [
        '$' as i32 as libc::c_char,
        ':' as i32 as libc::c_char,
        '\0' as i32 as libc::c_char,
    ];
    let mut limit: size_t = strcspn(string, delims.as_ptr());
    let x: cbuf = {
        let mut init = cbuf {
            string: string,
            size: limit,
        };
        init
    };
    return ('$' as i32 == *string.offset(limit as isize) as libc::c_int
        || ':' as i32 == *string.offset(limit as isize) as libc::c_int)
        && pool_lookup(keyword_pool.as_ptr(), &x, found) as libc::c_int != 0;
}
unsafe extern "C" fn run_static_initializers() {
    tiny_ciklog = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 23]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<
                &[u8; 23],
                &mut [uint8_t; 23],
            >(b"checked in with -k by \0"),
        };
        init
    };
    tiny_access = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 7], &mut [uint8_t; 7]>(b"access\0"),
        };
        init
    };
    tiny_author = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 7], &mut [uint8_t; 7]>(b"author\0"),
        };
        init
    };
    tiny_branch = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 7], &mut [uint8_t; 7]>(b"branch\0"),
        };
        init
    };
    tiny_branches = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 9], &mut [uint8_t; 9]>(b"branches\0"),
        };
        init
    };
    tiny_comment = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 8], &mut [uint8_t; 8]>(b"comment\0"),
        };
        init
    };
    tiny_commitid = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 9]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 9], &mut [uint8_t; 9]>(b"commitid\0"),
        };
        init
    };
    tiny_date = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(b"date\0"),
        };
        init
    };
    tiny_desc = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(b"desc\0"),
        };
        init
    };
    tiny_expand = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 7], &mut [uint8_t; 7]>(b"expand\0"),
        };
        init
    };
    tiny_head = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(b"head\0"),
        };
        init
    };
    tiny_integrity = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<
                &[u8; 10],
                &mut [uint8_t; 10],
            >(b"integrity\0"),
        };
        init
    };
    tiny_locks = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 6], &mut [uint8_t; 6]>(b"locks\0"),
        };
        init
    };
    tiny_log = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 4], &mut [uint8_t; 4]>(b"log\0"),
        };
        init
    };
    tiny_next = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 5], &mut [uint8_t; 5]>(b"next\0"),
        };
        init
    };
    tiny_state = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 6], &mut [uint8_t; 6]>(b"state\0"),
        };
        init
    };
    tiny_strict = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 7], &mut [uint8_t; 7]>(b"strict\0"),
        };
        init
    };
    tiny_symbols = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
            bytes: *::core::mem::transmute::<&[u8; 8], &mut [uint8_t; 8]>(b"symbols\0"),
        };
        init
    };
    tiny_text = {
        let mut init = tinysym {
            len: (::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as uint8_t,
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
