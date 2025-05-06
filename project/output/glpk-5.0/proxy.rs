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
    pub type glp_tree;
    fn glp_create_prob() -> *mut glp_prob;
    fn glp_set_obj_dir(P: *mut glp_prob, dir: i32);
    fn glp_add_rows(P: *mut glp_prob, nrs: i32) -> i32;
    fn glp_set_row_name(P: *mut glp_prob, i: i32, name: *const i8);
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
    fn glp_set_mat_row(
        P: *mut glp_prob,
        i: i32,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
    );
    fn glp_copy_prob(dest: *mut glp_prob, prob: *mut glp_prob, names: i32);
    fn glp_delete_prob(P: *mut glp_prob);
    fn glp_get_obj_dir(P: *mut glp_prob) -> i32;
    fn glp_get_num_cols(P: *mut glp_prob) -> i32;
    fn glp_get_col_lb(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_col_ub(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_obj_coef(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_simplex(P: *mut glp_prob, parm: *const glp_smcp) -> i32;
    fn glp_init_smcp(parm: *mut glp_smcp);
    fn glp_get_status(P: *mut glp_prob) -> i32;
    fn glp_get_col_prim(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_col_kind(P: *mut glp_prob, j: i32) -> i32;
    fn glp_intopt(P: *mut glp_prob, parm: *const glp_iocp) -> i32;
    fn glp_init_iocp(parm: *mut glp_iocp);
    fn glp_mip_status(P: *mut glp_prob) -> i32;
    fn glp_mip_obj_val(P: *mut glp_prob) -> libc::c_double;
    fn glp_mip_col_val(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_ios_reason(T: *mut glp_tree) -> i32;
    fn glp_ios_heur_sol(T: *mut glp_tree, x: *const libc::c_double) -> i32;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_term_out(flag: i32) -> i32;
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_mem_usage(
        count: *mut i32,
        cpeak: *mut i32,
        total: *mut size_t,
        tpeak: *mut size_t,
    );
    fn glp_time() -> libc::c_double;
    fn ceil(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_smcp {
    pub msg_lev: i32,
    pub meth: i32,
    pub pricing: i32,
    pub r_test: i32,
    pub tol_bnd: libc::c_double,
    pub tol_dj: libc::c_double,
    pub tol_piv: libc::c_double,
    pub obj_ll: libc::c_double,
    pub obj_ul: libc::c_double,
    pub it_lim: i32,
    pub tm_lim: i32,
    pub out_frq: i32,
    pub out_dly: i32,
    pub presolve: i32,
    pub excl: i32,
    pub shift: i32,
    pub aorn: i32,
    pub foo_bar: [libc::c_double; 33],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_iocp {
    pub msg_lev: i32,
    pub br_tech: i32,
    pub bt_tech: i32,
    pub tol_int: libc::c_double,
    pub tol_obj: libc::c_double,
    pub tm_lim: i32,
    pub out_frq: i32,
    pub out_dly: i32,
    pub cb_func: Option<unsafe extern "C" fn(*mut glp_tree, *mut libc::c_void) -> ()>,
    pub cb_info: *mut libc::c_void,
    pub cb_size: i32,
    pub pp_tech: i32,
    pub mip_gap: libc::c_double,
    pub mir_cuts: i32,
    pub gmi_cuts: i32,
    pub cov_cuts: i32,
    pub clq_cuts: i32,
    pub presolve: i32,
    pub binarize: i32,
    pub fp_heur: i32,
    pub ps_heur: i32,
    pub ps_tm_lim: i32,
    pub sr_heur: i32,
    pub use_sol: i32,
    pub save_sol: *const i8,
    pub alien: i32,
    pub flip: i32,
    pub foo_bar: [libc::c_double; 23],
}
pub type __time_t = i64;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csa {
    pub integer_obj: i32,
    pub b_vars_exist: i32,
    pub i_vars_exist: i32,
    pub startsol: *const libc::c_double,
    pub ckind: *mut i32,
    pub clb: *mut libc::c_double,
    pub cub: *mut libc::c_double,
    pub true_obj: *mut libc::c_double,
    pub dir: i32,
    pub ncols: i32,
    pub GLOtstart: time_t,
    pub lp_ref: *mut glp_prob,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_proxy(
    mut lp: *mut glp_prob,
    mut zfinal: *mut libc::c_double,
    mut xfinal: *mut libc::c_double,
    mut initsol: *const libc::c_double,
    mut rel_impr: libc::c_double,
    mut tlim: i32,
    mut verbose: i32,
) -> i32 {
    let mut csa_: csa = csa {
        integer_obj: 0,
        b_vars_exist: 0,
        i_vars_exist: 0,
        startsol: 0 as *const libc::c_double,
        ckind: 0 as *mut i32,
        clb: 0 as *mut libc::c_double,
        cub: 0 as *mut libc::c_double,
        true_obj: 0 as *mut libc::c_double,
        dir: 0,
        ncols: 0,
        GLOtstart: 0,
        lp_ref: 0 as *mut glp_prob,
    };
    let mut csa: *mut csa = &mut csa_;
    let mut parm: glp_iocp = glp_iocp {
        msg_lev: 0,
        br_tech: 0,
        bt_tech: 0,
        tol_int: 0.,
        tol_obj: 0.,
        tm_lim: 0,
        out_frq: 0,
        out_dly: 0,
        cb_func: None,
        cb_info: 0 as *mut libc::c_void,
        cb_size: 0,
        pp_tech: 0,
        mip_gap: 0.,
        mir_cuts: 0,
        gmi_cuts: 0,
        cov_cuts: 0,
        clq_cuts: 0,
        presolve: 0,
        binarize: 0,
        fp_heur: 0,
        ps_heur: 0,
        ps_tm_lim: 0,
        sr_heur: 0,
        use_sol: 0,
        save_sol: 0 as *const i8,
        alien: 0,
        flip: 0,
        foo_bar: [0.; 23],
    };
    let mut parm_lp: glp_smcp = glp_smcp {
        msg_lev: 0,
        meth: 0,
        pricing: 0,
        r_test: 0,
        tol_bnd: 0.,
        tol_dj: 0.,
        tol_piv: 0.,
        obj_ll: 0.,
        obj_ul: 0.,
        it_lim: 0,
        tm_lim: 0,
        out_frq: 0,
        out_dly: 0,
        presolve: 0,
        excl: 0,
        shift: 0,
        aorn: 0,
        foo_bar: [0.; 33],
    };
    let mut tpeak: size_t = 0;
    let mut refine: i32 = 0;
    let mut tref_lim: i32 = 0;
    let mut err: i32 = 0;
    let mut cutoff_row: i32 = 0;
    let mut niter: i32 = 0;
    let mut status: i32 = 0;
    let mut i: i32 = 0;
    let mut tout: i32 = 0;
    let mut xref: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut xstar: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut zstar: libc::c_double = 0.;
    let mut tela: libc::c_double = 0.;
    let mut cutoff: libc::c_double = 0.;
    let mut zz: libc::c_double = 0.;
    memset(csa as *mut libc::c_void, 0 as i32, ::core::mem::size_of::<csa>() as u64);
    (*csa).dir = glp_get_obj_dir(lp);
    (*csa).ncols = glp_get_num_cols(lp);
    get_info(csa, lp);
    check_integrality(csa);
    if (*csa).b_vars_exist == 0 as i32 {
        if verbose != 0 {
            glp_printf(
                b"The problem has not binary variables. Proximity search cannot be used.\n\0"
                    as *const u8 as *const i8,
            );
        }
        glp_free((*csa).ckind as *mut libc::c_void);
        glp_free((*csa).clb as *mut libc::c_void);
        glp_free((*csa).cub as *mut libc::c_void);
        glp_free((*csa).true_obj as *mut libc::c_void);
        return -(1 as i32);
    }
    xref = glp_alloc(
        (*csa).ncols + 1 as i32,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    refine = check_ref(csa, lp, xref);
    xstar = glp_alloc(
        (*csa).ncols + 1 as i32,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    if verbose != 0 {
        glp_printf(b"Applying PROXY heuristic...\n\0" as *const u8 as *const i8);
    }
    (*csa).GLOtstart = second() as time_t;
    glp_init_iocp(&mut parm);
    glp_init_smcp(&mut parm_lp);
    parm.bt_tech = 4 as i32;
    if rel_impr <= 0.0f64 {
        rel_impr = 0.01f64;
    }
    if tlim <= 0 as i32 {
        tlim = 2147483647 as i32;
    }
    if verbose != 0 {
        glp_printf(
            b"Proxy's time limit set to %d seconds.\n\0" as *const u8 as *const i8,
            tlim / 1000 as i32,
        );
        glp_printf(
            b"Proxy's relative improvement set to %2.2lf %c.\n\0" as *const u8
                as *const i8,
            rel_impr * 100 as i32 as libc::c_double,
            37 as i32,
        );
    }
    parm_lp.tm_lim = tlim;
    parm.mip_gap = 9999999.9f64;
    if verbose != 0 {
        glp_printf(
            b"Searching for a feasible solution...\n\0" as *const u8 as *const i8,
        );
    }
    if !initsol.is_null() {
        (*csa).startsol = initsol;
        parm.cb_func = Some(
            callback as unsafe extern "C" fn(*mut glp_tree, *mut libc::c_void) -> (),
        );
        parm.cb_info = csa as *mut libc::c_void;
        if verbose != 0 {
            glp_printf(b"Input solution found.\n\0" as *const u8 as *const i8);
        }
    }
    tout = glp_term_out(0 as i32);
    err = glp_simplex(lp, &mut parm_lp);
    glp_term_out(tout);
    status = glp_get_status(lp);
    if status != 5 as i32 {
        if verbose != 0 {
            glp_printf(b"Proxy heuristic terminated.\n\0" as *const u8 as *const i8);
        }
        glp_free(xref as *mut libc::c_void);
        glp_free(xstar as *mut libc::c_void);
        deallocate(csa, refine);
        return -(1 as i32);
    }
    tela = elapsed_time(csa);
    if tlim as libc::c_double - tela * 1000 as i32 as libc::c_double
        <= 0 as i32 as libc::c_double
    {
        if verbose != 0 {
            glp_printf(
                b"Time limit exceeded. Proxy could not find optimal solution to LP relaxation.\n\0"
                    as *const u8 as *const i8,
            );
            glp_printf(b"Proxy heuristic aborted.\n\0" as *const u8 as *const i8);
        }
        glp_free(xref as *mut libc::c_void);
        glp_free(xstar as *mut libc::c_void);
        deallocate(csa, refine);
        return -(1 as i32);
    }
    parm.tm_lim = (tlim as libc::c_double - tela * 1000 as i32 as libc::c_double) as i32;
    tref_lim = ((tlim as libc::c_double - tela * 1000 as i32 as libc::c_double)
        / 20 as i32 as libc::c_double) as i32;
    tout = glp_term_out(0 as i32);
    err = glp_intopt(lp, &mut parm);
    glp_term_out(tout);
    status = glp_mip_status(lp);
    if status == 4 as i32 || status == 1 as i32 {
        if err == 0x9 as i32 {
            if verbose != 0 {
                glp_printf(
                    b"Time limit exceeded. Proxy could not find an initial integer feasible solution.\n\0"
                        as *const u8 as *const i8,
                );
                glp_printf(b"Proxy heuristic aborted.\n\0" as *const u8 as *const i8);
            }
        } else if verbose != 0 {
            glp_printf(
                b"Proxy could not find an initial integer feasible solution.\n\0"
                    as *const u8 as *const i8,
            );
            glp_printf(b"Proxy heuristic aborted.\n\0" as *const u8 as *const i8);
        }
        glp_free(xref as *mut libc::c_void);
        glp_free(xstar as *mut libc::c_void);
        deallocate(csa, refine);
        return -(1 as i32);
    }
    get_sol(csa, lp, xstar);
    zstar = glp_mip_obj_val(lp);
    if verbose != 0 {
        glp_printf(b">>>>> first solution = %e;\n\0" as *const u8 as *const i8, zstar);
    }
    if err == 0x9 as i32 {
        if verbose != 0 {
            glp_printf(
                b"Time limit exceeded. Proxy heuristic terminated.\n\0" as *const u8
                    as *const i8,
            );
        }
    } else {
        tela = elapsed_time(csa);
        tpeak = 0 as i32 as size_t;
        glp_mem_usage(0 as *mut i32, 0 as *mut i32, 0 as *mut size_t, &mut tpeak);
        if verbose != 0 {
            glp_printf(
                b"Time used: %3.1lf secs.  Memory used: %2.1lf Mb\n\0" as *const u8
                    as *const i8,
                tela,
                tpeak as libc::c_double / 1048576 as i32 as libc::c_double,
            );
            glp_printf(b"Starting proximity search...\n\0" as *const u8 as *const i8);
        }
        cutoff_row = add_cutoff(csa, lp);
        if (*csa).dir == 2 as i32 {
            glp_set_obj_dir(lp, 1 as i32);
        }
        niter = 0 as i32;
        loop {
            niter += 1;
            niter;
            redefine_obj(lp, xstar, (*csa).ncols, (*csa).ckind, (*csa).clb, (*csa).cub);
            cutoff = update_cutoff(csa, lp, zstar, cutoff_row, rel_impr);
            tela = elapsed_time(csa);
            if tlim as libc::c_double - tela * 1000 as i32 as libc::c_double
                <= 0 as i32 as libc::c_double
            {
                if verbose != 0 {
                    glp_printf(
                        b"Time limit exceeded. Proxy heuristic terminated.\n\0"
                            as *const u8 as *const i8,
                    );
                }
                break;
            } else {
                parm_lp.tm_lim = (tlim as libc::c_double
                    - tela * 1000 as i32 as libc::c_double) as i32;
                tout = glp_term_out(0 as i32);
                err = glp_simplex(lp, &mut parm_lp);
                glp_term_out(tout);
                status = glp_get_status(lp);
                if status != 5 as i32 {
                    if status == 4 as i32 {
                        if verbose != 0 {
                            glp_printf(
                                b"Bound exceeded = %f. \0" as *const u8 as *const i8,
                                cutoff,
                            );
                        }
                    }
                    if verbose != 0 {
                        glp_printf(
                            b"Proxy heuristic terminated.\n\0" as *const u8 as *const i8,
                        );
                    }
                    break;
                } else {
                    tela = elapsed_time(csa);
                    if tlim as libc::c_double - tela * 1000 as i32 as libc::c_double
                        <= 0 as i32 as libc::c_double
                    {
                        if verbose != 0 {
                            glp_printf(
                                b"Time limit exceeded. Proxy heuristic terminated.\n\0"
                                    as *const u8 as *const i8,
                            );
                        }
                        break;
                    } else {
                        parm.tm_lim = (tlim as libc::c_double
                            - tela * 1000 as i32 as libc::c_double) as i32;
                        parm.cb_func = None;
                        tout = glp_term_out(0 as i32);
                        err = glp_intopt(lp, &mut parm);
                        glp_term_out(tout);
                        status = glp_mip_status(lp);
                        if status == 4 as i32 {
                            if verbose != 0 {
                                glp_printf(
                                    b"Bound exceeded = %f. Proxy heuristic terminated.\n\0"
                                        as *const u8 as *const i8,
                                    cutoff,
                                );
                            }
                            break;
                        } else if status == 1 as i32 {
                            if err == 0x9 as i32 {
                                if verbose != 0 {
                                    glp_printf(
                                        b"Time limit exceeded. Proxy heuristic terminated.\n\0"
                                            as *const u8 as *const i8,
                                    );
                                }
                            } else if verbose != 0 {
                                glp_printf(
                                    b"Proxy terminated unexpectedly.\n\0" as *const u8
                                        as *const i8,
                                );
                            }
                            break;
                        } else if status == 2 as i32 || status == 5 as i32 {
                            get_sol(csa, lp, xstar);
                            zz = objval((*csa).ncols, xstar, (*csa).true_obj);
                            if zz < zstar && (*csa).dir == 1 as i32
                                || zz > zstar && (*csa).dir == 2 as i32
                            {
                                if refine != 0 {
                                    array_copy(1 as i32, (*csa).ncols + 1 as i32, xstar, xref);
                                    err = do_refine(
                                        csa,
                                        (*csa).lp_ref,
                                        (*csa).ncols,
                                        (*csa).ckind,
                                        xref,
                                        &mut tlim,
                                        tref_lim,
                                        verbose,
                                    );
                                    if err == 0 {
                                        let mut zref: libc::c_double = objval(
                                            (*csa).ncols,
                                            xref,
                                            (*csa).true_obj,
                                        );
                                        if zref < zz && (*csa).dir == 1 as i32
                                            || zref > zz && (*csa).dir == 2 as i32
                                        {
                                            zz = zref;
                                            array_copy(1 as i32, (*csa).ncols + 1 as i32, xref, xstar);
                                        }
                                    }
                                }
                                zstar = zz;
                                tela = elapsed_time(csa);
                                if verbose != 0 {
                                    glp_printf(
                                        b">>>>> it: %3d:   mip = %e;   elapsed time %3.1lf sec.s\n\0"
                                            as *const u8 as *const i8,
                                        niter,
                                        zstar,
                                        tela,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    tela = elapsed_time(csa);
    glp_mem_usage(0 as *mut i32, 0 as *mut i32, 0 as *mut size_t, &mut tpeak);
    if verbose != 0 {
        glp_printf(
            b"Time used: %3.1lf.  Memory used: %2.1lf Mb\n\0" as *const u8 as *const i8,
            tela,
            tpeak as libc::c_double / 1048576 as i32 as libc::c_double,
        );
    }
    *zfinal = zstar;
    i = 1 as i32;
    while i < (*csa).ncols + 1 as i32 {
        *xfinal.offset(i as isize) = *xstar.offset(i as isize);
        i += 1;
        i;
    }
    glp_free(xref as *mut libc::c_void);
    glp_free(xstar as *mut libc::c_void);
    deallocate(csa, refine);
    return 0 as i32;
}
unsafe extern "C" fn callback(mut tree: *mut glp_tree, mut info: *mut libc::c_void) {
    let mut csa: *mut csa = info as *mut csa;
    match glp_ios_reason(tree) {
        3 => {
            glp_ios_heur_sol(tree, (*csa).startsol);
        }
        _ => {}
    };
}
unsafe extern "C" fn get_info(mut csa: *mut csa, mut lp: *mut glp_prob) {
    let mut i: i32 = 0;
    (*csa).ckind = glp_alloc(
        (*csa).ncols + 1 as i32,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    (*csa).clb = glp_alloc(
        (*csa).ncols + 1 as i32,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    (*csa).cub = glp_alloc(
        (*csa).ncols + 1 as i32,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    (*csa).true_obj = glp_alloc(
        (*csa).ncols + 1 as i32,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    i = 1 as i32;
    while i < (*csa).ncols + 1 as i32 {
        *((*csa).ckind).offset(i as isize) = glp_get_col_kind(lp, i);
        *((*csa).clb).offset(i as isize) = glp_get_col_lb(lp, i);
        *((*csa).cub).offset(i as isize) = glp_get_col_ub(lp, i);
        *((*csa).true_obj).offset(i as isize) = glp_get_obj_coef(lp, i);
        i += 1;
        i;
    }
    *((*csa).true_obj).offset(0 as i32 as isize) = glp_get_obj_coef(lp, 0 as i32);
}
unsafe extern "C" fn is_integer(mut csa: *mut csa) -> i32 {
    let mut i: i32 = 0;
    (*csa).integer_obj = 1 as i32;
    i = 1 as i32;
    while i < (*csa).ncols + 1 as i32 {
        if fabs(*((*csa).true_obj).offset(i as isize))
            > 2147483647 as i32 as libc::c_double
        {
            (*csa).integer_obj = 0 as i32;
        }
        if fabs(*((*csa).true_obj).offset(i as isize))
            <= 2147483647 as i32 as libc::c_double
        {
            let mut tmp: libc::c_double = 0.;
            let mut rem: libc::c_double = 0.;
            if fabs(*((*csa).true_obj).offset(i as isize))
                - floor(fabs(*((*csa).true_obj).offset(i as isize))) < 0.5f64
            {
                tmp = floor(fabs(*((*csa).true_obj).offset(i as isize)));
            } else {
                tmp = ceil(fabs(*((*csa).true_obj).offset(i as isize)));
            }
            rem = fabs(*((*csa).true_obj).offset(i as isize)) - tmp;
            rem = fabs(rem);
            if rem > 1e-6f64 {
                (*csa).integer_obj = 0 as i32;
            }
        }
        i += 1;
        i;
    }
    return (*csa).integer_obj;
}
unsafe extern "C" fn check_integrality(mut csa: *mut csa) {
    let mut i: i32 = 0;
    (*csa).integer_obj = is_integer(csa);
    (*csa).b_vars_exist = 0 as i32;
    (*csa).i_vars_exist = 0 as i32;
    i = 1 as i32;
    while i < (*csa).ncols + 1 as i32 {
        if *((*csa).ckind).offset(i as isize) == 2 as i32 {
            (*csa).i_vars_exist = 1 as i32;
        } else if *((*csa).ckind).offset(i as isize) == 3 as i32 {
            (*csa).b_vars_exist = 1 as i32;
        } else {
            (*csa).integer_obj = 0 as i32;
        }
        i += 1;
        i;
    }
}
unsafe extern "C" fn check_ref(
    mut csa: *mut csa,
    mut lp: *mut glp_prob,
    mut xref: *mut libc::c_double,
) -> i32 {
    let mut refine: i32 = 0 as i32;
    let mut i: i32 = 0;
    i = 1 as i32;
    while i < (*csa).ncols + 1 as i32 {
        if *((*csa).ckind).offset(i as isize) != 3 as i32 {
            refine = 1 as i32;
            break;
        } else {
            i += 1;
            i;
        }
    }
    if refine != 0 {
        (*csa).lp_ref = glp_create_prob();
        glp_copy_prob((*csa).lp_ref, lp, 1 as i32);
    }
    return refine;
}
unsafe extern "C" fn second() -> libc::c_double {
    return glp_time() / 1000.0f64;
}
unsafe extern "C" fn add_cutoff(mut csa: *mut csa, mut lp: *mut glp_prob) -> i32 {
    let mut obj_index: *mut i32 = glp_alloc(
        (*csa).ncols + 1 as i32,
        ::core::mem::size_of::<i32>() as u64 as i32,
    ) as *mut i32;
    let mut obj_value: *mut libc::c_double = glp_alloc(
        (*csa).ncols + 1 as i32,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    let mut obj_nzcnt: i32 = 0 as i32;
    let mut i: i32 = 0;
    let mut irow: i32 = 0;
    let mut rowname: *const i8 = 0 as *const i8;
    i = 1 as i32;
    while i < (*csa).ncols + 1 as i32 {
        if fabs(*((*csa).true_obj).offset(i as isize)) > 1e-6f64 {
            obj_nzcnt += 1;
            obj_nzcnt;
            *obj_index.offset(obj_nzcnt as isize) = i;
            *obj_value.offset(obj_nzcnt as isize) = *((*csa).true_obj)
                .offset(i as isize);
        }
        i += 1;
        i;
    }
    irow = glp_add_rows(lp, 1 as i32);
    rowname = b"Cutoff\0" as *const u8 as *const i8;
    glp_set_row_name(lp, irow, rowname);
    if (*csa).dir == 1 as i32 {
        glp_set_row_bnds(lp, irow, 3 as i32, 1e20f64, 1e20f64);
    } else {
        glp_set_row_bnds(lp, irow, 2 as i32, -1e20f64, -1e20f64);
    }
    glp_set_mat_row(
        lp,
        irow,
        obj_nzcnt,
        obj_index as *const i32,
        obj_value as *const libc::c_double,
    );
    glp_free(obj_index as *mut libc::c_void);
    glp_free(obj_value as *mut libc::c_void);
    return irow;
}
unsafe extern "C" fn get_sol(
    mut csa: *mut csa,
    mut lp: *mut glp_prob,
    mut xstar: *mut libc::c_double,
) {
    let mut i: i32 = 0;
    i = 1 as i32;
    while i < (*csa).ncols + 1 as i32 {
        *xstar.offset(i as isize) = glp_mip_col_val(lp, i);
        i += 1;
        i;
    }
}
unsafe extern "C" fn elapsed_time(mut csa: *mut csa) -> libc::c_double {
    let mut tela: libc::c_double = second() - (*csa).GLOtstart as libc::c_double;
    if tela < 0 as i32 as libc::c_double {
        tela += 86400.0f64;
    }
    return tela;
}
unsafe extern "C" fn redefine_obj(
    mut lp: *mut glp_prob,
    mut xtilde: *mut libc::c_double,
    mut ncols: i32,
    mut ckind: *mut i32,
    mut clb: *mut libc::c_double,
    mut cub: *mut libc::c_double,
) {
    let mut j: i32 = 0;
    let mut delta: *mut libc::c_double = glp_alloc(
        ncols + 1 as i32,
        ::core::mem::size_of::<libc::c_double>() as u64 as i32,
    ) as *mut libc::c_double;
    j = 1 as i32;
    while j < ncols + 1 as i32 {
        *delta.offset(j as isize) = 0.0f64;
        if !(*ckind.offset(j as isize) == 1 as i32) {
            if !(*cub.offset(j as isize) - *clb.offset(j as isize) < 0.5f64) {
                if *ckind.offset(j as isize) == 3 as i32 {
                    if *xtilde.offset(j as isize) > 0.5f64 {
                        *delta.offset(j as isize) = -1.0f64;
                    } else {
                        *delta.offset(j as isize) = 1.0f64;
                    }
                }
            }
        }
        j += 1;
        j;
    }
    j = 1 as i32;
    while j < ncols + 1 as i32 {
        glp_set_obj_coef(lp, j, *delta.offset(j as isize));
        j += 1;
        j;
    }
    glp_set_obj_coef(lp, 0 as i32, 0.0f64);
    glp_free(delta as *mut libc::c_void);
}
unsafe extern "C" fn update_cutoff(
    mut csa: *mut csa,
    mut lp: *mut glp_prob,
    mut zstar: libc::c_double,
    mut cutoff_row: i32,
    mut rel_impr: libc::c_double,
) -> libc::c_double {
    let mut cutoff: libc::c_double = 0.;
    zstar -= *((*csa).true_obj).offset(0 as i32 as isize);
    if (*csa).dir == 1 as i32 {
        cutoff = zstar - compute_delta(csa, zstar, rel_impr);
        glp_set_row_bnds(lp, cutoff_row, 3 as i32, cutoff, cutoff);
    } else {
        cutoff = zstar + compute_delta(csa, zstar, rel_impr);
        glp_set_row_bnds(lp, cutoff_row, 2 as i32, cutoff, cutoff);
    }
    return cutoff;
}
unsafe extern "C" fn compute_delta(
    mut csa: *mut csa,
    mut z: libc::c_double,
    mut rel_impr: libc::c_double,
) -> libc::c_double {
    let mut delta: libc::c_double = rel_impr * fabs(z);
    if (*csa).integer_obj != 0 {
        delta = ceil(delta);
    }
    return delta;
}
unsafe extern "C" fn objval(
    mut ncols: i32,
    mut x: *mut libc::c_double,
    mut true_obj: *mut libc::c_double,
) -> libc::c_double {
    let mut j: i32 = 0;
    let mut z: libc::c_double = 0.0f64;
    j = 1 as i32;
    while j < ncols + 1 as i32 {
        z += *x.offset(j as isize) * *true_obj.offset(j as isize);
        j += 1;
        j;
    }
    return z + *true_obj.offset(0 as i32 as isize);
}
unsafe extern "C" fn array_copy(
    mut begin: i32,
    mut end: i32,
    mut source: *mut libc::c_double,
    mut destination: *mut libc::c_double,
) {
    let mut i: i32 = 0;
    i = begin;
    while i < end {
        *destination.offset(i as isize) = *source.offset(i as isize);
        i += 1;
        i;
    }
}
unsafe extern "C" fn do_refine(
    mut csa: *mut csa,
    mut lp_ref: *mut glp_prob,
    mut ncols: i32,
    mut ckind: *mut i32,
    mut xref: *mut libc::c_double,
    mut tlim: *mut i32,
    mut tref_lim: i32,
    mut verbose: i32,
) -> i32 {
    let mut j: i32 = 0;
    let mut tout: i32 = 0;
    let mut refineStart: libc::c_double = second();
    let mut val: libc::c_double = 0.;
    let mut tela: libc::c_double = 0.;
    let mut tlimit: libc::c_double = 0.;
    if glp_get_num_cols(lp_ref) != ncols {
        if verbose != 0 {
            glp_printf(b"Error in Proxy refinement: \0" as *const u8 as *const i8);
            glp_printf(
                b"wrong number of columns (%d vs %d).\n\0" as *const u8 as *const i8,
                ncols,
                glp_get_num_cols(lp_ref),
            );
        }
        return 1 as i32;
    }
    val = -1.0f64;
    j = 1 as i32;
    while j < ncols + 1 as i32 {
        if *ckind.offset(j as isize) == 3 as i32 {
            val = 0.0f64;
            if *xref.offset(j as isize) > 0.5f64 {
                val = 1.0f64;
            }
            glp_set_col_bnds(lp_ref, j, 5 as i32, val, val);
        }
        j += 1;
        j;
    }
    if val > -1.0f64 {
        let mut parm_ref: glp_iocp = glp_iocp {
            msg_lev: 0,
            br_tech: 0,
            bt_tech: 0,
            tol_int: 0.,
            tol_obj: 0.,
            tm_lim: 0,
            out_frq: 0,
            out_dly: 0,
            cb_func: None,
            cb_info: 0 as *mut libc::c_void,
            cb_size: 0,
            pp_tech: 0,
            mip_gap: 0.,
            mir_cuts: 0,
            gmi_cuts: 0,
            cov_cuts: 0,
            clq_cuts: 0,
            presolve: 0,
            binarize: 0,
            fp_heur: 0,
            ps_heur: 0,
            ps_tm_lim: 0,
            sr_heur: 0,
            use_sol: 0,
            save_sol: 0 as *const i8,
            alien: 0,
            flip: 0,
            foo_bar: [0.; 23],
        };
        let mut parm_ref_lp: glp_smcp = glp_smcp {
            msg_lev: 0,
            meth: 0,
            pricing: 0,
            r_test: 0,
            tol_bnd: 0.,
            tol_dj: 0.,
            tol_piv: 0.,
            obj_ll: 0.,
            obj_ul: 0.,
            it_lim: 0,
            tm_lim: 0,
            out_frq: 0,
            out_dly: 0,
            presolve: 0,
            excl: 0,
            shift: 0,
            aorn: 0,
            foo_bar: [0.; 33],
        };
        let mut err: i32 = 0;
        let mut status: i32 = 0;
        glp_init_iocp(&mut parm_ref);
        parm_ref.presolve = 1 as i32;
        glp_init_smcp(&mut parm_ref_lp);
        parm_ref.tm_lim = tref_lim;
        if parm_ref.tm_lim > *tlim {
            parm_ref.tm_lim = *tlim;
        }
        parm_ref_lp.tm_lim = parm_ref.tm_lim;
        tout = glp_term_out(0 as i32);
        if (*csa).i_vars_exist == 1 as i32 {
            err = glp_intopt(lp_ref, &mut parm_ref);
        } else {
            err = glp_simplex(lp_ref, &mut parm_ref_lp);
        }
        glp_term_out(tout);
        if (*csa).i_vars_exist == 1 as i32 {
            status = glp_mip_status(lp_ref);
        } else {
            status = glp_get_status(lp_ref);
        }
        match status {
            5 | 2 => {}
            _ => {
                status = 1 as i32;
            }
        }
        if status == 1 as i32 {
            if err == 0x9 as i32 {
                return 1 as i32;
            }
        }
        j = 1 as i32;
        while j < ncols + 1 as i32 {
            if *ckind.offset(j as isize) != 3 as i32 {
                if (*csa).i_vars_exist == 1 as i32 {
                    *xref.offset(j as isize) = glp_mip_col_val(lp_ref, j);
                } else {
                    *xref.offset(j as isize) = glp_get_col_prim(lp_ref, j);
                }
            }
            j += 1;
            j;
        }
    }
    tela = second() - refineStart;
    return 0 as i32;
}
unsafe extern "C" fn deallocate(mut csa: *mut csa, mut refine: i32) {
    if refine != 0 {
        glp_delete_prob((*csa).lp_ref);
    }
    glp_free((*csa).ckind as *mut libc::c_void);
    glp_free((*csa).clb as *mut libc::c_void);
    glp_free((*csa).cub as *mut libc::c_void);
    glp_free((*csa).true_obj as *mut libc::c_void);
}