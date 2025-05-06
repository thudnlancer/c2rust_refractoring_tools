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
    fn _nettle_salsa20_core(dst: *mut uint32_t, src: *const uint32_t, rounds: u32);
    fn _nettle_salsa20_2core(dst: *mut uint32_t, src: *const uint32_t, rounds: u32);
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
pub struct salsa20_ctx {
    pub input: [uint32_t; 16],
}
#[no_mangle]
pub unsafe extern "C" fn _nettle_salsa20_crypt(
    mut ctx: *mut salsa20_ctx,
    mut rounds: u32,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut x: [uint32_t; 32] = [0; 32];
    while length > 64 as i32 as u64 {
        _nettle_salsa20_2core(x.as_mut_ptr(), ((*ctx).input).as_mut_ptr(), rounds);
        (*ctx).input[8 as i32 as usize] = ((*ctx).input[8 as i32 as usize] as u32)
            .wrapping_add(2 as i32 as u32) as uint32_t as uint32_t;
        (*ctx).input[9 as i32 as usize] = ((*ctx).input[9 as i32 as usize] as u32)
            .wrapping_add(
                ((*ctx).input[8 as i32 as usize] < 2 as i32 as u32) as i32 as u32,
            ) as uint32_t as uint32_t;
        if length <= (2 as i32 * 64 as i32) as u64 {
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
            (2 as i32 * 64 as i32) as size_t,
        );
        length = (length as u64).wrapping_sub((2 as i32 * 64 as i32) as u64) as size_t
            as size_t;
        dst = dst.offset((2 as i32 * 64 as i32) as isize);
        src = src.offset((2 as i32 * 64 as i32) as isize);
    }
    _nettle_salsa20_core(x.as_mut_ptr(), ((*ctx).input).as_mut_ptr(), rounds);
    (*ctx).input[8 as i32 as usize] = ((*ctx).input[8 as i32 as usize]).wrapping_add(1);
    (*ctx).input[9 as i32 as usize] = ((*ctx).input[9 as i32 as usize] as u32)
        .wrapping_add(((*ctx).input[8 as i32 as usize] == 0 as i32 as u32) as i32 as u32)
        as uint32_t as uint32_t;
    nettle_memxor3(
        dst as *mut libc::c_void,
        src as *const libc::c_void,
        x.as_mut_ptr() as *const libc::c_void,
        length,
    );
}