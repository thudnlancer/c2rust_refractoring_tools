use ndarray::{Array2, Axis};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Create a 10x3 matrix initialized with zeros
    let mut m = Array2::<f64>::zeros((10, 3));

    // Fill the matrix with values
    for i in 0..10 {
        for j in 0..3 {
            m[[i, j]] = 0.23 + 100.0 * (i as f64) + (j as f64);
        }
    }

    // Attempt to print values - will safely handle out of bounds access
    for i in 0..100 {
        for j in 0..3 {
            match m.get((i, j)) {
                Some(val) => println!("m({},{}) = {}", i, j, val),
                None => println!("m({},{}) is out of bounds", i, j),
            }
        }
    }

    Ok(())
}