use std::ffi::CString;
use std::ptr;
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStringExt;
use std::path::Path;
use std::io;
use std::str::FromStr;
use std::num::ParseIntError;
use libc::{uid_t, gid_t};
use nix::unistd::{User, Group};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseUserSpecError {
    #[error("can not omit both user and group")]
    BothOmitted,
    #[error("invalid user")]
    InvalidUser,
    #[error("cannot get the login group of a numeric UID")]
    LoginGroupOfNumericUid,
    #[error("invalid group")]
    InvalidGroup,
    #[error("memory allocation failed")]
    MemoryAllocationFailed,
    #[error("parse error: {0}")]
    ParseError(#[from] ParseIntError),
}

pub fn parse_user_spec(
    spec_arg: &str,
) -> Result<(Option<String>, Option<String>, uid_t, gid_t), ParseUserSpecError> {
    let mut username = None;
    let mut groupname = None;
    let mut uid = 0;
    let mut gid = 0;

    let separator_pos = spec_arg.find(|c| c == ':' || c == '.');

    let (user_part, group_part) = if let Some(pos) = separator_pos {
        let (u, g) = spec_arg.split_at(pos);
        (if u.is_empty() { None } else { Some(u) }, 
         if g.len() <= 1 { None } else { Some(&g[1..]) })
    } else {
        (Some(spec_arg), None)
    };

    if user_part.is_none() && group_part.is_none() {
        return Err(ParseUserSpecError::BothOmitted);
    }

    if let Some(u) = user_part {
        let user_str = if u.starts_with('+') { &u[1..] } else { u };
        
        if let Ok(parsed_uid) = user_str.parse::<uid_t>() {
            uid = parsed_uid;
            if separator_pos.is_some() && group_part.is_none() {
                return Err(ParseUserSpecError::LoginGroupOfNumericUid);
            }
        } else {
            let user = User::from_name(user_str)
                .map_err(|_| ParseUserSpecError::InvalidUser)?
                .ok_or(ParseUserSpecError::InvalidUser)?;
            
            uid = user.uid;
            if group_part.is_none() && separator_pos.is_some() {
                gid = user.gid;
                groupname = Some(
                    Group::from_gid(user.gid)
                        .map_err(|_| ParseUserSpecError::InvalidGroup)?
                        .and_then(|g| g.name)
                    .ok_or(ParseUserSpecError::InvalidGroup)?;
            }
        }

        username = Some(user_str.to_string());
    }

    if let Some(g) = group_part {
        let group_str = if g.starts_with('+') { &g[1..] } else { g };
        
        if let Ok(parsed_gid) = group_str.parse::<gid_t>() {
            gid = parsed_gid;
        } else {
            let group = Group::from_name(group_str)
                .map_err(|_| ParseUserSpecError::InvalidGroup)?
                .ok_or(ParseUserSpecError::InvalidGroup)?;
            gid = group.gid;
        }

        groupname = Some(group_str.to_string());
    }

    Ok((username, groupname, uid, gid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_user_spec() {
        let test_cases = vec![
            ("user", Ok((Some("user".to_string()), None, 1000, 0))),
            ("user:group", Ok((Some("user".to_string()), Some("group".to_string()), 1000, 1000))),
            (":group", Ok((None, Some("group".to_string()), 0, 1000))),
            ("user:", Err(ParseUserSpecError::LoginGroupOfNumericUid)),
            ("1000:1000", Ok((Some("1000".to_string()), Some("1000".to_string()), 1000, 1000))),
            ("1000:", Err(ParseUserSpecError::LoginGroupOfNumericUid)),
            (":", Err(ParseUserSpecError::BothOmitted)),
        ];

        for (input, expected) in test_cases {
            assert_eq!(parse_user_spec(input).map_err(|e| e.to_string()), 
                       expected.map_err(|e| e.to_string()));
        }
    }
}