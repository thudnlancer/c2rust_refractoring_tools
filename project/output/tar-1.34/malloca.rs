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
pub type size_t = u64;
pub type uintptr_t = u64;
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
            _ => panic!("Invalid value for C2RustUnnamed: {}", value),
        }
    }
}
pub type small_t = u8;
#[no_mangle]
pub unsafe extern "C" fn mmalloca(mut n: size_t) -> *mut libc::c_void {
    let mut nplus: size_t = n
        .wrapping_add(::core::mem::size_of::<small_t>() as u64)
        .wrapping_add((2 as i32 * C2RustUnnamed::sa_alignment_max as i32) as u64)
        .wrapping_sub(1 as i32 as u64);
    if nplus >= n {
        let mut mem: *mut i8 = malloc(nplus) as *mut i8;
        if !mem.is_null() {
            let mut p: *mut i8 = ((mem as uintptr_t)
                .wrapping_add(::core::mem::size_of::<small_t>() as u64)
                .wrapping_add(C2RustUnnamed::sa_alignment_max as i32 as u64)
                .wrapping_sub(1 as i32 as u64)
                & !((2 as i32 * C2RustUnnamed::sa_alignment_max as i32 - 1 as i32)
                    as uintptr_t))
                .wrapping_add(C2RustUnnamed::sa_alignment_max as i32 as u64) as *mut i8;
            *(p as *mut small_t).offset(-(1 as i32) as isize) = p.offset_from(mem) as i64
                as small_t;
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