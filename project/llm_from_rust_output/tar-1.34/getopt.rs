use std::env;
use std::ffi::{CStr, CString, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::ptr;
use std::str;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Ordering {
    RequireOrder,
    Permute,
    ReturnInOrder,
}

#[derive(Debug, Clone, Copy)]
struct Option {
    name: &'static str,
    has_arg: i32,
    flag: Option<&'static mut i32>,
    val: i32,
}

struct GetOpt {
    optind: i32,
    opterr: i32,
    optopt: i32,
    optarg: Option<String>,
    initialized: bool,
    nextchar: Option<String>,
    ordering: Ordering,
    first_nonopt: i32,
    last_nonopt: i32,
}

impl GetOpt {
    fn new() -> Self {
        GetOpt {
            optind: 1,
            opterr: 1,
            optopt: b'?' as i32,
            optarg: None,
            initialized: false,
            nextchar: None,
            ordering: Ordering::RequireOrder,
            first_nonopt: 0,
            last_nonopt: 0,
        }
    }

    fn initialize(&mut self, argc: i32, argv: &[String], optstring: &str, posixly_correct: bool) -> &str {
        if self.optind == 0 {
            self.optind = 1;
        }
        self.last_nonopt = self.optind;
        self.first_nonopt = self.last_nonopt;
        self.nextchar = None;

        let mut optstring = optstring;
        if optstring.starts_with('-') {
            self.ordering = Ordering::ReturnInOrder;
            optstring = &optstring[1..];
        } else if optstring.starts_with('+') {
            self.ordering = Ordering::RequireOrder;
            optstring = &optstring[1..];
        } else if posixly_correct || env::var_os("POSIXLY_CORRECT").is_some() {
            self.ordering = Ordering::RequireOrder;
        } else {
            self.ordering = Ordering::Permute;
        }

        self.initialized = true;
        optstring
    }

    fn exchange(&mut self, argv: &mut [String]) {
        let bottom = self.first_nonopt;
        let middle = self.last_nonopt;
        let top = self.optind;
        let mut tem;

        while top > middle && middle > bottom {
            if top - middle > middle - bottom {
                let len = middle - bottom;
                for i in 0..len {
                    tem = argv[(bottom + i) as usize].clone();
                    argv[(bottom + i) as usize] = argv[(top - (middle - bottom) + i) as usize].clone();
                    argv[(top - (middle - bottom) + i) as usize] = tem;
                }
                self.optind -= len;
            } else {
                let len = top - middle;
                for i in 0..len {
                    tem = argv[(bottom + i) as usize].clone();
                    argv[(bottom + i) as usize] = argv[(middle + i) as usize].clone();
                    argv[(middle + i) as usize] = tem;
                }
                self.first_nonopt += len;
            }
        }
        self.first_nonopt += self.optind - self.last_nonopt;
        self.last_nonopt = self.optind;
    }

    fn process_long_option(
        &mut self,
        argc: i32,
        argv: &[String],
        optstring: &str,
        longopts: &[Option],
        longind: &mut Option<usize>,
        long_only: bool,
        print_errors: bool,
        prefix: &str,
    ) -> i32 {
        let arg = argv[self.optind as usize].as_str();
        let nextchar = self.nextchar.as_ref().unwrap();
        let nameend = nextchar.find('=').unwrap_or(nextchar.len());
        let namelen = nameend;
        let name = &nextchar[..namelen];

        let mut pfound = None;
        let mut option_index = 0;

        for (i, opt) in longopts.iter().enumerate() {
            if opt.name == name {
                pfound = Some(opt);
                option_index = i;
                break;
            }
        }

        if pfound.is_none() {
            let mut ambig = false;
            let mut indfound = None;

            for (i, opt) in longopts.iter().enumerate() {
                if opt.name.starts_with(name) {
                    if pfound.is_some() {
                        if !long_only || pfound.unwrap().has_arg != opt.has_arg
                            || pfound.unwrap().flag != opt.flag || pfound.unwrap().val != opt.val
                        {
                            ambig = true;
                        }
                    } else {
                        pfound = Some(opt);
                        indfound = Some(i);
                    }
                }
            }

            if ambig {
                if print_errors {
                    eprintln!(
                        "{}: option '{}' is ambiguous",
                        argv[0],
                        prefix.to_owned() + name
                    );
                }
                self.nextchar = None;
                self.optind += 1;
                self.optopt = 0;
                return b'?' as i32;
            }
            option_index = indfound.unwrap();
        }

        let opt = pfound.unwrap();
        if opt.name != name {
            if !long_only
                || argv[self.optind as usize].chars().nth(1) == Some('-')
                || !optstring.contains(nextchar.chars().next().unwrap())
            {
                if print_errors {
                    eprintln!(
                        "{}: unrecognized option '{}'",
                        argv[0],
                        prefix.to_owned() + name
                    );
                }
                self.nextchar = None;
                self.optind += 1;
                self.optopt = 0;
                return b'?' as i32;
            }
            return -1;
        }

        self.optind += 1;
        self.nextchar = None;

        if nameend < nextchar.len() {
            if opt.has_arg != 0 {
                self.optarg = Some(nextchar[nameend + 1..].to_string());
            } else {
                if print_errors {
                    eprintln!(
                        "{}: option '{}' doesn't allow an argument",
                        argv[0],
                        opt.name
                    );
                }
                self.optopt = opt.val;
                return b'?' as i32;
            }
        } else if opt.has_arg == 1 {
            if self.optind < argc {
                self.optarg = Some(argv[self.optind as usize].clone());
                self.optind += 1;
            } else {
                if print_errors {
                    eprintln!(
                        "{}: option '{}' requires an argument",
                        argv[0],
                        opt.name
                    );
                }
                self.optopt = opt.val;
                return if optstring.starts_with(':') {
                    b':' as i32
                } else {
                    b'?' as i32
                };
            }
        }

        if let Some(idx) = longind {
            *idx = option_index;
        }

        if let Some(flag) = opt.flag {
            unsafe {
                *flag = opt.val;
            }
            return 0;
        }

        opt.val
    }

    fn getopt_internal(
        &mut self,
        argc: i32,
        argv: &mut [String],
        optstring: &str,
        longopts: Option<&[Option]>,
        longind: &mut Option<usize>,
        long_only: bool,
        posixly_correct: bool,
    ) -> i32 {
        let mut print_errors = self.opterr;
        if argc < 1 {
            return -1;
        }

        self.optarg = None;
        if !self.initialized {
            let optstring = self.initialize(argc, argv, optstring, posixly_correct);
        } else if optstring.starts_with('-') || optstring.starts_with('+') {
            let _ = &optstring[1..];
        }

        if optstring.starts_with(':') {
            print_errors = 0;
        }

        if self.nextchar.is_none() || self.nextchar.as_ref().unwrap().is_empty() {
            if self.last_nonopt > self.optind {
                self.last_nonopt = self.optind;
            }
            if self.first_nonopt > self.optind {
                self.first_nonopt = self.optind;
            }

            if self.ordering == Ordering::Permute {
                if self.first_nonopt != self.last_nonopt && self.last_nonopt != self.optind {
                    self.exchange(argv);
                } else if self.last_nonopt != self.optind {
                    self.first_nonopt = self.optind;
                }

                while self.optind < argc
                    && (argv[self.optind as usize].is_empty()
                        || !argv[self.optind as usize].starts_with('-')
                        || argv[self.optind as usize].len() == 1)
                {
                    self.optind += 1;
                }
                self.last_nonopt = self.optind;
            }

            if self.optind != argc && argv[self.optind as usize] == "--" {
                self.optind += 1;
                if self.first_nonopt != self.last_nonopt && self.last_nonopt != self.optind {
                    self.exchange(argv);
                } else if self.first_nonopt == self.last_nonopt {
                    self.first_nonopt = self.optind;
                }
                self.last_nonopt = argc;
                self.optind = argc;
            }

            if self.optind == argc {
                if self.first_nonopt != self.last_nonopt {
                    self.optind = self.first_nonopt;
                }
                return -1;
            }

            if !argv[self.optind as usize].starts_with('-')
                || argv[self.optind as usize].len() == 1
            {
                if self.ordering == Ordering::RequireOrder {
                    return -1;
                }
                self.optarg = Some(argv[self.optind as usize].clone());
                self.optind += 1;
                return 1;
            }

            if let Some(longopts) = longopts {
                if argv[self.optind as usize].starts_with("--") {
                    self.nextchar = Some(argv[self.optind as usize][2..].to_string());
                    return self.process_long_option(
                        argc,
                        argv,
                        optstring,
                        longopts,
                        longind,
                        long_only,
                        print_errors != 0,
                        "--",
                    );
                }

                if long_only
                    && (argv[self.optind as usize].len() > 2
                        || !optstring.contains(argv[self.optind as usize].chars().nth(1).unwrap()))
                {
                    self.nextchar = Some(argv[self.optind as usize][1..].to_string());
                    let code = self.process_long_option(
                        argc,
                        argv,
                        optstring,
                        longopts,
                        longind,
                        long_only,
                        print_errors != 0,
                        "-",
                    );
                    if code != -1 {
                        return code;
                    }
                }
            }

            self.nextchar = Some(argv[self.optind as usize][1..].to_string());
        }

        let c = self.nextchar.as_ref().unwrap().chars().next().unwrap();
        self.nextchar = Some(self.nextchar.as_ref().unwrap()[1..].to_string());
        if self.nextchar.as_ref().unwrap().is_empty() {
            self.optind += 1;
        }

        if !optstring.contains(c) || c == ':' || c == ';' {
            if print_errors != 0 {
                eprintln!(
                    "{}: invalid option -- '{}'",
                    argv[0],
                    c
                );
            }
            self.optopt = c as i32;
            return b'?' as i32;
        }

        if c == 'W' && optstring.contains("W;") && longopts.is_some() {
            if !self.nextchar.as_ref().unwrap().is_empty() {
                self.optarg = self.nextchar.clone();
            } else if self.optind == argc {
                if print_errors != 0 {
                    eprintln!(
                        "{}: option requires an argument -- '{}'",
                        argv[0],
                        c
                    );
                }
                self.optopt = c as i32;
                return if optstring.starts_with(':') {
                    b':' as i32
                } else {
                    b'?' as i32
                };
            } else {
                self.optarg = Some(argv[self.optind as usize].clone());
            }
            self.nextchar = self.optarg.clone();
            self.optarg = None;
            return self.process_long_option(
                argc,
                argv,
                optstring,
                longopts.unwrap(),
                longind,
                false,
                print_errors != 0,
                "-W ",
            );
        }

        if optstring.matches(c).count() > 1 && optstring.matches(':').count() >= 2 {
            if !self.nextchar.as_ref().unwrap().is_empty() {
                self.optarg = self.nextchar.clone();
                self.optind += 1;
            } else {
                self.optarg = None;
            }
            self.nextchar = None;
        } else if optstring.matches(':').count() >= 1 {
            if !self.nextchar.as_ref().unwrap().is_empty() {
                self.optarg = self.nextchar.clone();
                self.optind += 1;
            } else if self.optind == argc {
                if print_errors != 0 {
                    eprintln!(
                        "{}: option requires an argument -- '{}'",
                        argv[0],
                        c
                    );
                }
                self.optopt = c as i32;
                return if optstring.starts_with(':') {
                    b':' as i32
                } else {
                    b'?' as i32
                };
            } else {
                self.optarg = Some(argv[self.optind as usize].clone());
                self.optind += 1;
            }
            self.nextchar = None;
        }

        c as i32
    }
}

pub fn rpl_getopt(
    argc: i32,
    argv: &[String],
    optstring: &str,
) -> i32 {
    let mut getopt = GetOpt::new();
    getopt.getopt_internal(
        argc,
        &mut argv.to_vec(),
        optstring,
        None,
        &mut None,
        false,
        true,
    )
}

pub fn rpl_getopt_long(
    argc: i32,
    argv: &[String],
    optstring: &str,
    longopts: &[Option],
    longind: &mut Option<usize>,
) -> i32 {
    let mut getopt = GetOpt::new();
    getopt.getopt_internal(
        argc,
        &mut argv.to_vec(),
        optstring,
        Some(longopts),
        longind,
        false,
        true,
    )
}

pub fn rpl_getopt_long_only(
    argc: i32,
    argv: &[String],
    optstring: &str,
    longopts: &[Option],
    longind: &mut Option<usize>,
) -> i32 {
    let mut getopt = GetOpt::new();
    getopt.getopt_internal(
        argc,
        &mut argv.to_vec(),
        optstring,
        Some(longopts),
        longind,
        true,
        true,
    )
}