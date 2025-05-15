use gsl::deriv::{central, forward};
use gsl::Function;
use std::f64::consts::SQRT_2;

fn f(x: f64, _params: &mut ()) -> f64 {
    x.powf(1.5)
}

fn main() {
    let mut params = ();
    let mut result = 0.0;
    let mut abserr = 0.0;

    let mut F = Function::new(&f, &mut params);

    println!("f(x) = x^(3/2)");

    central(&mut F, 2.0, 1e-8, &mut result, &mut abserr).unwrap();
    println!("x = 2.0");
    println!("f'(x) = {:.10} +/- {:.10}", result, abserr);
    println!("exact = {:.10}\n", 1.5 * SQRT_2);

    forward(&mut F, 0.0, 1e-8, &mut result, &mut abserr).unwrap();
    println!("x = 0.0");
    println!("f'(x) = {:.10} +/- {:.10}", result, abserr);
    println!("exact = {:.10}", 0.0);
}