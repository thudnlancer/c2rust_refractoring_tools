use std::fs;
use std::io;
use std::path::Path;
use std::ffi::{CStr, CString};
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::time::{SystemTime, UNIX_EPOCH};
use nix::sys::stat::{SFlag, Mode};
use libc::{stat, S_IFMT, S_IFREG, S_IWUSR, S_IWGRP, S_IWOTH};
use std::ptr;
use std::mem;
use std::os::raw::{c_char, c_int};
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::process::Command;
use std::env;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

const KS_HWS: &str = " \t";
const QUIET_ARG: &str = "-q";

#[derive(Debug)]
struct Work {
    st: stat,
    force: bool,
}

#[derive(Debug)]
struct JStuff {
    jstuff: *mut Divvy,
    head: Link,
    tp: *mut Link,
    merge: *mut SymDef,
    expand: *const c_char,
    suffix: *const c_char,
    version: *const c_char,
    zone: *const c_char,
    d: *mut Delta,
    ls: *mut *const c_char,
    lastidx: c_int,
}

#[derive(Debug)]
struct Divvy;

#[derive(Debug)]
struct Link {
    next: *mut Link,
    entry: *const c_char,
}

#[derive(Debug)]
struct SymDef {
    meaningful: *const c_char,
}

#[derive(Debug)]
struct Delta {
    num: *const c_char,
    text: *const c_char,
    date: *const c_char,
    name: *const c_char,
}

fn skip_hws(var: &mut *const c_char) {
    unsafe {
        let s = CStr::from_ptr(*var).to_str().unwrap();
        *var = var.offset(strspn(s, KS_HWS) as isize;
    }
}

fn strspn(s: &str, accept: &str) -> usize {
    s.chars()
        .take_while(|&c| accept.contains(c))
        .count()
}

fn rmworkfile(work: &Work) -> bool {
    unsafe {
        let mode = work.st.st_mode;
        if (mode & (S_IWUSR | S_IWGRP | S_IWOTH) as u32) != 0 && !work.force {
            let mani_filename = CStr::from_ptr(MANI_FILENAME).to_str().unwrap();
            if !yesorno(
                false,
                &format!(
                    "writable {} exists{}; remove it",
                    mani_filename,
                    if stat_mine_p(&work.st) {
                        ""
                    } else {
                        ", and you do not own it"
                    }
                ),
            ) {
                if !BE_QUIET && ttystdin() {
                    perr("checkout aborted");
                } else {
                    perr(
                        "writable {} exists; checkout aborted",
                        mani_filename,
                    );
                }
                return false;
            }
        }
        true
    }
}

fn rmlock(delta: *const Delta) -> c_int {
    unsafe {
        let mut box_ = Link {
            next: GROK_LOCKS,
            entry: ptr::null(),
        };
        let tp = lock_delta_memq(&mut box_, delta);
        if tp.is_null() {
            return 0;
        }
        let rl = (*tp).next;
        if !caller_login_p((*rl).entry) {
            rerr(
                "revision %s locked by %s; use co -r or rcs -u",
                (*delta).num,
                (*rl).entry,
            );
            return -1;
        }
        lock_drop(&mut box_, tp);
        1
    }
}

fn jpush(rev: *const c_char, js: &mut JStuff) {
    unsafe {
        js.tp = extend(js.tp, rev, js.jstuff);
        js.lastidx += 1;
    }
}

fn addjoin(spec: *mut c_char, js: &mut JStuff) -> *mut c_char {
    const DELIMS: &str = " \t\n:,;";
    unsafe {
        let mut eot: *mut c_char = ptr::null_mut();
        let save: c_char;
        let cool: *mut Delta;
        let mut numrev: Cbuf = Cbuf {
            string: ptr::null_mut(),
        };

        skip_hws(&mut spec);
        eot = spec.offset(strcspn(CStr::from_ptr(spec).to_str().unwrap(), DELIMS) as isize);

        save = *eot;
        *eot = 0;
        cool = if fully_numeric_no_k(&mut numrev, spec) {
            delta_from_ref(numrev.string)
        } else {
            ptr::null_mut()
        };
        *eot = save;

        if cool.is_null() {
            return ptr::null_mut();
        }

        jpush((*cool).num, js);
        skip_hws(&mut eot);
        eot
    }
}

fn getancestor(r1: *const c_char, r2: *const c_char) -> *const c_char {
    unsafe {
        let t1: *const c_char;
        let t2: *const c_char;
        let l1: c_int;
        let l2: c_int;
        let l3: c_int;
        let r: *const c_char;

        l1 = countnumflds(r1);
        l2 = countnumflds(r2);
        if (2 < l1 || 2 < l2) && !NUM_EQ(r1, r2) {
            l3 = 0;
            while NUMF_EQ(1 + l3, r1, r2) && NUMF_EQ(2 + l3, r1, r2) {
                l3 += 2;
            }
            if l3 == 0 {
                t1 = TAKE(if l1 > 2 { 2 } else { l1 }, r1);
                t2 = TAKE(if l2 > 2 { 2 } else { l2 }, r2);
                r = if NUM_LT(t1, t2) { t1 } else { t2 };
                if !NUM_EQ(r, r1) && !NUM_EQ(r, r2) {
                    return str_save(r);
                }
            } else if !NUMF_EQ(1 + l3, r1, r2) {
                return str_save(TAKE(l3, r1));
            }
        }
        rerr(
            "common ancestor of %s and %s undefined",
            r1,
            r2,
        );
        ptr::null()
    }
}

fn preparejoin(argv: *mut c_char, js: &mut JStuff) -> bool {
    const KS_COMMA: &str = ",";
    unsafe {
        let mut s: *mut c_char = argv;
        let mut save: *mut c_char = ptr::null_mut();
        let mut j: *mut c_char;
        let mut rv: bool = true;

        js.jstuff = make_space(b"jstuff\0".as_ptr() as *const c_char);
        js.head.next = ptr::null_mut();
        js.tp = &mut js.head;
        if js.merge.is_null() {
            js.merge = ZLLOC(1, mem::size_of::<SymDef>()) as *mut SymDef;
            (*js.merge).meaningful = b"merge\0".as_ptr() as *const c_char;
        }

        js.lastidx = -1;
        while {
            j = strtok_r(s, KS_COMMA.as_ptr() as *const c_char, &mut save);
            !j.is_null()
        } {
            s = ptr::null_mut();
            j = addjoin(j, js);
            if j.is_null() {
                return false;
            }
            if *j == b':' as c_char {
                j = j.offset(1);
                skip_hws(&mut j);
                if *j == 0 {
                    goto!(incomplete);
                }
                j = addjoin(j, js);
                if j.is_null() {
                    return false;
                }
            } else {
                if js.lastidx == 0 {
                    let two = (*js.tp).entry;

                    jpush(two, js);
                    (*js.tp).entry = getancestor((*js.d).num, two);
                    if (*js.tp).entry.is_null() {
                        rv = false;
                    }
                } else {
                    incomplete:
                    rfat!("join pair incomplete");
                }
            }
        }
        if js.lastidx < 1 {
            rfat!("empty join");
        }

        js.ls = pointer_array(PLEXUS, 1 + js.lastidx);
        js.tp = js.head.next;
        for i in 0..=js.lastidx {
            *js.ls.offset(i as isize) = (*js.tp).entry;
            js.tp = (*js.tp).next;
        }
        close_space(js.jstuff);
        js.jstuff = ptr::null_mut();
        rv
    }
}

fn buildjoin(initialfile: *const c_char, js: &mut JStuff) -> bool {
    const VX: usize = 3;
    unsafe {
        let rev2: *const c_char;
        let rev3: *const c_char;
        let mut i: c_int;
        let mut cov: [*const c_char; 8 + VX] = [ptr::null(); 8 + VX];
        let mut mergev: [*const c_char; 11] = [ptr::null(); 11];
        let mut p: *mut *const c_char;
        let mut len: usize;
        let mut subs: *const c_char = ptr::null();

        rev2 = maketemp(0);
        rev3 = maketemp(3);

        cov[1] = PEER_SUPER();
        cov[2] = b"co\0".as_ptr() as *const c_char;
        p = &mut cov[1 + VX] as *mut *const c_char;
        if !js.expand.is_null() {
            *p = js.expand;
            p = p.offset(1);
        }
        if !js.suffix.is_null() {
            *p = js.suffix;
            p = p.offset(1);
        }
        if !js.version.is_null() {
            *p = js.version;
            p = p.offset(1);
        }
        if !js.zone.is_null() {
            *p = js.zone;
            p = p.offset(1);
        }
        *p = QUIET_ARG.as_ptr() as *const c_char;
        p = p.offset(1);
        *p = REPO_FILENAME;
        p = p.offset(1);
        *p = ptr::null();

        mergev[1] = find_peer_prog(js.merge);
        mergev[2] = b"-L\0".as_ptr() as *const c_char;
        mergev[4] = b"-L\0".as_ptr() as *const c_char;

        i = 0;
        while i < js.lastidx {
            if i == 0 {
                subs = (*js.d).num;
            } else {
                accf!(
                    SINGLE,
                    "%s,%s:%s",
                    subs,
                    *js.ls.offset((i - 2) as isize),
                    *js.ls.offset((i - 1) as isize)
                );
                subs = finish_string(SINGLE, &mut len);
            }
            diagnose!("revision %s", *js.ls.offset(i as isize));
            accf!(SINGLE, "-p%s", *js.ls.offset(i as isize));
            cov[VX] = finish_string(SINGLE, &mut len);
            if runv(-1, rev2, cov.as_ptr()) {
                goto!(badmerge);
            }
            diagnose!("revision %s", *js.ls.offset((i + 1) as isize));
            accf!(SINGLE, "-p%s", *js.ls.offset((i + 1) as isize));
            cov[VX] = finish_string(SINGLE, &mut len);
            if runv(-1, rev3, cov.as_ptr()) {
                goto!(badmerge);
            }
            diagnose!("merging...");
            mergev[3] = subs;
            mergev[5] = *js.ls.offset((i + 1) as isize);
            p = &mut mergev[6] as *mut *const c_char;
            if BE_QUIET {
                *p = QUIET_ARG.as_ptr() as *const c_char;
                p = p.offset(1);
            }
            if js.lastidx <= i + 2 && !MANI_STANDARD_OUTPUT.is_null() {
                *p = b"-p\0".as_ptr() as *const c_char;
                p = p.offset(1);
            }
            *p = initialfile;
            p = p.offset(1);
            *p = rev2;
            p = p.offset(1);
            *p = rev3;
            p = p.offset(1);
            *p = ptr::null();
            if DIFF_TROUBLE == runv(-1, ptr::null(), mergev.as_ptr()) {
                goto!(badmerge);
            }
            i += 2;
        }
        return true;

        badmerge:
        FLOW_ERRONEOUS = true;
        false
    }
}

fn co_main(cmd: *const c_char, argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut exitstatus: c_int = EXIT_SUCCESS;
    let mut work: Work = Work {
        st: unsafe { mem::zeroed() },
        force: false,
    };
    let mut jstuff: JStuff = unsafe { mem::zeroed() };
    let mut neworkptr: *mut FILE = ptr::null_mut();
    let mut lockflag: c_int = 0;
    let mut mtimeflag: bool = false;
    let mut a: *mut c_char;
    let mut joinflag: *mut c_char = ptr::null_mut();
    let mut newargv: *mut *mut c_char;
    let mut author: *const c_char = ptr::null();
    let mut date: *const c_char = ptr::null();
    let mut rev: *const c_char = ptr::null();
    let mut state: *const c_char = ptr::null();
    let mut joinname: *const c_char = ptr::null();
    let mut newdate: *const c_char = ptr::null();
    let mut neworkname: *const c_char = ptr::null();
    let mut changelock: c_int = 0;
    let mut expmode: c_int = -1;
    let mut r: c_int;
    let mut workstatstat: c_int;
    let mut tostdout: bool = false;
    let mut Ttimeflag: bool = false;
    let mut selfsame: bool = false;
    let mut finaldate: [c_char; DATESIZE] = [0; DATESIZE];
    let mut stdout_mode: c_int = 0;
    let mut deltas: *mut WLink = ptr::null_mut();

    unsafe {
        CHECK_HV(cmd);
        gnurcs_init(&program);
        memset(
            &mut jstuff as *mut _ as *mut c_void,
            0,
            mem::size_of::<JStuff>(),
        );

        setrid();
        argc = getRCSINIT(argc, argv, &mut newargv);
        argv = newargv;
        while {
            a = *argv.offset(1);
            argc -= 1;
            argc > 0 && *a == b'-' as c_char
        } {
            a = a.offset(1);
            match *a {
                b'r' => {
                    a = a.offset(1);
                    chk_set_rev(&mut rev, a);
                }
                b'f' => {
                    work.force = true;
                    a = a.offset(1);
                    chk_set_rev(&mut rev, a);
                }
                b'l' => {
                    if lockflag < 0 {
                        PWARN!("-u overridden by -l.");
                    }
                    lockflag = 1;
                    a = a.offset(1);
                    chk_set_rev(&mut rev, a);
                }
                b'u' => {
                    if 0 < lockflag {
                        PWARN!("-l overridden by -u.");
                    }
                    lockflag = -1;
                    a = a.offset(1);
                    chk_set_rev(&mut rev, a);
                }
                b'p' => {
                    tostdout = true;
                    a = a.offset(1);
                    chk_set_rev(&mut rev, a);
                }
                b'I' => {
                    BE_INTERACTIVE = true;
                    a = a.offset(1);
                    chk_set_rev(&mut rev, a);
                }
                b'q' => {
                    BE_QUIET = true;
                    a = a.offset(1);
                    chk_set_rev(&mut rev, a);
                }
                b'd' => {
                    if !date.is_null() {
                        redefined(b'd');
                    }
                    str2date(a, finaldate.as_mut_ptr());
                    date = finaldate.as_ptr();
                }
                b'j' => {
                    if *a != 0 {
                        if !joinflag.is_null() {
                            redefined(b'j');
                        }
                        joinflag = a;
                    }
                }
                b'M' => {
                    mtimeflag = true;
                    a = a.offset(1);
                    chk_set_rev(&mut rev, a);
                }
                b's' => {
                    if *a != 0 {
                        if !state.is_null() {
                            redefined(b's');
                        }
                        state = a;
                    }
                }
                b'S' => {
                    selfsame = true;
                }
                b'T' => {
                    if *a != 0 {
                        goto!(unknown);
                    }
                    Ttimeflag = true