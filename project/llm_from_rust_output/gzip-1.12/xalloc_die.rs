use std::process;

static EXIT_FAILURE: i32 = 1;

pub fn xalloc_die() -> ! {
    eprintln!("memory exhausted");
    process::exit(EXIT_FAILURE);
}