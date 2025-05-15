use glp_sys::*;
use std::ffi::CString;
use std::os::raw::c_int;
use std::ptr;

struct BranchingData {
    dn_cnt: Vec<i32>,
    dn_sum: Vec<f64>,
    up_cnt: Vec<i32>,
    up_sum: Vec<f64>,
}

impl BranchingData {
    fn new(n: usize) -> Self {
        Self {
            dn_cnt: vec![0; n + 1],
            dn_sum: vec![0.0; n + 1],
            up_cnt: vec![0; n + 1],
            up_sum: vec![0.0; n + 1],
        }
    }
}

pub fn ios_choose_var(tree: *mut glp_tree, next: &mut i32) -> i32 {
    unsafe {
        let br_tech = (*tree).parm.br_tech as i32;
        match br_tech {
            GLP_BR_FFV => branch_first(tree, next),
            GLP_BR_LFV => branch_last(tree, next),
            GLP_BR_MFV => branch_mostf(tree, next),
            GLP_BR_DTH => branch_drtom(tree, next),
            GLP_BR_PCH => ios_pcost_branch(tree, next),
            _ => {
                assert!(tree != tree);
                0
            }
        }
    }
}

fn branch_first(tree: *mut glp_tree, next: &mut i32) -> i32 {
    unsafe {
        let n = (*tree).n as usize;
        let non_int = (*tree).non_int;
        let mut j = 0;

        for i in 1..=n {
            if *non_int.add(i) != 0 {
                j = i as i32;
                break;
            }
        }

        assert!(j >= 1 && j <= n as i32);
        let beta = glp_get_col_prim((*tree).mip, j);

        *next = if beta - beta.floor() < beta.ceil() - beta {
            GLP_DN_BRNCH
        } else {
            GLP_UP_BRNCH
        };

        j
    }
}

fn branch_last(tree: *mut glp_tree, next: &mut i32) -> i32 {
    unsafe {
        let n = (*tree).n as usize;
        let non_int = (*tree).non_int;
        let mut j = 0;

        for i in (1..=n).rev() {
            if *non_int.add(i) != 0 {
                j = i as i32;
                break;
            }
        }

        assert!(j >= 1 && j <= n as i32);
        let beta = glp_get_col_prim((*tree).mip, j);

        *next = if beta - beta.floor() < beta.ceil() - beta {
            GLP_DN_BRNCH
        } else {
            GLP_UP_BRNCH
        };

        j
    }
}

fn branch_mostf(tree: *mut glp_tree, next: &mut i32) -> i32 {
    unsafe {
        let n = (*tree).n as usize;
        let non_int = (*tree).non_int;
        let mip = (*tree).mip;
        let mut jj = 0;
        let mut most = f64::MAX;

        for j in 1..=n {
            if *non_int.add(j) != 0 {
                let beta = glp_get_col_prim(mip, j as i32);
                let temp = beta.floor() + 0.5;
                let diff = (beta - temp).abs();

                if most > diff {
                    most = diff;
                    jj = j as i32;
                    *next = if beta < temp {
                        GLP_DN_BRNCH
                    } else {
                        GLP_UP_BRNCH
                    };
                }
            }
        }

        jj
    }
}

fn branch_drtom(tree: *mut glp_tree, next: &mut i32) -> i32 {
    unsafe {
        let mip = (*tree).mip;
        let n = (*tree).n as usize;
        let non_int = (*tree).non_int;
        let mut jj = 0;
        let mut degrad = -1.0;
        let mut dd_dn = 0.0;
        let mut dd_up = 0.0;

        assert_eq!(glp_get_status(mip), GLP_OPT as i32);

        let mut ind: Vec<i32> = vec![0; n + 1];
        let mut val: Vec<f64> = vec![0.0; n + 1];

        for j in 1..=n {
            if *non_int.add(j) == 0 {
                continue;
            }

            let x = glp_get_col_prim(mip, j as i32);
            let len = glp_eval_tab_row(mip, (*mip).m + j as i32, ind.as_mut_ptr(), val.as_mut_ptr());

            let mut dz_dn = 0.0;
            let mut dz_up = 0.0;

            for kase in [-1, 1].iter() {
                let k = glp_dual_rtest(
                    mip,
                    len,
                    ind.as_ptr(),
                    val.as_ptr(),
                    *kase,
                    1e-9,
                );

                if k == 0 {
                    dz_dn = if (*mip).dir == GLP_MIN {
                        f64::MAX
                    } else {
                        -f64::MAX
                    };
                    continue;
                }

                let mut alfa = 0.0;
                for t in 1..=len {
                    if ind[t] == k {
                        alfa = val[t];
                        break;
                    }
                }

                let delta_j = if *kase < 0 {
                    x.floor() - x
                } else {
                    x.ceil() - x
                };

                let mut delta_k = delta_j / alfa;

                if k > (*mip).m as i32 {
                    let kind = glp_get_col_kind(mip, k - (*mip).m as i32);
                    if kind != GLP_CV as i32 {
                        if (delta_k - delta_k.round()).abs() > 1e-3 {
                            delta_k = if delta_k > 0.0 {
                                delta_k.ceil()
                            } else {
                                delta_k.floor()
                            };
                        }
                    }
                }

                let (stat, mut dk) = if k <= (*mip).m as i32 {
                    (
                        glp_get_row_stat(mip, k),
                        glp_get_row_dual(mip, k),
                    )
                } else {
                    (
                        glp_get_col_stat(mip, k - (*mip).m as i32),
                        glp_get_col_dual(mip, k - (*mip).m as i32),
                    )
                };

                match (*mip).dir {
                    GLP_MIN => {
                        if (stat == GLP_NL as i32 && dk < 0.0)
                            || (stat == GLP_NU as i32 && dk > 0.0)
                            || stat == GLP_NF as i32
                        {
                            dk = 0.0;
                        }
                    }
                    GLP_MAX => {
                        if (stat == GLP_NL as i32 && dk > 0.0)
                            || (stat == GLP_NU as i32 && dk < 0.0)
                            || stat == GLP_NF as i32
                        {
                            dk = 0.0;
                        }
                    }
                    _ => unreachable!(),
                }

                let delta_z = dk * delta_k;

                match (*mip).dir {
                    GLP_MIN => assert!(delta_z >= 0.0),
                    GLP_MAX => assert!(delta_z <= 0.0),
                    _ => unreachable!(),
                }

                if *kase < 0 {
                    dz_dn = delta_z;
                } else {
                    dz_up = delta_z;
                }
            }

            if degrad < dz_dn.abs() || degrad < dz_up.abs() {
                jj = j as i32;
                if dz_dn.abs() < dz_up.abs() {
                    *next = GLP_DN_BRNCH;
                    degrad = dz_up.abs();
                } else {
                    *next = GLP_UP_BRNCH;
                    degrad = dz_dn.abs();
                }
                dd_dn = dz_dn;
                dd_up = dz_up;

                if degrad == f64::MAX {
                    break;
                }
            }
        }

        assert!(jj >= 1 && jj <= n as i32);

        if degrad < 1e-6 * (1.0 + 0.001 * (*mip).obj_val.abs()) {
            jj = branch_mostf(tree, next);
        }

        if (*tree).parm.msg_lev >= GLP_MSG_DBG as i32 {
            println!("branch_drtom: column {} chosen to branch on", jj);
            if dd_dn.abs() == f64::MAX {
                println!("branch_drtom: down-branch is infeasible");
            } else {
                println!(
                    "branch_drtom: down-branch bound is {:.9}",
                    glp_get_obj_val(mip) + dd_dn
                );
            }
            if dd_up.abs() == f64::MAX {
                println!("branch_drtom: up-branch is infeasible");
            } else {
                println!(
                    "branch_drtom: up-branch bound is {:.9}",
                    glp_get_obj_val(mip) + dd_up
                );
            }
        }

        jj
    }
}

fn ios_pcost_init(tree: *mut glp_tree) -> *mut BranchingData {
    unsafe {
        let n = (*tree).n as usize;
        let data = Box::new(BranchingData::new(n));
        Box::into_raw(data)
    }
}

fn eval_degrad(p: *mut glp_prob, j: i32, bnd: f64) -> f64 {
    unsafe {
        assert_eq!(glp_get_status(p), GLP_OPT as i32);

        let lp = glp_create_prob();
        glp_copy_prob(lp, p, 0);
        glp_set_col_bnds(lp, j, GLP_FX as i32, bnd, bnd);

        let mut parm = glp_smcp {
            msg_lev: GLP_MSG_OFF as i32,
            meth: GLP_DUAL as i32,
            it_lim: 30,
            out_dly: 1000,
            ..Default::default()
        };

        let ret = glp_simplex(lp, &mut parm);

        let degrad = if ret == 0 || ret == GLP_EITLIM as i32 {
            if glp_get_prim_stat(lp) == GLP_NOFEAS as i32 {
                f64::MAX
            } else if glp_get_dual_stat(lp) == GLP_FEAS as i32 {
                let val = match (*p).dir {
                    GLP_MIN => (*lp).obj_val - (*p).obj_val,
                    GLP_MAX => (*p).obj_val - (*lp).obj_val,
                    _ => unreachable!(),
                };
                if val < 1e-6 * (1.0 + 0.001 * (*p).obj_val.abs()) {
                    0.0
                } else {
                    val
                }
            } else {
                0.0
            }
        } else {
            0.0
        };

        glp_delete_prob(lp);
        degrad
    }
}

fn ios_pcost_update(tree: *mut glp_tree) {
    unsafe {
        let csa = (*tree).pcost as *mut BranchingData;
        if csa.is_null() || (*tree).curr.is_null() || (*(*tree).curr).up.is_null() {
            return;
        }

        let j = (*(*(*tree).curr).up).br_var;
        assert!(j >= 1 && j <= (*tree).n);

        let dx = (*tree).mip.col[j as usize].prim - (*(*tree).curr).up.br_val;
        assert_ne!(dx, 0.0);

        let dz = (*tree).mip.obj_val - (*(*tree).curr).up.lp_obj;
        let psi = dz.abs() / dx.abs();

        if dx < 0.0 {
            (*csa).dn_cnt[j as usize] += 1;
            (*csa).dn_sum[j as usize] += psi;
        } else {
            (*csa).up_cnt[j as usize] += 1;
            (*csa).up_sum[j as usize] += psi;
        }
    }
}

fn ios_pcost_free(tree: *mut glp_tree) {
    unsafe {
        if !(*tree).pcost.is_null() {
            let csa = Box::from_raw((*tree).pcost as *mut BranchingData);
            (*tree).pcost = ptr::null_mut();
        }
    }
}

fn eval_psi(tree: *mut glp_tree, j: i32, brnch: i32) -> f64 {
    unsafe {
        let csa = (*tree).pcost as *mut BranchingData;
        assert!(!csa.is_null());
        assert!(j >= 1 && j <= (*tree).n);

        match brnch {
            GLP_DN_BRNCH => {
                if (*csa).dn_cnt[j as usize] == 0 {
                    let beta = (*tree).mip.col[j as usize].prim;
                    let degrad = eval_degrad((*tree).mip, j, beta.floor());
                    if degrad == f64::MAX {
                        return f64::MAX;
                    }
                    (*csa).dn_cnt[j as usize] = 1;
                    (*csa).dn_sum[j as usize] = degrad / (beta - beta.floor());
                }
                (*csa).dn_sum[j as usize] / (*csa).dn_cnt[j as usize] as f64
            }
            GLP_UP_BRNCH => {
                if (*csa).up_cnt[j as usize] == 0 {
                    let beta = (*tree).mip.col[j as usize].prim;
                    let degrad = eval_degrad((*tree).mip, j, beta.ceil());
                    if degrad == f64::MAX {
                        return f64::MAX;
                    }
                    (*csa).up_cnt[j as usize] = 1;
                    (*csa).up_sum[j as usize] = degrad / (beta.ceil() - beta);
                }
                (*csa).up_sum[j as usize] / (*csa).up_cnt[j as usize] as f64
            }
            _ => unreachable!(),
        }
    }
}

fn ios_pcost_branch(tree: *mut glp_tree, next: &mut i32) -> i32 {
    unsafe {
        if (*tree).pcost.is_null() {
            (*tree).pcost = ios_pcost_init(tree);
        }

        let mut jjj = 0;
        let mut dmax = -1.0;
        let mut sel = 0;

        for j in 1..=(*tree).n {
            if glp_ios_can_branch(tree, j) == 0 {
                continue;
            }

            let beta = (*tree).mip.col[j as usize].prim;

            let psi_dn = eval_psi(tree, j, GLP_DN_BRNCH);
            if psi_dn == f64::MAX {
                jjj = j;
                sel = GLP_DN_BRNCH;
                break;
            }
            let d1 = psi_dn * (beta - beta.floor());

            let psi_up = eval_psi(tree, j, GLP_UP_BRNCH);
            if psi_up == f64::MAX {
                jjj = j;
                sel = GLP_UP_BRNCH;
                break;
            }
            let d2 = psi_up * (beta.ceil() - beta);

            let d = if d1 > d2 { d1 } else { d2 };
            if dmax < d {
                dmax = d;
                jjj = j;
                sel = if d1 <= d2 {
                    GLP_DN_BRNCH
                } else {
                    GLP_UP_BRNCH
                };
            }
        }

        if dmax == 0.0 {
            jjj = branch_mostf(tree, &mut sel);
        }

        *next = sel;
        jjj
    }
}