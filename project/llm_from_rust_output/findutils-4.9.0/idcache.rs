use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::Mutex;
use libc::{uid_t, gid_t, passwd, group};

#[derive(Debug, Clone)]
struct UserId {
    id: Id,
    name: String,
}

#[derive(Debug, Clone)]
enum Id {
    Uid(uid_t),
    Gid(gid_t),
}

lazy_static::lazy_static! {
    static ref USER_ALIST: Mutex<Vec<UserId>> = Mutex::new(Vec::new());
    static ref NOUSER_ALIST: Mutex<Vec<UserId>> = Mutex::new(Vec::new());
    static ref GROUP_ALIST: Mutex<Vec<UserId>> = Mutex::new(Vec::new());
    static ref NOGROUP_ALIST: Mutex<Vec<UserId>> = Mutex::new(Vec::new());
}

fn getuser(uid: uid_t) -> Option<String> {
    let user_alist = USER_ALIST.lock().unwrap();
    
    if let Some(user) = user_alist.iter().find(|u| matches!(u.id, Id::Uid(id) if id == uid)) {
        return Some(user.name.clone());
    }

    drop(user_alist);
    
    unsafe {
        let pwent = libc::getpwuid(uid);
        if pwent.is_null() {
            return None;
        }
        
        let name = CStr::from_ptr((*pwent).pw_name).to_string_lossy().into_owned();
        
        let mut user_alist = USER_ALIST.lock().unwrap();
        let user = UserId {
            id: Id::Uid(uid),
            name: name.clone(),
        };
        user_alist.push(user);
        
        Some(name)
    }
}

fn getuidbyname(user: &str) -> Option<uid_t> {
    let c_user = CString::new(user).unwrap();
    
    {
        let user_alist = USER_ALIST.lock().unwrap();
        if let Some(u) = user_alist.iter().find(|u| u.name == user) {
            if let Id::Uid(uid) = u.id {
                return Some(uid);
            }
        }
    }
    
    {
        let nouser_alist = NOUSER_ALIST.lock().unwrap();
        if nouser_alist.iter().any(|u| u.name == user) {
            return None;
        }
    }
    
    unsafe {
        let pwent = libc::getpwnam(c_user.as_ptr());
        if !pwent.is_null() {
            let uid = (*pwent).pw_uid;
            let mut user_alist = USER_ALIST.lock().unwrap();
            user_alist.push(UserId {
                id: Id::Uid(uid),
                name: user.to_string(),
            });
            Some(uid)
        } else {
            let mut nouser_alist = NOUSER_ALIST.lock().unwrap();
            nouser_alist.push(UserId {
                id: Id::Uid(0),
                name: user.to_string(),
            });
            None
        }
    }
}

fn getgroup(gid: gid_t) -> Option<String> {
    let group_alist = GROUP_ALIST.lock().unwrap();
    
    if let Some(group) = group_alist.iter().find(|g| matches!(g.id, Id::Gid(id) if id == gid)) {
        return Some(group.name.clone());
    }

    drop(group_alist);
    
    unsafe {
        let grent = libc::getgrgid(gid);
        if grent.is_null() {
            return None;
        }
        
        let name = CStr::from_ptr((*grent).gr_name).to_string_lossy().into_owned();
        
        let mut group_alist = GROUP_ALIST.lock().unwrap();
        let group = UserId {
            id: Id::Gid(gid),
            name: name.clone(),
        };
        group_alist.push(group);
        
        Some(name)
    }
}

fn getgidbyname(group: &str) -> Option<gid_t> {
    let c_group = CString::new(group).unwrap();
    
    {
        let group_alist = GROUP_ALIST.lock().unwrap();
        if let Some(g) in group_alist.iter().find(|g| g.name == group) {
            if let Id::Gid(gid) = g.id {
                return Some(gid);
            }
        }
    }
    
    {
        let nogroup_alist = NOGROUP_ALIST.lock().unwrap();
        if nogroup_alist.iter().any(|g| g.name == group) {
            return None;
        }
    }
    
    unsafe {
        let grent = libc::getgrnam(c_group.as_ptr());
        if !grent.is_null() {
            let gid = (*grent).gr_gid;
            let mut group_alist = GROUP_ALIST.lock().unwrap();
            group_alist.push(UserId {
                id: Id::Gid(gid),
                name: group.to_string(),
            });
            Some(gid)
        } else {
            let mut nogroup_alist = NOGROUP_ALIST.lock().unwrap();
            nogroup_alist.push(UserId {
                id: Id::Gid(0),
                name: group.to_string(),
            });
            None
        }
    }
}