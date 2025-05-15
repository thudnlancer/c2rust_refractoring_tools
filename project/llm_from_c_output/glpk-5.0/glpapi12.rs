use std::f64;
use glp_sys::*;
use std::ptr;

pub fn glp_bf_exists(lp: *mut glp_prob) -> i32 {
    unsafe {
        if (*lp).m == 0 || (*lp).valid != 0 {
            1
        } else {
            0
        }
    }
}

pub fn glp_factorize(lp: *mut glp_prob) -> i32 {
    unsafe {
        let m = (*lp).m;
        let n = (*lp).n;
        let mut ret = 0;

        (*lp).valid = 0;

        // Build basis header
        let mut j = 0;
        for k in 1..=m + n {
            let stat = if k <= m {
                (*(*lp).row.add(k - 1)).stat
            } else {
                (*(*lp).col.add(k - m - 1)).stat
            };

            if k <= m {
                (*(*lp).row.add(k - 1)).bind = 0;
            } else {
                (*(*lp).col.add(k - m - 1)).bind = 0;
            }

            if stat == GLP_BS as i32 {
                j += 1;
                if j > m {
                    ret = GLP_EBADB;
                    break;
                }
                *(*lp).head.add(j) = k as i32;
                if k <= m {
                    (*(*lp).row.add(k - 1)).bind = j as i32;
                } else {
                    (*(*lp).col.add(k - m - 1)).bind = j as i32;
                }
            }
        }

        if j < m {
            ret = GLP_EBADB;
        }

        if m > 0 && ret == 0 {
            if (*lp).bfd.is_null() {
                (*lp).bfd = bfd_create_it();
            }

            match bfd_factorize((*lp).bfd, m, Some(b_col), lp as *mut _) {
                0 => {
                    (*lp).valid = 1;
                    ret = 0;
                }
                BFD_ESING => {
                    ret = GLP_ESING;
                }
                BFD_ECOND => {
                    ret = GLP_ECOND;
                }
                _ => unreachable!(),
            }
        }

        ret
    }
}

extern "C" fn b_col(info: *mut libc::c_void, j: i32, ind: *mut i32, val: *mut f64) -> i32 {
    unsafe {
        let lp = info as *mut glp_prob;
        let m = (*lp).m;
        let mut len = 0;

        assert!(1 <= j && j <= m);

        let k = *(*lp).head.add(j as usize);
        if k <= m {
            *ind.offset(0) = k;
            *val.offset(0) = 1.0;
            len = 1;
        } else {
            let mut aij = (*(*lp).col.add((k - m) as usize - 1)).ptr;
            while !aij.is_null() {
                len += 1;
                *ind.offset(len as isize - 1) = (*aij).row.i;
                *val.offset(len as isize - 1) = -(*aij).row.rii * (*aij).val * (*aij).col.sjj;
                aij = (*aij).c_next;
            }
        }

        len
    }
}

pub fn glp_bf_updated(lp: *mut glp_prob) -> i32 {
    unsafe {
        if !((*lp).m == 0 || (*lp).valid != 0) {
            panic!("glp_bf_update: basis factorization does not exist");
        }

        if (*lp).m == 0 {
            0
        } else {
            bfd_get_count((*lp).bfd)
        }
    }
}

pub fn glp_get_bfcp(P: *mut glp_prob, parm: *mut glp_bfcp) {
    unsafe {
        if (*P).bfd.is_null() {
            (*P).bfd = bfd_create_it();
        }
        bfd_get_bfcp((*P).bfd, parm);
    }
}

pub fn glp_set_bfcp(P: *mut glp_prob, parm: *const glp_bfcp) {
    unsafe {
        if (*P).bfd.is_null() {
            (*P).bfd = bfd_create_it();
        }

        if !parm.is_null() {
            let type_ = (*parm).type_;
            if !(type_ == GLP_BF_LUF + GLP_BF_FT ||
                 type_ == GLP_BF_LUF + GLP_BF_BG ||
                 type_ == GLP_BF_LUF + GLP_BF_GR ||
                 type_ == GLP_BF_BTF + GLP_BF_BG ||
                 type_ == GLP_BF_BTF + GLP_BF_GR)
            {
                panic!("glp_set_bfcp: type = 0x{:02X}; invalid parameter", type_);
            }

            let piv_tol = (*parm).piv_tol;
            if !(0.0 < piv_tol && piv_tol < 1.0) {
                panic!("glp_set_bfcp: piv_tol = {}; invalid parameter", piv_tol);
            }

            let piv_lim = (*parm).piv_lim;
            if piv_lim < 1 {
                panic!("glp_set_bfcp: piv_lim = {}; invalid parameter", piv_lim);
            }

            let suhl = (*parm).suhl;
            if !(suhl == GLP_ON || suhl == GLP_OFF) {
                panic!("glp_set_bfcp: suhl = {}; invalid parameter", suhl);
            }

            let eps_tol = (*parm).eps_tol;
            if !(0.0 <= eps_tol && eps_tol <= 1e-6) {
                panic!("glp_set_bfcp: eps_tol = {}; invalid parameter", eps_tol);
            }

            let nfs_max = (*parm).nfs_max;
            if !(1 <= nfs_max && nfs_max <= 32767) {
                panic!("glp_set_bfcp: nfs_max = {}; invalid parameter", nfs_max);
            }

            let nrs_max = (*parm).nrs_max;
            if !(1 <= nrs_max && nrs_max <= 32767) {
                panic!("glp_set_bfcp: nrs_max = {}; invalid parameter", nrs_max);
            }
        }

        bfd_set_bfcp((*P).bfd, parm);
    }
}

pub fn glp_get_bhead(lp: *mut glp_prob, k: i32) -> i32 {
    unsafe {
        if !((*lp).m == 0 || (*lp).valid != 0) {
            panic!("glp_get_bhead: basis factorization does not exist");
        }
        if !(1 <= k && k <= (*lp).m) {
            panic!("glp_get_bhead: k = {}; index out of range", k);
        }
        *(*lp).head.add(k as usize)
    }
}

pub fn glp_get_row_bind(lp: *mut glp_prob, i: i32) -> i32 {
    unsafe {
        if !((*lp).m == 0 || (*lp).valid != 0) {
            panic!("glp_get_row_bind: basis factorization does not exist");
        }
        if !(1 <= i && i <= (*lp).m) {
            panic!("glp_get_row_bind: i = {}; row number out of range", i);
        }
        (*(*lp).row.add(i as usize - 1)).bind
    }
}

pub fn glp_get_col_bind(lp: *mut glp_prob, j: i32) -> i32 {
    unsafe {
        if !((*lp).m == 0 || (*lp).valid != 0) {
            panic!("glp_get_col_bind: basis factorization does not exist");
        }
        if !(1 <= j && j <= (*lp).n) {
            panic!("glp_get_col_bind: j = {}; column number out of range", j);
        }
        (*(*lp).col.add(j as usize - 1)).bind
    }
}

pub fn glp_ftran(lp: *mut glp_prob, x: *mut f64) {
    unsafe {
        let m = (*lp).m;
        if !(m == 0 || (*lp).valid != 0) {
            panic!("glp_ftran: basis factorization does not exist");
        }

        // b" := R*b
        for i in 1..=m {
            *x.add(i as usize) *= (*(*lp).row.add(i as usize - 1)).rii;
        }

        // x" := inv(B")*b"
        if m > 0 {
            bfd_ftran((*lp).bfd, x);
        }

        // x := SB*x"
        for i in 1..=m {
            let k = *(*lp).head.add(i as usize);
            if k <= m {
                *x.add(i as usize) /= (*(*lp).row.add(k as usize - 1)).rii;
            } else {
                *x.add(i as usize) *= (*(*lp).col.add((k - m) as usize - 1)).sjj;
            }
        }
    }
}

pub fn glp_btran(lp: *mut glp_prob, x: *mut f64) {
    unsafe {
        let m = (*lp).m;
        if !(m == 0 || (*lp).valid != 0) {
            panic!("glp_btran: basis factorization does not exist");
        }

        // b" := SB*b
        for i in 1..=m {
            let k = *(*lp).head.add(i as usize);
            if k <= m {
                *x.add(i as usize) /= (*(*lp).row.add(k as usize - 1)).rii;
            } else {
                *x.add(i as usize) *= (*(*lp).col.add((k - m) as usize - 1)).sjj;
            }
        }

        // x" := inv[(B")']*b"
        if m > 0 {
            bfd_btran((*lp).bfd, x);
        }

        // x := R*x"
        for i in 1..=m {
            *x.add(i as usize) *= (*(*lp).row.add(i as usize - 1)).rii;
        }
    }
}

// Additional functions would follow the same pattern, converting C to safe Rust
// while maintaining the same functionality and error handling.