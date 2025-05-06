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
    fn nettle_base16_encode_update(dst: *mut i8, length: size_t, src: *const uint8_t);
    fn nettle_base16_decode_init(ctx: *mut base16_decode_ctx);
    fn nettle_base16_decode_update(
        ctx: *mut base16_decode_ctx,
        dst_length: *mut size_t,
        dst: *mut uint8_t,
        src_length: size_t,
        src: *const i8,
    ) -> i32;
    fn nettle_base16_decode_final(ctx: *mut base16_decode_ctx) -> i32;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type nettle_armor_length_func = unsafe extern "C" fn(size_t) -> size_t;
pub type nettle_armor_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_armor_encode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut i8,
    size_t,
    *const uint8_t,
) -> size_t;
pub type nettle_armor_encode_final_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut i8,
) -> size_t;
pub type nettle_armor_decode_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *mut size_t,
    *mut uint8_t,
    size_t,
    *const i8,
) -> i32;
pub type nettle_armor_decode_final_func = unsafe extern "C" fn(*mut libc::c_void) -> i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_armor {
    pub name: *const i8,
    pub encode_context_size: u32,
    pub decode_context_size: u32,
    pub encode_final_length: u32,
    pub encode_init: Option<nettle_armor_init_func>,
    pub encode_length: Option<nettle_armor_length_func>,
    pub encode_update: Option<nettle_armor_encode_update_func>,
    pub encode_final: Option<nettle_armor_encode_final_func>,
    pub decode_init: Option<nettle_armor_init_func>,
    pub decode_length: Option<nettle_armor_length_func>,
    pub decode_update: Option<nettle_armor_decode_update_func>,
    pub decode_final: Option<nettle_armor_decode_final_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct base16_decode_ctx {
    pub word: u8,
    pub bits: u8,
}
unsafe extern "C" fn base16_encode_length(mut length: size_t) -> size_t {
    return length.wrapping_mul(2 as i32 as u64);
}
unsafe extern "C" fn base16_decode_length(mut length: size_t) -> size_t {
    return length.wrapping_add(1 as i32 as u64).wrapping_div(2 as i32 as u64);
}
unsafe extern "C" fn base16_encode_init(mut ctx: *mut libc::c_void) {}
unsafe extern "C" fn base16_encode_update_wrapper(
    mut ctx: *mut libc::c_void,
    mut dst: *mut i8,
    mut length: size_t,
    mut src: *const uint8_t,
) -> size_t {
    nettle_base16_encode_update(dst, length, src);
    return length.wrapping_mul(2 as i32 as u64);
}
unsafe extern "C" fn base16_encode_final(
    mut ctx: *mut libc::c_void,
    mut dst: *mut i8,
) -> size_t {
    return 0 as i32 as size_t;
}
#[no_mangle]
pub static mut nettle_base16: nettle_armor = unsafe {
    {
        let mut init = nettle_armor {
            name: b"base16\0" as *const u8 as *const i8,
            encode_context_size: 0 as i32 as u32,
            decode_context_size: ::core::mem::size_of::<base16_decode_ctx>() as u64
                as u32,
            encode_final_length: 0 as i32 as u32,
            encode_init: Some(base16_encode_init as nettle_armor_init_func),
            encode_length: Some(base16_encode_length as nettle_armor_length_func),
            encode_update: Some(
                base16_encode_update_wrapper as nettle_armor_encode_update_func,
            ),
            encode_final: Some(base16_encode_final as nettle_armor_encode_final_func),
            decode_init: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut base16_decode_ctx) -> ()>,
                Option<nettle_armor_init_func>,
            >(
                Some(
                    nettle_base16_decode_init
                        as unsafe extern "C" fn(*mut base16_decode_ctx) -> (),
                ),
            ),
            decode_length: Some(base16_decode_length as nettle_armor_length_func),
            decode_update: ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *mut base16_decode_ctx,
                        *mut size_t,
                        *mut uint8_t,
                        size_t,
                        *const i8,
                    ) -> i32,
                >,
                Option<nettle_armor_decode_update_func>,
            >(
                Some(
                    nettle_base16_decode_update
                        as unsafe extern "C" fn(
                            *mut base16_decode_ctx,
                            *mut size_t,
                            *mut uint8_t,
                            size_t,
                            *const i8,
                        ) -> i32,
                ),
            ),
            decode_final: ::core::mem::transmute::<
                Option<unsafe extern "C" fn(*mut base16_decode_ctx) -> i32>,
                Option<nettle_armor_decode_final_func>,
            >(
                Some(
                    nettle_base16_decode_final
                        as unsafe extern "C" fn(*mut base16_decode_ctx) -> i32,
                ),
            ),
        };
        init
    }
};