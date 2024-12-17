#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
}
pub type ptrdiff_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
pub type small_t = libc::c_uchar;
pub const sa_alignment_max: C2RustUnnamed = 16;
pub type idx_t = ptrdiff_t;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    sa_alignment_max = 16,
    sa_alignment_longdouble = 16,
    sa_alignment_longlong = 8,
    sa_alignment_double = 8,
    sa_alignment_long = 8,
}  // end of enum

#[no_mangle]
pub unsafe extern "C" fn mmalloca(mut n: size_t) -> *mut libc::c_void {
    let mut alignment2_mask: uintptr_t = (2 as libc::c_int
        * sa_alignment_max as libc::c_int - 1 as libc::c_int) as uintptr_t;
    let mut plus: libc::c_int = (::core::mem::size_of::<small_t>() as libc::c_ulong)
        .wrapping_add(alignment2_mask) as libc::c_int;
    let mut nplus: idx_t = 0;
    let (fresh0, fresh1) = n.overflowing_add(plus);
    *(&mut nplus as *mut idx_t) = fresh0;
    if !fresh1
        && !(1 as libc::c_int != 0 as libc::c_int
            && (if (9223372036854775807 as libc::c_long as libc::c_ulong)
                < 18446744073709551615 as libc::c_ulong
            {
                9223372036854775807 as libc::c_long as libc::c_ulong
            } else {
                (18446744073709551615 as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            })
                .wrapping_div(1 as libc::c_int as libc::c_ulong)
                < nplus as libc::c_ulong)
    {
        let mut mem: *mut libc::c_char = malloc(nplus as libc::c_ulong)
            as *mut libc::c_char;
        if !mem.is_null() {
            let mut umem: uintptr_t = mem as uintptr_t;
            let mut umemplus: uintptr_t = 0;
            let (fresh2, fresh3) = umem
                .overflowing_add(
                    (::core::mem::size_of::<small_t>() as libc::c_ulong)
                        .wrapping_add(sa_alignment_max as libc::c_int as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            *(&mut umemplus as *mut uintptr_t) = fresh2;
            let mut offset: idx_t = (umemplus & !alignment2_mask)
                .wrapping_add(sa_alignment_max as libc::c_int as libc::c_ulong)
                .wrapping_sub(umem) as idx_t;
            let mut vp: *mut libc::c_void = mem.offset(offset as isize)
                as *mut libc::c_void;
            let mut p: *mut small_t = vp as *mut small_t;
            *p.offset(-(1 as libc::c_int) as isize) = offset as small_t;
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
