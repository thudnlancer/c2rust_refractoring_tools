/*
 * Create font map for AFM files.
 * Copyright (c) 1995, 1996, 1997 Markku Rossi.
 *
 * Author: Markku Rossi <mtr@iki.fi>
 */

/*
 * This file is part of GNU enscript.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; see the file COPYING.  If not, write to
 * the Free Software Foundation, 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

use std::env;
use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process;
use getopts::Options;
use afm::{AFMError, AFMHandle, AFMFont};

// Constants
const DEFAULT_OUTPUT_FILE: &str = "font.map";

// Program name
static mut PROGRAM: Option<String> = None;

fn program_name() -> &'static str {
    unsafe { PROGRAM.as_deref().unwrap_or("afm2map") }
}

fn handle_error(msg: &str, error: AFMError) -> ! {
    let buf = error.to_string();
    eprintln!("{}: {}: {}", program_name(), msg, buf);
    process::exit(1);
}

fn usage(opts: &Options) {
    let brief = format!(
        "Usage: {} [OPTION]... FILE...\n\
         Mandatory arguments to long options are mandatory for short options too.",
        program_name()
    );
    print!("{}", opts.usage(&brief));
}

fn main() {
    // Set program name
    unsafe {
        PROGRAM = env::args()
            .next()
            .as_ref()
            .map(|s| Path::new(s).file_name().unwrap().to_string_lossy().into_owned())
            .or_else(|| Some("afm2map".to_string()));
    }

    // Internationalization would be handled here in Rust
    // For simplicity, we're omitting the NLS setup

    // Command line options
    let mut opts = Options::new();
    opts.optopt("p", "output-file", "print output to file NAME (default file is font.map). If FILE is `-`, leave output to stdout", "NAME");
    opts.optflag("h", "help", "print this help and exit");
    opts.optflag("V", "version", "print version number");

    let matches = match opts.parse(env::args().skip(1)) {
        Ok(m) => m,
        Err(f) => {
            eprintln!("{}", f);
            usage(&opts);
            process::exit(1);
        }
    };

    if matches.opt_present("h") {
        usage(&opts);
        return;
    }

    if matches.opt_present("V") {
        println!("{} for GNU enscript {}", program_name(), env!("CARGO_PKG_VERSION"));
        return;
    }

    let output_file = matches.opt_str("p").unwrap_or_else(|| DEFAULT_OUTPUT_FILE.to_string());
    let use_stdout = output_file == "-";
    let files = matches.free;

    // Open output file
    println!("file={}", if use_stdout { "stdout" } else { &output_file });

    let mut output: Box<dyn Write> = if use_stdout {
        Box::new(io::stdout())
    } else {
        match File::create(&output_file) {
            Ok(f) => Box::new(f),
            Err(e) => {
                eprintln!("{}: couldn't open output file \"{}\": {}", program_name(), output_file, e);
                process::exit(1);
            }
        }
    };

    let mut message_output: Box<dyn Write> = if use_stdout {
        Box::new(io::stderr())
    } else {
        Box::new(io::stdout())
    };

    // Initialize AFM library
    let afm = match afm::create(None, 0) {
        Ok(a) => a,
        Err(e) => handle_error("couldn't create AFM library", e),
    };

    for file in files {
        writeln!(message_output, "{}...", file).unwrap();
        match afm.open_file(afm::InfoLevel::Minimum, &file) {
            Ok(font) => {
                let cp = Path::new(&file)
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap_or(&file);

                let sf = cp.rfind('.');
                let len = sf.unwrap_or(cp.len());

                let font_name = &font.global_info().font_name;
                writeln!(output, "{:<30}\t{:.len$}", font_name, cp).unwrap();
                let _ = font.close();
            }
            Err(e) => {
                writeln!(message_output, "{}: {}", program_name(), e).unwrap();
            }
        }
    }
}