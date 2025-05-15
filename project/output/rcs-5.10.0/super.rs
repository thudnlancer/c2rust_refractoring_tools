use std::ops::{
    Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign, Rem, RemAssign,
};
use ::libc;
use ::c2rust_bitfields;
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
    fn looking_at(sym: *const tinysym, start: *const i8) -> bool;
    fn printf(_: *const i8, _: ...) -> i32;
    fn str_save(s: *const i8) -> *mut i8;
    fn gnurcs_init(program_0: *const program);
    fn gnurcs_goodbye();
    fn strcmp(_: *const i8, _: *const i8) -> i32;
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    fn mempcpy(
        __dest: *mut libc::c_void,
        __src: *const libc::c_void,
        __n: size_t,
    ) -> *mut libc::c_void;
    fn strlen(_: *const i8) -> u64;
    fn access(__name: *const i8, __type: i32) -> i32;
    fn nice_getopt(argc: i32, argv: *mut *mut i8, longopts: *const option) -> i32;
    fn check_hv(argc: i32, argv: *mut *mut i8, prog: *const program);
    static mut plexus: *mut divvy;
    static mut single: *mut divvy;
    fn generic_fatal(who: *const i8, fmt: *const i8, _: ...);
    static mut peer_super: symdef;
    fn one_beyond_last_dir_sep(name: *const i8) -> *const i8;
    fn find_peer_prog(prog: *mut symdef) -> *const i8;
    static ya_ci: yacmd;
    static ya_co: yacmd;
    static ya_rcs: yacmd;
    static ya_rcsclean: yacmd;
    static ya_rcsdiff: yacmd;
    static ya_rcsmerge: yacmd;
    static ya_rlog: yacmd;
}
pub type __uint8_t = u8;
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
pub type uint8_t = __uint8_t;
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut i8,
    pub next_free: *mut i8,
    pub chunk_limit: *mut i8,
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub plain: Option<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option<unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut i8,
    pub prev: *mut _obstack_chunk,
    pub contents: [i8; 0],
}
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
pub struct tinysym {
    pub len: uint8_t,
    pub bytes: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct divvy {
    pub name: *const i8,
    pub space: obstack,
    pub first: *mut libc::c_void,
    pub count: size_t,
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
    pub prev: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const i8,
    pub has_arg: i32,
    pub flag: *mut i32,
    pub val: i32,
}
pub type submain_t = unsafe extern "C" fn(*const i8, i32, *mut *mut i8) -> i32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct yacmd {
    pub func: Option<submain_t>,
    pub aka: *const uint8_t,
    pub pr: *mut program,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dynamic_root {
    pub top: *mut top,
    pub single: *mut divvy,
    pub plexus: *mut divvy,
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
#[repr(C)]
pub enum ddc_option_values {
    ddc_unrecognized = 0,
    ddc_commands,
    ddc_aliases,
}
impl ddc_option_values {
    fn to_libc_c_uint(self) -> u32 {
        match self {
            ddc_option_values::ddc_unrecognized => 0,
            ddc_option_values::ddc_commands => 1,
            ddc_option_values::ddc_aliases => 2,
        }
    }
    fn from_libc_c_uint(value: u32) -> ddc_option_values {
        match value {
            0 => ddc_option_values::ddc_unrecognized,
            1 => ddc_option_values::ddc_commands,
            2 => ddc_option_values::ddc_aliases,
            _ => panic!("Invalid value for ddc_option_values: {}", value),
        }
    }
}
impl AddAssign<u32> for ddc_option_values {
    fn add_assign(&mut self, rhs: u32) {
        *self = ddc_option_values::from_libc_c_uint(self.to_libc_c_uint() + rhs);
    }
}
impl SubAssign<u32> for ddc_option_values {
    fn sub_assign(&mut self, rhs: u32) {
        *self = ddc_option_values::from_libc_c_uint(self.to_libc_c_uint() - rhs);
    }
}
impl MulAssign<u32> for ddc_option_values {
    fn mul_assign(&mut self, rhs: u32) {
        *self = ddc_option_values::from_libc_c_uint(self.to_libc_c_uint() * rhs);
    }
}
impl DivAssign<u32> for ddc_option_values {
    fn div_assign(&mut self, rhs: u32) {
        *self = ddc_option_values::from_libc_c_uint(self.to_libc_c_uint() / rhs);
    }
}
impl RemAssign<u32> for ddc_option_values {
    fn rem_assign(&mut self, rhs: u32) {
        *self = ddc_option_values::from_libc_c_uint(self.to_libc_c_uint() % rhs);
    }
}
impl Add<u32> for ddc_option_values {
    type Output = ddc_option_values;
    fn add(self, rhs: u32) -> ddc_option_values {
        ddc_option_values::from_libc_c_uint(self.to_libc_c_uint() + rhs)
    }
}
impl Sub<u32> for ddc_option_values {
    type Output = ddc_option_values;
    fn sub(self, rhs: u32) -> ddc_option_values {
        ddc_option_values::from_libc_c_uint(self.to_libc_c_uint() - rhs)
    }
}
impl Mul<u32> for ddc_option_values {
    type Output = ddc_option_values;
    fn mul(self, rhs: u32) -> ddc_option_values {
        ddc_option_values::from_libc_c_uint(self.to_libc_c_uint() * rhs)
    }
}
impl Div<u32> for ddc_option_values {
    type Output = ddc_option_values;
    fn div(self, rhs: u32) -> ddc_option_values {
        ddc_option_values::from_libc_c_uint(self.to_libc_c_uint() / rhs)
    }
}
impl Rem<u32> for ddc_option_values {
    type Output = ddc_option_values;
    fn rem(self, rhs: u32) -> ddc_option_values {
        ddc_option_values::from_libc_c_uint(self.to_libc_c_uint() % rhs)
    }
}
static mut super_blurb: [i8; 25] = unsafe {
    *::core::mem::transmute::<&[u8; 25], &[i8; 25]>(b"Dispatch an RCS command.\0")
};
static mut super_help: [i8; 262] = unsafe {
    *::core::mem::transmute::<
        &[u8; 262],
        &[i8; 262],
    >(
        b"[options] command [command-arg ...]\nOptions:\n  --commands       Display available commands and exit.\n  --aliases        Display command aliases and exit.\n  --help COMMAND   Display help for COMMAND.\n\nTo display help for the legacy interface, use:\n  --help frob\n\0",
    )
};
unsafe extern "C" fn droot_global_to_stack(mut dr: *mut dynamic_root) {
    (*dr).top = top;
    (*dr).single = single;
    (*dr).plexus = plexus;
}
unsafe extern "C" fn droot_stack_to_global(mut dr: *mut dynamic_root) {
    top = (*dr).top;
    single = (*dr).single;
    plexus = (*dr).plexus;
}
unsafe extern "C" fn dispatch(
    mut exitval: *mut i32,
    mut sub: Option<submain_t>,
    mut cmd: *const i8,
    mut argc: i32,
    mut argv: *mut *mut i8,
) {
    let mut super_0: dynamic_root = dynamic_root {
        top: 0 as *mut top,
        single: 0 as *mut divvy,
        plexus: 0 as *mut divvy,
    };
    droot_global_to_stack(&mut super_0);
    *exitval = sub.expect("non-null function pointer")(cmd, argc, argv);
    droot_stack_to_global(&mut super_0);
}
static mut avail: [*const yacmd; 7] = unsafe {
    [
        &ya_ci as *const yacmd,
        &ya_co as *const yacmd,
        &ya_rcs as *const yacmd,
        &ya_rcsclean as *const yacmd,
        &ya_rcsdiff as *const yacmd,
        &ya_rcsmerge as *const yacmd,
        &ya_rlog as *const yacmd,
    ]
};
static mut n_avail: size_t = 0;
unsafe extern "C" fn recognize(mut maybe: *const i8) -> Option<submain_t> {
    let mut mlen: size_t = strlen(maybe);
    let mut i: size_t = 0;
    i = 0 as i32 as size_t;
    while i < n_avail {
        let mut y: *const yacmd = avail[i as usize];
        let mut aka: *const uint8_t = (*y).aka;
        let fresh0 = aka;
        aka = aka.offset(1);
        let mut count: size_t = *fresh0 as size_t;
        loop {
            let fresh1 = count;
            count = count.wrapping_sub(1);
            if !(fresh1 != 0) {
                break;
            }
            let mut sym: *mut tinysym = aka as *mut tinysym;
            if mlen == (*sym).len as u64 && looking_at(sym, maybe) as i32 != 0 {
                return (*y).func;
            }
            aka = aka.offset(((*sym).len as i32 + 1 as i32) as isize);
        }
        i = i.wrapping_add(1);
        i;
    }
    return None;
}
unsafe extern "C" fn string_from_sym(mut dest: *mut i8, mut sym: *const tinysym) {
    let mut len: size_t = (if 64 as i32 > (*sym).len as i32 {
        (*sym).len as i32
    } else {
        64 as i32 - 1 as i32
    }) as size_t;
    let mut end: *mut i8 = mempcpy(
        dest as *mut libc::c_void,
        ((*sym).bytes).as_ptr() as *const libc::c_void,
        len,
    ) as *mut i8;
    *end = '\0' as i32 as i8;
}
unsafe extern "C" fn display_commands() {
    printf(
        b"%-10s  %s\n\0" as *const u8 as *const i8,
        b"(command)\0" as *const u8 as *const i8,
        b"(description)\0" as *const u8 as *const i8,
    );
    let mut i: size_t = 0 as i32 as size_t;
    while i < n_avail {
        let mut y: *const yacmd = avail[i as usize];
        let mut aka: *const uint8_t = (*y).aka;
        let mut name: [i8; 64] = [0; 64];
        aka = aka.offset(1);
        string_from_sym(name.as_mut_ptr(), aka as *mut tinysym);
        printf(
            b" %-10s  %s\n\0" as *const u8 as *const i8,
            name.as_mut_ptr(),
            (*(*y).pr).desc,
        );
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn display_aliases() {
    printf(
        b"%-10s  %s\n\0" as *const u8 as *const i8,
        b"(command)\0" as *const u8 as *const i8,
        b"(aliases)\0" as *const u8 as *const i8,
    );
    let mut i: size_t = 0 as i32 as size_t;
    while i < n_avail {
        let mut y: *const yacmd = avail[i as usize];
        let mut aka: *const uint8_t = (*y).aka;
        let fresh2 = aka;
        aka = aka.offset(1);
        let mut count: size_t = *fresh2 as size_t;
        let mut j: size_t = 0 as i32 as size_t;
        while j < count {
            let mut sym: *mut tinysym = aka as *mut tinysym;
            let mut name: [i8; 64] = [0; 64];
            string_from_sym(name.as_mut_ptr(), sym);
            match j {
                0 => {
                    printf(b" %-10s \0" as *const u8 as *const i8, name.as_mut_ptr());
                }
                1 => {
                    printf(b" %s\0" as *const u8 as *const i8, name.as_mut_ptr());
                }
                _ => {
                    printf(b", %s\0" as *const u8 as *const i8, name.as_mut_ptr());
                }
            }
            aka = aka.offset((1 as i32 + (*sym).len as i32) as isize);
            j = j.wrapping_add(1);
            j;
        }
        printf(b"\n\0" as *const u8 as *const i8);
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub static mut top: *mut top = 0 as *const top as *mut top;
unsafe extern "C" fn all_options_short_p(mut argv: *mut *mut i8) -> bool {
    let mut ok: bool = false;
    let mut i: i32 = 0;
    ok = 1 as i32 != 0;
    i = 1 as i32;
    while !(*argv.offset(i as isize)).is_null() {
        if '-' as i32 != *(*argv.offset(i as isize)).offset(0 as i32 as isize) as i32 {
            break;
        }
        if '-' as i32 == *(*argv.offset(i as isize)).offset(1 as i32 as isize) as i32 {
            ok = 0 as i32 != 0;
            break;
        } else {
            i += 1;
            i;
        }
    }
    return ok;
}
static mut hint: [i8; 14] = unsafe {
    *::core::mem::transmute::<&[u8; 14], &[i8; 14]>(b" (try --help)\0")
};
unsafe extern "C" fn huh(mut what: *const i8, mut argv1: *const i8) {
    generic_fatal(
        0 as *const i8,
        b"unknown %s: %s%s\0" as *const u8 as *const i8,
        what,
        argv1,
        hint.as_ptr(),
    );
}
static mut program: program = unsafe {
    {
        let mut init = program {
            invoke: 0 as *const i8,
            name: 0 as *const i8,
            desc: super_blurb.as_ptr(),
            help: super_help.as_ptr(),
            tyag: 0 as i32,
        };
        init
    }
};
unsafe fn main_0(mut argc: i32, mut argv: *mut *mut i8) -> i32 {
    let mut current_block: u64;
    let mut cmd: *const i8 = 0 as *const i8;
    let mut sub: Option<submain_t> = None;
    let mut exitval: i32 = 0 as i32;
    if 3 as i32 == argc
        && strcmp(b"--help\0" as *const u8 as *const i8, *argv.offset(1 as i32 as isize))
            == 0
    {
        let mut tmp: *mut i8 = *argv.offset(2 as i32 as isize);
        let ref mut fresh3 = *argv.offset(2 as i32 as isize);
        *fresh3 = *argv.offset(1 as i32 as isize);
        let ref mut fresh4 = *argv.offset(1 as i32 as isize);
        *fresh4 = tmp;
    }
    program.invoke = *argv.offset(0 as i32 as isize);
    program.name = peer_super.meaningful;
    check_hv(argc, argv, &mut program);
    gnurcs_init(&mut program);
    if 2 as i32 > argc
        || '-' as i32
            == *(*argv.offset(1 as i32 as isize)).offset(0 as i32 as isize) as i32
            && all_options_short_p(argv) as i32 != 0
    {
        current_block = 7663898120542541961;
    } else {
        if '-' as i32
            == *(*argv.offset(1 as i32 as isize)).offset(0 as i32 as isize) as i32
        {
            let mut allddc: [option; 3] = [
                {
                    let mut init = option {
                        name: b"commands\0" as *const u8 as *const i8,
                        has_arg: 0 as i32,
                        flag: 0 as *mut i32,
                        val: ddc_option_values::ddc_commands as i32,
                    };
                    init
                },
                {
                    let mut init = option {
                        name: b"aliases\0" as *const u8 as *const i8,
                        has_arg: 0 as i32,
                        flag: 0 as *mut i32,
                        val: ddc_option_values::ddc_aliases as i32,
                    };
                    init
                },
                {
                    let mut init = option {
                        name: 0 as *const i8,
                        has_arg: 0 as i32,
                        flag: 0 as *mut i32,
                        val: 0 as i32,
                    };
                    init
                },
            ];
            match nice_getopt(argc, argv, allddc.as_mut_ptr()) {
                1 => {
                    display_commands();
                    current_block = 15092860978082374424;
                }
                2 => {
                    display_aliases();
                    current_block = 15092860978082374424;
                }
                _ => {
                    huh(
                        b"option\0" as *const u8 as *const i8,
                        *argv.offset(1 as i32 as isize),
                    );
                    current_block = 5948590327928692120;
                }
            }
        } else {
            current_block = 5948590327928692120;
        }
        match current_block {
            15092860978082374424 => {}
            _ => {
                cmd = *argv.offset(1 as i32 as isize);
                sub = recognize(cmd);
                if sub.is_some() {
                    let ref mut fresh5 = *argv.offset(1 as i32 as isize);
                    *fresh5 = if !(one_beyond_last_dir_sep(
                        *argv.offset(0 as i32 as isize),
                    ))
                        .is_null()
                    {
                        *argv.offset(0 as i32 as isize)
                    } else {
                        str_save(find_peer_prog(&mut peer_super))
                    };
                    dispatch(
                        &mut exitval,
                        sub,
                        cmd,
                        argc - 1 as i32,
                        argv.offset(1 as i32 as isize),
                    );
                    current_block = 15092860978082374424;
                } else if !(strchr(cmd, '/' as i32)).is_null()
                    || !(0 as i32 > access(cmd, 4 as i32))
                {
                    current_block = 7663898120542541961;
                } else {
                    huh(
                        b"command\0" as *const u8 as *const i8,
                        *argv.offset(1 as i32 as isize),
                    );
                    current_block = 15092860978082374424;
                }
            }
        }
    }
    match current_block {
        7663898120542541961 => {
            cmd = b"rcs\0" as *const u8 as *const i8;
            sub = recognize(cmd);
            dispatch(&mut exitval, sub, cmd, argc, argv);
        }
        _ => {}
    }
    gnurcs_goodbye();
    return exitval;
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
unsafe extern "C" fn run_static_initializers() {
    n_avail = (::core::mem::size_of::<[*const yacmd; 7]>() as u64)
        .wrapping_div(::core::mem::size_of::<*const yacmd>() as u64);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];