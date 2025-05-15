// Convert UTC calendar time to simple time. Like mktime but assumes UTC.

// Copyright (C) 1994-2023 Free Software Foundation, Inc.
// This file is part of the GNU C Library.

// The GNU C Library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.

// The GNU C Library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public
// License along with the GNU C Library; if not, see
// <https://www.gnu.org/licenses/>.

use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;

pub fn timegm(tm: &mut libc::tm) -> Result<i64, std::io::Error> {
    // Convert tm to SystemTime
    let time = SystemTime::UNIX_EPOCH
        .checked_add(std::time::Duration::from_secs(
            (tm.tm_sec
                + tm.tm_min * 60
                + tm.tm_hour * 3600
                + tm.tm_yday * 86400
                + (tm.tm_year - 70) * 31536000
                + ((tm.tm_year - 69) / 4) * 86400
                - ((tm.tm_year - 1) / 100) * 86400
                + ((tm.tm_year + 299) / 400) * 86400) as u64,
        ))
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "time overflow"))?;

    // Convert back to verify
    let duration = time.duration_since(UNIX_EPOCH)
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "time before epoch"))?;
    let secs = duration.as_secs();

    if secs > i64::MAX as u64 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "time overflow",
        ));
    }

    Ok(secs as i64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc::{tm, time_t};

    #[test]
    fn test_timegm() {
        let mut tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 1,
            tm_mon: 0,
            tm_year: 70,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: std::ptr::null_mut(),
        };

        assert_eq!(timegm(&mut tm).unwrap(), 0);
    }
}