use chrono::{Datelike, Timelike, NaiveDate, NaiveDateTime};
use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_double};
use std::ptr;

static WEEK: [&str; 7] = [
    "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"
];

static MONTHS: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
];

pub fn _glp_mpl_fn_gmtime(_mpl: *mut libc::c_void) -> c_double {
    let now = chrono::Utc::now();
    let epoch = NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0);
    let duration = now.naive_utc() - epoch;
    duration.num_seconds() as c_double
}

pub fn _glp_mpl_fn_str2time(
    _mpl: *mut libc::c_void,
    str: *const c_char,
    fmt: *const c_char,
) -> c_double {
    let c_str = unsafe { CStr::from_ptr(str) };
    let c_fmt = unsafe { CStr::from_ptr(fmt) };
    
    let s = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0.0,
    };
    
    let f = match c_fmt.to_str() {
        Ok(f) => f,
        Err(_) => return 0.0,
    };
    
    match NaiveDateTime::parse_from_str(s, f) {
        Ok(dt) => {
            let epoch = NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0);
            (dt - epoch).num_seconds() as c_double
        }
        Err(_) => 0.0,
    }
}

pub fn _glp_mpl_fn_time2str(
    _mpl: *mut libc::c_void,
    str: *mut c_char,
    t: c_double,
    fmt: *const c_char,
) {
    let epoch = NaiveDate::from_ymd(1970, 1, 1).and_hms(0, 0, 0);
    let dt = epoch + chrono::Duration::seconds(t as i64);
    
    let c_fmt = unsafe { CStr::from_ptr(fmt) };
    let format = match c_fmt.to_str() {
        Ok(f) => f,
        Err(_) => return,
    };
    
    let formatted = dt.format(format).to_string();
    let c_str = CString::new(formatted).unwrap();
    unsafe {
        ptr::copy_nonoverlapping(c_str.as_ptr(), str, c_str.as_bytes().len() + 1);
    }
}

fn weekday(j: i32) -> i32 {
    let epoch = NaiveDate::from_ymd(1970, 1, 1);
    let date = epoch + chrono::Duration::days(j as i64);
    date.weekday().number_from_monday() as i32
}

fn firstday(year: i32) -> i32 {
    let jan1 = NaiveDate::from_ymd(year, 1, 1);
    let days = (jan1.weekday().number_from_monday() - 1) as i32;
    let first_monday = jan1 - chrono::Duration::days(days as i64);
    (first_monday - NaiveDate::from_ymd(1970, 1, 1)).num_days() as i32
}