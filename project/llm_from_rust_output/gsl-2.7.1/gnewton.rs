use gsl::{
    enums::GslError,
    matrix::Matrix,
    permutation::Permutation,
    vector::Vector,
    multiroot::{MultirootFunctionFdf, MultirootFdfsolverType},
};

#[derive(Debug)]
struct GnewtonState {
    phi: f64,
    x_trial: Vector,
    d: Vector,
    lu: Matrix,
    permutation: Permutation,
}

impl GnewtonState {
    fn new(n: usize) -> Result<Self, GslError> {
        let lu = Matrix::new(n, n).map_err(|_| GslError::NoMem)?;
        let permutation = Permutation::new(n).map_err(|_| GslError::NoMem)?;
        let d = Vector::new(n).map_err(|_| GslError::NoMem)?;
        let x_trial = Vector::new(n).map_err(|_| GslError::NoMem)?;

        Ok(Self {
            phi: 0.0,
            x_trial,
            d,
            lu,
            permutation,
        })
    }
}

fn enorm(f: &Vector) -> f64 {
    f.iter().map(|&x| x * x).sum::<f64>().sqrt()
}

fn gnewton_alloc(n: usize) -> Result<GnewtonState, GslError> {
    GnewtonState::new(n)
}

fn gnewton_set(
    state: &mut GnewtonState,
    fdf: &mut MultirootFunctionFdf,
    x: &mut Vector,
    f: &mut Vector,
    j: &mut Matrix,
    dx: &mut Vector,
) -> Result<(), GslError> {
    fdf.fdf(x, f, j)?;
    dx.set_zero();
    state.phi = enorm(f);
    Ok(())
}

fn gnewton_iterate(
    state: &mut GnewtonState,
    fdf: &mut MultirootFunctionFdf,
    x: &mut Vector,
    f: &mut Vector,
    j: &mut Matrix,
    dx: &mut Vector,
) -> Result<(), GslError> {
    state.lu.copy_from(j)?;
    let signum = state.lu.decomp(&mut state.permutation)?;
    
    state.lu.solve(&state.permutation, f, &mut state.d)?;

    let mut t = 1.0;
    let phi0 = state.phi;

    loop {
        for i in 0..x.len() {
            let di = state.d.get(i);
            let xi = x.get(i);
            state.x_trial.set(i, xi - t * di);
        }

        fdf.f(&state.x_trial, f)?;
        let phi1 = enorm(f);

        if !(phi1 > phi0 && t > f64::EPSILON) {
            break;
        }

        let theta = phi1 / phi0;
        let u = (f64::sqrt(1.0 + 6.0 * theta) - 1.0) / (3.0 * theta);
        t *= u;
    }

    x.copy_from(&state.x_trial)?;

    for i in 0..dx.len() {
        let di = state.d.get(i);
        dx.set(i, -t * di);
    }

    fdf.df(x, j)?;
    state.phi = enorm(f);
    Ok(())
}

static GNEWTON_TYPE: MultirootFdfsolverType = MultirootFdfsolverType {
    name: "gnewton",
    size: std::mem::size_of::<GnewtonState>(),
    alloc: gnewton_alloc,
    set: gnewton_set,
    iterate: gnewton_iterate,
};

pub static GSL_MULTIROOT_FDFSOLVER_GNEWTON: &MultirootFdfsolverType = &GNEWTON_TYPE;