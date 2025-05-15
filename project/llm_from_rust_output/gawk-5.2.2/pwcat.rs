use std::ffi::CStr;
use std::io::{self, Write};

#[derive(Debug)]
struct Passwd {
    pw_name: String,
    pw_passwd: String,
    pw_uid: u32,
    pw_gid: u32,
    pw_gecos: String,
    pw_dir: String,
    pw_shell: String,
}

fn get_all_users() -> io::Result<Vec<Passwd>> {
    let mut users = Vec::new();
    
    unsafe {
        let mut p = libc::getpwent();
        while !p.is_null() {
            let pw = &*p;
            users.push(Passwd {
                pw_name: CStr::from_ptr(pw.pw_name).to_string_lossy().into_owned(),
                pw_passwd: CStr::from_ptr(pw.pw_passwd).to_string_lossy().into_owned(),
                pw_uid: pw.pw_uid,
                pw_gid: pw.pw_gid,
                pw_gecos: CStr::from_ptr(pw.pw_gecos).to_string_lossy().into_owned(),
                pw_dir: CStr::from_ptr(pw.pw_dir).to_string_lossy().into_owned(),
                pw_shell: CStr::from_ptr(pw.pw_shell).to_string_lossy().into_owned(),
            });
            p = libc::getpwent();
        }
        libc::endpwent();
    }
    
    Ok(users)
}

fn main() -> io::Result<()> {
    let users = get_all_users()?;
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    for user in users {
        writeln!(
            handle,
            "{}:{}:{}:{}:{}:{}:{}",
            user.pw_name,
            user.pw_passwd,
            user.pw_uid,
            user.pw_gid,
            user.pw_gecos,
            user.pw_dir,
            user.pw_shell
        )?;
    }
    
    Ok(())
}