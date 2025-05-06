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
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_add_vertices(G: *mut glp_graph, nadd: i32) -> i32;
    fn glp_add_arc(G: *mut glp_graph, i: i32, j: i32) -> *mut glp_arc;
    fn glp_erase_graph(G: *mut glp_graph, v_size: i32, a_size: i32);
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct set {
    pub size: i32,
    pub list: *mut i32,
    pub pos: *mut i32,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_kellerman(
    mut n: i32,
    mut func: Option<unsafe extern "C" fn(*mut libc::c_void, i32, *mut i32) -> i32>,
    mut info: *mut libc::c_void,
    mut H_: *mut libc::c_void,
) -> i32 {
    let mut H: *mut glp_graph = H_ as *mut glp_graph;
    let mut W_: set = set {
        size: 0,
        list: 0 as *mut i32,
        pos: 0 as *mut i32,
    };
    let mut W: *mut set = &mut W_;
    let mut V_: set = set {
        size: 0,
        list: 0 as *mut i32,
        pos: 0 as *mut i32,
    };
    let mut V: *mut set = &mut V_;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut m: i32 = 0;
    let mut t: i32 = 0;
    let mut len: i32 = 0;
    let mut card: i32 = 0;
    let mut best: i32 = 0;
    (n >= 0 as i32
        || {
            glp_assert_(
                b"n >= 0\0" as *const u8 as *const i8,
                b"misc/keller.c\0" as *const u8 as *const i8,
                99 as i32,
            );
            1 as i32 != 0
        }) as i32;
    glp_erase_graph(H, (*H).v_size, (*H).a_size);
    glp_add_vertices(H, n);
    (*W).size = 0 as i32;
    (*W).list = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*W).pos = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    memset(
        &mut *((*W).pos).offset(1 as i32 as isize) as *mut i32 as *mut libc::c_void,
        0 as i32,
        (::core::mem::size_of::<i32>() as u64).wrapping_mul(n as u64),
    );
    (*V).size = 0 as i32;
    (*V).list = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*V).pos = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    memset(
        &mut *((*V).pos).offset(1 as i32 as isize) as *mut i32 as *mut libc::c_void,
        0 as i32,
        (::core::mem::size_of::<i32>() as u64).wrapping_mul(n as u64),
    );
    i = 1 as i32;
    while i <= n {
        ((*W).size == 0 as i32
            || {
                glp_assert_(
                    b"W->size == 0\0" as *const u8 as *const i8,
                    b"misc/keller.c\0" as *const u8 as *const i8,
                    116 as i32,
                );
                1 as i32 != 0
            }) as i32;
        len = func.expect("non-null function pointer")(info, i, (*W).list);
        (0 as i32 <= len && len <= n
            || {
                glp_assert_(
                    b"0 <= len && len <= n\0" as *const u8 as *const i8,
                    b"misc/keller.c\0" as *const u8 as *const i8,
                    119 as i32,
                );
                1 as i32 != 0
            }) as i32;
        t = 1 as i32;
        while t <= len {
            j = *((*W).list).offset(t as isize);
            (1 as i32 <= j && j <= n
                || {
                    glp_assert_(
                        b"1 <= j && j <= n\0" as *const u8 as *const i8,
                        b"misc/keller.c\0" as *const u8 as *const i8,
                        122 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            if !(j >= i) {
                (*((*W).pos).offset(j as isize) == 0 as i32
                    || {
                        glp_assert_(
                            b"W->pos[j] == 0\0" as *const u8 as *const i8,
                            b"misc/keller.c\0" as *const u8 as *const i8,
                            124 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                (*W).size += 1;
                *((*W).list).offset((*W).size as isize) = j;
                *((*W).pos).offset(j as isize) = (*W).size;
            }
            t += 1;
            t;
        }
        if (*W).size == 0 as i32 {
            k = glp_add_vertices(H, 1 as i32) - n;
            glp_add_arc(H, i, n + k);
        } else {
            ((*V).size == 0 as i32
                || {
                    glp_assert_(
                        b"V->size == 0\0" as *const u8 as *const i8,
                        b"misc/keller.c\0" as *const u8 as *const i8,
                        138 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            k = (*H).nv - n;
            m = 1 as i32;
            while m <= k {
                if (*V).size == (*W).size {
                    break;
                }
                a = (**((*H).v).offset((n + m) as isize)).in_0;
                while !a.is_null() {
                    j = (*(*a).tail).i;
                    if *((*W).pos).offset(j as isize) == 0 as i32 {
                        break;
                    }
                    a = (*a).h_next;
                }
                if a.is_null() {
                    glp_add_arc(H, i, n + m);
                    a = (**((*H).v).offset((n + m) as isize)).in_0;
                    while !a.is_null() {
                        j = (*(*a).tail).i;
                        if *((*V).pos).offset(j as isize) == 0 as i32 {
                            (*V).size += 1;
                            *((*V).list).offset((*V).size as isize) = j;
                            *((*V).pos).offset(j as isize) = (*V).size;
                        }
                        a = (*a).h_next;
                    }
                }
                m += 1;
                m;
            }
            t = 1 as i32;
            while t <= (*V).size {
                j = *((*V).list).offset(t as isize);
                *((*V).pos).offset(j as isize) = 0 as i32;
                if *((*W).pos).offset(j as isize) != 0 as i32 {
                    if *((*W).pos).offset(j as isize) != (*W).size {
                        let mut jj: i32 = *((*W).list).offset((*W).size as isize);
                        *((*W).list).offset(*((*W).pos).offset(j as isize) as isize) = jj;
                        *((*W).pos).offset(jj as isize) = *((*W).pos).offset(j as isize);
                    }
                    (*W).size -= 1;
                    (*W).size;
                    *((*W).pos).offset(j as isize) = 0 as i32;
                }
                t += 1;
                t;
            }
            (*V).size = 0 as i32;
            while (*W).size > 0 as i32 {
                m = 0 as i32;
                best = -(1 as i32);
                k = (*H).nv - n;
                t = 1 as i32;
                while t <= k {
                    card = 0 as i32;
                    a = (**((*H).v).offset((n + t) as isize)).in_0;
                    while !a.is_null() {
                        j = (*(*a).tail).i;
                        if *((*W).pos).offset(j as isize) != 0 as i32 {
                            card += 1;
                            card;
                        }
                        a = (*a).h_next;
                    }
                    if best < card {
                        m = t;
                        best = card;
                    }
                    t += 1;
                    t;
                }
                (m > 0 as i32
                    || {
                        glp_assert_(
                            b"m > 0\0" as *const u8 as *const i8,
                            b"misc/keller.c\0" as *const u8 as *const i8,
                            198 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                k = glp_add_vertices(H, 1 as i32) - n;
                a = (**((*H).v).offset((n + m) as isize)).in_0;
                while !a.is_null() {
                    j = (*(*a).tail).i;
                    if *((*W).pos).offset(j as isize) != 0 as i32 {
                        glp_add_arc(H, j, n + k);
                        if *((*W).pos).offset(j as isize) != (*W).size {
                            let mut jj_0: i32 = *((*W).list).offset((*W).size as isize);
                            *((*W).list)
                                .offset(*((*W).pos).offset(j as isize) as isize) = jj_0;
                            *((*W).pos).offset(jj_0 as isize) = *((*W).pos)
                                .offset(j as isize);
                        }
                        (*W).size -= 1;
                        (*W).size;
                        *((*W).pos).offset(j as isize) = 0 as i32;
                    }
                    a = (*a).h_next;
                }
                glp_add_arc(H, i, n + k);
            }
        }
        i += 1;
        i;
    }
    glp_free((*W).list as *mut libc::c_void);
    glp_free((*W).pos as *mut libc::c_void);
    glp_free((*V).list as *mut libc::c_void);
    glp_free((*V).pos as *mut libc::c_void);
    return (*H).nv - n;
}