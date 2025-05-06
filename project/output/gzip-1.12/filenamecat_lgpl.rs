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
    fn strlen(_: *const i8) -> u64;
    fn mempcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn last_component(filename: *const i8) -> *mut i8;
    fn base_len(filename: *const i8) -> size_t;
}
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn mfile_name_concat(
    mut dir: *const i8,
    mut base: *const i8,
    mut base_in_result: *mut *mut i8,
) -> *mut i8 {
    let mut dirbase: *const i8 = last_component(dir);
    let mut dirbaselen: size_t = base_len(dirbase);
    let mut dirlen: size_t = (dirbase.offset_from(dir) as i64 as u64)
        .wrapping_add(dirbaselen);
    let mut baselen: size_t = strlen(base);
    let mut sep: i8 = '\0' as i32 as i8;
    if dirbaselen != 0 {
        if !(*dir.offset(dirlen.wrapping_sub(1 as i32 as u64) as isize) as i32
            == '/' as i32) && !(*base as i32 == '/' as i32)
        {
            sep = '/' as i32 as i8;
        }
    } else if *base as i32 == '/' as i32 {
        sep = '.' as i32 as i8;
    }
    let mut p_concat: *mut i8 = malloc(
        dirlen
            .wrapping_add((sep as i32 != '\0' as i32) as i32 as u64)
            .wrapping_add(baselen)
            .wrapping_add(1 as i32 as u64),
    ) as *mut i8;
    if p_concat.is_null() {
        return 0 as *mut i8;
    }
    let mut p: *mut i8 = 0 as *mut i8;
    p = mempcpy(p_concat as *mut libc::c_void, dir as *const libc::c_void, dirlen)
        as *mut i8;
    *p = sep;
    p = p.offset((sep as i32 != '\0' as i32) as i32 as isize);
    if !base_in_result.is_null() {
        *base_in_result = p;
    }
    p = mempcpy(p as *mut libc::c_void, base as *const libc::c_void, baselen) as *mut i8;
    *p = '\0' as i32 as i8;
    return p_concat;
}