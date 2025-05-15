use chrono::{DateTime, Datelike, FixedOffset, Local, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Timelike};
use std::fmt::Write;
use std::time::{SystemTime, UNIX_EPOCH};

const DATESIZE: usize = 20;
const FULLDATESIZE: usize = 50;

struct Version {
    major: u32,
    minor: u32,
}

impl Version {
    fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    fn less_than(&self, other: &Version) -> bool {
        self.major < other.major || (self.major == other.major && self.minor < other.minor)
    }
}

struct ZoneOffset {
    valid: bool,
    seconds: i64,
}

struct RuntimeState {
    now: SystemTime,
    zone_offset: ZoneOffset,
    version: Version,
}

impl RuntimeState {
    fn be(&self) -> &Self {
        self
    }
}

fn time2date(unixtime: i64, date: &mut [u8; DATESIZE], version: &Version) {
    let naive = NaiveDateTime::from_timestamp_opt(unixtime, 0).unwrap();
    let year = naive.year();
    let year = if (1900..=1999).contains(&year) {
        year - 1900
    } else {
        year
    };

    let _ = write!(
        unsafe { std::str::from_utf8_unchecked_mut(date) },
        "{:02}.{:02}.{:02}.{:02}.{:02}.{:02}",
        year,
        naive.month(),
        naive.day(),
        naive.hour(),
        naive.minute(),
        naive.second()
    );
}

fn str2time_checked(source: &str, default_time: i64, default_zone: i64) -> Result<i64, String> {
    let t = str2time(source, default_time, default_zone)?;
    if t == -1 {
        Err(format!("unknown date/time: {}", source))
    } else {
        Ok(t)
    }
}

fn str2date(source: &str, target: &mut [u8; DATESIZE], state: &RuntimeState) -> Result<(), String> {
    let default_zone = if state.be().zone_offset.valid {
        state.be().zone_offset.seconds
    } else if state.be().version.less_than(&Version::new(5, 0)) {
        -1 // TM_LOCAL_ZONE equivalent
    } else {
        0
    };

    let unixtime = str2time_checked(
        source,
        state.be().now.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64,
        default_zone,
    )?;
    time2date(unixtime, target, &state.version);
    Ok(())
}

fn date2time(source: &[u8; DATESIZE]) -> Result<i64, String> {
    let mut s = [0u8; FULLDATESIZE];
    let date_str = date2str(source, &mut s)?;
    str2time_checked(date_str, 0, 0)
}

fn zone_set(s: &str, state: &mut RuntimeState) -> Result<(), String> {
    state.be_mut().zone_offset.valid = !s.is_empty();
    if state.be().zone_offset.valid {
        let (zone, zonetail) = parzone(s)?;
        if !zonetail.is_empty() {
            return Err(format!("{}: not a known time zone", s));
        }
        state.be_mut().zone_offset.seconds = zone;
    }
    Ok(())
}

fn date2str(date: &[u8; DATESIZE], datebuf: &mut [u8; FULLDATESIZE], state: &RuntimeState) -> Result<&str, String> {
    let date_str = std::str::from_utf8(date).map_err(|e| e.to_string())?;
    let mut parts = date_str.split('.');

    if !state.be().zone_offset.valid {
        let year = parts.next().ok_or("Invalid date format")?;
        let month = parts.next().ok_or("Invalid date format")?;
        let day = parts.next().ok_or("Invalid date format")?;
        let hour = parts.next().ok_or("Invalid date format")?;
        let minute = parts.next().ok_or("Invalid date format")?;
        let second = parts.next().ok_or("Invalid date format")?;

        let prefix = if year.len() == 2 && !state.be().version.less_than(&Version::new(5, 0)) {
            ""
        } else {
            "19"
        };

        let _ = write!(
            unsafe { std::str::from_utf8_unchecked_mut(datebuf) },
            "{}{}/{}/{}/{} {}:{}:{}",
            prefix, year, month, day, hour, minute, second
        );
    } else {
        let year = parts.next().ok_or("Invalid date format")?.parse::<i32>().map_err(|e| e.to_string())?;
        let month = parts.next().ok_or("Invalid date format")?.parse::<u32>().map_err(|e| e.to_string())?;
        let day = parts.next().ok_or("Invalid date format")?.parse::<u32>().map_err(|e| e.to_string())?;
        let hour = parts.next().ok_or("Invalid date format")?.parse::<u32>().map_err(|e| e.to_string())?;
        let minute = parts.next().ok_or("Invalid date format")?.parse::<u32>().map_err(|e| e.to_string())?;
        let second = parts.next().ok_or("Invalid date format")?.parse::<u32>().map_err(|e| e.to_string())?;

        let naive_date = NaiveDate::from_ymd_opt(year, month, day).ok_or("Invalid date")?;
        let naive_time = NaiveTime::from_hms_opt(hour, minute, second).ok_or("Invalid time")?;
        let naive_datetime = NaiveDateTime::new(naive_date, naive_time);

        let zone = state.be().zone_offset.seconds;
        let offset = if zone == -1 { // TM_LOCAL_ZONE equivalent
            let local: DateTime<Local> = Local.from_local_datetime(&naive_datetime).unwrap();
            local.offset().fix().local_minus_utc() as i64
        } else {
            zone
        };

        let sign = if offset >= 0 { '+' } else { '-' };
        let abs_offset = offset.abs();
        let hours = abs_offset / 3600;
        let minutes = (abs_offset % 3600) / 60;
        let seconds = abs_offset % 60;

        let offset_datetime = naive_datetime + chrono::Duration::seconds(offset);

        let mut buf = String::new();
        write!(
            &mut buf,
            "{:04}-{:02}-{:02} {:02}:{:02}:{:02}{}{:02}",
            offset_datetime.year(),
            offset_datetime.month(),
            offset_datetime.day(),
            offset_datetime.hour(),
            offset_datetime.minute(),
            offset_datetime.second(),
            sign,
            hours
        ).unwrap();

        if minutes != 0 || seconds != 0 {
            write!(&mut buf, ":{:02}", minutes).unwrap();
            if seconds != 0 {
                write!(&mut buf, ":{:02}", seconds).unwrap();
            }
        }

        datebuf[..buf.len()].copy_from_slice(buf.as_bytes());
    }

    Ok(std::str::from_utf8(datebuf).map_err(|e| e.to_string())?)
}

fn parzone(s: &str) -> Result<(i64, &str), String> {
    // Simplified implementation - would need actual timezone parsing logic
    Ok((0, ""))
}

fn str2time(source: &str, default_time: i64, default_zone: i64) -> Result<i64, String> {
    // Simplified implementation - would need actual date parsing logic
    Ok(default_time)
}