use std::cmp::Ordering;
use std::ptr;
use libc::{c_double, c_int, c_void, size_t};

type Bool = c_int;
type Lit = c_int;
type Lbool = c_int;

const LBOOL_TRUE: Lbool = 1;
const LBOOL_FALSE: Lbool = -1;
const LBOOL_UNDEF: Lbool = 0;

struct Veci {
    size: c_int,
    cap: c_int,
    ptr: *mut c_int,
}

struct Vecp {
    size: c_int,
    cap: c_int,
    ptr: *mut *mut c_void,
}

struct Clause {
    size_learnt: c_int,
    lits: [Lit; 1],
}

struct Stats {
    starts: c_double,
    decisions: c_double,
    propagations: c_double,
    inspects: c_double,
    conflicts: c_double,
    clauses: c_double,
    clauses_literals: c_double,
    learnts: c_double,
    learnts_literals: c_double,
    max_literals: c_double,
    tot_literals: c_double,
}

struct Solver {
    size: c_int,
    cap: c_int,
    qhead: c_int,
    qtail: c_int,
    clauses: Vecp,
    learnts: Vecp,
    var_inc: c_double,
    var_decay: c_double,
    cla_inc: c_float,
    cla_decay: c_float,
    wlists: *mut Vecp,
    activity: *mut c_double,
    assigns: *mut Lbool,
    orderpos: *mut c_int,
    reasons: *mut *mut Clause,
    levels: *mut c_int,
    trail: *mut Lit,
    binary: *mut Clause,
    tags: *mut Lbool,
    tagged: Veci,
    stack: Veci,
    order: Veci,
    trail_lim: Veci,
    model: Veci,
    root_level: c_int,
    simpdb_assigns: c_int,
    simpdb_props: c_int,
    random_seed: c_double,
    progress_estimate: c_double,
    verbosity: c_int,
    stats: Stats,
}

impl Solver {
    fn new() -> *mut Solver {
        unsafe {
            let s = ymalloc(std::mem::size_of::<Solver>() as c_int) as *mut Solver;
            ptr::write(
                s,
                Solver {
                    clauses: Vecp {
                        size: 0,
                        cap: 4,
                        ptr: ymalloc((std::mem::size_of::<*mut c_void>() * 4) as c_int)
                            as *mut *mut c_void,
                    },
                    learnts: Vecp {
                        size: 0,
                        cap: 4,
                        ptr: ymalloc((std::mem::size_of::<*mut c_void>() * 4) as c_int)
                            as *mut *mut c_void,
                    },
                    order: Veci {
                        size: 0,
                        cap: 4,
                        ptr: ymalloc((std::mem::size_of::<c_int>() * 4) as c_int) as *mut c_int,
                    },
                    trail_lim: Veci {
                        size: 0,
                        cap: 4,
                        ptr: ymalloc((std::mem::size_of::<c_int>() * 4) as c_int) as *mut c_int,
                    },
                    tagged: Veci {
                        size: 0,
                        cap: 4,
                        ptr: ymalloc((std::mem::size_of::<c_int>() * 4) as c_int) as *mut c_int,
                    },
                    stack: Veci {
                        size: 0,
                        cap: 4,
                        ptr: ymalloc((std::mem::size_of::<c_int>() * 4) as c_int) as *mut c_int,
                    },
                    model: Veci {
                        size: 0,
                        cap: 4,
                        ptr: ymalloc((std::mem::size_of::<c_int>() * 4) as c_int) as *mut c_int,
                    },
                    wlists: ptr::null_mut(),
                    activity: ptr::null_mut(),
                    assigns: ptr::null_mut(),
                    orderpos: ptr::null_mut(),
                    reasons: ptr::null_mut(),
                    levels: ptr::null_mut(),
                    trail: ptr::null_mut(),
                    tags: ptr::null_mut(),
                    binary: ymalloc(
                        (std::mem::size_of::<Clause>() + std::mem::size_of::<Lit>() * 2) as c_int,
                    ) as *mut Clause,
                    size: 0,
                    cap: 0,
                    qhead: 0,
                    qtail: 0,
                    cla_inc: 1.0,
                    cla_decay: 1.0,
                    var_inc: 1.0,
                    var_decay: 1.0,
                    root_level: 0,
                    simpdb_assigns: 0,
                    simpdb_props: 0,
                    random_seed: 91648253.0,
                    progress_estimate: 0.0,
                    verbosity: 0,
                    stats: Stats {
                        starts: 0.0,
                        decisions: 0.0,
                        propagations: 0.0,
                        inspects: 0.0,
                        conflicts: 0.0,
                        clauses: 0.0,
                        clauses_literals: 0.0,
                        learnts: 0.0,
                        learnts_literals: 0.0,
                        max_literals: 0.0,
                        tot_literals: 0.0,
                    },
                },
            );
            (*s).binary.size_learnt = (2 << 1);
            s
        }
    }

    fn delete(s: *mut Solver) {
        unsafe {
            for i in 0..(*s).clauses.size {
                yfree(*(*s).clauses.ptr.offset(i as isize));
            }
            for i in 0..(*s).learnts.size {
                yfree(*(*s).learnts.ptr.offset(i as isize));
            }
            yfree((*s).clauses.ptr as *mut c_void);
            yfree((*s).learnts.ptr as *mut c_void);
            yfree((*s).order.ptr as *mut c_void);
            yfree((*s).trail_lim.ptr as *mut c_void);
            yfree((*s).tagged.ptr as *mut c_void);
            yfree((*s).stack.ptr as *mut c_void);
            yfree((*s).model.ptr as *mut c_void);
            yfree((*s).binary as *mut c_void);
            if !(*s).wlists.is_null() {
                for i in 0..(*s).size * 2 {
                    yfree((*(*s).wlists.offset(i as isize)).ptr as *mut c_void);
                }
                yfree((*s).wlists as *mut c_void);
                yfree((*s).activity as *mut c_void);
                yfree((*s).assigns as *mut c_void);
                yfree((*s).orderpos as *mut c_void);
                yfree((*s).reasons as *mut c_void);
                yfree((*s).levels as *mut c_void);
                yfree((*s).trail as *mut c_void);
                yfree((*s).tags as *mut c_void);
            }
            yfree(s as *mut c_void);
        }
    }
}

unsafe fn ymalloc(size: c_int) -> *mut c_void {
    assert!(size > 0);
    let ptr = malloc(size as size_t);
    if ptr.is_null() {
        panic!("MiniSat: no memory available");
    }
    ptr
}

unsafe fn yrealloc(ptr: *mut c_void, size: c_int) -> *mut c_void {
    assert!(size > 0);
    let new_ptr = if ptr.is_null() {
        malloc(size as size_t)
    } else {
        realloc(ptr, size as size_t)
    };
    if new_ptr.is_null() {
        panic!("MiniSat: no memory available");
    }
    new_ptr
}

unsafe fn yfree(ptr: *mut c_void) {
    assert!(!ptr.is_null());
    free(ptr);
}

unsafe fn drand(seed: *mut c_double) -> c_double {
    *seed *= 1389796.0;
    let q = (*seed / 2147483647.0) as c_int;
    *seed -= q as c_double * 2147483647.0;
    *seed / 2147483647.0
}

unsafe fn irand(seed: *mut c_double, size: c_int) -> c_int {
    (drand(seed) * size as c_double) as c_int
}

fn clause_cmp(a: *const c_void, b: *const c_void) -> c_int {
    unsafe {
        let a = a as *const Clause;
        let b = b as *const Clause;
        if (*a).size_learnt >> 1 > 2
            && ((*b).size_learnt >> 1 == 2
                || *((*a).lits.as_ptr().offset(((*a).size_learnt >> 1) as isize)
                    as *const c_float)
                    < *((*b).lits.as_ptr().offset(((*b).size_learnt >> 1) as isize)
                        as *const c_float))
        {
            -1
        } else {
            1
        }
    }
}

// Additional helper functions would be implemented here following the same patterns