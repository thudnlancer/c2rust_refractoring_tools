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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
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
pub unsafe extern "C" fn glp_check_asnprob(
    mut G: *mut glp_graph,
    mut v_set: i32,
) -> i32 {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut ret: i32 = 0 as i32;
    if v_set >= 0 as i32
        && v_set > (*G).v_size - ::core::mem::size_of::<i32>() as u64 as i32
    {
        (glp_error_(b"api/ckasn.c\0" as *const u8 as *const i8, 43 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_check_asnprob: v_set = %d; invalid offset\n\0" as *const u8
                as *const i8,
            v_set,
        );
    }
    i = 1 as i32;
    while i <= (*G).nv {
        v = *((*G).v).offset(i as isize);
        if v_set >= 0 as i32 {
            memcpy(
                &mut k as *mut i32 as *mut libc::c_void,
                ((*v).data as *mut i8).offset(v_set as isize) as *const libc::c_void,
                ::core::mem::size_of::<i32>() as u64,
            );
            if k == 0 as i32 {
                if !((*v).in_0).is_null() {
                    ret = 1 as i32;
                    break;
                }
            } else if k == 1 as i32 {
                if !((*v).out).is_null() {
                    ret = 2 as i32;
                    break;
                }
            } else {
                ret = 3 as i32;
                break;
            }
        } else if !((*v).in_0).is_null() && !((*v).out).is_null() {
            ret = 4 as i32;
            break;
        }
        i += 1;
        i;
    }
    return ret;
}