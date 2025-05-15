use libc::{c_long, c_int, c_double};

pub type TimeT = c_long;
pub type SyscallSLongT = c_long;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Timespec {
    pub tv_sec: TimeT,
    pub tv_nsec: SyscallSLongT,
}

impl Timespec {
    pub fn new(s: TimeT, ns: SyscallSLongT) -> Self {
        Timespec {
            tv_sec: s,
            tv_nsec: ns,
        }
    }

    pub fn cmp(&self, other: &Timespec) -> c_int {
        2 * ((self.tv_sec > other.tv_sec) as c_int - (self.tv_sec < other.tv_sec) as c_int)
            + ((self.tv_nsec > other.tv_nsec) as c_int - (self.tv_nsec < other.tv_nsec) as c_int)
    }

    pub fn sign(&self) -> c_int {
        (self.tv_sec > 0) as c_int - (self.tv_sec < 0) as c_int
            + ((self.tv_sec == 0) as c_int & (self.tv_nsec != 0) as c_int)
    }

    pub fn to_seconds(&self) -> c_double {
        self.tv_sec as c_double + self.tv_nsec as c_double / 1e9f64
    }
}