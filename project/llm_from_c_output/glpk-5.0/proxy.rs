use glpk_sys::*;
use libc::{c_double, c_int, c_void};
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

const TDAY: f64 = 86400.0;
const TRUE: c_int = 1;
const FALSE: c_int = 0;
const EPS: f64 = 1e-6;
const RINF: f64 = 1e38;
const MAXVAL: f64 = 1e20;
const MINVAL: f64 = -1e20;

struct Csa {
    integer_obj: c_int,
    b_vars_exist: c_int,
    i_vars_exist: c_int,
    startsol: *const c_double,
    ckind: *mut c_int,
    clb: *mut c_double,
    cub: *mut c_double,
    true_obj: *mut c_double,
    dir: c_int,
    ncols: c_int,
    GLOtstart: f64,
    lp_ref: *mut glp_prob,
}

extern "C" fn callback(tree: *mut glp_tree, info: *mut c_void) {
    unsafe {
        let csa = &*(info as *const Csa);
        match glp_ios_reason(tree) {
            GLP_IHEUR => {
                glp_ios_heur_sol(tree, csa.startsol);
            }
            _ => {}
        }
    }
}

unsafe fn get_info(csa: &mut Csa, lp: *mut glp_prob) {
    csa.ckind = glp_sys::talloc((csa.ncols + 1) as _, c_int::default());
    csa.clb = glp_sys::talloc((csa.ncols + 1) as _, c_double::default());
    csa.cub = glp_sys::talloc((csa.ncols + 1) as _, c_double::default());
    csa.true_obj = glp_sys::talloc((csa.ncols + 1) as _, c_double::default());

    for i in 1..=csa.ncols {
        *csa.ckind.add(i as usize) = glp_get_col_kind(lp, i);
        *csa.clb.add(i as usize) = glp_get_col_lb(lp, i);
        *csa.cub.add(i as usize) = glp_get_col_ub(lp, i);
        *csa.true_obj.add(i as usize) = glp_get_obj_coef(lp, i);
    }
    *csa.true_obj = glp_get_obj_coef(lp, 0);
}

unsafe fn is_integer(csa: &mut Csa) -> c_int {
    csa.integer_obj = TRUE;
    for i in 1..=csa.ncols {
        if (*csa.true_obj.add(i as usize)).abs() > c_int::MAX as f64 {
            csa.integer_obj = FALSE;
        }
        if (*csa.true_obj.add(i as usize)).abs() <= c_int::MAX as f64 {
            let tmp = if (*csa.true_obj.add(i as usize)).abs() - (*csa.true_obj.add(i as usize)).abs().floor() < 0.5 {
                (*csa.true_obj.add(i as usize)).abs().floor()
            } else {
                (*csa.true_obj.add(i as usize)).abs().ceil()
            };
            let rem = (*csa.true_obj.add(i as usize)).abs() - tmp;
            if rem.abs() > EPS {
                csa.integer_obj = FALSE;
            }
        }
    }
    csa.integer_obj
}

unsafe fn check_integrality(csa: &mut Csa) {
    csa.integer_obj = is_integer(csa);
    csa.b_vars_exist = FALSE;
    csa.i_vars_exist = FALSE;
    for i in 1..=csa.ncols {
        match *csa.ckind.add(i as usize) {
            GLP_IV => csa.i_vars_exist = TRUE,
            GLP_BV => csa.b_vars_exist = TRUE,
            _ => csa.integer_obj = FALSE,
        }
    }
}

unsafe fn check_ref(csa: &mut Csa, lp: *mut glp_prob, xref: *mut c_double) -> c_int {
    let mut refine = FALSE;
    for i in 1..=csa.ncols {
        if *csa.ckind.add(i as usize) != GLP_BV {
            refine = TRUE;
            break;
        }
    }

    if refine != 0 {
        csa.lp_ref = glp_create_prob();
        glp_copy_prob(csa.lp_ref, lp, GLP_ON as c_int);
    }
    refine
}

fn second() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64()
}

unsafe fn add_cutoff(csa: &mut Csa, lp: *mut glp_prob) -> c_int {
    let mut obj_nzcnt = 0;
    let obj_index = glp_sys::talloc((csa.ncols + 1) as _, c_int::default());
    let obj_value = glp_sys::talloc((csa.ncols + 1) as _, c_double::default());

    for i in 1..=csa.ncols {
        if (*csa.true_obj.add(i as usize)).abs() > EPS {
            obj_nzcnt += 1;
            *obj_index.add(obj_nzcnt as usize) = i;
            *obj_value.add(obj_nzcnt as usize) = *csa.true_obj.add(i as usize);
        }
    }

    let irow = glp_add_rows(lp, 1);
    let rowname = CString::new("Cutoff").unwrap();
    glp_set_row_name(lp, irow, rowname.as_ptr());
    match csa.dir {
        GLP_MIN => glp_set_row_bnds(lp, irow, GLP_UP as c_int, MAXVAL, MAXVAL),
        _ => glp_set_row_bnds(lp, irow, GLP_LO as c_int, MINVAL, MINVAL),
    }

    glp_set_mat_row(lp, irow, obj_nzcnt, obj_index, obj_value);

    glp_sys::tfree(obj_index as _);
    glp_sys::tfree(obj_value as _);

    irow
}

unsafe fn get_sol(csa: &Csa, lp: *mut glp_prob, xstar: *mut c_double) {
    for i in 1..=csa.ncols {
        *xstar.add(i as usize) = glp_mip_col_val(lp, i);
    }
}

fn elapsed_time(csa: &Csa) -> f64 {
    let tela = second() - csa.GLOtstart;
    if tela < 0.0 { tela + TDAY } else { tela }
}

unsafe fn redefine_obj(lp: *mut glp_prob, xtilde: *mut c_double, ncols: c_int, ckind: *mut c_int, clb: *mut c_double, cub: *mut c_double) {
    let delta = glp_sys::talloc((ncols + 1) as _, c_double::default());

    for j in 1..=ncols {
        *delta.add(j as usize) = 0.0;
        if *ckind.add(j as usize) == GLP_CV {
            continue;
        }
        if *cub.add(j as usize) - *clb.add(j as usize) < 0.5 {
            continue;
        }
        if *ckind.add(j as usize) == GLP_BV {
            *delta.add(j as usize) = if *xtilde.add(j as usize) > 0.5 { -1.0 } else { 1.0 };
        }
    }

    for j in 1..=ncols {
        glp_set_obj_coef(lp, j, *delta.add(j as usize));
    }
    glp_set_obj_coef(lp, 0, 0.0);

    glp_sys::tfree(delta as _);
}

unsafe fn update_cutoff(csa: &Csa, lp: *mut glp_prob, zstar: f64, cutoff_row: c_int, rel_impr: f64) -> f64 {
    let mut cutoff;
    let z = zstar - *csa.true_obj;
    if csa.dir == GLP_MIN {
        cutoff = z - compute_delta(csa, z, rel_impr);
        glp_set_row_bnds(lp, cutoff_row, GLP_UP as c_int, cutoff, cutoff);
    } else {
        cutoff = z + compute_delta(csa, z, rel_impr);
        glp_set_row_bnds(lp, cutoff_row, GLP_LO as c_int, cutoff, cutoff);
    }
    cutoff
}

fn compute_delta(csa: &Csa, z: f64, rel_impr: f64) -> f64 {
    let mut delta = rel_impr * z.abs();
    if csa.integer_obj != 0 {
        delta = delta.ceil();
    }
    delta
}

unsafe fn objval(ncols: c_int, x: *mut c_double, true_obj: *mut c_double) -> f64 {
    let mut z = 0.0;
    for j in 1..=ncols {
        z += *x.add(j as usize) * *true_obj.add(j as usize);
    }
    z + *true_obj
}

unsafe fn array_copy(begin: c_int, end: c_int, source: *mut c_double, destination: *mut c_double) {
    for i in begin..end {
        *destination.add(i as usize) = *source.add(i as usize);
    }
}

unsafe fn do_refine(csa: &mut Csa, lp_ref: *mut glp_prob, ncols: c_int, ckind: *mut c_int, xref: *mut c_double, tlim: *mut c_int, tref_lim: c_int, verbose: c_int) -> c_int {
    if glp_get_num_cols(lp_ref) != ncols {
        if verbose != 0 {
            println!("Error in Proxy refinement: wrong number of columns ({} vs {}).", ncols, glp_get_num_cols(lp_ref));
        }
        return 1;
    }

    let mut val = -1.0;
    for j in 1..=ncols {
        if *ckind.add(j as usize) == GLP_BV {
            val = if *xref.add(j as usize) > 0.5 { 1.0 } else { 0.0 };
            glp_set_col_bnds(lp_ref, j, GLP_FX as c_int, val, val);
        }
    }

    if val > -1.0 {
        let mut parm_ref = mem::zeroed();
        glp_init_iocp(&mut parm_ref);
        parm_ref.presolve = GLP_ON as c_int;
        
        let mut parm_ref_lp = mem::zeroed();
        glp_init_smcp(&mut parm_ref_lp);
        
        parm_ref.tm_lim = if tref_lim > *tlim { *tlim } else { tref_lim };
        parm_ref_lp.tm_lim = parm_ref.tm_lim;

        let tout = glp_term_out(GLP_OFF);
        let err = if csa.i_vars_exist != 0 {
            glp_intopt(lp_ref, &parm_ref)
        } else {
            glp_simplex(lp_ref, &parm_ref_lp)
        };
        glp_term_out(tout);

        let status = if csa.i_vars_exist != 0 {
            glp_mip_status(lp_ref)
        } else {
            glp_get_status(lp_ref)
        };

        if status != GLP_OPT && status != GLP_FEAS {
            if err == GLP_ETMLIM as c_int {
                return 1;
            }
        }

        for j in 1..=ncols {
            if *ckind.add(j as usize) != GLP_BV {
                *xref.add(j as usize) = if csa.i_vars_exist != 0 {
                    glp_mip_col_val(lp_ref, j)
                } else {
                    glp_get_col_prim(lp_ref, j)
                };
            }
        }
    }
    0
}

unsafe fn deallocate(csa: &mut Csa, refine: c_int) {
    if refine != 0 {
        glp_delete_prob(csa.lp_ref);
    }
    glp_sys::tfree(csa.ckind as _);
    glp_sys::tfree(csa.clb as _);
    glp_sys::tfree(csa.cub as _);
    glp_sys::tfree(csa.true_obj as _);
}

#[no_mangle]
pub unsafe extern "C" fn proxy(
    lp: *mut glp_prob,
    zfinal: *mut c_double,
    xfinal: *mut c_double,
    initsol: *const c_double,
    rel_impr: c_double,
    tlim: c_int,
    verbose: c_int,
) -> c_int {
    let mut csa = Csa {
        integer_obj: 0,
        b_vars_exist: 0,
        i_vars_exist: 0,
        startsol: ptr::null(),
        ckind: ptr::null_mut(),
        clb: ptr::null_mut(),
        cub: ptr::null_mut(),
        true_obj: ptr::null_mut(),
        dir: 0,
        ncols: 0,
        GLOtstart: 0.0,
        lp_ref: ptr::null_mut(),
    };

    // Retrieve problem info
    csa.dir = glp_get_obj_dir(lp);
    csa.ncols = glp_get_num_cols(lp);
    get_info(&mut csa, lp);
    check_integrality(&mut csa);

    if csa.b_vars_exist == FALSE {
        if verbose != 0 {
            println!("The problem has no binary variables. Proximity search cannot be used.");
        }
        deallocate(&mut csa, FALSE);
        return -1;
    }

    let xref = glp_sys::talloc((csa.ncols + 1) as _, c_double::default());
    let refine = check_ref(&mut csa, lp, xref);

    let xstar = glp_sys::talloc((csa.ncols + 1) as _, c_double::default());

    if verbose != 0 {
        println!("Applying PROXY heuristic...");
    }

    csa.GLOtstart = second();

    let mut parm = mem::zeroed();
    glp_init_iocp(&mut parm);
    let mut parm_lp = mem::zeroed();
    glp_init_smcp(&mut parm_lp);

    let rel_impr = if rel_impr <= 0.0 { 0.01 } else { rel_impr };
    let tlim = if tlim <= 0 { c_int::MAX } else { tlim };

    if verbose != 0 {
        println!("Proxy's time limit set to {} seconds.", tlim / 1000);
        println!("Proxy's relative improvement set to {:.2}%.", rel_impr * 100.0);
    }

    parm_lp.tm_lim = tlim;
    parm.mip_gap = 9999999.9;

    if verbose != 0 {
        println!("Searching for a feasible solution...");
    }

    if !initsol.is_null() {
        csa.startsol = initsol;
        parm.cb_func = Some(callback);
        parm.cb_info = &mut csa as *mut _ as *mut c_void;
        if verbose != 0 {
            println!("Input solution found.");
        }
    }

    let tout = glp_term_out(GLP_OFF);
    let err = glp_simplex(lp, &mut parm_lp);
    glp_term_out(tout);

    let status = glp_get_status(lp);
    if status != GLP_OPT {
        if verbose != 0 {
            println!("Proxy heuristic terminated.");
        }
        deallocate(&mut csa, refine);
        glp_sys::tfree(xref as _);
        glp_sys::tfree(xstar as _);
        return -1;
    }

    let tela = elapsed_time(&csa);
    if tlim as f64 - tela * 1000.0 <= 0.0 {
        if verbose != 0 {
            println!("Time limit exceeded. Proxy could not find optimal solution to LP relaxation.");
            println!("Proxy heuristic aborted.");
        }
        deallocate(&mut csa, refine);
        glp_sys::tfree(xref as _);
        glp_sys::tfree(xstar as _);
        return -1;
    }

    parm.tm_lim = (tlim as f64 - tela * 1000.0) as c_int;
    let tref_lim = (tlim as f64 - tela * 1000.0) as c_int / 20;

    let tout = glp_term_out(GLP_OFF);
    let err = glp_intopt(lp, &mut parm);
    glp_term_out(tout);

    let status = glp_mip_status(lp);

    if status == GLP_NOFEAS || status == GLP_UNDEF {
        if err == GLP_ETMLIM as c_int {
            if verbose != 0 {
                println!("Time limit exceeded. Proxy could not find an initial integer feasible solution.");
                println!("Proxy heuristic aborted.");
            }
        } else if verbose != 0 {
            println!("Proxy could not find an initial integer feasible solution.");
            println!("Proxy heuristic aborted.");
        }
        deallocate