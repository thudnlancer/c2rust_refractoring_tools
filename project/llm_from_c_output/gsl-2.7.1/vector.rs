fn main() {
    // Create a vector with 3 elements initialized to 0.0
    let mut v = vec![0.0; 3];
    
    // Set values in the vector
    for i in 0..3 {
        v[i] = 1.23 + i as f64;
    }
    
    // Attempt to access elements (with bounds checking)
    for i in 0..100 {
        match v.get(i) {
            Some(val) => println!("v_{} = {}", i, val),
            None => println!("Error: Index {} out of bounds for vector of length {}", i, v.len()),
        }
    }
    
    // Vector is automatically deallocated when it goes out of scope
}