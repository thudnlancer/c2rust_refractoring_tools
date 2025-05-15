use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::mem;
use std::fmt;
use std::io::{self, Write};
use std::collections::HashMap;
use std::cmp::Ordering;
use std::str;
use std::env;
use std::cell::RefCell;
use std::rc::Rc;

type ArgpState = *mut argp_state;
type Argp = *const argp;
type ArgpOption = *const argp_option;
type ArgpChild = *const argp_child;
type FILE = *mut libc::FILE;

#[repr(C)]
pub struct argp_state {
    pub root_argp: Argp,
    pub argc: c_int,
    pub argv: *mut *mut c_char,
    pub next: c_int,
    pub flags: c_int,
    pub input: *mut c_void,
    pub arg_index: c_int,
    pub name: *mut c_char,
    pub err_stream: FILE,
    pub pstate: *mut c_void,
}

#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: Option<extern "C" fn(key: c_int, arg: *mut c_char, state: *mut argp_state) -> c_int>,
    pub args_doc: *const c_char,
    pub doc: *const c_char,
    pub children: *const argp_child,
    pub help_filter: Option<extern "C" fn(key: c_int, text: *const c_char, input: *mut c_void) -> *mut c_char>,
    pub argp_domain: *const c_char,
}

#[repr(C)]
pub struct argp_option {
    pub name: *const c_char,
    pub key: c_int,
    pub arg: *const c_char,
    pub flags: c_int,
    pub doc: *const c_char,
    pub group: c_int,
}

#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: c_int,
    pub header: *const c_char,
    pub group: c_int,
}

struct HolEntry {
    opt: *const argp_option,
    num: u32,
    short_options: String,
    group: i32,
    cluster: Option<Rc<RefCell<HolCluster>>>,
    argp: *const argp,
    ord: usize,
}

struct HolCluster {
    header: String,
    index: i32,
    group: i32,
    parent: Option<Rc<RefCell<HolCluster>>>,
    argp: *const argp,
    depth: i32,
    next: Option<Rc<RefCell<HolCluster>>>,
}

struct Hol {
    entries: Vec<HolEntry>,
    short_options: String,
    clusters: Option<Rc<RefCell<HolCluster>>>,
}

struct UpParams {
    dup_args: bool,
    dup_args_note: bool,
    short_opt_col: i32,
    long_opt_col: i32,
    doc_opt_col: i32,
    opt_doc_col: i32,
    header_col: i32,
    usage_indent: i32,
    rmargin: i32,
    valid: bool,
}

static mut UPARAMS: UpParams = UpParams {
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
};

struct UpParamName {
    name: &'static str,
    is_bool: bool,
    uparams_offs: usize,
}

static UPARAM_NAMES: &[UpParamName] = &[
    UpParamName { name: "dup-args", is_bool: true, uparams_offs: offset_of!(UpParams, dup_args) },
    UpParamName { name: "dup-args-note", is_bool: true, uparams_offs: offset_of!(UpParams, dup_args_note) },
    UpParamName { name: "short-opt-col", is_bool: false, uparams_offs: offset_of!(UpParams, short_opt_col) },
    UpParamName { name: "long-opt-col", is_bool: false, uparams_offs: offset_of!(UpParams, long_opt_col) },
    UpParamName { name: "doc-opt-col", is_bool: false, uparams_offs: offset_of!(UpParams, doc_opt_col) },
    UpParamName { name: "opt-doc-col", is_bool: false, uparams_offs: offset_of!(UpParams, opt_doc_col) },
    UpParamName { name: "header-col", is_bool: false, uparams_offs: offset_of!(UpParams, header_col) },
    UpParamName { name: "usage-indent", is_bool: false, uparams_offs: offset_of!(UpParams, usage_indent) },
    UpParamName { name: "rmargin", is_bool: false, uparams_offs: offset_of!(UpParams, rmargin) },
];

fn validate_uparams(state: *const argp_state, upptr: &UpParams) {
    for up in UPARAM_NAMES {
        if up.is_bool || up.uparams_offs == offset_of!(UpParams, rmargin) {
            continue;
        }
        unsafe {
            let val = *(upptr as *const UpParams as *const u8).add(up.uparams_offs) as i32;
            if val >= upptr.rmargin {
                __argp_failure(
                    state,
                    0,
                    0,
                    "ARGP_HELP_FMT: %s value is less than or equal to %s",
                    "rmargin",
                    up.name,
                );
                return;
            }
        }
    }
    unsafe {
        UPARAMS = *upptr;
        UPARAMS.valid = true;
    }
}

fn fill_in_uparams(state: *const argp_state) {
    let var = env::var("ARGP_HELP_FMT").unwrap_or_default();
    let mut new_params = unsafe { UPARAMS };

    if !var.is_empty() {
        let mut parts = var.split(',');
        for part in parts {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            if let Some((name, value)) = part.split_once('=') {
                let name = name.trim();
                let value = value.trim();

                if let Some(up) = UPARAM_NAMES.iter().find(|u| u.name == name) {
                    if up.is_bool {
                        __argp_failure(
                            state,
                            0,
                            0,
                            "%.*s: ARGP_HELP_FMT parameter requires a value",
                            name.len(),
                            name,
                        );
                    } else if let Ok(val) = value.parse::<i32>() {
                        if val < 0 {
                            __argp_failure(
                                state,
                                0,
                                0,
                                "%.*s: ARGP_HELP_FMT parameter must be positive",
                                name.len(),
                                name,
                            );
                        } else {
                            unsafe {
                                *(new_params as *mut UpParams as *mut u8)
                                    .add(up.uparams_offs) = val as u8;
                            }
                        }
                    }
                } else {
                    __argp_failure(
                        state,
                        0,
                        0,
                        "%.*s: Unknown ARGP_HELP_FMT parameter",
                        name.len(),
                        name,
                    );
                }
            } else {
                let name = part.trim();
                if let Some(up) = UPARAM_NAMES.iter().find(|u| u.name == name) {
                    if up.is_bool {
                        let val = if name.starts_with("no-") { false } else { true };
                        unsafe {
                            *(new_params as *mut UpParams as *mut u8)
                                .add(up.uparams_offs) = val as u8;
                        }
                    }
                } else {
                    __argp_failure(
                        state,
                        0,
                        0,
                        "Garbage in ARGP_HELP_FMT: %s",
                        name,
                    );
                }
            }
        }
        validate_uparams(state, &new_params);
    }
}

fn make_hol(argp: *const argp, cluster: Option<Rc<RefCell<HolCluster>>>) -> Hol {
    let opts = unsafe { (*argp).options };
    let mut hol = Hol {
        entries: Vec::new(),
        short_options: String::new(),
        clusters: None,
    };

    if !opts.is_null() {
        let mut cur_group = 0;
        let mut num_short_options = 0;

        // Calculate space needed
        let mut o = opts;
        unsafe {
            while !__option_is_end(o) {
                if !__option_is_alias(o) {
                    hol.entries.push(HolEntry {
                        opt: o,
                        num: 0,
                        short_options: String::new(),
                        group: 0,
                        cluster: cluster.clone(),
                        argp,
                        ord: 0,
                    });
                }
                if __option_is_short(o) {
                    num_short_options += 1;
                }
                o = o.add(1);
            }
        }

        // Fill in entries
        hol.short_options.reserve(num_short_options);
        let mut so_pos = 0;
        let mut o = opts;
        for entry in hol.entries.iter_mut() {
            entry.opt = o;
            entry.num = 0;
            entry.group = unsafe {
                if (*o).group != 0 {
                    (*o).group
                } else if (*o).name.is_null() && (*o).key == 0 {
                    cur_group += 1;
                    cur_group
                } else {
                    cur_group
                }
            };
            entry.cluster = cluster.clone();
            entry.argp = argp;

            unsafe {
                while !__option_is_end(o) && __option_is_alias(o) {
                    entry.num += 1;
                    if __option_is_short(o) && !hol.short_options.contains((*o).key as u8 as char) {
                        hol.short_options.push((*o).key as u8 as char);
                    }
                    o = o.add(1);
                }
            }
        }
    }

    hol
}

fn hol_add_cluster(
    hol: &mut Hol,
    group: i32,
    header: &str,
    index: i32,
    parent: Option<Rc<RefCell<HolCluster>>>,
    argp: *const argp,
) -> Option<Rc<RefCell<HolCluster>>> {
    let depth = parent.as_ref().map_or(0, |p| p.borrow().depth + 1);
    let cluster = Rc::new(RefCell::new(HolCluster {
        header: header.to_string(),
        index,
        group,
        parent: parent.clone(),
        argp,
        depth,
        next: None,
    }));

    if let Some(mut cl) = hol.clusters.take() {
        let mut current = cl.clone();
        while current.borrow().next.is_some() {
            let next = current.borrow().next.clone().unwrap();
            current = next;
        }
        current.borrow_mut().next = Some(cluster.clone());
        hol.clusters = Some(cl);
    } else {
        hol.clusters = Some(cluster.clone());
    }

    Some(cluster)
}

fn hol_free(hol: Hol) {
    // Rust's drop will automatically clean up the memory
}

fn hol_entry_short_iterate<F>(
    entry: &HolEntry,
    mut func: F,
    domain: &str,
    cookie: &mut dyn std::any::Any,
) -> i32
where
    F: FnMut(*const argp_option, *const argp_option, &str, &mut dyn std::any::Any) -> i32,
{
    let mut val = 0;
    let mut num = entry.num;
    let mut opt = entry.opt;
    let mut so = entry.short_options.chars();

    unsafe {
        while num > 0 && val == 0 {
            if __option_is_short(opt) && so.next() == Some((*opt).key as u8 as char) {
                let real = if __option_is_alias(opt) {
                    entry.opt
                } else {
                    opt
                };
                if __option_is_visible(opt) {
                    val = func(opt, real, domain, cookie);
                }
            }
            opt = opt.add(1);
            num -= 1;
        }
    }

    val
}

fn hol_entry_first_short(entry: &HolEntry) -> char {
    let mut result = 0;
    let mut cookie = &mut result as *mut _ as *mut dyn std::any::Any;
    hol_entry_short_iterate(
        entry,
        |opt, _real, _domain, cookie| {
            unsafe {
                *(cookie as *mut i32) = (*opt).key;
            }
            (*opt).key
        },
        unsafe { CStr::from_ptr((*entry.argp).argp_domain) }
            .to_str()
            .unwrap_or(""),
        unsafe { &mut *cookie },
    );
    result as u8 as char
}

fn hol_entry_first_long(entry: &HolEntry) -> Option<String> {
    unsafe {
        let mut opt = entry.opt;
        let mut num = entry.num;
        while num > 0 {
            if !(*opt).name.is_null() && __option_is_visible(opt) {
                return Some(CStr::from_ptr((*opt).name).to_str().unwrap().to_string();
            }
            opt = opt.add(1);
            num -= 1;
        }
    }
    None
}

fn hol_find_entry(hol: &Hol, name: &str) -> Option<&HolEntry> {
    for entry in &hol.entries {
        unsafe {
            let mut opt = entry.opt;
            let mut num = entry.num;
            while num > 0 {
                if !(*opt).name.is_null()
                    && __option_is_visible(opt)
                    && CStr::from_ptr((*opt).name).to_str().unwrap() == name
                {
                    return Some(entry);
                }
                opt = opt.add(1);
                num -= 1;
            }
        }
    }
    None
}

fn hol_set_group(hol: &mut Hol, name: &str, group: i32) {
    if let Some(entry) = hol_find_entry(hol, name) {
        // Need to get mutable reference - this is simplified for example
        // In real code would need RefCell or similar
        unsafe {
            let entry_ptr = entry as *const HolEntry as *mut HolEntry;
            (*entry_ptr).group = group;
        }
    }
}

fn hol_sort(hol: &mut Hol) {
    if !hol.entries.is_empty() {
        for (i, entry) in hol.entries.iter_mut().enumerate() {
            entry.ord = i;
        }
        hol.entries.sort_by(|a, b| {
            // Simplified comparison - actual implementation would be more complex
            a.group.cmp(&b.group).then(a.ord.cmp(&b.ord))
        });
    }
}

fn hol_append(hol: &mut Hol, more: Hol) {
    // Merge clusters
    let mut cl_end = &mut hol.clusters;
    while let Some(cl) = cl_end {
        cl_end = &mut cl.borrow_mut().next;
    }
    *cl_end = more.clusters;

    // Merge entries
    if !more.entries.is_empty() {
        if hol.entries.is_empty() {
            hol.entries = more.entries;
            hol.short_options = more.short_options;
        } else {
            let mut entries = Vec::with_capacity(hol.entries.len() + more.entries.len());
            entries.extend_from_slice(&hol.entries);
            entries.extend_from_slice(&more.entries);

            let mut short_options = hol.short_options.clone();
            let more_so = more.short_options.chars().collect::<Vec<_>>();
            let mut so_pos = short_options.len();

            for entry in entries.iter_mut().skip(hol.entries.len()) {
                let mut new_so = String::new();
                unsafe {
                    let mut opt = entry.opt;
                    let mut num = entry.num;
                    while num > 0 {
                        if __option_is_short(opt) {
                            let ch = (*opt).key as u8 as char;
                            if !short_options.contains(ch) {
                                new_so.push(ch);
                            }
                        }
                        opt = opt.add(1);
                        num -= 1;
                    }
                }
                entry.short_options = new_so;
                short_options.push_str(&entry.short_options);
            }

            hol.entries = entries;
            hol.short_options = short_options;
        }
    }
}

fn argp_hol(argp: *const argp, cluster: Option<Rc<RefCell<HolCluster>>>) -> Hol {
    let mut hol = make_hol(argp, cluster.clone());
    unsafe {
        if !(*argp).children.is_null() {
            let mut child = (*argp).children;
            while !(*child).argp.is_null() {
                let child_cluster = if (*child).group != 0 || !(*child).header.is_null() {
                    hol_add_cluster(
                        &mut hol,
                        (*child).group,
                        CStr::from_ptr((*child).header).to_str().unwrap(),
                        child as usize as i32 - (*argp).children as usize as i32,
                        cluster.clone(),
                        argp,
                    )
                } else {
                    cluster.clone()
                };
                hol_append(&mut hol, argp_hol((*child).argp, child_cluster));
                child = child.add(1);
            }
        }
    }
    hol
}

fn __argp_help(
    argp: *const argp,
    stream: *mut libc::FILE,
    flags: u32,
    name: *const c_char,
) {
    let mut state = argp_state {
        root_argp: argp,
        argc: 0,
        argv: ptr::null_mut(),
        next: 0,
        flags: 0,
        input: