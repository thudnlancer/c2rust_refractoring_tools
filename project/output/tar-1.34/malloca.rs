#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
pub const sa_alignment_max: C2RustUnnamed = 16;
pub type small_t = libc::c_uchar;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    sa_alignment_max = 16,
    sa_alignment_longdouble = 16,
    sa_alignment_longlong = 8,
    sa_alignment_double = 8,
    sa_alignment_long = 8,
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> libc::c_uint {
        match self {
            C2RustUnnamed::sa_alignment_max => 16,
            C2RustUnnamed::sa_alignment_longdouble => 16,
            C2RustUnnamed::sa_alignment_longlong => 8,
            C2RustUnnamed::sa_alignment_double => 8,
            C2RustUnnamed::sa_alignment_long => 8,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn mmalloca(mut n: size_t) -> *mut libc::c_void {
    let mut nplus: size_t = n
        .wrapping_add(::core::mem::size_of::<small_t>() as libc::c_ulong)
        .wrapping_add(
            (2 as libc::c_int * sa_alignment_max as libc::c_int) as libc::c_ulong,
        )
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    if nplus >= n {
        let mut mem: *mut libc::c_char = malloc(nplus) as *mut libc::c_char;
        if !mem.is_null() {
            let mut p: *mut libc::c_char = ((mem as uintptr_t)
                .wrapping_add(::core::mem::size_of::<small_t>() as libc::c_ulong)
                .wrapping_add(sa_alignment_max as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !((2 as libc::c_int * sa_alignment_max as libc::c_int
                    - 1 as libc::c_int) as uintptr_t))
                .wrapping_add(sa_alignment_max as libc::c_int as libc::c_ulong)
                as *mut libc::c_char;
            *(p as *mut small_t)
                .offset(
                    -(1 as libc::c_int) as isize,
                ) = p.offset_from(mem) as libc::c_long as small_t;
            return p as *mut libc::c_void;
        }
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn freea(mut p: *mut libc::c_void) {
    if p as uintptr_t
        & (sa_alignment_max as libc::c_int - 1 as libc::c_int) as libc::c_ulong != 0
    {
        abort();
    }
    if p as uintptr_t & sa_alignment_max as libc::c_int as libc::c_ulong != 0 {
        let mut mem: *mut libc::c_void = (p as *mut libc::c_char)
            .offset(
                -(*(p as *mut small_t).offset(-(1 as libc::c_int) as isize)
                    as libc::c_int as isize),
            ) as *mut libc::c_void;
        rpl_free(mem);
    }
}
