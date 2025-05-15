use std::cmp::Ordering;
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_uint, c_ulong, c_void};
use std::ptr;

type size_t = c_ulong;
type uintmax_t = c_ulong;

#[derive(Debug, Clone, Copy)]
struct File {
    name: *const c_char,
    hname: *const c_char,
    vpath: *const c_char,
    deps: *mut Dep,
    cmds: *mut Commands,
    stem: *const c_char,
    also_make: *mut Dep,
    prev: *mut File,
    last: *mut File,
    renamed: *mut File,
    variables: *mut VariableSetList,
    pat_variables: *mut VariableSetList,
    parent: *mut File,
    double_colon: *mut File,
    last_mtime: uintmax_t,
    mtime_before_update: uintmax_t,
    considered: c_uint,
    command_flags: c_int,
    flags: u32,
}

#[derive(Debug, Clone, Copy)]
struct Dep {
    next: *mut Dep,
    name: *const c_char,
    file: *mut File,
    shuf: *mut Dep,
    stem: *const c_char,
    flags: u16,
}

#[derive(Debug, Clone, Copy)]
struct Commands {
    fileinfo: Floc,
    commands: *mut c_char,
    command_lines: *mut *mut c_char,
    lines_flags: *mut u8,
    ncommand_lines: u16,
    recipe_prefix: c_char,
    any_recurse: u8,
}

#[derive(Debug, Clone, Copy)]
struct Floc {
    filenm: *const c_char,
    lineno: c_ulong,
    offset: c_ulong,
}

#[derive(Debug, Clone, Copy)]
struct VariableSetList {
    next: *mut VariableSetList,
    set: *mut VariableSet,
    next_is_parent: c_int,
}

#[derive(Debug, Clone, Copy)]
struct VariableSet {
    table: HashTable,
}

#[derive(Debug, Clone, Copy)]
struct HashTable {
    ht_vec: *mut *mut c_void,
    ht_hash_1: Option<unsafe extern "C" fn(*const c_void) -> c_ulong>,
    ht_hash_2: Option<unsafe extern "C" fn(*const c_void) -> c_ulong>,
    ht_compare: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ht_size: c_ulong,
    ht_capacity: c_ulong,
    ht_fill: c_ulong,
    ht_empty_slots: c_ulong,
    ht_collisions: c_ulong,
    ht_lookups: c_ulong,
    ht_rehashes: c_uint,
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    next: *mut Rule,
    targets: *mut *const c_char,
    lens: *mut c_uint,
    suffixes: *mut *const c_char,
    deps: *mut Dep,
    cmds: *mut Commands,
    _defn: *mut c_char,
    num: u16,
    terminal: c_char,
    in_use: c_char,
}

#[derive(Debug, Clone, Copy)]
struct PatDeps {
    name: *const c_char,
    pattern: *const c_char,
    file: *mut File,
    flags: u8,
}

#[derive(Debug, Clone, Copy)]
struct TryRule {
    rule: *mut Rule,
    stemlen: size_t,
    matches: c_uint,
    order: c_uint,
    checked_lastslash: c_char,
}

#[derive(Debug, Clone, Copy)]
struct Nameseq {
    next: *mut Nameseq,
    name: *const c_char,
}

static mut stdout: *mut libc::FILE = ptr::null_mut();
static mut stopchar_map: [u16; 0] = [];
static mut no_intermediates: c_uint = 0;
static mut pattern_rules: *mut Rule = ptr::null_mut();
static mut num_pattern_rules: c_uint = 0;
static mut max_pattern_deps: c_uint = 0;
static mut max_pattern_targets: c_uint = 0;
static mut max_pattern_dep_length: size_t = 0;
static mut db_level: c_int = 0;

extern "C" {
    fn fflush(stream: *mut libc::FILE) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn qsort(
        base: *mut c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    );
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memrchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn mempcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn dcgettext(domainname: *const c_char, msgid: *const c_char, category: c_int) -> *mut c_char;
    fn xmalloc(size: size_t) -> *mut c_void;
    fn xcalloc(size: size_t) -> *mut c_void;
    fn xrealloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn lindex(s: *const c_char, e: *const c_char, c: c_int) -> *mut c_char;
    fn print_spaces(n: c_uint);
    fn ar_name(name: *const c_char) -> c_int;
    fn file_exists_p(name: *const c_char) -> c_int;
    fn file_impossible_p(name: *const c_char) -> c_int;
    fn file_impossible(name: *const c_char);
    fn vpath_search(
        file: *const c_char,
        mtime_ptr: *mut uintmax_t,
        vpath_index: *mut c_uint,
        path_index: *mut c_uint,
    ) -> *const c_char;
    fn strcache_add(str: *const c_char) -> *const c_char;
    fn strcache_add_len(str: *const c_char, len: size_t) -> *const c_char;
    fn lookup_file(name: *const c_char) -> *mut File;
    fn enter_file(name: *const c_char) -> *mut File;
    fn get_rule_defn(rule: *mut Rule) -> *const c_char;
    fn parse_file_seq(
        stringp: *mut *mut c_char,
        size: size_t,
        stopmap: c_int,
        prefix: *const c_char,
        flags: c_int,
    ) -> *mut c_void;
    fn free_ns_chain(n: *mut Nameseq);
    fn variable_expand_for_file(line: *const c_char, file: *mut File) -> *mut c_char;
    fn free_variable_set(set: *mut VariableSetList);
    fn initialize_file_variables(file: *mut File, reading: c_int);
    fn merge_variable_set_lists(
        to_list: *mut *mut VariableSetList,
        from_list: *mut VariableSetList,
    );
    fn define_variable_in_set(
        name: *const c_char,
        length: size_t,
        value: *const c_char,
        origin: c_int,
        recursive: c_int,
        set: *mut VariableSet,
        flocp: *const Floc,
    ) -> *mut Variable;
    fn set_file_variables(file: *mut File, stem: *const c_char);
    fn shuffle_deps_recursive(g: *mut Dep);
}

pub fn try_implicit_rule(file: *mut File, depth: c_uint) -> c_int {
    unsafe {
        if db_level & 0x8 != 0 {
            print_spaces(depth);
            printf(
                dcgettext(
                    ptr::null(),
                    b"Looking for an implicit rule for '%s'.\n\0".as_ptr() as *const c_char,
                    5,
                ),
                (*file).name,
            );
            fflush(stdout);
        }

        if pattern_search(file, 0, depth, 0, 0) != 0 {
            return 1;
        }

        if ar_name((*file).name) != 0 {
            if db_level & 0x8 != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        ptr::null(),
                        b"Looking for archive-member implicit rule for '%s'.\n\0".as_ptr()
                            as *const c_char,
                        5,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }

            if pattern_search(file, 1, depth, 0, 0) != 0 {
                return 1;
            }

            if db_level & 0x8 != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        ptr::null(),
                        b"No archive-member implicit rule found for '%s'.\n\0".as_ptr()
                            as *const c_char,
                        5,
                    ),
                    (*file).name,
                );
                fflush(stdout);
            }
        }

        0
    }
}

fn stemlen_compare(v1: *const c_void, v2: *const c_void) -> c_int {
    unsafe {
        let r1 = &*(v1 as *const TryRule);
        let r2 = &*(v2 as *const TryRule);
        let r = (r1.stemlen as isize - r2.stemlen as isize) as c_int;
        if r != 0 {
            r
        } else {
            (r1.order as isize - r2.order as isize) as c_int
        }
    }
}

fn pattern_search(
    file: *mut File,
    archive: c_int,
    depth: c_uint,
    recursions: c_uint,
    allow_compat_rules: c_int,
) -> c_int {
    unsafe {
        let filename = if archive != 0 {
            strchr((*file).name, b'(' as i32)
        } else {
            (*file).name
        };
        let namelen = strlen(filename);
        let lastslash = if archive != 0 || ar_name(filename) != 0 {
            ptr::null()
        } else {
            memrchr(
                filename as *const c_void,
                b'/' as i32,
                namelen.wrapping_sub(1),
            ) as *const c_char
        };

        let pathlen = if !lastslash.is_null() {
            lastslash.offset_from(filename) as isize + 1
        } else {
            0
        } as size_t;

        let mut max_deps = max_pattern_deps;
        let mut deplist = xmalloc(max_deps as size_t * mem::size_of::<PatDeps>() as size_t)
            as *mut PatDeps;
        let mut pat = deplist;
        let deplen = namelen
            .wrapping_add(max_pattern_dep_length)
            .wrapping_add(4);
        let mut depname: Vec<c_char> = vec![0; deplen as usize];
        let mut stem: *const c_char = ptr::null();
        let mut stemlen = 0;
        let mut fullstemlen = 0;

        let mut tryrules = xmalloc(
            (num_pattern_rules * max_pattern_targets) as size_t
                * mem::size_of::<TryRule>() as size_t,
        ) as *mut TryRule;
        let mut nrules = 0;
        let mut foundrule = 0;
        let mut intermed_ok = 0;
        let mut file_vars_initialized = 0;
        let mut specific_rule_matched = 0;
        let mut found_compat_rule = 0;
        let mut rule = pattern_rules;
        let mut pathdir: *mut c_char = ptr::null_mut();
        let mut stem_str: [c_char; 4097] = [0; 4097];
        let mut int_file: *mut File = ptr::null_mut();

        while !rule.is_null() {
            if !((*rule).deps.is_null() && (*rule).cmds.is_null()) {
                if (*rule).in_use != 0 {
                    if db_level & 0x8 != 0 {
                        print_spaces(depth);
                        printf(
                            dcgettext(
                                ptr::null(),
                                b"Avoiding implicit rule recursion for rule '%s'.\n\0".as_ptr()
                                    as *const c_char,
                                5,
                            ),
                            get_rule_defn(rule),
                        );
                        fflush(stdout);
                    }
                } else {
                    for ti in 0..(*rule).num as usize {
                        let target = *(*rule).targets.add(ti);
                        let suffix = *(*rule).suffixes.add(ti);
                        let check_lastslash = if !lastslash.is_null() {
                            strchr(target, b'/' as i32).is_null() as c_char
                        } else {
                            0
                        };

                        if !(recursions > 0
                            && *target.add(1) == 0
                            && (*rule).terminal == 0)
                        {
                            if *(*rule).lens.add(ti) as size_t <= namelen {
                                stem = filename
                                    .offset((suffix.offset_from(target) as isize - 1));
                                stemlen = namelen
                                    .wrapping_sub(*(*rule).lens.add(ti) as size_t)
                                    .wrapping_add(1);

                                if check_lastslash != 0 {
                                    if pathlen > stemlen {
                                        continue;
                                    }
                                    stemlen -= pathlen;
                                    stem = stem.offset(pathlen as isize);
                                }

                                if check_lastslash != 0 {
                                    if stem > lastslash.offset(1)
                                        && strncmp(
                                            target,
                                            lastslash.offset(1),
                                            stem.offset_from(lastslash) as isize - 1,
                                        ) != 0
                                    {
                                        continue;
                                    }
                                } else if stem > filename
                                    && strncmp(
                                        target,
                                        filename,
                                        stem.offset_from(filename) as isize,
                                    ) != 0
                                {
                                    continue;
                                }

                                if *suffix != *stem.add(stemlen as isize)
                                    || (*suffix != 0
                                        && !(*suffix.add(1) == *stem.add(stemlen + 1)
                                            || (*suffix.add(1) == 0
                                                && strcmp(
                                                    suffix.add(2),
                                                    stem.add(stemlen + 2),
                                                ) == 0)))
                                {
                                    if *target.add(1) != 0 {
                                        specific_rule_matched = 1;
                                    }
                                    if !((*rule).deps.is_null() && (*rule).cmds.is_null()) {
                                        (*tryrules.add(nrules as isize)).rule = rule;
                                        (*tryrules.add(nrules as isize)).matches = ti as c_uint;
                                        (*tryrules.add(nrules as isize)).stemlen = stemlen
                                            + if check_lastslash != 0 {
                                                pathlen
                                            } else {
                                                0
                                            };
                                        (*tryrules.add(nrules as isize)).order = nrules;
                                        (*tryrules.add(nrules as isize)).checked_lastslash =
                                            check_lastslash;
                                        nrules += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            rule = (*rule).next;
        }

        if nrules == 0 {
            free(tryrules as *mut c_void);
            free(deplist as *mut c_void);
            return 0;
        }

        if nrules > 1 {
            qsort(
                tryrules as *mut c_void,
                nrules as size_t,
                mem::size_of::<TryRule>() as size_t,
                Some(stemlen_compare),
            );
        }

        if specific_rule_matched != 0 {
            for ri in 0..nrules {
                if (*(*tryrules.add(ri as isize)).rule).terminal == 0 {
                    for j in 0..(*(*tryrules.add(ri as isize)).rule).num as usize {
                        if *(*((*(*tryrules.add(ri as isize)).rule).targets.add(j))
                            .add(1)
                            == 0
                        {
                            (*tryrules.add(ri as isize)).rule = ptr::null_mut();
                            break;
                        }
                    }
                }
            }
        }

        let mut result = 0;
        'outer: while intermed_ok < 2 {
            pat = deplist;
            if intermed_ok != 0 && db_level & 0x8 != 0 {
                print_spaces(depth);
                printf(
                    dcgettext(
                        ptr::null(),
                        b"Trying harder.\n\0".as_ptr() as *const c_char,
                        5,
                    ),
                );
                fflush(stdout);
            }

            for ri in 0..nrules {
                let mut failed = 0;
                let mut file_variables_set = 0;
                let mut deps_found = 0;
                let mut order_only = 0;

                rule = (*tryrules.add(