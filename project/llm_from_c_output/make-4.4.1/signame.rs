use std::ffi::CString;
use std::fmt::Write;

lazy_static::lazy_static! {
    static ref UNDOC: String = "unknown signal".to_string();
    static ref SYS_SIGLIST: Vec<String> = {
        let mut list = vec![UNDOC.clone(); NSIG];
        init_signal_names(&mut list);
        list
    };
    static ref SIG_TABLE: Vec<NumAbbrev> = {
        let mut table = Vec::with_capacity(SIG_TABLE_SIZE);
        init_signal_table(&mut table);
        table
    };
}

const NSIG: usize = 32;
const SIG_TABLE_SIZE: usize = NSIG * 2;

#[derive(Clone)]
struct NumAbbrev {
    number: i32,
    abbrev: String,
}

fn init_signal_names(list: &mut Vec<String>) {
    macro_rules! init_sig {
        ($number:expr, $abbrev:expr, $name:expr) => {
            if $number >= 0 && ($number as usize) < list.len() {
                list[$number as usize] = $name.to_string();
            }
        };
    }

    // Initialize signal names
    #[cfg(feature = "SIGHUP")]
    init_sig!(libc::SIGHUP, "HUP", "Hangup");
    #[cfg(feature = "SIGINT")]
    init_sig!(libc::SIGINT, "INT", "Interrupt");
    #[cfg(feature = "SIGQUIT")]
    init_sig!(libc::SIGQUIT, "QUIT", "Quit");
    #[cfg(feature = "SIGILL")]
    init_sig!(libc::SIGILL, "ILL", "Illegal Instruction");
    #[cfg(feature = "SIGTRAP")]
    init_sig!(libc::SIGTRAP, "TRAP", "Trace/breakpoint trap");
    #[cfg(feature = "SIGABRT")]
    init_sig!(libc::SIGABRT, "ABRT", "Aborted");
    #[cfg(feature = "SIGIOT")]
    init_sig!(libc::SIGIOT, "IOT", "IOT trap");
    #[cfg(feature = "SIGEMT")]
    init_sig!(libc::SIGEMT, "EMT", "EMT trap");
    #[cfg(feature = "SIGFPE")]
    init_sig!(libc::SIGFPE, "FPE", "Floating point exception");
    #[cfg(feature = "SIGKILL")]
    init_sig!(libc::SIGKILL, "KILL", "Killed");
    #[cfg(feature = "SIGBUS")]
    init_sig!(libc::SIGBUS, "BUS", "Bus error");
    #[cfg(feature = "SIGSEGV")]
    init_sig!(libc::SIGSEGV, "SEGV", "Segmentation fault");
    #[cfg(feature = "SIGSYS")]
    init_sig!(libc::SIGSYS, "SYS", "Bad system call");
    #[cfg(feature = "SIGPIPE")]
    init_sig!(libc::SIGPIPE, "PIPE", "Broken pipe");
    #[cfg(feature = "SIGALRM")]
    init_sig!(libc::SIGALRM, "ALRM", "Alarm clock");
    #[cfg(feature = "SIGTERM")]
    init_sig!(libc::SIGTERM, "TERM", "Terminated");
    #[cfg(feature = "SIGUSR1")]
    init_sig!(libc::SIGUSR1, "USR1", "User defined signal 1");
    #[cfg(feature = "SIGUSR2")]
    init_sig!(libc::SIGUSR2, "USR2", "User defined signal 2");
    #[cfg(feature = "SIGCHLD")]
    init_sig!(libc::SIGCHLD, "CHLD", "Child exited");
    #[cfg(feature = "SIGCLD")]
    init_sig!(libc::SIGCLD, "CLD", "Child exited");
    #[cfg(feature = "SIGPWR")]
    init_sig!(libc::SIGPWR, "PWR", "Power failure");
    #[cfg(feature = "SIGTSTP")]
    init_sig!(libc::SIGTSTP, "TSTP", "Stopped");
    #[cfg(feature = "SIGTTIN")]
    init_sig!(libc::SIGTTIN, "TTIN", "Stopped (tty input)");
    #[cfg(feature = "SIGTTOU")]
    init_sig!(libc::SIGTTOU, "TTOU", "Stopped (tty output)");
    #[cfg(feature = "SIGSTOP")]
    init_sig!(libc::SIGSTOP, "STOP", "Stopped (signal)");
    #[cfg(feature = "SIGXCPU")]
    init_sig!(libc::SIGXCPU, "XCPU", "CPU time limit exceeded");
    #[cfg(feature = "SIGXFSZ")]
    init_sig!(libc::SIGXFSZ, "XFSZ", "File size limit exceeded");
    #[cfg(feature = "SIGVTALRM")]
    init_sig!(libc::SIGVTALRM, "VTALRM", "Virtual timer expired");
    #[cfg(feature = "SIGPROF")]
    init_sig!(libc::SIGPROF, "PROF", "Profiling timer expired");
    #[cfg(feature = "SIGWINCH")]
    init_sig!(libc::SIGWINCH, "WINCH", "Window changed");
    #[cfg(feature = "SIGCONT")]
    init_sig!(libc::SIGCONT, "CONT", "Continued");
    #[cfg(feature = "SIGURG")]
    init_sig!(libc::SIGURG, "URG", "Urgent I/O condition");
    #[cfg(feature = "SIGIO")]
    init_sig!(libc::SIGIO, "IO", "I/O possible");
    #[cfg(feature = "SIGWIND")]
    init_sig!(libc::SIGWIND, "WIND", "SIGWIND");
    #[cfg(feature = "SIGPHONE")]
    init_sig!(libc::SIGPHONE, "PHONE", "SIGPHONE");
    #[cfg(feature = "SIGPOLL")]
    init_sig!(libc::SIGPOLL, "POLL", "I/O possible");
    #[cfg(feature = "SIGLOST")]
    init_sig!(libc::SIGLOST, "LOST", "Resource lost");
    #[cfg(feature = "SIGDANGER")]
    init_sig!(libc::SIGDANGER, "DANGER", "Danger signal");
    #[cfg(feature = "SIGINFO")]
    init_sig!(libc::SIGINFO, "INFO", "Information request");
    #[cfg(feature = "SIGNOFP")]
    init_sig!(libc::SIGNOFP, "NOFP", "Floating point co-processor not available");
}

fn init_signal_table(table: &mut Vec<NumAbbrev>) {
    macro_rules! add_sig {
        ($number:expr, $abbrev:expr) => {
            if table.len() < SIG_TABLE_SIZE {
                table.push(NumAbbrev {
                    number: $number,
                    abbrev: $abbrev.to_string(),
                });
            }
        };
    }

    #[cfg(feature = "SIGHUP")]
    add_sig!(libc::SIGHUP, "HUP");
    #[cfg(feature = "SIGINT")]
    add_sig!(libc::SIGINT, "INT");
    #[cfg(feature = "SIGQUIT")]
    add_sig!(libc::SIGQUIT, "QUIT");
    #[cfg(feature = "SIGILL")]
    add_sig!(libc::SIGILL, "ILL");
    #[cfg(feature = "SIGTRAP")]
    add_sig!(libc::SIGTRAP, "TRAP");
    #[cfg(feature = "SIGABRT")]
    add_sig!(libc::SIGABRT, "ABRT");
    #[cfg(feature = "SIGIOT")]
    add_sig!(libc::SIGIOT, "IOT");
    #[cfg(feature = "SIGEMT")]
    add_sig!(libc::SIGEMT, "EMT");
    #[cfg(feature = "SIGFPE")]
    add_sig!(libc::SIGFPE, "FPE");
    #[cfg(feature = "SIGKILL")]
    add_sig!(libc::SIGKILL, "KILL");
    #[cfg(feature = "SIGBUS")]
    add_sig!(libc::SIGBUS, "BUS");
    #[cfg(feature = "SIGSEGV")]
    add_sig!(libc::SIGSEGV, "SEGV");
    #[cfg(feature = "SIGSYS")]
    add_sig!(libc::SIGSYS, "SYS");
    #[cfg(feature = "SIGPIPE")]
    add_sig!(libc::SIGPIPE, "PIPE");
    #[cfg(feature = "SIGALRM")]
    add_sig!(libc::SIGALRM, "ALRM");
    #[cfg(feature = "SIGTERM")]
    add_sig!(libc::SIGTERM, "TERM");
    #[cfg(feature = "SIGUSR1")]
    add_sig!(libc::SIGUSR1, "USR1");
    #[cfg(feature = "SIGUSR2")]
    add_sig!(libc::SIGUSR2, "USR2");
    #[cfg(feature = "SIGCHLD")]
    add_sig!(libc::SIGCHLD, "CHLD");
    #[cfg(feature = "SIGCLD")]
    add_sig!(libc::SIGCLD, "CLD");
    #[cfg(feature = "SIGPWR")]
    add_sig!(libc::SIGPWR, "PWR");
    #[cfg(feature = "SIGTSTP")]
    add_sig!(libc::SIGTSTP, "TSTP");
    #[cfg(feature = "SIGTTIN")]
    add_sig!(libc::SIGTTIN, "TTIN");
    #[cfg(feature = "SIGTTOU")]
    add_sig!(libc::SIGTTOU, "TTOU");
    #[cfg(feature = "SIGSTOP")]
    add_sig!(libc::SIGSTOP, "STOP");
    #[cfg(feature = "SIGXCPU")]
    add_sig!(libc::SIGXCPU, "XCPU");
    #[cfg(feature = "SIGXFSZ")]
    add_sig!(libc::SIGXFSZ, "XFSZ");
    #[cfg(feature = "SIGVTALRM")]
    add_sig!(libc::SIGVTALRM, "VTALRM");
    #[cfg(feature = "SIGPROF")]
    add_sig!(libc::SIGPROF, "PROF");
    #[cfg(feature = "SIGWINCH")]
    add_sig!(libc::SIGWINCH, "WINCH");
    #[cfg(feature = "SIGCONT")]
    add_sig!(libc::SIGCONT, "CONT");
    #[cfg(feature = "SIGURG")]
    add_sig!(libc::SIGURG, "URG");
    #[cfg(feature = "SIGIO")]
    add_sig!(libc::SIGIO, "IO");
    #[cfg(feature = "SIGWIND")]
    add_sig!(libc::SIGWIND, "WIND");
    #[cfg(feature = "SIGPHONE")]
    add_sig!(libc::SIGPHONE, "PHONE");
    #[cfg(feature = "SIGPOLL")]
    add_sig!(libc::SIGPOLL, "POLL");
    #[cfg(feature = "SIGLOST")]
    add_sig!(libc::SIGLOST, "LOST");
    #[cfg(feature = "SIGDANGER")]
    add_sig!(libc::SIGDANGER, "DANGER");
    #[cfg(feature = "SIGINFO")]
    add_sig!(libc::SIGINFO, "INFO");
    #[cfg(feature = "SIGNOFP")]
    add_sig!(libc::SIGNOFP, "NOFP");
}

pub fn strsignal(sig: i32) -> String {
    if sig > 0 && (sig as usize) < SYS_SIGLIST.len() {
        return SYS_SIGLIST[sig as usize].clone();
    }

    format!("Signal {}", sig)
}