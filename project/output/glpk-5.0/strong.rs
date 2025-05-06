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
    fn _glp_mc13d(
        n: i32,
        icn: *const i32,
        ip: *const i32,
        lenr: *const i32,
        ior: *mut i32,
        ib: *mut i32,
        lowl: *mut i32,
        numb: *mut i32,
        prev: *mut i32,
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
pub unsafe extern "C" fn glp_strong_comp(mut G: *mut glp_graph, mut v_num: i32) -> i32 {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut last: i32 = 0;
    let mut n: i32 = 0;
    let mut na: i32 = 0;
    let mut nc: i32 = 0;
    let mut icn: *mut i32 = 0 as *mut i32;
    let mut ip: *mut i32 = 0 as *mut i32;
    let mut lenr: *mut i32 = 0 as *mut i32;
    let mut ior: *mut i32 = 0 as *mut i32;
    let mut ib: *mut i32 = 0 as *mut i32;
    let mut lowl: *mut i32 = 0 as *mut i32;
    let mut numb: *mut i32 = 0 as *mut i32;
    let mut prev: *mut i32 = 0 as *mut i32;
    if v_num >= 0 as i32
        && v_num > (*G).v_size - ::core::mem::size_of::<i32>() as u64 as i32
    {
        (glp_error_(b"api/strong.c\0" as *const u8 as *const i8, 60 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_strong_comp: v_num = %d; invalid offset\n\0" as *const u8 as *const i8,
            v_num,
        );
    }
    n = (*G).nv;
    if n == 0 as i32 {
        nc = 0 as i32;
    } else {
        na = (*G).na;
        icn = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        ip = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        lenr = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        ior = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        ib = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        lowl = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        numb = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        prev = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
        k = 1 as i32;
        i = 1 as i32;
        while i <= n {
            v = *((*G).v).offset(i as isize);
            *ip.offset(i as isize) = k;
            a = (*v).out;
            while !a.is_null() {
                let fresh0 = k;
                k = k + 1;
                *icn.offset(fresh0 as isize) = (*(*a).head).i;
                a = (*a).t_next;
            }
            *lenr.offset(i as isize) = k - *ip.offset(i as isize);
            i += 1;
            i;
        }
        (na == k - 1 as i32
            || {
                glp_assert_(
                    b"na == k-1\0" as *const u8 as *const i8,
                    b"api/strong.c\0" as *const u8 as *const i8,
                    84 as i32,
                );
                1 as i32 != 0
            }) as i32;
        nc = _glp_mc13d(
            n,
            icn as *const i32,
            ip as *const i32,
            lenr as *const i32,
            ior,
            ib,
            lowl,
            numb,
            prev,
        );
        if v_num >= 0 as i32 {
            (*ib.offset(1 as i32 as isize) == 1 as i32
                || {
                    glp_assert_(
                        b"ib[1] == 1\0" as *const u8 as *const i8,
                        b"api/strong.c\0" as *const u8 as *const i8,
                        87 as i32,
                    );
                    1 as i32 != 0
                }) as i32;
            k = 1 as i32;
            while k <= nc {
                last = if k < nc {
                    *ib.offset((k + 1 as i32) as isize)
                } else {
                    n + 1 as i32
                };
                (*ib.offset(k as isize) < last
                    || {
                        glp_assert_(
                            b"ib[k] < last\0" as *const u8 as *const i8,
                            b"api/strong.c\0" as *const u8 as *const i8,
                            90 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                i = *ib.offset(k as isize);
                while i < last {
                    v = *((*G).v).offset(*ior.offset(i as isize) as isize);
                    memcpy(
                        ((*v).data as *mut i8).offset(v_num as isize)
                            as *mut libc::c_void,
                        &mut k as *mut i32 as *const libc::c_void,
                        ::core::mem::size_of::<i32>() as u64,
                    );
                    i += 1;
                    i;
                }
                k += 1;
                k;
            }
        }
        glp_free(icn as *mut libc::c_void);
        glp_free(ip as *mut libc::c_void);
        glp_free(lenr as *mut libc::c_void);
        glp_free(ior as *mut libc::c_void);
        glp_free(ib as *mut libc::c_void);
        glp_free(lowl as *mut libc::c_void);
        glp_free(numb as *mut libc::c_void);
        glp_free(prev as *mut libc::c_void);
    }
    return nc;
}