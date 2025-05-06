#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type AVL;
    pub type AVLNODE;
    pub type DMP;
    pub type glp_file;
    pub type BFD;
    pub type glp_tree;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn _glp_mpl_terminate(mpl: *mut MPL);
    fn _glp_mpl_postsolve(mpl: *mut MPL) -> i32;
    fn _glp_mpl_put_col_soln(
        mpl: *mut MPL,
        j: i32,
        stat: i32,
        prim: libc::c_double,
        dual: libc::c_double,
    );
    fn _glp_mpl_put_row_soln(
        mpl: *mut MPL,
        i: i32,
        stat: i32,
        prim: libc::c_double,
        dual: libc::c_double,
    );
    fn _glp_mpl_has_solve_stmt(mpl: *mut MPL) -> i32;
    fn _glp_mpl_get_col_bnds(
        mpl: *mut MPL,
        j: i32,
        lb: *mut libc::c_double,
        ub: *mut libc::c_double,
    ) -> i32;
    fn _glp_mpl_get_col_kind(mpl: *mut MPL, j: i32) -> i32;
    fn _glp_mpl_get_col_name(mpl: *mut MPL, j: i32) -> *mut i8;
    fn _glp_mpl_get_row_c0(mpl: *mut MPL, i: i32) -> libc::c_double;
    fn _glp_mpl_get_mat_row(
        mpl: *mut MPL,
        i: i32,
        ndx: *mut i32,
        val: *mut libc::c_double,
    ) -> i32;
    fn _glp_mpl_get_row_bnds(
        mpl: *mut MPL,
        i: i32,
        lb: *mut libc::c_double,
        ub: *mut libc::c_double,
    ) -> i32;
    fn _glp_mpl_get_row_kind(mpl: *mut MPL, i: i32) -> i32;
    fn _glp_mpl_get_row_name(mpl: *mut MPL, i: i32) -> *mut i8;
    fn _glp_mpl_get_num_cols(mpl: *mut MPL) -> i32;
    fn _glp_mpl_get_num_rows(mpl: *mut MPL) -> i32;
    fn _glp_mpl_get_prob_name(mpl: *mut MPL) -> *mut i8;
    fn _glp_mpl_generate(mpl: *mut MPL, file: *mut i8) -> i32;
    fn _glp_mpl_read_data(mpl: *mut MPL, file: *mut i8) -> i32;
    fn _glp_mpl_read_model(mpl: *mut MPL, file: *mut i8, skip_data: i32) -> i32;
    fn _glp_mpl_initialize() -> *mut MPL;
    fn glp_printf(fmt: *const i8, _: ...);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_free(ptr: *mut libc::c_void);
    fn _glp_rng_init_rand(rand: *mut RNG, seed: i32);
    fn glp_set_prob_name(P: *mut glp_prob, name: *const i8);
    fn glp_set_obj_name(P: *mut glp_prob, name: *const i8);
    fn glp_set_obj_dir(P: *mut glp_prob, dir: i32);
    fn glp_add_rows(P: *mut glp_prob, nrs: i32) -> i32;
    fn glp_add_cols(P: *mut glp_prob, ncs: i32) -> i32;
    fn glp_set_row_name(P: *mut glp_prob, i: i32, name: *const i8);
    fn glp_set_col_name(P: *mut glp_prob, j: i32, name: *const i8);
    fn glp_set_row_bnds(
        P: *mut glp_prob,
        i: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_col_bnds(
        P: *mut glp_prob,
        j: i32,
        type_0: i32,
        lb: libc::c_double,
        ub: libc::c_double,
    );
    fn glp_set_obj_coef(P: *mut glp_prob, j: i32, coef: libc::c_double);
    fn glp_set_mat_row(
        P: *mut glp_prob,
        i: i32,
        len: i32,
        ind: *const i32,
        val: *const libc::c_double,
    );
    fn glp_erase_prob(P: *mut glp_prob);
    fn glp_get_num_rows(P: *mut glp_prob) -> i32;
    fn glp_get_num_cols(P: *mut glp_prob) -> i32;
    fn glp_get_row_stat(P: *mut glp_prob, i: i32) -> i32;
    fn glp_get_row_prim(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_row_dual(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_get_col_stat(P: *mut glp_prob, j: i32) -> i32;
    fn glp_get_col_prim(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_get_col_dual(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_ipt_row_prim(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_ipt_row_dual(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_ipt_col_prim(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_ipt_col_dual(P: *mut glp_prob, j: i32) -> libc::c_double;
    fn glp_set_col_kind(P: *mut glp_prob, j: i32, kind: i32);
    fn glp_mip_row_val(P: *mut glp_prob, i: i32) -> libc::c_double;
    fn glp_mip_col_val(P: *mut glp_prob, j: i32) -> libc::c_double;
}
pub type __jmp_buf = [i64; 8];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [u64; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: i32,
    pub __saved_mask: __sigset_t,
}
pub type jmp_buf = [__jmp_buf_tag; 1];
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RNG {
    pub A: [i32; 56],
    pub fptr: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_tran {
    pub line: i32,
    pub c: i32,
    pub token: i32,
    pub imlen: i32,
    pub image: *mut i8,
    pub value: libc::c_double,
    pub b_token: i32,
    pub b_imlen: i32,
    pub b_image: *mut i8,
    pub b_value: libc::c_double,
    pub f_dots: i32,
    pub f_scan: i32,
    pub f_token: i32,
    pub f_imlen: i32,
    pub f_image: *mut i8,
    pub f_value: libc::c_double,
    pub context: *mut i8,
    pub c_ptr: i32,
    pub flag_d: i32,
    pub pool: *mut DMP,
    pub tree: *mut AVL,
    pub model: *mut STATEMENT,
    pub flag_x: i32,
    pub as_within: i32,
    pub as_in: i32,
    pub as_binary: i32,
    pub flag_s: i32,
    pub strings: *mut DMP,
    pub symbols: *mut DMP,
    pub tuples: *mut DMP,
    pub arrays: *mut DMP,
    pub members: *mut DMP,
    pub elemvars: *mut DMP,
    pub formulae: *mut DMP,
    pub elemcons: *mut DMP,
    pub a_list: *mut ARRAY,
    pub sym_buf: *mut i8,
    pub tup_buf: *mut i8,
    pub rand: *mut RNG,
    pub flag_p: i32,
    pub stmt: *mut STATEMENT,
    pub dca: *mut TABDCA,
    pub m: i32,
    pub n: i32,
    pub row: *mut *mut ELEMCON,
    pub col: *mut *mut ELEMVAR,
    pub in_fp: *mut glp_file,
    pub in_file: *mut i8,
    pub out_fp: *mut glp_file,
    pub out_file: *mut i8,
    pub prt_fp: *mut glp_file,
    pub prt_file: *mut i8,
    pub jump: jmp_buf,
    pub phase: i32,
    pub mod_file: *mut i8,
    pub mpl_buf: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMVAR {
    pub j: i32,
    pub var: *mut VARIABLE,
    pub memb: *mut MEMBER,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub temp: libc::c_double,
    pub stat: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MEMBER {
    pub tuple: *mut TUPLE,
    pub next: *mut MEMBER,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union VALUE {
    pub none: *mut libc::c_void,
    pub num: libc::c_double,
    pub sym: *mut SYMBOL,
    pub bit: i32,
    pub tuple: *mut TUPLE,
    pub set: *mut ELEMSET,
    pub var: *mut ELEMVAR,
    pub form: *mut FORMULA,
    pub con: *mut ELEMCON,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ELEMCON {
    pub i: i32,
    pub con: *mut CONSTRAINT,
    pub memb: *mut MEMBER,
    pub form: *mut FORMULA,
    pub lbnd: libc::c_double,
    pub ubnd: libc::c_double,
    pub stat: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FORMULA {
    pub coef: libc::c_double,
    pub var: *mut ELEMVAR,
    pub next: *mut FORMULA,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONSTRAINT {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub dim: i32,
    pub domain: *mut DOMAIN1,
    pub type_0: i32,
    pub code: *mut CODE,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARRAY {
    pub type_0: i32,
    pub dim: i32,
    pub size: i32,
    pub head: *mut MEMBER,
    pub tail: *mut MEMBER,
    pub tree: *mut AVL,
    pub prev: *mut ARRAY,
    pub next: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CODE {
    pub op: i32,
    pub arg: OPERANDS,
    pub type_0: i32,
    pub dim: i32,
    pub up: *mut CODE,
    pub vflag: i32,
    pub valid: i32,
    pub value: VALUE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union OPERANDS {
    pub num: libc::c_double,
    pub str_0: *mut i8,
    pub index: C2RustUnnamed_5,
    pub par: C2RustUnnamed_4,
    pub set: C2RustUnnamed_3,
    pub var: C2RustUnnamed_2,
    pub con: C2RustUnnamed_1,
    pub list: *mut ARG_LIST,
    pub slice: *mut DOMAIN_BLOCK,
    pub arg: C2RustUnnamed_0,
    pub loop_0: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub domain: *mut DOMAIN1,
    pub x: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN1 {
    pub list: *mut DOMAIN_BLOCK,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_BLOCK {
    pub list: *mut DOMAIN_SLOT,
    pub code: *mut CODE,
    pub backup: *mut TUPLE,
    pub next: *mut DOMAIN_BLOCK,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TUPLE {
    pub sym: *mut SYMBOL,
    pub next: *mut TUPLE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SYMBOL {
    pub num: libc::c_double,
    pub str_0: *mut STRING,
}
pub type STRING = i8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOMAIN_SLOT {
    pub name: *mut i8,
    pub code: *mut CODE,
    pub value: *mut SYMBOL,
    pub list: *mut CODE,
    pub next: *mut DOMAIN_SLOT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub x: *mut CODE,
    pub y: *mut CODE,
    pub z: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ARG_LIST {
    pub x: *mut CODE,
    pub next: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub con: *mut CONSTRAINT,
    pub list: *mut ARG_LIST,
    pub suff: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub var: *mut VARIABLE,
    pub list: *mut ARG_LIST,
    pub suff: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VARIABLE {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub dim: i32,
    pub domain: *mut DOMAIN1,
    pub type_0: i32,
    pub lbnd: *mut CODE,
    pub ubnd: *mut CODE,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub set: *mut SET,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SET {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub dim: i32,
    pub domain: *mut DOMAIN1,
    pub dimen: i32,
    pub within: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub gadget: *mut GADGET,
    pub data: i32,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GADGET {
    pub set: *mut SET,
    pub ind: [i32; 20],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WITHIN {
    pub code: *mut CODE,
    pub next: *mut WITHIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub par: *mut PARAMETER,
    pub list: *mut ARG_LIST,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PARAMETER {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub dim: i32,
    pub domain: *mut DOMAIN1,
    pub type_0: i32,
    pub cond: *mut CONDITION,
    pub in_0: *mut WITHIN,
    pub assign: *mut CODE,
    pub option: *mut CODE,
    pub data: i32,
    pub defval: *mut SYMBOL,
    pub array: *mut ARRAY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CONDITION {
    pub rho: i32,
    pub code: *mut CODE,
    pub next: *mut CONDITION,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub slot: *mut DOMAIN_SLOT,
    pub next: *mut CODE,
}
pub type ELEMSET = ARRAY;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABDCA {
    pub id: i32,
    pub link: *mut libc::c_void,
    pub na: i32,
    pub arg: *mut *mut i8,
    pub nf: i32,
    pub name: *mut *mut i8,
    pub type_0: *mut i32,
    pub num: *mut libc::c_double,
    pub str_0: *mut *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STATEMENT {
    pub line: i32,
    pub type_0: i32,
    pub u: C2RustUnnamed_6,
    pub next: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub tab: *mut TABLE,
    pub slv: *mut libc::c_void,
    pub chk: *mut CHECK,
    pub dpy: *mut DISPLAY,
    pub prt: *mut PRINTF,
    pub fur: *mut FOR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FOR {
    pub domain: *mut DOMAIN1,
    pub list: *mut STATEMENT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF {
    pub domain: *mut DOMAIN1,
    pub fmt: *mut CODE,
    pub list: *mut PRINTF1,
    pub fname: *mut CODE,
    pub app: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PRINTF1 {
    pub code: *mut CODE,
    pub next: *mut PRINTF1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY {
    pub domain: *mut DOMAIN1,
    pub list: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DISPLAY1 {
    pub type_0: i32,
    pub u: C2RustUnnamed_7,
    pub next: *mut DISPLAY1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub slot: *mut DOMAIN_SLOT,
    pub set: *mut SET,
    pub par: *mut PARAMETER,
    pub var: *mut VARIABLE,
    pub con: *mut CONSTRAINT,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CHECK {
    pub domain: *mut DOMAIN1,
    pub code: *mut CODE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABLE {
    pub name: *mut i8,
    pub alias: *mut i8,
    pub type_0: i32,
    pub arg: *mut TABARG,
    pub u: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub in_0: C2RustUnnamed_10,
    pub out: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub domain: *mut DOMAIN1,
    pub list: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABOUT {
    pub code: *mut CODE,
    pub name: *mut i8,
    pub next: *mut TABOUT,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub set: *mut SET,
    pub fld: *mut TABFLD,
    pub list: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABIN {
    pub par: *mut PARAMETER,
    pub name: *mut i8,
    pub next: *mut TABIN,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABFLD {
    pub name: *mut i8,
    pub next: *mut TABFLD,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TABARG {
    pub code: *mut CODE,
    pub next: *mut TABARG,
}
pub type MPL = glp_tran;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glp_prob {
    pub pool: *mut DMP,
    pub tree: *mut glp_tree,
    pub name: *mut i8,
    pub obj: *mut i8,
    pub dir: i32,
    pub c0: libc::c_double,
    pub m_max: i32,
    pub n_max: i32,
    pub m: i32,
    pub n: i32,
    pub nnz: i32,
    pub row: *mut *mut GLPROW,
    pub col: *mut *mut GLPCOL,
    pub r_tree: *mut AVL,
    pub c_tree: *mut AVL,
    pub valid: i32,
    pub head: *mut i32,
    pub bfd: *mut BFD,
    pub pbs_stat: i32,
    pub dbs_stat: i32,
    pub obj_val: libc::c_double,
    pub it_cnt: i32,
    pub some: i32,
    pub ipt_stat: i32,
    pub ipt_obj: libc::c_double,
    pub mip_stat: i32,
    pub mip_obj: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPCOL {
    pub j: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub kind: i32,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub coef: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub sjj: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPAIJ {
    pub row: *mut GLPROW,
    pub col: *mut GLPCOL,
    pub val: libc::c_double,
    pub r_prev: *mut GLPAIJ,
    pub r_next: *mut GLPAIJ,
    pub c_prev: *mut GLPAIJ,
    pub c_next: *mut GLPAIJ,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GLPROW {
    pub i: i32,
    pub name: *mut i8,
    pub node: *mut AVLNODE,
    pub level: i32,
    pub origin: u8,
    pub klass: u8,
    pub type_0: i32,
    pub lb: libc::c_double,
    pub ub: libc::c_double,
    pub ptr: *mut GLPAIJ,
    pub rii: libc::c_double,
    pub stat: i32,
    pub bind: i32,
    pub prim: libc::c_double,
    pub dual: libc::c_double,
    pub pval: libc::c_double,
    pub dval: libc::c_double,
    pub mipx: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn glp_mpl_alloc_wksp() -> *mut glp_tran {
    let mut tran: *mut glp_tran = 0 as *mut glp_tran;
    tran = _glp_mpl_initialize();
    return tran;
}
#[no_mangle]
pub unsafe extern "C" fn glp_mpl_init_rand(mut tran: *mut glp_tran, mut seed: i32) {
    if (*tran).phase != 0 as i32 {
        (glp_error_(b"api/mpl.c\0" as *const u8 as *const i8, 35 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_mpl_init_rand: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    _glp_rng_init_rand((*tran).rand, seed);
}
#[no_mangle]
pub unsafe extern "C" fn glp_mpl_read_model(
    mut tran: *mut glp_tran,
    mut fname: *const i8,
    mut skip: i32,
) -> i32 {
    let mut ret: i32 = 0;
    if (*tran).phase != 0 as i32 {
        (glp_error_(b"api/mpl.c\0" as *const u8 as *const i8, 44 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mpl_read_model: invalid call sequence\n\0" as *const u8 as *const i8,
        );
    }
    ret = _glp_mpl_read_model(tran, fname as *mut i8, skip);
    if ret == 1 as i32 || ret == 2 as i32 {
        ret = 0 as i32;
    } else if ret == 4 as i32 {
        ret = 1 as i32;
    } else {
        (ret != ret
            || {
                glp_assert_(
                    b"ret != ret\0" as *const u8 as *const i8,
                    b"api/mpl.c\0" as *const u8 as *const i8,
                    51 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glp_mpl_read_data(
    mut tran: *mut glp_tran,
    mut fname: *const i8,
) -> i32 {
    let mut ret: i32 = 0;
    if !((*tran).phase == 1 as i32 || (*tran).phase == 2 as i32) {
        (glp_error_(b"api/mpl.c\0" as *const u8 as *const i8, 59 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_mpl_read_data: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    ret = _glp_mpl_read_data(tran, fname as *mut i8);
    if ret == 2 as i32 {
        ret = 0 as i32;
    } else if ret == 4 as i32 {
        ret = 1 as i32;
    } else {
        (ret != ret
            || {
                glp_assert_(
                    b"ret != ret\0" as *const u8 as *const i8,
                    b"api/mpl.c\0" as *const u8 as *const i8,
                    66 as i32,
                );
                1 as i32 != 0
            }) as i32;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glp_mpl_generate(
    mut tran: *mut glp_tran,
    mut fname: *const i8,
) -> i32 {
    let mut ret: i32 = 0;
    if !((*tran).phase == 1 as i32 || (*tran).phase == 2 as i32) {
        (glp_error_(b"api/mpl.c\0" as *const u8 as *const i8, 74 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_mpl_generate: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    ret = _glp_mpl_generate(tran, fname as *mut i8);
    if ret == 3 as i32 {
        ret = 0 as i32;
    } else if ret == 4 as i32 {
        ret = 1 as i32;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glp_mpl_build_prob(
    mut tran: *mut glp_tran,
    mut prob: *mut glp_prob,
) {
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut t: i32 = 0;
    let mut kind: i32 = 0;
    let mut type_0: i32 = 0;
    let mut len: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut lb: libc::c_double = 0.;
    let mut ub: libc::c_double = 0.;
    let mut val: *mut libc::c_double = 0 as *mut libc::c_double;
    if (*tran).phase != 3 as i32 {
        (glp_error_(b"api/mpl.c\0" as *const u8 as *const i8, 88 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mpl_build_prob: invalid call sequence\n\0" as *const u8 as *const i8,
        );
    }
    glp_erase_prob(prob);
    glp_set_prob_name(prob, _glp_mpl_get_prob_name(tran));
    m = _glp_mpl_get_num_rows(tran);
    if m > 0 as i32 {
        glp_add_rows(prob, m);
    }
    i = 1 as i32;
    while i <= m {
        glp_set_row_name(prob, i, _glp_mpl_get_row_name(tran, i));
        type_0 = _glp_mpl_get_row_bnds(tran, i, &mut lb, &mut ub);
        match type_0 {
            401 => {
                type_0 = 1 as i32;
            }
            402 => {
                type_0 = 2 as i32;
            }
            403 => {
                type_0 = 3 as i32;
            }
            404 => {
                type_0 = 4 as i32;
            }
            405 => {
                type_0 = 5 as i32;
            }
            _ => {
                (type_0 != type_0
                    || {
                        glp_assert_(
                            b"type != type\0" as *const u8 as *const i8,
                            b"api/mpl.c\0" as *const u8 as *const i8,
                            108 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
        if type_0 == 4 as i32 && fabs(lb - ub) < 1e-9f64 * (1.0f64 + fabs(lb)) {
            type_0 = 5 as i32;
            if fabs(lb) <= fabs(ub) {
                ub = lb;
            } else {
                lb = ub;
            }
        }
        glp_set_row_bnds(prob, i, type_0, lb, ub);
        if _glp_mpl_get_row_c0(tran, i) != 0.0f64 {
            glp_printf(
                b"glp_mpl_build_prob: row %s; constant term %.12g ignored\n\0"
                    as *const u8 as *const i8,
                _glp_mpl_get_row_name(tran, i),
                _glp_mpl_get_row_c0(tran, i),
            );
        }
        i += 1;
        i;
    }
    n = _glp_mpl_get_num_cols(tran);
    if n > 0 as i32 {
        glp_add_cols(prob, n);
    }
    j = 1 as i32;
    while j <= n {
        glp_set_col_name(prob, j, _glp_mpl_get_col_name(tran, j));
        kind = _glp_mpl_get_col_kind(tran, j);
        match kind {
            421 => {}
            422 | 423 => {
                glp_set_col_kind(prob, j, 2 as i32);
            }
            _ => {
                (kind != kind
                    || {
                        glp_assert_(
                            b"kind != kind\0" as *const u8 as *const i8,
                            b"api/mpl.c\0" as *const u8 as *const i8,
                            138 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
        type_0 = _glp_mpl_get_col_bnds(tran, j, &mut lb, &mut ub);
        match type_0 {
            401 => {
                type_0 = 1 as i32;
            }
            402 => {
                type_0 = 2 as i32;
            }
            403 => {
                type_0 = 3 as i32;
            }
            404 => {
                type_0 = 4 as i32;
            }
            405 => {
                type_0 = 5 as i32;
            }
            _ => {
                (type_0 != type_0
                    || {
                        glp_assert_(
                            b"type != type\0" as *const u8 as *const i8,
                            b"api/mpl.c\0" as *const u8 as *const i8,
                            148 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
        }
        if kind == 423 as i32 {
            if type_0 == 1 as i32 || type_0 == 3 as i32 || lb < 0.0f64 {
                lb = 0.0f64;
            }
            if type_0 == 1 as i32 || type_0 == 2 as i32 || ub > 1.0f64 {
                ub = 1.0f64;
            }
            type_0 = 4 as i32;
        }
        if type_0 == 4 as i32 && fabs(lb - ub) < 1e-9f64 * (1.0f64 + fabs(lb)) {
            type_0 = 5 as i32;
            if fabs(lb) <= fabs(ub) {
                ub = lb;
            } else {
                lb = ub;
            }
        }
        glp_set_col_bnds(prob, j, type_0, lb, ub);
        j += 1;
        j;
    }
    ind = glp_alloc(1 as i32 + n, ::core::mem::size_of::<i32>() as u64 as i32)
        as *mut i32;
    val = glp_alloc(1 as i32 + n, ::core::mem::size_of::<libc::c_double>() as u64 as i32)
        as *mut libc::c_double;
    i = 1 as i32;
    while i <= m {
        len = _glp_mpl_get_mat_row(tran, i, ind, val);
        glp_set_mat_row(prob, i, len, ind as *const i32, val as *const libc::c_double);
        i += 1;
        i;
    }
    i = 1 as i32;
    while i <= m {
        kind = _glp_mpl_get_row_kind(tran, i);
        if kind == 412 as i32 || kind == 413 as i32 {
            glp_set_obj_name(prob, _glp_mpl_get_row_name(tran, i));
            glp_set_obj_dir(prob, if kind == 412 as i32 { 1 as i32 } else { 2 as i32 });
            glp_set_obj_coef(prob, 0 as i32, _glp_mpl_get_row_c0(tran, i));
            len = _glp_mpl_get_mat_row(tran, i, ind, val);
            t = 1 as i32;
            while t <= len {
                glp_set_obj_coef(prob, *ind.offset(t as isize), *val.offset(t as isize));
                t += 1;
                t;
            }
            break;
        } else {
            i += 1;
            i;
        }
    }
    glp_free(ind as *mut libc::c_void);
    glp_free(val as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn glp_mpl_postsolve(
    mut tran: *mut glp_tran,
    mut prob: *mut glp_prob,
    mut sol: i32,
) -> i32 {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut stat: i32 = 0;
    let mut ret: i32 = 0;
    let mut prim: libc::c_double = 0.;
    let mut dual: libc::c_double = 0.;
    if !((*tran).phase == 3 as i32 && (*tran).flag_p == 0) {
        (glp_error_(b"api/mpl.c\0" as *const u8 as *const i8, 196 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_mpl_postsolve: invalid call sequence\n\0" as *const u8 as *const i8);
    }
    if !(sol == 1 as i32 || sol == 2 as i32 || sol == 3 as i32) {
        (glp_error_(b"api/mpl.c\0" as *const u8 as *const i8, 198 as i32))
            .expect(
                "non-null function pointer",
            )(
            b"glp_mpl_postsolve: sol = %d; invalid parameter\n\0" as *const u8
                as *const i8,
            sol,
        );
    }
    m = _glp_mpl_get_num_rows(tran);
    n = _glp_mpl_get_num_cols(tran);
    if !(m == glp_get_num_rows(prob) && n == glp_get_num_cols(prob)) {
        (glp_error_(b"api/mpl.c\0" as *const u8 as *const i8, 204 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_mpl_postsolve: wrong problem object\n\0" as *const u8 as *const i8);
    }
    if _glp_mpl_has_solve_stmt(tran) == 0 {
        ret = 0 as i32;
    } else {
        i = 1 as i32;
        while i <= m {
            if sol == 1 as i32 {
                stat = glp_get_row_stat(prob, i);
                prim = glp_get_row_prim(prob, i);
                dual = glp_get_row_dual(prob, i);
            } else if sol == 2 as i32 {
                stat = 0 as i32;
                prim = glp_ipt_row_prim(prob, i);
                dual = glp_ipt_row_dual(prob, i);
            } else if sol == 3 as i32 {
                stat = 0 as i32;
                prim = glp_mip_row_val(prob, i);
                dual = 0.0f64;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const i8,
                            b"api/mpl.c\0" as *const u8 as *const i8,
                            226 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            if fabs(prim) < 1e-9f64 {
                prim = 0.0f64;
            }
            if fabs(dual) < 1e-9f64 {
                dual = 0.0f64;
            }
            _glp_mpl_put_row_soln(tran, i, stat, prim, dual);
            i += 1;
            i;
        }
        j = 1 as i32;
        while j <= n {
            if sol == 1 as i32 {
                stat = glp_get_col_stat(prob, j);
                prim = glp_get_col_prim(prob, j);
                dual = glp_get_col_dual(prob, j);
            } else if sol == 2 as i32 {
                stat = 0 as i32;
                prim = glp_ipt_col_prim(prob, j);
                dual = glp_ipt_col_dual(prob, j);
            } else if sol == 3 as i32 {
                stat = 0 as i32;
                prim = glp_mip_col_val(prob, j);
                dual = 0.0f64;
            } else {
                (sol != sol
                    || {
                        glp_assert_(
                            b"sol != sol\0" as *const u8 as *const i8,
                            b"api/mpl.c\0" as *const u8 as *const i8,
                            248 as i32,
                        );
                        1 as i32 != 0
                    }) as i32;
            }
            if fabs(prim) < 1e-9f64 {
                prim = 0.0f64;
            }
            if fabs(dual) < 1e-9f64 {
                dual = 0.0f64;
            }
            _glp_mpl_put_col_soln(tran, j, stat, prim, dual);
            j += 1;
            j;
        }
        ret = _glp_mpl_postsolve(tran);
        if ret == 3 as i32 {
            ret = 0 as i32;
        } else if ret == 4 as i32 {
            ret = 1 as i32;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn glp_mpl_free_wksp(mut tran: *mut glp_tran) {
    _glp_mpl_terminate(tran);
}