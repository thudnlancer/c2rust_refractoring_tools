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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
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
unsafe extern "C" fn top_sort(mut G: *mut glp_graph, mut num: *mut i32) -> i32 {
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut cnt: i32 = 0;
    let mut top: i32 = 0;
    let mut stack: *mut i32 = 0 as *mut i32;
    let mut indeg: *mut i32 = 0 as *mut i32;
    indeg = glp_alloc(1 as i32 + (*G).nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    stack = glp_alloc(1 as i32 + (*G).nv, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    top = 0 as i32;
    i = 1 as i32;
    while i <= (*G).nv {
        let ref mut fresh0 = *indeg.offset(i as isize);
        *fresh0 = 0 as i32;
        *num.offset(i as isize) = *fresh0;
        a = (**((*G).v).offset(i as isize)).in_0;
        while !a.is_null() {
            let ref mut fresh1 = *indeg.offset(i as isize);
            *fresh1 += 1;
            *fresh1;
            a = (*a).h_next;
        }
        if *indeg.offset(i as isize) == 0 as i32 {
            top += 1;
            *stack.offset(top as isize) = i;
        }
        i += 1;
        i;
    }
    cnt = 0 as i32;
    while top > 0 as i32 {
        let fresh2 = top;
        top = top - 1;
        i = *stack.offset(fresh2 as isize);
        (*indeg.offset(i as isize) == 0 as i32
            || {
                glp_assert_(
                    b"indeg[i] == 0\0" as *const u8 as *const i8,
                    b"api/topsort.c\0" as *const u8 as *const i8,
                    78 as i32,
                );
                1 as i32 != 0
            }) as i32;
        (*num.offset(i as isize) == 0 as i32
            || {
                glp_assert_(
                    b"num[i] == 0\0" as *const u8 as *const i8,
                    b"api/topsort.c\0" as *const u8 as *const i8,
                    80 as i32,
                );
                1 as i32 != 0
            }) as i32;
        cnt += 1;
        *num.offset(i as isize) = cnt;
        a = (**((*G).v).offset(i as isize)).out;
        while !a.is_null() {
            j = (*(*a).head).i;
            (*indeg.offset(j as isize) > 0 as i32
                || {
                    glp_assert_(
                        b"indeg[j] > 0\0" as *const u8 as *const i8,
                        b"api/topsort.c\0" as *const u8 as *const i8,
                        88 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            let ref mut fresh3 = *indeg.offset(j as isize);
            *fresh3 -= 1;
            *fresh3;
            if *indeg.offset(j as isize) == 0 as i32 {
                top += 1;
                *stack.offset(top as isize) = j;
            }
            a = (*a).t_next;
        }
    }
    glp_free(indeg as *mut libc::c_void);
    glp_free(stack as *mut libc::c_void);
    return (*G).nv - cnt;
}
#[no_mangle]
pub unsafe extern "C" fn glp_top_sort(mut G: *mut glp_graph, mut v_num: i32) -> i32 {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut i: i32 = 0;
    let mut cnt: i32 = 0;
    let mut num: *mut i32 = 0 as *mut i32;
    if v_num >= 0 as i32
        && v_num > (*G).v_size - ::core::mem::size_of::<i32>() as u64 as i32
    {
        (glp_error_(b"api/topsort.c\0" as *const u8 as *const i8, 104 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_top_sort: v_num = %d; invalid offset\n\0" as *const u8 as *const i8,
            v_num,
        );
    }
    if (*G).nv == 0 as i32 {
        cnt = 0 as i32;
    } else {
        num = glp_alloc(1 as i32 + (*G).nv, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        cnt = top_sort(G, num);
        if v_num >= 0 as i32 {
            i = 1 as i32;
            while i <= (*G).nv {
                v = *((*G).v).offset(i as isize);
                memcpy(
                    ((*v).data as *mut i8).offset(v_num as isize) as *mut libc::c_void,
                    &mut *num.offset(i as isize) as *mut i32 as *const libc::c_void,
                    ::core::mem::size_of::<i32>() as u64,
                );
                i += 1;
                i;
            }
        }
        glp_free(num as *mut libc::c_void);
    }
    return cnt;
}