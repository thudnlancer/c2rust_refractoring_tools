use std::num::ParseIntError;
use std::str::FromStr;
use std::u64::MAX as UINTMAX_MAX;

fn xstrtoumax(s: &str) -> Result<u64, ParseIntError> {
    u64::from_str(s)
}

const STRTOL_T_MINIMUM: u64 = 0;
const STRTOL_T_MAXIMUM: u64 = UINTMAX_MAX;