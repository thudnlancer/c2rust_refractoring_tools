use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;
use std::sync::Mutex;
use libc::{uid_t, gid_t};
use users::{get_user_by_uid, get_user_by_name, get_group_by_gid, get_group_by_name};

lazy_static::lazy_static! {
    static ref USER_CACHE: Mutex<HashMap<uid_t, Option<String>>> = Mutex::new(HashMap::new());
    static ref NO_USER_CACHE: Mutex<HashMap<String, ()>> = Mutex::new(HashMap::new());
    static ref GROUP_CACHE: Mutex<HashMap<gid_t, Option<String>>> = Mutex::new(HashMap::new());
    static ref NO_GROUP_CACHE: Mutex<HashMap<String, ()>> = Mutex::new(HashMap::new());
}

pub fn getuser(uid: uid_t) -> Option<String> {
    let mut cache = USER_CACHE.lock().unwrap();
    
    if let Some(name) = cache.get(&uid) {
        return name.clone();
    }

    let name = get_user_by_uid(uid)
        .map(|user| user.name().to_string_lossy().into_owned());

    cache.insert(uid, name.clone());
    name
}

pub fn getuidbyname(user: &str) -> Option<uid_t> {
    {
        let cache = USER_CACHE.lock().unwrap();
        for (uid, name) in cache.iter() {
            if let Some(ref n) = name {
                if n == user {
                    return Some(*uid);
                }
            }
        }
    }

    {
        let no_cache = NO_USER_CACHE.lock().unwrap();
        if no_cache.contains_key(user) {
            return None;
        }
    }

    if let Some(user) = get_user_by_name(user) {
        let uid = user.uid();
        let mut cache = USER_CACHE.lock().unwrap();
        cache.insert(uid, Some(user.name().to_string_lossy().into_owned()));
        Some(uid)
    } else {
        let mut no_cache = NO_USER_CACHE.lock().unwrap();
        no_cache.insert(user.to_string(), ());
        None
    }
}

pub fn getgroup(gid: gid_t) -> Option<String> {
    let mut cache = GROUP_CACHE.lock().unwrap();
    
    if let Some(name) = cache.get(&gid) {
        return name.clone();
    }

    let name = get_group_by_gid(gid)
        .map(|group| group.name().to_string_lossy().into_owned());

    cache.insert(gid, name.clone());
    name
}

pub fn getgidbyname(group: &str) -> Option<gid_t> {
    {
        let cache = GROUP_CACHE.lock().unwrap();
        for (gid, name) in cache.iter() {
            if let Some(ref n) = name {
                if n == group {
                    return Some(*gid);
                }
            }
        }
    }

    {
        let no_cache = NO_GROUP_CACHE.lock().unwrap();
        if no_cache.contains_key(group) {
            return None;
        }
    }

    if let Some(group) = get_group_by_name(group) {
        let gid = group.gid();
        let mut cache = GROUP_CACHE.lock().unwrap();
        cache.insert(gid, Some(group.name().to_string_lossy().into_owned()));
        Some(gid)
    } else {
        let mut no_cache = NO_GROUP_CACHE.lock().unwrap();
        no_cache.insert(group.to_string(), ());
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use users::get_current_uid;

    #[test]
    fn test_getuser() {
        let uid = get_current_uid();
        assert!(getuser(uid).is_some());
    }

    #[test]
    fn test_getuidbyname() {
        let user = getuser(get_current_uid()).unwrap();
        assert!(getuidbyname(&user).is_some());
        assert!(getuidbyname("nonexistentuser").is_none());
    }

    #[test]
    fn test_getgroup() {
        let gid = users::get_current_gid();
        assert!(getgroup(gid).is_some());
    }

    #[test]
    fn test_getgidbyname() {
        let group = getgroup(users::get_current_gid()).unwrap();
        assert!(getgidbyname(&group).is_some());
        assert!(getgidbyname("nonexistentgroup").is_none());
    }
}