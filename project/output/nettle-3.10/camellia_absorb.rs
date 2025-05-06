#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[no_mangle]
pub unsafe extern "C" fn _nettle_camellia_absorb(
    mut nkeys: u32,
    mut dst: *mut uint64_t,
    mut subkey: *mut uint64_t,
) {
    let mut kw2: uint64_t = 0;
    let mut kw4: uint64_t = 0;
    let mut dw: uint32_t = 0;
    let mut tl: uint32_t = 0;
    let mut tr: uint32_t = 0;
    let mut i: u32 = 0;
    kw2 = *subkey.offset(1 as i32 as isize);
    let ref mut fresh0 = *subkey.offset(3 as i32 as isize);
    *fresh0 ^= kw2;
    let ref mut fresh1 = *subkey.offset(5 as i32 as isize);
    *fresh1 ^= kw2;
    let ref mut fresh2 = *subkey.offset(7 as i32 as isize);
    *fresh2 ^= kw2;
    i = 8 as i32 as u32;
    while i < nkeys {
        kw2
            ^= (kw2 & !*subkey.offset(i.wrapping_add(1 as i32 as u32) as isize))
                << 32 as i32;
        dw = ((kw2 & *subkey.offset(i.wrapping_add(1 as i32 as u32) as isize))
            >> 32 as i32) as uint32_t;
        kw2 ^= (dw << 1 as i32 | dw >> (-(1 as i32) & 31 as i32)) as u64;
        let ref mut fresh3 = *subkey.offset(i.wrapping_add(3 as i32 as u32) as isize);
        *fresh3 ^= kw2;
        let ref mut fresh4 = *subkey.offset(i.wrapping_add(5 as i32 as u32) as isize);
        *fresh4 ^= kw2;
        let ref mut fresh5 = *subkey.offset(i.wrapping_add(7 as i32 as u32) as isize);
        *fresh5 ^= kw2;
        i = i.wrapping_add(8 as i32 as u32);
    }
    let ref mut fresh6 = *subkey.offset(i as isize);
    *fresh6 ^= kw2;
    kw4 = *subkey.offset(nkeys.wrapping_add(1 as i32 as u32) as isize);
    i = nkeys.wrapping_sub(8 as i32 as u32);
    while i > 0 as i32 as u32 {
        let ref mut fresh7 = *subkey.offset(i.wrapping_add(6 as i32 as u32) as isize);
        *fresh7 ^= kw4;
        let ref mut fresh8 = *subkey.offset(i.wrapping_add(4 as i32 as u32) as isize);
        *fresh8 ^= kw4;
        let ref mut fresh9 = *subkey.offset(i.wrapping_add(2 as i32 as u32) as isize);
        *fresh9 ^= kw4;
        kw4 ^= (kw4 & !*subkey.offset(i as isize)) << 32 as i32;
        dw = ((kw4 & *subkey.offset(i as isize)) >> 32 as i32) as uint32_t;
        kw4 ^= (dw << 1 as i32 | dw >> (-(1 as i32) & 31 as i32)) as u64;
        i = i.wrapping_sub(8 as i32 as u32);
    }
    let ref mut fresh10 = *subkey.offset(6 as i32 as isize);
    *fresh10 ^= kw4;
    let ref mut fresh11 = *subkey.offset(4 as i32 as isize);
    *fresh11 ^= kw4;
    let ref mut fresh12 = *subkey.offset(2 as i32 as isize);
    *fresh12 ^= kw4;
    let ref mut fresh13 = *subkey.offset(0 as i32 as isize);
    *fresh13 ^= kw4;
    *dst.offset(0 as i32 as isize) = *subkey.offset(0 as i32 as isize)
        ^ *subkey.offset(2 as i32 as isize);
    *dst.offset(1 as i32 as isize) = *subkey.offset(3 as i32 as isize);
    *dst.offset(2 as i32 as isize) = *subkey.offset(2 as i32 as isize)
        ^ *subkey.offset(4 as i32 as isize);
    *dst.offset(3 as i32 as isize) = *subkey.offset(3 as i32 as isize)
        ^ *subkey.offset(5 as i32 as isize);
    *dst.offset(4 as i32 as isize) = *subkey.offset(4 as i32 as isize)
        ^ *subkey.offset(6 as i32 as isize);
    *dst.offset(5 as i32 as isize) = *subkey.offset(5 as i32 as isize)
        ^ *subkey.offset(7 as i32 as isize);
    i = 8 as i32 as u32;
    while i < nkeys {
        tl = (*subkey.offset(i.wrapping_add(2 as i32 as u32) as isize) >> 32 as i32
            ^ *subkey.offset(i.wrapping_add(2 as i32 as u32) as isize)
                & !*subkey.offset(i as isize)) as uint32_t;
        dw = (tl as u64 & *subkey.offset(i as isize) >> 32 as i32) as uint32_t;
        tr = (*subkey.offset(i.wrapping_add(2 as i32 as u32) as isize)
            ^ (dw << 1 as i32 | dw >> (-(1 as i32) & 31 as i32)) as u64) as uint32_t;
        *dst.offset(i.wrapping_sub(2 as i32 as u32) as isize) = *subkey
            .offset(i.wrapping_sub(2 as i32 as u32) as isize)
            ^ ((tl as uint64_t) << 32 as i32 | tr as u64);
        *dst.offset(i.wrapping_sub(1 as i32 as u32) as isize) = *subkey
            .offset(i as isize);
        *dst.offset(i as isize) = *subkey
            .offset(i.wrapping_add(1 as i32 as u32) as isize);
        tl = (*subkey.offset(i.wrapping_sub(1 as i32 as u32) as isize) >> 32 as i32
            ^ *subkey.offset(i.wrapping_sub(1 as i32 as u32) as isize)
                & !*subkey.offset(i.wrapping_add(1 as i32 as u32) as isize)) as uint32_t;
        dw = (tl as u64
            & *subkey.offset(i.wrapping_add(1 as i32 as u32) as isize) >> 32 as i32)
            as uint32_t;
        tr = (*subkey.offset(i.wrapping_sub(1 as i32 as u32) as isize)
            ^ (dw << 1 as i32 | dw >> (-(1 as i32) & 31 as i32)) as u64) as uint32_t;
        *dst.offset(i.wrapping_add(1 as i32 as u32) as isize) = *subkey
            .offset(i.wrapping_add(3 as i32 as u32) as isize)
            ^ ((tl as uint64_t) << 32 as i32 | tr as u64);
        *dst.offset(i.wrapping_add(2 as i32 as u32) as isize) = *subkey
            .offset(i.wrapping_add(2 as i32 as u32) as isize)
            ^ *subkey.offset(i.wrapping_add(4 as i32 as u32) as isize);
        *dst.offset(i.wrapping_add(3 as i32 as u32) as isize) = *subkey
            .offset(i.wrapping_add(3 as i32 as u32) as isize)
            ^ *subkey.offset(i.wrapping_add(5 as i32 as u32) as isize);
        *dst.offset(i.wrapping_add(4 as i32 as u32) as isize) = *subkey
            .offset(i.wrapping_add(4 as i32 as u32) as isize)
            ^ *subkey.offset(i.wrapping_add(6 as i32 as u32) as isize);
        *dst.offset(i.wrapping_add(5 as i32 as u32) as isize) = *subkey
            .offset(i.wrapping_add(5 as i32 as u32) as isize)
            ^ *subkey.offset(i.wrapping_add(7 as i32 as u32) as isize);
        i = i.wrapping_add(8 as i32 as u32);
    }
    *dst.offset(i.wrapping_sub(2 as i32 as u32) as isize) = *subkey
        .offset(i.wrapping_sub(2 as i32 as u32) as isize);
    *dst.offset(i.wrapping_sub(1 as i32 as u32) as isize) = *subkey.offset(i as isize)
        ^ *subkey.offset(i.wrapping_sub(1 as i32 as u32) as isize);
}