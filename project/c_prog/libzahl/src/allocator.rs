use ::libc;
use core::arch::asm;
extern "C" {
    fn zfree(_: *mut zahl);
    fn __errno_location() -> *mut libc::c_int;
    static mut libzahl_jmp_buf: jmp_buf;
    static mut libzahl_temp_allocation: *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    static mut libzahl_temp_stack_head: *mut *mut zahl;
    static mut libzahl_temp_stack: *mut *mut zahl;
    static mut libzahl_error: libc::c_int;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    static mut libzahl_pool_n: [size_t; 64];
    static mut libzahl_pool: [*mut *mut zahl_char_t; 64];
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
}
pub type __jmp_buf = [libc::c_long; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: libc::c_int,
    pub padding__: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
#[inline]
unsafe extern "C" fn libzahl_memfailure() {
    if *__errno_location() == 0 {
        *__errno_location() = 2 as libc::c_int;
    }
    libzahl_failure(*__errno_location());
}
unsafe extern "C" fn libzahl_failure(mut error: libc::c_int) {
    libzahl_error = error;
    if !libzahl_temp_stack.is_null() {
        while libzahl_temp_stack_head != libzahl_temp_stack {
            libzahl_temp_stack_head = libzahl_temp_stack_head.offset(-1);
            zfree(*libzahl_temp_stack_head);
        }
    }
    free(libzahl_temp_allocation);
    libzahl_temp_allocation = 0 as *mut libc::c_void;
    longjmp(libzahl_jmp_buf.as_mut_ptr(), 1 as libc::c_int);
}
#[inline]
unsafe extern "C" fn libzahl_memcpy(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d
                .offset(
                    (20 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((20 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 6889638665495251059;
        }
        19 => {
            current_block_42 = 6889638665495251059;
        }
        18 => {
            current_block_42 = 8090808381109473009;
        }
        17 => {
            current_block_42 = 2982102097796752862;
        }
        16 => {
            current_block_42 = 13548460290820688580;
        }
        15 => {
            current_block_42 = 6092851763900975673;
        }
        14 => {
            current_block_42 = 9649840228698225878;
        }
        13 => {
            current_block_42 = 3789886231521795873;
        }
        12 => {
            current_block_42 = 16799923664198218091;
        }
        11 => {
            current_block_42 = 8063720752635849591;
        }
        10 => {
            current_block_42 = 4137182613802644463;
        }
        9 => {
            current_block_42 = 1173833415142193473;
        }
        8 => {
            current_block_42 = 11758535441039906730;
        }
        7 => {
            current_block_42 = 16488618047263812036;
        }
        6 => {
            current_block_42 = 11026915586042411209;
        }
        5 => {
            current_block_42 = 15572653368432836304;
        }
        4 => {
            current_block_42 = 12286440525352895828;
        }
        3 => {
            current_block_42 = 12533097940262452874;
        }
        2 => {
            current_block_42 = 14322856256522612491;
        }
        1 => {
            current_block_42 = 546901562697472104;
        }
        0 => {
            current_block_42 = 1836292691772056875;
        }
        _ => {
            let mut t: zahl_char_t = 0;
            asm!(
                "\n    shlq $3, {3}\n    addq {1}, {3}\n 1:\n    movq 0({2}), {0}\n    movq {0}, 0({1})\n    movq 8({2}), {0}\n    movq {0}, 8({1})\n    movq 16({2}), {0}\n    movq {0}, 16({1})\n    movq 24({2}), {0}\n    movq {0}, 24({1})\n    addq $32, {2}\n    addq $32, {1}\n    cmpq {3}, {1}\n    jl 1b",
                lateout(reg) t, inlateout(reg) d, inlateout(reg) s, inlateout(reg) n,
                options(preserves_flags, att_syntax)
            );
            current_block_42 = 1836292691772056875;
        }
    }
    match current_block_42 {
        6889638665495251059 => {
            *d
                .offset(
                    (19 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((19 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 8090808381109473009;
        }
        _ => {}
    }
    match current_block_42 {
        8090808381109473009 => {
            *d
                .offset(
                    (18 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((18 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 2982102097796752862;
        }
        _ => {}
    }
    match current_block_42 {
        2982102097796752862 => {
            *d
                .offset(
                    (17 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((17 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 13548460290820688580;
        }
        _ => {}
    }
    match current_block_42 {
        13548460290820688580 => {
            *d
                .offset(
                    (16 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((16 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 6092851763900975673;
        }
        _ => {}
    }
    match current_block_42 {
        6092851763900975673 => {
            *d
                .offset(
                    (15 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((15 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 9649840228698225878;
        }
        _ => {}
    }
    match current_block_42 {
        9649840228698225878 => {
            *d
                .offset(
                    (14 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((14 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 3789886231521795873;
        }
        _ => {}
    }
    match current_block_42 {
        3789886231521795873 => {
            *d
                .offset(
                    (13 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((13 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 16799923664198218091;
        }
        _ => {}
    }
    match current_block_42 {
        16799923664198218091 => {
            *d
                .offset(
                    (12 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((12 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 8063720752635849591;
        }
        _ => {}
    }
    match current_block_42 {
        8063720752635849591 => {
            *d
                .offset(
                    (11 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 4137182613802644463;
        }
        _ => {}
    }
    match current_block_42 {
        4137182613802644463 => {
            *d
                .offset(
                    (10 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((10 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 1173833415142193473;
        }
        _ => {}
    }
    match current_block_42 {
        1173833415142193473 => {
            *d
                .offset(
                    (9 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((9 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11758535441039906730;
        }
        _ => {}
    }
    match current_block_42 {
        11758535441039906730 => {
            *d
                .offset(
                    (8 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((8 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 16488618047263812036;
        }
        _ => {}
    }
    match current_block_42 {
        16488618047263812036 => {
            *d
                .offset(
                    (7 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((7 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11026915586042411209;
        }
        _ => {}
    }
    match current_block_42 {
        11026915586042411209 => {
            *d
                .offset(
                    (6 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((6 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 15572653368432836304;
        }
        _ => {}
    }
    match current_block_42 {
        15572653368432836304 => {
            *d
                .offset(
                    (5 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((5 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 12286440525352895828;
        }
        _ => {}
    }
    match current_block_42 {
        12286440525352895828 => {
            *d
                .offset(
                    (4 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((4 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 12533097940262452874;
        }
        _ => {}
    }
    match current_block_42 {
        12533097940262452874 => {
            *d
                .offset(
                    (3 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((3 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 14322856256522612491;
        }
        _ => {}
    }
    match current_block_42 {
        14322856256522612491 => {
            *d
                .offset(
                    (2 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 546901562697472104;
        }
        _ => {}
    }
    match current_block_42 {
        546901562697472104 => {
            *d
                .offset(
                    (1 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((1 as libc::c_int - 1 as libc::c_int) as isize);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn libzahl_realloc(mut a: *mut zahl, mut need: size_t) {
    let mut i: size_t = 0;
    let mut new_size: size_t = 1 as libc::c_int as size_t;
    let mut new: *mut zahl_char_t = 0 as *mut zahl_char_t;
    i = (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(::core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        .wrapping_sub(need.leading_zeros() as i32 as size_t);
    new_size <<= i;
    if (new_size != need) as libc::c_int as libc::c_long != 0 {
        i = (i as libc::c_ulong).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        new_size <<= 1 as libc::c_int;
    }
    if (libzahl_pool_n[i as usize] != 0) as libc::c_int as libc::c_long != 0 {
        libzahl_pool_n[i as usize] = (libzahl_pool_n[i as usize]).wrapping_sub(1);
        libzahl_pool_n[i as usize];
        new = *(libzahl_pool[i as usize]).offset(libzahl_pool_n[i as usize] as isize);
        libzahl_memcpy(new, (*a).chars, (*a).alloced);
        zfree(a);
        (*a).chars = new;
    } else {
        (*a)
            .chars = realloc(
            (*a).chars as *mut libc::c_void,
            new_size
                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<zahl_char_t>() as libc::c_ulong),
        ) as *mut zahl_char_t;
        if ((*a).chars).is_null() as libc::c_int as libc::c_long != 0 {
            libzahl_memfailure();
        }
    }
    (*a).alloced = new_size;
}
