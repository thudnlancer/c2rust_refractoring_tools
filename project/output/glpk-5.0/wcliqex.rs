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
    fn floor(_: libc::c_double) -> libc::c_double;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_wclique(n: i32, w: *const i32, a: *const u8, ind: *mut i32) -> i32;
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
unsafe extern "C" fn set_edge(mut nv: i32, mut a: *mut u8, mut i: i32, mut j: i32) {
    let mut k: i32 = 0;
    (1 as i32 <= j && j < i && i <= nv
        || {
            glp_assert_(
                b"1 <= j && j < i && i <= nv\0" as *const u8 as *const i8,
                b"api/wcliqex.c\0" as *const u8 as *const i8,
                28 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k = (i - 1 as i32) * (i - 2 as i32) / 2 as i32 + (j - 1 as i32);
    let ref mut fresh0 = *a.offset((k / 8 as i32) as isize);
    *fresh0 = (*fresh0 as i32
        | ((1 as i32) << 8 as i32 - 1 as i32 - k % 8 as i32) as u8 as i32) as u8;
}
#[no_mangle]
pub unsafe extern "C" fn glp_wclique_exact(
    mut G: *mut glp_graph,
    mut v_wgt: i32,
    mut sol: *mut libc::c_double,
    mut v_set: i32,
) -> i32 {
    let mut current_block: u64;
    let mut e: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut len: i32 = 0;
    let mut x: i32 = 0;
    let mut w: *mut i32 = 0 as *mut i32;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut ret: i32 = 0 as i32;
    let mut a: *mut u8 = 0 as *mut u8;
    let mut s: libc::c_double = 0.;
    let mut t: libc::c_double = 0.;
    if v_wgt >= 0 as i32
        && v_wgt > (*G).v_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/wcliqex.c\0" as *const u8 as *const i8, 42 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_wclique_exact: v_wgt = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            v_wgt,
        );
    }
    if v_set >= 0 as i32
        && v_set > (*G).v_size - ::core::mem::size_of::<i32>() as u64 as i32
    {
        (glp_error_(b"api/wcliqex.c\0" as *const u8 as *const i8, 45 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_wclique_exact: v_set = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            v_set,
        );
    }
    if (*G).nv == 0 as i32 {
        if !sol.is_null() {
            *sol = 0.0f64;
        }
        return 0 as i32;
    }
    w = glp_alloc(1 as i32 + (*G).nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    ind = glp_alloc(1 as i32 + (*G).nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    len = (*G).nv;
    len = len * (len - 1 as i32) / 2 as i32;
    len = (len + (8 as i32 - 1 as i32)) / 8 as i32;
    a = glp_alloc(len, ::core::mem::size_of::<i8>() as u64 as i32) as *mut u8;
    memset(
        a as *mut libc::c_void,
        0 as i32,
        (len as u64).wrapping_mul(::core::mem::size_of::<i8>() as u64),
    );
    s = 0.0f64;
    i = 1 as i32;
    loop {
        if !(i <= (*G).nv) {
            current_block = 15089075282327824602;
            break;
        }
        if v_wgt >= 0 as i32 {
            memcpy(
                &mut t as *mut libc::c_double as *mut libc::c_void,
                ((**((*G).v).offset(i as isize)).data as *mut i8).offset(v_wgt as isize)
                    as *const libc::c_void,
                ::core::mem::size_of::<libc::c_double>() as u64,
            );
            if !(0.0f64 <= t && t <= 2147483647 as i32 as libc::c_double
                && t == floor(t))
            {
                ret = 0x12 as i32;
                current_block = 4910572822068081928;
                break;
            } else {
                *w.offset(i as isize) = t as i32;
            }
        } else {
            *w.offset(i as isize) = 1 as i32;
        }
        s += *w.offset(i as isize) as libc::c_double;
        i += 1;
        i;
    }
    match current_block {
        15089075282327824602 => {
            if s > 2147483647 as i32 as libc::c_double {
                ret = 0x12 as i32;
            } else {
                i = 1 as i32;
                while i <= (*G).nv {
                    e = (**((*G).v).offset(i as isize)).in_0;
                    while !e.is_null() {
                        j = (*(*e).tail).i;
                        if i > j {
                            set_edge((*G).nv, a, i, j);
                        }
                        e = (*e).h_next;
                    }
                    e = (**((*G).v).offset(i as isize)).out;
                    while !e.is_null() {
                        j = (*(*e).head).i;
                        if i > j {
                            set_edge((*G).nv, a, i, j);
                        }
                        e = (*e).t_next;
                    }
                    i += 1;
                    i;
                }
                len = _glp_wclique((*G).nv, w as *const i32, a as *const u8, ind);
                s = 0.0f64;
                k = 1 as i32;
                while k <= len {
                    i = *ind.offset(k as isize);
                    (1 as i32 <= i && i <= (*G).nv
                        || {
                            glp_assert_(
                                b"1 <= i && i <= G->nv\0" as *const u8 as *const i8,
                                b"api/wcliqex.c\0" as *const u8 as *const i8,
                                98 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    s += *w.offset(i as isize) as libc::c_double;
                    k += 1;
                    k;
                }
                if !sol.is_null() {
                    *sol = s;
                }
                if v_set >= 0 as i32 {
                    x = 0 as i32;
                    i = 1 as i32;
                    while i <= (*G).nv {
                        memcpy(
                            ((**((*G).v).offset(i as isize)).data as *mut i8)
                                .offset(v_set as isize) as *mut libc::c_void,
                            &mut x as *mut i32 as *const libc::c_void,
                            ::core::mem::size_of::<i32>() as u64,
                        );
                        i += 1;
                        i;
                    }
                    x = 1 as i32;
                    k = 1 as i32;
                    while k <= len {
                        i = *ind.offset(k as isize);
                        memcpy(
                            ((**((*G).v).offset(i as isize)).data as *mut i8)
                                .offset(v_set as isize) as *mut libc::c_void,
                            &mut x as *mut i32 as *const libc::c_void,
                            ::core::mem::size_of::<i32>() as u64,
                        );
                        k += 1;
                        k;
                    }
                }
            }
        }
        _ => {}
    }
    glp_free(w as *mut libc::c_void);
    glp_free(ind as *mut libc::c_void);
    glp_free(a as *mut libc::c_void);
    return ret;
}