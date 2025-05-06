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
    pub type BFD;
    pub type DMP;
    pub type glp_tree;
    fn glp_free(ptr: *mut libc::c_void);
    fn glp_alloc(n: i32, size: i32) -> *mut libc::c_void;
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn glp_error_(file: *const i8, line: i32) -> glp_errfunc;
    fn glp_printf(fmt: *const i8, _: ...);
    fn _glp_minisat_new() -> *mut solver;
    fn _glp_minisat_delete(s: *mut solver);
    fn _glp_minisat_addclause(s: *mut solver, begin: *mut lit, end: *mut lit) -> bool_0;
    fn _glp_minisat_solve(s: *mut solver, begin: *mut lit, end: *mut lit) -> bool_0;
    fn _glp_minisat_setnvars(s: *mut solver, n: i32);
    fn glp_check_cnfsat(P: *mut glp_prob) -> i32;
}
pub type size_t = u64;
pub type glp_errfunc = Option<unsafe extern "C" fn(*const i8, ...) -> ()>;
pub type bool_0 = i32;
pub type lit = i32;
pub type lbool = i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct veci {
    pub size: i32,
    pub cap: i32,
    pub ptr: *mut i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vecp {
    pub size: i32,
    pub cap: i32,
    pub ptr: *mut *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clause {
    pub size_learnt: i32,
    pub lits: [lit; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stats {
    pub starts: libc::c_double,
    pub decisions: libc::c_double,
    pub propagations: libc::c_double,
    pub inspects: libc::c_double,
    pub conflicts: libc::c_double,
    pub clauses: libc::c_double,
    pub clauses_literals: libc::c_double,
    pub learnts: libc::c_double,
    pub learnts_literals: libc::c_double,
    pub max_literals: libc::c_double,
    pub tot_literals: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct solver {
    pub size: i32,
    pub cap: i32,
    pub qhead: i32,
    pub qtail: i32,
    pub clauses: vecp,
    pub learnts: vecp,
    pub var_inc: libc::c_double,
    pub var_decay: libc::c_double,
    pub cla_inc: libc::c_float,
    pub cla_decay: libc::c_float,
    pub wlists: *mut vecp,
    pub activity: *mut libc::c_double,
    pub assigns: *mut lbool,
    pub orderpos: *mut i32,
    pub reasons: *mut *mut clause,
    pub levels: *mut i32,
    pub trail: *mut lit,
    pub binary: *mut clause,
    pub tags: *mut lbool,
    pub tagged: veci,
    pub stack: veci,
    pub order: veci,
    pub trail_lim: veci,
    pub model: veci,
    pub root_level: i32,
    pub simpdb_assigns: i32,
    pub simpdb_props: i32,
    pub random_seed: libc::c_double,
    pub progress_estimate: libc::c_double,
    pub verbosity: i32,
    pub stats: stats,
}
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
pub unsafe extern "C" fn glp_minisat1(mut P: *mut glp_prob) -> i32 {
    let mut current_block: u64;
    let mut s: *mut solver = 0 as *mut solver;
    let mut aij: *mut GLPAIJ = 0 as *mut GLPAIJ;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut len: i32 = 0;
    let mut ret: i32 = 0;
    let mut ind: *mut i32 = 0 as *mut i32;
    let mut sum: libc::c_double = 0.;
    if !((*P).tree).is_null() {
        (glp_error_(b"api/minisat1.c\0" as *const u8 as *const i8, 39 as i32))
            .expect(
                "non-null function pointer",
            )(b"glp_minisat1: operation not allowed\n\0" as *const u8 as *const i8);
    }
    (*P).mip_stat = 1 as i32;
    (*P).mip_obj = 0.0f64;
    if glp_check_cnfsat(P) != 0 as i32 {
        glp_printf(
            b"glp_minisat1: problem object does not encode CNF-SAT instance\n\0"
                as *const u8 as *const i8,
        );
        ret = 0x12 as i32;
    } else if ::core::mem::size_of::<*mut libc::c_void>() as u64
        != ::core::mem::size_of::<size_t>() as u64
    {
        glp_printf(
            b"glp_minisat1: sorry, MiniSat solver is not supported on this platform\n\0"
                as *const u8 as *const i8,
        );
        ret = 0x5 as i32;
    } else {
        glp_printf(b"Solving CNF-SAT problem...\n\0" as *const u8 as *const i8);
        glp_printf(
            b"Instance has %d variable%s, %d clause%s, and %d literal%s\n\0" as *const u8
                as *const i8,
            (*P).n,
            if (*P).n == 1 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"s\0" as *const u8 as *const i8
            },
            (*P).m,
            if (*P).m == 1 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"s\0" as *const u8 as *const i8
            },
            (*P).nnz,
            if (*P).nnz == 1 as i32 {
                b"\0" as *const u8 as *const i8
            } else {
                b"s\0" as *const u8 as *const i8
            },
        );
        if (*P).m == 0 as i32 {
            (*P).mip_stat = 5 as i32;
            j = 1 as i32;
            while j <= (*P).n {
                (**((*P).col).offset(j as isize)).mipx = 0.0f64;
                j += 1;
                j;
            }
        } else {
            i = 1 as i32;
            loop {
                if !(i <= (*P).m) {
                    current_block = 13797916685926291137;
                    break;
                }
                if ((**((*P).row).offset(i as isize)).ptr).is_null() {
                    (*P).mip_stat = 4 as i32;
                    current_block = 3801366316939345657;
                    break;
                } else {
                    i += 1;
                    i;
                }
            }
            match current_block {
                3801366316939345657 => {}
                _ => {
                    s = _glp_minisat_new();
                    _glp_minisat_setnvars(s, (*P).n);
                    ind = glp_alloc(
                        1 as i32 + (*P).n,
                        ::core::mem::size_of::<i32>() as u64 as i32,
                    ) as *mut i32;
                    i = 1 as i32;
                    loop {
                        if !(i <= (*P).m) {
                            current_block = 14434620278749266018;
                            break;
                        }
                        len = 0 as i32;
                        aij = (**((*P).row).offset(i as isize)).ptr;
                        while !aij.is_null() {
                            len += 1;
                            *ind.offset(len as isize) = (*(*aij).col).j - 1 as i32
                                + ((*(*aij).col).j - 1 as i32);
                            if (*aij).val < 0.0f64 {
                                *ind.offset(len as isize) = *ind.offset(len as isize)
                                    ^ 1 as i32;
                            }
                            aij = (*aij).r_next;
                        }
                        (len > 0 as i32
                            || {
                                glp_assert_(
                                    b"len > 0\0" as *const u8 as *const i8,
                                    b"api/minisat1.c\0" as *const u8 as *const i8,
                                    97 as i32,
                                );
                                1 as i32 != 0
                            }) as i32;
                        if _glp_minisat_addclause(
                            s,
                            &mut *ind.offset(1 as i32 as isize),
                            &mut *ind.offset((1 as i32 + len) as isize),
                        ) == 0
                        {
                            glp_free(ind as *mut libc::c_void);
                            _glp_minisat_delete(s);
                            (*P).mip_stat = 4 as i32;
                            current_block = 3801366316939345657;
                            break;
                        } else {
                            i += 1;
                            i;
                        }
                    }
                    match current_block {
                        3801366316939345657 => {}
                        _ => {
                            glp_free(ind as *mut libc::c_void);
                            (*s).verbosity = 1 as i32;
                            if _glp_minisat_solve(s, 0 as *mut lit, 0 as *mut lit) != 0 {
                                (*P).mip_stat = 5 as i32;
                                ((*s).model.size == (*P).n
                                    || {
                                        glp_assert_(
                                            b"s->model.size == P->n\0" as *const u8 as *const i8,
                                            b"api/minisat1.c\0" as *const u8 as *const i8,
                                            117 as i32,
                                        );
                                        1 as i32 != 0
                                    }) as i32;
                                j = 1 as i32;
                                while j <= (*P).n {
                                    (**((*P).col).offset(j as isize)).mipx = if *((*s)
                                        .model
                                        .ptr)
                                        .offset((j - 1 as i32) as isize) == 1 as i32
                                    {
                                        1.0f64
                                    } else {
                                        0.0f64
                                    };
                                    j += 1;
                                    j;
                                }
                                i = 1 as i32;
                                while i <= (*P).m {
                                    sum = 0 as i32 as libc::c_double;
                                    aij = (**((*P).row).offset(i as isize)).ptr;
                                    while !aij.is_null() {
                                        sum += (*aij).val * (*(*aij).col).mipx;
                                        aij = (*aij).r_next;
                                    }
                                    (**((*P).row).offset(i as isize)).mipx = sum;
                                    i += 1;
                                    i;
                                }
                                i = 1 as i32;
                                while i <= (*P).m {
                                    if (**((*P).row).offset(i as isize)).mipx
                                        < (**((*P).row).offset(i as isize)).lb
                                    {
                                        (*P).mip_stat = 1 as i32;
                                        break;
                                    } else {
                                        i += 1;
                                        i;
                                    }
                                }
                            } else {
                                (*P).mip_stat = 4 as i32;
                            }
                            _glp_minisat_delete(s);
                        }
                    }
                }
            }
        }
        if (*P).mip_stat == 5 as i32 {
            glp_printf(b"SATISFIABLE\n\0" as *const u8 as *const i8);
            ret = 0 as i32;
        } else if (*P).mip_stat == 4 as i32 {
            glp_printf(b"UNSATISFIABLE\n\0" as *const u8 as *const i8);
            ret = 0 as i32;
        } else {
            glp_printf(b"glp_minisat1: solver failed\n\0" as *const u8 as *const i8);
            ret = 0x5 as i32;
        }
    }
    return ret;
}