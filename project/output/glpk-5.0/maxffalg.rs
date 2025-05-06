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
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn _glp_ffalg(
        nv: i32,
        na: i32,
        tail: *const i32,
        head: *const i32,
        s: i32,
        t: i32,
        cap: *const i32,
        x: *mut i32,
        cut: *mut i8,
    );
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
pub unsafe extern "C" fn glp_maxflow_ffalg(
    mut G: *mut glp_graph,
    mut s: i32,
    mut t: i32,
    mut a_cap: i32,
    mut sol: *mut libc::c_double,
    mut a_x: i32,
    mut v_cut: i32,
) -> i32 {
    let mut current_block: u64;
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut nv: i32 = 0;
    let mut na: i32 = 0;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut flag: i32 = 0;
    let mut tail: *mut i32 = 0 as *mut i32;
    let mut head: *mut i32 = 0 as *mut i32;
    let mut cap: *mut i32 = 0 as *mut i32;
    let mut x: *mut i32 = 0 as *mut i32;
    let mut ret: i32 = 0;
    let mut cut: *mut i8 = 0 as *mut i8;
    let mut temp: libc::c_double = 0.;
    if !(1 as i32 <= s && s <= (*G).nv) {
        (glp_error_(b"api/maxffalg.c\0" as *const u8 as *const i8, 35 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_maxflow_ffalg: s = %d; source node number out of range\n\0"
                as *const u8 as *const i8,
            s,
        );
    }
    if !(1 as i32 <= t && t <= (*G).nv) {
        (glp_error_(b"api/maxffalg.c\0" as *const u8 as *const i8, 38 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_maxflow_ffalg: t = %d: sink node number out of range\n\0" as *const u8
                as *const i8,
            t,
        );
    }
    if s == t {
        (glp_error_(b"api/maxffalg.c\0" as *const u8 as *const i8, 41 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_maxflow_ffalg: s = t = %d; source and sink nodes must be distinct\n\0"
                as *const u8 as *const i8,
            s,
        );
    }
    if a_cap >= 0 as i32
        && a_cap > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/maxffalg.c\0" as *const u8 as *const i8, 44 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_maxflow_ffalg: a_cap = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_cap,
        );
    }
    if v_cut >= 0 as i32
        && v_cut > (*G).v_size - ::core::mem::size_of::<i32>() as u64 as i32
    {
        (glp_error_(b"api/maxffalg.c\0" as *const u8 as *const i8, 47 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_maxflow_ffalg: v_cut = %d; invalid offset\n\0" as *const u8
                as *const i8,
            v_cut,
        );
    }
    nv = (*G).nv;
    na = (*G).na;
    tail = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    head = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    cap = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    x = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    if v_cut < 0 as i32 {
        cut = 0 as *mut i8;
    } else {
        cut = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i8>() as u64 as i32)
            as *mut i8;
    }
    k = 0 as i32;
    i = 1 as i32;
    's_85: loop {
        if !(i <= (*G).nv) {
            current_block = 11636175345244025579;
            break;
        }
        v = *((*G).v).offset(i as isize);
        a = (*v).out;
        while !a.is_null() {
            k += 1;
            k;
            *tail.offset(k as isize) = (*(*a).tail).i;
            *head.offset(k as isize) = (*(*a).head).i;
            if *tail.offset(k as isize) == *head.offset(k as isize) {
                ret = 0x12 as i32;
                current_block = 16103803578376386848;
                break 's_85;
            } else {
                if a_cap >= 0 as i32 {
                    memcpy(
                        &mut temp as *mut libc::c_double as *mut libc::c_void,
                        ((*a).data as *mut i8).offset(a_cap as isize)
                            as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_double>() as u64,
                    );
                } else {
                    temp = 1.0f64;
                }
                if !(0.0f64 <= temp && temp <= 2147483647 as i32 as libc::c_double
                    && temp == floor(temp))
                {
                    ret = 0x12 as i32;
                    current_block = 16103803578376386848;
                    break 's_85;
                } else {
                    *cap.offset(k as isize) = temp as i32;
                    a = (*a).t_next;
                }
            }
        }
        i += 1;
        i;
    }
    match current_block {
        11636175345244025579 => {
            (k == na
                || {
                    glp_assert_(
                        b"k == na\0" as *const u8 as *const i8,
                        b"api/maxffalg.c\0" as *const u8 as *const i8,
                        84 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            _glp_ffalg(
                nv,
                na,
                tail as *const i32,
                head as *const i32,
                s,
                t,
                cap as *const i32,
                x,
                cut,
            );
            ret = 0 as i32;
            if !sol.is_null() {
                temp = 0.0f64;
                k = 1 as i32;
                while k <= na {
                    if *tail.offset(k as isize) == s {
                        temp += *x.offset(k as isize) as libc::c_double;
                    } else if *head.offset(k as isize) == s {
                        temp -= *x.offset(k as isize) as libc::c_double;
                    }
                    k += 1;
                    k;
                }
                *sol = temp;
            }
            if a_x >= 0 as i32 {
                k = 0 as i32;
                i = 1 as i32;
                while i <= (*G).nv {
                    v = *((*G).v).offset(i as isize);
                    a = (*v).out;
                    while !a.is_null() {
                        k += 1;
                        temp = *x.offset(k as isize) as libc::c_double;
                        memcpy(
                            ((*a).data as *mut i8).offset(a_x as isize)
                                as *mut libc::c_void,
                            &mut temp as *mut libc::c_double as *const libc::c_void,
                            ::core::mem::size_of::<libc::c_double>() as u64,
                        );
                        a = (*a).t_next;
                    }
                    i += 1;
                    i;
                }
            }
            if v_cut >= 0 as i32 {
                i = 1 as i32;
                while i <= (*G).nv {
                    v = *((*G).v).offset(i as isize);
                    flag = *cut.offset(i as isize) as i32;
                    memcpy(
                        ((*v).data as *mut i8).offset(v_cut as isize)
                            as *mut libc::c_void,
                        &mut flag as *mut i32 as *const libc::c_void,
                        ::core::mem::size_of::<i32>() as u64,
                    );
                    i += 1;
                    i;
                }
            }
        }
        _ => {}
    }
    glp_free(tail as *mut libc::c_void);
    glp_free(head as *mut libc::c_void);
    glp_free(cap as *mut libc::c_void);
    glp_free(x as *mut libc::c_void);
    if !cut.is_null() {
        glp_free(cut as *mut libc::c_void);
    }
    return ret;
}