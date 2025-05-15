use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_long, c_uchar, c_ulong, c_void};
use std::ptr;

#[repr(C)]
pub struct _IO_FILE {
    _flags: c_int,
    _IO_read_ptr: *mut c_char,
    _IO_read_end: *mut c_char,
    _IO_read_base: *mut c_char,
    _IO_write_base: *mut c_char,
    _IO_write_ptr: *mut c_char,
    _IO_write_end: *mut c_char,
    _IO_buf_base: *mut c_char,
    _IO_buf_end: *mut c_char,
    _IO_save_base: *mut c_char,
    _IO_backup_base: *mut c_char,
    _IO_save_end: *mut c_char,
    _markers: *mut _IO_marker,
    _chain: *mut _IO_FILE,
    _fileno: c_int,
    _flags2: c_int,
    _old_offset: libc::off_t,
    _cur_column: libc::c_ushort,
    _vtable_offset: libc::c_schar,
    _shortbuf: [c_char; 1],
    _lock: *mut c_void,
    _offset: libc::off64_t,
    __pad1: *mut c_void,
    __pad2: *mut c_void,
    __pad3: *mut c_void,
    __pad4: *mut c_void,
    __pad5: libc::size_t,
    _mode: c_int,
    _unused2: [c_char; 20],
}

#[repr(C)]
pub struct _IO_marker {
    _next: *mut _IO_marker,
    _sbuf: *mut _IO_FILE,
    _pos: c_int,
}

pub type FILE = _IO_FILE;
pub type uint8_t = c_uchar;
pub type size_t = c_ulong;

#[repr(C)]
pub struct cbuf {
    string: *const c_char,
    size: size_t,
}

#[repr(C)]
pub struct delta {
    num: *const c_char,
    date: *const c_char,
    author: *const c_char,
    lockedby: *const c_char,
    state: *const c_char,
    log: *mut atat,
    text: *mut atat,
    name: *const c_char,
    pretty_log: cbuf,
    branches: *mut wlink,
    commitid: *const c_char,
    ilk: *mut delta,
    selector: bool,
    neck: libc::off_t,
}

#[repr(C)]
pub struct wlink {
    entry: *mut c_void,
    next: *mut wlink,
}

#[repr(C)]
pub struct atat {
    count: size_t,
    lno: size_t,
    line_count: size_t,
    from: *mut fro,
    beg: libc::off_t,
    holes: [libc::off_t; 0],
}

#[repr(C)]
pub struct fro {
    fd: c_int,
    end: libc::off_t,
    rm: readmethod,
    ptr: *mut c_char,
    lim: *mut c_char,
    base: *mut c_char,
    deallocate: Option<unsafe extern "C" fn(*mut fro)>,
    stream: *mut FILE,
    verbatim: libc::off_t,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum readmethod {
    RM_MMAP = 0,
    RM_MEM = 1,
    RM_STDIO = 2,
}

#[repr(C)]
pub struct rcslock {
    login: *const c_char,
    delta: *mut delta,
}

#[repr(C)]
pub struct maybe {
    open: Option<unsafe extern "C" fn(*mut maybe) -> *mut fro>,
    mustread: bool,
    tentative: cbuf,
    space: *mut divvy,
    bestfit: cbuf,
    status: *mut libc::stat,
    eno: c_int,
}

#[repr(C)]
pub struct divvy {
    name: *const c_char,
    space: obstack,
    first: *mut c_void,
    count: size_t,
}

#[repr(C)]
pub struct obstack {
    chunk_size: size_t,
    chunk: *mut _obstack_chunk,
    object_base: *mut c_char,
    next_free: *mut c_char,
    chunk_limit: *mut c_char,
    temp: C2RustUnnamed_1,
    alignment_mask: size_t,
    chunkfun: C2RustUnnamed_0,
    freefun: C2RustUnnamed,
    extra_arg: *mut c_void,
    use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    c2rust_padding: [u8; 7],
}

#[repr(C)]
pub union C2RustUnnamed {
    plain: Option<unsafe extern "C" fn(*mut c_void)>,
    extra: Option<unsafe extern "C" fn(*mut c_void, *mut c_void)>,
}

#[repr(C)]
pub union C2RustUnnamed_0 {
    plain: Option<unsafe extern "C" fn(size_t) -> *mut c_void>,
    extra: Option<unsafe extern "C" fn(*mut c_void, size_t) -> *mut c_void>,
}

#[repr(C)]
pub union C2RustUnnamed_1 {
    i: size_t,
    p: *mut c_void,
}

#[repr(C)]
pub struct _obstack_chunk {
    limit: *mut c_char,
    prev: *mut _obstack_chunk,
    contents: [c_char; 0],
}

#[repr(C)]
pub struct program {
    invoke: *const c_char,
    name: *const c_char,
    desc: *const c_char,
    help: *const c_char,
    tyag: c_int,
}

#[repr(C)]
pub struct behavior {
    invdir: *const c_char,
    unbuffered: bool,
    quiet: bool,
    interactive_valid: bool,
    interactive: bool,
    inclusive_of_Locker_in_Id_val: bool,
    strictly_locking: bool,
    version_set: bool,
    version: c_int,
    stick_with_euid: bool,
    ruid: c_int,
    euid: c_int,
    ruid_cached: bool,
    euid_cached: bool,
    already_setuid: bool,
    kws: c_int,
    pe: *const c_char,
    zone_offset: zone_offset,
    username: *mut c_char,
    now: libc::timespec,
    fixed_SIGCHLD: bool,
    Oerrloop: bool,
    cwd: *mut c_char,
    mem_limit: libc::off_t,
    sff: *mut sff,
    isr: *mut isr_scratch,
    ephemstuff: *mut ephemstuff,
    maketimestuff: *mut maketimestuff,
}

#[repr(C)]
pub struct zone_offset {
    valid: bool,
    seconds: c_long,
}

#[repr(C)]
pub struct manifestation {
    filename: *mut c_char,
    standard_output: *mut FILE,
    prev: C2RustUnnamed_2,
}

#[repr(C)]
pub struct C2RustUnnamed_2 {
    valid: bool,
    author: *mut c_char,
    date: *mut c_char,
    name: *mut c_char,
    rev: *mut c_char,
    state: *mut c_char,
}

#[repr(C)]
pub struct repo {
    head: *const c_char,
    branch: *const c_char,
    access_count: size_t,
    access: *mut link,
    symbols_count: size_t,
    symbols: *mut link,
    locks_count: size_t,
    locks: *mut link,
    strict: bool,
    integrity: *mut atat,
    comment: *mut atat,
    expand: c_int,
    deltas_count: size_t,
    deltas: *mut wlink,
    desc: *mut atat,
    neck: libc::off_t,
    lockdefs: *mut lockdef,
    ht: *mut hash,
}

#[repr(C)]
pub struct link {
    entry: *const c_void,
    next: *mut link,
}

#[repr(C)]
pub struct repository {
    filename: *const c_char,
    fd_lock: c_int,
    stat: libc::stat,
    r: *mut repo,
    tip: *mut delta,
    log_lead: cbuf,
}

#[repr(C)]
pub struct flow {
    from: *mut fro,
    rewr: *mut FILE,
    to: *mut FILE,
    res: *mut FILE,
    result: *const c_char,
    erroneous: bool,
}

#[repr(C)]
pub struct top {
    program: *const program,
    behavior: behavior,
    manifestation: manifestation,
    repository: repository,
    flow: flow,
}

#[repr(C)]
pub struct revrange {
    beg: *const c_char,
    end: *const c_char,
    nfield: c_int,
}

#[repr(C)]
pub struct daterange {
    beg: [c_char; 22],
    end: [c_char; 22],
    open_end: bool,
}

#[repr(C)]
pub struct date_selection {
    in_0: *mut link,
    by: *mut link,
}

#[repr(C)]
pub struct criteria {
    revs: *mut link,
    actual: *mut link,
    authors: *mut link,
    lockers: *mut link,
    states: *mut link,
}

static mut rlog_blurb: [c_char; 58] = *b"Print log messages and other information about RCS files.\0";
static mut rlog_help: [c_char; 1601] = *b"[options] file ...\nOptions:\n  -L            Ignore RCS files with no locks set.\n  -R            Print the RCS file name only.\n  -h            Print only the \"header\" information.\n  -t            Like -h, but also include the description.\n  -N            Omit symbolic names.\n  -b            Select the default branch.\n  -dDATES       Select revisions in the range DATES, with spec:\n                  D      -- single revision D or earlier\n                  D1<D2  -- between D1 and D2, exclusive\n                  D2>D1  -- likewise\n                  <D, D> -- before D\n                  >D, D< -- after D\n                Use <= or >= to make ranges inclusive; DATES\n                may also be a list of semicolon-separated specs.\n  -l[WHO]       Select revisions locked by WHO (comma-separated list)\n                only, or by anyone if WHO is omitted.\n  -r[REVS]      Select revisions in REVS, a comma-separated list of\n                range specs, one of: REV, REV:, :REV, REV1:REV2\n  -sSTATES      Select revisions with state in STATES (comma-separated list).\n  -w[WHO]       Select revisions checked in by WHO (comma-separated list),\n                or by the user if WHO is omitted.\n  -T            No effect; included for compatibility with other commands.\n  -V            Obsolete; do not use.\n  -VN           Emulate RCS version N.\n  -xSUFF        Specify SUFF as a slash-separated list of suffixes\n                used to identify RCS file names.\n  -zZONE        Specify date output format in keyword-substitution.\n  -q            No effect, included for consistency with other commands.\n\0";

static mut ks_delims: [c_char; 6] = *b", \t\n;\0";

unsafe fn tokenize(argv: *mut c_char, chain: *mut *mut link) -> bool {
    let before = *chain;
    let mut save = ptr::null_mut();
    let mut s = argv;
    
    loop {
        let token = libc::strtok_r(s, ks_delims.as_ptr(), &mut save);
        if token.is_null() {
            break;
        }
        *chain = prepend(token as *const c_void, *chain, plexus);
        s = ptr::null_mut();
    }
    
    *chain != before
}

unsafe fn cleanup(exitstatus: *mut c_int) {
    if (*top).flow.erroneous {
        *exitstatus = exit_failure;
    }
    fro_zclose(&mut (*top).flow.from);
}

unsafe fn getlocker(argv: *mut c_char, criteria: *mut criteria) {
    tokenize(argv, &mut (*criteria).lockers);
}

unsafe fn read_positive_integer(p: *mut *const c_char) -> c_long {
    let mut end = ptr::null_mut();
    *libc::__errno_location() = 0;
    let rv = libc::strtol(*p, &mut end, 10);
    
    if 1 > rv {
        generic_fatal(
            (*top).repository.filename,
            b"non-positive integer\0".as_ptr() as *const c_char,
        );
    }
    if *libc::__errno_location() == libc::ERANGE {
        generic_fatal(
            (*top).repository.filename,
            b"bad integer\0".as_ptr() as *const c_char,
        );
    }
    
    *p = end;
    rv
}

unsafe fn count_a_d(a: *mut c_long, d: *mut c_long, edits: *mut atat) {
    let s = string_from_atat(single, edits);
    let end = s.string.offset(s.size as isize);
    let totals = zlloc(
        single,
        2 * std::mem::size_of::<c_long>() as size_t,
    ) as *mut c_long;
    
    let mut p = s.string;
    while p < end {
        let add = *p == b'a' as c_char;
        p = p.offset(1);
        
        p = libc::strchr(p, b' ' as c_int);
        let count = read_positive_integer(&mut p);
        
        *totals.offset(add as isize) += count;
        
        if add {
            for _ in 0..count {
                let remaining = end.offset_from(p) as size_t;
                p = p.offset(1);
                p = libc::memchr(p as *const c_void, b'\n' as c_int, remaining) as *const c_char;
                if p.is_null() {
                    break;
                }
            }
        }
        p = p.offset(1);
    }
    
    *a = *totals.offset(1);
    *d = *totals.offset(0);
    brush_off(single, totals as *mut c_void);
}

// ... (remaining functions would follow similar translation patterns)