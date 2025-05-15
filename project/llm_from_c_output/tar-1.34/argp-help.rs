use std::{
    ffi::{CStr, CString},
    fmt::{self, Write},
    io::{self, Write as IoWrite},
    mem,
    os::raw::c_char,
    ptr,
    str,
};

const ARGP_HELP_FMT: &str = "ARGP_HELP_FMT";
const ARGP_KEY_HELP_HEADER: i32 = 0;
const ARGP_KEY_HELP_DUP_ARGS_NOTE: i32 = 1;
const ARGP_KEY_HELP_ARGS_DOC: i32 = 2;
const ARGP_KEY_HELP_PRE_DOC: i32 = 3;
const ARGP_KEY_HELP_POST_DOC: i32 = 4;
const ARGP_KEY_HELP_EXTRA: i32 = 5;

struct UpParams {
    dup_args: bool,
    dup_args_note: bool,
    short_opt_col: usize,
    long_opt_col: usize,
    doc_opt_col: usize,
    opt_doc_col: usize,
    header_col: usize,
    usage_indent: usize,
    rmargin: usize,
    valid: bool,
}

impl Default for UpParams {
    fn default() -> Self {
        Self {
            dup_args: false,
            dup_args_note: true,
            short_opt_col: 2,
            long_opt_col: 6,
            doc_opt_col: 2,
            opt_doc_col: 29,
            header_col: 1,
            usage_indent: 12,
            rmargin: 79,
            valid: false,
        }
    }
}

struct HolEntry {
    opt: *const argp_option,
    num: u32,
    short_options: *mut c_char,
    group: i32,
    cluster: *mut HolCluster,
    argp: *const argp,
    ord: u32,
}

struct HolCluster {
    header: *const c_char,
    index: i32,
    group: i32,
    parent: *mut HolCluster,
    argp: *const argp,
    depth: i32,
    next: *mut HolCluster,
}

struct Hol {
    entries: *mut HolEntry,
    num_entries: u32,
    short_options: *mut c_char,
    clusters: *mut HolCluster,
}

fn validate_uparams(state: *const argp_state, upptr: *mut UpParams) {
    unsafe {
        let uparams_names = [
            ("dup-args", true, offset_of!(UpParams, dup_args)),
            ("dup-args-note", true, offset_of!(UpParams, dup_args_note)),
            ("short-opt-col", false, offset_of!(UpParams, short_opt_col)),
            ("long-opt-col", false, offset_of!(UpParams, long_opt_col)),
            ("doc-opt-col", false, offset_of!(UpParams, doc_opt_col)),
            ("opt-doc-col", false, offset_of!(UpParams, opt_doc_col)),
            ("header-col", false, offset_of!(UpParams, header_col)),
            ("usage-indent", false, offset_of!(UpParams, usage_indent)),
            ("rmargin", false, offset_of!(UpParams, rmargin)),
        ];

        for (name, is_bool, offset) in &uparams_names {
            if *is_bool || offset == &offset_of!(UpParams, rmargin) {
                continue;
            }
            let val = *(upptr as *const u8).add(*offset) as usize;
            if val >= (*upptr).rmargin {
                argp_failure(
                    state,
                    0,
                    0,
                    "ARGP_HELP_FMT: %s value is less than or equal to %s",
                    "rmargin",
                    *name,
                );
                return;
            }
        }
        (*upptr).valid = true;
    }
}

fn fill_in_uparams(state: *const argp_state) {
    unsafe {
        let mut new_params = UpParams::default();
        if let Ok(var) = std::env::var(ARGP_HELP_FMT) {
            let mut var = var.as_str();
            while !var.is_empty() {
                var = var.trim_start();
                if var.chars().next().map_or(false, |c| c.is_alphabetic()) {
                    let name_end = var
                        .find(|c: char| !c.is_alphanumeric() && c != '-' && c != '_')
                        .unwrap_or(var.len());
                    let name = &var[..name_end];
                    var = &var[name_end..];
                    var = var.trim_start();

                    let (unspec, val) = if var.is_empty() || var.starts_with(',') {
                        (true, if name.starts_with("no-") { 0 } else { 1 })
                    } else if var.starts_with('=') {
                        var = &var[1..];
                        var = var.trim_start();
                        if let Some(digit_end) = var.find(|c: char| !c.is_digit(10)) {
                            let val = var[..digit_end].parse().unwrap_or(0);
                            var = &var[digit_end..];
                            (false, val)
                        } else {
                            (false, 0)
                        }
                    } else {
                        (true, 0)
                    };

                    var = var.trim_start();

                    for (up_name, is_bool, offset) in &[
                        ("dup-args", true, offset_of!(UpParams, dup_args)),
                        ("dup-args-note", true, offset_of!(UpParams, dup_args_note)),
                        ("short-opt-col", false, offset_of!(UpParams, short_opt_col)),
                        ("long-opt-col", false, offset_of!(UpParams, long_opt_col)),
                        ("doc-opt-col", false, offset_of!(UpParams, doc_opt_col)),
                        ("opt-doc-col", false, offset_of!(UpParams, opt_doc_col)),
                        ("header-col", false, offset_of!(UpParams, header_col)),
                        ("usage-indent", false, offset_of!(UpParams, usage_indent)),
                        ("rmargin", false, offset_of!(UpParams, rmargin)),
                    ] {
                        if name == *up_name {
                            if unspec && !is_bool {
                                argp_failure(
                                    state,
                                    0,
                                    0,
                                    "%.*s: ARGP_HELP_FMT parameter requires a value",
                                    name.len(),
                                    name,
                                );
                            } else {
                                let ptr = (&mut new_params as *mut UpParams as *mut u8)
                                    .add(*offset);
                                if *is_bool {
                                    *(ptr as *mut bool) = val != 0;
                                } else {
                                    *(ptr as *mut usize) = val;
                                }
                            }
                            break;
                        }
                    }

                    if var.starts_with(',') {
                        var = &var[1..];
                    }
                } else if !var.is_empty() {
                    argp_failure(
                        state,
                        0,
                        0,
                        "Garbage in ARGP_HELP_FMT: %s",
                        var,
                    );
                    break;
                }
            }
            validate_uparams(state, &mut new_params);
        }
    }
}

fn make_hol(argp: *const argp, cluster: *mut HolCluster) -> *mut Hol {
    unsafe {
        let opts = (*argp).options;
        let mut num_entries = 0;
        let mut num_short_options = 0;

        if !opts.is_null() {
            let mut o = opts;
            while !(*o).name.is_null() || (*o).key != 0 {
                if !((*o).flags & OPTION_ALIAS != 0) {
                    num_entries += 1;
                }
                if (*o).key != 0 {
                    num_short_options += 1;
                }
                o = o.add(1);
            }

            let hol = Box::into_raw(Box::new(Hol {
                entries: vec![HolEntry {
                    opt: ptr::null(),
                    num: 0,
                    short_options: ptr::null_mut(),
                    group: 0,
                    cluster: ptr::null_mut(),
                    argp: ptr::null(),
                    ord: 0,
                }]
                .into_boxed_slice()
                .as_mut_ptr(),
                num_entries,
                short_options: vec![0; num_short_options + 1].into_boxed_slice().as_mut_ptr(),
                clusters: ptr::null_mut(),
            }));

            let mut so = (*hol).short_options;
            let mut entry = (*hol).entries;
            let mut o = opts;
            let mut cur_group = 0;

            while !(*o).name.is_null() || (*o).key != 0 {
                (*entry).opt = o;
                (*entry).num = 0;
                (*entry).short_options = so;
                (*entry).group = if (*o).group != 0 {
                    (*o).group
                } else if (*o).name.is_null() && (*o).key == 0 {
                    cur_group += 1;
                    cur_group
                } else {
                    cur_group
                };
                (*entry).cluster = cluster;
                (*entry).argp = argp;

                loop {
                    (*entry).num += 1;
                    if (*o).key != 0 && !find_char((*o).key, (*hol).short_options, so) {
                        *so = (*o).key as c_char;
                        so = so.add(1);
                    }
                    o = o.add(1);
                    if (*o).name.is_null() && (*o).key == 0 || (*o).flags & OPTION_ALIAS == 0 {
                        break;
                    }
                }

                entry = entry.add(1);
            }

            *so = 0;
            hol
        } else {
            Box::into_raw(Box::new(Hol {
                entries: ptr::null_mut(),
                num_entries: 0,
                short_options: ptr::null_mut(),
                clusters: ptr::null_mut(),
            }))
        }
    }
}

fn hol_add_cluster(
    hol: *mut Hol,
    group: i32,
    header: *const c_char,
    index: i32,
    parent: *mut HolCluster,
    argp: *const argp,
) -> *mut HolCluster {
    unsafe {
        let cl = Box::into_raw(Box::new(HolCluster {
            header,
            index,
            group,
            parent,
            argp,
            depth: if !parent.is_null() { (*parent).depth + 1 } else { 0 },
            next: ptr::null_mut(),
        }));

        let mut cl_end = &mut (*hol).clusters;
        while !(*cl_end).is_null() {
            cl_end = &mut (**cl_end).next;
        }
        *cl_end = cl;
        cl
    }
}

fn hol_free(hol: *mut Hol) {
    unsafe {
        let mut cl = (*hol).clusters;
        while !cl.is_null() {
            let next = (*cl).next;
            drop(Box::from_raw(cl));
            cl = next;
        }

        if (*hol).num_entries > 0 {
            drop(Box::from_raw((*hol).entries));
            drop(Box::from_raw((*hol).short_options));
        }

        drop(Box::from_raw(hol));
    }
}

fn hol_entry_short_iterate(
    entry: *const HolEntry,
    func: Option<unsafe extern "C" fn(*const argp_option, *const argp_option, *const c_char, *mut std::ffi::c_void) -> i32>,
    domain: *const c_char,
    cookie: *mut std::ffi::c_void,
) -> i32 {
    unsafe {
        let mut val = 0;
        let mut nopts = (*entry).num;
        let mut opt = (*entry).opt;
        let mut real = (*entry).opt;
        let mut so = (*entry).short_options;

        while nopts > 0 && val == 0 {
            if (*opt).key != 0 && *so == (*opt).key as c_char {
                if (*opt).flags & OPTION_ALIAS == 0 {
                    real = opt;
                }
                if (*opt).flags & OPTION_HIDDEN == 0 {
                    if let Some(f) = func {
                        val = f(opt, real, domain, cookie);
                    }
                }
                so = so.add(1);
            }
            opt = opt.add(1);
            nopts -= 1;
        }

        val
    }
}

fn hol_entry_first_short(entry: *const HolEntry) -> c_char {
    unsafe {
        let mut val = 0;
        hol_entry_short_iterate(
            entry,
            Some(
                |opt: *const argp_option,
                 _real: *const argp_option,
                 _domain: *const c_char,
                 cookie: *mut std::ffi::c_void| {
                    if (*opt).arg.is_null() {
                        *(cookie as *mut c_char) = (*opt).key as c_char;
                        1
                    } else {
                        0
                    }
                },
            ),
            ptr::null(),
            &mut val as *mut _ as *mut std::ffi::c_void,
        );
        val
    }
}

fn hol_entry_first_long(entry: *const HolEntry) -> *const c_char {
    unsafe {
        let mut num = (*entry).num;
        let mut opt = (*entry).opt;
        while num > 0 {
            if !(*opt).name.is_null() && (*opt).flags & OPTION_HIDDEN == 0 {
                return (*opt).name;
            }
            opt = opt.add(1);
            num -= 1;
        }
        ptr::null()
    }
}

fn hol_find_entry(hol: *const Hol, name: *const c_char) -> *mut HolEntry {
    unsafe {
        let mut entry = (*hol).entries;
        let mut num_entries = (*hol).num_entries;

        while num_entries > 0 {
            let mut opt = (*entry).opt;
            let mut num_opts = (*entry).num;

            while num_opts > 0 {
                if !(*opt).name.is_null()
                    && (*opt).flags & OPTION_HIDDEN == 0
                    && libc::strcmp((*opt).name, name) == 0
                {
                    return entry;
                }
                opt = opt.add(1);
                num_opts -= 1;
            }

            entry = entry.add(1);
            num_entries -= 1;
        }

        ptr::null_mut()
    }
}

fn hol_set_group(hol: *mut Hol, name: *const c_char, group: i32) {
    unsafe {
        if let Some(entry) = hol_find_entry(hol, name).as_mut() {
            (*entry).group = group;
        }
    }
}

fn hol_sort(hol: *mut Hol) {
    unsafe {
        if (*hol).num_entries > 0 {
            let mut entry = (*hol).entries;
            let mut i = 0;
            while i < (*hol).num_entries {
                (*entry).ord = i;
                entry = entry.add(1);
                i += 1;
            }

            let slice = std::slice::from_raw_parts_mut((*hol).entries, (*hol).num_entries as usize);
            slice.sort_by(|a, b| hol_entry_cmp(a, b));
        }
    }
}

fn hol_entry_cmp(a: *const HolEntry, b: *const HolEntry) -> std::cmp::Ordering {
    unsafe {
        let group_a = if (*a).cluster.is_null() {
            (*a).group
        } else {
            hol_cluster_base((*a).cluster).group
        };
        let group_b = if (*b).cluster.is_null() {
            (*b).group
        } else {
            hol_cluster_base((*b).cluster).group
        };

        match group_a.cmp(&group_b) {
            std::cmp::Ordering::Equal => {
                let cluster_a = (*a).cluster.is_null();
                let cluster_b = (*b).cluster.is_null();
                match cluster_a.cmp(&cluster_b) {
                    std::cmp::Ordering::Equal => {
                        if !(*a).cluster.is_null() {
                            match hol_cluster_cmp((*a).cluster, (*b).cluster) {
                                std::cmp::Ordering::Equal => (*a).group.cmp(&(*b).group),
                                ord => ord,
                            }
                        } else {
                            (*a).group.cmp(&(*b).group)
                        }
                    }
                    ord => ord,
                }
            }
            ord => ord,
        }
    }
}

fn hol_cluster_cmp(a: *const HolCluster, b: *const HolCluster) -> std::cmp::Ordering {
    unsafe {
        let a_depth = (*a).depth;
        let b_depth = (*b).depth;

        if a_depth > b_depth {
            let mut a = a;
            while (*a).depth > b_depth {
                a = (*a).parent;
            }
            match hol_cousin_cluster_cmp(a, b) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Greater,
                ord => ord,
            }
        } else if a_depth < b_depth {
            let mut b = b;
            while a_depth < (*b).depth {
                b = (*b).parent;
            }
            match hol_cousin_cluster_cmp(a, b) {
                std::cmp::Ordering::Equal => std::cmp::Ordering::Less,
                ord => ord,
            }
        } else {
            hol_cousin_cluster_cmp(a, b)
        }
    }
}

fn hol_cousin_cluster_cmp(a: *const HolCluster, b: *const HolCluster) -> std::cmp::Ordering {
    unsafe {
        if (*a).parent == (*b).parent {
            hol_sibling_cluster_cmp(a, b)
        } else {
            match hol_cousin_cluster_cmp((*a).parent, (*b).parent) {
                std::cmp::Ordering::Equal => match (*a).group.cmp(&(*b).group) {
                    std::cmp::Ordering::Equal => (*b).index.cmp(&(*a).index),
                    ord => ord,
                },
                ord => ord,
            }
        }
    }
}

fn hol_sibling_cluster_cmp(a: *