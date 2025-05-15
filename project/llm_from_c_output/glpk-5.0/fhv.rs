/* fhv.rs (sparse updatable FHV-factorization) */

use std::mem;
use std::ptr;
use std::f64;
use std::cmp;

use crate::luf::LUF;
use crate::sva::SVA;

#[derive(Debug)]
pub struct FHV {
    pub luf: *mut LUF,
    pub nfs_max: i32,
    pub nfs: i32,
    pub hh_ind: *mut i32,
    pub hh_ref: i32,
    pub p0_ind: *mut i32,
    pub p0_inv: *mut i32,
}

impl FHV {
    pub fn ft_update(
        &mut self,
        q: i32,
        aq_len: i32,
        aq_ind: *const i32,
        aq_val: *const f64,
        ind: *mut i32,
        val: *mut f64,
        work: *mut f64,
    ) -> i32 {
        unsafe {
            let luf = &mut *self.luf;
            let n = luf.n;
            let sva = &mut *luf.sva;
            let sv_ind = sva.ind;
            let sv_val = sva.val;
            let vr_ref = luf.vr_ref;
            let vr_ptr = &sva.ptr[vr_ref as usize - 1..];
            let vr_len = &sva.len[vr_ref as usize - 1..];
            let vr_cap = &sva.cap[vr_ref as usize - 1..];
            let vr_piv = luf.vr_piv;
            let vc_ref = luf.vc_ref;
            let vc_ptr = &sva.ptr[vc_ref as usize - 1..];
            let vc_len = &sva.len[vc_ref as usize - 1..];
            let vc_cap = &sva.cap[vc_ref as usize - 1..];
            let pp_ind = luf.pp_ind;
            let pp_inv = luf.pp_inv;
            let qq_ind = luf.qq_ind;
            let qq_inv = luf.qq_inv;
            let hh_ind = self.hh_ind;
            let hh_ref = self.hh_ref;
            let hh_ptr = &sva.ptr[hh_ref as usize - 1..];
            let hh_len = &sva.len[hh_ref as usize - 1..];

            const EPS_TOL: f64 = f64::EPSILON;
            const VPQ_TOL: f64 = 1e-5;
            const ERR_TOL: f64 = 1e-10;

            let mut end: i32;
            let mut i: i32;
            let mut i_end: i32;
            let mut i_ptr: i32;
            let mut j: i32;
            let mut j_end: i32;
            let mut j_ptr: i32;
            let mut k: i32;
            let mut len: i32;
            let mut nnz: i32;
            let mut p: i32;
            let mut p_end: i32;
            let mut p_ptr: i32;
            let mut ptr: i32;
            let mut q_end: i32;
            let mut q_ptr: i32;
            let mut s: i32;
            let mut t: i32;
            let mut f: f64;
            let mut vpq: f64;
            let mut temp: f64;

            // Replace current q-th column of matrix V by new one
            assert!(1 <= q && q <= n);
            
            // Convert new q-th column of matrix A to dense format
            for i in 1..=n {
                *work.offset(i as isize) = 0.0;
            }
            assert!(0 <= aq_len && aq_len <= n);
            for k in 1..=aq_len {
                i = *aq_ind.offset(k as isize);
                assert!(1 <= i && i <= n);
                assert!(*work.offset(i as isize) == 0.0);
                assert!(*aq_val.offset(k as isize) != 0.0);
                *work.offset(i as isize) = *aq_val.offset(k as isize);
            }

            // Compute new q-th column of matrix V: new V[q] = inv(F * H) * (new A[q])
            luf.pp_ind = self.p0_ind;
            luf.pp_inv = self.p0_inv;
            luf.f_solve(work);
            luf.pp_ind = pp_ind;
            luf.pp_inv = pp_inv;
            self.h_solve(work);

            // q-th column of V = s-th column of U
            s = *qq_inv.offset(q as isize);

            // Determine row number of element v[p,q] that corresponds to diagonal element u[s,s]
            p = *pp_inv.offset(s as isize);

            // Convert new q-th column of V to sparse format
            vpq = 0.0;
            len = 0;
            for i in 1..=n {
                temp = *work.offset(i as isize);
                if -EPS_TOL < temp && temp < EPS_TOL {
                    continue;
                } else if i == p {
                    vpq = temp;
                } else {
                    *ind.offset((len + 1) as isize) = i;
                    *val.offset((len + 1) as isize) = temp;
                    len += 1;
                }
            }

            // Clear q-th column of matrix V
            q_ptr = *vc_ptr.offset(q as isize);
            q_end = q_ptr + *vc_len.offset(q as isize);
            while q_ptr < q_end {
                // Get row index of v[i,q]
                i = *sv_ind.offset(q_ptr as isize);
                
                // Find and remove v[i,q] from i-th row
                i_ptr = *vr_ptr.offset(i as isize);
                i_end = i_ptr + *vr_len.offset(i as isize);
                while *sv_ind.offset(i_ptr as isize) != q {
                    i_ptr += 1;
                }
                assert!(i_ptr < i_end);
                *sv_ind.offset(i_ptr as isize) = *sv_ind.offset((i_end - 1) as isize);
                *sv_val.offset(i_ptr as isize) = *sv_val.offset((i_end - 1) as isize);
                *vr_len.offset(i as isize) -= 1;
                q_ptr += 1;
            }

            // Now q-th column of matrix V is empty
            *vc_len.offset(q as isize) = 0;

            // Put new q-th column of V (except element v[p,q] = u[s,s]) in column-wise format
            if len > 0 {
                if *vc_cap.offset(q as isize) < len {
                    if sva.r_ptr - sva.m_ptr < len {
                        sva.more_space(len);
                        sv_ind = sva.ind;
                        sv_val = sva.val;
                    }
                    sva.enlarge_cap(vc_ref + q - 1, len, 0);
                }
                ptr = *vc_ptr.offset(q as isize);
                ptr::copy_nonoverlapping(
                    ind.offset(1),
                    sv_ind.offset(ptr as isize),
                    len as usize,
                );
                ptr::copy_nonoverlapping(
                    val.offset(1),
                    sv_val.offset(ptr as isize),
                    len as usize,
                );
                *vc_len.offset(q as isize) = len;
            }

            // Put new q-th column of V (except element v[p,q] = u[s,s]) in row-wise format,
            // and determine largest row number t such that u[s,t] != 0
            t = if vpq == 0.0 { 0 } else { s };
            for k in 1..=len {
                // Get row index of v[i,q]
                i = *ind.offset(k as isize);
                
                // Put v[i,q] to i-th row
                if *vr_cap.offset(i as isize) == *vr_len.offset(i as isize) {
                    // Reserve extra locations in i-th row
                    let need = *vr_len.offset(i as isize) + 5;
                    if sva.r_ptr - sva.m_ptr < need {
                        sva.more_space(need);
                        sv_ind = sva.ind;
                        sv_val = sva.val;
                    }
                    sva.enlarge_cap(vr_ref + i - 1, need, 0);
                }
                ptr = *vr_ptr.offset(i as isize) + *vr_len.offset(i as isize);
                *sv_ind.offset(ptr as isize) = q;
                *sv_val.offset(ptr as isize) = *val.offset(k as isize);
                *vr_len.offset(i as isize) += 1;
                
                // v[i,q] is non-zero; increase t
                if t < *pp_ind.offset(i as isize) {
                    t = *pp_ind.offset(i as isize);
                }
            }

            // Check if matrix U is already upper triangular
            if s >= t {
                // No spike; matrix U is already upper triangular
                // Store its diagonal element u[s,s] = v[p,q]
                *vr_piv.offset(p as isize) = vpq;
                if s > t {
                    // Matrix U is structurally singular
                    assert!(vpq == 0.0);
                    return 1;
                } else if -VPQ_TOL < vpq && vpq < VPQ_TOL {
                    // Matrix U is not well conditioned
                    return 2;
                } else {
                    // Normal case
                    return 0;
                }
            }

            // Perform implicit symmetric permutations of rows and columns of matrix U
            assert!(p == *pp_inv.offset(s as isize) && q == *qq_ind.offset(s as isize));
            for k in s..t {
                *pp_inv.offset(k as isize) = *pp_inv.offset((k + 1) as isize);
                *pp_ind.offset(*pp_inv.offset(k as isize) as isize) = k;
                *qq_ind.offset(k as isize) = *qq_ind.offset((k + 1) as isize);
                *qq_inv.offset(*qq_ind.offset(k as isize) as isize) = k;
            }
            *pp_inv.offset(t as isize) = p;
            *pp_ind.offset(p as isize) = t;
            *qq_ind.offset(t as isize) = q;
            *qq_inv.offset(q as isize) = t;

            // Check if matrix U is already upper triangular
            p_ptr = *vr_ptr.offset(p as isize);
            p_end = p_ptr + *vr_len.offset(p as isize);
            while p_ptr < p_end {
                if *qq_inv.offset(*sv_ind.offset(p_ptr as isize) as isize) < t {
                    break; // Spike detected
                }
                p_ptr += 1;
            }
            if p_ptr == p_end {
                // No spike; matrix U is already upper triangular
                // Store its diagonal element u[t,t] = v[p,q]
                *vr_piv.offset(p as isize) = vpq;
                if -VPQ_TOL < vpq && vpq < VPQ_TOL {
                    // Matrix U is not well conditioned
                    return 2;
                } else {
                    // Normal case
                    return 0;
                }
            }

            // Copy p-th row of matrix V, which is t-th row of matrix U, to working array
            for j in 1..=n {
                *work.offset(j as isize) = 0.0;
            }
            *work.offset(q as isize) = vpq;
            p_ptr = *vr_ptr.offset(p as isize);
            p_end = p_ptr + *vr_len.offset(p as isize);
            while p_ptr < p_end {
                j = *sv_ind.offset(p_ptr as isize);
                *work.offset(j as isize) = *sv_val.offset(p_ptr as isize);
                
                // Find and remove v[p,j] from j-th column
                j_ptr = *vc_ptr.offset(j as isize);
                j_end = j_ptr + *vc_len.offset(j as isize);
                while *sv_ind.offset(j_ptr as isize) != p {
                    j_ptr += 1;
                }
                assert!(j_ptr < j_end);
                *sv_ind.offset(j_ptr as isize) = *sv_ind.offset((j_end - 1) as isize);
                *sv_val.offset(j_ptr as isize) = *sv_val.offset((j_end - 1) as isize);
                *vc_len.offset(j as isize) -= 1;
                p_ptr += 1;
            }
            *vr_len.offset(p as isize) = 0;

            // Perform gaussian elimination
            nnz = 0;
            for k in s..t {
                i = *pp_inv.offset(k as isize);
                j = *qq_ind.offset(k as isize);
                temp = *work.offset(j as isize);
                if -EPS_TOL < temp && temp < EPS_TOL {
                    continue;
                }
                *ind.offset((nnz + 1) as isize) = i;
                f = temp / *vr_piv.offset(i as isize);
                *val.offset((nnz + 1) as isize) = f;
                nnz += 1;
                
                // Gaussian transformation
                i_ptr = *vr_ptr.offset(i as isize);
                i_end = i_ptr + *vr_len.offset(i as isize);
                while i_ptr < i_end {
                    j = *sv_ind.offset(i_ptr as isize);
                    *work.offset(j as isize) -= f * *sv_val.offset(i_ptr as isize);
                    i_ptr += 1;
                }
            }

            if -VPQ_TOL < *work.offset(q as isize) && *work.offset(q as isize) < VPQ_TOL {
                return 3;
            }

            // Create new row-like factor H[k] and add to eta file H
            if nnz > 0 {
                if self.nfs == self.nfs_max {
                    return 4;
                }
                self.nfs += 1;
                k = self.nfs;
                *hh_ind.offset(k as isize) = p;
                
                if sva.r_ptr - sva.m_ptr < nnz {
                    sva.more_space(nnz);
                    sv_ind = sva.ind;
                    sv_val = sva.val;
                }
                sva.reserve_cap(hh_ref + k - 1, nnz);
                ptr = *hh_ptr.offset(k as isize);
                ptr::copy_nonoverlapping(
                    ind.offset(1),
                    sv_ind.offset(ptr as isize),
                    nnz as usize,
                );
                ptr::copy_nonoverlapping(
                    val.offset(1),
                    sv_val.offset(ptr as isize),
                    nnz as usize,
                );
                *hh_len.offset(k as isize) = nnz;
            }

            // Copy transformed p-th row of matrix V back to matrix V
            len = 0;
            for k in (t + 1)..=n {
                j = *qq_ind.offset(k as isize);
                temp = *work.offset(j as isize);
                if -EPS_TOL < temp && temp < EPS_TOL {
                    continue;
                }
                
                if *vc_cap.offset(j as isize) == *vc_len.offset(j as isize) {
                    let need = *vc_len.offset(j as isize) + 5;
                    if sva.r_ptr - sva.m_ptr < need {
                        sva.more_space(need);
                        sv_ind = sva.ind;
                        sv_val = sva.val;
                    }
                    sva.enlarge_cap(vc_ref + j - 1, need, 0);
                }
                ptr = *vc_ptr.offset(j as isize) + *vc_len.offset(j as isize);
                *sv_ind.offset(ptr as isize) = p;
                *sv_val.offset(ptr as isize) = temp;
                *vc_len.offset(j as isize) += 1;
                
                *ind.offset((len + 1) as isize) = j;
                *val.offset((len + 1) as isize) = temp;
                len += 1;
            }

            if *vr_cap.offset(p as isize) < len {
                if sva.r_ptr - sva.m_ptr < len {
                    sva.more_space(len);
                    sv_ind = sva.ind;
                    sv_val = sva.val;
                }
                sva.enlarge_cap(vr_ref + p - 1, len, 0);
            }
            ptr = *vr_ptr.offset(p as isize);
            ptr::copy_nonoverlapping(
                ind.offset(1),
                sv_ind.offset(ptr as isize),
                len as usize,
            );
            ptr::copy_nonoverlapping(
                val.offset(1),
                sv_val.offset(ptr as isize),
                len as usize,
            );
            *vr_len.offset(p as isize) = len;
            *vr_piv.offset(p as isize) = *work.offset(q as isize);

            // Perform accuracy test (only if new H[k] was added)
            if nnz > 0 {
                for j in 1..=n {
                    *work.offset(j as isize) = 0.0;
                }
                k = self.nfs;
                ptr = *hh_ptr.offset(k as isize);
                end = ptr + *hh_len.offset(k as isize);
                while ptr < end {
                    j = *sv_ind.offset(ptr as isize);
                    *work.offset(j as isize) = *sv_val.offset(ptr as isize);
                    ptr += 1;
                }

                temp = *vr_piv.offset(p as isize);
                ptr = *vc_ptr.offset(q as isize);
                end = ptr + *vc_len.offset(q as isize);
