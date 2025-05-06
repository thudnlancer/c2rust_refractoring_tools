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
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn glp_check_asnprob(G: *mut glp_graph, v_set: i32) -> i32;
    fn _glp_okalg(
        nv: i32,
        na: i32,
        tail: *const i32,
        head: *const i32,
        low: *const i32,
        cap: *const i32,
        cost: *const i32,
        x: *mut i32,
        pi: *mut i32,
    ) -> i32;
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
pub unsafe extern "C" fn glp_asnprob_okalg(
    mut form: i32,
    mut G: *mut glp_graph,
    mut v_set: i32,
    mut a_cost: i32,
    mut sol: *mut libc::c_double,
    mut a_x: i32,
) -> i32 {
    let mut current_block: u64;
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut nv: i32 = 0;
    let mut na: i32 = 0;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut tail: *mut i32 = 0 as *mut i32;
    let mut head: *mut i32 = 0 as *mut i32;
    let mut low: *mut i32 = 0 as *mut i32;
    let mut cap: *mut i32 = 0 as *mut i32;
    let mut cost: *mut i32 = 0 as *mut i32;
    let mut x: *mut i32 = 0 as *mut i32;
    let mut pi: *mut i32 = 0 as *mut i32;
    let mut ret: i32 = 0;
    let mut temp: libc::c_double = 0.;
    if !(form == 1 as i32 || form == 2 as i32 || form == 3 as i32) {
        (glp_error_(b"api/asnokalg.c\0" as *const u8 as *const i8, 35 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_asnprob_okalg: form = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            form,
        );
    }
    if v_set >= 0 as i32
        && v_set > (*G).v_size - ::core::mem::size_of::<i32>() as u64 as i32
    {
        (glp_error_(b"api/asnokalg.c\0" as *const u8 as *const i8, 38 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_asnprob_okalg: v_set = %d; invalid offset\n\0" as *const u8
                as *const i8,
            v_set,
        );
    }
    if a_cost >= 0 as i32
        && a_cost > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/asnokalg.c\0" as *const u8 as *const i8, 41 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_asnprob_okalg: a_cost = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_cost,
        );
    }
    if a_x >= 0 as i32 && a_x > (*G).a_size - ::core::mem::size_of::<i32>() as u64 as i32
    {
        (glp_error_(b"api/asnokalg.c\0" as *const u8 as *const i8, 44 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_asnprob_okalg: a_x = %d; invalid offset\n\0" as *const u8 as *const i8,
            a_x,
        );
    }
    if glp_check_asnprob(G, v_set) != 0 {
        return 0x12 as i32;
    }
    nv = (*G).nv + 1 as i32;
    na = (*G).na + (*G).nv;
    tail = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    head = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    low = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    cap = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    cost = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    x = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    pi = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    k = 0 as i32;
    i = 1 as i32;
    's_79: loop {
        if !(i <= (*G).nv) {
            current_block = 8693738493027456495;
            break;
        }
        v = *((*G).v).offset(i as isize);
        a = (*v).out;
        while !a.is_null() {
            k += 1;
            k;
            *tail.offset(k as isize) = (*(*a).tail).i;
            *head.offset(k as isize) = (*(*a).head).i;
            *low.offset(k as isize) = 0 as i32;
            *cap.offset(k as isize) = 1 as i32;
            if a_cost >= 0 as i32 {
                memcpy(
                    &mut temp as *mut libc::c_double as *mut libc::c_void,
                    ((*a).data as *mut i8).offset(a_cost as isize)
                        as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as u64,
                );
            } else {
                temp = 1.0f64;
            }
            if !(fabs(temp) <= 2147483647 as i32 as libc::c_double
                && temp == floor(temp))
            {
                ret = 0x12 as i32;
                current_block = 11179004639396760561;
                break 's_79;
            } else {
                *cost.offset(k as isize) = temp as i32;
                if form != 1 as i32 {
                    *cost.offset(k as isize) = -*cost.offset(k as isize);
                }
                a = (*a).t_next;
            }
        }
        i += 1;
        i;
    }
    match current_block {
        8693738493027456495 => {
            i = 1 as i32;
            while i <= (*G).nv {
                v = *((*G).v).offset(i as isize);
                k += 1;
                k;
                if ((*v).out).is_null() {
                    *tail.offset(k as isize) = i;
                    *head.offset(k as isize) = nv;
                } else if ((*v).in_0).is_null() {
                    *tail.offset(k as isize) = nv;
                    *head.offset(k as isize) = i;
                } else {
                    (v != v
                        || {
                            glp_assert_(
                                b"v != v\0" as *const u8 as *const i8,
                                b"api/asnokalg.c\0" as *const u8 as *const i8,
                                91 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                }
                *low.offset(k as isize) = if form == 3 as i32 {
                    0 as i32
                } else {
                    1 as i32
                };
                *cap.offset(k as isize) = 1 as i32;
                *cost.offset(k as isize) = 0 as i32;
                i += 1;
                i;
            }
            (k == na
                || {
                    glp_assert_(
                        b"k == na\0" as *const u8 as *const i8,
                        b"api/asnokalg.c\0" as *const u8 as *const i8,
                        96 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            ret = _glp_okalg(
                nv,
                na,
                tail as *const i32,
                head as *const i32,
                low as *const i32,
                cap as *const i32,
                cost as *const i32,
                x,
                pi,
            );
            match ret {
                0 => {
                    ret = 0 as i32;
                    current_block = 13678349939556791712;
                }
                1 => {
                    ret = 0xa as i32;
                    current_block = 13678349939556791712;
                }
                2 => {
                    ret = 0x13 as i32;
                    current_block = 11179004639396760561;
                }
                3 => {
                    ret = 0x5 as i32;
                    current_block = 11179004639396760561;
                }
                _ => {
                    (ret != ret
                        || {
                            glp_assert_(
                                b"ret != ret\0" as *const u8 as *const i8,
                                b"api/asnokalg.c\0" as *const u8 as *const i8,
                                117 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    current_block = 13678349939556791712;
                }
            }
            match current_block {
                11179004639396760561 => {}
                _ => {
                    if !sol.is_null() {
                        temp = 0.0f64;
                        k = 1 as i32;
                        while k <= na {
                            temp
                                += *cost.offset(k as isize) as libc::c_double
                                    * *x.offset(k as isize) as libc::c_double;
                            k += 1;
                            k;
                        }
                        if form != 1 as i32 {
                            temp = -temp;
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
                                k;
                                if ret == 0 as i32 {
                                    (*x.offset(k as isize) == 0 as i32
                                        || *x.offset(k as isize) == 1 as i32
                                        || {
                                            glp_assert_(
                                                b"x[k] == 0 || x[k] == 1\0" as *const u8 as *const i8,
                                                b"api/asnokalg.c\0" as *const u8 as *const i8,
                                                136 as i32,
                                            );
                                            1 as i32 != 0
                                        }) as i32;
                                }
                                memcpy(
                                    ((*a).data as *mut i8).offset(a_x as isize)
                                        as *mut libc::c_void,
                                    &mut *x.offset(k as isize) as *mut i32
                                        as *const libc::c_void,
                                    ::core::mem::size_of::<i32>() as u64,
                                );
                                a = (*a).t_next;
                            }
                            i += 1;
                            i;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    glp_free(tail as *mut libc::c_void);
    glp_free(head as *mut libc::c_void);
    glp_free(low as *mut libc::c_void);
    glp_free(cap as *mut libc::c_void);
    glp_free(cost as *mut libc::c_void);
    glp_free(x as *mut libc::c_void);
    glp_free(pi as *mut libc::c_void);
    return ret;
}