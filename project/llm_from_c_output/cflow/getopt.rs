use std::env;
use std::ffi::{CString, OsString};
use std::os::unix::ffi::OsStringExt;
use std::ptr;

#[derive(Debug, Clone)]
pub struct Option {
    pub name: String,
    pub has_arg: i32,
    pub flag: Option<Box<i32>>,
    pub val: i32,
}

#[derive(Debug, Clone)]
pub struct GetoptData {
    pub optind: i32,
    pub opterr: i32,
    pub optopt: i32,
    pub optarg: Option<String>,
    pub __initialized: bool,
    pub __nextchar: Option<String>,
    pub __posixly_correct: bool,
    pub __ordering: Ordering,
    pub __first_nonopt: i32,
    pub __last_nonopt: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ordering {
    Permute,
    ReturnInOrder,
    RequireOrder,
}

pub static mut optarg: Option<String> = None;
pub static mut optind: i32 = 1;
pub static mut opterr: i32 = 1;
pub static mut optopt: i32 = '?' as i32;

fn exchange(argv: &mut Vec<String>, d: &mut GetoptData) {
    let mut bottom = d.__first_nonopt;
    let mut middle = d.__last_nonopt;
    let mut top = d.optind;

    while top > middle && middle > bottom {
        if top - middle > middle - bottom {
            let len = middle - bottom;
            for i in 0..len {
                argv.swap((bottom + i) as usize, (top - (middle - bottom) + i as usize);
            }
            top -= len;
        } else {
            let len = top - middle;
            for i in 0..len {
                argv.swap((bottom + i) as usize, (middle + i) as usize);
            }
            bottom += len;
        }
    }

    d.__first_nonopt += d.optind - d.__last_nonopt;
    d.__last_nonopt = d.optind;
}

fn _getopt_initialize(
    argc: i32,
    argv: &[String],
    optstring: &str,
    posixly_correct: bool,
    d: &mut GetoptData,
) -> String {
    d.__first_nonopt = d.__last_nonopt = d.optind;
    d.__nextchar = None;

    d.__posixly_correct = posixly_correct || env::var("POSIXLY_CORRECT").is_ok();

    let mut optstring = optstring.to_string();
    if optstring.starts_with('-') {
        d.__ordering = Ordering::ReturnInOrder;
        optstring.remove(0);
    } else if optstring.starts_with('+') {
        d.__ordering = Ordering::RequireOrder;
        optstring.remove(0);
    } else if d.__posixly_correct {
        d.__ordering = Ordering::RequireOrder;
    } else {
        d.__ordering = Ordering::Permute;
    }

    optstring
}

fn _getopt_internal_r(
    argc: i32,
    argv: &mut Vec<String>,
    optstring: &str,
    longopts: Option<&[Option]>,
    longind: Option<&mut i32>,
    long_only: bool,
    posixly_correct: bool,
    d: &mut GetoptData,
) -> i32 {
    let print_errors = if optstring.starts_with(':') {
        false
    } else {
        d.opterr != 0
    };

    if argc < 1 {
        return -1;
    }

    d.optarg = None;

    if d.optind == 0 || !d.__initialized {
        if d.optind == 0 {
            d.optind = 1;
        }
        let new_optstring = _getopt_initialize(argc, argv, optstring, posixly_correct, d);
        d.__initialized = true;
        return _getopt_internal_r(
            argc,
            argv,
            &new_optstring,
            longopts,
            longind,
            long_only,
            posixly_correct,
            d,
        );
    }

    if d.__nextchar.is_none() || d.__nextchar.as_ref().unwrap().is_empty() {
        if d.__last_nonopt > d.optind {
            d.__last_nonopt = d.optind;
        }
        if d.__first_nonopt > d.optind {
            d.__first_nonopt = d.optind;
        }

        if d.__ordering == Ordering::Permute {
            if d.__first_nonopt != d.__last_nonopt && d.__last_nonopt != d.optind {
                exchange(argv, d);
            } else if d.__last_nonopt != d.optind {
                d.__first_nonopt = d.optind;
            }

            while d.optind < argc
                && (argv[d.optind as usize].len() <= 1
                    || !argv[d.optind as usize].starts_with('-'))
            {
                d.optind += 1;
            }
            d.__last_nonopt = d.optind;
        }

        if d.optind != argc && argv[d.optind as usize] == "--" {
            d.optind += 1;

            if d.__first_nonopt != d.__last_nonopt && d.__last_nonopt != d.optind {
                exchange(argv, d);
            } else if d.__first_nonopt == d.__last_nonopt {
                d.__first_nonopt = d.optind;
            }
            d.__last_nonopt = argc;

            d.optind = argc;
        }

        if d.optind == argc {
            if d.__first_nonopt != d.__last_nonopt {
                d.optind = d.__first_nonopt;
            }
            return -1;
        }

        if argv[d.optind as usize].len() <= 1 || !argv[d.optind as usize].starts_with('-') {
            if d.__ordering == Ordering::RequireOrder {
                return -1;
            }
            d.optarg = Some(argv[d.optind as usize].clone());
            d.optind += 1;
            return 1;
        }

        let next_char = if longopts.is_some() && argv[d.optind as usize].starts_with("--") {
            argv[d.optind as usize][2..].to_string()
        } else {
            argv[d.optind as usize][1..].to_string()
        };
        d.__nextchar = Some(next_char);
    }

    if longopts.is_some()
        && (argv[d.optind as usize].starts_with("--")
            || (long_only
                && (argv[d.optind as usize].len() > 2
                    || !optstring.contains(&argv[d.optind as usize][1..2]))))
    {
        let nameend = d.__nextchar.as_ref().unwrap().find('=').unwrap_or_else(|| {
            d.__nextchar.as_ref().unwrap().len()
        });
        let mut pfound = None;
        let mut exact = false;
        let mut ambig = false;
        let mut indfound = -1;

        for (i, opt) in longopts.unwrap().iter().enumerate() {
            if d.__nextchar.as_ref().unwrap().starts_with(&opt.name) {
                if nameend == opt.name.len() {
                    pfound = Some(opt);
                    indfound = i as i32;
                    exact = true;
                    break;
                } else if pfound.is_none() {
                    pfound = Some(opt);
                    indfound = i as i32;
                } else if long_only
                    || pfound.unwrap().has_arg != opt.has_arg
                    || pfound.unwrap().flag != opt.flag
                    || pfound.unwrap().val != opt.val
                {
                    ambig = true;
                }
            }
        }

        if ambig && !exact {
            if print_errors {
                eprintln!(
                    "{}: option '{}' is ambiguous",
                    argv[0],
                    argv[d.optind as usize]
                );
            }
            d.__nextchar = None;
            d.optind += 1;
            d.optopt = 0;
            return '?' as i32;
        }

        if let Some(pfound) = pfound {
            if let Some(longind) = longind {
                *longind = indfound;
            }
            d.optind += 1;

            if nameend < d.__nextchar.as_ref().unwrap().len() {
                if pfound.has_arg != 0 {
                    d.optarg = Some(d.__nextchar.as_ref().unwrap()[nameend + 1..].to_string());
                } else {
                    if print_errors {
                        if argv[d.optind as usize - 1].starts_with("--") {
                            eprintln!(
                                "{}: option '--{}' doesn't allow an argument",
                                argv[0],
                                pfound.name
                            );
                        } else {
                            eprintln!(
                                "{}: option '-{}' doesn't allow an argument",
                                argv[0],
                                pfound.name
                            );
                        }
                    }
                    d.__nextchar = None;
                    d.optopt = pfound.val;
                    return '?' as i32;
                }
            } else if pfound.has_arg == 1 {
                if d.optind < argc {
                    d.optarg = Some(argv[d.optind as usize].clone());
                    d.optind += 1;
                } else {
                    if print_errors {
                        eprintln!(
                            "{}: option '{}' requires an argument",
                            argv[0],
                            argv[d.optind as usize - 1]
                        );
                    }
                    d.__nextchar = None;
                    d.optopt = pfound.val;
                    return if optstring.starts_with(':') { ':' as i32 } else { '?' as i32 };
                }
            }

            d.__nextchar = None;
            if let Some(flag) = pfound.flag {
                *flag = pfound.val;
                return 0;
            }
            return pfound.val;
        }

        if !long_only || argv[d.optind as usize].starts_with("--") {
            if print_errors {
                if argv[d.optind as usize].starts_with("--") {
                    eprintln!(
                        "{}: unrecognized option '--{}'",
                        argv[0],
                        d.__nextchar.as_ref().unwrap()
                    );
                } else {
                    eprintln!(
                        "{}: unrecognized option '-{}'",
                        argv[0],
                        d.__nextchar.as_ref().unwrap()
                    );
                }
            }
            d.__nextchar = None;
            d.optind += 1;
            d.optopt = 0;
            return '?' as i32;
        }
    }

    let c = d.__nextchar.as_ref().unwrap().chars().next().unwrap();
    let temp = optstring.find(c);

    if d.__nextchar.as_ref().unwrap().len() == 1 {
        d.optind += 1;
    }

    if temp.is_none() || c == ':' {
        if print_errors {
            if d.__posixly_correct {
                eprintln!("{}: illegal option -- {}", argv[0], c);
            } else {
                eprintln!("{}: invalid option -- {}", argv[0], c);
            }
        }
        d.optopt = c as i32;
        return '?' as i32;
    }

    let temp = temp.unwrap();
    let optstring_chars: Vec<char> = optstring.chars().collect();

    if optstring_chars[temp + 1] == 'W' && optstring_chars[temp + 2] == ';' {
        if d.__nextchar.as_ref().unwrap().len() > 1 {
            d.optarg = Some(d.__nextchar.as_ref().unwrap()[1..].to_string());
            d.optind += 1;
        } else if d.optind == argc {
            if print_errors {
                eprintln!("{}: option requires an argument -- {}", argv[0], c);
            }
            d.optopt = c as i32;
            return if optstring.starts_with(':') { ':' as i32 } else { '?' as i32 };
        } else {
            d.optarg = Some(argv[d.optind as usize].clone());
            d.optind += 1;
        }

        let nameend = d.optarg.as_ref().unwrap().find('=').unwrap_or_else(|| {
            d.optarg.as_ref().unwrap().len()
        });

        let mut pfound = None;
        let mut exact = false;
        let mut ambig = false;
        let mut indfound = 0;

        for (i, opt) in longopts.unwrap().iter().enumerate() {
            if d.optarg.as_ref().unwrap().starts_with(&opt.name) {
                if nameend == opt.name.len() {
                    pfound = Some(opt);
                    indfound = i as i32;
                    exact = true;
                    break;
                } else if pfound.is_none() {
                    pfound = Some(opt);
                    indfound = i as i32;
                } else {
                    ambig = true;
                }
            }
        }

        if ambig && !exact {
            if print_errors {
                eprintln!(
                    "{}: option '-W {}' is ambiguous",
                    argv[0],
                    d.optarg.as_ref().unwrap()
                );
            }
            d.__nextchar = None;
            d.optind += 1;
            return '?' as i32;
        }

        if let Some(pfound) = pfound {
            if nameend < d.optarg.as_ref().unwrap().len() {
                if pfound.has_arg != 0 {
                    d.optarg = Some(d.optarg.as_ref().unwrap()[nameend + 1..].to_string());
                } else {
                    if print_errors {
                        eprintln!(
                            "{}: option '-W {}' doesn't allow an argument",
                            argv[0],
                            pfound.name
                        );
                    }
                    d.__nextchar = None;
                    return '?' as i32;
                }
            } else if pfound.has_arg == 1 {
                if d.optind < argc {
                    d.optarg = Some(argv[d.optind as usize].clone());
                    d.optind += 1;
                } else {
                    if print_errors {
                        eprintln!(
                            "{}: option '{}' requires an argument",
                            argv[0],
                            argv[d.optind as usize - 1]
                        );
                    }
                    d.__nextchar = None;
                    return if optstring.starts_with(':') { ':' as i32 } else { '?' as i32 };
                }
            }

            d.__nextchar = None;
            if let Some(longind) = longind {
                *longind = indfound;
            }
            if let Some(flag) = pfound.flag {
                *flag = pfound.val;
                return 0;
            }
            return pfound.val;
        }

        d.__nextchar = None;
        return 'W' as i32;
    }

    if optstring_chars[temp + 1] == ':' {
        if optstring_chars[temp + 2] == ':' {
            if d.__nextchar.as_ref().unwrap().len() > 1 {
                d.optarg = Some(d.__nextchar.as_ref().unwrap()[1..].to_string());
                d.optind += 1;
            } else {
                d.optarg = None;
            }
            d.__nextchar = None;
        } else {
            if d.__nextchar.as_ref().unwrap().len() > 1 {
                d.optarg = Some(d.__nextchar.as_ref().unwrap()[1..].to_string());
                d.optind += 1;
            } else if d.optind == argc {
                if print_errors {
                    eprintln!("{}: option requires an argument -- {}", argv[0], c);
                }
                d.optopt = c as i32;
                return if optstring.starts_with(':') { ':' as i32 } else { '?' as i32 };
            } else {
                d.optarg = Some(argv[d.optind as usize].clone());
                d.optind += 1;
            }
            d.__nextchar = None;
        }
    }

    c as i32
}

pub fn getopt(
    argc: i32,
    argv: &mut Vec<String>,
    optstring: &str,
    longopts: Option<&[Option]>,
    longind: Option<&mut i32>,
) -> i32 {
    let posixly_correct = env::var("POSIXLY_CORRECT").is_ok();
    let mut d = GetoptData {
        optind: unsafe { optind },
        opterr: unsafe { opterr },
        optopt: 0,
        optarg: None,
        __initialized: false,
        __nextchar: None,
        __posixly_correct: posixly_correct,
        __ordering: Ordering::Permute,
        __first_nonopt: 0,
        __last_nonopt: 0,
    };

    let result = _getopt_internal_r(
        argc,
        argv,
        optstring,
        longopts,
        longind,
        false,
        posixly_correct,
        &mut d,
    );

    unsafe {
        optind = d.optind;
        optarg = d.optarg;
        optopt = d.optopt;
    }

    result
}