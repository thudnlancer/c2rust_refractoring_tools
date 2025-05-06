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
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn glp_check_asnprob(G: *mut glp_graph, v_set: i32) -> i32;
    fn _glp_mc21a(
        n: i32,
        icn: *const i32,
        ip: *const i32,
        lenr: *const i32,
        iperm: *mut i32,
        pr: *mut i32,
        arp: *mut i32,
        cv: *mut i32,
        out: *mut i32,
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
pub unsafe extern "C" fn glp_asnprob_hall(
    mut G: *mut glp_graph,
    mut v_set: i32,
    mut a_x: i32,
) -> i32 {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut card: i32 = 0;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut loc: i32 = 0;
    let mut n: i32 = 0;
    let mut n1: i32 = 0;
    let mut n2: i32 = 0;
    let mut xij: i32 = 0;
    let mut num: *mut i32 = 0 as *mut i32;
    let mut icn: *mut i32 = 0 as *mut i32;
    let mut ip: *mut i32 = 0 as *mut i32;
    let mut lenr: *mut i32 = 0 as *mut i32;
    let mut iperm: *mut i32 = 0 as *mut i32;
    let mut pr: *mut i32 = 0 as *mut i32;
    let mut arp: *mut i32 = 0 as *mut i32;
    let mut cv: *mut i32 = 0 as *mut i32;
    let mut out: *mut i32 = 0 as *mut i32;
    if v_set >= 0 as i32
        && v_set > (*G).v_size - ::core::mem::size_of::<i32>() as u64 as i32
    {
        (glp_error_(b"api/asnhall.c\0" as *const u8 as *const i8, 62 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_asnprob_hall: v_set = %d; invalid offset\n\0" as *const u8
                as *const i8,
            v_set,
        );
    }
    if a_x >= 0 as i32 && a_x > (*G).a_size - ::core::mem::size_of::<i32>() as u64 as i32
    {
        (glp_error_(b"api/asnhall.c\0" as *const u8 as *const i8, 65 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_asnprob_hall: a_x = %d; invalid offset\n\0" as *const u8 as *const i8,
            a_x,
        );
    }
    if glp_check_asnprob(G, v_set) != 0 {
        return -(1 as i32);
    }
    num = glp_alloc(1 as i32 + (*G).nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    n2 = 0 as i32;
    n1 = n2;
    i = 1 as i32;
    while i <= (*G).nv {
        v = *((*G).v).offset(i as isize);
        if ((*v).in_0).is_null() && !((*v).out).is_null() {
            n1 += 1;
            n1;
            *num.offset(i as isize) = 0 as i32;
        } else if !((*v).in_0).is_null() && ((*v).out).is_null() {
            n2 += 1;
            n2;
            *num.offset(i as isize) = n2;
        } else {
            (((*v).in_0).is_null() && ((*v).out).is_null()
                || {
                    glp_assert_(
                        b"v->in == NULL && v->out == NULL\0" as *const u8 as *const i8,
                        b"api/asnhall.c\0" as *const u8 as *const i8,
                        80 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            *num.offset(i as isize) = -(1 as i32);
        }
        i += 1;
        i;
    }
    n = if n1 >= n2 { n1 } else { n2 };
    icn = glp_alloc(1 as i32 + (*G).na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    ip = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    lenr = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    iperm = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    pr = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    arp = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    cv = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    out = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    k = 0 as i32;
    loc = 1 as i32;
    i = 1 as i32;
    while i <= (*G).nv {
        if !(*num.offset(i as isize) != 0 as i32) {
            k += 1;
            *ip.offset(k as isize) = loc;
            v = *((*G).v).offset(i as isize);
            a = (*v).out;
            while !a.is_null() {
                (*num.offset((*(*a).head).i as isize) != 0 as i32
                    || {
                        glp_assert_(
                            b"num[a->head->i] != 0\0" as *const u8 as *const i8,
                            b"api/asnhall.c\0" as *const u8 as *const i8,
                            105 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                let fresh0 = loc;
                loc = loc + 1;
                *icn.offset(fresh0 as isize) = *num.offset((*(*a).head).i as isize);
                a = (*a).t_next;
            }
            *lenr.offset(k as isize) = loc - *ip.offset(k as isize);
        }
        i += 1;
        i;
    }
    (loc - 1 as i32 == (*G).na
        || {
            glp_assert_(
                b"loc-1 == G->na\0" as *const u8 as *const i8,
                b"api/asnhall.c\0" as *const u8 as *const i8,
                110 as i32,
            );
            1 as i32 != 0
        }) as i32;
    k += 1;
    k;
    while k <= n {
        *ip.offset(k as isize) = loc;
        *lenr.offset(k as isize) = 0 as i32;
        k += 1;
        k;
    }
    card = _glp_mc21a(
        n,
        icn as *const i32,
        ip as *const i32,
        lenr as *const i32,
        iperm,
        pr,
        arp,
        cv,
        out,
    );
    i = 1 as i32;
    while i <= n {
        *arp.offset(i as isize) = 0 as i32;
        i += 1;
        i;
    }
    i = 1 as i32;
    while i <= card {
        k = *iperm.offset(i as isize);
        (1 as i32 <= k && k <= n
            || {
                glp_assert_(
                    b"1 <= k && k <= n\0" as *const u8 as *const i8,
                    b"api/asnhall.c\0" as *const u8 as *const i8,
                    124 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*arp.offset(k as isize) == 0 as i32
            || {
                glp_assert_(
                    b"arp[k] == 0\0" as *const u8 as *const i8,
                    b"api/asnhall.c\0" as *const u8 as *const i8,
                    125 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *arp.offset(k as isize) = i;
        i += 1;
        i;
    }
    if !(a_x < 0 as i32) {
        k = 0 as i32;
        i = 1 as i32;
        while i <= (*G).nv {
            if !(*num.offset(i as isize) != 0 as i32) {
                k += 1;
                k;
                v = *((*G).v).offset(i as isize);
                a = (*v).out;
                while !a.is_null() {
                    if *arp.offset(k as isize) == *num.offset((*(*a).head).i as isize) {
                        (*arp.offset(k as isize) != 0 as i32
                            || {
                                glp_assert_(
                                    b"arp[k] != 0\0" as *const u8 as *const i8,
                                    b"api/asnhall.c\0" as *const u8 as *const i8,
                                    140 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        xij = 1 as i32;
                    } else {
                        xij = 0 as i32;
                    }
                    memcpy(
                        ((*a).data as *mut i8).offset(a_x as isize) as *mut libc::c_void,
                        &mut xij as *mut i32 as *const libc::c_void,
                        ::core::mem::size_of::<i32>() as u64,
                    );
                    a = (*a).t_next;
                }
            }
            i += 1;
            i;
        }
    }
    glp_free(num as *mut libc::c_void);
    glp_free(icn as *mut libc::c_void);
    glp_free(ip as *mut libc::c_void);
    glp_free(lenr as *mut libc::c_void);
    glp_free(iperm as *mut libc::c_void);
    glp_free(pr as *mut libc::c_void);
    glp_free(arp as *mut libc::c_void);
    glp_free(cv as *mut libc::c_void);
    glp_free(out as *mut libc::c_void);
    return card;
}