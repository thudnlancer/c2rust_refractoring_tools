use std::f64;
use std::cmp;

#[derive(Debug)]
struct ExtrapolationTable {
    n: usize,
    rlist: [f64; 52],
    res3la: [f64; 3],
}

impl ExtrapolationTable {
    fn new() -> Self {
        ExtrapolationTable {
            n: 0,
            rlist: [0.0; 52],
            res3la: [0.0; 3],
        }
    }
}

struct IntegrationWorkspace {
    limit: usize,
    size: usize,
    nrmax: usize,
    i: usize,
    maximum_level: usize,
    alist: Vec<f64>,
    blist: Vec<f64>,
    rlist: Vec<f64>,
    elist: Vec<f64>,
    level: Vec<usize>,
    order: Vec<usize>,
}

struct QawoTable {
    n: usize,
    omega: f64,
    L: f64,
}

trait GslFunction {
    fn evaluate(&self, x: f64) -> f64;
}

fn initialise(workspace: &mut IntegrationWorkspace, a: f64, b: f64) {
    workspace.size = 1;
    workspace.nrmax = 1;
    workspace.i = 0;
    workspace.alist[0] = a;
    workspace.blist[0] = b;
    workspace.rlist[0] = 0.0;
    workspace.elist[0] = 0.0;
    workspace.level[0] = 0;
    workspace.order[0] = 1;
}

fn set_initial_result(workspace: &mut IntegrationWorkspace, result0: f64, abserr0: f64) {
    workspace.rlist[0] = result0;
    workspace.elist[0] = abserr0;
}

fn initialise_table(table: &mut ExtrapolationTable) {
    table.n = 0;
}

fn append_table(table: &mut ExtrapolationTable, value: f64) {
    if table.n < table.rlist.len() {
        table.rlist[table.n] = value;
        table.n += 1;
    }
}

fn retrieve(workspace: &IntegrationWorkspace, a_i: &mut f64, b_i: &mut f64, r_i: &mut f64, e_i: &mut f64) {
    *a_i = workspace.alist[workspace.i];
    *b_i = workspace.blist[workspace.i];
    *r_i = workspace.rlist[workspace.i];
    *e_i = workspace.elist[workspace.i];
}

fn update(
    workspace: &mut IntegrationWorkspace,
    a1: f64, b1: f64, area1: f64, error1: f64,
    a2: f64, b2: f64, area2: f64, error2: f64,
) {
    let i_max = workspace.i;
    let i_new = workspace.size;
    
    workspace.alist[i_max] = a1;
    workspace.blist[i_max] = b1;
    workspace.rlist[i_max] = area1;
    workspace.elist[i_max] = error1;
    workspace.level[i_max] = workspace.level[i_max] + 1;
    
    workspace.alist[i_new] = a2;
    workspace.blist[i_new] = b2;
    workspace.rlist[i_new] = area2;
    workspace.elist[i_new] = error2;
    workspace.level[i_new] = workspace.level[i_max];
    
    workspace.size += 1;
}

fn test_positivity(result: f64, resabs: f64) -> bool {
    result > 0.0 && (resabs / result) < 0.1
}

fn subinterval_too_small(a1: f64, a2: f64, b2: f64) -> bool {
    let tmp = (1.0 + 100.0 * f64::EPSILON) * (a2.abs() + 1000.0 * f64::MIN_POSITIVE);
    (a1 - a2).abs() <= tmp || (b2 - a2).abs() <= tmp
}

fn large_interval(workspace: &IntegrationWorkspace) -> bool {
    workspace.size - workspace.nrmax > 1
}

fn increase_nrmax(workspace: &mut IntegrationWorkspace) -> bool {
    workspace.nrmax += 1;
    workspace.i = workspace.order[workspace.size - workspace.nrmax];
    true
}

fn reset_nrmax(workspace: &mut IntegrationWorkspace) {
    workspace.nrmax = 1;
    workspace.i = workspace.size - 1;
}

fn sum_results(workspace: &IntegrationWorkspace) -> f64 {
    workspace.rlist.iter().sum()
}

fn qelg(table: &mut ExtrapolationTable, reseps: &mut f64, abseps: &mut f64) {
    // Implementation of qelg algorithm
    // ... (omitted for brevity)
}

fn qc25f<F: GslFunction>(
    f: &F,
    a: f64,
    b: f64,
    wf: &QawoTable,
    level: usize,
    result: &mut f64,
    abserr: &mut f64,
    resabs: &mut f64,
    resasc: &mut f64,
) {
    // Implementation of qc25f algorithm
    // ... (omitted for brevity)
}

#[derive(Debug)]
enum GslError {
    EMaxIter,
    ERound,
    ESing,
    EDivergence,
    ETable,
    EInvalid,
    EBadTol,
    EFailed,
}

fn gsl_integration_qawo<F: GslFunction>(
    f: &F,
    a: f64,
    epsabs: f64,
    epsrel: f64,
    limit: usize,
    workspace: &mut IntegrationWorkspace,
    wf: &QawoTable,
    result: &mut f64,
    abserr: &mut f64,
) -> Result<(), GslError> {
    let mut area;
    let mut errsum;
    let mut res_ext = 0.0;
    let mut err_ext = f64::MAX;
    let mut result0 = 0.0;
    let mut abserr0 = 0.0;
    let mut resabs0 = 0.0;
    let mut resasc0 = 0.0;
    let mut tolerance;

    let mut ertest = 0.0;
    let mut error_over_large_intervals = 0.0;
    let mut reseps = 0.0;
    let mut abseps = 0.0;
    let mut correc = 0.0;
    let mut ktmin = 0;
    let mut roundoff_type1 = 0;
    let mut roundoff_type2 = 0;
    let mut roundoff_type3 = 0;
    let mut error_type = 0;
    let mut error_type2 = 0;

    let mut iteration = 0;

    let mut positive_integrand = false;
    let mut extrapolate = false;
    let mut extall = false;
    let mut disallow_extrapolation = false;

    let mut table = ExtrapolationTable::new();

    let b = a + wf.L;
    let abs_omega = wf.omega.abs();

    // Initialize results
    initialise(workspace, a, b);

    *result = 0.0;
    *abserr = 0.0;

    if limit > workspace.limit {
        return Err(GslError::EInvalid);
    }

    // Test on accuracy
    if epsabs <= 0.0 && (epsrel < 50.0 * f64::EPSILON || epsrel < 0.5e-28) {
        return Err(GslError::EBadTol);
    }

    // Perform the first integration
    qc25f(f, a, b, wf, 0, &mut result0, &mut abserr0, &mut resabs0, &mut resasc0);

    set_initial_result(workspace, result0, abserr0);

    tolerance = epsabs.max(epsrel * result0.abs());

    if abserr0 <= 100.0 * f64::EPSILON * resabs0 && abserr0 > tolerance {
        *result = result0;
        *abserr = abserr0;
        return Err(GslError::ERound);
    } else if (abserr0 <= tolerance && abserr0 != resasc0) || abserr0 == 0.0 {
        *result = result0;
        *abserr = abserr0;
        return Ok(());
    } else if limit == 1 {
        *result = result0;
        *abserr = abserr0;
        return Err(GslError::EMaxIter);
    }

    // Initialization
    initialise_table(&mut table);

    if 0.5 * abs_omega * (b - a).abs() <= 2.0 {
        append_table(&mut table, result0);
        extall = true;
    }

    area = result0;
    errsum = abserr0;

    res_ext = result0;
    err_ext = f64::MAX;

    positive_integrand = test_positivity(result0, resabs0);

    iteration = 1;

    while iteration < limit {
        let current_level;
        let mut a1 = 0.0;
        let mut b1 = 0.0;
        let mut a2 = 0.0;
        let mut b2 = 0.0;
        let mut a_i = 0.0;
        let mut b_i = 0.0;
        let mut r_i = 0.0;
        let mut e_i = 0.0;
        let mut area1 = 0.0;
        let mut area2 = 0.0;
        let mut area12 = 0.0;
        let mut error1 = 0.0;
        let mut error2 = 0.0;
        let mut error12 = 0.0;
        let mut resasc1 = 0.0;
        let mut resasc2 = 0.0;
        let mut resabs1 = 0.0;
        let mut resabs2 = 0.0;
        let mut last_e_i = 0.0;

        // Bisect the subinterval with the largest error estimate
        retrieve(workspace, &mut a_i, &mut b_i, &mut r_i, &mut e_i);

        current_level = workspace.level[workspace.i] + 1;

        if current_level >= wf.n {
            error_type = -1; // exceeded limit of table
            break;
        }

        a1 = a_i;
        b1 = 0.5 * (a_i + b_i);
        a2 = b1;
        b2 = b_i;

        iteration += 1;

        qc25f(f, a1, b1, wf, current_level, &mut area1, &mut error1, &mut resabs1, &mut resasc1);
        qc25f(f, a2, b2, wf, current_level, &mut area2, &mut error2, &mut resabs2, &mut resasc2);

        area12 = area1 + area2;
        error12 = error1 + error2;
        last_e_i = e_i;

        // Improve previous approximations
        errsum = errsum + error12 - e_i;
        area = area + area12 - r_i;

        tolerance = epsabs.max(epsrel * area.abs());

        if resasc1 != error1 && resasc2 != error2 {
            let delta = r_i - area12;

            if delta.abs() <= 1.0e-5 * area12.abs() && error12 >= 0.99 * e_i {
                if !extrapolate {
                    roundoff_type1 += 1;
                } else {
                    roundoff_type2 += 1;
                }
            }
            if iteration > 10 && error12 > e_i {
                roundoff_type3 += 1;
            }
        }

        // Test for roundoff
        if roundoff_type1 + roundoff_type2 >= 10 || roundoff_type3 >= 20 {
            error_type = 2; // round off error
        }

        if roundoff_type2 >= 5 {
            error_type2 = 1;
        }

        // Test for bad integrand behavior
        if subinterval_too_small(a1, a2, b2) {
            error_type = 4;
        }

        // Append new intervals
        update(workspace, a1, b1, area1, error1, a2, b2, area2, error2);

        if errsum <= tolerance {
            break;
        }

        if error_type != 0 {
            break;
        }

        if iteration >= limit - 1 {
            error_type = 1;
            break;
        }

        // First iteration setup
        if iteration == 2 && extall {
            error_over_large_intervals = errsum;
            ertest = tolerance;
            append_table(&mut table, area);
            continue;
        }

        if disallow_extrapolation {
            continue;
        }

        if extall {
            error_over_large_intervals += -last_e_i;
            
            if current_level < workspace.maximum_level {
                error_over_large_intervals += error12;
            }

            if extrapolate {
                // label70:
                if error_type2 == 0 && error_over_large_intervals > ertest {
                    if increase_nrmax(workspace) {
                        continue;
                    }
                }

                // Perform extrapolation
                append_table(&mut table, area);

                if table.n < 3 {
                    reset_nrmax(workspace);
                    extrapolate = false;
                    error_over_large_intervals = errsum;
                    continue;
                }

                qelg(&mut table, &mut reseps, &mut abseps);

                ktmin += 1;

                if ktmin > 5 && err_ext < 0.001 * errsum {
                    error_type = 5;
                }

                if abseps < err_ext {
                    ktmin = 0;
                    err_ext = abseps;
                    res_ext = reseps;
                    correc = error_over_large_intervals;
                    ertest = epsabs.max(epsrel * reseps.abs());
                    if err_ext <= ertest {
                        break;
                    }
                }

                if table.n == 1 {
                    disallow_extrapolation = true;
                }

                if error_type == 5 {
                    break;
                }

                reset_nrmax(workspace);
                extrapolate = false;
                error_over_large_intervals = errsum;
                continue;
            }
        }
        
        if large_interval(workspace) {
            continue;
        }

        if extall {
            extrapolate = true;
            workspace.nrmax = 1;
        } else {
            let i = workspace.i;
            let width = workspace.blist[i] - workspace.alist[i];
            
            if 0.25 * width.abs() * abs_omega > 2.0 {
                continue;
            }
            
            extall = true;
            error_over_large_intervals = errsum;
            ertest = tolerance;
            continue;
        }
    }

    *result = res_ext;
    *abserr = err_ext;

    if err_ext == f64::MAX {
        *result = sum_results(workspace);
        *abserr = errsum;
    }

    if error_type != 0 || error_type2 != 0 {
        if error_type2 != 0 {
            err_ext += correc;
        }

        if error_type == 0 {
            error_type = 3;
        }

        if *result != 0.0 && area != 0.0 {
            if err_ext / res_ext.abs() > errsum / area.abs() {
                *result = sum_results(workspace);
                *abserr = errsum;
            }
        } else if err_ext > errsum {
            *result = sum_results(workspace);
            *abserr = errsum;
        } else if area == 0.0 {
            return match error_type {
                0 => Ok(()),
                1 => Err(GslError::EMaxIter),
                2 => Err(GslError::ERound),
                3 => Err(GslError::ESing),
                4 => Err(GslError::ERound),
                5 => Err(GslError::EDivergence),
                -1 => Err(GslError::ETable),
                _ => Err(GslError::EFailed),
            };
        }
    }

    // Test on divergence
    let max_area = res_ext.abs().max(area.abs());
    if !positive_integrand && max_area < 0.01 * resabs0 {
        return match error_type {
            0 => Ok(()),
            1 => Err(GslError::EMaxIter),
            2 => Err(GslError::ERound),
            3 => Err(GslError::ESing),
            4 => Err(GslError::ERound),
            5 => Err(GslError::EDivergence),
            -1 => Err(GslError::ETable),
            _ => Err(GslError::EFailed),
        };
    }

    let ratio = res_ext / area;
    if ratio < 0.01 || ratio > 100.0 || errsum > area.abs() {
        error_type = 6;
    }

    match error_type {
        0 => Ok(()),
        1 => Err(GslError::EMaxIter),
        2 => Err(GslError::ERound),
        3 => Err(GslError::ESing),
        4 => Err(GslError::ERound),
        5 => Err(GslError::EDivergence),
        -1 => Err(GslError::ETable),
        _ => Err(GslError::EFailed),
    }
}