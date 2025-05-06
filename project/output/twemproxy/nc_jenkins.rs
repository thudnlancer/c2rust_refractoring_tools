#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type uint32_t = __uint32_t;
pub type __uint32_t = u32;
pub type size_t = u64;
pub type uint8_t = __uint8_t;
pub type __uint8_t = u8;
pub type uint16_t = __uint16_t;
pub type __uint16_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ptr: *const libc::c_void,
    pub i: size_t,
}
#[no_mangle]
pub unsafe extern "C" fn hash_jenkins(
    mut key: *const i8,
    mut length: size_t,
) -> uint32_t {
    let mut a: uint32_t = 0;
    let mut b: uint32_t = 0;
    let mut c: uint32_t = 0;
    let mut u: C2RustUnnamed = C2RustUnnamed {
        ptr: 0 as *const libc::c_void,
    };
    c = (0xdeadbeef as u32)
        .wrapping_add(length as uint32_t)
        .wrapping_add(13 as i32 as u32);
    b = c;
    a = b;
    u.ptr = key as *const libc::c_void;
    if u.i & 0x3 as i32 as u64 == 0 as i32 as u64 {
        let mut k: *const uint32_t = key as *const uint32_t;
        while length > 12 as i32 as u64 {
            a = (a as u32).wrapping_add(*k.offset(0 as i32 as isize)) as uint32_t
                as uint32_t;
            b = (b as u32).wrapping_add(*k.offset(1 as i32 as isize)) as uint32_t
                as uint32_t;
            c = (c as u32).wrapping_add(*k.offset(2 as i32 as isize)) as uint32_t
                as uint32_t;
            a = (a as u32).wrapping_sub(c) as uint32_t as uint32_t;
            a ^= c << 4 as i32 | c >> 32 as i32 - 4 as i32;
            c = (c as u32).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as u32).wrapping_sub(a) as uint32_t as uint32_t;
            b ^= a << 6 as i32 | a >> 32 as i32 - 6 as i32;
            a = (a as u32).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as u32).wrapping_sub(b) as uint32_t as uint32_t;
            c ^= b << 8 as i32 | b >> 32 as i32 - 8 as i32;
            b = (b as u32).wrapping_add(a) as uint32_t as uint32_t;
            a = (a as u32).wrapping_sub(c) as uint32_t as uint32_t;
            a ^= c << 16 as i32 | c >> 32 as i32 - 16 as i32;
            c = (c as u32).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as u32).wrapping_sub(a) as uint32_t as uint32_t;
            b ^= a << 19 as i32 | a >> 32 as i32 - 19 as i32;
            a = (a as u32).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as u32).wrapping_sub(b) as uint32_t as uint32_t;
            c ^= b << 4 as i32 | b >> 32 as i32 - 4 as i32;
            b = (b as u32).wrapping_add(a) as uint32_t as uint32_t;
            length = (length as u64).wrapping_sub(12 as i32 as u64) as size_t as size_t;
            k = k.offset(3 as i32 as isize);
        }
        match length {
            12 => {
                c = (c as u32).wrapping_add(*k.offset(2 as i32 as isize)) as uint32_t
                    as uint32_t;
                b = (b as u32).wrapping_add(*k.offset(1 as i32 as isize)) as uint32_t
                    as uint32_t;
                a = (a as u32).wrapping_add(*k.offset(0 as i32 as isize)) as uint32_t
                    as uint32_t;
            }
            11 => {
                c = (c as u32)
                    .wrapping_add(*k.offset(2 as i32 as isize) & 0xffffff as i32 as u32)
                    as uint32_t as uint32_t;
                b = (b as u32).wrapping_add(*k.offset(1 as i32 as isize)) as uint32_t
                    as uint32_t;
                a = (a as u32).wrapping_add(*k.offset(0 as i32 as isize)) as uint32_t
                    as uint32_t;
            }
            10 => {
                c = (c as u32)
                    .wrapping_add(*k.offset(2 as i32 as isize) & 0xffff as i32 as u32)
                    as uint32_t as uint32_t;
                b = (b as u32).wrapping_add(*k.offset(1 as i32 as isize)) as uint32_t
                    as uint32_t;
                a = (a as u32).wrapping_add(*k.offset(0 as i32 as isize)) as uint32_t
                    as uint32_t;
            }
            9 => {
                c = (c as u32)
                    .wrapping_add(*k.offset(2 as i32 as isize) & 0xff as i32 as u32)
                    as uint32_t as uint32_t;
                b = (b as u32).wrapping_add(*k.offset(1 as i32 as isize)) as uint32_t
                    as uint32_t;
                a = (a as u32).wrapping_add(*k.offset(0 as i32 as isize)) as uint32_t
                    as uint32_t;
            }
            8 => {
                b = (b as u32).wrapping_add(*k.offset(1 as i32 as isize)) as uint32_t
                    as uint32_t;
                a = (a as u32).wrapping_add(*k.offset(0 as i32 as isize)) as uint32_t
                    as uint32_t;
            }
            7 => {
                b = (b as u32)
                    .wrapping_add(*k.offset(1 as i32 as isize) & 0xffffff as i32 as u32)
                    as uint32_t as uint32_t;
                a = (a as u32).wrapping_add(*k.offset(0 as i32 as isize)) as uint32_t
                    as uint32_t;
            }
            6 => {
                b = (b as u32)
                    .wrapping_add(*k.offset(1 as i32 as isize) & 0xffff as i32 as u32)
                    as uint32_t as uint32_t;
                a = (a as u32).wrapping_add(*k.offset(0 as i32 as isize)) as uint32_t
                    as uint32_t;
            }
            5 => {
                b = (b as u32)
                    .wrapping_add(*k.offset(1 as i32 as isize) & 0xff as i32 as u32)
                    as uint32_t as uint32_t;
                a = (a as u32).wrapping_add(*k.offset(0 as i32 as isize)) as uint32_t
                    as uint32_t;
            }
            4 => {
                a = (a as u32).wrapping_add(*k.offset(0 as i32 as isize)) as uint32_t
                    as uint32_t;
            }
            3 => {
                a = (a as u32)
                    .wrapping_add(*k.offset(0 as i32 as isize) & 0xffffff as i32 as u32)
                    as uint32_t as uint32_t;
            }
            2 => {
                a = (a as u32)
                    .wrapping_add(*k.offset(0 as i32 as isize) & 0xffff as i32 as u32)
                    as uint32_t as uint32_t;
            }
            1 => {
                a = (a as u32)
                    .wrapping_add(*k.offset(0 as i32 as isize) & 0xff as i32 as u32)
                    as uint32_t as uint32_t;
            }
            0 => return c,
            _ => return c,
        }
    } else if u.i & 0x1 as i32 as u64 == 0 as i32 as u64 {
        let mut k_0: *const uint16_t = key as *const uint16_t;
        let mut k8: *const uint8_t = 0 as *const uint8_t;
        while length > 12 as i32 as u64 {
            a = (a as u32)
                .wrapping_add(
                    (*k_0.offset(0 as i32 as isize) as u32)
                        .wrapping_add(
                            (*k_0.offset(1 as i32 as isize) as uint32_t) << 16 as i32,
                        ),
                ) as uint32_t as uint32_t;
            b = (b as u32)
                .wrapping_add(
                    (*k_0.offset(2 as i32 as isize) as u32)
                        .wrapping_add(
                            (*k_0.offset(3 as i32 as isize) as uint32_t) << 16 as i32,
                        ),
                ) as uint32_t as uint32_t;
            c = (c as u32)
                .wrapping_add(
                    (*k_0.offset(4 as i32 as isize) as u32)
                        .wrapping_add(
                            (*k_0.offset(5 as i32 as isize) as uint32_t) << 16 as i32,
                        ),
                ) as uint32_t as uint32_t;
            a = (a as u32).wrapping_sub(c) as uint32_t as uint32_t;
            a ^= c << 4 as i32 | c >> 32 as i32 - 4 as i32;
            c = (c as u32).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as u32).wrapping_sub(a) as uint32_t as uint32_t;
            b ^= a << 6 as i32 | a >> 32 as i32 - 6 as i32;
            a = (a as u32).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as u32).wrapping_sub(b) as uint32_t as uint32_t;
            c ^= b << 8 as i32 | b >> 32 as i32 - 8 as i32;
            b = (b as u32).wrapping_add(a) as uint32_t as uint32_t;
            a = (a as u32).wrapping_sub(c) as uint32_t as uint32_t;
            a ^= c << 16 as i32 | c >> 32 as i32 - 16 as i32;
            c = (c as u32).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as u32).wrapping_sub(a) as uint32_t as uint32_t;
            b ^= a << 19 as i32 | a >> 32 as i32 - 19 as i32;
            a = (a as u32).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as u32).wrapping_sub(b) as uint32_t as uint32_t;
            c ^= b << 4 as i32 | b >> 32 as i32 - 4 as i32;
            b = (b as u32).wrapping_add(a) as uint32_t as uint32_t;
            length = (length as u64).wrapping_sub(12 as i32 as u64) as size_t as size_t;
            k_0 = k_0.offset(6 as i32 as isize);
        }
        k8 = k_0 as *const uint8_t;
        let mut current_block_104: u64;
        match length {
            12 => {
                c = (c as u32)
                    .wrapping_add(
                        (*k_0.offset(4 as i32 as isize) as u32)
                            .wrapping_add(
                                (*k_0.offset(5 as i32 as isize) as uint32_t) << 16 as i32,
                            ),
                    ) as uint32_t as uint32_t;
                b = (b as u32)
                    .wrapping_add(
                        (*k_0.offset(2 as i32 as isize) as u32)
                            .wrapping_add(
                                (*k_0.offset(3 as i32 as isize) as uint32_t) << 16 as i32,
                            ),
                    ) as uint32_t as uint32_t;
                a = (a as u32)
                    .wrapping_add(
                        (*k_0.offset(0 as i32 as isize) as u32)
                            .wrapping_add(
                                (*k_0.offset(1 as i32 as isize) as uint32_t) << 16 as i32,
                            ),
                    ) as uint32_t as uint32_t;
                current_block_104 = 8102658916883067714;
            }
            11 => {
                c = (c as u32)
                    .wrapping_add(
                        (*k8.offset(10 as i32 as isize) as uint32_t) << 16 as i32,
                    ) as uint32_t as uint32_t;
                current_block_104 = 734694507375975489;
            }
            10 => {
                current_block_104 = 734694507375975489;
            }
            9 => {
                c = (c as u32).wrapping_add(*k8.offset(8 as i32 as isize) as u32)
                    as uint32_t as uint32_t;
                current_block_104 = 1804405408969159832;
            }
            8 => {
                current_block_104 = 1804405408969159832;
            }
            7 => {
                b = (b as u32)
                    .wrapping_add(
                        (*k8.offset(6 as i32 as isize) as uint32_t) << 16 as i32,
                    ) as uint32_t as uint32_t;
                current_block_104 = 6289042292453410006;
            }
            6 => {
                current_block_104 = 6289042292453410006;
            }
            5 => {
                b = (b as u32).wrapping_add(*k8.offset(4 as i32 as isize) as u32)
                    as uint32_t as uint32_t;
                current_block_104 = 16991020224475499786;
            }
            4 => {
                current_block_104 = 16991020224475499786;
            }
            3 => {
                a = (a as u32)
                    .wrapping_add(
                        (*k8.offset(2 as i32 as isize) as uint32_t) << 16 as i32,
                    ) as uint32_t as uint32_t;
                current_block_104 = 104452356125129868;
            }
            2 => {
                current_block_104 = 104452356125129868;
            }
            1 => {
                a = (a as u32).wrapping_add(*k8.offset(0 as i32 as isize) as u32)
                    as uint32_t as uint32_t;
                current_block_104 = 8102658916883067714;
            }
            0 => return c,
            _ => return c,
        }
        match current_block_104 {
            6289042292453410006 => {
                b = (b as u32).wrapping_add(*k_0.offset(2 as i32 as isize) as u32)
                    as uint32_t as uint32_t;
                a = (a as u32)
                    .wrapping_add(
                        (*k_0.offset(0 as i32 as isize) as u32)
                            .wrapping_add(
                                (*k_0.offset(1 as i32 as isize) as uint32_t) << 16 as i32,
                            ),
                    ) as uint32_t as uint32_t;
            }
            1804405408969159832 => {
                b = (b as u32)
                    .wrapping_add(
                        (*k_0.offset(2 as i32 as isize) as u32)
                            .wrapping_add(
                                (*k_0.offset(3 as i32 as isize) as uint32_t) << 16 as i32,
                            ),
                    ) as uint32_t as uint32_t;
                a = (a as u32)
                    .wrapping_add(
                        (*k_0.offset(0 as i32 as isize) as u32)
                            .wrapping_add(
                                (*k_0.offset(1 as i32 as isize) as uint32_t) << 16 as i32,
                            ),
                    ) as uint32_t as uint32_t;
            }
            734694507375975489 => {
                c = (c as u32).wrapping_add(*k_0.offset(4 as i32 as isize) as u32)
                    as uint32_t as uint32_t;
                b = (b as u32)
                    .wrapping_add(
                        (*k_0.offset(2 as i32 as isize) as u32)
                            .wrapping_add(
                                (*k_0.offset(3 as i32 as isize) as uint32_t) << 16 as i32,
                            ),
                    ) as uint32_t as uint32_t;
                a = (a as u32)
                    .wrapping_add(
                        (*k_0.offset(0 as i32 as isize) as u32)
                            .wrapping_add(
                                (*k_0.offset(1 as i32 as isize) as uint32_t) << 16 as i32,
                            ),
                    ) as uint32_t as uint32_t;
            }
            16991020224475499786 => {
                a = (a as u32)
                    .wrapping_add(
                        (*k_0.offset(0 as i32 as isize) as u32)
                            .wrapping_add(
                                (*k_0.offset(1 as i32 as isize) as uint32_t) << 16 as i32,
                            ),
                    ) as uint32_t as uint32_t;
            }
            104452356125129868 => {
                a = (a as u32).wrapping_add(*k_0.offset(0 as i32 as isize) as u32)
                    as uint32_t as uint32_t;
            }
            _ => {}
        }
    } else {
        let mut k_1: *const uint8_t = key as *const uint8_t;
        while length > 12 as i32 as u64 {
            a = (a as u32).wrapping_add(*k_1.offset(0 as i32 as isize) as u32)
                as uint32_t as uint32_t;
            a = (a as u32)
                .wrapping_add((*k_1.offset(1 as i32 as isize) as uint32_t) << 8 as i32)
                as uint32_t as uint32_t;
            a = (a as u32)
                .wrapping_add((*k_1.offset(2 as i32 as isize) as uint32_t) << 16 as i32)
                as uint32_t as uint32_t;
            a = (a as u32)
                .wrapping_add((*k_1.offset(3 as i32 as isize) as uint32_t) << 24 as i32)
                as uint32_t as uint32_t;
            b = (b as u32).wrapping_add(*k_1.offset(4 as i32 as isize) as u32)
                as uint32_t as uint32_t;
            b = (b as u32)
                .wrapping_add((*k_1.offset(5 as i32 as isize) as uint32_t) << 8 as i32)
                as uint32_t as uint32_t;
            b = (b as u32)
                .wrapping_add((*k_1.offset(6 as i32 as isize) as uint32_t) << 16 as i32)
                as uint32_t as uint32_t;
            b = (b as u32)
                .wrapping_add((*k_1.offset(7 as i32 as isize) as uint32_t) << 24 as i32)
                as uint32_t as uint32_t;
            c = (c as u32).wrapping_add(*k_1.offset(8 as i32 as isize) as u32)
                as uint32_t as uint32_t;
            c = (c as u32)
                .wrapping_add((*k_1.offset(9 as i32 as isize) as uint32_t) << 8 as i32)
                as uint32_t as uint32_t;
            c = (c as u32)
                .wrapping_add((*k_1.offset(10 as i32 as isize) as uint32_t) << 16 as i32)
                as uint32_t as uint32_t;
            c = (c as u32)
                .wrapping_add((*k_1.offset(11 as i32 as isize) as uint32_t) << 24 as i32)
                as uint32_t as uint32_t;
            a = (a as u32).wrapping_sub(c) as uint32_t as uint32_t;
            a ^= c << 4 as i32 | c >> 32 as i32 - 4 as i32;
            c = (c as u32).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as u32).wrapping_sub(a) as uint32_t as uint32_t;
            b ^= a << 6 as i32 | a >> 32 as i32 - 6 as i32;
            a = (a as u32).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as u32).wrapping_sub(b) as uint32_t as uint32_t;
            c ^= b << 8 as i32 | b >> 32 as i32 - 8 as i32;
            b = (b as u32).wrapping_add(a) as uint32_t as uint32_t;
            a = (a as u32).wrapping_sub(c) as uint32_t as uint32_t;
            a ^= c << 16 as i32 | c >> 32 as i32 - 16 as i32;
            c = (c as u32).wrapping_add(b) as uint32_t as uint32_t;
            b = (b as u32).wrapping_sub(a) as uint32_t as uint32_t;
            b ^= a << 19 as i32 | a >> 32 as i32 - 19 as i32;
            a = (a as u32).wrapping_add(c) as uint32_t as uint32_t;
            c = (c as u32).wrapping_sub(b) as uint32_t as uint32_t;
            c ^= b << 4 as i32 | b >> 32 as i32 - 4 as i32;
            b = (b as u32).wrapping_add(a) as uint32_t as uint32_t;
            length = (length as u64).wrapping_sub(12 as i32 as u64) as size_t as size_t;
            k_1 = k_1.offset(12 as i32 as isize);
        }
        let mut current_block_156: u64;
        match length {
            12 => {
                c = (c as u32)
                    .wrapping_add(
                        (*k_1.offset(11 as i32 as isize) as uint32_t) << 24 as i32,
                    ) as uint32_t as uint32_t;
                current_block_156 = 2244607058334101863;
            }
            11 => {
                current_block_156 = 2244607058334101863;
            }
            10 => {
                current_block_156 = 4424137763270823416;
            }
            9 => {
                current_block_156 = 15074530908411770723;
            }
            8 => {
                current_block_156 = 16827810745824149869;
            }
            7 => {
                current_block_156 = 5190931071520054375;
            }
            6 => {
                current_block_156 = 1331651794105737270;
            }
            5 => {
                current_block_156 = 9541762626985535097;
            }
            4 => {
                current_block_156 = 6819057449786810088;
            }
            3 => {
                current_block_156 = 14801812128931644662;
            }
            2 => {
                current_block_156 = 11743040019909382591;
            }
            1 => {
                current_block_156 = 2902982006963107671;
            }
            0 => return c,
            _ => return c,
        }
        match current_block_156 {
            2244607058334101863 => {
                c = (c as u32)
                    .wrapping_add(
                        (*k_1.offset(10 as i32 as isize) as uint32_t) << 16 as i32,
                    ) as uint32_t as uint32_t;
                current_block_156 = 4424137763270823416;
            }
            _ => {}
        }
        match current_block_156 {
            4424137763270823416 => {
                c = (c as u32)
                    .wrapping_add(
                        (*k_1.offset(9 as i32 as isize) as uint32_t) << 8 as i32,
                    ) as uint32_t as uint32_t;
                current_block_156 = 15074530908411770723;
            }
            _ => {}
        }
        match current_block_156 {
            15074530908411770723 => {
                c = (c as u32).wrapping_add(*k_1.offset(8 as i32 as isize) as u32)
                    as uint32_t as uint32_t;
                current_block_156 = 16827810745824149869;
            }
            _ => {}
        }
        match current_block_156 {
            16827810745824149869 => {
                b = (b as u32)
                    .wrapping_add(
                        (*k_1.offset(7 as i32 as isize) as uint32_t) << 24 as i32,
                    ) as uint32_t as uint32_t;
                current_block_156 = 5190931071520054375;
            }
            _ => {}
        }
        match current_block_156 {
            5190931071520054375 => {
                b = (b as u32)
                    .wrapping_add(
                        (*k_1.offset(6 as i32 as isize) as uint32_t) << 16 as i32,
                    ) as uint32_t as uint32_t;
                current_block_156 = 1331651794105737270;
            }
            _ => {}
        }
        match current_block_156 {
            1331651794105737270 => {
                b = (b as u32)
                    .wrapping_add(
                        (*k_1.offset(5 as i32 as isize) as uint32_t) << 8 as i32,
                    ) as uint32_t as uint32_t;
                current_block_156 = 9541762626985535097;
            }
            _ => {}
        }
        match current_block_156 {
            9541762626985535097 => {
                b = (b as u32).wrapping_add(*k_1.offset(4 as i32 as isize) as u32)
                    as uint32_t as uint32_t;
                current_block_156 = 6819057449786810088;
            }
            _ => {}
        }
        match current_block_156 {
            6819057449786810088 => {
                a = (a as u32)
                    .wrapping_add(
                        (*k_1.offset(3 as i32 as isize) as uint32_t) << 24 as i32,
                    ) as uint32_t as uint32_t;
                current_block_156 = 14801812128931644662;
            }
            _ => {}
        }
        match current_block_156 {
            14801812128931644662 => {
                a = (a as u32)
                    .wrapping_add(
                        (*k_1.offset(2 as i32 as isize) as uint32_t) << 16 as i32,
                    ) as uint32_t as uint32_t;
                current_block_156 = 11743040019909382591;
            }
            _ => {}
        }
        match current_block_156 {
            11743040019909382591 => {
                a = (a as u32)
                    .wrapping_add(
                        (*k_1.offset(1 as i32 as isize) as uint32_t) << 8 as i32,
                    ) as uint32_t as uint32_t;
            }
            _ => {}
        }
        a = (a as u32).wrapping_add(*k_1.offset(0 as i32 as isize) as u32) as uint32_t
            as uint32_t;
    }
    c ^= b;
    c = (c as u32).wrapping_sub(b << 14 as i32 | b >> 32 as i32 - 14 as i32) as uint32_t
        as uint32_t;
    a ^= c;
    a = (a as u32).wrapping_sub(c << 11 as i32 | c >> 32 as i32 - 11 as i32) as uint32_t
        as uint32_t;
    b ^= a;
    b = (b as u32).wrapping_sub(a << 25 as i32 | a >> 32 as i32 - 25 as i32) as uint32_t
        as uint32_t;
    c ^= b;
    c = (c as u32).wrapping_sub(b << 16 as i32 | b >> 32 as i32 - 16 as i32) as uint32_t
        as uint32_t;
    a ^= c;
    a = (a as u32).wrapping_sub(c << 4 as i32 | c >> 32 as i32 - 4 as i32) as uint32_t
        as uint32_t;
    b ^= a;
    b = (b as u32).wrapping_sub(a << 14 as i32 | a >> 32 as i32 - 14 as i32) as uint32_t
        as uint32_t;
    c ^= b;
    c = (c as u32).wrapping_sub(b << 24 as i32 | b >> 32 as i32 - 24 as i32) as uint32_t
        as uint32_t;
    return c;
}