use std::fmt;
use std::ptr;
use std::mem;
use std::cmp;
use std::f64;
use std::slice;

#[derive(Debug, Clone, Copy)]
pub struct GslRngType {
    pub name: &'static str,
    pub max: u64,
    pub min: u64,
    pub size: usize,
    pub set: Option<fn(*mut u8, u64)>,
    pub get: Option<fn(*mut u8) -> u64>,
    pub get_double: Option<fn(*mut u8) -> f64>,
}

#[derive(Debug, Clone, Copy)]
pub struct GslRng {
    pub type_: &'static GslRngType,
    pub state: *mut u8,
}

pub type GslSimanEfuncT = fn(*mut u8) -> f64;
pub type GslSimanStepT = fn(&GslRng, *mut u8, f64);
pub type GslSimanMetricT = fn(*mut u8, *mut u8) -> f64;
pub type GslSimanPrintT = fn(*mut u8);
pub type GslSimanCopyT = fn(*mut u8, *mut u8);
pub type GslSimanCopyConstructT = fn(*mut u8) -> *mut u8;
pub type GslSimanDestroyT = fn(*mut u8);

#[derive(Debug, Clone, Copy)]
pub struct GslSimanParamsT {
    pub n_tries: i32,
    pub iters_fixed_T: i32,
    pub step_size: f64,
    pub k: f64,
    pub t_initial: f64,
    pub mu_t: f64,
    pub t_min: f64,
}

fn gsl_rng_uniform(r: &GslRng) -> f64 {
    (r.type_.get_double.unwrap())(r.state)
}

fn boltzmann(E: f64, new_E: f64, T: f64, params: &GslSimanParamsT) -> f64 {
    let x = -(new_E - E) / (params.k * T);
    if x < -7.0839641853226408e+02 {
        0.0
    } else {
        x.exp()
    }
}

fn copy_state(src: *mut u8, dst: *mut u8, size: usize, copyfunc: Option<GslSimanCopyT>) {
    if let Some(copy) = copyfunc {
        copy(src, dst);
    } else {
        unsafe {
            ptr::copy_nonoverlapping(src, dst, size);
        }
    }
}

pub fn gsl_siman_solve(
    r: &GslRng,
    x0_p: *mut u8,
    Ef: GslSimanEfuncT,
    take_step: GslSimanStepT,
    distance: Option<GslSimanMetricT>,
    print_position: Option<GslSimanPrintT>,
    copyfunc: Option<GslSimanCopyT>,
    copy_constructor: Option<GslSimanCopyConstructT>,
    destructor: Option<GslSimanDestroyT>,
    element_size: usize,
    params: GslSimanParamsT,
) {
    assert!(
        (copyfunc.is_some() && copy_constructor.is_some() && destructor.is_some())
            || element_size != 0
    );

    let mut x: *mut u8 = ptr::null_mut();
    let mut new_x: *mut u8 = ptr::null_mut();
    let mut best_x: *mut u8 = ptr::null_mut();
    let mut E = 0.0;
    let mut new_E = 0.0;
    let mut best_E = 0.0;
    let mut n_evals = 1;
    let mut n_iter = 0;
    let mut n_accepts = 0;
    let mut n_rejects = 0;
    let mut n_eless = 0;

    E = Ef(x0_p);

    if let Some(copy) = copyfunc {
        x = copy_constructor.unwrap()(x0_p);
        new_x = copy_constructor.unwrap()(x0_p);
        best_x = copy_constructor.unwrap()(x0_p);
    } else {
        x = unsafe { libc::malloc(element_size) } as *mut u8;
        unsafe { ptr::copy_nonoverlapping(x0_p, x, element_size) };
        new_x = unsafe { libc::malloc(element_size) } as *mut u8;
        best_x = unsafe { libc::malloc(element_size) } as *mut u8;
        unsafe { ptr::copy_nonoverlapping(x0_p, best_x, element_size) };
    }

    best_E = E;
    let mut T = params.t_initial;
    let T_factor = 1.0 / params.mu_t;

    if let Some(print) = print_position {
        println!("#-iter  #-evals   temperature     position   energy");
    }

    loop {
        n_accepts = 0;
        n_rejects = 0;
        n_eless = 0;

        for _ in 0..params.iters_fixed_T {
            copy_state(x, new_x, element_size, copyfunc);
            take_step(r, new_x, params.step_size);
            new_E = Ef(new_x);

            if new_E <= best_E {
                copy_state(new_x, best_x, element_size, copyfunc);
                best_E = new_E;
            }

            n_evals += 1;

            if new_E < E {
                if new_E < best_E {
                    copy_state(new_x, best_x, element_size, copyfunc);
                    best_E = new_E;
                }
                copy_state(new_x, x, element_size, copyfunc);
                E = new_E;
                n_eless += 1;
            } else if gsl_rng_uniform(r) < boltzmann(E, new_E, T, &params) {
                copy_state(new_x, x, element_size, copyfunc);
                E = new_E;
                n_accepts += 1;
            } else {
                n_rejects += 1;
            }
        }

        if let Some(print) = print_position {
            println!("{:5}   {:7}  {:12}", n_iter, n_evals, T);
            print(x);
            println!("  {:12}  {:12}", E, best_E);
        }

        T *= T_factor;
        n_iter += 1;

        if T < params.t_min {
            break;
        }
    }

    copy_state(best_x, x0_p, element_size, copyfunc);

    if let Some(destroy) = destructor {
        destroy(x);
        destroy(new_x);
        destroy(best_x);
    } else {
        unsafe {
            libc::free(x as *mut libc::c_void);
            libc::free(new_x as *mut libc::c_void);
            libc::free(best_x as *mut libc::c_void);
        }
    }
}

pub fn gsl_siman_solve_many(
    r: &GslRng,
    x0_p: *mut u8,
    Ef: GslSimanEfuncT,
    take_step: GslSimanStepT,
    distance: Option<GslSimanMetricT>,
    print_position: Option<GslSimanPrintT>,
    element_size: usize,
    params: GslSimanParamsT,
) {
    if let Some(print) = print_position {
        println!("#-iter    temperature       position         delta_pos        energy");
    }

    let mut x = unsafe { libc::malloc((params.n_tries as usize) * element_size) } as *mut u8;
    let mut new_x = unsafe { libc::malloc((params.n_tries as usize) * element_size) } as *mut u8;
    let mut energies = unsafe {
        libc::malloc((params.n_tries as usize) * mem::size_of::<f64>())
    } as *mut f64;
    let mut probs = unsafe {
        libc::malloc((params.n_tries as usize) * mem::size_of::<f64>())
    } as *mut f64;
    let mut sum_probs = unsafe {
        libc::malloc((params.n_tries as usize) * mem::size_of::<f64>())
    } as *mut f64;

    let mut T = params.t_initial;
    let T_factor = 1.0 / params.mu_t;

    unsafe {
        ptr::copy_nonoverlapping(x0_p, x, element_size);
    }

    let mut n_iter = 0;

    loop {
        let Ex = Ef(x);

        for i in 0..(params.n_tries - 1) {
            unsafe {
                *sum_probs.offset(i as isize) = 0.0;
                ptr::copy_nonoverlapping(
                    x,
                    new_x.offset((i as usize * element_size) as isize),
                    element_size,
                );
                take_step(
                    r,
                    new_x.offset((i as usize * element_size) as isize),
                    params.step_size,
                );
                *energies.offset(i as isize) = Ef(new_x.offset((i as usize * element_size) as isize));
                *probs.offset(i as isize) = boltzmann(Ex, *energies.offset(i as isize), T, &params);
            }
        }

        unsafe {
            ptr::copy_nonoverlapping(
                x,
                new_x.offset(((params.n_tries - 1) as usize * element_size) as isize),
                element_size,
            );
            *energies.offset((params.n_tries - 1) as isize) = Ex;
            *probs.offset((params.n_tries - 1) as isize) = boltzmann(Ex, Ex, T, &params);

            *sum_probs.offset(0) = *probs.offset(0);
            for i in 1..params.n_tries {
                *sum_probs.offset(i as isize) =
                    *sum_probs.offset((i - 1) as isize) + *probs.offset(i as isize);
            }

            let u = gsl_rng_uniform(r) * *sum_probs.offset((params.n_tries - 1) as isize);

            for i in 0..params.n_tries {
                if u < *sum_probs.offset(i as isize) {
                    ptr::copy_nonoverlapping(
                        new_x.offset((i as usize * element_size) as isize),
                        x,
                        element_size,
                    );
                    break;
                }
            }
        }

        if let Some(print) = print_position {
            print!("{:5}\t{:12}\t", n_iter, T);
            print(x);
            println!(
                "\t{:12}\t{:12}",
                distance.unwrap()(x, x0_p),
                Ex
            );
        }

        T *= T_factor;
        n_iter += 1;

        if T < params.t_min {
            break;
        }
    }

    unsafe {
        ptr::copy_nonoverlapping(x, x0_p, element_size);
        libc::free(x as *mut libc::c_void);
        libc::free(new_x as *mut libc::c_void);
        libc::free(energies as *mut libc::c_void);
        libc::free(probs as *mut libc::c_void);
        libc::free(sum_probs as *mut libc::c_void);
    }
}