use libc::{c_double, c_int, c_uint, c_ulong, c_void};
use std::ptr;

pub type size_t = c_ulong;

pub enum GslError {
    Success = 0,
    Failure = -1,
    Continue = -2,
    Dom = 1,
    Range = 2,
    Fault = 3,
    Inval = 4,
    Failed = 5,
    Factor = 6,
    Sanity = 7,
    Nomem = 8,
    Badfunc = 9,
    Runaway = 10,
    Maxiter = 11,
    Zerodiv = 12,
    Badtol = 13,
    Tol = 14,
    Undrflw = 15,
    Ovrflw = 16,
    Loss = 17,
    Round = 18,
    Badlen = 19,
    Notsqr = 20,
    Sing = 21,
    Diverge = 22,
    Unsup = 23,
    Unimpl = 24,
    Cache = 25,
    Table = 26,
    Noprog = 27,
    Noprogj = 28,
    Tolf = 29,
    Tolx = 30,
    Tolg = 31,
    Eof = 32,
}

#[derive(Clone, Copy)]
pub struct GslBlock {
    pub size: size_t,
    pub data: *mut c_double,
}

#[derive(Clone, Copy)]
pub struct GslVector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

#[derive(Clone, Copy)]
pub struct GslVectorView {
    pub vector: GslVector,
}

#[derive(Clone, Copy)]
pub struct GslVectorConstView {
    pub vector: GslVector,
}

pub enum GslMovstatEndType {
    PadZero = 0,
    PadValue = 1,
    Truncate = 2,
}

pub struct GslMovstatAccum {
    pub size: Option<unsafe extern "C" fn(size_t) -> size_t>,
    pub init: Option<unsafe extern "C" fn(size_t, *mut c_void) -> c_int>,
    pub insert: Option<unsafe extern "C" fn(c_double, *mut c_void) -> c_int>,
    pub delete_oldest: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut c_void, *mut c_double, *const c_void) -> c_int>,
}

pub struct GslMovstatWorkspace {
    pub H: size_t,
    pub J: size_t,
    pub K: size_t,
    pub work: *mut c_double,
    pub state: *mut c_void,
    pub state_size: size_t,
}

pub enum GslFilterEndType {
    PadZero = 0,
    PadValue = 1,
    Truncate = 2,
}

pub struct GslFilterRmedianWorkspace {
    pub H: size_t,
    pub K: size_t,
    pub state: *mut c_void,
    pub window: *mut c_double,
    pub minmaxacc: *const GslMovstatAccum,
    pub movstat_workspace_p: *mut GslMovstatWorkspace,
}

struct RmedianState {
    minmax_acc: *const GslMovstatAccum,
    minmax_state: *mut c_void,
}

extern "C" {
    fn gsl_vector_subvector(
        v: *mut GslVector,
        i: size_t,
        n: size_t,
    ) -> GslVectorView;
    fn gsl_vector_const_subvector(
        v: *const GslVector,
        i: size_t,
        n: size_t,
    ) -> GslVectorConstView;
    fn gsl_error(
        reason: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
        gsl_errno: libc::c_int,
    );
    fn gsl_movstat_alloc_with_size(
        accum_state_size: size_t,
        H: size_t,
        J: size_t,
    ) -> *mut GslMovstatWorkspace;
    fn gsl_movstat_free(w: *mut GslMovstatWorkspace);
    fn gsl_movstat_apply_accum(
        endtype: GslMovstatEndType,
        x: *const GslVector,
        accum: *const GslMovstatAccum,
        accum_params: *mut c_void,
        y: *mut GslVector,
        z: *mut GslVector,
        w: *mut GslMovstatWorkspace,
    ) -> c_int;
    fn gsl_movstat_fill(
        endtype: GslMovstatEndType,
        x: *const GslVector,
        idx: size_t,
        H: size_t,
        J: size_t,
        window: *mut c_double,
    ) -> size_t;
    static gsl_movstat_accum_minmax: *const GslMovstatAccum;
    fn gsl_stats_median(
        sorted_data: *mut c_double,
        stride: size_t,
        n: size_t,
    ) -> c_double;
}

pub fn gsl_filter_rmedian_alloc(K: size_t) -> *mut GslFilterRmedianWorkspace {
    unsafe {
        let mut w = ptr::null_mut::<GslFilterRmedianWorkspace>();
        let mut state_size: size_t = 0;
        
        w = libc::calloc(
            1,
            std::mem::size_of::<GslFilterRmedianWorkspace>(),
        ) as *mut GslFilterRmedianWorkspace;
        
        if w.is_null() {
            gsl_error(
                b"failed to allocate space for workspace\0".as_ptr() as *const _,
                b"rmedian.c\0".as_ptr() as *const _,
                63,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }
        
        (*w).H = K / 2;
        (*w).K = 2 * (*w).H + 1;
        (*w).minmaxacc = gsl_movstat_accum_minmax;
        
        (*w).window = libc::malloc((*w).K * std::mem::size_of::<c_double>()) as *mut c_double;
        if (*w).window.is_null() {
            gsl_filter_rmedian_free(w);
            gsl_error(
                b"failed to allocate space for window\0".as_ptr() as *const _,
                b"rmedian.c\0".as_ptr() as *const _,
                74,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }
        
        state_size = rmedian_size((*w).H + 1);
        (*w).state = libc::malloc(state_size);
        if (*w).state.is_null() {
            gsl_filter_rmedian_free(w);
            gsl_error(
                b"failed to allocate space for min/max state\0".as_ptr() as *const _,
                b"rmedian.c\0".as_ptr() as *const _,
                83,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }
        
        (*w).movstat_workspace_p = gsl_movstat_alloc_with_size(
            state_size,
            0,
            (*w).H,
        );
        if (*w).movstat_workspace_p.is_null() {
            gsl_filter_rmedian_free(w);
            gsl_error(
                b"failed to allocate space for movstat workspace\0".as_ptr() as *const _,
                b"rmedian.c\0".as_ptr() as *const _,
                90,
                GslError::Nomem as c_int,
            );
            return ptr::null_mut();
        }
        
        w
    }
}

pub fn gsl_filter_rmedian_free(w: *mut GslFilterRmedianWorkspace) {
    unsafe {
        if !(*w).state.is_null() {
            libc::free((*w).state);
        }
        if !(*w).window.is_null() {
            libc::free((*w).window as *mut c_void);
        }
        if !(*w).movstat_workspace_p.is_null() {
            gsl_movstat_free((*w).movstat_workspace_p);
        }
        libc::free(w as *mut c_void);
    }
}

pub fn gsl_filter_rmedian(
    endtype: GslFilterEndType,
    x: *const GslVector,
    y: *mut GslVector,
    w: *mut GslFilterRmedianWorkspace,
) -> c_int {
    unsafe {
        if (*x).size != (*y).size {
            gsl_error(
                b"input and output vectors must have same length\0".as_ptr() as *const _,
                b"rmedian.c\0".as_ptr() as *const _,
                126,
                GslError::Badlen as c_int,
            );
            return GslError::Badlen as c_int;
        }
        
        let mut status = GslError::Success as c_int;
        let n = (*x).size;
        let H = (*w).H as c_int;
        let mut yprev = 0.0;
        
        let wsize = gsl_movstat_fill(
            endtype as GslMovstatEndType,
            x,
            0,
            H as size_t,
            H as size_t,
            (*w).window,
        ) as c_int;
        
        yprev = gsl_stats_median(
            (*w).window,
            1,
            wsize as size_t,
        );
        
        gsl_vector_set(y, 0, yprev);
        
        if (*x).size > 1 {
            let xv = gsl_vector_const_subvector(
                x,
                1,
                n - 1,
            );
            let mut yv = gsl_vector_subvector(
                y,
                1,
                n - 1,
            );
            
            status = gsl_movstat_apply_accum(
                endtype as GslMovstatEndType,
                &xv.vector,
                &rmedian_accum_type,
                &mut yprev as *mut c_double as *mut c_void,
                &mut yv.vector,
                ptr::null_mut(),
                (*w).movstat_workspace_p,
            );
        }
        
        status
    }
}

unsafe fn rmedian_size(n: size_t) -> size_t {
    let mut size = 0;
    let acc = gsl_movstat_accum_minmax;
    
    size += std::mem::size_of::<RmedianState>();
    size += ((*acc).size.unwrap())(n);
    
    size
}

unsafe fn rmedian_init(n: size_t, vstate: *mut c_void) -> c_int {
    let state = vstate as *mut RmedianState;
    (*state).minmax_acc = gsl_movstat_accum_minmax;
    (*state).minmax_state = (vstate as *mut u8).offset(std::mem::size_of::<RmedianState>() as isize) as *mut c_void;
    ((*(*state).minmax_acc).init.unwrap())(n, (*state).minmax_state);
    GslError::Success as c_int
}

unsafe fn rmedian_insert(x: c_double, vstate: *mut c_void) -> c_int {
    let state = vstate as *mut RmedianState;
    ((*(*state).minmax_acc).insert.unwrap())(x, (*state).minmax_state)
}

unsafe fn rmedian_delete(vstate: *mut c_void) -> c_int {
    let state = vstate as *mut RmedianState;
    ((*(*state).minmax_acc).delete_oldest.unwrap())((*state).minmax_state)
}

unsafe fn rmedian_get(
    params: *mut c_void,
    result: *mut c_double,
    vstate: *const c_void,
) -> c_int {
    let state = vstate as *const RmedianState;
    let yprev = params as *mut c_double;
    let mut y = 0.0;
    let mut xminmax = [0.0; 2];
    
    ((*(*state).minmax_acc).get.unwrap())(
        ptr::null_mut(),
        xminmax.as_mut_ptr(),
        (*state).minmax_state,
    );
    
    if *yprev <= xminmax[0] {
        y = xminmax[0];
    } else if *yprev <= xminmax[1] {
        y = *yprev;
    } else {
        y = xminmax[1];
    }
    
    *result = y;
    *yprev = y;
    
    GslError::Success as c_int
}

static rmedian_accum_type: GslMovstatAccum = GslMovstatAccum {
    size: Some(rmedian_size),
    init: Some(rmedian_init),
    insert: Some(rmedian_insert),
    delete_oldest: Some(rmedian_delete),
    get: Some(rmedian_get),
};

unsafe fn gsl_vector_set(v: *mut GslVector, i: size_t, x: c_double) {
    *((*v).data.add(i * (*v).stride)) = x;
}