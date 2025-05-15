use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;
use std::sync::Mutex;
use libc::{uid_t, gid_t};
use nix::unistd::{User, Group};

lazy_static::lazy_static! {
    static ref USER_CACHE: Mutex<HashMap<uid_t, String>> = Mutex::new(HashMap::new());
    static ref NOUSER_CACHE: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref GROUP_CACHE: Mutex<HashMap<gid_t, String>> = Mutex::new(HashMap::new());
    static ref NOGROUP_CACHE: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn getuser(uid: uid_t) -> Option<String> {
    let mut cache = USER_CACHE.lock().unwrap();
    if let Some(name) = cache.get(&uid) {
        return Some(name.clone());
    }

    match User::from_uid(uid) {
        Ok(Some(user)) => {
            let name = user.name;
            cache.insert(uid, name.clone());
            Some(name)
        }
        _ => {
            let name = uid.to_string();
            cache.insert(uid, name.clone());
            Some(name)
        }
    }
}

pub fn getuidbyname(user: &str) -> Option<uid_t> {
    {
        let cache = USER_CACHE.lock().unwrap();
        for (uid, name) in cache.iter() {
            if name == user {
                return Some(*uid);
            }
        }
    }

    {
        let cache = NOUSER_CACHE.lock().unwrap();
        if cache.contains(&user.to_string()) {
            return None;
        }
    }

    match User::from_name(user) {
        Ok(Some(user)) => {
            let uid = user.uid;
            let mut cache = USER_CACHE.lock().unwrap();
            cache.insert(uid, user.name);
            Some(uid)
        }
        _ => {
            let mut cache = NOUSER_CACHE.lock().unwrap();
            cache.push(user.to_string());
            None
        }
    }
}

pub fn getgroup(gid: gid_t) -> Option<String> {
    let mut cache = GROUP_CACHE.lock().unwrap();
    if let Some(name) = cache.get(&gid) {
        return Some(name.clone());
    }

    match Group::from_gid(gid) {
        Ok(Some(group)) => {
            let name = group.name;
            cache.insert(gid, name.clone());
            Some(name)
        }
        _ => {
            let name = gid.to_string();
            cache.insert(gid, name.clone());
            Some(name)
        }
    }
}

pub fn getgidbyname(group: &str) -> Option<gid_t> {
    {
        let cache = GROUP_CACHE.lock().unwrap();
        for (gid, name) in cache.iter() {
            if name == group {
                return Some(*gid);
            }
        }
    }

    {
        let cache = NOGROUP_CACHE.lock().unwrap();
        if cache.contains(&group.to_string()) {
            return None;
        }
    }

    match Group::from_name(group) {
        Ok(Some(group)) => {
            let gid = group.gid;
            let mut cache = GROUP_CACHE.lock().unwrap();
            cache.insert(gid, group.name);
            Some(gid)
        }
        _ => {
            let mut cache = NOGROUP_CACHE.lock().unwrap();
            cache.push(group.to_string());
            None
        }
    }
}