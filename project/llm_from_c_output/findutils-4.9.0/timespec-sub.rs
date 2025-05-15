//! Subtract two struct timespec values.
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation, either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//! Written by Paul Eggert.
//!
//! Return the difference between two timespec values A and B. On
//! overflow, return an extremal value. This assumes 0 <= tv_nsec <
//! TIMESPEC_HZ.

use std::time::Duration;

const TIMESPEC_HZ: i32 = 1_000_000_000;

#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i32,
}

impl Timespec {
    pub fn sub(self, other: Self) -> Self {
        let mut rs = self.tv_sec;
        let mut bs = other.tv_sec;
        let ns = self.tv_nsec - other.tv_nsec;
        let mut rns = ns;

        if ns < 0 {
            rns = ns + TIMESPEC_HZ;
            if let Some(new_bs) = bs.checked_add(1) {
                bs = new_bs;
            } else if rs > i64::MIN {
                rs -= 1;
            } else {
                return Self::low_overflow();
            }
        }

        if let Some(new_rs) = rs.checked_sub(bs) {
            rs = new_rs;
        } else {
            if bs > 0 {
                return Self::low_overflow();
            } else {
                return Self::high_overflow();
            }
        }

        Self { tv_sec: rs, tv_nsec: rns }
    }

    fn low_overflow() -> Self {
        Self {
            tv_sec: i64::MIN,
            tv_nsec: 0,
        }
    }

    fn high_overflow() -> Self {
        Self {
            tv_sec: i64::MAX,
            tv_nsec: TIMESPEC_HZ - 1,
        }
    }
}

impl From<Duration> for Timespec {
    fn from(d: Duration) -> Self {
        Self {
            tv_sec: d.as_secs() as i64,
            tv_nsec: d.subsec_nanos() as i32,
        }
    }
}

impl From<Timespec> for Duration {
    fn from(t: Timespec) -> Self {
        Duration::new(t.tv_sec as u64, t.tv_nsec as u32)
    }
}