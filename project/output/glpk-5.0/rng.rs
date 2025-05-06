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
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RNG {
    pub A: [i32; 56],
    pub fptr: *mut i32,
}
unsafe extern "C" fn flip_cycle(mut rand: *mut RNG) -> i32 {
    let mut ii: *mut i32 = 0 as *mut i32;
    let mut jj: *mut i32 = 0 as *mut i32;
    ii = &mut *((*rand).A).as_mut_ptr().offset(1 as i32 as isize) as *mut i32;
    jj = &mut *((*rand).A).as_mut_ptr().offset(32 as i32 as isize) as *mut i32;
    while jj <= &mut *((*rand).A).as_mut_ptr().offset(55 as i32 as isize) as *mut i32 {
        *ii = *ii - *jj & 0x7fffffff as i32;
        ii = ii.offset(1);
        ii;
        jj = jj.offset(1);
        jj;
    }
    jj = &mut *((*rand).A).as_mut_ptr().offset(1 as i32 as isize) as *mut i32;
    while ii <= &mut *((*rand).A).as_mut_ptr().offset(55 as i32 as isize) as *mut i32 {
        *ii = *ii - *jj & 0x7fffffff as i32;
        ii = ii.offset(1);
        ii;
        jj = jj.offset(1);
        jj;
    }
    (*rand).fptr = &mut *((*rand).A).as_mut_ptr().offset(54 as i32 as isize) as *mut i32;
    return (*rand).A[55 as i32 as usize];
}
#[no_mangle]
pub unsafe extern "C" fn _glp_rng_create_rand() -> *mut RNG {
    let mut rand: *mut RNG = 0 as *mut RNG;
    let mut i: i32 = 0;
    rand = glp_alloc(1 as i32, ::core::mem::size_of::<RNG>() as u64 as i32) as *mut RNG;
    (*rand).A[0 as i32 as usize] = -(1 as i32);
    i = 1 as i32;
    while i <= 55 as i32 {
        (*rand).A[i as usize] = 0 as i32;
        i += 1;
        i;
    }
    (*rand).fptr = ((*rand).A).as_mut_ptr();
    _glp_rng_init_rand(rand, 1 as i32);
    return rand;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_rng_init_rand(mut rand: *mut RNG, mut seed: i32) {
    let mut i: i32 = 0;
    let mut prev: i32 = seed;
    let mut next: i32 = 1 as i32;
    prev = prev - 0 as i32 & 0x7fffffff as i32;
    seed = prev;
    (*rand).A[55 as i32 as usize] = prev;
    i = 21 as i32;
    while i != 0 {
        (*rand).A[i as usize] = next;
        next = prev - next & 0x7fffffff as i32;
        if seed & 1 as i32 != 0 {
            seed = 0x40000000 as i32 + (seed >> 1 as i32);
        } else {
            seed >>= 1 as i32;
        }
        next = next - seed & 0x7fffffff as i32;
        prev = (*rand).A[i as usize];
        i = (i + 21 as i32) % 55 as i32;
    }
    flip_cycle(rand);
    flip_cycle(rand);
    flip_cycle(rand);
    flip_cycle(rand);
    flip_cycle(rand);
}
#[no_mangle]
pub unsafe extern "C" fn _glp_rng_next_rand(mut rand: *mut RNG) -> i32 {
    return if *(*rand).fptr >= 0 as i32 {
        let fresh0 = (*rand).fptr;
        (*rand).fptr = ((*rand).fptr).offset(-1);
        *fresh0
    } else {
        flip_cycle(rand)
    };
}
#[no_mangle]
pub unsafe extern "C" fn _glp_rng_unif_rand(mut rand: *mut RNG, mut m: i32) -> i32 {
    let mut t: u32 = (0x80000000 as u32)
        .wrapping_sub((0x80000000 as u32).wrapping_rem(m as u32));
    let mut r: i32 = 0;
    (m > 0 as i32
        || {
            glp_assert_(
                b"m > 0\0" as *const u8 as *const i8,
                b"misc/rng.c\0" as *const u8 as *const i8,
                175 as i32,
            );
            1 as i32 != 0
        }) as i32;
    loop {
        r = _glp_rng_next_rand(rand);
        if !(t <= r as u32) {
            break;
        }
    }
    return r % m;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_rng_delete_rand(mut rand: *mut RNG) {
    glp_free(rand as *mut libc::c_void);
}