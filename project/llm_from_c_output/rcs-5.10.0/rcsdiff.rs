/* Compare RCS revisions.

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

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;
use std::mem;
use std::path::Path;
use std::fs::{File, metadata};
use std::io::{self, Read, Write};
use std::process::{Command, ExitStatus};
use std::time::SystemTime;
use libc::{stat, strcmp, strncmp, strlen};

const DATESIZE: usize = 24;
const FULLDATESIZE: usize = 32;
const DIFF_SUCCESS: i32 = 0;
const DIFF_FAILURE: i32 = 1;
const DIFF_TROUBLE: i32 = 2;

struct Unique {
    eqval_ok: bool,
    minlen: usize,
    full: &'static str,
}

static MINUS_Y: Unique = Unique {
    eqval_ok: false,
    minlen: 4,
    full: "--side-by-side",
};

static MINUS_D: Unique = Unique {
    eqval_ok: true,
    minlen: 4,
    full: "--ifdef",
};

struct Work {
    st: stat,
    fro: *mut File,
}

fn longopt_maybe_p(arg: &str, u: &Unique) -> bool {
    let equal_pos = if u.eqval_ok { arg.find('=') } else { None };
    let len = equal_pos.unwrap_or_else(|| arg.len());
    
    len >= u.minlen && arg.starts_with(u.full)
}

fn cleanup(exitstatus: &mut i32, work: &mut Work) {
    if FLOW.erroneous {
        *exitstatus = DIFF_TROUBLE;
    }
    fro_zclose(&mut FLOW.from);
    fro_zclose(&mut work.fro);
}

fn setup_label(num: Option<&str>, date: &[u8; DATESIZE]) -> String {
    let datestr = date2str(date);
    let mut label = format!("--label={}\t{}", MANI.filename, datestr);
    if let Some(n) = num {
        label.push_str(&format!("\t{}", n));
    }
    label
}

const COMMAND_LINE_VARYING: usize = 4 + if DIFF_L { 0 } else { 1 };

fn rcsdiff_main(cmd: &str, argc: i32, argv: *mut *mut c_char) -> i32 {
    let mut exitstatus = DIFF_SUCCESS;
    let mut work = Work {
        st: unsafe { mem::zeroed() },
        fro: ptr::null_mut(),
    };
    let mut revnums = 0;
    let mut rev1: Option<String> = None;
    let mut rev2: Option<String> = None;
    let mut xrev1: Option<String> = None;
    let mut xrev2: Option<String> = None;
    let mut expandarg: Option<String> = None;
    let mut suffixarg: Option<String> = None;
    let mut versionarg: Option<String> = None;
    let mut zonearg: Option<String> = None;
    let mut file_labels = 0;
    let mut diff_label1: Option<String> = None;
    let mut diff_label2: Option<String> = None;
    let mut date2 = [0u8; DATESIZE];
    let mut cov: [Option<String>; 7 + COMMAND_LINE_VARYING] = Default::default();
    let mut diffv: Vec<String> = Vec::new();
    let mut no_diff_means_no_output = true;

    CHECK_HV(cmd);
    gnurcs_init(&program);
    
    // Initialize diffv with prog_diff
    diffv.push("diff".to_string());

    let mut new_argv = getRCSINIT(argc, argv);
    let mut args = new_argv.iter().skip(1);
    
    while let Some(arg) = args.next() {
        let mut chars = arg.chars().skip(1); // Skip '-'
        while let Some(c) = chars.next() {
            match c {
                'r' => {
                    revnums += 1;
                    let rev = chars.as_str().to_string();
                    match revnums {
                        1 => rev1 = Some(rev),
                        2 => rev2 = Some(rev),
                        _ => PERR("too many revision numbers"),
                    }
                    break;
                }
                '-' | 'D' => {
                    if c == 'D' || 
                       longopt_maybe_p(arg, &MINUS_D) || 
                       longopt_maybe_p(arg, &MINUS_Y) {
                        no_diff_means_no_output = false;
                    }
                    // Fall through
                }
                'C' | 'F' | 'I' | 'L' | 'U' | 'W' => {
                    if DIFF_L && c == 'L' {
                        file_labels += 1;
                        if file_labels > 2 {
                            PFATAL("too many -L options");
                        }
                    }
                    diffv.push(arg.to_string());
                    if chars.as_str().is_empty() {
                        if let Some(next_arg) = args.next() {
                            diffv.push(next_arg.to_string());
                        } else {
                            PFATAL("option requires an argument");
                        }
                    }
                }
                'y' => {
                    no_diff_means_no_output = false;
                    // Fall through
                }
                'B' | 'H' | '0'..='9' | 'a'..='w' => {
                    diffv.push(arg.to_string());
                }
                'q' => {
                    BE.quiet = true;
                }
                'x' => {
                    suffixarg = Some(chars.as_str().to_string());
                    BE.pe = chars.as_str();
                    break;
                }
                'z' => {
                    zonearg = Some(chars.as_str().to_string());
                    zone_set(chars.as_str());
                    break;
                }
                'T' => {
                    if chars.as_str().is_empty() {
                        break;
                    }
                    // Unknown option
                }
                'V' => {
                    versionarg = Some(chars.as_str().to_string());
                    setRCSversion(&versionarg.unwrap());
                    break;
                }
                'k' => {
                    expandarg = Some(chars.as_str().to_string());
                    if str2expmode(&expandarg.unwrap()) >= 0 {
                        break;
                    }
                    // Fall through to unknown
                }
                _ => {
                    bad_option(arg);
                }
            }
        }
    }

    if !BE.quiet {
        let diffvstr = diffv.join(" ");
        println!("{}", diffvstr);
    }

    if DIFF_L {
        if file_labels < 2 {
            if file_labels == 0 {
                diff_label1 = Some(String::new());
            }
            diff_label2 = Some(String::new());
        }
    }

    cov[1] = Some(PEER_SUPER.to_string());
    cov[2] = Some("co".to_string());
    cov[3] = Some("-q".to_string());
    if !DIFF_L {
        cov[COMMAND_LINE_VARYING - 1] = Some("-M".to_string());
    }

    if FLOW.erroneous {
        cleanup(&mut exitstatus, &mut work);
    } else if new_argv.is_empty() {
        PFATAL("no input file");
    } else {
        for filename in new_argv {
            let mut numericrev = String::new();
            let tip = REPO.tip;
            let mani_filename = MANI.filename;
            let kws = BE.kws;
            let defbr = GROK.branch;

            println!("RCS file: {}", REPO.filename);

            if rev2.is_none() {
                work.fro = match fro_open(mani_filename, FOPEN_R_WORK, &mut work.st) {
                    Ok(file) => file,
                    Err(e) => {
                        syserror_errno(mani_filename);
                        continue;
                    }
                };
            }

            if tip.is_none() {
                RERR("no revisions present");
                continue;
            }

            let rev1 = rev1.as_deref().unwrap_or_else(|| defbr.unwrap_or(tip.unwrap().num));
            if !fully_numeric(&mut numericrev, rev1, work.fro) {
                continue;
            }

            let target = match delta_from_ref(&numericrev) {
                Some(t) => t,
                None => continue,
            };
            xrev1 = Some(target.num.to_string());

            if DIFF_L && diff_label1.is_some() {
                *diff_label1.as_mut().unwrap() = setup_label(Some(target.num), &target.date);
            }

            let lexpandarg = expandarg.as_deref();
            if revnums == 2 {
                let rev2 = rev2.as_deref().unwrap_or_else(|| defbr.unwrap_or(tip.unwrap().num));
                if !fully_numeric(&mut numericrev, rev2, work.fro) {
                    continue;
                }

                let target = match delta_from_ref(&numericrev) {
                    Some(t) => t,
                    None => continue,
                };
                xrev2 = Some(target.num.to_string());

                if no_diff_means_no_output && xrev1 == xrev2 {
                    continue;
                }
            } else if target.lockedby.is_some() && lexpandarg.is_none() && 
                      kws == kwsub_kv && 
                      WORKMODE(REPO.stat.st_mode, true) == work.st.st_mode {
                lexpandarg = Some("-kkvl");
            }

            fro_zclose(&mut work.fro);

            if DIFF_L && diff_label2.is_some() {
                if revnums == 2 {
                    *diff_label2.as_mut().unwrap() = setup_label(Some(target.num), &target.date);
                } else {
                    time2date(work.st.st_mtime, &mut date2);
                    *diff_label2.as_mut().unwrap() = setup_label(None, &date2);
                }
            }

            let commarg = minus_p(xrev1.as_deref().unwrap(), rev1);

            let mut pp = COMMAND_LINE_VARYING;
            cov[pp] = Some(commarg);
            pp += 1;
            if let Some(arg) = lexpandarg {
                cov[pp] = Some(arg.to_string());
                pp += 1;
            }
            if let Some(arg) = suffixarg.as_deref() {
                cov[pp] = Some(arg.to_string());
                pp += 1;
            }
            if let Some(arg) = versionarg.as_deref() {
                cov[pp] = Some(arg.to_string());
                pp += 1;
            }
            if let Some(arg) = zonearg.as_deref() {
                cov[pp] = Some(arg.to_string());
                pp += 1;
            }
            cov[pp] = Some(REPO.filename.to_string());
            pp += 1;
            cov[pp] = None;

            let mut diffp = diffv.len();
            if OPEN_O_BINARY && kws == kwsub_b {
                diffv.push("--binary".to_string());
                diffp += 1;
            }

            diffv.push(maketemp(0));
            if runv(-1, &diffv[diffp], &cov) != 0 {
                RERR("co failed");
                continue;
            }

            if rev2.is_none() {
                diffv.push(mani_filename.to_string());
                if mani_filename.starts_with('-') {
                    diffv.push(format!(".{}", SLASH));
                    diffv.push(mani_filename.to_string());
                }
            } else {
                let commarg = minus_p(xrev2.as_deref().unwrap(), rev2.as_deref().unwrap());
                cov[COMMAND_LINE_VARYING] = Some(commarg);
                diffv.push(maketemp(1));
                if runv(-1, &diffv[diffp + 1], &cov) != 0 {
                    RERR("co failed");
                    continue;
                }
            }

            if rev2.is_none() {
                println!("diff -r{} {}", xrev1.unwrap(), mani_filename);
            } else {
                println!("diff -r{} -r{}", xrev1.unwrap(), xrev2.unwrap());
            }

            diffv.push(String::new());
            let status = runv(-1, "", &diffv);
            if status == DIFF_TROUBLE {
                MERR("diff failed");
            }
            if status == DIFF_FAILURE && exitstatus == DIFF_SUCCESS {
                exitstatus = status;
            }
        }
    }

    tempunlink();
    gnurcs_goodbye();
    exitstatus
}

static RCSDIFF_AKA: [u8; 14] = [
    2, // count
    4, b'd', b'i', b'f', b'f',
    7, b'r', b'c', b's', b'd', b'i', b'f', b'f'
];

/*:help
[options] file ...
Options:
  -rREV         (zero, one, or two times) Name a revision.
  -kSUBST       Substitute using mode SUBST (see co(1)).
  -q            Quiet mode.
  -T            No effect; included for compatibility with other commands.
  -V            Obsolete; do not use.
  -VN           Emulate RCS version N.
  -xSUFF        Specify SUFF as a slash-separated list of suffixes
                used to identify RCS file names.
  -zZONE        Specify date output format in keyword-substitution.

If given two revisions (-rREV1 -rREV2), compare those revisions.
If given only one revision (-rREV), compare the working file with it.
If given no revisions, compare the working file with the latest
revision on the default branch.

Additionally, the following options (and their argument, if any) are
passed to the underlying diff(1) command:
  -0, -1, -2, -3, -4, -5, -6, -7, -8, -9, -B, -C, -D, -F, -H, -I,
  -L, -U, -W, -a, -b, -c, -d, -e, -f, -h, -i, -n, -p, -t, -u, -w, -y,
  [long options (that start with "--")].
(Not all of these options are meaningful.)
*/