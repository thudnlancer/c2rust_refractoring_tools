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
    fn malloc(_: u64) -> *mut libc::c_void;
    fn abort() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
}
pub type ptrdiff_t = i64;
pub type size_t = u64;
pub type uintptr_t = u64;
pub type small_t = u8;
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum C2RustUnnamed {
    sa_alignment_max = 16,
    sa_alignment_longdouble = 16,
    sa_alignment_longlong = 8,
    sa_alignment_double = 8,
    sa_alignment_long = 8,
}
impl C2RustUnnamed {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            C2RustUnnamed::sa_alignment_max => 16,
            C2RustUnnamed::sa_alignment_longdouble => 16,
            C2RustUnnamed::sa_alignment_longlong => 8,
            C2RustUnnamed::sa_alignment_double => 8,
            C2RustUnnamed::sa_alignment_long => 8,
        }
    }
    fn from_libc_c_uint(value: u32) -> C2RustUnnamed {
        match value {
            16 => C2RustUnnamed::sa_alignment_max,
            16 => C2RustUnnamed::sa_alignment_longdouble,
            8 => C2RustUnnamed::sa_alignment_longlong,
            8 => C2RustUnnamed::sa_alignment_double,
            8 => C2RustUnnamed::sa_alignment_long,
        }
    }
}
pub type idx_t = ptrdiff_t;
#[no_mangle]
pub unsafe extern "C" fn mmalloca(mut n: size_t) -> *mut libc::c_void {
    let mut alignment2_mask: uintptr_t = (2 as i32
        * C2RustUnnamed::sa_alignment_max as i32 - 1 as i32) as uintptr_t;
    let mut plus: i32 = (::core::mem::size_of::<small_t>() as u64)
        .wrapping_add(alignment2_mask) as i32;
    let mut nplus: idx_t = 0;
    let (fresh0, fresh1) = n.overflowing_add(plus);
    *(&mut nplus as *mut idx_t) = fresh0;
    if !fresh1
        && !(1 as i32 != 0 as i32
            && (if (9223372036854775807 as i64 as u64) < 18446744073709551615 as u64 {
                9223372036854775807 as i64 as u64
            } else {
                (18446744073709551615 as u64).wrapping_sub(1 as i32 as u64)
            })
                .wrapping_div(1 as i32 as u64) < nplus as u64)
    {
        let mut mem: *mut i8 = malloc(nplus as u64) as *mut i8;
        if !mem.is_null() {
            let mut umem: uintptr_t = mem as uintptr_t;
            let mut umemplus: uintptr_t = 0;
            let (fresh2, fresh3) = umem
                .overflowing_add(
                    (::core::mem::size_of::<small_t>() as u64)
                        .wrapping_add(C2RustUnnamed::sa_alignment_max as i32 as u64)
                        .wrapping_sub(1 as i32 as u64),
                );
            *(&mut umemplus as *mut uintptr_t) = fresh2;
            let mut offset: idx_t = (umemplus & !alignment2_mask)
                .wrapping_add(C2RustUnnamed::sa_alignment_max as i32 as u64)
                .wrapping_sub(umem) as idx_t;
            let mut vp: *mut libc::c_void = mem.offset(offset as isize)
                as *mut libc::c_void;
            let mut p: *mut small_t = vp as *mut small_t;
            *p.offset(-(1 as i32) as isize) = offset as small_t;
            return p as *mut libc::c_void;
        }
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn freea(mut p: *mut libc::c_void) {
    if p as uintptr_t & (C2RustUnnamed::sa_alignment_max as i32 - 1 as i32) as u64 != 0 {
        abort();
    }
    if p as uintptr_t & C2RustUnnamed::sa_alignment_max as i32 as u64 != 0 {
        let mut mem: *mut libc::c_void = (p as *mut i8)
            .offset(-(*(p as *mut small_t).offset(-(1 as i32) as isize) as i32 as isize))
            as *mut libc::c_void;
        rpl_free(mem);
    }
}