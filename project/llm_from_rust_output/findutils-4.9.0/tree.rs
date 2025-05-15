use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;

#[repr(C)]
struct Timespec {
    tv_sec: i64,
    tv_nsec: i64,
}

#[repr(C)]
struct Stat {
    st_dev: u64,
    st_ino: u64,
    st_nlink: u64,
    st_mode: u32,
    st_uid: u32,
    st_gid: u32,
    __pad0: c_int,
    st_rdev: u64,
    st_size: i64,
    st_blksize: i64,
    st_blocks: i64,
    st_atim: Timespec,
    st_mtim: Timespec,
    st_ctim: Timespec,
    __glibc_reserved: [i64; 3],
}

#[repr(C)]
struct RePatternBuffer {
    buffer: *mut c_void,
    allocated: usize,
    used: usize,
    syntax: u64,
    fastmap: *mut c_char,
    translate: *mut u8,
    re_nsub: usize,
    flags: u8,
    _padding: [u8; 7],
}

#[repr(C)]
struct BuildcmdState {
    cmd_argc: usize,
    cmd_argv: *mut *mut c_char,
    cmd_argv_alloc: usize,
    argbuf: *mut c_char,
    cmd_argv_chars: usize,
    cmd_initial_argv_chars: usize,
    usercontext: *mut c_void,
    todo: c_int,
    dir_fd: c_int,
    largest_successful_arg_count: usize,
    smallest_failed_arg_count: usize,
}

#[repr(C)]
struct BuildcmdControl {
    exit_if_size_exceeded: c_int,
    posix_arg_size_max: usize,
    posix_arg_size_min: usize,
    arg_max: usize,
    max_arg_count: usize,
    rplen: usize,
    replace_pat: *const c_char,
    initial_argc: usize,
    exec_callback: Option<unsafe extern "C" fn(*mut BuildcmdControl, *mut c_void, c_int, *mut *mut c_char) -> c_int>,
    lines_per_exec: u64,
    args_per_exec: usize,
}

#[repr(C)]
struct Predicate {
    pred_func: Option<unsafe extern "C" fn(*const c_char, *mut Stat, *mut Predicate) -> bool>,
    p_name: *const c_char,
    p_type: PredicateType,
    p_prec: PredicatePrecedence,
    side_effects: bool,
    no_default_print: bool,
    need_stat: bool,
    need_type: bool,
    need_inum: bool,
    p_cost: EvaluationCost,
    est_success_rate: f32,
    literal_control_chars: bool,
    artificial: bool,
    arg_text: *const c_char,
    args: PredicateArgs,
    pred_next: *mut Predicate,
    pred_left: *mut Predicate,
    pred_right: *mut Predicate,
    perf: PredicatePerformanceInfo,
    parser_entry: *const ParserTable,
}

#[repr(C)]
struct ParserTable {
    type_: ArgType,
    parser_name: *const c_char,
    parser_func: Option<unsafe extern "C" fn(*const ParserTable, *mut *mut c_char, *mut c_int) -> bool>,
    pred_func: Option<unsafe extern "C" fn(*const c_char, *mut Stat, *mut Predicate) -> bool>,
}

#[repr(C)]
struct PredicatePerformanceInfo {
    visits: u64,
    successes: u64,
}

#[repr(C)]
union PredicateArgs {
    str_: *const c_char,
    regex: *mut RePatternBuffer,
    exec_vec: ExecVal,
    numinfo: LongVal,
    size: SizeVal,
    uid: u32,
    gid: u32,
    reftime: TimeVal,
    perm: PermVal,
    samefileid: SamefileFileId,
    types: [bool; 7],
    printf_vec: FormatVal,
    scontext: *mut c_char,
}

#[repr(C)]
struct FormatVal {
    segment: *mut Segment,
    stream: *mut libc::FILE,
    filename: *const c_char,
    dest_is_tty: bool,
    quote_opts: *mut c_void,
}

#[repr(C)]
struct Segment {
    segkind: SegmentKind,
    format_char: [c_char; 2],
    text: *mut c_char,
    text_len: c_int,
    next: *mut Segment,
}

#[repr(C)]
struct SamefileFileId {
    ino: u64,
    dev: u64,
    fd: c_int,
}

#[repr(C)]
struct PermVal {
    kind: PermissionsType,
    val: [u32; 2],
}

#[repr(C)]
struct TimeVal {
    xval: XVal,
    kind: ComparisonType,
    ts: Timespec,
}

#[repr(C)]
struct SizeVal {
    kind: ComparisonType,
    blocksize: c_int,
    size: u64,
}

#[repr(C)]
struct LongVal {
    kind: ComparisonType,
    negative: bool,
    l_val: u64,
}

#[repr(C)]
struct ExecVal {
    multiple: bool,
    ctl: BuildcmdControl,
    state: BuildcmdState,
    replace_vec: *mut *mut c_char,
    num_args: c_int,
    close_stdin: bool,
    wd_for_exec: *mut c_void,
    last_child_status: c_int,
}

#[repr(C)]
struct Options {
    do_dir_first: bool,
    explicit_depth: bool,
    maxdepth: c_int,
    mindepth: c_int,
    no_leaf_check: bool,
    stay_on_filesystem: bool,
    ignore_readdir_race: bool,
    literal_control_chars: bool,
    warnings: bool,
    posixly_correct: bool,
    start_time: Timespec,
    cur_day_start: Timespec,
    full_days: bool,
    output_block_size: c_int,
    debug_options: u64,
    symlink_handling: SymlinkOption,
    xstat: Option<unsafe extern "C" fn(*const c_char, *mut Stat) -> c_int>,
    open_nofollow_available: bool,
    regex_options: c_int,
    x_getfilecon: Option<unsafe extern "C" fn(c_int, *const c_char, *mut *mut c_char) -> c_int>,
    optimisation_level: u16,
    err_quoting_style: QuotingStyle,
    files0_from: *const c_char,
    ok_prompt_stdin: bool,
}

#[repr(C)]
struct State {
    curdepth: c_int,
    have_stat: bool,
    have_type: bool,
    type_: u32,
    rel_pathname: *const c_char,
    cwd_dir_fd: c_int,
    starting_path_length: c_int,
    stop_at_current_level: bool,
    exit_status: c_int,
    execdirs_outstanding: bool,
    shared_files: *mut c_void,
    already_issued_stat_error_msg: bool,
}

#[repr(C)]
struct PredList {
    head: *mut Predicate,
    tail: *mut Predicate,
}

#[repr(C)]
struct PredCostLookup {
    fn_: Option<unsafe extern "C" fn(*const c_char, *mut Stat, *mut Predicate) -> bool>,
    cost: EvaluationCost,
}

#[repr(C)]
struct CostAssoc {
    cost: EvaluationCost,
    name: *const c_char,
}

#[repr(C)]
struct PrecAssoc {
    prec: i16,
    prec_name: *const c_char,
}

#[repr(C)]
struct OpAssoc {
    type_: i16,
    type_name: *const c_char,
}

#[derive(PartialEq, Eq)]
#[repr(u32)]
enum ArgType {
    Option = 0,
    Noop = 1,
    PositionalOption = 2,
    Test = 3,
    SpecialParse = 4,
    Punctuation = 5,
    Action = 6,
}

#[derive(PartialEq, Eq)]
#[repr(u32)]
enum PredicateType {
    NoType = 0,
    PrimaryType = 1,
    UniOp = 2,
    BiOp = 3,
    OpenParen = 4,
    CloseParen = 5,
}

#[derive(PartialEq, Eq)]
#[repr(u32)]
enum PredicatePrecedence {
    NoPrec = 0,
    CommaPrec = 1,
    OrPrec = 2,
    AndPrec = 3,
    NegatePrec = 4,
    MaxPrec = 5,
}

#[derive(PartialEq, Eq)]
#[repr(u32)]
enum EvaluationCost {
    NeedsNothing = 0,
    NeedsInodeNumber = 1,
    NeedsType = 2,
    NeedsStatInfo = 3,
    NeedsLinkName = 4,
    NeedsAccessInfo = 5,
    NeedsSyncDiskHit = 6,
    NeedsEventualExec = 7,
    NeedsImmediateExec = 8,
    NeedsUserInteraction = 9,
    NeedsUnknown = 10,
    NumEvaluationCosts = 11,
}

#[derive(PartialEq, Eq)]
#[repr(u32)]
enum SegmentKind {
    Plain = 0,
    Stop = 1,
    Format = 2,
}

#[derive(PartialEq, Eq)]
#[repr(u32)]
enum PermissionsType {
    AtLeast = 0,
    Any = 1,
    Exact = 2,
}

#[derive(PartialEq, Eq)]
#[repr(u32)]
enum ComparisonType {
    Gt = 0,
    Lt = 1,
    Eq = 2,
}

#[derive(PartialEq, Eq)]
#[repr(u32)]
enum XVal {
    Atime = 0,
    Birthtime = 1,
    Ctime = 2,
    Mtime = 3,
    Time = 4,
}

#[derive(PartialEq, Eq)]
#[repr(u32)]
enum QuotingStyle {
    Literal = 0,
    Shell = 1,
    ShellAlways = 2,
    ShellEscape = 3,
    ShellEscapeAlways = 4,
    C = 5,
    CMaybe = 6,
    Escape = 7,
    Locale = 8,
    Clocale = 9,
    Custom = 10,
}

#[derive(PartialEq, Eq)]
#[repr(u32)]
enum SymlinkOption {
    NeverDeref = 0,
    AlwaysDeref = 1,
    DerefArgsonly = 2,
}

#[derive(PartialEq, Eq)]
#[repr(i32)]
enum DebugOption {
    None = 0,
    Stat = 2,
    Search = 4,
    Help = 16,
    Exec = 32,
    SuccessRates = 64,
    Time = 128,
    All = -17,
}

static mut predicates: *mut Predicate = ptr::null_mut();
static mut eval_tree: *mut Predicate = ptr::null_mut();
static mut last_pred: *mut Predicate = ptr::null_mut();
static mut start_points: *mut *mut c_char = ptr::null_mut();
static mut num_start_points: usize = 0;

static mut options: Options = Options {
    do_dir_first: false,
    explicit_depth: false,
    maxdepth: 0,
    mindepth: 0,
    no_leaf_check: false,
    stay_on_filesystem: false,
    ignore_readdir_race: false,
    literal_control_chars: false,
    warnings: false,
    posixly_correct: false,
    start_time: Timespec { tv_sec: 0, tv_nsec: 0 },
    cur_day_start: Timespec { tv_sec: 0, tv_nsec: 0 },
    full_days: false,
    output_block_size: 0,
    debug_options: 0,
    symlink_handling: SymlinkOption::NeverDeref,
    xstat: None,
    open_nofollow_available: false,
    regex_options: 0,
    x_getfilecon: None,
    optimisation_level: 0,
    err_quoting_style: QuotingStyle::Literal,
    files0_from: ptr::null(),
    ok_prompt_stdin: false,
};

static mut state: State = State {
    curdepth: 0,
    have_stat: false,
    have_type: false,
    type_: 0,
    rel_pathname: ptr::null(),
    cwd_dir_fd: 0,
    starting_path_length: 0,
    stop_at_current_level: false,
    exit_status: 0,
    execdirs_outstanding: false,
    shared_files: ptr::null_mut(),
    already_issued_stat_error_msg: false,
};

static mut cost_table: [CostAssoc; 11] = [
    CostAssoc {
        cost: EvaluationCost::NeedsNothing,
        name: b"Nothing\0".as_ptr() as *const c_char,
    },
    CostAssoc {
        cost: EvaluationCost::NeedsInodeNumber,
        name: b"InodeNumber\0".as_ptr() as *const c_char,
    },
    CostAssoc {
        cost: EvaluationCost::NeedsType,
        name: b"Type\0".as_ptr() as *const c_char,
    },
    CostAssoc {
        cost: EvaluationCost::NeedsStatInfo,
        name: b"StatInfo\0".as_ptr() as *const c_char,
    },
    CostAssoc {
        cost: EvaluationCost::NeedsLinkName,
        name: b"LinkName\0".as_ptr() as *const c_char,
    },
    CostAssoc {
        cost: EvaluationCost::NeedsAccessInfo,
        name: b"AccessInfo\0".as_ptr() as *const c_char,
    },
    CostAssoc {
        cost: EvaluationCost::NeedsSyncDiskHit,
        name: b"SyncDiskHit\0".as_ptr() as *const c_char,
    },
    CostAssoc {
        cost: EvaluationCost::NeedsEventualExec,
        name: b"EventualExec\0".as_ptr() as *const c_char,
    },
    CostAssoc {
        cost: EvaluationCost::NeedsImmediateExec,
        name: b"ImmediateExec\0".as_ptr() as *const c_char,
    },
    CostAssoc {
        cost: EvaluationCost::NeedsUserInteraction,
        name: b"UserInteraction\0".as_ptr() as *const c_char,
    },
    CostAssoc {
        cost: EvaluationCost::NeedsUnknown,
        name: b"Unknown\0".as_ptr() as *const c_char,
    },
];

static mut prec_table: [PrecAssoc; 7] = [
    PrecAssoc {
        prec: PredicatePrecedence::NoPrec as i16,
        prec_name: b"no\0".as_ptr() as *const c_char,
    },
    PrecAssoc {
        prec: PredicatePrecedence::CommaPrec as i16,
        prec_name: b"comma\0".as_ptr() as *const c_char,
    },
    PrecAssoc {
        prec: PredicatePrecedence::OrPrec as i16,
        prec_name: b"or\0".as_ptr() as *const c_char,
    },
    PrecAssoc {
        prec: PredicatePrecedence::AndPrec as i16,
        prec_name: b"and\0".as_ptr() as *const c_char,
    },
    PrecAssoc {
        prec: PredicatePrecedence::NegatePrec as i16,
        prec_name: b"negate\0".as_ptr() as *const c_char,
    },
    PrecAssoc {
        prec: PredicatePrecedence::MaxPrec as i16,
        prec_name: b"max\0".as_ptr() as *const c_char,
    },
    PrecAssoc {
        prec: -1,
        prec_name: b"unknown \0".as_ptr() as *const c_char,
    },
];

static mut type_table: [OpAssoc; 7] = [
    OpAssoc {
        type_: PredicateType::NoType as i16,
        type_name: b"no\0".as_ptr() as *const c_char,
    },
    OpAssoc {
        type_: PredicateType::PrimaryType as i16,
        type_name: b"primary\0".as_ptr() as *const c_char,
    },
    OpAssoc {
        type_: PredicateType::UniOp as i16,
        type_name: b"uni_op\0".as_ptr() as *const c_char,
    },
    OpAssoc {
        type_: PredicateType::BiOp as i16,
        type_name: b"bi_op\0".as_ptr() as *const c_char,
    },
    OpAssoc {
        type_: PredicateType::OpenParen as i16,
        type_name: b"open_paren  \0".as_ptr() as *const c_char,
    },
    OpAssoc {
        type_: PredicateType::CloseParen as i16,
        type_name: b"close_paren \0".as_ptr() as *const c_char,
    },
    OpAssoc {
        type_: -1,
        type_name: b"unknown\0".as_ptr() as *const c_char,
    },
];

// Additional helper functions would be implemented here