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
    fn _nettle_chacha_core(dst: *mut uint32_t, src: *const uint32_t, rounds: u32);
    fn nettle_memxor3(
        dst: *mut libc::c_void,
        a: *const libc::c_void,
        b: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = u64;
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
pub unsafe extern "C" fn nettle_chacha_crypt(
    mut ctx: *mut chacha_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length == 0 {
        return;
    }
    loop {
        let mut x: [uint32_t; 16] = [0; 16];
        _nettle_chacha_core(
            x.as_mut_ptr(),
            ((*ctx).state).as_mut_ptr(),
            20 as i32 as u32,
        );
        (*ctx).state[12 as i32 as usize] = ((*ctx).state[12 as i32 as usize])
            .wrapping_add(1);
        (*ctx).state[13 as i32 as usize] = ((*ctx).state[13 as i32 as usize] as u32)
            .wrapping_add(
                ((*ctx).state[12 as i32 as usize] == 0 as i32 as u32) as i32 as u32,
            ) as uint32_t as uint32_t;
        if length <= 64 as i32 as u64 {
            nettle_memxor3(
                dst as *mut libc::c_void,
                src as *const libc::c_void,
                x.as_mut_ptr() as *const libc::c_void,
                length,
            );
            return;
        }
        nettle_memxor3(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            x.as_mut_ptr() as *const libc::c_void,
            64 as i32 as size_t,
        );
        length = (length as u64).wrapping_sub(64 as i32 as u64) as size_t as size_t;
        dst = dst.offset(64 as i32 as isize);
        src = src.offset(64 as i32 as isize);
    };
}
#[no_mangle]
pub unsafe extern "C" fn nettle_chacha_crypt32(
    mut ctx: *mut chacha_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    if length == 0 {
        return;
    }
    loop {
        let mut x: [uint32_t; 16] = [0; 16];
        _nettle_chacha_core(
            x.as_mut_ptr(),
            ((*ctx).state).as_mut_ptr(),
            20 as i32 as u32,
        );
        (*ctx).state[12 as i32 as usize] = ((*ctx).state[12 as i32 as usize])
            .wrapping_add(1);
        (*ctx).state[12 as i32 as usize];
        if length <= 64 as i32 as u64 {
            nettle_memxor3(
                dst as *mut libc::c_void,
                src as *const libc::c_void,
                x.as_mut_ptr() as *const libc::c_void,
                length,
            );
            return;
        }
        nettle_memxor3(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            x.as_mut_ptr() as *const libc::c_void,
            64 as i32 as size_t,
        );
        length = (length as u64).wrapping_sub(64 as i32 as u64) as size_t as size_t;
        dst = dst.offset(64 as i32 as isize);
        src = src.offset(64 as i32 as isize);
    };
}