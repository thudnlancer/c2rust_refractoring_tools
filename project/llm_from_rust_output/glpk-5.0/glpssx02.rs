use std::ffi::CString;
use std::os::raw::{c_int, c_double, c_void, c_char, c_uint};
use std::ptr;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct mpz {
    pub val: c_int,
    pub ptr: *mut mpz_seg,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct mpz_seg {
    pub d: [c_uint; 6],
    pub next: *mut mpz_seg,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct mpq {
    pub p: mpz,
    pub q: mpz,
}

pub type mpq_t = *mut mpq;

#[repr(C)]
#[derive(Debug)]
pub struct SSX {
    pub m: c_int,
    pub n: c_int,
    pub type_: *mut c_int,
    pub lb: *mut mpq_t,
    pub ub: *mut mpq_t,
    pub dir: c_int,
    pub coef: *mut mpq_t,
    pub a_ptr: *mut c_int,
    pub a_ind: *mut c_int,
    pub a_val: *mut mpq_t,
    pub stat: *mut c_int,
    pub q_row: *mut c_int,
    pub q_col: *mut c_int,
    pub binv: *mut c_void,
    pub bbar: *mut mpq_t,
    pub pi: *mut mpq_t,
    pub cbar: *mut mpq_t,
    pub p: c_int,
    pub rho: *mut mpq_t,
    pub ap: *mut mpq_t,
    pub q: c_int,
    pub aq: *mut mpq_t,
    pub q_dir: c_int,
    pub p_stat: c_int,
    pub delta: mpq_t,
    pub msg_lev: c_int,
    pub it_lim: c_int,
    pub it_cnt: c_int,
    pub tm_lim: c_double,
    pub out_frq: c_double,
    pub tm_beg: c_double,
    pub tm_lag: c_double,
}

extern "C" {
    fn glp_difftime(t1: c_double, t0: c_double) -> c_double;
    fn glp_time() -> c_double;
    fn glp_free(ptr: *mut c_void);
    fn glp_alloc(n: c_int, size: c_int) -> *mut c_void;
    fn glp_assert_(expr: *const c_char, file: *const c_char, line: c_int);
    fn glp_printf(fmt: *const c_char, ...);
    fn _glp_mpq_add(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_sub(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_mul(z: mpq_t, x: mpq_t, y: mpq_t);
    fn _glp_mpq_neg(z: mpq_t, x: mpq_t);
    fn _glp_mpq_cmp(x: mpq_t, y: mpq_t) -> c_int;
    fn _glp_mpq_sgn(x: mpq_t) -> c_int;
    fn _glp_mpq_get_d(x: mpq_t) -> c_double;
    fn _glp_mpq_set_si(x: mpq_t, p: c_int, q: c_uint);
    fn _glp_mpq_set(z: mpq_t, x: mpq_t);
    fn _glp_mpq_clear(x: mpq_t);
    fn _glp_mpq_init() -> mpq_t;
    fn _glp_ssx_factorize(ssx: *mut SSX) -> c_int;
    fn _glp_ssx_eval_bbar(ssx: *mut SSX);
    fn _glp_ssx_eval_pi(ssx: *mut SSX);
    fn _glp_ssx_eval_cbar(ssx: *mut SSX);
    fn _glp_ssx_eval_rho(ssx: *mut SSX);
    fn _glp_ssx_eval_row(ssx: *mut SSX);
    fn _glp_ssx_eval_col(ssx: *mut SSX);
    fn _glp_ssx_chuzc(ssx: *mut SSX);
    fn _glp_ssx_chuzr(ssx: *mut SSX);
    fn _glp_ssx_update_bbar(ssx: *mut SSX);
    fn _glp_ssx_update_pi(ssx: *mut SSX);
    fn _glp_ssx_update_cbar(ssx: *mut SSX);
    fn _glp_ssx_change_basis(ssx: *mut SSX);
}

fn show_progress(ssx: &mut SSX, phase: c_int) {
    unsafe {
        let mut def = 0;
        for i in 1..=ssx.m {
            let k = *ssx.q_col.offset(i as isize);
            if *ssx.type_.offset(k as isize) == 4 {
                def += 1;
            }
        }

        let phase_str = if phase == 1 { " " } else { "*" };
        let desc_str = if phase == 1 { "infsum" } else { "objval" };
        let val = _glp_mpq_get_d(*ssx.bbar.offset(0));

        glp_printf(
            CString::new("%s%6d:   %s = %22.15g   (%d)\n").unwrap().as_ptr(),
            CString::new(phase_str).unwrap().as_ptr(),
            ssx.it_cnt,
            CString::new(desc_str).unwrap().as_ptr(),
            val,
            def,
        );

        ssx.tm_lag = glp_time();
    }
}

#[no_mangle]
pub extern "C" fn _glp_ssx_phase_i(ssx: *mut SSX) -> c_int {
    unsafe {
        let ssx = &mut *ssx;
        let m = ssx.m;
        let n = ssx.n;
        let type_ = ssx.type_;
        let lb = ssx.lb;
        let ub = ssx.ub;
        let coef = ssx.coef;
        let a_ptr = ssx.a_ptr;
        let a_ind = ssx.a_ind;
        let a_val = ssx.a_val;
        let q_col = ssx.q_col;
        let bbar = ssx.bbar;
        let pi = ssx.pi;
        let cbar = ssx.cbar;

        let orig_type = glp_alloc(m + n + 1, std::mem::size_of::<c_int>() as c_int) as *mut c_int;
        let orig_lb = glp_alloc(m + n + 1, std::mem::size_of::<mpq_t>() as c_int) as *mut mpq_t;
        let orig_ub = glp_alloc(m + n + 1, std::mem::size_of::<mpq_t>() as c_int) as *mut mpq_t;
        let orig_coef = glp_alloc(m + n + 1, std::mem::size_of::<mpq_t>() as c_int) as *mut mpq_t;

        for k in 1..=m + n {
            *orig_type.offset(k as isize) = *type_.offset(k as isize);
            *orig_lb.offset(k as isize) = _glp_mpq_init();
            _glp_mpq_set(*orig_lb.offset(k as isize), *lb.offset(k as isize));
            *orig_ub.offset(k as isize) = _glp_mpq_init();
            _glp_mpq_set(*orig_ub.offset(k as isize), *ub.offset(k as isize));
        }

        let orig_dir = ssx.dir;

        for k in 0..=m + n {
            *orig_coef.offset(k as isize) = _glp_mpq_init();
            _glp_mpq_set(*orig_coef.offset(k as isize), *coef.offset(k as isize));
        }

        ssx.dir = 0;

        for k in 0..=m + n {
            _glp_mpq_set_si(*coef.offset(k as isize), 0, 1);
        }

        _glp_mpq_set_si(*bbar.offset(0), 0, 1);

        for i in 1..=m {
            let k = *q_col.offset(i as isize);
            let t = *type_.offset(k as isize);

            if t == 1 || t == 3 || t == 4 {
                if _glp_mpq_cmp(*bbar.offset(i as isize), *lb.offset(k as isize)) < 0 {
                    *type_.offset(k as isize) = 2;
                    _glp_mpq_set(*ub.offset(k as isize), *lb.offset(k as isize));
                    _glp_mpq_set_si(*lb.offset(k as isize), 0, 1);
                    _glp_mpq_set_si(*coef.offset(k as isize), -1, 1);
                    _glp_mpq_add(*bbar.offset(0), *bbar.offset(0), *ub.offset(k as isize));
                    _glp_mpq_sub(*bbar.offset(0), *bbar.offset(0), *bbar.offset(i as isize));
                }
            }

            if t == 2 || t == 3 || t == 4 {
                if _glp_mpq_cmp(*bbar.offset(i as isize), *ub.offset(k as isize)) > 0 {
                    *type_.offset(k as isize) = 1;
                    _glp_mpq_set(*lb.offset(k as isize), *ub.offset(k as isize));
                    _glp_mpq_set_si(*ub.offset(k as isize), 0, 1);
                    _glp_mpq_set_si(*coef.offset(k as isize), 1, 1);
                    _glp_mpq_add(*bbar.offset(0), *bbar.offset(0), *bbar.offset(i as isize));
                    _glp_mpq_sub(*bbar.offset(0), *bbar.offset(0), *lb.offset(k as isize));
                }
            }
        }

        _glp_ssx_eval_pi(ssx);
        _glp_ssx_eval_cbar(ssx);

        if ssx.msg_lev >= 2 {
            show_progress(ssx, 1);
        }

        let mut ret = 0;

        loop {
            if ssx.msg_lev >= 2 {
                if glp_difftime(glp_time(), ssx.tm_lag) >= ssx.out_frq - 0.001 {
                    show_progress(ssx, 1);
                }
            }

            if _glp_mpq_sgn(*bbar.offset(0)) == 0 {
                ret = 0;
                break;
            } else if ssx.it_lim == 0 {
                ret = 2;
                break;
            } else if ssx.tm_lim >= 0.0 && ssx.tm_lim <= glp_difftime(glp_time(), ssx.tm_beg) {
                ret = 3;
                break;
            } else {
                _glp_ssx_chuzc(ssx);
                if ssx.q == 0 {
                    ret = 1;
                    break;
                } else {
                    _glp_ssx_eval_col(ssx);
                    _glp_ssx_chuzr(ssx);
                    assert!(ssx.p != 0, "ssx->p != 0");
                    _glp_ssx_update_bbar(ssx);

                    if ssx.p > 0 {
                        _glp_ssx_eval_rho(ssx);
                        _glp_ssx_eval_row(ssx);
                        assert!(
                            _glp_mpq_cmp(*ssx.aq.offset(ssx.p as isize), *ssx.ap.offset(ssx.q as isize)) == 0,
                            "mpq_cmp(ssx->aq[ssx->p], ssx->ap[ssx->q]) == 0"
                        );
                        _glp_ssx_update_pi(ssx);
                        _glp_ssx_update_cbar(ssx);
                    }

                    if ssx.p > 0 {
                        let k = *q_col.offset(ssx.p as isize);
                        if *type_.offset(k as isize) != *orig_type.offset(k as isize) {
                            *type_.offset(k as isize) = *orig_type.offset(k as isize);
                            _glp_mpq_set(*lb.offset(k as isize), *orig_lb.offset(k as isize));
                            _glp_mpq_set(*ub.offset(k as isize), *orig_ub.offset(k as isize));

                            assert!(
                                ssx.p_stat == 1 || ssx.p_stat == 2,
                                "ssx->p_stat == SSX_NL || ssx->p_stat == SSX_NU"
                            );

                            ssx.p_stat = if ssx.p_stat == 1 { 2 } else { 1 };

                            if *type_.offset(k as isize) == 4 {
                                ssx.p_stat = 4;
                            }

                            _glp_mpq_set_si(*coef.offset(k as isize), 0, 1);

                            if k <= m {
                                _glp_mpq_neg(*cbar.offset(ssx.q as isize), *pi.offset(k as isize));
                            } else {
                                let mut temp = _glp_mpq_init();
                                _glp_mpq_set_si(*cbar.offset(ssx.q as isize), 0, 1);
                                let mut ptr = *a_ptr.offset((k - m) as isize);
                                while ptr < *a_ptr.offset((k - m + 1) as isize) {
                                    _glp_mpq_mul(
                                        temp,
                                        *pi.offset(*a_ind.offset(ptr as isize) as isize),
                                        *a_val.offset(ptr as isize),
                                    );
                                    _glp_mpq_add(
                                        *cbar.offset(ssx.q as isize),
                                        *cbar.offset(ssx.q as isize),
                                        temp,
                                    );
                                    ptr += 1;
                                }
                                _glp_mpq_clear(temp);
                            }
                        }
                    }

                    _glp_ssx_change_basis(ssx);

                    if ssx.it_lim > 0 {
                        ssx.it_lim -= 1;
                    }
                    ssx.it_cnt += 1;
                }
            }
        }

        if ssx.msg_lev >= 2 {
            show_progress(ssx, 1);
        }

        for k in 1..=m + n {
            *type_.offset(k as isize) = *orig_type.offset(k as isize);
            _glp_mpq_set(*lb.offset(k as isize), *orig_lb.offset(k as isize));
            _glp_mpq_clear(*orig_lb.offset(k as isize));
            _glp_mpq_set(*ub.offset(k as isize), *orig_ub.offset(k as isize));
            _glp_mpq_clear(*orig_ub.offset(k as isize));
        }

        ssx.dir = orig_dir;

        for k in 0..=m + n {
            _glp_mpq_set(*coef.offset(k as isize), *orig_coef.offset(k as isize));
            _glp_mpq_clear(*orig_coef.offset(k as isize));
        }

        glp_free(orig_type as *mut c_void);
        glp_free(orig_lb as *mut c_void);
        glp_free(orig_ub as *mut c_void);
        glp_free(orig_coef as *mut c_void);

        ret
    }
}

#[no_mangle]
pub extern "C" fn _glp_ssx_phase_ii(ssx: *mut SSX) -> c_int {
    unsafe {
        let ssx = &mut *ssx;
        let mut ret = 0;

        if ssx.msg_lev >= 2 {
            show_progress(ssx, 2);
        }

        loop {
            if ssx.msg_lev >= 2 {
                if glp_difftime(glp_time(), ssx.tm_lag) >= ssx.out_frq - 0.001 {
                    show_progress(ssx, 2);
                }
            }

            if ssx.it_lim == 0 {
                ret = 2;
                break;
            } else if ssx.tm_lim >= 0.0 && ssx.tm_lim <= glp_difftime(glp_time(), ssx.tm_beg) {
                ret = 3;
                break;
            } else {
                _glp_ssx_chuzc(ssx);
                if ssx.q == 0 {
                    ret = 0;
                    break;
                } else {
                    _glp_ssx_eval_col(ssx);
                    _glp_ssx_chuzr(ssx);
                    if ssx.p == 0 {
                        ret = 1;
                        break;
                    } else {
                        _glp_ssx_update_bbar(ssx);
                        if ssx.p > 0 {
                            _glp_ssx_eval_rho(ssx);
                            _glp_ssx_eval_row(