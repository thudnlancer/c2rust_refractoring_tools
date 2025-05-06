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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn nettle_aes128_set_encrypt_key(ctx: *mut aes128_ctx, key: *const uint8_t);
    fn nettle_aes128_encrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn _nettle_umac_l2_init(size: u32, k: *mut uint32_t);
    fn _nettle_umac_l3_init(size: u32, k: *mut uint64_t);
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
#[inline]
unsafe extern "C" fn nettle_bswap32_n(mut n: u32, mut x: *mut uint32_t) {
    let mut i: u32 = 0;
    i = 0 as i32 as u32;
    while i < n {
        *x.offset(i as isize) = (*x.offset(i as isize)).swap_bytes();
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn umac_kdf(
    mut aes: *mut aes128_ctx,
    mut index: u32,
    mut length: u32,
    mut dst: *mut uint8_t,
) {
    let mut block: [uint8_t; 16] = [0; 16];
    let mut count: uint64_t = 0;
    block[0 as i32 as usize] = (index as uint64_t >> 56 as i32 & 0xff as i32 as u64)
        as uint8_t;
    block[1 as i32 as usize] = (index as uint64_t >> 48 as i32 & 0xff as i32 as u64)
        as uint8_t;
    block[2 as i32 as usize] = (index as uint64_t >> 40 as i32 & 0xff as i32 as u64)
        as uint8_t;
    block[3 as i32 as usize] = (index as uint64_t >> 32 as i32 & 0xff as i32 as u64)
        as uint8_t;
    block[4 as i32 as usize] = (index as uint64_t >> 24 as i32 & 0xff as i32 as u64)
        as uint8_t;
    block[5 as i32 as usize] = (index as uint64_t >> 16 as i32 & 0xff as i32 as u64)
        as uint8_t;
    block[6 as i32 as usize] = (index as uint64_t >> 8 as i32 & 0xff as i32 as u64)
        as uint8_t;
    block[7 as i32 as usize] = (index as uint64_t & 0xff as i32 as u64) as uint8_t;
    count = 1 as i32 as uint64_t;
    while length >= 16 as i32 as u32 {
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(0 as i32 as isize) = (count
            >> 56 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(1 as i32 as isize) = (count
            >> 48 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(2 as i32 as isize) = (count
            >> 40 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(3 as i32 as isize) = (count
            >> 32 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(4 as i32 as isize) = (count
            >> 24 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(5 as i32 as isize) = (count
            >> 16 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(6 as i32 as isize) = (count
            >> 8 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(7 as i32 as isize) = (count
            & 0xff as i32 as u64) as uint8_t;
        nettle_aes128_encrypt(aes, 16 as i32 as size_t, dst, block.as_mut_ptr());
        length = length.wrapping_sub(16 as i32 as u32);
        dst = dst.offset(16 as i32 as isize);
        count = count.wrapping_add(1);
        count;
    }
    if length > 0 as i32 as u32 {
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(0 as i32 as isize) = (count
            >> 56 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(1 as i32 as isize) = (count
            >> 48 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(2 as i32 as isize) = (count
            >> 40 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(3 as i32 as isize) = (count
            >> 32 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(4 as i32 as isize) = (count
            >> 24 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(5 as i32 as isize) = (count
            >> 16 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(6 as i32 as isize) = (count
            >> 8 as i32 & 0xff as i32 as u64) as uint8_t;
        *block.as_mut_ptr().offset(8 as i32 as isize).offset(7 as i32 as isize) = (count
            & 0xff as i32 as u64) as uint8_t;
        nettle_aes128_encrypt(
            aes,
            16 as i32 as size_t,
            block.as_mut_ptr(),
            block.as_mut_ptr(),
        );
        memcpy(
            dst as *mut libc::c_void,
            block.as_mut_ptr() as *const libc::c_void,
            length as u64,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_umac_set_key(
    mut l1_key: *mut uint32_t,
    mut l2_key: *mut uint32_t,
    mut l3_key1: *mut uint64_t,
    mut l3_key2: *mut uint32_t,
    mut aes: *mut aes128_ctx,
    mut key: *const uint8_t,
    mut n: u32,
) {
    let mut size: u32 = 0;
    let mut buffer: [uint8_t; 16] = [0; 16];
    nettle_aes128_set_encrypt_key(aes, key);
    size = ((1024 as i32 / 4 as i32) as u32)
        .wrapping_add((4 as i32 as u32).wrapping_mul(n.wrapping_sub(1 as i32 as u32)));
    umac_kdf(
        aes,
        1 as i32 as u32,
        (size as u64).wrapping_mul(::core::mem::size_of::<uint32_t>() as u64) as u32,
        l1_key as *mut uint8_t,
    );
    nettle_bswap32_n(size, l1_key);
    size = (6 as i32 as u32).wrapping_mul(n);
    umac_kdf(
        aes,
        2 as i32 as u32,
        (size as u64).wrapping_mul(::core::mem::size_of::<uint32_t>() as u64) as u32,
        l2_key as *mut uint8_t,
    );
    _nettle_umac_l2_init(size, l2_key);
    size = (8 as i32 as u32).wrapping_mul(n);
    umac_kdf(
        aes,
        3 as i32 as u32,
        (size as u64).wrapping_mul(::core::mem::size_of::<uint64_t>() as u64) as u32,
        l3_key1 as *mut uint8_t,
    );
    _nettle_umac_l3_init(size, l3_key1);
    umac_kdf(
        aes,
        4 as i32 as u32,
        (n as u64).wrapping_mul(::core::mem::size_of::<uint32_t>() as u64) as u32,
        l3_key2 as *mut uint8_t,
    );
    umac_kdf(aes, 0 as i32 as u32, 16 as i32 as u32, buffer.as_mut_ptr());
    nettle_aes128_set_encrypt_key(aes, buffer.as_mut_ptr());
}