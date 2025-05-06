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
    fn memchr(_: *const libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
pub type longword = u64;
pub type uintptr_t = u64;
#[no_mangle]
pub unsafe extern "C" fn memchr2(
    mut s: *const libc::c_void,
    mut c1_in: i32,
    mut c2_in: i32,
    mut n: size_t,
) -> *mut libc::c_void {
    let mut char_ptr: *const u8 = 0 as *const u8;
    let mut void_ptr: *const libc::c_void = 0 as *const libc::c_void;
    let mut longword_ptr: *const longword = 0 as *const longword;
    let mut repeated_one: longword = 0;
    let mut repeated_c1: longword = 0;
    let mut repeated_c2: longword = 0;
    let mut c1: u8 = 0;
    let mut c2: u8 = 0;
    c1 = c1_in as u8;
    c2 = c2_in as u8;
    if c1 as i32 == c2 as i32 {
        return memchr(s, c1 as i32, n);
    }
    void_ptr = s;
    while n > 0 as i32 as u64
        && (void_ptr as uintptr_t)
            .wrapping_rem(::core::mem::size_of::<longword>() as u64) != 0 as i32 as u64
    {
        char_ptr = void_ptr as *const u8;
        if *char_ptr as i32 == c1 as i32 || *char_ptr as i32 == c2 as i32 {
            return void_ptr as *mut libc::c_void;
        }
        void_ptr = char_ptr.offset(1 as i32 as isize) as *const libc::c_void;
        n = n.wrapping_sub(1);
        n;
    }
    longword_ptr = void_ptr as *const longword;
    repeated_one = 0x1010101 as i32 as longword;
    repeated_c1 = (c1 as i32 | (c1 as i32) << 8 as i32) as longword;
    repeated_c2 = (c2 as i32 | (c2 as i32) << 8 as i32) as longword;
    repeated_c1 |= repeated_c1 << 16 as i32;
    repeated_c2 |= repeated_c2 << 16 as i32;
    if (0xffffffff as u32 as u64) < -(1 as i32) as longword {
        repeated_one |= (repeated_one << 31 as i32) << 1 as i32;
        repeated_c1 |= (repeated_c1 << 31 as i32) << 1 as i32;
        repeated_c2 |= (repeated_c2 << 31 as i32) << 1 as i32;
        if (8 as i32 as u64) < ::core::mem::size_of::<longword>() as u64 {
            let mut i: size_t = 0;
            i = 64 as i32 as size_t;
            while i
                < (::core::mem::size_of::<longword>() as u64)
                    .wrapping_mul(8 as i32 as u64)
            {
                repeated_one |= repeated_one << i;
                repeated_c1 |= repeated_c1 << i;
                repeated_c2 |= repeated_c2 << i;
                i = (i as u64).wrapping_mul(2 as i32 as u64) as size_t as size_t;
            }
        }
    }
    while n >= ::core::mem::size_of::<longword>() as u64 {
        let mut longword1: longword = *longword_ptr ^ repeated_c1;
        let mut longword2: longword = *longword_ptr ^ repeated_c2;
        if (longword1.wrapping_sub(repeated_one) & !longword1
            | longword2.wrapping_sub(repeated_one) & !longword2)
            & repeated_one << 7 as i32 != 0 as i32 as u64
        {
            break;
        }
        longword_ptr = longword_ptr.offset(1);
        longword_ptr;
        n = (n as u64).wrapping_sub(::core::mem::size_of::<longword>() as u64) as size_t
            as size_t;
    }
    char_ptr = longword_ptr as *const u8;
    while n > 0 as i32 as u64 {
        if *char_ptr as i32 == c1 as i32 || *char_ptr as i32 == c2 as i32 {
            return char_ptr as *mut libc::c_void;
        }
        n = n.wrapping_sub(1);
        n;
        char_ptr = char_ptr.offset(1);
        char_ptr;
    }
    return 0 as *mut libc::c_void;
}