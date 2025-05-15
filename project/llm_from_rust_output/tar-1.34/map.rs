use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::unix::ffi::OsStrExt;
use std::path::Path;
use std::ptr;
use libc::{uid_t, gid_t, c_char, c_int, c_void, uintmax_t};
use nix::unistd::{User, Group};

struct MapEntry {
    orig_id: uintmax_t,
    new_id: uintmax_t,
    new_name: Option<CString>,
}

struct OwnerMap {
    map: HashMap<uintmax_t, MapEntry>,
    default_uid: Option<uid_t>,
    default_name: Option<CString>,
}

impl OwnerMap {
    fn new() -> Self {
        OwnerMap {
            map: HashMap::new(),
            default_uid: None,
            default_name: None,
        }
    }

    fn read(&mut self, file_path: &Path) -> Result<(), String> {
        let file = File::open(file_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| e.to_string())?;
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(format!("Malformed line at {}", line_num + 1));
            }

            let orig_id = parse_id(parts[0], "UID")?;
            let (new_id, new_name) = parse_mapping(parts[1], "UID")?;

            self.map.insert(orig_id, MapEntry {
                orig_id,
                new_id,
                new_name: new_name.map(|s| CString::new(s).unwrap()),
            });
        }

        Ok(())
    }

    fn translate(&self, uid: uid_t) -> (Option<uid_t>, Option<&CStr>) {
        if let Some(entry) = self.map.get(&(uid as uintmax_t)) {
            return (Some(entry.new_id as uid_t), entry.new_name.as_ref().map(|n| n.as_c_str()));
        }
        (self.default_uid, self.default_name.as_ref().map(|n| n.as_c_str()))
    }
}

fn parse_id(s: &str, what: &str) -> Result<uintmax_t, String> {
    if s.starts_with('+') {
        s[1..].parse().map_err(|_| format!("Invalid {}: {}", what, s))
    } else {
        User::from_name(s)
            .map(|u| u.map(|user| user.uid as uintmax_t))
            .unwrap_or(None)
            .ok_or_else(|| format!("Can't obtain {} of {}", what, s))
    }
}

fn parse_mapping(s: &str, what: &str) -> Result<(uintmax_t, Option<String>), String> {
    if let Some(colon) = s.find(':') {
        let name = if colon > 0 { Some(s[..colon].to_string()) } else { None };
        let id = parse_id(&s[colon + 1..], what)?;
        Ok((id, name))
    } else if s.starts_with('+') {
        Ok((parse_id(s, what)?, None))
    } else {
        let id = parse_id(s, what)?;
        Ok((id, Some(s.to_string())))
    }
}

struct GroupMap {
    map: HashMap<uintmax_t, MapEntry>,
    default_gid: Option<gid_t>,
    default_name: Option<CString>,
}

impl GroupMap {
    fn new() -> Self {
        GroupMap {
            map: HashMap::new(),
            default_gid: None,
            default_name: None,
        }
    }

    fn read(&mut self, file_path: &Path) -> Result<(), String> {
        let file = File::open(file_path).map_err(|e| e.to_string())?;
        let reader = BufReader::new(file);

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| e.to_string())?;
            if line.trim().is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                return Err(format!("Malformed line at {}", line_num + 1));
            }

            let orig_id = parse_gid(parts[0], "GID")?;
            let (new_id, new_name) = parse_mapping(parts[1], "GID")?;

            self.map.insert(orig_id, MapEntry {
                orig_id,
                new_id,
                new_name: new_name.map(|s| CString::new(s).unwrap()),
            });
        }

        Ok(())
    }

    fn translate(&self, gid: gid_t) -> (Option<gid_t>, Option<&CStr>) {
        if let Some(entry) = self.map.get(&(gid as uintmax_t)) {
            return (Some(entry.new_id as gid_t), entry.new_name.as_ref().map(|n| n.as_c_str()));
        }
        (self.default_gid, self.default_name.as_ref().map(|n| n.as_c_str()))
    }
}

fn parse_gid(s: &str, what: &str) -> Result<uintmax_t, String> {
    if s.starts_with('+') {
        s[1..].parse().map_err(|_| format!("Invalid {}: {}", what, s))
    } else {
        Group::from_name(s)
            .map(|g| g.map(|group| group.gid as uintmax_t))
            .unwrap_or(None)
            .ok_or_else(|| format!("Can't obtain {} of {}", what, s))
    }
}

#[no_mangle]
pub extern "C" fn owner_map_read(file: *const c_char) {
    let path = unsafe { CStr::from_ptr(file) }.to_str().unwrap();
    OWNER_MAP.read(Path::new(path)).unwrap();
}

#[no_mangle]
pub extern "C" fn owner_map_translate(
    uid: uid_t,
    new_uid: *mut uid_t,
    new_name: *mut *const c_char,
) -> c_int {
    let (uid, name) = OWNER_MAP.translate(uid);
    unsafe {
        if let Some(u) = uid {
            *new_uid = u;
        }
        if let Some(n) = name {
            *new_name = n.as_ptr();
        }
    }
    if uid.is_some() || name.is_some() { 0 } else { 1 }
}

#[no_mangle]
pub extern "C" fn group_map_read(file: *const c_char) {
    let path = unsafe { CStr::from_ptr(file) }.to_str().unwrap();
    GROUP_MAP.read(Path::new(path)).unwrap();
}

#[no_mangle]
pub extern "C" fn group_map_translate(
    gid: gid_t,
    new_gid: *mut gid_t,
    new_name: *mut *const c_char,
) -> c_int {
    let (gid, name) = GROUP_MAP.translate(gid);
    unsafe {
        if let Some(g) = gid {
            *new_gid = g;
        }
        if let Some(n) = name {
            *new_name = n.as_ptr();
        }
    }
    if gid.is_some() || name.is_some() { 0 } else { 1 }
}

lazy_static! {
    static ref OWNER_MAP: OwnerMap = OwnerMap::new();
    static ref GROUP_MAP: GroupMap = GroupMap::new();
}