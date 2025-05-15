use std::env;
use std::fs::{self, File};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::process;

const LOCAL_RC: &str = ".cflowrc";

fn expand_argcv(argc_ptr: &mut usize, argv_ptr: &mut Vec<OsString>, argv: Vec<OsString>) {
    argv_ptr.extend(argv);
    *argc_ptr = argv_ptr.len();
}

fn parse_rc(argc_ptr: &mut usize, argv_ptr: &mut Vec<OsString>, name: &Path) -> io::Result<()> {
    let buf = fs::read_to_string(name)?;
    
    for line in buf.lines() {
        let args: Vec<OsString> = line.split_whitespace()
            .map(OsString::from)
            .collect();
        expand_argcv(argc_ptr, argv_ptr, args);
    }
    
    Ok(())
}

fn sourcerc(argc_ptr: &mut usize, argv_ptr: &mut Vec<OsString>) -> io::Result<()> {
    let mut xargv = vec![argv_ptr[0].clone()];
    let mut xargc = 1;

    if let Some(env) = env::var_os("CFLOW_OPTIONS") {
        let args: Vec<OsString> = env.to_string_lossy()
            .split_whitespace()
            .map(OsString::from)
            .collect();
        expand_argcv(&mut xargc, &mut xargv, args);
    }

    if let Some(env_path) = env::var_os("CFLOWRC") {
        parse_rc(&mut xargc, &mut xargv, Path::new(&env_path))?;
    } else if let Some(home) = env::var_os("HOME") {
        let mut path = PathBuf::from(home);
        path.push(LOCAL_RC);
        if path.exists() {
            parse_rc(&mut xargc, &mut xargv, &path)?;
        }
    }

    if xargc > 1 {
        let rest_args = argv_ptr[1..].to_vec();
        expand_argcv(&mut xargc, &mut xargv, rest_args);
        *argc_ptr = xargc;
        *argv_ptr = xargv;
    }

    Ok(())
}

fn main() {
    let mut args: Vec<OsString> = env::args_os().collect();
    let mut argc = args.len();
    
    if let Err(e) = sourcerc(&mut argc, &mut args) {
        eprintln!("Error processing configuration: {}", e);
        process::exit(1);
    }
    
    // Rest of the program logic...
}