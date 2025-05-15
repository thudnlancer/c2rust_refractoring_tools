#[derive(Copy, Clone, Debug)]
pub struct Floc {
    pub filenm: Option<String>,
    pub lineno: u64,
    pub offset: u64,
}

pub fn guile_gmake_setup(floc: Option<&Floc>) -> i32 {
    1
}