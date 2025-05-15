use std::ffi::CStr;
use std::os::unix::fs::MetadataExt;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::env;
use std::collections::LinkedList;
use std::path::Path;
use nix::unistd::{Uid, getuid, geteuid, seteuid};
use nix::sys::stat::stat;
use libc::{uid_t, getlogin_r, getpwuid_r, passwd};
use std::ffi::CString;
use std::io::{self, Error, ErrorKind};

static RUID_CACHED: AtomicBool = AtomicBool::new(false);
static RUID: AtomicUid = AtomicUid::new(Uid::from_raw(0));
static EUID_CACHED: AtomicBool = AtomicBool::new(false);
static EUID: AtomicUid = AtomicUid::new(Uid::from_raw(0));
static STICK_WITH_EUID: AtomicBool = AtomicBool::new(false);
static ALREADY_SETUID: AtomicBool = AtomicBool::new(false);
static USERNAME: OnceCell<String> = OnceCell::new();

struct Link<T> {
    next: Option<Box<Link<T>>>,
    entry: T,
}

struct RcsLock {
    login: String,
    delta: Delta,
}

struct Delta {
    num: String,
    lockedby: Option<String>,
}

fn ruid() -> uid_t {
    if !RUID_CACHED.load(Ordering::SeqCst) {
        RUID.store(getuid().as_raw(), Ordering::SeqCst);
        RUID_CACHED.store(true, Ordering::SeqCst);
    }
    RUID.load(Ordering::SeqCst)
}

fn stat_mine_p(st: &std::fs::Metadata) -> bool {
    ruid() == st.uid()
}

fn euid() -> uid_t {
    if !EUID_CACHED.load(Ordering::SeqCst) {
        EUID.store(geteuid().as_raw(), Ordering::SeqCst);
        EUID_CACHED.store(true, Ordering::SeqCst);
    }
    EUID.load(Ordering::SeqCst)
}

fn currently_setuid_p() -> bool {
    euid() != ruid()
}

fn set_uid_to(u: uid_t) -> Result<(), nix::Error> {
    if !currently_setuid_p() {
        return Ok(());
    }
    
    if seteuid(Uid::from_raw(u)).is_err() {
        if ALREADY_SETUID.load(Ordering::SeqCst) {
            return Ok(());
        }
        ALREADY_SETUID.store(true, Ordering::SeqCst);
        return Err(Error::new(ErrorKind::PermissionDenied, "root setuid not supported"));
    }
    Ok(())
}

fn nosetid() {
    STICK_WITH_EUID.store(true, Ordering::SeqCst);
}

fn seteid() -> Result<(), nix::Error> {
    if !STICK_WITH_EUID.load(Ordering::SeqCst) {
        set_uid_to(euid())
    } else {
        Ok(())
    }
}

fn setrid() -> Result<(), nix::Error> {
    if !STICK_WITH_EUID.load(Ordering::SeqCst) {
        set_uid_to(ruid())
    } else {
        Ok(())
    }
}

fn getusername(suspicious: bool) -> Result<String, io::Error> {
    if let Some(username) = USERNAME.get() {
        return Ok(username.clone());
    }

    let username = if suspicious {
        get_username_secure()?
    } else {
        get_username_fast()?
    };

    USERNAME.set(username.clone()).unwrap();
    Ok(username)
}

fn get_username_fast() -> Result<String, io::Error> {
    const CONSULT_FIRST: &str = if cfg!(USER_OVER_LOGNAME) { "USER" } else { "LOGNAME" };
    const CONSULT_SECOND: &str = if cfg!(USER_OVER_LOGNAME) { "LOGNAME" } else { "USER" };

    if let Ok(user) = env::var(CONSULT_FIRST) {
        return Ok(user);
    }
    if let Ok(user) = env::var(CONSULT_SECOND) {
        return Ok(user);
    }

    let mut buf = [0; 256];
    unsafe {
        if getlogin_r(buf.as_mut_ptr(), buf.len()) == 0 {
            if let Ok(s) = CStr::from_ptr(buf.as_ptr()).to_str() {
                return Ok(s.to_string());
            }
        }
    }

    get_username_secure()
}

fn get_username_secure() -> Result<String, io::Error> {
    let mut buf = [0; 1024];
    let mut passwd = unsafe { std::mem::zeroed::<passwd>() };
    let mut result = ptr::null_mut();

    unsafe {
        if getpwuid_r(
            ruid(),
            &mut passwd,
            buf.as_mut_ptr(),
            buf.len(),
            &mut result,
        ) != 0
        {
            return Err(io::Error::last_os_error());
        }
    }

    if result.is_null() {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("no password entry for userid {}", ruid()),
        ));
    }

    unsafe {
        let name = CStr::from_ptr(passwd.pw_name)
            .to_str()
            .map_err(|_| Error::new(ErrorKind::InvalidData, "invalid username"))?;
        Ok(name.to_string())
    }
}

fn getcaller() -> Result<String, io::Error> {
    getusername(currently_setuid_p())
}

fn caller_login_p(login: &str) -> Result<bool, io::Error> {
    Ok(getcaller()? == login)
}

fn lock_memq(ls: &mut Link<RcsLock>, login: bool, x: &str) -> Option<&mut Link<RcsLock>> {
    let mut current = ls;
    while let Some(next) = &mut current.next {
        let rl = &next.entry;
        if (login && rl.login == x) || (!login && rl.delta.num == x) {
            return Some(current);
        }
        current = next;
    }
    None
}

fn lock_on(delta: &Delta, locks: &LinkedList<RcsLock>) -> Option<&RcsLock> {
    locks.iter().find(|rl| rl.delta.num == delta.num)
}

fn lock_drop(box_: &mut Link<RcsLock>, tp: &mut Link<RcsLock>) {
    if let Some(next) = tp.next.take() {
        tp.next = next.next;
        let rl = next.entry;
        rl.delta.lockedby = None;
    }
}

fn addlock_maybe(
    delta: &mut Delta,
    selfsame: bool,
    verbose: bool,
    locks: &mut LinkedList<RcsLock>,
) -> Result<i32, io::Error> {
    if let Some(was) = lock_on(delta, locks) {
        if !selfsame && caller_login_p(&was.login)? {
            return Ok(0);
        }
        if verbose {
            return Err(Error::new(
                ErrorKind::PermissionDenied,
                format!("Revision {} is already locked by {}.", delta.num, was.login),
            ));
        }
        return Ok(-1);
    }

    let caller = getcaller()?;
    let rl = RcsLock {
        login: caller.clone(),
        delta: Delta {
            num: delta.num.clone(),
            lockedby: Some(caller),
        },
    };
    locks.push_front(rl);
    Ok(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stat_mine_p() {
        let meta = std::fs::metadata(".").unwrap();
        assert!(stat_mine_p(&meta));
    }
}