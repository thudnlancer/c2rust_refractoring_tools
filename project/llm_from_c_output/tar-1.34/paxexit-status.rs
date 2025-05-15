// Assuming PAXEXIT_SUCCESS is defined as 0 in the original C code
pub const PAXEXIT_SUCCESS: i32 = 0;

// Using Rust's thread-safe global variable with lazy_static
lazy_static::lazy_static! {
    pub static ref EXIT_STATUS: std::sync::Mutex<i32> = std::sync::Mutex::new(PAXEXIT_SUCCESS);
}