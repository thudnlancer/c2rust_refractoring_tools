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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn abort() -> !;
}
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_graph {
    pub pool: *mut libc::c_void,
    pub name: *mut i8,
    pub nv_max: i32,
    pub nv: i32,
    pub na: i32,
    pub v: *mut *mut glp_vertex,
    pub index: *mut libc::c_void,
    pub v_size: i32,
    pub a_size: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_vertex {
    pub i: i32,
    pub name: *mut i8,
    pub entry: *mut libc::c_void,
    pub data: *mut libc::c_void,
    pub temp: *mut libc::c_void,
    pub in_0: *mut glp_arc,
    pub out: *mut glp_arc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_arc {
    pub tail: *mut glp_vertex,
    pub head: *mut glp_vertex,
    pub data: *mut libc::c_void,
    pub temp: *mut libc::c_void,
    pub t_prev: *mut glp_arc,
    pub t_next: *mut glp_arc,
    pub h_prev: *mut glp_arc,
    pub h_next: *mut glp_arc,
}
#[no_mangle]
pub unsafe extern "C" fn glp_rmfgen(
    mut G_: *mut glp_graph,
    mut s_: *mut i32,
    mut t_: *mut i32,
    mut a_cap_: i32,
    mut parm: *const i32,
) -> i32 {
    static mut func: [i8; 11] = unsafe {
        *::core::mem::transmute::<&[u8; 11], &[i8; 11]>(b"glp_rmfgen\0")
    };
    (G_ == G_
        || {
            glp_assert_(
                b"G_ == G_\0" as *const u8 as *const i8,
                b"api/rmfgen.c\0" as *const u8 as *const i8,
                9 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (s_ == s_
        || {
            glp_assert_(
                b"s_ == s_\0" as *const u8 as *const i8,
                b"api/rmfgen.c\0" as *const u8 as *const i8,
                10 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (t_ == t_
        || {
            glp_assert_(
                b"t_ == t_\0" as *const u8 as *const i8,
                b"api/rmfgen.c\0" as *const u8 as *const i8,
                11 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (a_cap_ == a_cap_
        || {
            glp_assert_(
                b"a_cap_ == a_cap_\0" as *const u8 as *const i8,
                b"api/rmfgen.c\0" as *const u8 as *const i8,
                12 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (parm == parm
        || {
            glp_assert_(
                b"parm == parm\0" as *const u8 as *const i8,
                b"api/rmfgen.c\0" as *const u8 as *const i8,
                13 as i32,
            );
            1 as i32 != 0
        }) as i32;
    (glp_error_(b"api/rmfgen.c\0" as *const u8 as *const i8, 14 as i32))
        .expect(
            "non-null function pointer",
        )(
        b"%s: sorry, this routine is temporarily disabled due to licensing problems\n\0"
            as *const u8 as *const i8,
        func.as_ptr(),
    );
    abort();
}