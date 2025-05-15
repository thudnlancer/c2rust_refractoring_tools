use std::ffi::{CStr, CString};
use std::ptr;
use std::str::FromStr;
use libc::{uid_t, gid_t, c_char, c_int};
use std::mem::MaybeUninit;
use std::os::raw::c_void;

#[derive(Debug)]
struct Passwd {
    pw_name: Option<CString>,
    pw_passwd: Option<CString>,
    pw_uid: uid_t,
    pw_gid: gid_t,
    pw_gecos: Option<CString>,
    pw_dir: Option<CString>,
    pw_shell: Option<CString>,
}

#[derive(Debug)]
struct Group {
    gr_name: Option<CString>,
    gr_passwd: Option<CString>,
    gr_gid: gid_t,
    gr_mem: Vec<Option<CString>>,
}

fn is_number(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_digit())
}

fn parse_user_spec(
    spec_arg: &str,
) -> Result<(Option<uid_t>, Option<gid_t>, Option<CString>, Option<CString>), String> {
    let mut uid = None;
    let mut gid = None;
    let mut username = None;
    let mut groupname = None;

    let separator_pos = spec_arg.find(|c| c == ':' || c == '.');

    let (u_part, g_part) = if let Some(pos) = separator_pos {
        let (u, rest) = spec_arg.split_at(pos);
        let g = &rest[1..];
        (u, g)
    } else {
        (spec_arg, "")
    };

    if u_part.is_empty() && g_part.is_empty() {
        return Err("cannot omit both user and group".to_string());
    }

    // Parse user part
    if !u_part.is_empty() {
        if u_part.starts_with('+') {
            let num_str = &u_part[1..];
            if is_number(num_str) {
                uid = Some(num_str.parse().map_err(|_| "invalid user numeric ID")?);
            } else {
                return Err("invalid user".to_string());
            }
        } else {
            if is_number(u_part) {
                uid = Some(u_part.parse().map_err(|_| "invalid user numeric ID")?);
                if !g_part.is_empty() && separator_pos.is_some() {
                    return Err("cannot get the login group of a numeric UID".to_string());
                }
            } else {
                // In real implementation, would look up in /etc/passwd
                return Err("user lookup not implemented".to_string());
            }
        }
    }

    // Parse group part
    if !g_part.is_empty() {
        if g_part.starts_with('+') {
            let num_str = &g_part[1..];
            if is_number(num_str) {
                gid = Some(num_str.parse().map_err(|_| "invalid group numeric ID")?);
            } else {
                return Err("invalid group".to_string());
            }
        } else {
            if is_number(g_part) {
                gid = Some(g_part.parse().map_err(|_| "invalid group numeric ID")?);
            } else {
                // In real implementation, would look up in /etc/group
                return Err("group lookup not implemented".to_string());
            }
        }
        groupname = Some(CString::new(g_part).map_err(|_| "invalid group name")?);
    }

    if !u_part.is_empty() {
        username = Some(CString::new(u_part).map_err(|_| "invalid user name")?);
    }

    Ok((uid, gid, username, groupname))
}