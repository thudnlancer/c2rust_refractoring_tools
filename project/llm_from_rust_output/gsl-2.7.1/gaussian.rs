use libc::{c_double, c_int, c_uint, c_ulong, c_void};
use std::mem;
use std::ptr;

type size_t = c_ulong;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub enum GslMovstatEnd {
    PadZero = 0,
    PadValue = 1,
    Truncate = 2,
}

#[derive(Debug, Clone, Copy)]
pub enum GslFilterEnd {
    PadZero = 0,
    PadValue = 1,
    Truncate = 2,
}

#[derive(Debug)]
pub struct GslBlock {
    pub size: size_t,
    pub data: *mut c_double,
}

#[derive(Debug)]
pub struct GslVector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut c_double,
    pub block: *mut GslBlock,
    pub owner: c_int,
}

#[derive(Debug)]
pub struct GslVectorView {
    pub vector: GslVector,
}

#[derive(Debug)]
pub struct GslMovstatAccum {
    pub size: Option<unsafe extern "C" fn(size_t) -> size_t>,
    pub init: Option<unsafe extern "C" fn(size_t, *mut c_void) -> c_int>,
    pub insert: Option<unsafe extern "C" fn(c_double, *mut c_void) -> c_int>,
    pub delete_oldest: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut c_void, *mut c_double, *const c_void) -> c_int>,
}

#[derive(Debug)]
pub struct GslMovstatWorkspace {
    pub H: size_t,
    pub J: size_t,
    pub K: size_t,
    pub work: *mut c_double,
    pub state: *mut c_void,
    pub state_size: size_t,
}

#[derive(Debug)]
pub struct GslFilterGaussianWorkspace {
    pub K: size_t,
    pub kernel: *mut c_double,
    pub movstat_workspace_p: *mut GslMovstatWorkspace,
}

#[derive(Debug)]
struct Ringbuf {
    array: *mut c_double,
    head: c_int,
    tail: c_int,
    size: c_int,
}

#[derive(Debug)]
struct GaussianState {
    n: size_t,
    window: *mut c_double,
    rbuf: *mut Ringbuf,
}

impl GslFilterGaussianWorkspace {
    pub fn new(K: size_t) -> Option<Box<Self>> {
        let H = K / 2;
        let mut w = Box::new(Self {
            K: 2 * H + 1,
            kernel: ptr::null_mut(),
            movstat_workspace_p: ptr::null_mut(),
        });

        unsafe {
            w.kernel = libc::malloc(w.K * mem::size_of::<c_double>() as size_t) as *mut c_double;
            if w.kernel.is_null() {
                return None;
            }

            let state_size = gaussian_size(w.K);
            w.movstat_workspace_p = gsl_movstat_alloc_with_size(state_size, H, H);
            if w.movstat_workspace_p.is_null() {
                libc::free(w.kernel as *mut c_void);
                return None;
            }
        }

        Some(w)
    }

    pub fn free(&mut self) {
        unsafe {
            if !self.kernel.is_null() {
                libc::free(self.kernel as *mut c_void);
                self.kernel = ptr::null_mut();
            }
            if !self.movstat_workspace_p.is_null() {
                gsl_movstat_free(self.movstat_workspace_p);
                self.movstat_workspace_p = ptr::null_mut();
            }
        }
    }

    pub fn apply(
        &mut self,
        endtype: GslFilterEnd,
        alpha: c_double,
        order: size_t,
        x: &GslVector,
        y: &mut GslVector,
    ) -> Result<(), GslError> {
        if x.size != y.size {
            return Err(GslError::Badlen);
        }
        if alpha <= 0.0 {
            return Err(GslError::Dom);
        }

        let mut kernel_view = GslVectorView {
            vector: GslVector {
                size: self.K,
                stride: 1,
                data: self.kernel,
                block: ptr::null_mut(),
                owner: 0,
            },
        };

        unsafe {
            gsl_filter_gaussian_kernel(alpha, order, 1, &mut kernel_view.vector)?;
            gsl_movstat_apply_accum(
                endtype as c_uint,
                x,
                &GAUSSIAN_ACCUM_TYPE,
                self.kernel as *mut c_void,
                y,
                ptr::null_mut(),
                self.movstat_workspace_p,
            )
        }
    }
}

fn gsl_filter_gaussian_kernel(
    alpha: c_double,
    order: size_t,
    normalize: c_int,
    kernel: &mut GslVector,
) -> Result<(), GslError> {
    let N = kernel.size;
    if alpha <= 0.0 {
        return Err(GslError::Dom);
    }
    if order > 10 {
        return Err(GslError::Dom);
    }

    let half = 0.5 * (N as c_double - 1.0);
    let mut sum = 0.0;

    if N == 1 {
        unsafe {
            *kernel.data = if order == 0 { 1.0 } else { 0.0 };
        }
        return Ok(());
    }

    for i in 0..N {
        let xi = (i as c_double - half) / half;
        let yi = alpha * xi;
        let gi = (-0.5 * yi * yi).exp();
        unsafe {
            *kernel.data.offset(i as isize) = gi;
        }
        sum += gi;
    }

    if normalize != 0 {
        let scale = 1.0 / sum;
        unsafe {
            for i in 0..N {
                *kernel.data.offset(i as isize) *= scale;
            }
        }
    }

    if order > 0 {
        let beta = -0.5 * alpha * alpha;
        let mut q = [0.0; 11];
        q[0] = 1.0 / half.powi(order as i32);

        for k in 1..=order {
            let mut qm1 = q[0];
            q[0] = q[1];
            for i in 1..=k {
                let tmp = q[i];
                q[i] = (i as c_double + 1.0) * q[i + 1] + 2.0 * beta * qm1;
                qm1 = tmp;
            }
        }

        for i in 0..N {
            let xi = (i as c_double - half) / half;
            let qn = q[0];
            for j in 1..=order {
                qn = qn * xi + q[j];
            }
            unsafe {
                *kernel.data.offset(i as isize) *= qn;
            }
        }
    }

    Ok(())
}

static GAUSSIAN_ACCUM_TYPE: GslMovstatAccum = GslMovstatAccum {
    size: Some(gaussian_size),
    init: Some(gaussian_init),
    insert: Some(gaussian_insert),
    delete_oldest: Some(gaussian_delete),
    get: Some(gaussian_get),
};

unsafe extern "C" fn gaussian_size(n: size_t) -> size_t {
    mem::size_of::<GaussianState>() as size_t
        + n * mem::size_of::<c_double>() as size_t
        + ringbuf_size(n)
}

unsafe extern "C" fn gaussian_init(n: size_t, vstate: *mut c_void) -> c_int {
    let state = &mut *(vstate as *mut GaussianState);
    state.n = n;
    state.window = vstate.add(mem::size_of::<GaussianState>()) as *mut c_double;
    state.rbuf = state.window.add(n) as *mut Ringbuf;
    ringbuf_init(n, state.rbuf)
}

unsafe extern "C" fn gaussian_insert(x: c_double, vstate: *mut c_void) -> c_int {
    let state = &mut *(vstate as *mut GaussianState);
    ringbuf_insert(x, state.rbuf)
}

unsafe extern "C" fn gaussian_delete(vstate: *mut c_void) -> c_int {
    let state = &mut *(vstate as *mut GaussianState);
    if ringbuf_is_empty(state.rbuf) == 0 {
        ringbuf_pop_back(state.rbuf)
    } else {
        GslError::Success as c_int
    }
}

unsafe extern "C" fn gaussian_get(
    params: *mut c_void,
    result: *mut c_double,
    vstate: *const c_void,
) -> c_int {
    let state = &*(vstate as *const GaussianState);
    let kernel = params as *const c_double;
    let n = ringbuf_copy(state.window, state.rbuf);
    let mut sum = 0.0;

    for i in 0..n {
        sum += *state.window.add(i) * *kernel.add(n - i - 1);
    }

    *result = sum;
    GslError::Success as c_int
}

unsafe extern "C" fn ringbuf_size(n: size_t) -> size_t {
    mem::size_of::<Ringbuf>() as size_t + n * mem::size_of::<c_double>() as size_t
}

unsafe extern "C" fn ringbuf_init(n: size_t, b: *mut Ringbuf) -> c_int {
    (*b).array = b.add(1) as *mut c_double;
    (*b).head = -1;
    (*b).tail = 0;
    (*b).size = n as c_int;
    GslError::Success as c_int
}

unsafe extern "C" fn ringbuf_is_empty(b: *const Ringbuf) -> c_int {
    (b.head == -1) as c_int
}

unsafe extern "C" fn ringbuf_insert(x: c_double, b: *mut Ringbuf) -> c_int {
    if (*b).head == -1 {
        (*b).head = 0;
        (*b).tail = 0;
    } else if (*b).head == 0 {
        (*b).head = (*b).size - 1;
        if (*b).tail == (*b).head && (*b).size > 1 {
            (*b).tail -= 1;
        }
    } else {
        (*b).head -= 1;
        if (*b).tail == (*b).head {
            if (*b).tail == 0 {
                (*b).tail = (*b).size - 1;
            } else {
                (*b).tail -= 1;
            }
        }
    }
    *(*b).array.offset((*b).head as isize) = x;
    GslError::Success as c_int
}

unsafe extern "C" fn ringbuf_pop_back(b: *mut Ringbuf) -> c_int {
    if ringbuf_is_empty(b) != 0 || (*b).tail < 0 {
        GslError::Badlen as c_int
    } else {
        if (*b).head == (*b).tail {
            (*b).head = -1;
            (*b).tail = -1;
        } else if (*b).tail == 0 {
            (*b).tail = (*b).size - 1;
        } else {
            (*b).tail -= 1;
        }
        GslError::Success as c_int
    }
}

unsafe extern "C" fn ringbuf_copy(dest: *mut c_double, b: *const Ringbuf) -> size_t {
    if ringbuf_is_empty(b) != 0 || (*b).tail < 0 {
        0
    } else {
        let n = if (*b).head > (*b).tail {
            (*b).size - (*b).head + (*b).tail + 1
        } else {
            (*b).tail - (*b).head + 1
        };

        for i in 0..n {
            *dest.add((n - i - 1) as isize) =
                *(*b).array.offset((((*b).head + i) % (*b).size) as isize);
        }

        n as size_t
    }
}

unsafe extern "C" fn gsl_movstat_alloc_with_size(
    accum_state_size: size_t,
    H: size_t,
    J: size_t,
) -> *mut GslMovstatWorkspace {
    ptr::null_mut()
}

unsafe extern "C" fn gsl_movstat_free(_w: *mut GslMovstatWorkspace) {}

unsafe extern "C" fn gsl_movstat_apply_accum(
    _endtype: c_uint,
    _x: *const GslVector,
    _accum: *const GslMovstatAccum,
    _accum_params: *mut c_void,
    _y: *mut GslVector,
    _z: *mut GslVector,
    _w: *mut GslMovstatWorkspace,
) -> c_int {
    GslError::Success as c_int
}