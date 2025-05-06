#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
extern "C" {
    pub type wlink;
    pub type atat;
    pub type fro;
    pub type maketimestuff;
    pub type ephemstuff;
    pub type isr_scratch;
    pub type hash;
    pub type lockdef;
    pub type link;
    fn merge(tostdout: bool, edarg: *const i8, three_manifestations: *mut symdef) -> i32;
    fn thank_you_and_goodnight(how: i32);
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn bad_option(option: *const i8);
    fn check_hv(argc: i32, argv: *mut *mut i8, prog: *const program);
    fn display_version(prog: *const program, flags: i32);
    fn generic_error(who: *const i8, fmt: *const i8, _: ...);
    fn generic_fatal(who: *const i8, fmt: *const i8, _: ...);
}
pub type __dev_t = u64;
pub type __uid_t = u32;
pub type __gid_t = u32;
pub type __ino_t = u64;
pub type __mode_t = u32;
pub type __nlink_t = u64;
pub type __off_t = i64;
pub type __off64_t = i64;
pub type __time_t = i64;
pub type __blksize_t = i64;
pub type __blkcnt_t = i64;
pub type __syscall_slong_t = i64;
pub type size_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut i8,
    pub _IO_read_end: *mut i8,
    pub _IO_read_base: *mut i8,
    pub _IO_write_base: *mut i8,
    pub _IO_write_ptr: *mut i8,
    pub _IO_write_end: *mut i8,
    pub _IO_buf_base: *mut i8,
    pub _IO_buf_end: *mut i8,
    pub _IO_save_base: *mut i8,
    pub _IO_backup_base: *mut i8,
    pub _IO_save_end: *mut i8,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    pub _flags2: i32,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [i8; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: i32,
    pub _unused2: [i8; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: i32,
}
pub type FILE = _IO_FILE;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: i32,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cbuf {
    pub string: *const i8,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct delta {
    pub num: *const i8,
    pub date: *const i8,
    pub author: *const i8,
    pub lockedby: *const i8,
    pub state: *const i8,
    pub log: *mut atat,
    pub text: *mut atat,
    pub name: *const i8,
    pub pretty_log: cbuf,
    pub branches: *mut wlink,
    pub commitid: *const i8,
    pub ilk: *mut delta,
    pub selector: bool,
    pub neck: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct symdef {
    pub meaningful: *const i8,
    pub underlying: *const i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct program {
    pub invoke: *const i8,
    pub name: *const i8,
    pub desc: *const i8,
    pub help: *const i8,
    pub tyag: i32,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum maker {
    notmade,
    real,
    effective,
}
impl maker {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            maker::notmade => 0,
            maker::real => 1,
            maker::effective => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> maker {
        match value {
            0 => maker::notmade,
            1 => maker::real,
            2 => maker::effective,
            _ => panic!("Invalid value for maker: {}", value),
        }
    }
}
impl AddAssign<u32> for maker {
    fn add_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for maker {
    fn sub_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for maker {
    fn mul_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for maker {
    fn div_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for maker {
    fn rem_assign(&mut self, rhs: u32) {
        *self = maker::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for maker {
    type Output = maker;
    fn add(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for maker {
    type Output = maker;
    fn sub(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for maker {
    type Output = maker;
    fn mul(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for maker {
    type Output = maker;
    fn div(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for maker {
    type Output = maker;
    fn rem(self, rhs: u32) -> maker {
        maker::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sff {
    pub filename: *const i8,
    pub disposition: maker,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct behavior {
    pub invdir: *const i8,
    pub unbuffered: bool,
    pub quiet: bool,
    pub interactive_valid: bool,
    pub interactive: bool,
    pub inclusive_of_Locker_in_Id_val: bool,
    pub strictly_locking: bool,
    pub version_set: bool,
    pub version: i32,
    pub stick_with_euid: bool,
    pub ruid: i32,
    pub euid: i32,
    pub ruid_cached: bool,
    pub euid_cached: bool,
    pub already_setuid: bool,
    pub kws: i32,
    pub pe: *const i8,
    pub zone_offset: zone_offset,
    pub username: *mut i8,
    pub now: timespec,
    pub fixed_SIGCHLD: bool,
    pub Oerrloop: bool,
    pub cwd: *mut i8,
    pub mem_limit: off_t,
    pub sff: *mut sff,
    pub isr: *mut isr_scratch,
    pub ephemstuff: *mut ephemstuff,
    pub maketimestuff: *mut maketimestuff,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zone_offset {
    pub valid: bool,
    pub seconds: i64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct manifestation {
    pub filename: *mut i8,
    pub standard_output: *mut FILE,
    pub prev: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub valid: bool,
    pub author: *mut i8,
    pub date: *mut i8,
    pub name: *mut i8,
    pub rev: *mut i8,
    pub state: *mut i8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repo {
    pub head: *const i8,
    pub branch: *const i8,
    pub access_count: size_t,
    pub access: *mut link,
    pub symbols_count: size_t,
    pub symbols: *mut link,
    pub locks_count: size_t,
    pub locks: *mut link,
    pub strict: bool,
    pub integrity: *mut atat,
    pub comment: *mut atat,
    pub expand: i32,
    pub deltas_count: size_t,
    pub deltas: *mut wlink,
    pub desc: *mut atat,
    pub neck: off_t,
    pub lockdefs: *mut lockdef,
    pub ht: *mut hash,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct repository {
    pub filename: *const i8,
    pub fd_lock: i32,
    pub stat: stat,
    pub r: *mut repo,
    pub tip: *mut delta,
    pub log_lead: cbuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct flow {
    pub from: *mut fro,
    pub rewr: *mut FILE,
    pub to: *mut FILE,
    pub res: *mut FILE,
    pub result: *const i8,
    pub erroneous: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct top {
    pub program: *const program,
    pub behavior: behavior,
    pub manifestation: manifestation,
    pub repository: repository,
    pub flow: flow,
}
static mut merge_blurb: [i8; 22] = unsafe {
    *::core::mem::transmute::<&[u8; 22], &[i8; 22]>(b"Three-way file merge.\0")
};
static mut merge_help: [i8; 494] = unsafe {
    *::core::mem::transmute::<
        &[u8; 494],
        &[i8; 494],
    >(
        b"[options] receiving-sibling parent other-sibling\nOptions:\n  -A            Use `diff3 -A' style.\n  -E            Use `diff3 -E' style (default).\n  -e            Use `diff3 -e' style.\n  -p            Write to stdout instead of overwriting RECEIVING-SIBLING.\n  -q            Quiet mode; suppress conflict warnings.\n  -L LABEL      (up to three times) Specify the conflict labels for\n                RECEIVING-SIBLING, PARENT and OTHER-SIBLING, respectively.\n  -V            Obsolete; do not use.\n\0",
    )
};
#[no_mangle]
pub static mut top: *mut top = 0 as *const top as *mut top;
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const i8,
            name: 0 as *const i8,
            desc: merge_blurb.as_ptr(),
            help: merge_help.as_ptr(),
            tyag: (1 as i32) << 1 as i32 | (1 as i32) << 0 as i32,
        };
        init
    }
};
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut a: *const i8 = 0 as *const i8;
    let mut three_manifestations: [symdef; 3] = [symdef {
        meaningful: 0 as *const i8,
        underlying: 0 as *const i8,
    }; 3];
    let mut edarg: *const i8 = 0 as *const i8;
    let mut labels: i32 = 0;
    let mut exitstatus: i32 = 0;
    let mut tostdout: bool = 0 as i32 != 0;
    program.invoke = *argv.offset(0 as i32 as isize);
    program.name = b"merge\0" as *const u8 as *const i8;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    labels = 0 as i32;
    let mut current_block_24: u64;
    loop {
        argv = argv.offset(1);
        a = *argv;
        if !(!a.is_null()
            && {
                let fresh0 = a;
                a = a.offset(1);
                *fresh0 as i32 == '-' as i32
            })
        {
            break;
        }
        let fresh1 = a;
        a = a.offset(1);
        match *fresh1 as i32 {
            65 | 69 | 101 => {
                if !edarg.is_null()
                    && *edarg.offset(1 as i32 as isize) as i32
                        != *(*argv).offset(1 as i32 as isize) as i32
                {
                    generic_error(
                        0 as *const i8,
                        b"%s and %s are incompatible\0" as *const u8 as *const i8,
                        edarg,
                        *argv,
                    );
                }
                edarg = *argv;
                current_block_24 = 7172762164747879670;
            }
            112 => {
                tostdout = 1 as i32 != 0;
                current_block_24 = 7172762164747879670;
            }
            113 => {
                (*top).behavior.quiet = 1 as i32 != 0;
                current_block_24 = 7172762164747879670;
            }
            76 => {
                if 3 as i32 <= labels {
                    generic_fatal(
                        0 as *const i8,
                        b"too many -L options\0" as *const u8 as *const i8,
                    );
                }
                argv = argv.offset(1);
                let fresh2 = labels;
                labels = labels + 1;
                three_manifestations[fresh2 as usize].meaningful = *argv;
                if (three_manifestations[fresh2 as usize].meaningful).is_null() {
                    generic_fatal(
                        0 as *const i8,
                        b"-L needs following argument\0" as *const u8 as *const i8,
                    );
                }
                argc -= 1;
                argc;
                current_block_24 = 7172762164747879670;
            }
            86 => {
                if *a.offset(0 as i32 as isize) != 0 {
                    bad_option(a.offset(-(2 as i32 as isize)));
                } else {
                    display_version(&mut program, 1 as i32);
                }
                gnurcs_goodbye();
                return if *a.offset(0 as i32 as isize) as i32 != 0 {
                    1 as i32
                } else {
                    0 as i32
                };
            }
            _ => {
                bad_option(a.offset(-(2 as i32 as isize)));
                current_block_24 = 7815301370352969686;
            }
        }
        match current_block_24 {
            7172762164747879670 => {
                if *a != 0 {
                    bad_option(a.offset(-(2 as i32 as isize)));
                }
            }
            _ => {}
        }
        argc -= 1;
        argc;
    }
    if argc != 4 as i32 {
        generic_fatal(
            0 as *const i8,
            b"%s arguments\0" as *const u8 as *const i8,
            if argc < 4 as i32 {
                b"not enough\0" as *const u8 as *const i8
            } else {
                b"too many\0" as *const u8 as *const i8
            },
        );
    }
    let mut i: i32 = 0 as i32;
    while i < 3 as i32 {
        three_manifestations[i as usize].underlying = *argv.offset(i as isize);
        if labels <= i {
            three_manifestations[i as usize].meaningful = three_manifestations[i
                    as usize]
                .underlying;
        }
        i += 1;
        i;
    }
    if (*top).flow.erroneous {
        thank_you_and_goodnight((*(*top).program).tyag);
    }
    exitstatus = merge(tostdout, edarg, three_manifestations.as_mut_ptr());
    gnurcs_goodbye();
    return exitstatus;
}
pub fn main() {
    let mut args: Vec<*mut i8> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0((args.len() - 1) as i32, args.as_mut_ptr() as *mut *mut i8) as i32,
        )
    }
}