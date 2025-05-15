use std::ptr;
use std::ffi::c_void;
use std::mem;
use std::os::raw::c_int;

// Assuming these are defined in the GLPK Rust bindings
extern "C" {
    fn glp_add_rows(prob: *mut c_void, n: c_int) -> c_int;
    fn glp_set_mat_row(prob: *mut c_void, i: c_int, len: c_int, ind: *const c_int, val: *const f64);
    fn glp_set_row_bnds(prob: *mut c_void, i: c_int, type_: c_int, lb: f64, ub: f64);
    fn glp_create_prob() -> *mut c_void;
    fn glp_delete_prob(prob: *mut c_void);
    fn glp_minisat1(prob: *mut c_void) -> c_int;
    fn glp_del_rows(prob: *mut c_void, n: c_int, ind: *const c_int);
    fn xerror(fmt: *const i8, ...);
    fn xprintf(fmt: *const i8, ...);
    fn xcalloc(n: usize, size: usize) -> *mut c_void;
    fn xfree(ptr: *mut c_void);
    fn npp_create_wksp() -> *mut c_void;
    fn npp_load_prob(npp: *mut c_void, prob: *mut c_void, presolve: c_int, sol: c_int, check: c_int);
    fn npp_sat_encode_prob(npp: *mut c_void) -> c_int;
    fn npp_build_prob(npp: *mut c_void, prob: *mut c_void);
    fn npp_postprocess(npp: *mut c_void, prob: *mut c_void);
    fn npp_unload_sol(npp: *mut c_void, prob: *mut c_void);
    fn npp_delete_wksp(npp: *mut c_void);
    fn xassert(expr: bool);
}

const GLP_IV: c_int = 1;
const GLP_FX: c_int = 1;
const GLP_DB: c_int = 2;
const GLP_LO: c_int = 3;
const GLP_UP: c_int = 4;
const GLP_OFF: c_int = 0;
const GLP_MIP: c_int = 1;
const GLP_MIN: c_int = 1;
const GLP_MAX: c_int = 2;
const GLP_OPT: c_int = 1;
const GLP_FEAS: c_int = 2;
const GLP_UNDEF: c_int = 0;
const GLP_EDATA: c_int = 1;
const GLP_EBOUND: c_int = 2;
const GLP_ENOPFS: c_int = 3;
const GLP_ERANGE: c_int = 4;
const GLP_PROB_MAGIC: c_int = 0x474C5001;

#[repr(C)]
struct GLPCOL {
    kind: c_int,
    type_: c_int,
    lb: f64,
    ub: f64,
    coef: f64,
    mipx: f64,
    j: c_int,
}

#[repr(C)]
struct GLPROW {
    type_: c_int,
    lb: f64,
    ub: f64,
    mipx: f64,
    ptr: *mut GLPAIJ,
}

#[repr(C)]
struct GLPAIJ {
    val: f64,
    col: *mut GLPCOL,
    r_next: *mut GLPAIJ,
}

#[repr(C)]
struct GLPProb {
    magic: c_int,
    m: c_int,
    n: c_int,
    nnz: c_int,
    dir: c_int,
    c0: f64,
    mip_stat: c_int,
    mip_obj: f64,
    row: *mut *mut GLPROW,
    col: *mut *mut GLPCOL,
    tree: *mut c_void,
}

pub unsafe fn glp_intfeas1(P: *mut GLPProb, use_bound: c_int, obj_bound: c_int) -> c_int {
    let mut npp: *mut c_void = ptr::null_mut();
    let mut mip: *mut c_void = ptr::null_mut();
    let mut obj_ind: *mut c_int = ptr::null_mut();
    let mut obj_val: *mut f64 = ptr::null_mut();
    let mut obj_row: c_int = 0;
    let mut obj_len: c_int = 0;
    let mut ret: c_int = 0;

    if (*P).tree != ptr::null_mut() {
        xerror(b"glp_intfeas1: operation not allowed\n\0" as *const u8 as *const i8);
    }

    (*P).mip_stat = GLP_UNDEF;
    (*P).mip_obj = 0.0;

    // Check columns
    for j in 1..=(*P).n {
        let col = *(*P).col.offset(j as isize);
        if !(((*col).kind == GLP_IV && (*col).lb == 0.0 && (*col).ub == 1.0) || (*col).type_ == GLP_FX {
            xprintf(b"glp_intfeas1: column %d: non-binary non-fixed variable not allowed\n\0" as *const u8 as *const i8, j);
            ret = GLP_EDATA;
            goto done;
        }

        let temp = (*col).lb as c_int;
        if temp as f64 != (*col).lb {
            if (*col).type_ == GLP_FX {
                xprintf(b"glp_intfeas1: column %d: fixed value %g is non-integer or out of range\n\0" as *const u8 as *const i8, j, (*col).lb);
            } else {
                xprintf(b"glp_intfeas1: column %d: lower bound %g is non-integer or out of range\n\0" as *const u8 as *const i8, j, (*col).lb);
            }
            ret = GLP_EDATA;
            goto done;
        }

        let temp = (*col).ub as c_int;
        if temp as f64 != (*col).ub {
            xprintf(b"glp_intfeas1: column %d: upper bound %g is non-integer or out of range\n\0" as *const u8 as *const i8, j, (*col).ub);
            ret = GLP_EDATA;
            goto done;
        }

        if (*col).type_ == GLP_DB && (*col).lb > (*col).ub {
            xprintf(b"glp_intfeas1: column %d: lower bound %g is greater than upper bound %g\n\0" as *const u8 as *const i8, j, (*col).lb, (*col).ub);
            ret = GLP_EBOUND;
            goto done;
        }
    }

    // Check rows
    for i in 1..=(*P).m {
        let row = *(*P).row.offset(i as isize);
        let mut aij = (*row).ptr;
        while aij != ptr::null_mut() {
            let temp = (*(*aij).val) as c_int;
            if temp as f64 != *(*aij).val {
                xprintf(b"glp_intfeas1: row = %d, column %d: constraint coefficient %g is non-integer or out of range\n\0" as *const u8 as *const i8, i, (*(*aij).col).j, *(*aij).val);
                ret = GLP_EDATA;
                goto done;
            }
            aij = (*aij).r_next;
        }

        let temp = (*row).lb as c_int;
        if temp as f64 != (*row).lb {
            if (*row).type_ == GLP_FX {
                xprintf(b"glp_intfeas1: row = %d: fixed value %g is non-integer or out of range\n\0" as *const u8 as *const i8, i, (*row).lb);
            } else {
                xprintf(b"glp_intfeas1: row = %d: lower bound %g is non-integer or out of range\n\0" as *const u8 as *const i8, i, (*row).lb);
            }
            ret = GLP_EDATA;
            goto done;
        }

        let temp = (*row).ub as c_int;
        if temp as f64 != (*row).ub {
            xprintf(b"glp_intfeas1: row = %d: upper bound %g is non-integer or out of range\n\0" as *const u8 as *const i8, i, (*row).ub);
            ret = GLP_EDATA;
            goto done;
        }

        if (*row).type_ == GLP_DB && (*row).lb > (*row).ub {
            xprintf(b"glp_intfeas1: row %d: lower bound %g is greater than upper bound %g\n\0" as *const u8 as *const i8, i, (*row).lb, (*row).ub);
            ret = GLP_EBOUND;
            goto done;
        }
    }

    // Check objective function
    if use_bound != 0 {
        let temp = (*P).c0 as c_int;
        if temp as f64 != (*P).c0 {
            xprintf(b"glp_intfeas1: objective constant term %g is non-integer or out of range\n\0" as *const u8 as *const i8, (*P).c0);
            ret = GLP_EDATA;
            goto done;
        }

        for j in 1..=(*P).n {
            let col = *(*P).col.offset(j as isize);
            let temp = (*col).coef as c_int;
            if temp as f64 != (*col).coef {
                xprintf(b"glp_intfeas1: column %d: objective coefficient is non-integer or out of range\n\0" as *const u8 as *const i8, j, (*col).coef);
                ret = GLP_EDATA;
                goto done;
            }
        }
    }

    // Save objective function
    obj_ind = xcalloc(((*P).n + 1) as usize, mem::size_of::<c_int>()) as *mut c_int;
    obj_val = xcalloc(((*P).n + 1) as usize, mem::size_of::<f64>()) as *mut f64;
    obj_len = 0;
    *obj_ind.offset(0) = 0;
    *obj_val.offset(0) = (*P).c0;
    (*P).c0 = 0.0;

    for j in 1..=(*P).n {
        let col = *(*P).col.offset(j as isize);
        if (*col).coef != 0.0 {
            obj_len += 1;
            *obj_ind.offset(obj_len as isize) = j;
            *obj_val.offset(obj_len as isize) = (*col).coef;
            (*col).coef = 0.0;
        }
    }

    // Add objective bound constraint if needed
    if use_bound == 0 {
        xprintf(b"Will search for ANY feasible solution\n\0" as *const u8 as *const i8);
    } else {
        xprintf(b"Will search only for solution not worse than %d\n\0" as *const u8 as *const i8, obj_bound);
        obj_row = glp_add_rows(P as *mut c_void, 1);
        glp_set_mat_row(P as *mut c_void, obj_row, obj_len, obj_ind, obj_val);
        if (*P).dir == GLP_MIN {
            glp_set_row_bnds(P as *mut c_void, obj_row, GLP_UP, 0.0, obj_bound as f64 - *obj_val.offset(0));
        } else if (*P).dir == GLP_MAX {
            glp_set_row_bnds(P as *mut c_void, obj_row, GLP_LO, obj_bound as f64 - *obj_val.offset(0), 0.0);
        } else {
            xassert(false);
        }
    }

    // Create preprocessor workspace
    xprintf(b"Translating to CNF-SAT...\n\0" as *const u8 as *const i8);
    xprintf(b"Original problem has %d row%s, %d column%s, and %d non-zero%s\n\0" as *const u8 as *const i8,
        (*P).m, if (*P).m == 1 { b"\0" } else { b"s\0" },
        (*P).n, if (*P).n == 1 { b"\0" } else { b"s\0" },
        (*P).nnz, if (*P).nnz == 1 { b"\0" } else { b"s\0" });

    npp = npp_create_wksp();
    npp_load_prob(npp, P as *mut c_void, GLP_OFF, GLP_MIP, GLP_OFF);

    ret = npp_sat_encode_prob(npp);
    if ret == 0 {
        // Continue
    } else if ret == GLP_ENOPFS {
        xprintf(b"PROBLEM HAS NO INTEGER FEASIBLE SOLUTION\n\0" as *const u8 as *const i8);
        goto done;
    } else if ret == GLP_ERANGE {
        xprintf(b"glp_intfeas1: translation to SAT-CNF failed because of integer overflow\n\0" as *const u8 as *const i8);
        goto done;
    } else {
        xassert(false);
    }

    mip = glp_create_prob();
    npp_build_prob(npp, mip);
    ret = glp_minisat1(mip);

    let mip_stat = (*(mip as *mut GLPProb)).mip_stat;
    if !(mip_stat == GLP_OPT || mip_stat == GLP_FEAS) {
        (*P).mip_stat = mip_stat;
        goto done;
    }

    npp_postprocess(npp, mip);
    glp_delete_prob(mip);
    mip = ptr::null_mut();

    npp_unload_sol(npp, P as *mut c_void);
    (*P).mip_stat = GLP_FEAS;

    // Verify solution
    for i in 1..=(*P).m {
        let row = *(*P).row.offset(i as isize);
        let mut sum = 0.0;
        let mut aij = (*row).ptr;
        while aij != ptr::null_mut() {
            sum += *(*aij).val * (*(*aij).col).mipx;
            aij = (*aij).r_next;
        }
        xassert(sum == (*row).mipx);
        if (*row).type_ == GLP_LO || (*row).type_ == GLP_DB || (*row).type_ == GLP_FX {
            xassert(sum >= (*row).lb);
        }
        if (*row).type_ == GLP_UP || (*row).type_ == GLP_DB || (*row).type_ == GLP_FX {
            xassert(sum <= (*row).ub);
        }
    }

    // Compute objective value
    (*P).mip_obj = *obj_val.offset(0);
    for k in 1..=obj_len {
        (*P).mip_obj += *obj_val.offset(k as isize) * (*(*P).col.offset(*obj_ind.offset(k as isize) as isize)).mipx;
    }
    xprintf(b"Objective value = %17.9e\n\0" as *const u8 as *const i8, (*P).mip_obj);

done:
    if !mip.is_null() {
        glp_delete_prob(mip);
    }
    if !npp.is_null() {
        npp_delete_wksp(npp);
    }
    if obj_row > 0 {
        let mut ind = [0; 2];
        ind[1] = obj_row;
        glp_del_rows(P as *mut c_void, 1, ind.as_ptr());
    }
    if !obj_ind.is_null() {
        (*P).c0 = *obj_val.offset(0);
        for k in 1..=obj_len {
            (*(*P).col.offset(*obj_ind.offset(k as isize) as isize)).coef = *obj_val.offset(k as isize);
        }
        xfree(obj_ind as *mut c_void);
        xfree(obj_val as *mut c_void);
    }

    ret
}

// Helper macro for goto simulation
macro_rules! goto {
    ($label:ident) => {
        break $label;
    };
}