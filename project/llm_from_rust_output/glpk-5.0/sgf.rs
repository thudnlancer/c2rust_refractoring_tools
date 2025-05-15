use std::ptr;
use std::mem;
use std::cmp;
use std::f64;

struct SVA {
    n_max: i32,
    n: i32,
    ptr: *mut i32,
    len: *mut i32,
    cap: *mut i32,
    size: i32,
    m_ptr: i32,
    r_ptr: i32,
    head: i32,
    tail: i32,
    prev: *mut i32,
    next: *mut i32,
    ind: *mut i32,
    val: *mut f64,
    talky: i32,
}

struct LUF {
    n: i32,
    sva: *mut SVA,
    fr_ref: i32,
    fc_ref: i32,
    vr_ref: i32,
    vr_piv: *mut f64,
    vc_ref: i32,
    pp_ind: *mut i32,
    pp_inv: *mut i32,
    qq_ind: *mut i32,
    qq_inv: *mut i32,
}

struct SGF {
    luf: *mut LUF,
    rs_head: *mut i32,
    rs_prev: *mut i32,
    rs_next: *mut i32,
    cs_head: *mut i32,
    cs_prev: *mut i32,
    cs_next: *mut i32,
    vr_max: *mut f64,
    flag: *mut i8,
    work: *mut f64,
    updat: i32,
    piv_tol: f64,
    piv_lim: i32,
    suhl: i32,
    eps_tol: f64,
}

fn sgf_reduce_nuc(
    luf: &mut LUF,
    k1_: &mut i32,
    k2_: &mut i32,
    cnt: *mut i32,
    list: *mut i32,
) -> i32 {
    unsafe {
        let n = luf.n;
        let sva = luf.sva;
        let sv_ind = (*sva).ind;
        let vr_ref = luf.vr_ref;
        let vr_ptr = (*sva).ptr.offset(vr_ref - 1);
        let vr_len = (*sva).len.offset(vr_ref - 1);
        let vc_ref = luf.vc_ref;
        let vc_ptr = (*sva).ptr.offset(vc_ref - 1);
        let vc_len = (*sva).len.offset(vc_ref - 1);
        let pp_ind = luf.pp_ind;
        let pp_inv = luf.pp_inv;
        let qq_ind = luf.qq_ind;
        let qq_inv = luf.qq_inv;

        let mut k1 = 1;
        let mut k2 = n;
        let mut ns = 0;

        for j in 1..=n {
            *cnt.offset(j as isize) = *vc_len.offset(j as isize);
            if *cnt.offset(j as isize) == 1 {
                ns += 1;
                *list.offset(ns as isize) = j;
            }
        }

        while ns > 0 {
            let j = *list.offset(ns as isize);
            ns -= 1;
            if *cnt.offset(j as isize) == 0 {
                return 1;
            }

            let mut ptr = *vc_ptr.offset(j as isize);
            let end = ptr + *vc_len.offset(j as isize);

            while ptr < end && *pp_ind.offset(*sv_ind.offset(ptr as isize) as isize) < k1 {
                ptr += 1;
            }

            assert!(ptr < end, "ptr < end");

            let i = *sv_ind.offset(ptr as isize);
            let ii = *pp_ind.offset(i as isize);

            let j1 = *pp_inv.offset(k1 as isize);
            let j2 = *pp_inv.offset(ii as isize);
            *pp_ind.offset(j1 as isize) = ii;
            *pp_inv.offset(ii as isize) = j1;
            *pp_ind.offset(j2 as isize) = k1;
            *pp_inv.offset(k1 as isize) = j2;

            let jj = *qq_inv.offset(j as isize);
            let i1 = *qq_ind.offset(k1 as isize);
            let i2 = *qq_ind.offset(jj as isize);
            *qq_ind.offset(k1 as isize) = i2;
            *qq_inv.offset(i2 as isize) = k1;
            *qq_ind.offset(jj as isize) = i1;
            *qq_inv.offset(i1 as isize) = jj;

            k1 += 1;

            ptr = *vr_ptr.offset(i as isize);
            let end = ptr + *vr_len.offset(i as isize);
            while ptr < end {
                let j = *sv_ind.offset(ptr as isize);
                *cnt.offset(j as isize) -= 1;
                if *cnt.offset(j as isize) == 1 {
                    ns += 1;
                    *list.offset(ns as isize) = j;
                }
                ptr += 1;
            }
        }

        if k1 <= n {
            ns = 0;
            for i in 1..=n {
                if *pp_ind.offset(i as isize) < k1 {
                    *cnt.offset(i as isize) = 0;
                } else {
                    *cnt.offset(i as isize) = *vr_len.offset(i as isize);
                    if *cnt.offset(i as isize) == 1 {
                        ns += 1;
                        *list.offset(ns as isize) = i;
                    }
                }
            }

            while ns > 0 {
                let i = *list.offset(ns as isize);
                ns -= 1;
                if *cnt.offset(i as isize) == 0 {
                    return 2;
                }

                let mut ptr = *vr_ptr.offset(i as isize);
                let end = ptr + *vr_len.offset(i as isize);

                while ptr < end && *qq_inv.offset(*sv_ind.offset(ptr as isize) as isize) > k2 {
                    ptr += 1;
                }

                assert!(ptr < end, "ptr < end");

                let ii = *pp_ind.offset(i as isize);
                let j1 = *pp_inv.offset(k2 as isize);
                let j2 = *pp_inv.offset(ii as isize);
                *pp_ind.offset(j1 as isize) = ii;
                *pp_inv.offset(ii as isize) = j1;
                *pp_ind.offset(j2 as isize) = k2;
                *pp_inv.offset(k2 as isize) = j2;

                let jj = *qq_inv.offset(*sv_ind.offset(ptr as isize) as isize);
                let i1 = *qq_ind.offset(k2 as isize);
                let i2 = *qq_ind.offset(jj as isize);
                *qq_ind.offset(k2 as isize) = i2;
                *qq_inv.offset(i2 as isize) = k2;
                *qq_ind.offset(jj as isize) = i1;
                *qq_inv.offset(i1 as isize) = jj;

                k2 -= 1;

                ptr = *vc_ptr.offset(*sv_ind.offset(ptr as isize) as isize);
                let end = ptr + *vc_len.offset(*sv_ind.offset(ptr as isize) as isize);
                while ptr < end {
                    let i = *sv_ind.offset(ptr as isize);
                    *cnt.offset(i as isize) -= 1;
                    if *cnt.offset(i as isize) == 1 {
                        ns += 1;
                        *list.offset(ns as isize) = i;
                    }
                    ptr += 1;
                }
            }

            assert!(k1 < k2, "k1 < k2");
        }

        *k1_ = k1;
        *k2_ = k2;
        0
    }
}

fn sgf_singl_phase(
    luf: &mut LUF,
    k1: i32,
    k2: i32,
    updat: i32,
    ind: *mut i32,
    val: *mut f64,
) -> i32 {
    unsafe {
        let n = luf.n;
        let sva = luf.sva;
        let sv_ind = (*sva).ind;
        let sv_val = (*sva).val;
        let fc_ref = luf.fc_ref;
        let fc_ptr = (*sva).ptr.offset(fc_ref - 1);
        let fc_len = (*sva).len.offset(fc_ref - 1);
        let vr_ref = luf.vr_ref;
        let vr_ptr = (*sva).ptr.offset(vr_ref - 1);
        let vr_len = (*sva).len.offset(vr_ref - 1);
        let vr_piv = luf.vr_piv;
        let vc_ref = luf.vc_ref;
        let vc_ptr = (*sva).ptr.offset(vc_ref - 1);
        let vc_len = (*sva).len.offset(vc_ref - 1);
        let pp_ind = luf.pp_ind;
        let pp_inv = luf.pp_inv;
        let qq_ind = luf.qq_ind;
        let qq_inv = luf.qq_inv;

        assert!(
            (1 <= k1 && k1 < k2 && k2 <= n) || (k1 == n + 1 && k2 == n),
            "Invalid k1/k2 range"
        );

        for k in k1..=k2 {
            *qq_inv.offset(*qq_ind.offset(k as isize) as isize) = k - k2 + n;
            *pp_ind.offset(*pp_inv.offset(k as isize) as isize) = *qq_inv.offset(*qq_ind.offset(k as isize) as isize);
        }

        for k in (k2 + 1)..=n {
            *qq_inv.offset(*qq_ind.offset(k as isize) as isize) = n - k + k1;
            *pp_ind.offset(*pp_inv.offset(k as isize) as isize) = *qq_inv.offset(*qq_ind.offset(k as isize) as isize);
        }

        for k in 1..=n {
            *qq_ind.offset(*qq_inv.offset(k as isize) as isize) = k;
            *pp_inv.offset(*pp_ind.offset(k as isize) as isize) = *qq_ind.offset(*qq_inv.offset(k as isize) as isize);
        }

        let mut k2 = n - k2 + k1;

        for k in 1..k1 {
            let i = *pp_inv.offset(k as isize);
            let mut ptr = *vr_ptr.offset(i as isize);
            let end = ptr + *vr_len.offset(i as isize);

            while *qq_inv.offset(*sv_ind.offset(ptr as isize) as isize) != k {
                ptr += 1;
            }

            assert!(ptr < end, "ptr < end");

            *vr_piv.offset(i as isize) = *sv_val.offset(ptr as isize);
            *sv_ind.offset(ptr as isize) = *sv_ind.offset((end - 1) as isize);
            *sv_val.offset(ptr as isize) = *sv_val.offset((end - 1) as isize);
            *vr_len.offset(i as isize) -= 1;
            *vc_len.offset(*qq_ind.offset(k as isize) as isize) = 0;
        }

        for k in k1..k2 {
            *vr_len.offset(*pp_inv.offset(k as isize) as isize) = 0;
        }

        for k in k2..=n {
            let i = *pp_inv.offset(k as isize);
            let mut ptr1 = *vr_ptr.offset(i as isize);
            let mut ptr = ptr1;
            let end = ptr + *vr_len.offset(i as isize);

            while ptr < end {
                if *qq_inv.offset(*sv_ind.offset(ptr as isize) as isize) >= k2 {
                    *sv_ind.offset(ptr1 as isize) = *sv_ind.offset(ptr as isize);
                    *sv_val.offset(ptr1 as isize) = *sv_val.offset(ptr as isize);
                    ptr1 += 1;
                }
                ptr += 1;
            }

            *vr_len.offset(i as isize) = ptr1 - *vr_ptr.offset(i as isize);

            let j = *qq_ind.offset(k as isize);
            ptr1 = *vc_ptr.offset(j as isize);
            ptr = ptr1;
            let end = ptr + *vc_len.offset(j as isize);

            while ptr < end {
                if *pp_ind.offset(*sv_ind.offset(ptr as isize) as isize) >= k2 {
                    *sv_ind.offset(ptr1 as isize) = *sv_ind.offset(ptr as isize);
                    ptr1 += 1;
                }
                ptr += 1;
            }

            *vc_len.offset(j as isize) = ptr1 - *vc_ptr.offset(j as isize);
        }

        for k in k1..k2 {
            let j = *qq_ind.offset(k as isize);
            let mut len = 0;
            let mut piv = 0.0;
            let mut ptr = *vc_ptr.offset(j as isize);
            let end = ptr + *vc_len.offset(j as isize);

            while ptr < end {
                let i = *sv_ind.offset(ptr as isize);
                if *pp_ind.offset(i as isize) == k {
                    *vr_piv.offset(i as isize) = *sv_val.offset(ptr as isize);
                    piv = *vr_piv.offset(i as isize);
                } else if *pp_ind.offset(i as isize) > k {
                    len += 1;
                    *ind.offset(len as isize) = i;
                    *val.offset(len as isize) = *sv_val.offset(ptr as isize);
                }
                ptr += 1;
            }

            *vc_len.offset(j as isize) = 0;
            let j = *pp_inv.offset(k as isize);
            assert!(piv != 0.0, "piv != 0.0");

            if len > 0 {
                if (*sva).r_ptr - (*sva).m_ptr < len {
                    _glp_sva_more_space(sva, len);
                    sv_ind = (*sva).ind;
                    sv_val = (*sva).val;
                }

                _glp_sva_reserve_cap(sva, fc_ref - 1 + j, len);
                let mut ptr = *fc_ptr.offset(j as isize);
                for ptr1 in 1..=len {
                    *sv_ind.offset(ptr as isize) = *ind.offset(ptr1 as isize);
                    *sv_val.offset(ptr as isize) = *val.offset(ptr1 as isize) / piv;
                    ptr += 1;
                }

                *fc_len.offset(j as isize) = len;
            }
        }

        if updat == 0 {
            for k in 1..k2 {
                let i = *pp_inv.offset(k as isize);
                let len = *vr_len.offset(i as isize);
                if (*sva).r_ptr - (*sva).m_ptr < len {
                    _glp_sva_more_space(sva, len);
                    sv_ind = (*sva).ind;
                    sv_val = (*sva).val;
                }
                _glp_sva_make_static(sva, vr_ref - 1 + i);
            }
        }

        k2
    }
}

fn sgf_choose_pivot(sgf: &mut SGF, p_: &mut i32, q_: &mut i32) -> i32 {
    unsafe {
        let luf = (*sgf).luf;
        let n = (*luf).n;
        let sva = (*luf).sva;
        let sv_ind = (*sva).ind;
        let sv_val = (*sva).val;
        let vr_ref = (*luf).vr_ref;
        let vr_ptr = (*sva).ptr.offset(vr_ref - 1);
        let vr_len = (*sva).len.offset(vr_ref - 1);
        let vc_ref = (*luf).vc_ref;
        let vc_ptr = (*sva).ptr.offset(vc_ref - 1);
        let vc_len = (*sva).len.offset(vc_ref - 1);
        let rs_head = (*sgf).rs_head;
        let rs_next = (*sgf).rs_next;
        let cs_head = (*sgf).cs_head;
        let cs_prev = (*sgf).cs_prev;
        let cs_next = (*sgf).cs_next;
        let vr_max = (*sgf).vr_max;
        let piv_tol = (*sgf).piv_tol;
        let piv_lim = (*sgf).piv_lim;
        let suhl = (*sgf).suhl;

        let mut p = 0;
        let mut q = 0;
        let mut best = f64::MAX;
        let mut ncand = 0;

        let mut j = *cs_head.offset(1);
        if j != 0 {
            assert!(*vc_len.offset(j as isize) == 1, "vc_len[j] == 1");
            p = *sv_ind.offset(*vc_ptr.offset(j as isize) as isize);
            q = j;
        } else {
            let mut i = *rs