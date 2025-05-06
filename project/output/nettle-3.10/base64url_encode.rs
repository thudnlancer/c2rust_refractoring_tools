#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base64_encode_ctx {
    pub alphabet: *const i8,
    pub word: libc::c_ushort,
    pub bits: u8,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_base64url_encode_init(mut ctx: *mut base64_encode_ctx) {
    static mut base64url_encode_table: [i8; 64] = unsafe {
        *::core::mem::transmute::<
            &[u8; 64],
            &[i8; 64],
        >(b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_")
    };
    (*ctx).bits = 0 as i32 as u8;
    (*ctx).word = (*ctx).bits as libc::c_ushort;
    (*ctx).alphabet = base64url_encode_table.as_ptr();
}