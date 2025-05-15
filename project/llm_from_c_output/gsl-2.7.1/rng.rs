use rand::{rngs::StdRng, SeedableRng, Rng};
use rand::distributions::Standard;

fn main() {
    // Initialize the random number generator with default seed
    let mut rng = StdRng::from_entropy();
    
    println!("generator type: {}", std::any::type_name::<StdRng>());
    println!("seed = {:?}", rng.gen::<u64>()); // Note: actual seed isn't directly accessible in StdRng
    
    // Generate first random value
    let first_value: u64 = rng.sample(Standard);
    println!("first value = {}", first_value);
    
    // No explicit free needed - Rust's ownership system handles cleanup
}