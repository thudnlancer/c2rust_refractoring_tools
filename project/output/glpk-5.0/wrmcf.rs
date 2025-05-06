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
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn _glp_close(f: *mut glp_file) -> i32;
    fn _glp_format(f: *mut glp_file, fmt: *const i8, _: ...) -> i32;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn _glp_get_err_msg() -> *const i8;
    fn _glp_open(name: *const i8, mode: *const i8) -> *mut glp_file;
    fn _glp_ioerr(f: *mut glp_file) -> i32;
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
pub unsafe extern "C" fn glp_write_mincost(
    mut G: *mut glp_graph,
    mut v_rhs: i32,
    mut a_low: i32,
    mut a_cap: i32,
    mut a_cost: i32,
    mut fname: *const i8,
) -> i32 {
    let mut fp: *mut glp_file = 0 as *mut glp_file;
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: i32 = 0;
    let mut count: i32 = 0 as i32;
    let mut ret: i32 = 0;
    let mut rhs: libc::c_double = 0.;
    let mut low: libc::c_double = 0.;
    let mut cap: libc::c_double = 0.;
    let mut cost: libc::c_double = 0.;
    if v_rhs >= 0 as i32
        && v_rhs > (*G).v_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/wrmcf.c\0" as *const u8 as *const i8, 55 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_write_mincost: v_rhs = %d; invalid offset\n\0" as *const u8
                as *const i8,
            v_rhs,
        );
    }
    if a_low >= 0 as i32
        && a_low > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/wrmcf.c\0" as *const u8 as *const i8, 58 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_write_mincost: a_low = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_low,
        );
    }
    if a_cap >= 0 as i32
        && a_cap > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/wrmcf.c\0" as *const u8 as *const i8, 61 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_write_mincost: a_cap = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_cap,
        );
    }
    if a_cost >= 0 as i32
        && a_cost > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/wrmcf.c\0" as *const u8 as *const i8, 64 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_write_mincost: a_cost = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_cost,
        );
    }
    glp_printf(
        b"Writing min-cost flow problem data to '%s'...\n\0" as *const u8 as *const i8,
        fname,
    );
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
        _glp_format(fp, b"p min %d %d\n\0" as *const u8 as *const i8, (*G).nv, (*G).na);
        count += 1;
        count;
        if v_rhs >= 0 as i32 {
            i = 1 as i32;
            while i <= (*G).nv {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    &mut rhs as *mut libc::c_double as *mut libc::c_void,
                    ((*v).data as *mut i8).offset(v_rhs as isize) as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as u64,
                );
                if rhs != 0.0f64 {
                    _glp_format(
                        fp,
                        b"n %d %.*g\n\0" as *const u8 as *const i8,
                        i,
                        15 as i32,
                        rhs,
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
            a = (*v).out;
            while !a.is_null() {
                if a_low >= 0 as i32 {
                    memcpy(
                        &mut low as *mut libc::c_double as *mut libc::c_void,
                        ((*a).data as *mut i8).offset(a_low as isize)
                            as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_double>() as u64,
                    );
                } else {
                    low = 0.0f64;
                }
                if a_cap >= 0 as i32 {
                    memcpy(
                        &mut cap as *mut libc::c_double as *mut libc::c_void,
                        ((*a).data as *mut i8).offset(a_cap as isize)
                            as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_double>() as u64,
                    );
                } else {
                    cap = 1.0f64;
                }
                if a_cost >= 0 as i32 {
                    memcpy(
                        &mut cost as *mut libc::c_double as *mut libc::c_void,
                        ((*a).data as *mut i8).offset(a_cost as isize)
                            as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_double>() as u64,
                    );
                } else {
                    cost = 0.0f64;
                }
                _glp_format(
                    fp,
                    b"a %d %d %.*g %.*g %.*g\n\0" as *const u8 as *const i8,
                    (*(*a).tail).i,
                    (*(*a).head).i,
                    15 as i32,
                    low,
                    15 as i32,
                    cap,
                    15 as i32,
                    cost,
                );
                count += 1;
                count;
                a = (*a).t_next;
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