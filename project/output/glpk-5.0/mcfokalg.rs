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
    fn floor(_: libc::c_double) -> libc::c_double;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
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
pub unsafe extern "C" fn glp_mincost_okalg(
    mut G: *mut glp_graph,
    mut v_rhs: i32,
    mut a_low: i32,
    mut a_cap: i32,
    mut a_cost: i32,
    mut sol: *mut libc::c_double,
    mut a_x: i32,
    mut v_pi: i32,
) -> i32 {
    let mut current_block: u64;
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut nv: i32 = 0;
    let mut na: i32 = 0;
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut s: i32 = 0;
    let mut t: i32 = 0;
    let mut tail: *mut i32 = 0 as *mut i32;
    let mut head: *mut i32 = 0 as *mut i32;
    let mut low: *mut i32 = 0 as *mut i32;
    let mut cap: *mut i32 = 0 as *mut i32;
    let mut cost: *mut i32 = 0 as *mut i32;
    let mut x: *mut i32 = 0 as *mut i32;
    let mut pi: *mut i32 = 0 as *mut i32;
    let mut ret: i32 = 0;
    let mut sum: libc::c_double = 0.;
    let mut temp: libc::c_double = 0.;
    if v_rhs >= 0 as i32
        && v_rhs > (*G).v_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcfokalg.c\0" as *const u8 as *const i8, 35 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_okalg: v_rhs = %d; invalid offset\n\0" as *const u8
                as *const i8,
            v_rhs,
        );
    }
    if a_low >= 0 as i32
        && a_low > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcfokalg.c\0" as *const u8 as *const i8, 38 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_okalg: a_low = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_low,
        );
    }
    if a_cap >= 0 as i32
        && a_cap > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcfokalg.c\0" as *const u8 as *const i8, 41 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_okalg: a_cap = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_cap,
        );
    }
    if a_cost >= 0 as i32
        && a_cost > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcfokalg.c\0" as *const u8 as *const i8, 44 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_okalg: a_cost = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_cost,
        );
    }
    if a_x >= 0 as i32
        && a_x > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcfokalg.c\0" as *const u8 as *const i8, 47 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_okalg: a_x = %d; invalid offset\n\0" as *const u8 as *const i8,
            a_x,
        );
    }
    if v_pi >= 0 as i32
        && v_pi > (*G).v_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcfokalg.c\0" as *const u8 as *const i8, 49 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_okalg: v_pi = %d; invalid offset\n\0" as *const u8
                as *const i8,
            v_pi,
        );
    }
    s = (*G).nv + 1 as i32;
    t = s + 1 as i32;
    nv = t;
    na = (*G).na + 1 as i32;
    i = 1 as i32;
    while i <= (*G).nv {
        v = *((*G).v).offset(i as isize);
        if v_rhs >= 0 as i32 {
            memcpy(
                &mut temp as *mut libc::c_double as *mut libc::c_void,
                ((*v).data as *mut i8).offset(v_rhs as isize) as *const libc::c_void,
                ::core::mem::size_of::<libc::c_double>() as u64,
            );
        } else {
            temp = 0.0f64;
        }
        if temp != 0.0f64 {
            na += 1;
            na;
        }
        i += 1;
        i;
    }
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
    's_132: loop {
        if !(i <= (*G).nv) {
            current_block = 14945149239039849694;
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
                current_block = 6634122153602923610;
                break 's_132;
            } else {
                if a_low >= 0 as i32 {
                    memcpy(
                        &mut temp as *mut libc::c_double as *mut libc::c_void,
                        ((*a).data as *mut i8).offset(a_low as isize)
                            as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_double>() as u64,
                    );
                } else {
                    temp = 0.0f64;
                }
                if !(0.0f64 <= temp && temp <= 2147483647 as i32 as libc::c_double
                    && temp == floor(temp))
                {
                    ret = 0x12 as i32;
                    current_block = 6634122153602923610;
                    break 's_132;
                } else {
                    *low.offset(k as isize) = temp as i32;
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
                    if !(*low.offset(k as isize) as libc::c_double <= temp
                        && temp <= 2147483647 as i32 as libc::c_double
                        && temp == floor(temp))
                    {
                        ret = 0x12 as i32;
                        current_block = 6634122153602923610;
                        break 's_132;
                    } else {
                        *cap.offset(k as isize) = temp as i32;
                        if a_cost >= 0 as i32 {
                            memcpy(
                                &mut temp as *mut libc::c_double as *mut libc::c_void,
                                ((*a).data as *mut i8).offset(a_cost as isize)
                                    as *const libc::c_void,
                                ::core::mem::size_of::<libc::c_double>() as u64,
                            );
                        } else {
                            temp = 0.0f64;
                        }
                        if !(fabs(temp) <= 2147483647 as i32 as libc::c_double
                            && temp == floor(temp))
                        {
                            ret = 0x12 as i32;
                            current_block = 6634122153602923610;
                            break 's_132;
                        } else {
                            *cost.offset(k as isize) = temp as i32;
                            a = (*a).t_next;
                        }
                    }
                }
            }
        }
        i += 1;
        i;
    }
    match current_block {
        14945149239039849694 => {
            sum = 0.0f64;
            i = 1 as i32;
            loop {
                if !(i <= (*G).nv) {
                    current_block = 10853015579903106591;
                    break;
                }
                v = *((*G).v).offset(i as isize);
                if v_rhs >= 0 as i32 {
                    memcpy(
                        &mut temp as *mut libc::c_double as *mut libc::c_void,
                        ((*v).data as *mut i8).offset(v_rhs as isize)
                            as *const libc::c_void,
                        ::core::mem::size_of::<libc::c_double>() as u64,
                    );
                } else {
                    temp = 0.0f64;
                }
                if !(fabs(temp) <= 2147483647 as i32 as libc::c_double
                    && temp == floor(temp))
                {
                    ret = 0x12 as i32;
                    current_block = 6634122153602923610;
                    break;
                } else {
                    if temp > 0.0f64 {
                        k += 1;
                        k;
                        *tail.offset(k as isize) = s;
                        *head.offset(k as isize) = i;
                        let ref mut fresh0 = *cap.offset(k as isize);
                        *fresh0 = temp as i32;
                        *low.offset(k as isize) = *fresh0;
                        *cost.offset(k as isize) = 0 as i32;
                        sum += temp;
                    } else if temp < 0.0f64 {
                        k += 1;
                        k;
                        *tail.offset(k as isize) = i;
                        *head.offset(k as isize) = t;
                        let ref mut fresh1 = *cap.offset(k as isize);
                        *fresh1 = -temp as i32;
                        *low.offset(k as isize) = *fresh1;
                        *cost.offset(k as isize) = 0 as i32;
                    }
                    i += 1;
                    i;
                }
            }
            match current_block {
                6634122153602923610 => {}
                _ => {
                    k += 1;
                    k;
                    (k == na
                        || {
                            glp_assert_(
                                b"k == na\0" as *const u8 as *const i8,
                                b"api/mcfokalg.c\0" as *const u8 as *const i8,
                                150 as i32,
                            );
                            1 as i32 != 0
                        }) as i32;
                    *tail.offset(k as isize) = t;
                    *head.offset(k as isize) = s;
                    if sum > 2147483647 as i32 as libc::c_double {
                        ret = 0x12 as i32;
                    } else {
                        let ref mut fresh2 = *cap.offset(k as isize);
                        *fresh2 = sum as i32;
                        *low.offset(k as isize) = *fresh2;
                        *cost.offset(k as isize) = 0 as i32;
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
                                current_block = 14648606000749551097;
                            }
                            1 => {
                                ret = 0xa as i32;
                                current_block = 14648606000749551097;
                            }
                            2 => {
                                ret = 0x13 as i32;
                                current_block = 6634122153602923610;
                            }
                            3 => {
                                ret = 0x5 as i32;
                                current_block = 6634122153602923610;
                            }
                            _ => {
                                (ret != ret
                                    || {
                                        glp_assert_(
                                            b"ret != ret\0" as *const u8 as *const i8,
                                            b"api/mcfokalg.c\0" as *const u8 as *const i8,
                                            179 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                                current_block = 14648606000749551097;
                            }
                        }
                        match current_block {
                            6634122153602923610 => {}
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
                                if v_pi >= 0 as i32 {
                                    i = 1 as i32;
                                    while i <= (*G).nv {
                                        v = *((*G).v).offset(i as isize);
                                        temp = -(*pi.offset(i as isize) as libc::c_double);
                                        memcpy(
                                            ((*v).data as *mut i8).offset(v_pi as isize)
                                                as *mut libc::c_void,
                                            &mut temp as *mut libc::c_double as *const libc::c_void,
                                            ::core::mem::size_of::<libc::c_double>() as u64,
                                        );
                                        i += 1;
                                        i;
                                    }
                                }
                            }
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