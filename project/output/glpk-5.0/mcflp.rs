#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type glp_prob;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn strlen(_: *const i8) -> u64;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: u64) -> *mut libc::c_void;
    fn sprintf(_: *mut i8, _: *const i8, _: ...) -> i32;
    fn glp_set_prob_name(P: *mut glp_prob, name: *const i8);
    fn glp_add_rows(P: *mut glp_prob, nrs: i32) -> i32;
    fn glp_add_cols(P: *mut glp_prob, ncs: i32) -> i32;
    fn glp_set_row_name(P: *mut glp_prob, i: i32, name: *const i8);
    fn glp_set_col_name(P: *mut glp_prob, j: i32, name: *const i8);
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_obj_coef(P: *mut glp_prob, j: i32, coef: libc::c_double);
    fn glp_set_mat_col(
        P: *mut glp_prob,
        j: i32,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
    );
    fn glp_erase_prob(P: *mut glp_prob);
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
pub unsafe extern "C" fn glp_mincost_lp(
    mut lp: *mut glp_prob,
    mut G: *mut glp_graph,
    mut names: i32,
    mut v_rhs: i32,
    mut a_low: i32,
    mut a_cap: i32,
    mut a_cost: i32,
) {
    let mut v: *mut glp_vertex = 0 as *mut glp_vertex;
    let mut a: *mut glp_arc = 0 as *mut glp_arc;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut type_0: i32 = 0;
    let mut ind: [i32; 3] = [0; 3];
    let mut rhs: libc::c_double = 0.;
    let mut low: libc::c_double = 0.;
    let mut cap: libc::c_double = 0.;
    let mut cost: libc::c_double = 0.;
    let mut val: [libc::c_double; 3] = [0.; 3];
    if !(names == 1 as i32 || names == 0 as i32) {
        (glp_error_(b"api/mcflp.c\0" as *const u8 as *const i8, 47 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_lp: names = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            names,
        );
    }
    if v_rhs >= 0 as i32
        && v_rhs > (*G).v_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcflp.c\0" as *const u8 as *const i8, 50 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_lp: v_rhs = %d; invalid offset\n\0" as *const u8 as *const i8,
            v_rhs,
        );
    }
    if a_low >= 0 as i32
        && a_low > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcflp.c\0" as *const u8 as *const i8, 52 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_lp: a_low = %d; invalid offset\n\0" as *const u8 as *const i8,
            a_low,
        );
    }
    if a_cap >= 0 as i32
        && a_cap > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcflp.c\0" as *const u8 as *const i8, 54 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_lp: a_cap = %d; invalid offset\n\0" as *const u8 as *const i8,
            a_cap,
        );
    }
    if a_cost >= 0 as i32
        && a_cost > (*G).a_size - ::core::mem::size_of::<libc::c_double>() as u64 as i32
    {
        (glp_error_(b"api/mcflp.c\0" as *const u8 as *const i8, 56 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mincost_lp: a_cost = %d; invalid offset\n\0" as *const u8 as *const i8,
            a_cost,
        );
    }
    glp_erase_prob(lp);
    if names != 0 {
        glp_set_prob_name(lp, (*G).name);
    }
    if (*G).nv > 0 as i32 {
        glp_add_rows(lp, (*G).nv);
    }
    i = 1 as i32;
    while i <= (*G).nv {
        v = *((*G).v).offset(i as isize);
        if names != 0 {
            glp_set_row_name(lp, i, (*v).name);
        }
        if v_rhs >= 0 as i32 {
            memcpy(
                &mut rhs as *mut libc::c_double as *mut libc::c_void,
                ((*v).data as *mut i8).offset(v_rhs as isize) as *const libc::c_void,
                ::core::mem::size_of::<libc::c_double>() as u64,
            );
        } else {
            rhs = 0.0f64;
        }
        glp_set_row_bnds(lp, i, 5 as i32, rhs, rhs);
        i += 1;
        i;
    }
    if (*G).na > 0 as i32 {
        glp_add_cols(lp, (*G).na);
    }
    i = 1 as i32;
    j = 0 as i32;
    while i <= (*G).nv {
        v = *((*G).v).offset(i as isize);
        a = (*v).out;
        while !a.is_null() {
            j += 1;
            j;
            if names != 0 {
                let mut name: [i8; 51] = [0; 51];
                sprintf(
                    name.as_mut_ptr(),
                    b"x[%d,%d]\0" as *const u8 as *const i8,
                    (*(*a).tail).i,
                    (*(*a).head).i,
                );
                (strlen(name.as_mut_ptr()) < ::core::mem::size_of::<[i8; 51]>() as u64
                    || {
                        glp_assert_(
                            b"strlen(name) < sizeof(name)\0" as *const u8 as *const i8,
                            b"api/mcflp.c\0" as *const u8 as *const i8,
                            78 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
                glp_set_col_name(lp, j, name.as_mut_ptr());
            }
            if (*(*a).tail).i != (*(*a).head).i {
                ind[1 as i32 as usize] = (*(*a).tail).i;
                val[1 as i32 as usize] = 1.0f64;
                ind[2 as i32 as usize] = (*(*a).head).i;
                val[2 as i32 as usize] = -1.0f64;
                glp_set_mat_col(
                    lp,
                    j,
                    2 as i32,
                    ind.as_mut_ptr() as *const i32,
                    val.as_mut_ptr() as *const libc::c_double,
                );
            }
            if a_low >= 0 as i32 {
                memcpy(
                    &mut low as *mut libc::c_double as *mut libc::c_void,
                    ((*a).data as *mut i8).offset(a_low as isize) as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as u64,
                );
            } else {
                low = 0.0f64;
            }
            if a_cap >= 0 as i32 {
                memcpy(
                    &mut cap as *mut libc::c_double as *mut libc::c_void,
                    ((*a).data as *mut i8).offset(a_cap as isize) as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_double>() as u64,
                );
            } else {
                cap = 1.0f64;
            }
            if cap == 1.7976931348623157e+308f64 {
                type_0 = 2 as i32;
            } else if low != cap {
                type_0 = 4 as i32;
            } else {
                type_0 = 5 as i32;
            }
            glp_set_col_bnds(lp, j, type_0, low, cap);
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
            glp_set_obj_coef(lp, j, cost);
            a = (*a).t_next;
        }
        i += 1;
        i;
    }
    (j == (*G).na
        || {
            glp_assert_(
                b"j == G->na\0" as *const u8 as *const i8,
                b"api/mcflp.c\0" as *const u8 as *const i8,
                108 as i32,
            );
            1 as i32 != 0
        }) as i32;
}