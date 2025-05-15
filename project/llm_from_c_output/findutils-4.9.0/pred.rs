use std::ffi::CString;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::{self, File, Metadata};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::io::{self, Write};
use std::process::exit;
use std::ptr;
use libc::{self, c_int, mode_t, uid_t, gid_t, dev_t, ino_t};
use nix::fcntl::{openat, OFlag};
use nix::sys::stat::{fstatat, FileStat, SFlag};
use nix::dir::{Dir, Type};
use nix::unistd;
use fnmatch::fnmatch;
use glob::Pattern;
use regex::Regex;
use chrono::{DateTime, Utc};
use std::ffi::OsStr;
use std::os::unix::fs::OpenOptionsExt;

const DAYSECS: i64 = 86400;

#[derive(Debug, Clone, Copy)]
enum Comparison {
    Greater,
    Less,
    Equal,
}

#[derive(Debug)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[derive(Debug)]
struct Predicate {
    pred_func: fn(&str, &Metadata, &Predicate) -> bool,
    pred_left: Option<Box<Predicate>>,
    pred_right: Option<Box<Predicate>>,
    args: PredicateArgs,
    side_effects: bool,
    no_default_print: bool,
}

#[derive(Debug)]
enum PredicateArgs {
    Reftime {
        kind: Comparison,
        ts: Timespec,
    },
    Str(String),
    Numinfo {
        kind: Comparison,
        l_val: i64,
    },
    Size {
        kind: Comparison,
        size: u64,
        blocksize: u64,
    },
    // Other argument types...
}

fn ts_difference(ts1: Timespec, ts2: Timespec) -> f64 {
    let sec_diff = (ts1.tv_sec - ts2.tv_sec) as f64;
    let nsec_diff = (ts1.tv_nsec - ts2.tv_nsec) as f64;
    sec_diff + (nsec_diff * 1.0e-9)
}

fn compare_ts(ts1: Timespec, ts2: Timespec) -> i32 {
    if ts1.tv_sec == ts2.tv_sec && ts1.tv_nsec == ts2.tv_nsec {
        0
    } else {
        let diff = ts_difference(ts1, ts2);
        if diff < 0.0 {
            -1
        } else {
            1
        }
    }
}

fn pred_timewindow(ts: Timespec, pred_ptr: &Predicate, window: i64) -> bool {
    if let PredicateArgs::Reftime { kind, ts: ref_ts } = pred_ptr.args {
        match kind {
            Comparison::Greater => compare_ts(ts, ref_ts) > 0,
            Comparison::Less => compare_ts(ts, ref_ts) < 0,
            Comparison::Equal => {
                let delta = ts_difference(ts, ref_ts);
                delta > 0.0 && delta <= window as f64
            }
        }
    } else {
        panic!("Invalid predicate arguments for timewindow");
    }
}

fn pred_amin(pathname: &str, stat_buf: &Metadata, pred_ptr: &Predicate) -> bool {
    let atime = Timespec {
        tv_sec: stat_buf.atime(),
        tv_nsec: stat_buf.atime_nsec() as i64,
    };
    pred_timewindow(atime, pred_ptr, 60)
}

fn pred_and(pathname: &str, stat_buf: &Metadata, pred_ptr: &Predicate) -> bool {
    if pred_ptr.pred_left.is_none() || apply_predicate(pathname, stat_buf, pred_ptr.pred_left.as_ref().unwrap()) {
        apply_predicate(pathname, stat_buf, pred_ptr.pred_right.as_ref().unwrap())
    } else {
        false
    }
}

fn pred_anewer(pathname: &str, stat_buf: &Metadata, pred_ptr: &Predicate) -> bool {
    if let PredicateArgs::Reftime { kind: Comparison::Greater, ts } = pred_ptr.args {
        let atime = Timespec {
            tv_sec: stat_buf.atime(),
            tv_nsec: stat_buf.atime_nsec() as i64,
        };
        compare_ts(atime, ts) > 0
    } else {
        panic!("Invalid predicate arguments for anewer");
    }
}

fn pred_atime(pathname: &str, stat_buf: &Metadata, pred_ptr: &Predicate) -> bool {
    let atime = Timespec {
        tv_sec: stat_buf.atime(),
        tv_nsec: stat_buf.atime_nsec() as i64,
    };
    pred_timewindow(atime, pred_ptr, DAYSECS)
}

fn pred_closeparen(_pathname: &str, _stat_buf: &Metadata, _pred_ptr: &Predicate) -> bool {
    true
}

fn pred_cmin(pathname: &str, stat_buf: &Metadata, pred_ptr: &Predicate) -> bool {
    let ctime = Timespec {
        tv_sec: stat_buf.ctime(),
        tv_nsec: stat_buf.ctime_nsec() as i64,
    };
    pred_timewindow(ctime, pred_ptr, 60)
}

fn pred_cnewer(pathname: &str, stat_buf: &Metadata, pred_ptr: &Predicate) -> bool {
    if let PredicateArgs::Reftime { kind: Comparison::Greater, ts } = pred_ptr.args {
        let ctime = Timespec {
            tv_sec: stat_buf.ctime(),
            tv_nsec: stat_buf.ctime_nsec() as i64,
        };
        compare_ts(ctime, ts) > 0
    } else {
        panic!("Invalid predicate arguments for cnewer");
    }
}

fn pred_comma(pathname: &str, stat_buf: &Metadata, pred_ptr: &Predicate) -> bool {
    if let Some(left) = &pred_ptr.pred_left {
        apply_predicate(pathname, stat_buf, left);
    }
    apply_predicate(pathname, stat_buf, pred_ptr.pred_right.as_ref().unwrap())
}

fn pred_ctime(pathname: &str, stat_buf: &Metadata, pred_ptr: &Predicate) -> bool {
    let ctime = Timespec {
        tv_sec: stat_buf.ctime(),
        tv_nsec: stat_buf.ctime_nsec() as i64,
    };
    pred_timewindow(ctime, pred_ptr, DAYSECS)
}

// ... Additional predicate functions ...

fn apply_predicate(pathname: &str, stat_buf: &Metadata, pred_ptr: &Predicate) -> bool {
    (pred_ptr.pred_func)(pathname, stat_buf, pred_ptr)
}

fn main() {
    // Example usage
    let pred = Predicate {
        pred_func: pred_amin,
        pred_left: None,
        pred_right: None,
        args: PredicateArgs::Reftime {
            kind: Comparison::Greater,
            ts: Timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
        },
        side_effects: false,
        no_default_print: false,
    };

    let metadata = std::fs::metadata(".").unwrap();
    println!("{}", pred_amin(".", &metadata, &pred));
}