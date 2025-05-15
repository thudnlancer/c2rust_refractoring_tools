/* Three-way file merge.

   Copyright (C) 2010-2020 Thien-Thi Nguyen
   Copyright (C) 1991, 1992, 1993, 1994, 1995 Paul Eggert

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

use std::env;
use std::process;
use std::ffi::OsString;
use std::path::PathBuf;

struct Symdef {
    fname: Option<PathBuf>,
    label: Option<String>,
}

struct ProgramOptions {
    quiet: bool,
    to_stdout: bool,
    edarg: Option<String>,
    labels: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = ProgramOptions {
        quiet: false,
        to_stdout: false,
        edarg: None,
        labels: Vec::new(),
    };

    let mut three_manifestations = [
        Symdef { fname: None, label: None },
        Symdef { fname: None, label: None },
        Symdef { fname: None, label: None },
    ];

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if !arg.starts_with('-') {
            break;
        }

        let option = arg.chars().nth(1).unwrap_or('\0');
        match option {
            'A' | 'E' | 'e' => {
                if let Some(ref ed) = opts.edarg {
                    if ed.chars().nth(1) != arg.chars().nth(1) {
                        eprintln!("{} and {} are incompatible", ed, arg);
                        process::exit(1);
                    }
                }
                opts.edarg = Some(arg.clone());
            },
            'p' => {
                opts.to_stdout = true;
            },
            'q' => {
                opts.quiet = true;
            },
            'L' => {
                if opts.labels.len() >= 3 {
                    eprintln!("too many -L options");
                    process::exit(1);
                }
                i += 1;
                if i >= args.len() {
                    eprintln!("-L needs following argument");
                    process::exit(1);
                }
                opts.labels.push(args[i].clone());
            },
            'V' => {
                if arg.len() > 2 {
                    eprintln!("invalid option: {}", arg);
                    process::exit(1);
                } else {
                    // Display version and exit
                    println!("GNU RCS merge version info");
                    process::exit(0);
                }
            },
            _ => {
                eprintln!("invalid option: {}", arg);
                process::exit(1);
            },
        }
        if arg.len() > 2 {
            eprintln!("invalid option: {}", arg);
            process::exit(1);
        }
        i += 1;
    }

    let remaining_args = &args[i..];
    if remaining_args.len() != 3 {
        eprintln!("{} arguments",
            if remaining_args.len() < 3 { "not enough" } else { "too many" });
        process::exit(1);
    }

    for (idx, arg) in remaining_args.iter().enumerate().take(3) {
        three_manifestations[idx].fname = Some(PathBuf::from(arg));
        three_manifestations[idx].label = if idx < opts.labels.len() {
            Some(opts.labels[idx].clone())
        } else {
            Some(arg.clone())
        };
    }

    // TODO: Implement merge function
    let exit_status = merge(opts, three_manifestations);
    process::exit(exit_status);
}

fn merge(_opts: ProgramOptions, _manifestations: [Symdef; 3]) -> i32 {
    // TODO: Implement actual merge functionality
    0
}

/*:help
[options] receiving-sibling parent other-sibling
Options:
  -A            Use `diff3 -A' style.
  -E            Use `diff3 -E' style (default).
  -e            Use `diff3 -e' style.
  -p            Write to stdout instead of overwriting RECEIVING-SIBLING.
  -q            Quiet mode; suppress conflict warnings.
  -L LABEL      (up to three times) Specify the conflict labels for
                RECEIVING-SIBLING, PARENT and OTHER-SIBLING, respectively.
  -V            Obsolete; do not use.
*/

/* merge.rs ends here */