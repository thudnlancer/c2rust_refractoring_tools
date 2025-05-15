use std::f64;
use std::ptr;

const NBIT_MAX: usize = 31;

struct NPP;
struct NPPROW;
struct NPPCOL;
struct NPPAIJ;
struct NPPLSE;
struct NPPLIT {
    col: *mut NPPCOL,
    neg: i32,
}
struct NPPSED {
    x: NPPLIT,
    y: NPPLIT,
    z: NPPLIT,
    s: *mut NPPCOL,
    c: *mut NPPCOL,
}

impl NPP {
    fn del_row(&mut self, row: *mut NPPROW) {
        unsafe {
            ptr::drop_in_place(row);
        }
    }

    fn del_col(&mut self, col: *mut NPPCOL) {
        unsafe {
            ptr::drop_in_place(col);
        }
    }

    fn add_row(&mut self) -> *mut NPPROW {
        Box::into_raw(Box::new(NPPROW {
            lb: -f64::INFINITY,
            ub: f64::INFINITY,
            ptr: ptr::null_mut(),
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }))
    }

    fn add_col(&mut self) -> *mut NPPCOL {
        Box::into_raw(Box::new(NPPCOL {
            is_int: 0,
            lb: 0.0,
            ub: 0.0,
            j: 0,
            ptr: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }

    fn add_aij(&mut self, row: *mut NPPROW, col: *mut NPPCOL, val: f64) {
        unsafe {
            let aij = Box::into_raw(Box::new(NPPAIJ {
                val,
                row,
                col,
                r_next: (*row).ptr,
                c_next: (*col).ptr,
            }));
            (*row).ptr = aij;
            (*col).ptr = aij;
        }
    }

    fn del_aij(&mut self, aij: *mut NPPAIJ) {
        unsafe {
            ptr::drop_in_place(aij);
        }
    }

    fn push_tse(&mut self, _rcv: fn(&mut NPP, *mut ()) -> *mut (), _size: usize) -> *mut () {
        ptr::null_mut()
    }

    fn row_nnz(&self, row: *const NPPROW) -> i32 {
        let mut count = 0;
        unsafe {
            let mut aij = (*row).ptr;
            while !aij.is_null() {
                count += 1;
                aij = (*aij).r_next;
            }
        }
        count
    }
}

fn npp_sat_free_row(npp: &mut NPP, p: *mut NPPROW) {
    unsafe {
        assert!((*p).lb == -f64::INFINITY && (*p).ub == f64::INFINITY);
        npp.del_row(p);
    }
}

struct SatFixedCol {
    q: i32,
    s: i32,
}

fn rcv_sat_fixed_col(_npp: &mut NPP, info_: *mut ()) -> i32 {
    0
}

fn npp_sat_fixed_col(npp: &mut NPP, q: *mut NPPCOL) -> i32 {
    unsafe {
        assert!((*q).lb == (*q).ub);
        
        let info = Box::new(SatFixedCol {
            q: (*q).j,
            s: (*q).lb as i32,
        });
        let info_ptr = Box::into_raw(info);
        
        assert!((*info_ptr).s as f64 == (*q).lb);
        
        if (*info_ptr).s == 0 {
            npp.del_col(q);
            return 0;
        }
        
        let mut aij = (*q).ptr;
        while !aij.is_null() {
            let i = (*aij).row;
            if (*(*aij).row).lb != -f64::INFINITY {
                (*(*aij).row).lb -= (*aij).val * (*info_ptr).s as f64;
                let temp = (*(*aij).row).lb as i32;
                if temp as f64 != (*(*aij).row).lb {
                    return 1;
                }
            }
            if (*(*aij).row).ub != f64::INFINITY {
                (*(*aij).row).ub -= (*aij).val * (*info_ptr).s as f64;
                let temp = (*(*aij).row).ub as i32;
                if temp as f64 != (*(*aij).row).ub {
                    return 2;
                }
            }
            aij = (*aij).c_next;
        }
        
        npp.del_col(q);
        0
    }
}

fn npp_sat_is_bin_comb(_npp: &NPP, row: *mut NPPROW) -> i32 {
    unsafe {
        let mut aij = (*row).ptr;
        while !aij.is_null() {
            if !((*aij).val == 1.0 || (*aij).val == -1.0) {
                return 0;
            }
            let col = (*aij).col;
            if !((*col).is_int != 0 && (*col).lb == 0.0 && (*col).ub == 1.0) {
                return 0;
            }
            aij = (*aij).r_next;
        }
        1
    }
}

fn npp_sat_num_pos_coef(_npp: &NPP, row: *mut NPPROW) -> i32 {
    unsafe {
        let mut num = 0;
        let mut aij = (*row).ptr;
        while !aij.is_null() {
            if (*aij).val > 0.0 {
                num += 1;
            }
            aij = (*aij).r_next;
        }
        num
    }
}

fn npp_sat_num_neg_coef(_npp: &NPP, row: *mut NPPROW) -> i32 {
    unsafe {
        let mut num = 0;
        let mut aij = (*row).ptr;
        while !aij.is_null() {
            if (*aij).val < 0.0 {
                num += 1;
            }
            aij = (*aij).r_next;
        }
        num
    }
}

fn npp_sat_is_cover_ineq(npp: &NPP, row: *mut NPPROW) -> i32 {
    unsafe {
        if (*row).lb != -f64::INFINITY && (*row).ub == f64::INFINITY {
            if npp_sat_is_bin_comb(npp, row) != 0 {
                if (*row).lb == 1.0 - npp_sat_num_neg_coef(npp, row) as f64 {
                    return 1;
                }
            }
        } else if (*row).lb == -f64::INFINITY && (*row).ub != f64::INFINITY {
            if npp_sat_is_bin_comb(npp, row) != 0 {
                if (*row).ub == npp_sat_num_pos_coef(npp, row) as f64 - 1.0 {
                    return 2;
                }
            }
        }
        0
    }
}

fn npp_sat_is_pack_ineq(npp: &NPP, row: *mut NPPROW) -> i32 {
    unsafe {
        if (*row).lb == -f64::INFINITY && (*row).ub != f64::INFINITY {
            if npp_sat_is_bin_comb(npp, row) != 0 {
                if (*row).ub == 1.0 - npp_sat_num_neg_coef(npp, row) as f64 {
                    return 1;
                }
            }
        } else if (*row).lb != -f64::INFINITY && (*row).ub == f64::INFINITY {
            if npp_sat_is_bin_comb(npp, row) != 0 {
                if (*row).lb == npp_sat_num_pos_coef(npp, row) as f64 - 1.0 {
                    return 2;
                }
            }
        }
        0
    }
}

fn npp_sat_is_partn_eq(npp: &NPP, row: *mut NPPROW) -> i32 {
    unsafe {
        if (*row).lb == (*row).ub {
            if npp_sat_is_bin_comb(npp, row) != 0 {
                if (*row).lb == 1.0 - npp_sat_num_neg_coef(npp, row) as f64 {
                    return 1;
                }
                if (*row).ub == npp_sat_num_pos_coef(npp, row) as f64 - 1.0 {
                    return 2;
                }
            }
        }
        0
    }
}

fn npp_sat_reverse_row(npp: &mut NPP, row: *mut NPPROW) -> i32 {
    unsafe {
        let mut ret = 0;
        let mut aij = (*row).ptr;
        while !aij.is_null() {
            (*aij).val = -(*aij).val;
            let temp = (*aij).val as i32;
            if temp as f64 != (*aij).val {
                ret = 1;
            }
            aij = (*aij).r_next;
        }
        
        let old_lb = (*row).lb;
        let old_ub = (*row).ub;
        
        if old_ub == f64::INFINITY {
            (*row).lb = -f64::INFINITY;
        } else {
            (*row).lb = -old_ub;
            let temp = (*row).lb as i32;
            if temp as f64 != (*row).lb {
                ret = 2;
            }
        }
        
        if old_lb == -f64::INFINITY {
            (*row).ub = f64::INFINITY;
        } else {
            (*row).ub = -old_lb;
            let temp = (*row).ub as i32;
            if temp as f64 != (*row).ub {
                ret = 3;
            }
        }
        
        ret
    }
}

fn npp_sat_split_pack(npp: &mut NPP, row: *mut NPPROW, nlit: i32) -> *mut NPPROW {
    unsafe {
        assert!(npp_sat_is_pack_ineq(npp, row) == 1);
        assert!(0 < nlit && nlit < npp.row_nnz(row));
        
        let rrr = npp.add_row();
        (*rrr).lb = -f64::INFINITY;
        (*rrr).ub = 1.0;
        
        for _ in 1..=nlit {
            let aij = (*row).ptr;
            assert!(!aij.is_null());
            
            npp.add_aij(npp, rrr, (*aij).col, (*aij).val);
            
            if (*aij).val < 0.0 {
                (*rrr).ub -= 1.0;
                (*row).ub += 1.0;
            }
            
            npp.del_aij(aij);
        }
        
        let col = npp.add_col();
        (*col).is_int = 1;
        (*col).lb = 0.0;
        (*col).ub = 1.0;
        
        npp.add_aij(npp, rrr, col, -1.0);
        (*rrr).ub -= 1.0;
        
        npp.add_aij(npp, row, col, 1.0);
        
        rrr
    }
}

fn npp_sat_encode_pack(npp: &mut NPP, row: *mut NPPROW) {
    unsafe {
        assert!(npp_sat_is_pack_ineq(npp, row) == 1);
        
        let mut aij = (*row).ptr;
        while !aij.is_null() {
            let mut aik = (*aij).r_next;
            while !aik.is_null() {
                let rrr = npp.add_row();
                (*rrr).lb = -f64::INFINITY;
                (*rrr).ub = 1.0;
                
                npp.add_aij(npp, rrr, (*aij).col, (*aij).val);
                if (*aij).val < 0.0 {
                    (*rrr).ub -= 1.0;
                }
                
                npp.add_aij(npp, rrr, (*aik).col, (*aik).val);
                if (*aik).val < 0.0 {
                    (*rrr).ub -= 1.0;
                }
                
                npp_sat_reverse_row(npp, rrr);
                assert!(npp_sat_is_cover_ineq(npp, rrr) == 1);
                
                aik = (*aik).r_next;
            }
            aij = (*aij).r_next;
        }
        
        npp.del_row(row);
    }
}

fn npp_sat_encode_sum2(npp: &mut NPP, set: *mut NPPLSE, sed: &mut NPPSED) {
    unsafe {
        assert!(!set.is_null());
        assert!(!(*set).next.is_null());
        assert!((*(*set).next).next.is_null());
        
        sed.x = (*set).lit;
        assert!(sed.x.neg == 0 || sed.x.neg == 1);
        sed.y = (*(*set).next).lit;
        assert!(sed.y.neg == 0 || sed.y.neg == 1);
        sed.z.col = ptr::null_mut();
        sed.z.neg = 0;
        
        sed.s = npp.add_col();
        (*(*sed).s).is_int = 1;
        (*(*sed).s).lb = 0.0;
        (*(*sed).s).ub = 1.0;
        
        for x in 0..=1 {
            for y in 0..=1 {
                for s in 0..=1 {
                    if (x + y) % 2 != s {
                        let row = npp.add_row();
                        (*row).lb = 1.0;
                        (*row).ub = f64::INFINITY;
                        
                        if x == sed.x.neg {
                            npp.add_aij(npp, row, sed.x.col, 1.0);
                        } else {
                            npp.add_aij(npp, row, sed.x.col, -1.0);
                            (*row).lb -= 1.0;
                        }
                        
                        if y == sed.y.neg {
                            npp.add_aij(npp, row, sed.y.col, 1.0);
                        } else {
                            npp.add_aij(npp, row, sed.y.col, -1.0);
                            (*row).lb -= 1.0;
                        }
                        
                        if s == 0 {
                            npp.add_aij(npp, row, sed.s, 1.0);
                        } else {
                            npp.add_aij(npp, row, sed.s, -1.0);
                            (*row).lb -= 1.0;
                        }
                    }
                }
            }
        }
        
        sed.c = npp.add_col();
        (*(*sed).c).is_int = 1;
        (*(*sed).c).lb = 0.0;
        (*(*sed).c).ub = 1.0;
        
        for x in 0..=1 {
            for y in 0..=1 {
                for c in 0..=1 {
                    if (x + y) / 2 != c {
                        let row = npp.add_row();
                        (*row).lb = 1.0;
                        (*row).ub = f64::INFINITY;
                        
                        if x == sed.x.neg {
                            npp.add_aij(npp, row, sed.x.col, 1.0);
                        } else {
                            npp.add_aij(npp, row, sed.x.col, -1.0);
                            (*row).lb -= 1.0;
                        }
                        
                        if y == sed.y.neg {
                            npp.add_aij(npp, row, sed.y.col, 1.0);
                        } else {
                            npp.add_aij(npp, row, sed.y.col, -1.0);
                            (*row).lb -= 1.0;
                        }
                        
                        if c == 0 {
                            npp.add_aij(npp, row, sed.c, 1.0);
                        } else {
                            npp.add_aij(npp, row, sed.c, -1.0);
                            (*row).lb -= 1.0;
                        }
                    }
                }
            }
        }
    }
}

fn npp_sat_encode_sum3(npp: &mut NPP, set: *mut NPPLSE, sed: &mut NPPSED) {
    unsafe {
        assert!(!set.is_null());
        assert!(!(*set).next.is_null());
        assert!(!(*(*set).next).next.is_null());
        
        sed.x = (*set).lit;
        assert!(sed.x.neg == 0 || sed.x.neg == 1);
        sed.y = (*(*set).next).lit;
        assert!(sed.y.neg == 0 || sed.y.neg == 1);
        sed.z = (*(*(*set).next).next).lit;
        assert!(sed.z.neg == 0 || sed.z.neg == 1);
        
        sed.s = npp.add_col();
        (*(*sed).s).is_int = 1;
        (*(*sed).s).lb = 0.0;
        (*(*sed).s).ub = 1.0;
        
        for x in 0..=1 {
            for y in 0..=1 {
                for z in 0..=1 {
                    for s in 0..=1 {
                        if (x + y + z) % 2 != s {
                            let row = npp.add_row();
                            (*row).lb = 1.0;
                            (*row