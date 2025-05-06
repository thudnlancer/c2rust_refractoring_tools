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
    static mut libzahl_tmp_div: [zahl; 1];
    static mut libzahl_tmp_mod: [zahl; 1];
    static mut libzahl_temp_stack: *mut *mut zahl;
    fn free(__ptr: *mut libc::c_void);
    static mut libzahl_pool: [*mut *mut zahl_char_t; 64];
    static mut libzahl_pool_n: [size_t; 64];
    static mut libzahl_tmp_divmod_ds: [z_t; 64];
    static mut libzahl_tmp_ptest_n4: z_t;
    static mut libzahl_tmp_ptest_n1: z_t;
    static mut libzahl_tmp_ptest_d: z_t;
    static mut libzahl_tmp_ptest_a: z_t;
    static mut libzahl_tmp_ptest_x: z_t;
    static mut libzahl_tmp_divmod_d: z_t;
    static mut libzahl_tmp_divmod_b: z_t;
    static mut libzahl_tmp_divmod_a: z_t;
    static mut libzahl_tmp_modsqr: z_t;
    static mut libzahl_tmp_pow_d: z_t;
    static mut libzahl_tmp_pow_c: z_t;
    static mut libzahl_tmp_pow_b: z_t;
    static mut libzahl_tmp_modmul: z_t;
    static mut libzahl_tmp_sub: z_t;
    static mut libzahl_tmp_gcd_v: z_t;
    static mut libzahl_tmp_gcd_u: z_t;
    static mut libzahl_tmp_str_rem: z_t;
    static mut libzahl_tmp_str_div: z_t;
    static mut libzahl_tmp_str_mag: z_t;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_set_up: i32;
}
pub type size_t = u64;
pub type __uint64_t = u64;
pub type uint64_t = __uint64_t;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: i32,
    pub padding__: i32,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
pub type z_t = [zahl; 1];
#[no_mangle]
pub unsafe extern "C" fn zunsetup() {
    let mut i: size_t = 0;
    if libzahl_set_up != 0 {
        libzahl_set_up = 0 as i32;
        free((*libzahl_tmp_div.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_mod.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_str_num.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_str_mag.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_str_div.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_str_rem.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_gcd_u.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_gcd_v.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_sub.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_modmul.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_pow_b.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_pow_c.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_pow_d.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_modsqr.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_divmod_a.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_divmod_b.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_divmod_d.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_ptest_x.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_ptest_a.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_ptest_d.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_ptest_n1.as_mut_ptr()).chars as *mut libc::c_void);
        free((*libzahl_tmp_ptest_n4.as_mut_ptr()).chars as *mut libc::c_void);
        i = 64 as i32 as size_t;
        loop {
            let fresh0 = i;
            i = i.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            free(
                (*(libzahl_tmp_divmod_ds[i as usize]).as_mut_ptr()).chars
                    as *mut libc::c_void,
            );
        }
        i = (::core::mem::size_of::<[*mut *mut zahl_char_t; 64]>() as u64)
            .wrapping_div(::core::mem::size_of::<*mut *mut zahl_char_t>() as u64);
        loop {
            let fresh1 = i;
            i = i.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            loop {
                let fresh2 = libzahl_pool_n[i as usize];
                libzahl_pool_n[i as usize] = (libzahl_pool_n[i as usize])
                    .wrapping_sub(1);
                if !(fresh2 != 0) {
                    break;
                }
                free(
                    *(libzahl_pool[i as usize])
                        .offset(libzahl_pool_n[i as usize] as isize) as *mut libc::c_void,
                );
            }
            free(libzahl_pool[i as usize] as *mut libc::c_void);
        }
        free(libzahl_temp_stack as *mut libc::c_void);
    }
}