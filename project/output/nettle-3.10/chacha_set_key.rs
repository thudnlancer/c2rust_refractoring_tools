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
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chacha_ctx {
    pub state: [uint32_t; 16],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_chacha_set_key(
    mut ctx: *mut chacha_ctx,
    mut key: *const uint8_t,
) {
    static mut sigma: [uint32_t; 4] = [
        0x61707865 as i32 as uint32_t,
        0x3320646e as i32 as uint32_t,
        0x79622d32 as i32 as uint32_t,
        0x6b206574 as i32 as uint32_t,
    ];
    (*ctx).state[4 as i32 as usize] = (*key
        .offset(0 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(0 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(0 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(0 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).state[5 as i32 as usize] = (*key
        .offset(4 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(4 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(4 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(4 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).state[6 as i32 as usize] = (*key
        .offset(8 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(8 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(8 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(8 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).state[7 as i32 as usize] = (*key
        .offset(12 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(12 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(12 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(12 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).state[8 as i32 as usize] = (*key
        .offset(16 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(16 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(16 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(16 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).state[9 as i32 as usize] = (*key
        .offset(20 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(20 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(20 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(20 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).state[10 as i32 as usize] = (*key
        .offset(24 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(24 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(24 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(24 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    (*ctx).state[11 as i32 as usize] = (*key
        .offset(28 as i32 as isize)
        .offset(3 as i32 as isize) as uint32_t) << 24 as i32
        | (*key.offset(28 as i32 as isize).offset(2 as i32 as isize) as uint32_t)
            << 16 as i32
        | (*key.offset(28 as i32 as isize).offset(1 as i32 as isize) as uint32_t)
            << 8 as i32
        | *key.offset(28 as i32 as isize).offset(0 as i32 as isize) as uint32_t;
    memcpy(
        ((*ctx).state).as_mut_ptr() as *mut libc::c_void,
        sigma.as_ptr() as *const libc::c_void,
        ::core::mem::size_of::<[uint32_t; 4]>() as u64,
    );
}