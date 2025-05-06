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
    fn _glp_relax4(csa: *mut relax4_csa) -> i32;
    fn _glp_relax4_inidat(csa: *mut relax4_csa);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct relax4_csa {
    pub n: i32,
    pub na: i32,
    pub large: i32,
    pub repeat: i32,
    pub crash: i32,
    pub startn: *mut i32,
    pub endn: *mut i32,
    pub fou: *mut i32,
    pub nxtou: *mut i32,
    pub fin: *mut i32,
    pub nxtin: *mut i32,
    pub rc: *mut i32,
    pub u: *mut i32,
    pub dfct: *mut i32,
    pub x: *mut i32,
    pub nmultinode: i32,
    pub iter: i32,
    pub num_augm: i32,
    pub num_ascnt: i32,
    pub nsp: i32,
    pub label: *mut i32,
    pub prdcsr: *mut i32,
    pub save: *mut i32,
    pub tfstou: *mut i32,
    pub tnxtou: *mut i32,
    pub tfstin: *mut i32,
    pub tnxtin: *mut i32,
    pub nxtqueue: *mut i32,
    pub scan: *mut i8,
    pub mark: *mut i8,
    pub extend_arc: *mut i32,
    pub sb_level: *mut i32,
    pub sb_arc: *mut i32,
}
unsafe extern "C" fn overflow(mut u: i32, mut v: i32) -> i32 {
    if u > 0 as i32 && v > 0 as i32 && u + v < 0 as i32 {
        return 1 as i32;
    }
    if u < 0 as i32 && v < 0 as i32 && u + v > 0 as i32 {
        return 1 as i32;
    }
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn glp_mincost_relax4(
    mut G: *mut glp_graph,
    mut v_rhs: i32,
    mut a_low: i32,
    mut a_cap: i32,
    mut a_cost: i32,
    mut crash: i32,
    mut sol: *mut libc::c_double,
    mut a_x: i32,
    mut a_rc: i32,
) -> i32 {
    let mut current_block: u64;
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut csa: relax4_csa = relax4_csa {
        n: 0,
        na: 0,
        large: 0,
        repeat: 0,
        crash: 0,
        startn: 0 as *mut i32,
        endn: 0 as *mut i32,
        fou: 0 as *mut i32,
        nxtou: 0 as *mut i32,
        fin: 0 as *mut i32,
        nxtin: 0 as *mut i32,
        rc: 0 as *mut i32,
        u: 0 as *mut i32,
        dfct: 0 as *mut i32,
        x: 0 as *mut i32,
        nmultinode: 0,
        iter: 0,
        num_augm: 0,
        num_ascnt: 0,
        nsp: 0,
        label: 0 as *mut i32,
        prdcsr: 0 as *mut i32,
        save: 0 as *mut i32,
        tfstou: 0 as *mut i32,
        tnxtou: 0 as *mut i32,
        tfstin: 0 as *mut i32,
        tnxtin: 0 as *mut i32,
        nxtqueue: 0 as *mut i32,
        scan: 0 as *mut i8,
        mark: 0 as *mut i8,
        extend_arc: 0 as *mut i32,
        sb_level: 0 as *mut i32,
        sb_arc: 0 as *mut i32,
    };
    let mut i: i32 = 0;
    let mut k: i32 = 0;
    let mut large: i32 = 0;
    let mut n: i32 = 0;
    let mut na: i32 = 0;
    let mut ret: i32 = 0;
    let mut cap: libc::c_double = 0.;
    let mut cost: libc::c_double = 0.;
    let mut low: libc::c_double = 0.;
    let mut rc: libc::c_double = 0.;
    let mut rhs: libc::c_double = 0.;
    let mut sum: libc::c_double = 0.;
    let mut x: libc::c_double = 0.;
    if v_rhs >= 0 as i32
        && v_rhs > (*G).v_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcfrelax.c\0" as *const u8 as *const i8, 43 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_relax4: v_rhs = %d; invalid offset\n\0" as *const u8
                as *const i8,
            v_rhs,
        );
    }
    if a_low >= 0 as i32
        && a_low > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcfrelax.c\0" as *const u8 as *const i8, 46 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_relax4: a_low = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_low,
        );
    }
    if a_cap >= 0 as i32
        && a_cap > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcfrelax.c\0" as *const u8 as *const i8, 49 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_relax4: a_cap = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_cap,
        );
    }
    if a_cost >= 0 as i32
        && a_cost > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcfrelax.c\0" as *const u8 as *const i8, 52 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_relax4: a_cost = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_cost,
        );
    }
    if a_x >= 0 as i32
        && a_x > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcfrelax.c\0" as *const u8 as *const i8, 55 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_relax4: a_x = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_x,
        );
    }
    if a_rc >= 0 as i32
        && a_rc > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcfrelax.c\0" as *const u8 as *const i8, 58 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_relax4: a_rc = %d; invalid offset\n\0" as *const u8
                as *const i8,
            a_rc,
        );
    }
    n = (*G).nv;
    csa.n = n;
    na = (*G).na;
    csa.na = na;
    large = 2147483647 as i32 / 4 as i32;
    csa.large = large;
    csa.repeat = 0 as i32;
    csa.crash = crash;
    csa.startn = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.endn = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.fou = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.nxtou = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.fin = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.nxtin = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.rc = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.u = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.dfct = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.x = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.label = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.prdcsr = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.save = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.tfstou = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.tnxtou = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.tfstin = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.tnxtin = glp_alloc(1 as i32 + na, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.nxtqueue = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    csa.scan = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i8>() as u64 as i32)
        as *mut i8;
    csa.mark = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i8>() as u64 as i32)
        as *mut i8;
    if crash != 0 {
        csa.extend_arc = glp_alloc(
            1 as i32 + n,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        csa.sb_level = glp_alloc(
            1 as i32 + n,
            ::core::mem::size_of::<i32>() as u64 as i32,
        ) as *mut i32;
        csa.sb_arc = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
            as *mut i32;
    } else {
        csa.extend_arc = 0 as *mut i32;
        csa.sb_level = 0 as *mut i32;
        csa.sb_arc = 0 as *mut i32;
    }
    i = 1 as i32;
    loop {
        if !(i <= n) {
            current_block = 10891380440665537214;
            break;
        }
        v = *((*G).v).offset(i as isize);
        if v_rhs >= 0 as i32 {
            memcpy(
                &mut rhs as *mut libc::c_double as *mut libc::c_void,
                ((*v).data as *mut i8).offset(v_rhs as isize) as *const libc::c_void,
                ::core::mem::size_of::<libc::c_double>() as u64,
            );
        } else {
            rhs = 0.0f64;
        }
        if !(fabs(rhs) <= large as libc::c_double && rhs == floor(rhs)) {
            ret = 0x12 as i32;
            current_block = 14265366926986509618;
            break;
        } else {
            *(csa.dfct).offset(i as isize) = -(rhs as i32);
            i += 1;
            i;
        }
    }
    match current_block {
        10891380440665537214 => {
            k = 0 as i32;
            i = 1 as i32;
            's_211: loop {
                if !(i <= n) {
                    current_block = 10512632378975961025;
                    break;
                }
                v = *((*G).v).offset(i as isize);
                a = (*v).out;
                while !a.is_null() {
                    k += 1;
                    k;
                    if (*(*a).tail).i == (*(*a).head).i {
                        ret = 0x12 as i32;
                        current_block = 14265366926986509618;
                        break 's_211;
                    } else {
                        *(csa.startn).offset(k as isize) = (*(*a).tail).i;
                        *(csa.endn).offset(k as isize) = (*(*a).head).i;
                        if a_cost >= 0 as i32 {
                            memcpy(
                                &mut cost as *mut libc::c_double as *mut libc::c_void,
                                ((*a).data as *mut i8).offset(a_cost as isize)
                                    as *const libc::c_void,
                                ::core::mem::size_of::<libc::c_double>() as u64,
                            );
                        } else {
                            cost = 0.0f64;
                        }
                        if !(fabs(cost) <= large as libc::c_double
                            && cost == floor(cost))
                        {
                            ret = 0x12 as i32;
                            current_block = 14265366926986509618;
                            break 's_211;
                        } else {
                            *(csa.rc).offset(k as isize) = cost as i32;
                            if a_low >= 0 as i32 {
                                memcpy(
                                    &mut low as *mut libc::c_double as *mut libc::c_void,
                                    ((*a).data as *mut i8).offset(a_low as isize)
                                        as *const libc::c_void,
                                    ::core::mem::size_of::<libc::c_double>() as u64,
                                );
                            } else {
                                low = 0.0f64;
                            }
                            if !(0.0f64 <= low && low <= large as libc::c_double
                                && low == floor(low))
                            {
                                ret = 0x12 as i32;
                                current_block = 14265366926986509618;
                                break 's_211;
                            } else {
                                if a_cap >= 0 as i32 {
                                    memcpy(
                                        &mut cap as *mut libc::c_double as *mut libc::c_void,
                                        ((*a).data as *mut i8).offset(a_cap as isize)
                                            as *const libc::c_void,
                                        ::core::mem::size_of::<libc::c_double>() as u64,
                                    );
                                } else {
                                    cap = 1.0f64;
                                }
                                if !(low <= cap && cap <= large as libc::c_double
                                    && cap == floor(cap))
                                {
                                    ret = 0x12 as i32;
                                    current_block = 14265366926986509618;
                                    break 's_211;
                                } else {
                                    *(csa.u).offset(k as isize) = (cap - low) as i32;
                                    if overflow(
                                        *(csa.dfct).offset((*(*a).tail).i as isize),
                                        low as i32,
                                    ) != 0
                                    {
                                        ret = 0x13 as i32;
                                        current_block = 14265366926986509618;
                                        break 's_211;
                                    } else {
                                        *(csa.dfct).offset((*(*a).tail).i as isize) += low as i32;
                                        if overflow(
                                            *(csa.dfct).offset((*(*a).head).i as isize),
                                            -low as i32,
                                        ) != 0
                                        {
                                            ret = 0x13 as i32;
                                            current_block = 14265366926986509618;
                                            break 's_211;
                                        } else {
                                            *(csa.dfct).offset((*(*a).head).i as isize) -= low as i32;
                                            a = (*a).t_next;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                i += 1;
                i;
            }
            match current_block {
                14265366926986509618 => {}
                _ => {
                    _glp_relax4_inidat(&mut csa);
                    ret = _glp_relax4(&mut csa);
                    if ret != 0 as i32 {
                        (1 as i32 <= ret && ret <= 8 as i32
                            || {
                                glp_assert_(
                                    b"1 <= ret && ret <= 8\0" as *const u8 as *const i8,
                                    b"api/mcfrelax.c\0" as *const u8 as *const i8,
                                    184 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        ret = 0xa as i32;
                    } else {
                        sum = 0.0f64;
                        k = 0 as i32;
                        i = 1 as i32;
                        while i <= n {
                            v = *((*G).v).offset(i as isize);
                            a = (*v).out;
                            while !a.is_null() {
                                k += 1;
                                k;
                                if a_low >= 0 as i32 {
                                    memcpy(
                                        &mut low as *mut libc::c_double as *mut libc::c_void,
                                        ((*a).data as *mut i8).offset(a_low as isize)
                                            as *const libc::c_void,
                                        ::core::mem::size_of::<libc::c_double>() as u64,
                                    );
                                } else {
                                    low = 0.0f64;
                                }
                                x = *(csa.x).offset(k as isize) as libc::c_double + low;
                                if a_x >= 0 as i32 {
                                    memcpy(
                                        ((*a).data as *mut i8).offset(a_x as isize)
                                            as *mut libc::c_void,
                                        &mut x as *mut libc::c_double as *const libc::c_void,
                                        ::core::mem::size_of::<libc::c_double>() as u64,
                                    );
                                }
                                rc = *(csa.rc).offset(k as isize) as libc::c_double;
                                if a_rc >= 0 as i32 {
                                    memcpy(
                                        ((*a).data as *mut i8).offset(a_rc as isize)
                                            as *mut libc::c_void,
                                        &mut rc as *mut libc::c_double as *const libc::c_void,
                                        ::core::mem::size_of::<libc::c_double>() as u64,
                                    );
                                }
                                if a_cost >= 0 as i32 {
                                    memcpy(
                                        &mut cost as *mut libc::c_double as *mut libc::c_void,
                                        ((*a).data as *mut i8).offset(a_cost as isize)
                                            as *const libc::c_void,
                                        ::core::mem::size_of::<libc::c_double>() as u64,
                                    );
                                } else {
                                    cost = 0.0f64;
                                }
                                sum += cost * x;
                                a = (*a).t_next;
                            }
                            i += 1;
                            i;
                        }
                        if !sol.is_null() {
                            *sol = sum;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    glp_free(csa.startn as *mut libc::c_void);
    glp_free(csa.endn as *mut libc::c_void);
    glp_free(csa.fou as *mut libc::c_void);
    glp_free(csa.nxtou as *mut libc::c_void);
    glp_free(csa.fin as *mut libc::c_void);
    glp_free(csa.nxtin as *mut libc::c_void);
    glp_free(csa.rc as *mut libc::c_void);
    glp_free(csa.u as *mut libc::c_void);
    glp_free(csa.dfct as *mut libc::c_void);
    glp_free(csa.x as *mut libc::c_void);
    glp_free(csa.label as *mut libc::c_void);
    glp_free(csa.prdcsr as *mut libc::c_void);
    glp_free(csa.save as *mut libc::c_void);
    glp_free(csa.tfstou as *mut libc::c_void);
    glp_free(csa.tnxtou as *mut libc::c_void);
    glp_free(csa.tfstin as *mut libc::c_void);
    glp_free(csa.tnxtin as *mut libc::c_void);
    glp_free(csa.nxtqueue as *mut libc::c_void);
    glp_free(csa.scan as *mut libc::c_void);
    glp_free(csa.mark as *mut libc::c_void);
    if crash != 0 {
        glp_free(csa.extend_arc as *mut libc::c_void);
        glp_free(csa.sb_level as *mut libc::c_void);
        glp_free(csa.sb_arc as *mut libc::c_void);
    }
    return ret;
}