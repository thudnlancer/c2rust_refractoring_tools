#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type glp_file;
    fn _glp_close(f: *mut glp_file) -> i32;
    fn _glp_format(f: *mut glp_file, fmt: *const i8, _: ...) -> i32;
    fn _glp_ioerr(f: *mut glp_file) -> i32;
    fn _glp_open(name: *const i8, mode: *const i8) -> *mut glp_file;
    fn _glp_get_err_msg() -> *const i8;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_printf(fmt: *const i8, _: ...);
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
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
pub unsafe extern "C" fn glp_write_ccdata(
    mut G: *mut glp_graph,
    mut v_wgt: i32,
    mut fname: *const i8,
) -> i32 {
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut e: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut ret: i32 = 0;
    let mut w: libc::c_double = 0.;
    if v_wgt >= 0 as i32
        && v_wgt > (*G).v_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/wrcc.c\0" as *const u8 as *const i8, 53 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_write_ccdata: v_wgt = %d; invalid offset\n\0" as *const u8
                as *const i8,
            v_wgt,
        );
    }
    glp_printf(b"Writing graph to '%s'\n\0" as *const u8 as *const i8, fname);
    fp = _glp_open(fname, b"w\0" as *const u8 as *const i8);
    if fp.is_null() {
        glp_printf(
            b"Unable to create '%s' - %s\n\0" as *const u8 as *const i8,
            fname,
            _glp_get_err_msg(),
        );
        ret = 1 as i32;
    } else {
        _glp_format(
            fp,
            b"c %s\n\0" as *const u8 as *const i8,
            (if ((*G).name).is_null() {
                b"unknown\0" as *const u8 as *const i8
            } else {
                (*G).name
            }),
        );
        count += 1;
        count;
        _glp_format(fp, b"p edge %d %d\n\0" as *const u8 as *const i8, (*G).nv, (*G).na);
        count += 1;
        count;
        if v_wgt >= 0 as i32 {
            i = 1 as i32;
            while i <= (*G).nv {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    &mut w as *mut libc::c_double as *mut libc::c_void,
                    ((*v).data as *mut i8).offset(v_wgt as isize) as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as u64,
                );
                if w != 1.0f64 {
                    _glp_format(
                        fp,
                        b"n %d %.*g\n\0" as *const u8 as *const i8,
                        i,
                        15 as i32,
                        w,
                    );
                    count += 1;
                    count;
                }
                i += 1;
                i;
            }
        }
        i = 1 as i32;
        while i <= (*G).nv {
            v = *((*G).v).offset(i as isize);
            e = (*v).out;
            while !e.is_null() {
                _glp_format(
                    fp,
                    b"e %d %d\n\0" as *const u8 as *const i8,
                    (*(*e).tail).i,
                    (*(*e).head).i,
                );
                count += 1;
                count;
                e = (*e).t_next;
            }
            i += 1;
            i;
        }
        _glp_format(fp, b"c eof\n\0" as *const u8 as *const i8);
        count += 1;
        count;
        if _glp_ioerr(fp) != 0 {
            glp_printf(
                b"Write error on '%s' - %s\n\0" as *const u8 as *const i8,
                fname,
                _glp_get_err_msg(),
            );
            ret = 1 as i32;
        } else {
            glp_printf(b"%d lines were written\n\0" as *const u8 as *const i8, count);
            ret = 0 as i32;
        }
    }
    if !fp.is_null() {
        _glp_close(fp);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glp_write_graph(
    mut G: *mut glp_graph,
    mut fname: *const i8,
) -> i32 {
    return glp_write_ccdata(G, -(1 as i32), fname);
}