/* 
   Change RCS file attributes.

   Copyright (C) 2010-2020 Thien-Thi Nguyen
   Copyright (C) 1990, 1991, 1992, 1993, 1994, 1995 Paul Eggert
   Copyright (C) 1982, 1988, 1989 Walter Tichy

   This file is part of GNU RCS.

   GNU RCS is free software: you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   GNU RCS is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty
   of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
   See the GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::mem;
use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{stat, umask, mode_t};

mod base {
    pub const S_IRUSR: mode_t = 0o400;
    pub const S_IRGRP: mode_t = 0o40;
    pub const S_IROTH: mode_t = 0o4;
}

struct ULog {
    revno: *const c_char,
    message: Cbuf,
}

struct UState {
    revno: *const c_char,
    status: *const c_char,
}

enum ChangeAccess {
    Append,
    Erase,
}

struct ChAccess {
    login: *const c_char,
    command: ChangeAccess,
}

struct DelRevPair {
    strt: *const c_char,
    end: *const c_char,
    code: c_int,
}

struct AdminStuff {
    rv: c_int,
    deltas: *mut WLink,
    suppress_mail: bool,
    lockhead: bool,
    unlockcaller: bool,
    newlocks: *mut Link,
    byelocks: *mut Link,
    headstate: *const c_char,
    headstate_changed: bool,
    states: Link,
    tp_state: *mut Link,
    accesses: Link,
    tp_access: *mut Link,
    assocs: Link,
    tp_assoc: *mut Link,
    logs: Link,
    tp_log: *mut Link,
    cuthead: *mut Delta,
    cuttail: *mut Delta,
    delstrt: *mut Delta,
    delrev: DelRevPair,
}

static KS_WS_COMMA: &[u8] = b" \n\t,";

extern "C" {
    fn FLOW(erroneous: *mut bool) -> *mut bool;
    fn exit_failure() -> c_int;
    fn fro_zclose(from: *mut *mut Fro);
    fn Ozclose(res: *mut *mut FILE);
    fn ORCSclose();
    fn dirtempunlink();
    fn PERR(fmt: *const c_char, ...);
    fn ZLLOC(count: usize, ty: usize) -> *mut c_void;
    fn extend(list: *mut Link, entry: *const c_void, plexus: usize) -> *mut Link;
    fn checkid(id: *const c_char, delim: c_char) -> *const c_char;
    fn SHSNIP(len: *mut usize, s: *const c_char, end: *const c_char) -> *const c_char;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn cleanlogmsg(msg: *const c_char, len: usize) -> Cbuf;
    fn set_empty_log_message(cb: *mut Cbuf);
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn complain(fmt: *const c_char, ...);
    fn yesorno(default: bool, prompt: *const c_char) -> bool;
    fn getcstdin() -> c_int;
    fn feof(stream: *mut FILE) -> c_int;
    fn fatal_sys(msg: *const c_char);
    fn aprintf(stream: *mut FILE, fmt: *const c_char, ...);
    fn afputc(c: c_int, stream: *mut FILE);
    fn aflush(stream: *mut FILE);
    fn rewind(stream: *mut FILE);
    fn run(fdin: c_int, fdout: c_int, cmd: *const c_char, ...) -> c_int;
    fn runv(fdin: c_int, fdout: c_int, argv: *const *const c_char) -> c_int;
    fn fileno(stream: *mut FILE) -> c_int;
    fn tmpfile() -> *mut FILE;
    fn getRCSINIT(argc: c_int, argv: *mut *mut c_char, newargv: *mut *mut *mut c_char) -> c_int;
    fn redefined(opt: c_char);
    fn pairnames(count: c_int, names: *mut *mut c_char, opener: usize, must_exist: bool, quiet: bool) -> c_int;
    fn rcswriteopen(name: *const c_char) -> c_int;
    fn rcsreadopen(name: *const c_char) -> c_int;
    fn parse_revpairs(opt: c_char, spec: *const c_char, data: *mut c_void, callback: usize);
    fn str2expmode(s: *const c_char) -> c_int;
    fn bad_option(opt: *const c_char);
    fn ffree();
    fn checkaccesslist() -> bool;
    fn putadmin();
    fn puttree(tip: *mut Delta, stream: *mut FILE);
    fn putdesc(desc: *mut Cbuf, textflag: bool, textfile: *const c_char);
    fn fro_trundling(enable: bool, fro: *mut Fro);
    fn IGNORE_REST(fro: *mut Fro);
    fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    fn file_mtime(keep: bool, st: *const stat) -> SystemTime;
    fn donerewrite(changed: bool, mtime: SystemTime) -> c_int;
    fn tempunlink();
    fn gnurcs_init(prog: *const c_char);
    fn gnurcs_goodbye();
    fn diagnose(fmt: *const c_char, ...);
    fn RWARN(fmt: *const c_char, ...);
    fn RERR(fmt: *const c_char, ...);
    fn RFATAL(fmt: *const c_char, ...);
    fn PWARN(fmt: *const c_char, ...);
    fn PFATAL(fmt: *const c_char, ...);
    fn STR_DIFF(s1: *const c_char, s2: *const c_char) -> bool;
    fn STR_SAME(s1: *const c_char, s2: *const c_char) -> bool;
    fn MEM_DIFF(len: usize, s1: *const c_char, s2: *const c_char) -> bool;
    fn NUM_EQ(s1: *const c_char, s2: *const c_char) -> bool;
    fn NUM_GT(s1: *const c_char, s2: *const c_char) -> bool;
    fn NUM_LT(s1: *const c_char, s2: *const c_char) -> bool;
    fn countnumflds(s: *const c_char) -> usize;
    fn compartial(s1: *const c_char, s2: *const c_char, len: usize) -> bool;
    fn fully_numeric_no_k(cb: *mut Cbuf, s: *const c_char) -> bool;
    fn gr_revno(rev: *const c_char, deltas: *mut *mut WLink) -> *mut Delta;
    fn BRANCHNO(rev: *const c_char) -> *const c_char;
    fn ODDP(n: usize) -> bool;
    fn EVENP(n: usize) -> bool;
    fn tiprev() -> *const c_char;
    fn addsymbol(rev: *const c_char, sym: *const c_char, overwrite: bool) -> bool;
    fn addlock(delta: *mut Delta, force: bool) -> c_int;
    fn lock_on(delta: *mut Delta) -> bool;
    fn lock_delta_memq(box_: *mut Link, delta: *const Delta) -> *mut Link;
    fn lock_drop(box_: *mut Link, tp: *mut Link);
    fn caller_login_p(login: *const c_char) -> bool;
    fn getcaller() -> *const c_char;
    fn getfullRCSname() -> *const c_char;
    fn basefilename(name: *const c_char) -> *const c_char;
    fn make_editstuff() -> *mut EditStuff;
    fn unmake_editstuff(es: *mut EditStuff);
    fn editstring(es: *mut EditStuff, at: *mut AtAt, prompt: *const c_char);
    fn enterstring(es: *mut EditStuff, at: *mut AtAt);
    fn snapshotedit(es: *mut EditStuff, stream: *mut FILE);
    fn finishedit(es: *mut EditStuff, diffname: *const c_char, result: *const c_char, keep: bool);
    fn putdtext(delta: *mut Delta, diffname: *const c_char, stream: *mut FILE, is_diff: bool) -> bool;
    fn SAME_AFTER(fro: *mut Fro, desc: *const c_char);
    fn maketemp(dir: c_int) -> *const c_char;
    fn str_save(s: *const c_char) -> *const c_char;
    fn newline(stream: *mut FILE);
    fn putstring(stream: *mut FILE, s: Cbuf, quote: bool);
    fn atat_put(stream: *mut FILE, at: *mut AtAt);
    fn fro_spew_partial(stream: *mut FILE, fro: *mut Fro, range: *mut Range);
    fn fro_move(fro: *mut Fro, pos: usize);
    fn string_from_atat(which: usize, at: *mut AtAt) -> Cbuf;
    fn findlock(quiet: bool, delta: *mut *mut Delta) -> c_int;
}

struct Cbuf {
    string: *mut c_char,
    size: usize,
}

struct Link {
    entry: *const c_void,
    next: *mut Link,
}

struct WLink {
    entry: *mut Delta,
    next: *mut WLink,
}

struct Delta {
    num: *const c_char,
    state: *const c_char,
    log: *mut AtAt,
    text: *mut AtAt,
    neck: usize,
    selector: bool,
    pretty_log: Cbuf,
    ilk: *mut Delta,
    branches: *mut WLink,
}

struct AtAt {
    beg: usize,
    end: usize,
}

struct Range {
    beg: usize,
    end: usize,
}

struct Fro {
    // Implementation details
}

struct EditStuff {
    // Implementation details
}

struct RcsLock {
    login: *const c_char,
    delta: *mut Delta,
}

struct SymDef {
    meaningful: *const c_char,
    underlying: *const c_char,
}

struct USymDef {
    u: SymDef,
    override: bool,
}

struct Repo {
    filename: *const c_char,
    tip: *mut Delta,
    stat: stat,
    log_lead: Cbuf,
}

struct Mani {
    filename: *const c_char,
}

struct Flow {
    from: *mut Fro,
    res: *mut FILE,
    rewr: *mut FILE,
    to: *mut FILE,
    result: *const c_char,
    erroneous: bool,
}

struct Prog {
    name: *const c_char,
    kws: c_int,
    pe: *const c_char,
    quiet: bool,
    interactive: bool,
    strictly_locking: bool,
}

static mut PROGRAM: Prog = Prog {
    name: b"rcs\0" as *const u8 as *const c_char,
    kws: 0,
    pe: ptr::null(),
    quiet: false,
    interactive: false,
    strictly_locking: false,
};

static mut REPO: Repo = Repo {
    filename: ptr::null(),
    tip: ptr::null_mut(),
    stat: stat {
        st_mode: 0,
        st_nlink: 0,
        // Other fields omitted for brevity
    },
    log_lead: Cbuf {
        string: ptr::null_mut(),
        size: 0,
    },
};

static mut MANI: Mani = Mani {
    filename: ptr::null(),
};

static mut FLOW: Flow = Flow {
    from: ptr::null_mut(),
    res: ptr::null_mut(),
    rewr: ptr::null_mut(),
    to: ptr::null_mut(),
    result: ptr::null(),
    erroneous: false,
};

static mut GROK: Prog = Prog {
    name: ptr::null(),
    kws: 0,
    pe: ptr::null(),
    quiet: false,
    interactive: false,
    strictly_locking: false,
};

fn cleanup(exitstatus: &mut c_int) {
    unsafe {
        if FLOW.erroneous {
            *exitstatus = exit_failure();
        }
        fro_zclose(&mut FLOW.from);
        Ozclose(&mut FLOW.res);
        ORCSclose();
        dirtempunlink();
    }
}

fn rcs_main(cmd: *const c_char, argc: c_int, argv: *mut *mut c_char) -> c_int {
    unsafe {
        let mut dc: AdminStuff = mem::zeroed();
        dc.rv = 0;

        nosetid();

        let mut branchsym: *const c_char = ptr::null();
        let mut commsyml: *const c_char = ptr::null();
        let mut textfile: *const c_char = ptr::null();
        let mut branchflag = false;
        let mut commsymlen = 0;
        let mut boxlock = Link {
            entry: ptr::null(),
            next: ptr::null_mut(),
        };
        let mut tplock = &mut boxlock;
        let mut boxrm = Link {
            entry: ptr::null(),
            next: ptr::null_mut(),
        };
        let mut tprm = &mut boxrm;
        let mut expmode = -1;
        let mut initflag = false;
        let mut textflag = false;
        let mut strict_selected = false;
        let mut Ttimeflag = false;

        let mut newargv: *mut *mut c_char = ptr::null_mut();
        let argc = getRCSINIT(argc, argv, &mut newargv);
        argv = newargv;

        while argc > 0 && !(*argv).is_null() && (**argv).is_null() {
            let a = *argv;
            argv = argv.offset(1);
            argc -= 1;

            if a.is_null() || (*a).is_null() {
                continue;
            }

            let mut a_ptr = a.offset(1);
            if *a_ptr == 0 {
                a_ptr = a_ptr.offset(1);
            }

            match *a_ptr {
                b'i' => initflag = true,
                b'b' => {
                    if branchflag {
                        redefined(b'b' as c_char);
                    }
                    branchflag = true;
                    branchsym = a_ptr.offset(1);
                }
                b'c' => {
                    if !commsyml.is_null() {
                        redefined(b'c' as c_char);
                    }
                    commsyml = a_ptr.offset(1);
                    commsymlen = strlen(commsyml);
                }
                b'a' => getaccessor(&mut dc, a, ChangeAccess::Append),
                b'A' => {
                    if *a_ptr.offset(1) == 0 {
                        PERR(b"missing filename after -A\0" as *const u8 as *const c_char);
                        continue;
                    }
                    *argv = a_ptr.offset(1);
                    if pairnames(1, argv, rcsreadopen as usize, true, false) > 0 {
                        let mut ls = GROK.accesses;
                        while !ls.is_null() {
                            getchaccess(
                                &mut dc,
                                str_save((*ls).entry as *const c_char),
                                ChangeAccess::Append,
                            );
                            ls = (*ls).next;
                        }
                        fro_zclose(&mut FLOW.from);
                    }
                }
                b'e' => getaccessor(&mut dc, a, ChangeAccess::Erase),
                b'l' => {
                    if *a_ptr.offset(1) == 0 {
                        dc.lockhead = true;
                    } else {
                        tplock = extend(tplock, a_ptr.offset(1) as *const c_void, 0);
                    }
                }
                b'u' => {
                    if *a_ptr.offset(1) == 0 {
                        dc.unlockcaller = true;
                    } else {
                        tprm = extend(tprm, a_ptr.offset(1) as *const c_void, 0);
                        dc.newlocks = boxlock.next;
                        tplock = rmnewlocklst(&mut dc, a_ptr.offset(1));
                    }
                }
                b'L' => {
                    if strict_selected && !dc.strictly_locking {
                        PWARN(b"-U overridden by -L\0" as *const u8 as *const c_char);
                    }
                    dc.strictly_locking = true;
                    strict_selected = true;
                }
                b'U' => {
                    if strict_selected && dc.strictly_locking {
                        PWARN(b"-L overridden by -U\0" as *const u8 as *const c_char);
                    }
                    strict_selected = true