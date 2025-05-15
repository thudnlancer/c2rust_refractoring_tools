use std::collections::HashMap;
use std::ffi::CString;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;
use nix::unistd::{Uid, Gid};
use nix::libc::{uid_t, gid_t};
use libc::{getpwnam, getgrnam, passwd, group};
use std::ptr;
use std::ffi::CStr;

#[derive(Debug)]
struct MapEntry {
    orig_id: u64,
    new_id: u64,
    new_name: Option<String>,
}

struct OwnerGroupMaps {
    owner_map: HashMap<u64, MapEntry>,
    group_map: HashMap<u64, MapEntry>,
    owner_option: Option<uid_t>,
    owner_name_option: Option<String>,
    group_option: Option<gid_t>,
    group_name_option: Option<String>,
}

impl OwnerGroupMaps {
    fn new() -> Self {
        Self {
            owner_map: HashMap::new(),
            group_map: HashMap::new(),
            owner_option: None,
            owner_name_option: None,
            group_option: None,
            group_name_option: None,
        }
    }

    fn name_to_uid(name: &str) -> Option<u64> {
        let cname = CString::new(name).unwrap();
        unsafe {
            let passwd = getpwnam(cname.as_ptr());
            if passwd.is_null() {
                None
            } else {
                Some((*passwd).pw_uid as u64)
            }
        }
    }

    fn name_to_gid(name: &str) -> Option<u64> {
        let cname = CString::new(name).unwrap();
        unsafe {
            let group = getgrnam(cname.as_ptr());
            if group.is_null() {
                None
            } else {
                Some((*group).gr_gid as u64)
            }
        }
    }

    fn parse_id(s: &str, maxval: u64) -> Result<u64, ParseIntError> {
        let val = u64::from_str(s)?;
        if val > maxval {
            Err(ParseIntError::from(std::num::IntErrorKind::PosOverflow))
        } else {
            Ok(val)
        }
    }

    fn read_map_file(
        &mut self,
        file: &Path,
        name_to_id: fn(&str) -> Option<u64>,
        what: &str,
        maxval: u64,
        is_owner: bool,
    ) -> io::Result<()> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let mut errors = Vec::new();

        for (line_num, line) in reader.lines().enumerate() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 2 {
                errors.push(format!("{}:{}: malformed line", file.display(), line_num + 1));
                continue;
            }

            let (orig_id, new_id, new_name) = match Self::process_line(
                parts[0],
                parts[1],
                name_to_id,
                what,
                maxval,
                file.display().to_string(),
                line_num + 1,
            ) {
                Ok(tuple) => tuple,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };

            let entry = MapEntry {
                orig_id,
                new_id,
                new_name: new_name.map(|s| s.to_string()),
            };

            if is_owner {
                self.owner_map.insert(orig_id, entry);
            } else {
                self.group_map.insert(orig_id, entry);
            }
        }

        if !errors.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                errors.join("\n"),
            ));
        }

        Ok(())
    }

    fn process_line(
        orig: &str,
        new: &str,
        name_to_id: fn(&str) -> Option<u64>,
        what: &str,
        maxval: u64,
        file: String,
        line_num: usize,
    ) -> Result<(u64, u64, Option<&str>), String> {
        let orig_id = if orig.starts_with('+') {
            Self::parse_id(&orig[1..], maxval).map_err(|e| {
                format!(
                    "{}:{}: invalid {}: {} ({})",
                    file,
                    line_num,
                    what,
                    orig,
                    e
                )
            })?
        } else {
            name_to_id(orig).ok_or_else(|| {
                format!(
                    "{}:{}: can't obtain {} of {}",
                    file,
                    line_num,
                    what,
                    orig
                )
            })?
        };

        let (new_id, new_name) = if let Some(colon_pos) = new.find(':') {
            let (name_part, id_part) = new.split_at(colon_pos);
            let id_part = &id_part[1..];
            let id = Self::parse_id(id_part, maxval).map_err(|e| {
                format!(
                    "{}:{}: invalid {}: {} ({})",
                    file,
                    line_num,
                    what,
                    id_part,
                    e
                )
            })?;
            let name = if name_part.is_empty() {
                None
            } else {
                Some(name_part)
            };
            (id, name)
        } else if new.starts_with('+') {
            let id = Self::parse_id(&new[1..], maxval).map_err(|e| {
                format!(
                    "{}:{}: invalid {}: {} ({})",
                    file,
                    line_num,
                    what,
                    new,
                    e
                )
            })?;
            (id, None)
        } else {
            let id = name_to_id(new).ok_or_else(|| {
                format!(
                    "{}:{}: can't obtain {} of {}",
                    file,
                    line_num,
                    what,
                    new
                )
            })?;
            (id, Some(new))
        };

        Ok((orig_id, new_id, new_name))
    }

    fn owner_map_translate(&self, uid: uid_t) -> (Option<uid_t>, Option<&str>) {
        if let Some(entry) = self.owner_map.get(&(uid as u64)) {
            return (Some(entry.new_id as uid_t), entry.new_name.as_deref());
        }

        (self.owner_option, self.owner_name_option.as_deref())
    }

    fn group_map_translate(&self, gid: gid_t) -> (Option<gid_t>, Option<&str>) {
        if let Some(entry) = self.group_map.get(&(gid as u64)) {
            return (Some(entry.new_id as gid_t), entry.new_name.as_deref());
        }

        (self.group_option, self.group_name_option.as_deref())
    }
}

fn main() {
    // Example usage
    let mut maps = OwnerGroupMaps::new();
    maps.owner_option = Some(1000);
    maps.owner_name_option = Some("user".to_string());
    maps.group_option = Some(1000);
    maps.group_name_option = Some("group".to_string());

    if let Err(e) = maps.read_map_file(
        Path::new("/etc/owner.map"),
        OwnerGroupMaps::name_to_uid,
        "UID",
        u32::MAX as u64,
        true,
    ) {
        eprintln!("Error reading owner map: {}", e);
    }

    if let Err(e) = maps.read_map_file(
        Path::new("/etc/group.map"),
        OwnerGroupMaps::name_to_gid,
        "GID",
        u32::MAX as u64,
        false,
    ) {
        eprintln!("Error reading group map: {}", e);
    }

    let (uid, uname) = maps.owner_map_translate(1001);
    println!("Translated UID: {:?}, Name: {:?}", uid, uname);

    let (gid, gname) = maps.group_map_translate(1001);
    println!("Translated GID: {:?}, Name: {:?}", gid, gname);
}