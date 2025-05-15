/* npp5.rs */

use std::f64::{MAX, MIN};
use std::ptr;

#[derive(Debug)]
struct NPP {
    r_head: *mut NPPROW,
    r_tail: *mut NPPROW,
    c_head: *mut NPPCOL,
    sol: i32,
    // Other fields as needed
}

#[derive(Debug)]
struct NPPROW {
    lb: f64,
    ub: f64,
    ptr: *mut NPPAIJ,
    next: *mut NPPROW,
    prev: *mut NPPROW,
    temp: i32,
    // Other fields as needed
}

#[derive(Debug)]
struct NPPCOL {
    lb: f64,
    ub: f64,
    is_int: bool,
    ptr: *mut NPPAIJ,
    next: *mut NPPCOL,
    temp: i32,
    ll: LL,
    uu: UU,
    // Other fields as needed
}

#[derive(Debug)]
struct NPPAIJ {
    row: *mut NPPROW,
    col: *mut NPPCOL,
    r_next: *mut NPPAIJ,
    c_next: *mut NPPAIJ,
    // Other fields as needed
}

#[derive(Debug)]
struct LL {
    ll: f64,
}

#[derive(Debug)]
struct UU {
    uu: f64,
}

const GLP_DEBUG: bool = false;
const GLP_ENOPFS: i32 = 1;
const GLP_ENODFS: i32 = 2;
const GLP_SOL: i32 = 1;
const GLP_MIP: i32 = 2;

fn xassert(cond: bool) {
    if !cond {
        panic!("Assertion failed");
    }
}

fn xprintf(s: &str) {
    if GLP_DEBUG {
        print!("{}", s);
    }
}

fn npp_free_row(npp: *mut NPP, row: *mut NPPROW) {
    // Implementation of npp_free_row
    unsafe {
        // Remove row from linked list
        if !(*row).prev.is_null() {
            (*(*row).prev).next = (*row).next;
        } else {
            (*npp).r_head = (*row).next;
        }
        if !(*row).next.is_null() {
            (*(*row).next).prev = (*row).prev;
        } else {
            (*npp).r_tail = (*row).prev;
        }
        // Free memory and other cleanup
    }
}

fn npp_make_equality(npp: *mut NPP, row: *mut NPPROW) -> i32 {
    // Implementation of npp_make_equality
    0
}

fn npp_fixed_col(npp: *mut NPP, col: *mut NPPCOL) {
    // Implementation of npp_fixed_col
    unsafe {
        // Remove column from linked list
        if !(*col).next.is_null() {
            (*(*col).next).prev = ptr::null_mut();
        }
        if !(*col).ptr.is_null() {
            // Clean up aij entries
        }
        // Free memory and other cleanup
    }
}

fn npp_make_fixed(npp: *mut NPP, col: *mut NPPCOL) -> i32 {
    // Implementation of npp_make_fixed
    0
}

fn npp_empty_row(npp: *mut NPP, row: *mut NPPROW) -> i32 {
    // Implementation of npp_empty_row
    0
}

fn npp_eq_singlet(npp: *mut NPP, row: *mut NPPROW) -> i32 {
    // Implementation of npp_eq_singlet
    0
}

fn npp_ineq_singlet(npp: *mut NPP, row: *mut NPPROW) -> i32 {
    // Implementation of npp_ineq_singlet
    0
}

fn npp_activate_row(npp: *mut NPP, row: *mut NPPROW) {
    // Implementation of npp_activate_row
}

fn npp_analyze_row(npp: *mut NPP, row: *mut NPPROW) -> i32 {
    // Implementation of npp_analyze_row
    0
}

fn npp_inactive_bound(npp: *mut NPP, row: *mut NPPROW, bound: i32) {
    // Implementation of npp_inactive_bound
}

fn npp_forcing_row(npp: *mut NPP, row: *mut NPPROW, bound: i32) -> i32 {
    // Implementation of npp_forcing_row
    0
}

fn npp_improve_bounds(npp: *mut NPP, row: *mut NPPROW, flag: i32) -> i32 {
    // Implementation of npp_improve_bounds
    0
}

fn npp_implied_bounds(npp: *mut NPP, row: *mut NPPROW) {
    // Implementation of npp_implied_bounds
}

fn npp_implied_lower(npp: *mut NPP, col: *mut NPPCOL, bound: f64) -> i32 {
    // Implementation of npp_implied_lower
    0
}

fn npp_implied_upper(npp: *mut NPP, col: *mut NPPCOL, bound: f64) -> i32 {
    // Implementation of npp_implied_upper
    0
}

fn npp_empty_col(npp: *mut NPP, col: *mut NPPCOL) -> i32 {
    // Implementation of npp_empty_col
    0
}

fn npp_implied_slack(npp: *mut NPP, col: *mut NPPCOL) {
    // Implementation of npp_implied_slack
}

fn npp_implied_free(npp: *mut NPP, col: *mut NPPCOL) -> i32 {
    // Implementation of npp_implied_free
    0
}

fn npp_activate_col(npp: *mut NPP, col: *mut NPPCOL) {
    // Implementation of npp_activate_col
}

fn npp_deactivate_row(npp: *mut NPP, row: *mut NPPROW) {
    // Implementation of npp_deactivate_row
}

fn npp_deactivate_col(npp: *mut NPP, col: *mut NPPCOL) {
    // Implementation of npp_deactivate_col
}

fn npp_binarize_prob(npp: *mut NPP) {
    // Implementation of npp_binarize_prob
}

fn npp_hidden_packing(npp: *mut NPP, row: *mut NPPROW) -> i32 {
    // Implementation of npp_hidden_packing
    0
}

fn npp_hidden_covering(npp: *mut NPP, row: *mut NPPROW) -> i32 {
    // Implementation of npp_hidden_covering
    0
}

fn npp_reduce_ineq_coef(npp: *mut NPP, row: *mut NPPROW) -> i32 {
    // Implementation of npp_reduce_ineq_coef
    0
}

fn npp_clean_prob(npp: *mut NPP) {
    unsafe {
        xassert(!npp.is_null());
        
        let mut row = (*npp).r_head;
        while !row.is_null() {
            let next_row = (*row).next;
            if (*row).lb == -MAX && (*row).ub == MAX {
                if GLP_DEBUG {
                    xprintf("1");
                }
                npp_free_row(npp, row);
            }
            row = next_row;
        }
        
        row = (*npp).r_head;
        while !row.is_null() {
            let next_row = (*row).next;
            if (*row).lb != -MAX && (*row).ub != MAX && (*row).lb < (*row).ub {
                let ret = npp_make_equality(npp, row);
                if ret == 1 && GLP_DEBUG {
                    xprintf("2");
                } else {
                    xassert(ret != ret);
                }
            }
            row = next_row;
        }
        
        let mut col = (*npp).c_head;
        while !col.is_null() {
            let next_col = (*col).next;
            if (*col).lb == (*col).ub {
                if GLP_DEBUG {
                    xprintf("3");
                }
                npp_fixed_col(npp, col);
            }
            col = next_col;
        }
        
        col = (*npp).c_head;
        while !col.is_null() {
            let next_col = (*col).next;
            if (*col).lb != -MAX && (*col).ub != MAX && (*col).lb < (*col).ub {
                let ret = npp_make_fixed(npp, col);
                if ret == 1 {
                    if GLP_DEBUG {
                        xprintf("4");
                    }
                    npp_fixed_col(npp, col);
                }
            }
            col = next_col;
        }
    }
}

fn npp_process_row(npp: *mut NPP, row: *mut NPPROW, hard: i32) -> i32 {
    unsafe {
        xassert(!((*row).lb == -MAX && (*row).ub == MAX));
        
        if (*row).ptr.is_null() {
            let ret = npp_empty_row(npp, row);
            match ret {
                0 => {
                    if GLP_DEBUG { xprintf("A"); }
                    return 0;
                }
                1 => return GLP_ENOPFS,
                _ => xassert(false),
            }
        }
        
        if (*(*row).ptr).r_next.is_null() {
            let col = (*(*row).ptr).col;
            if (*row).lb == (*row).ub {
                let ret = npp_eq_singlet(npp, row);
                match ret {
                    0 => {
                        if GLP_DEBUG { xprintf("B"); }
                        let mut aij = (*col).ptr;
                        while !aij.is_null() {
                            npp_activate_row(npp, (*aij).row);
                            aij = (*aij).c_next;
                        }
                        npp_fixed_col(npp, col);
                        return 0;
                    }
                    1 | 2 => return GLP_ENOPFS,
                    _ => xassert(false),
                }
            } else {
                let ret = npp_ineq_singlet(npp, row);
                match ret {
                    0..=3 => {
                        if GLP_DEBUG { xprintf("C"); }
                        npp_activate_col(npp, col);
                        if ret >= 2 {
                            let mut aij = (*col).ptr;
                            while !aij.is_null() {
                                npp_activate_row(npp, (*aij).row);
                                aij = (*aij).c_next;
                            }
                        }
                        if ret == 3 {
                            if GLP_DEBUG { xprintf("D"); }
                            npp_fixed_col(npp, col);
                        }
                        return 0;
                    }
                    4 => return GLP_ENOPFS,
                    _ => xassert(false),
                }
            }
        }
        
        let ret = npp_analyze_row(npp, row);
        xassert(0x00 <= ret && ret <= 0xFF);
        
        if ret == 0x33 {
            return GLP_ENOPFS;
        }
        
        match ret & 0x0F {
            0x00 => {
                if (*row).lb != -MAX {
                    if GLP_DEBUG { xprintf("F"); }
                    npp_inactive_bound(npp, row, 0);
                }
            }
            0x01 => {}
            0x02 => {
                if GLP_DEBUG { xprintf("G"); }
                if npp_forcing_row(npp, row, 0) == 0 {
                    let mut aij = (*row).ptr;
                    while !aij.is_null() {
                        if GLP_DEBUG { xprintf("H"); }
                        let col = (*aij).col;
                        let next_aij = (*aij).r_next;
                        let mut aaa = (*col).ptr;
                        while !aaa.is_null() {
                            npp_activate_row(npp, (*aaa).row);
                            aaa = (*aaa).c_next;
                        }
                        npp_fixed_col(npp, col);
                        aij = next_aij;
                    }
                    npp_free_row(npp, row);
                    return 0;
                }
            }
            _ => xassert(false),
        }
        
        match ret & 0xF0 {
            0x00 => {
                if (*row).ub != MAX {
                    if GLP_DEBUG { xprintf("I"); }
                    npp_inactive_bound(npp, row, 1);
                }
            }
            0x10 => {}
            0x20 => {
                if GLP_DEBUG { xprintf("J"); }
                if npp_forcing_row(npp, row, 1) == 0 {
                    // Same fixup code as above
                    let mut aij = (*row).ptr;
                    while !aij.is_null() {
                        if GLP_DEBUG { xprintf("H"); }
                        let col = (*aij).col;
                        let next_aij = (*aij).r_next;
                        let mut aaa = (*col).ptr;
                        while !aaa.is_null() {
                            npp_activate_row(npp, (*aaa).row);
                            aaa = (*aaa).c_next;
                        }
                        npp_fixed_col(npp, col);
                        aij = next_aij;
                    }
                    npp_free_row(npp, row);
                    return 0;
                }
            }
            _ => xassert(false),
        }
        
        if (*row).lb == -MAX && (*row).ub == MAX {
            if GLP_DEBUG { xprintf("K"); }
            let mut aij = (*row).ptr;
            while !aij.is_null() {
                npp_activate_col(npp, (*aij).col);
                aij = (*aij).r_next;
            }
            npp_free_row(npp, row);
            return 0;
        }
        
        if (*npp).sol == GLP_MIP && hard != 0 {
            if npp_improve_bounds(npp, row, 1) < 0 {
                return GLP_ENOPFS;
            }
        }
        
        0
    }
}

fn npp_process_col(npp: *mut NPP, col: *mut NPPCOL) -> i32 {
    unsafe {
        xassert((*col).lb < (*col).ub);
        
        if (*col).ptr.is_null() {
            let ret = npp_empty_col(npp, col);
            match ret {
                0 => {
                    if GLP_DEBUG { xprintf("N"); }
                    return 0;
                }
                1 => return GLP_ENODFS,
                _ => xassert(false),
            }
        }
        
        if (*(*col).ptr).c_next.is_null() {
            let row = (*(*col).ptr).row;
            if (*row).lb == (*row).ub {
                if !(*col).is_int {
                    if GLP_DEBUG { xprintf("O"); }
                    npp_implied_slack(npp, col);
                    if (*row).lb == -MAX && (*row).ub == MAX {
                        if GLP_DEBUG { xprintf("P"); }
                        let mut aij = (*row).ptr;
                        while !aij.is_null() {
                            npp_activate_col(npp, (*aij).col);
                            aij = (*aij).r_next;
                        }
                        npp_free_row(npp, row);
                    } else {
                        npp_activate_row(npp, row);
                    }
                    return 0;
                }
            } else {
                if !(*col).is_int {
                    let ret = npp_implied_free(npp, col);
                    match ret {
                        0 => {
                            if GLP_DEBUG { xprintf("Q"); }
                            // Same as slack case above
                            if GLP_DEBUG { xprintf("O"); }
                            npp_implied_slack(npp, col);
                            if (*row).lb == -MAX && (*row).ub == MAX {
                                if GLP_DEBUG { xprintf("P"); }
                                let mut aij = (*row).ptr;
                                while !aij.is_null() {
                                    npp_activate_col(npp, (*aij).col);
                                    aij = (*aij).r_next;
                                }
                                npp_free_row(npp, row);
                            } else {
                                npp_activate_row(npp, row);
                            }
                            return 0;
                        }
                        1 => {}
                        2 => return GLP_ENODFS,
                        _ => xassert(false),
                    }
                }
            }
        }
        
        0
    }
}

fn npp_process_prob(npp: *mut NPP, hard: i32) -> i32 {
    unsafe {
        npp_clean_prob(npp);
        
        let mut row = (*npp).r_head;
        while !row.is_null() {
            (*row).temp = 1;
            row = (*row).next;
        }
        
        let mut col = (*npp).c_head;
        while !col.is_null() {
            (*col).temp = 1;
            col = (*col).next;
        }
        
        let mut processing = 1;
        while processing != 0 {
            processing = 0;
            
            loop {
                row = (*npp).r_head;
                if row.is_null() || (*row).temp == 0 {
                    break;
                }
                npp_deactivate_row(npp, row);
                let ret = npp_process_row(npp, row, hard);
                if ret != 0 {
                    return ret;
                }
                processing = 1;
            }
            
            loop {
                col = (*npp).c_head;
                if col.is_null() || (*col).temp == 0 {
                    break;
                }
                npp_deactivate_col(npp, col);
                let ret = npp_process_col(npp, col);
                if ret != 0 {
                    return ret;
                }
                processing = 1;
            }
        }
        
        if (*npp).sol == GLP_MIP && hard == 0 {
            let mut row = (*npp).r_head;
            while !row.is_null() {
                if npp_improve_bounds(npp, row, 0)