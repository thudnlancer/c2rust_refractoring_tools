use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::mem;
use std::cmp::Ordering;
use std::collections::HashMap;

struct File {
    name: *const c_char,
    is_target: bool,
    is_explicit: bool,
    intermediate: bool,
    notintermediate: bool,
    precious: bool,
    tried_implicit: bool,
    was_shuffled: bool,
    deps: *mut Dep,
    cmds: *mut Commands,
    stem: *const c_char,
    also_make: *mut Dep,
    variables: *mut VariableSet,
    pat_variables: *mut VariableSet,
    pat_searched: bool,
}

struct Dep {
    name: *const c_char,
    file: *mut File,
    ignore_mtime: bool,
    ignore_automatic_vars: bool,
    is_explicit: bool,
    wait_here: bool,
    changed: bool,
    next: *mut Dep,
}

struct Rule {
    targets: *mut *mut c_char,
    suffixes: *mut *mut c_char,
    lens: *mut usize,
    deps: *mut Dep,
    cmds: *mut Commands,
    num: usize,
    terminal: bool,
    in_use: bool,
    next: *mut Rule,
}

struct Commands;
struct VariableSet;

struct PatDeps {
    name: *const c_char,
    pattern: *const c_char,
    file: *mut File,
    ignore_mtime: bool,
    ignore_automatic_vars: bool,
    is_explicit: bool,
    wait_here: bool,
}

struct TryRule {
    rule: *mut Rule,
    stemlen: usize,
    matches: usize,
    order: usize,
    checked_lastslash: bool,
}

extern {
    fn ar_name(name: *const c_char) -> bool;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn memrchr(s: *const c_void, c: c_int, n: usize) -> *mut c_void;
    fn alloca(size: usize) -> *mut c_void;
    fn xmalloc(size: usize) -> *mut c_void;
    fn xrealloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn qsort(base: *mut c_void, nmemb: usize, size: usize, compar: extern fn(*const c_void, *const c_void) -> c_int);
    fn strcache_add(s: *const c_char) -> *const c_char;
    fn strcache_add_len(s: *const c_char, len: usize) -> *const c_char;
    fn lookup_file(name: *const c_char) -> *mut File;
    fn enter_file(name: *const c_char) -> *mut File;
    fn file_impossible(name: *const c_char);
    fn file_exists_p(name: *const c_char) -> bool;
    fn vpath_search(name: *const c_char, skip_vpath: bool, path: *mut *const c_char, mtime_ptr: *mut time_t) -> *const c_char;
    fn initialize_file_variables(file: *mut File, reading: bool);
    fn set_file_variables(file: *mut File, stem: *const c_char);
    fn define_variable_for_file(name: *const c_char, length: usize, value: *const c_char, origin: u8, target: bool, file: *mut File);
    fn variable_expand_for_file(s: *const c_char, file: *mut File) -> *mut c_char;
    fn free_variable_set(set: *mut VariableSet);
    fn free_dep_chain(dep: *mut Dep);
    fn shuffle_deps_recursive(deps: *mut Dep);
    fn get_rule_defn(rule: *mut Rule) -> *const c_char;
    fn alloc_dep() -> *mut Dep;
    fn merge_variable_set_lists(dest: *mut *mut VariableSet, src: *mut VariableSet);
}

const DB_IMPLICIT: i32 = 0;
const MAX: fn(usize, usize) -> usize = std::cmp::max;
const MAP_NUL: i32 = 0;
const MAP_PIPE: i32 = 1;
const PARSEFS_ONEWORD: i32 = 1;
const PARSEFS_WAIT: i32 = 2;
const o_automatic: u8 = 0;

fn pattern_search(file: *mut File, archive: bool, depth: usize, recursions: usize, allow_compat_rules: bool) -> bool {
    unsafe {
        let filename = if archive { strchr((*file).name, '(' as i32) } else { (*file).name };
        let namelen = strlen(filename);
        let lastslash = if archive || ar_name(filename) {
            ptr::null()
        } else {
            memrchr(filename as *const c_void, '/' as i32, namelen - 1)
        };
        let pathlen = if lastslash.is_null() {
            0
        } else {
            lastslash.offset_from(filename as *const c_void) as usize + 1
        };

        let max_deps = max_pattern_deps;
        let mut deplist = xmalloc(max_deps * mem::size_of::<PatDeps>()) as *mut PatDeps;
        let mut pat = deplist;

        let deplen = namelen + max_pattern_dep_length + 4;
        let depname = alloca(deplen) as *mut c_char;

        let mut stem = ptr::null();
        let mut stemlen = 0;
        let mut fullstemlen = 0;

        let tryrules = xmalloc(num_pattern_rules * max_pattern_targets * mem::size_of::<TryRule>()) as *mut TryRule;
        let mut nrules = 0;

        let mut foundrule = 0;
        let mut intermed_ok = false;
        let mut file_vars_initialized = false;
        let mut specific_rule_matched = false;
        let mut found_compat_rule = false;

        let mut pathdir = ptr::null_mut();
        let mut stem_str = [0; 1024];

        for rule in pattern_rules_iter() {
            if !(*rule).deps.is_null() && (*rule).cmds.is_null() {
                continue;
            }
            if (*rule).in_use {
                continue;
            }

            for ti in 0..(*rule).num {
                let target = *(*rule).targets.offset(ti as isize);
                let suffix = *(*rule).suffixes.offset(ti as isize);
                
                if recursions > 0 && *target.offset(1) == 0 && !(*rule).terminal {
                    continue;
                }
                if *(*rule).lens.offset(ti as isize) > namelen {
                    continue;
                }

                stem = filename.offset((suffix as isize - target as isize - 1) as isize);
                stemlen = namelen - *(*rule).lens.offset(ti as isize) + 1;

                let mut check_lastslash = false;
                if !lastslash.is_null() {
                    check_lastslash = strchr(target, '/' as i32).is_null();
                }

                if check_lastslash {
                    if pathlen > stemlen {
                        continue;
                    }
                    stemlen -= pathlen;
                    stem = stem.offset(pathlen as isize);
                }

                if !check_lastslash {
                    memcpy(stem_str.as_mut_ptr() as *mut c_void, stem as *const c_void, stemlen);
                    stem_str[stemlen] = 0;
                } else {
                    memcpy(stem_str.as_mut_ptr() as *mut c_void, filename as *const c_void, pathlen);
                    memcpy(stem_str.as_mut_ptr().offset(pathlen as isize) as *mut c_void, stem as *const c_void, stemlen);
                    stem_str[pathlen + stemlen] = 0;
                }

                if *suffix != *stem.offset(stemlen as isize) || (*suffix != 0 && strcmp(suffix.offset(1), stem.offset(stemlen as isize + 1)) != 0) {
                    continue;
                }

                if *target.offset(1) != 0 {
                    specific_rule_matched = true;
                }

                if (*rule).deps.is_null() && (*rule).cmds.is_null() {
                    continue;
                }

                (*tryrules.offset(nrules as isize)).rule = rule;
                (*tryrules.offset(nrules as isize)).matches = ti;
                (*tryrules.offset(nrules as isize)).stemlen = stemlen + if check_lastslash { pathlen } else { 0 };
                (*tryrules.offset(nrules as isize)).order = nrules;
                (*tryrules.offset(nrules as isize)).checked_lastslash = check_lastslash;
                nrules += 1;
            }
        }

        if nrules == 0 {
            goto done;
        }

        if nrules > 1 {
            qsort(tryrules as *mut c_void, nrules, mem::size_of::<TryRule>(), stemlen_compare);
        }

        if specific_rule_matched {
            for ri in 0..nrules {
                if !(*(*tryrules.offset(ri as isize)).rule).terminal {
                    for j in 0..(*(*tryrules.offset(ri as isize)).rule.num {
                        if *(*(*(*tryrules.offset(ri as isize)).rule).targets.offset(j as isize)).offset(1) == 0 {
                            (*tryrules.offset(ri as isize)).rule = ptr::null_mut();
                            break;
                        }
                    }
                }
            }
        }

        for intermed_ok in 0..2 {
            pat = deplist;
            if intermed_ok != 0 {
                // DBF(DB_IMPLICIT, "Trying harder.\n");
            }

            for ri in 0..nrules {
                let rule = (*tryrules.offset(ri as isize)).rule;
                if rule.is_null() {
                    continue;
                }
                if intermed_ok != 0 && (*rule).terminal {
                    continue;
                }

                let matches = (*tryrules.offset(ri as isize)).matches;
                stem = filename.offset((*(*rule).suffixes.offset(matches as isize) as isize - *(*rule).targets.offset(matches as isize) as isize - 1) as isize);
                stemlen = namelen - *(*rule).lens.offset(matches as isize) + 1;
                let check_lastslash = (*tryrules.offset(ri as isize)).checked_lastslash;

                if check_lastslash {
                    stem = stem.offset(pathlen as isize);
                    stemlen -= pathlen;
                    if pathdir.is_null() {
                        pathdir = alloca(pathlen + 1) as *mut c_char;
                        memcpy(pathdir as *mut c_void, filename as *const c_void, pathlen);
                        *pathdir.offset(pathlen as isize) = 0;
                    }
                }

                if stemlen + if check_lastslash { pathlen } else { 0 } > GET_PATH_MAX {
                    continue;
                }

                if !check_lastslash {
                    memcpy(stem_str.as_mut_ptr() as *mut c_void, stem as *const c_void, stemlen);
                    stem_str[stemlen] = 0;
                } else {
                    memcpy(stem_str.as_mut_ptr() as *mut c_void, filename as *const c_void, pathlen);
                    memcpy(stem_str.as_mut_ptr().offset(pathlen as isize) as *mut c_void, stem as *const c_void, stemlen);
                    stem_str[pathlen + stemlen] = 0;
                }

                if (*rule).deps.is_null() {
                    break;
                }

                (*rule).in_use = true;

                let mut failed = false;
                let mut file_variables_set = false;
                let mut deps_found = 0;
                let mut nptr = dep_name((*rule).deps);
                let mut order_only = false;

                loop {
                    if nptr.is_null() {
                        let dep = (*(*rule).deps).next;
                        if dep.is_null() {
                            break;
                        }
                        nptr = dep_name(dep);
                    }

                    if !(*(*rule).deps).need_2nd_expansion {
                        // Handle simple case
                    } else {
                        // Handle second expansion
                    }

                    if deps_found > max_deps {
                        let l = pat.offset_from(deplist) as usize;
                        max_pattern_deps = MAX(max_pattern_deps, deps_found);
                        max_deps = max_pattern_deps;
                        deplist = xrealloc(deplist as *mut c_void, max_deps * mem::size_of::<PatDeps>()) as *mut PatDeps;
                        pat = deplist.offset(l as isize);
                    }

                    // Process dependencies
                    if failed {
                        break;
                    }
                }

                (*rule).in_use = false;

                if !failed {
                    break;
                }
            }

            if ri < nrules {
                break;
            }
        }

        if rule.is_null() {
            goto done;
        }

        foundrule = ri;

        if recursions > 0 {
            (*file).name = *(*rule).targets.offset((*tryrules.offset(foundrule as isize)).matches as isize);
        }

        while pat > deplist {
            pat = pat.offset(-1);
            // Process dependencies
        }

        if !(*file).was_shuffled {
            shuffle_deps_recursive((*file).deps);
        }

        if !(*tryrules.offset(foundrule as isize)).checked_lastslash {
            (*file).stem = strcache_add_len(stem, stemlen);
            fullstemlen = stemlen;
        } else {
            fullstemlen = pathlen + stemlen;
            memcpy(stem_str.as_mut_ptr() as *mut c_void, filename as *const c_void, pathlen);
            memcpy(stem_str.as_mut_ptr().offset(pathlen as isize) as *mut c_void, stem as *const c_void, stemlen);
            stem_str[fullstemlen] = 0;
            (*file).stem = strcache_add(stem_str.as_ptr());
        }

        (*file).cmds = (*rule).cmds;
        (*file).is_target = true;

        // Process also_make targets
        if (*rule).num > 1 {
            for ri in 0..(*rule).num {
                if ri != (*tryrules.offset(foundrule as isize)).matches {
                    // Add to also_make
                }
            }
        }

    done:
        free(tryrules as *mut c_void);
        free(deplist as *mut c_void);

        if !rule.is_null() {
            return true;
        }

        if found_compat_rule {
            return pattern_search(file, archive, depth, recursions, true);
        }

        false
    }
}

fn try_implicit_rule(file: *mut File, depth: usize) -> bool {
    unsafe {
        if pattern_search(file, false, depth, 0, false) {
            return true;
        }

        if ar_name((*file).name) {
            if pattern_search(file, true, depth, 0, false) {
                return true;
            }
        }

        false
    }
}

extern "C" fn stemlen_compare(v1: *const c_void, v2: *const c_void) -> c_int {
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

fn pattern_rules_iter() -> PatternRulesIter {
    PatternRulesIter { current: pattern_rules }
}

struct PatternRulesIter {
    current: *mut Rule,
}

impl Iterator for PatternRulesIter {
    type Item = *mut Rule;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.current.is_null() {
                None
            } else {
                let rule = self.current;
                self.current = (*rule).next;
                Some(rule)
            }
        }
    }
}

static mut pattern_rules: *mut Rule = ptr::null_mut();
static mut max_pattern_deps: usize = 0;
static mut max_pattern_dep_length: usize = 0;
static mut num_pattern_rules: usize = 0;
static mut max_pattern_targets: usize = 0;
static mut no_intermediates: bool = false;
static mut GET_PATH_MAX: usize = 1024;