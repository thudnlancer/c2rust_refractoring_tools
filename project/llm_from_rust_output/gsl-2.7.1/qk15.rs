use std::os::raw::{c_double, c_int, c_void};

pub struct GslFunction {
    pub function: Option<fn(c_double, &mut c_void) -> c_double>,
    pub params: Box<c_void>,
}

const XGK: [c_double; 8] = [
    0.991455371120812639206854697526329,
    0.949107912342758524526189684047851,
    0.864864423359769072789712788640926,
    0.741531185599394439863864773280788,
    0.586087235467691130294144838258730,
    0.405845151377397166906606412076961,
    0.207784955007898467600689403773245,
    0.000000000000000000000000000000000,
];

const WG: [c_double; 4] = [
    0.129484966168869693270611432679082,
    0.279705391489276667901467771423780,
    0.381830050505118944950369775488975,
    0.417959183673469387755102040816327,
];

const WGK: [c_double; 8] = [
    0.022935322010529224963732008058970,
    0.063092092629978553290700663189204,
    0.104790010322250183839876322541518,
    0.140653259715525918745189590510238,
    0.169004726639267902826583426598550,
    0.190350578064785409913256402421014,
    0.204432940075298892414161999234649,
    0.209482141084727828012999174891714,
];

extern "C" {
    fn gsl_integration_qk(
        n: c_int,
        xgk: *const c_double,
        wg: *const c_double,
        wgk: *const c_double,
        fv1: *mut c_double,
        fv2: *mut c_double,
        f: *const GslFunction,
        a: c_double,
        b: c_double,
        result: *mut c_double,
        abserr: *mut c_double,
        resabs: *mut c_double,
        resasc: *mut c_double,
    );
}

pub fn gsl_integration_qk15(
    f: &GslFunction,
    a: c_double,
    b: c_double,
) -> (c_double, c_double, c_double, c_double) {
    let mut fv1 = [0.0; 8];
    let mut fv2 = [0.0; 8];
    let mut result = 0.0;
    let mut abserr = 0.0;
    let mut resabs = 0.0;
    let mut resasc = 0.0;

    unsafe {
        gsl_integration_qk(
            8,
            XGK.as_ptr(),
            WG.as_ptr(),
            WGK.as_ptr(),
            fv1.as_mut_ptr(),
            fv2.as_mut_ptr(),
            f as *const _,
            a,
            b,
            &mut result,
            &mut abserr,
            &mut resabs,
            &mut resasc,
        );
    }

    (result, abserr, resabs, resasc)
}