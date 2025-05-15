/*
 * mpfr.rs - routines for arbitrary-precision number support in gawk.
 *
 * Copyright (C) 2012-2022, the Free Software Foundation, Inc.
 *
 * This file is part of GAWK, the GNU implementation of the
 * AWK Programming Language.
 *
 * GAWK is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * GAWK is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_ulong};
use std::ptr;
use std::mem;
use std::time::{SystemTime, UNIX_EPOCH};

use gmp::mpz::{Mpz, mpz_t};
use mpfr::{mpfr_t, mpfr_prec_t, mpfr_rnd_t, mpfr_exp_t};
use mpfr::rnd::RoundingMode;
use mpfr::ops::*;

use crate::awk::*;
use crate::eval::*;
use crate::interpret::*;
use crate::node::*;
use crate::numeric::*;
use crate::strnum::*;

static mut MPFR_ROUND_MODE: mpfr_rnd_t = mpfr_rnd_t::RNDN;
static mut MPFR_DEFAULT_PREC: mpfr_prec_t = 53;
static mut DO_IEEE_FMT: bool = false;
static mut MIN_EXP: mpfr_exp_t = mpfr_exp_t::MIN;
static mut MAX_EXP: mpfr_exp_t = mpfr_exp_t::MAX;

static mut MNR: Mpz = Mpz::new();
static mut MFNR: Mpz = Mpz::new();
static mut MPZVAL: Mpz = Mpz::new();

static mut _MPF_T1: mpfr_t = unsafe { mem::zeroed() };
static mut _MPF_T2: mpfr_t = unsafe { mem::zeroed() };

const PRECISION_MIN: mpfr_prec_t = 64;

pub fn init_mpfr(prec: mpfr_prec_t, rmode: &str) {
    unsafe {
        MPFR_DEFAULT_PREC = prec;
        MPFR_ROUND_MODE = get_rnd_mode(rmode.chars().next().unwrap_or('N'));
        mpfr::set_default_prec(prec);
        mpfr::set_default_rounding_mode(MPFR_ROUND_MODE);
        
        MAKE_NUMBER = Some(mpg_make_number);
        STR2NUMBER = Some(mpg_force_number);
        FORMAT_VAL = Some(mpg_format_val);
        CMP_NUMBERS = Some(mpg_cmp);
        
        MNR.set_ui(0);
        MFNR.set_ui(0);
        DO_IEEE_FMT = false;
        
        mpfr::init2(&mut _MPF_T1, PRECISION_MIN);
        mpfr::init2(&mut _MPF_T2, PRECISION_MIN);
        MPZVAL.set_ui(0);
        
        register_exec_hook(mpg_interpret, 0);
    }
}

pub fn cleanup_mpfr() {
    unsafe {
        mpfr::clear(&mut _MPF_T1);
        mpfr::clear(&mut _MPF_T2);
    }
}

pub fn mpg_node(flags: u32) -> *mut NODE {
    let r = make_number_node(flags);
    
    unsafe {
        if flags == MPFN {
            mpfr::init((*r).mpg_numbr);
        } else {
            (*r).mpg_i.set_ui(0);
        }
    }
    
    r
}

pub fn mpg_make_number(x: f64) -> *mut NODE {
    let ival = double_to_int(x);
    let r: *mut NODE;
    
    if ival != x {
        r = mpg_float();
        unsafe {
            let tval = mpfr::set_d((*r).mpg_numbr, x, MPFR_ROUND_MODE);
            IEEE_FMT((*r).mpg_numbr, tval);
        }
    } else {
        r = mpg_integer();
        unsafe {
            (*r).mpg_i.set_d(ival);
        }
    }
    
    r
}

pub fn mpg_strtoui(zi: &mut Mpz, str: &str, base: i32) -> Result<(), ()> {
    let mut s = str.trim_start_matches("0x").trim_start_matches("0X");
    
    if base == 16 && str.len() >= 2 && str.starts_with("0x") || str.starts_with("0X") {
        s = &str[2..];
    } else if base == 8 && str.starts_with('0') {
        s = &str[1..];
    }
    
    match zi.set_str(s, base) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

pub fn mpg_maybe_float(str: &str, use_locale: bool) -> bool {
    if str.len() >= 3 {
        let s = str.to_lowercase();
        if s.starts_with("inf") || s.starts_with("nan") {
            return true;
        }
    }
    
    let dec_point = if use_locale {
        // TODO: Handle locale decimal point
        '.'
    } else {
        '.'
    };
    
    str.contains(dec_point) || str.contains('e') || str.contains('E')
}

pub fn mpg_zero(n: *mut NODE) {
    unsafe {
        if is_mpg_float(n) {
            mpfr::clear((*n).mpg_numbr);
            (*n).flags &= !MPFN;
        }
        
        if !is_mpg_integer(n) {
            (*n).mpg_i.set_ui(0);
            (*n).flags |= MPZN;
        } else {
            (*n).mpg_i.set_si(0);
        }
    }
}

pub fn force_mpnum(n: *mut NODE, do_nondec: bool, use_locale: bool) -> bool {
    unsafe {
        if (*n).stlen == 0 || ((*n).flags & REGEX) != 0 {
            mpg_zero(n);
            return false;
        }
        
        let mut cp = (*n).stptr;
        let cpend = cp + (*n).stlen;
        
        while cp < cpend && (*cp as char).is_whitespace() {
            cp += 1;
        }
        
        if cp == cpend {
            mpg_zero(n);
            return false;
        }
        
        let save = *cpend;
        *cpend = 0;
        
        let mut cp1 = cp;
        if *cp == b'+' || *cp == b'-' {
            cp1 += 1;
        }
        
        if *cp1 == 0 {
            *cpend = save;
            mpg_zero(n);
            return false;
        }
        
        let base = if do_nondec {
            get_numbase(cp1, (cpend - cp1) as usize, use_locale)
        } else {
            10
        };
        
        if base != 10 || !mpg_maybe_float(std::str::from_utf8_unchecked(cp1), use_locale) {
            mpg_zero(n);
            let s = std::str::from_utf8_unchecked(cp1);
            if (*n).mpg_i.set_str(s, base).is_err() {
                *cpend = save;
                return false;
            }
            if *cp == b'-' {
                (*n).mpg_i.neg();
            }
        } else {
            if is_mpg_integer(n) {
                (*n).mpg_i.clear();
                (*n).flags &= !MPZN;
            }
            
            if !is_mpg_float(n) {
                mpfr::init((*n).mpg_numbr);
                (*n).flags |= MPFN;
            }
            
            let s = std::str::from_utf8_unchecked(cp);
            let tval = mpfr::set_str((*n).mpg_numbr, s, base, MPFR_ROUND_MODE);
            IEEE_FMT((*n).mpg_numbr, tval);
        }
        
        *cpend = save;
        true
    }
}

pub fn mpg_force_number(n: *mut NODE) -> *mut NODE {
    unsafe {
        if (*n).type_ == NodeType::NodeElemNew {
            (*n).type_ = NodeType::NodeVal;
            (*n).flags &= !STRING;
            *(*n).stptr = b'0';
            (*n).stlen = 1;
            return n;
        }
        
        if ((*n).flags & NUMCUR) != 0 {
            return n;
        }
        (*n).flags |= NUMCUR;
        
        let mut cp = (*n).stptr;
        let mut cpend = cp + (*n).stlen;
        
        while cp < cpend && (*cp as char).is_whitespace() {
            cp += 1;
        }
        
        if cp == cpend {
            goto_badnum(n);
            return n;
        }
        
        while (*cpend as char).is_whitespace() {
            cpend -= 1;
        }
        
        if !DO_POSIX {
            if (*cp as char).is_alphabetic() {
                goto_badnum(n);
                return n;
            } else if is_ieee_magic_val(cp) {
                if cpend != cp + 4 {
                    goto_badnum(n);
                    return n;
                }
            }
        }
        
        if force_mpnum(n, (DO_NON_DECIMAL_DATA && !DO_TRADITIONAL), true) {
            if ((*n).flags & USER_INPUT) != 0 {
                (*n).flags &= !STRING;
                (*n).flags |= NUMBER;
            }
        } else {
            (*n).flags &= !USER_INPUT;
        }
        
        return n;
        
        fn goto_badnum(n: *mut NODE) {
            unsafe {
                mpg_zero(n);
                (*n).flags &= !USER_INPUT;
            }
        }
    }
}

pub fn mpg_format_val(format: &str, index: i32, s: *mut NODE) -> *mut NODE {
    unsafe {
        if out_of_range(s) {
            let result = format_nan_inf(s, 'g');
            return make_string(result.as_ptr(), result.len());
        }
        
        let mut dummy = [ptr::null_mut(), s];
        let oflags = (*s).flags;
        let r: *mut NODE;
        
        if is_mpg_integer(s) || mpfr::integer_p((*s).mpg_numbr) {
            r = format_tree("%d", 2, dummy.as_mut_ptr(), 2);
            (*s).stfmt = STFMT_UNUSED;
        } else {
            let fmt = (*FMT_LIST.offset(index as isize)).stptr;
            let fmt_len = (*FMT_LIST.offset(index as isize)).stlen;
            r = format_tree(fmt, fmt_len as i32, dummy.as_mut_ptr(), 2);
            assert!(!r.is_null());
            (*s).stfmt = index;
        }
        
        (*s).flags = oflags;
        (*s).stlen = (*r).stlen;
        if ((*s).flags & (MALLOC | STRCUR)) == (MALLOC | STRCUR) {
            free((*s).stptr as *mut _);
        }
        (*s).stptr = (*r).stptr;
        (*s).flags |= STRCUR;
        (*s).strndmode = MPFR_ROUND_MODE as i32;
        free_node(r);
        free_wstr(s);
        s
    }
}

pub fn mpg_cmp(t1: *const NODE, t2: *const NODE) -> i32 {
    unsafe {
        if is_mpg_float(t1) {
            if is_mpg_float(t2) {
                if mpfr::nan_p((*t1).mpg_numbr) {
                    return !mpfr::nan_p((*t2).mpg_numbr) as i32;
                }
                if mpfr::nan_p((*t2).mpg_numbr) {
                    return -1;
                }
                return mpfr::cmp((*t1).mpg_numbr, (*t2).mpg_numbr);
            }
            if mpfr::nan_p((*t1).mpg_numbr) {
                return 1;
            }
            return mpfr::cmp_z((*t1).mpg_numbr, &(*t2).mpg_i);
        } else if is_mpg_float(t2) {
            if mpfr::nan_p((*t2).mpg_numbr) {
                return -1;
            }
            let ret = mpfr::cmp_z((*t2).mpg_numbr, &(*t1).mpg_i);
            if ret > 0 {
                -1
            } else if ret < 0 {
                1
            } else {
                0
            }
        } else if is_mpg_integer(t1) {
            return (*t1).mpg_i.cmp(&(*t2).mpg_i) as i32;
        }
        
        cmp_awknums(t1, t2)
    }
}

pub fn mpg_cmp_as_numbers(t1: *const NODE, t2: *const NODE, comparison_type: ScalarCmp) -> bool {
    unsafe {
        let t1_nan = is_mpg_float(t1) && mpfr::nan_p((*t1).mpg_numbr);
        let t2_nan = is_mpg_float(t2) && mpfr::nan_p((*t2).mpg_numbr);
        
        if t1_nan || t2_nan {
            return comparison_type == ScalarCmp::NEQ;
        }
        
        let di = mpg_cmp(t1, t2);
        
        match comparison_type {
            ScalarCmp::EQ => di == 0,
            ScalarCmp::NEQ => di != 0,
            ScalarCmp::LT => di < 0,
            ScalarCmp::LE => di <= 0,
            ScalarCmp::GT => di > 0,
            ScalarCmp::GE => di >= 0,
        }
    }
}

pub fn mpg_update_var(n: *mut NODE) -> *mut NODE {
    unsafe {
        let val = (*n).var_value;
        let (nr, nq) = if n == NR_NODE {
            (NR, &mut MNR)
        } else if n == FNR_NODE {
            (FNR, &mut MFNR)
        } else {
            panic!("invalid node for mpg_update_var");
        };
        
        if nq.cmp_ui(0) == 0 {
            if is_mpg_float(val) || (*val).mpg_i.cmp_si(nr) != 0 {
                unref((*n).var_value);
                (*n).var_value = mpg_integer();
                (*n).var_value.mpg_i.set_si(nr);
            }
        } else {
            unref((*n).var_value);
            (*n).var_value = mpg_integer();
            (*n).var_value.mpg_i.set_si(nr);
            (*n).var_value.mpg_i.addmul_ui(nq, c_ulong::MAX);
        }
        (*n).var_value
    }
}

pub fn mpg_set_var(n: *mut NODE) -> c_long {
    unsafe {
        let (nq, val) = if n == NR_NODE {
            (&mut MNR, (*n).var_value)
        } else if n == FNR_NODE {
            (&mut MFNR, (*n).var_value)
        } else {
            panic!("invalid node for mpg_set_var");
        };
        
        let r = if is_mpg_integer(val) {
            &(*val).mpg_i
        } else {
            mpfr::get_z(&mut MPZVAL, (*val).mpg_numbr, RoundingMode::Zero);
            &MPZVAL
        };
        
        let nr = nq.fdiv_q_ui(r, c_ulong::MAX);
        nr as c_long
    }
}

pub fn set_PREC() {
    unsafe {
        if !DO_MPFR {
            return;
        }
        
        let val = fixtype((*PREC_NODE).var_value);
        let mut prec = 0;
        
        if ((*val).flags & STRING) != 0 {
            let ieee_fmts = [
                ("half", 11, 16, -23),
                ("single", 24, 128, -148),
                ("double", 53, 1024, -1073),
                ("quad", 113, 16384, -16493),
                ("oct", 237, 262144, -262377),
            ];
            
            let s = std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                (*val).stptr,
                (*val).stlen as usize,
            ));
            
            for &(name, p, emax, emin) in &ieee_fmts {
                if s.eq_ignore_ascii_case(name) {
                    prec = p;
                    MAX_EXP = emax;
                    MIN_EXP = emin;
                    DO_IEEE_FMT = true;
                    break;
                }
            }
        }
        
        if prec <= 0 {
            force_number(val);
            prec = get_number_si(val);
            if prec < mpfr::PREC_MIN || prec > mpfr::PREC_MAX {
               