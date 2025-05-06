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
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
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
pub unsafe extern "C" fn glp_weak_comp(mut G: *mut glp_graph, mut v_num: i32) -> i32 {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut f: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut nc: i32 = 0;
    let mut nv: i32 = 0;
    let mut pos1: i32 = 0;
    let mut pos2: i32 = 0;
    let mut prev: *mut i32 = 0 as *mut i32;
    let mut next: *mut i32 = 0 as *mut i32;
    let mut list: *mut i32 = 0 as *mut i32;
    if v_num >= 0 as i32
        && v_num > (*G).v_size - ::core::mem::size_of::<i32>() as u64 as i32
    {
        (glp_error_(b"api/weak.c\0" as *const u8 as *const i8, 56 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_weak_comp: v_num = %d; invalid offset\n\0" as *const u8 as *const i8,
            v_num,
        );
    }
    nv = (*G).nv;
    if nv == 0 as i32 {
        nc = 0 as i32;
    } else {
        prev = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        next = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        list = glp_alloc(1 as i32 + nv, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        f = 1 as i32;
        i = 1 as i32;
        while i <= nv {
            *prev.offset(i as isize) = i - 1 as i32;
            *next.offset(i as isize) = i + 1 as i32;
            i += 1;
            i;
        }
        *next.offset(nv as isize) = 0 as i32;
        nc = 0 as i32;
        while f != 0 as i32 {
            i = f;
            f = *next.offset(i as isize);
            if f != 0 as i32 {
                *prev.offset(f as isize) = 0 as i32;
            }
            *prev.offset(i as isize) = -(1 as i32);
            nc += 1;
            *next.offset(i as isize) = nc;
            *list.offset(1 as i32 as isize) = i;
            pos2 = 1 as i32;
            pos1 = pos2;
            while pos1 <= pos2 {
                let fresh0 = pos1;
                pos1 = pos1 + 1;
                i = *list.offset(fresh0 as isize);
                a = (**((*G).v).offset(i as isize)).in_0;
                while !a.is_null() {
                    j = (*(*a).tail).i;
                    if *prev.offset(j as isize) >= 0 as i32 {
                        if *prev.offset(j as isize) == 0 as i32 {
                            f = *next.offset(j as isize);
                        } else {
                            *next.offset(*prev.offset(j as isize) as isize) = *next
                                .offset(j as isize);
                        }
                        if !(*next.offset(j as isize) == 0 as i32) {
                            *prev.offset(*next.offset(j as isize) as isize) = *prev
                                .offset(j as isize);
                        }
                        *prev.offset(j as isize) = -(1 as i32);
                        *next.offset(j as isize) = nc;
                        pos2 += 1;
                        *list.offset(pos2 as isize) = j;
                    }
                    a = (*a).h_next;
                }
                a = (**((*G).v).offset(i as isize)).out;
                while !a.is_null() {
                    j = (*(*a).head).i;
                    if *prev.offset(j as isize) >= 0 as i32 {
                        if *prev.offset(j as isize) == 0 as i32 {
                            f = *next.offset(j as isize);
                        } else {
                            *next.offset(*prev.offset(j as isize) as isize) = *next
                                .offset(j as isize);
                        }
                        if !(*next.offset(j as isize) == 0 as i32) {
                            *prev.offset(*next.offset(j as isize) as isize) = *prev
                                .offset(j as isize);
                        }
                        *prev.offset(j as isize) = -(1 as i32);
                        *next.offset(j as isize) = nc;
                        pos2 += 1;
                        *list.offset(pos2 as isize) = j;
                    }
                    a = (*a).t_next;
                }
            }
        }
        if v_num >= 0 as i32 {
            i = 1 as i32;
            while i <= nv {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    ((*v).data as *mut i8).offset(v_num as isize) as *mut libc::c_void,
                    &mut *next.offset(i as isize) as *mut i32 as *const libc::c_void,
                    ::core::mem::size_of::<i32>() as u64,
                );
                i += 1;
                i;
            }
        }
        glp_free(prev as *mut libc::c_void);
        glp_free(next as *mut libc::c_void);
        glp_free(list as *mut libc::c_void);
    }
    return nc;
}