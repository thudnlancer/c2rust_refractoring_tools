use std::cmp::Ordering;
use std::ffi::CStr;

const WARNING_ARGS: &[&str] = &[
    "all",
    "alone-zero-block",
    "bad-dumpdir",
    "cachedir",
    "contiguous-cast",
    "file-changed",
    "file-ignored",
    "file-removed",
    "file-shrank",
    "file-unchanged",
    "filename-with-nuls",
    "ignore-archive",
    "ignore-newer",
    "new-directory",
    "rename-directory",
    "symlink-cast",
    "timestamp",
    "unknown-cast",
    "unknown-keyword",
    "xdev",
    "decompress-program",
    "existing-file",
    "xattr-write",
    "record-size",
    "failed-read",
];

const WARNING_TYPES: &[i32] = &[
    !(0x2000 | 0x1000 | 0x80000 | 0x100000 | 0x400000),
    0x1,
    0x2,
    0x4,
    0x8,
    0x10,
    0x20,
    0x40,
    0x80,
    0x100,
    0x200,
    0x400,
    0x800,
    0x1000,
    0x2000,
    0x4000,
    0x8000,
    0x10000,
    0x20000,
    0x40000,
    0x80000,
    0x100000,
    0x200000,
    0x400000,
    0x800000,
];

static mut WARNING_OPTION: i32 = !(0x2000 | 0x1000 | 0x80000 | 0x100000 | 0x400000);

fn find_warning_index(arg: &str) -> Option<usize> {
    WARNING_ARGS.iter().position(|&s| s == arg)
}

pub fn set_warning_option(arg: &str) {
    let mut negate = false;
    let mut arg = arg;

    if arg == "none" {
        unsafe { WARNING_OPTION = 0 };
        return;
    }

    if arg.len() > 2 && arg.starts_with("no-") {
        negate = true;
        arg = &arg[3..];
    }

    if let Some(index) = find_warning_index(arg) {
        let option = WARNING_TYPES[index];
        unsafe {
            if negate {
                WARNING_OPTION &= !option;
            } else {
                WARNING_OPTION |= option;
            }
        }
    }
}