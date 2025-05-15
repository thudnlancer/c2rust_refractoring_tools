use ::libc;
extern "C" {
    fn nettle_aes256_set_encrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn nettle_memxor(
        dst: *mut libc::c_void,
        src: *const libc::c_void,
        n: size_t,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [libc::c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct drbg_ctr_aes256_ctx {
    pub key: aes256_ctx,
    pub V: nettle_block16,
}
#[inline]
unsafe extern "C" fn block16_zero(mut r: *mut nettle_block16) {
    static mut zero_block: nettle_block16 = nettle_block16 { b: [0; 16] };
    *r = zero_block;
}
#[inline]
unsafe extern "C" fn block16_set(
    mut r: *mut nettle_block16,
    mut x: *const nettle_block16,
) {
    (*r).u64_0[0 as libc::c_int as usize] = (*x).u64_0[0 as libc::c_int as usize];
    (*r).u64_0[1 as libc::c_int as usize] = (*x).u64_0[1 as libc::c_int as usize];
}
unsafe extern "C" fn drbg_ctr_aes256_output(
    mut key: *const aes256_ctx,
    mut V: *mut nettle_block16,
    mut n: size_t,
    mut dst: *mut uint8_t,
) {
    while n >= 16 as libc::c_int as libc::c_ulong {
        let mut increment_i: libc::c_uint = (16 as libc::c_int - 1 as libc::c_int)
            as libc::c_uint;
        (*V).b[increment_i as usize] = ((*V).b[increment_i as usize]).wrapping_add(1);
        if (*V).b[increment_i as usize] as libc::c_int == 0 as libc::c_int {
            while increment_i > 0 as libc::c_int as libc::c_uint
                && {
                    increment_i = increment_i.wrapping_sub(1);
                    (*V)
                        .b[increment_i
                        as usize] = ((*V).b[increment_i as usize]).wrapping_add(1);
                    (*V).b[increment_i as usize] as libc::c_int == 0 as libc::c_int
                }
            {}
        }
        nettle_aes256_encrypt(
            key,
            16 as libc::c_int as size_t,
            dst,
            ((*V).b).as_mut_ptr(),
        );
        n = (n as libc::c_ulong).wrapping_sub(16 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        dst = dst.offset(16 as libc::c_int as isize);
    }
    if n > 0 as libc::c_int as libc::c_ulong {
        let mut block: nettle_block16 = nettle_block16 { b: [0; 16] };
        let mut increment_i_0: libc::c_uint = (16 as libc::c_int - 1 as libc::c_int)
            as libc::c_uint;
        (*V)
            .b[increment_i_0
            as usize] = ((*V).b[increment_i_0 as usize]).wrapping_add(1);
        if (*V).b[increment_i_0 as usize] as libc::c_int == 0 as libc::c_int {
            while increment_i_0 > 0 as libc::c_int as libc::c_uint
                && {
                    increment_i_0 = increment_i_0.wrapping_sub(1);
                    (*V)
                        .b[increment_i_0
                        as usize] = ((*V).b[increment_i_0 as usize]).wrapping_add(1);
                    (*V).b[increment_i_0 as usize] as libc::c_int == 0 as libc::c_int
                }
            {}
        }
        nettle_aes256_encrypt(
            key,
            16 as libc::c_int as size_t,
            (block.b).as_mut_ptr(),
            ((*V).b).as_mut_ptr(),
        );
        memcpy(
            dst as *mut libc::c_void,
            (block.b).as_mut_ptr() as *const libc::c_void,
            n,
        );
    }
}
unsafe extern "C" fn drbg_ctr_aes256_update(
    mut key: *mut aes256_ctx,
    mut V: *mut nettle_block16,
    mut provided_data: *const uint8_t,
) {
    let mut tmp: [nettle_block16; 3] = [nettle_block16 { b: [0; 16] }; 3];
    drbg_ctr_aes256_output(
        key,
        V,
        (16 as libc::c_int + 32 as libc::c_int) as size_t,
        (tmp[0 as libc::c_int as usize].b).as_mut_ptr(),
    );
    if !provided_data.is_null() {
        nettle_memxor(
            (tmp[0 as libc::c_int as usize].b).as_mut_ptr() as *mut libc::c_void,
            provided_data as *const libc::c_void,
            (16 as libc::c_int + 32 as libc::c_int) as size_t,
        );
    }
    nettle_aes256_set_encrypt_key(key, (tmp[0 as libc::c_int as usize].b).as_mut_ptr());
    block16_set(V, &mut *tmp.as_mut_ptr().offset(2 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn nettle_drbg_ctr_aes256_init(
    mut ctx: *mut drbg_ctr_aes256_ctx,
    mut seed_material: *mut uint8_t,
) {
    static mut zero_key: [uint8_t; 32] = [
        0 as libc::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    nettle_aes256_set_encrypt_key(&mut (*ctx).key, zero_key.as_ptr());
    block16_zero(&mut (*ctx).V);
    drbg_ctr_aes256_update(&mut (*ctx).key, &mut (*ctx).V, seed_material);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_drbg_ctr_aes256_random(
    mut ctx: *mut drbg_ctr_aes256_ctx,
    mut n: size_t,
    mut dst: *mut uint8_t,
) {
    drbg_ctr_aes256_output(&mut (*ctx).key, &mut (*ctx).V, n, dst);
    drbg_ctr_aes256_update(&mut (*ctx).key, &mut (*ctx).V, 0 as *const uint8_t);
}
