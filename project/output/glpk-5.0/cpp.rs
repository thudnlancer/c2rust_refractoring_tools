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
    fn glp_top_sort(G: *mut glp_graph, v_num: i32) -> i32;
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
pub unsafe extern "C" fn glp_cpp(
    mut G: *mut glp_graph,
    mut v_t: i32,
    mut v_es: i32,
    mut v_ls: i32,
) -> libc::c_double {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut k: i32 = 0;
    let mut nv: i32 = 0;
    let mut list: *mut i32 = 0 as *mut i32;
    let mut temp: libc::c_double = 0.;
    let mut total: libc::c_double = 0.;
    let mut t: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut es: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ls: *mut libc::c_double = 0 as *mut libc::c_double;
    if v_t >= 0 as i32
        && v_t > (*G).v_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/cpp.c\0" as *const u8 as *const i8, 69 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_cpp: v_t = %d; invalid offset\n\0" as *const u8 as *const i8, v_t);
    }
    if v_es >= 0 as i32
        && v_es > (*G).v_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/cpp.c\0" as *const u8 as *const i8, 71 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_cpp: v_es = %d; invalid offset\n\0" as *const u8 as *const i8, v_es);
    }
    if v_ls >= 0 as i32
        && v_ls > (*G).v_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/cpp.c\0" as *const u8 as *const i8, 73 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_cpp: v_ls = %d; invalid offset\n\0" as *const u8 as *const i8, v_ls);
    }
    nv = (*G).nv;
    if nv == 0 as i32 {
        total = 0.0f64;
    } else {
        t = glp_alloc(
            1 as i32 + nv,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        es = glp_alloc(
            1 as i32 + nv,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        ls = glp_alloc(
            1 as i32 + nv,
            ::core::mem::size_of::<libc::c_double>() as u64 as i32,
        ) as *mut libc::c_double;
        list = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        i = 1 as i32;
        while i <= nv {
            v = *((*G).v).offset(i as isize);
            if v_t >= 0 as i32 {
                memcpy(
                    &mut *t.offset(i as isize) as *mut libc::c_double
                        as *mut libc::c_void,
                    ((*v).data as *mut i8).offset(v_t as isize) as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as u64,
                );
                if *t.offset(i as isize) < 0.0f64 {
                    (glp_error_(b"api/cpp.c\0" as *const u8 as *const i8, 90 as i32))
                        .expect(
                            "non-null function pointer",
                        )(
                        b"glp_cpp: t[%d] = %g; invalid time\n\0" as *const u8
                            as *const i8,
                        i,
                        *t.offset(i as isize),
                    );
                }
            } else {
                *t.offset(i as isize) = 1.0f64;
            }
            i += 1;
            i;
        }
        sorting(G, list);
        k = 1 as i32;
        while k <= nv {
            j = *list.offset(k as isize);
            *es.offset(j as isize) = 0.0f64;
            a = (**((*G).v).offset(j as isize)).in_0;
            while !a.is_null() {
                i = (*(*a).tail).i;
                temp = *es.offset(i as isize) + *t.offset(i as isize);
                if *es.offset(j as isize) < temp {
                    *es.offset(j as isize) = temp;
                }
                a = (*a).h_next;
            }
            k += 1;
            k;
        }
        total = 0.0f64;
        i = 1 as i32;
        while i <= nv {
            temp = *es.offset(i as isize) + *t.offset(i as isize);
            if total < temp {
                total = temp;
            }
            i += 1;
            i;
        }
        k = nv;
        while k >= 1 as i32 {
            i = *list.offset(k as isize);
            *ls.offset(i as isize) = total - *t.offset(i as isize);
            a = (**((*G).v).offset(i as isize)).out;
            while !a.is_null() {
                j = (*(*a).head).i;
                temp = *ls.offset(j as isize) - *t.offset(i as isize);
                if *ls.offset(i as isize) > temp {
                    *ls.offset(i as isize) = temp;
                }
                a = (*a).t_next;
            }
            if *ls.offset(i as isize) < *es.offset(i as isize) {
                *ls.offset(i as isize) = *es.offset(i as isize);
            }
            k -= 1;
            k;
        }
        if v_es >= 0 as i32 {
            i = 1 as i32;
            while i <= nv {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    ((*v).data as *mut i8).offset(v_es as isize) as *mut libc::c_void,
                    &mut *es.offset(i as isize) as *mut libc::c_double
                        as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as u64,
                );
                i += 1;
                i;
            }
        }
        if v_ls >= 0 as i32 {
            i = 1 as i32;
            while i <= nv {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    ((*v).data as *mut i8).offset(v_ls as isize) as *mut libc::c_void,
                    &mut *ls.offset(i as isize) as *mut libc::c_double
                        as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as u64,
                );
                i += 1;
                i;
            }
        }
        glp_free(t as *mut libc::c_void);
        glp_free(es as *mut libc::c_void);
        glp_free(ls as *mut libc::c_void);
        glp_free(list as *mut libc::c_void);
    }
    return total;
}
unsafe extern "C" fn sorting(mut G: *mut glp_graph, mut list: *mut i32) {
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut nv: i32 = 0;
    let mut v_size: i32 = 0;
    let mut num: *mut i32 = 0 as *mut i32;
    let mut save: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    nv = (*G).nv;
    v_size = (*G).v_size;
    save = glp_alloc(
        1 as i32 + nv,
        ::core::mem::size_of::<*mut libc::c_void>() as u64 as i32,
    ) as *mut *mut libc::c_void;
    num = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    (*G).v_size = ::core::mem::size_of::<i32>() as u64 as i32;
    i = 1 as i32;
    while i <= nv {
        let ref mut fresh0 = *save.offset(i as isize);
        *fresh0 = (**((*G).v).offset(i as isize)).data;
        let ref mut fresh1 = (**((*G).v).offset(i as isize)).data;
        *fresh1 = &mut *num.offset(i as isize) as *mut i32 as *mut libc::c_void;
        *list.offset(i as isize) = 0 as i32;
        i += 1;
        i;
    }
    if glp_top_sort(G, 0 as i32) != 0 as i32 {
        (glp_error_(b"api/cpp.c\0" as *const u8 as *const i8, 169 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_cpp: project network is not acyclic\n\0" as *const u8 as *const i8);
    }
    (*G).v_size = v_size;
    i = 1 as i32;
    while i <= nv {
        let ref mut fresh2 = (**((*G).v).offset(i as isize)).data;
        *fresh2 = *save.offset(i as isize);
        k = *num.offset(i as isize);
        (1 as i32 <= k && k <= nv
            || {
                glp_assert_(
                    b"1 <= k && k <= nv\0" as *const u8 as *const i8,
                    b"api/cpp.c\0" as *const u8 as *const i8,
                    174 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*list.offset(k as isize) == 0 as i32
            || {
                glp_assert_(
                    b"list[k] == 0\0" as *const u8 as *const i8,
                    b"api/cpp.c\0" as *const u8 as *const i8,
                    175 as i32,
                );
                1 as i32 != 0
            }) as i32;
        *list.offset(k as isize) = i;
        i += 1;
        i;
    }
    glp_free(save as *mut libc::c_void);
    glp_free(num as *mut libc::c_void);
}