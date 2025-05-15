use std::ffi::{CStr, CString};
use std::ptr;
use std::sync::Mutex;
use libc::{uid_t, gid_t, passwd, group, uintmax_t};

#[derive(Debug, Clone)]
pub enum Id {
    Uid(uid_t),
    Gid(gid_t),
}

#[derive(Debug, Clone)]
pub struct UserId {
    pub id: Id,
    pub name: CString,
    pub next: Option<Box<UserId>>,
}

lazy_static! {
    static ref USER_ALIST: Mutex<Option<Box<UserId>>> = Mutex::new(None);
    static ref NOUSER_ALIST: Mutex<Option<Box<UserId>>> = Mutex::new(None);
    static ref GROUP_ALIST: Mutex<Option<Box<UserId>>> = Mutex::new(None);
    static ref NOGROUP_ALIST: Mutex<Option<Box<UserId>>> = Mutex::new(None);
}

pub fn getuser(uid: uid_t) -> Option<CString> {
    let mut user_alist = USER_ALIST.lock().unwrap();
    
    let mut current = &mut *user_alist;
    while let Some(ref mut node) = current {
        if let Id::Uid(node_uid) = node.id {
            if node_uid == uid {
                return Some(node.name.clone());
            }
        }
        current = &mut node.next;
    }

    unsafe {
        let pwent = libc::getpwuid(uid);
        if pwent.is_null() {
            let mut nbuf = [0; 21];
            let num_str = CStr::from_ptr(libc::umaxtostr(uid as uintmax_t, nbuf.as_mut_ptr()));
            let name = CString::new(num_str.to_bytes()).unwrap();
            
            let new_node = Box::new(UserId {
                id: Id::Uid(uid),
                name,
                next: user_alist.take(),
            });
            *user_alist = Some(new_node);
            user_alist.as_ref().map(|n| n.name.clone())
        } else {
            let name = CStr::from_ptr((*pwent).pw_name).to_owned();
            
            let new_node = Box::new(UserId {
                id: Id::Uid(uid),
                name,
                next: user_alist.take(),
            });
            *user_alist = Some(new_node);
            user_alist.as_ref().map(|n| n.name.clone())
        }
    }
}

pub fn getuidbyname(user: &CStr) -> Option<uid_t> {
    let mut user_alist = USER_ALIST.lock().unwrap();
    let mut nouser_alist = NOUSER_ALIST.lock().unwrap();

    let mut current = &mut *user_alist;
    while let Some(ref mut node) = current {
        if node.name.as_bytes() == user.to_bytes() {
            if let Id::Uid(uid) = node.id {
                return Some(uid);
            }
        }
        current = &mut node.next;
    }

    let mut current = &mut *nouser_alist;
    while let Some(ref mut node) = current {
        if node.name.as_bytes() == user.to_bytes() {
            return None;
        }
        current = &mut node.next;
    }

    unsafe {
        let pwent = libc::getpwnam(user.as_ptr());
        let name = user.to_owned();
        
        if pwent.is_null() {
            let new_node = Box::new(UserId {
                id: Id::Uid(0),
                name,
                next: nouser_alist.take(),
            });
            *nouser_alist = Some(new_node);
            None
        } else {
            let uid = (*pwent).pw_uid;
            let new_node = Box::new(UserId {
                id: Id::Uid(uid),
                name,
                next: user_alist.take(),
            });
            *user_alist = Some(new_node);
            Some(uid)
        }
    }
}

pub fn getgroup(gid: gid_t) -> Option<CString> {
    let mut group_alist = GROUP_ALIST.lock().unwrap();
    
    let mut current = &mut *group_alist;
    while let Some(ref mut node) = current {
        if let Id::Gid(node_gid) = node.id {
            if node_gid == gid {
                return Some(node.name.clone());
            }
        }
        current = &mut node.next;
    }

    unsafe {
        let grent = libc::getgrgid(gid);
        if grent.is_null() {
            let mut nbuf = [0; 21];
            let num_str = CStr::from_ptr(libc::umaxtostr(gid as uintmax_t, nbuf.as_mut_ptr()));
            let name = CString::new(num_str.to_bytes()).unwrap();
            
            let new_node = Box::new(UserId {
                id: Id::Gid(gid),
                name,
                next: group_alist.take(),
            });
            *group_alist = Some(new_node);
            group_alist.as_ref().map(|n| n.name.clone())
        } else {
            let name = CStr::from_ptr((*grent).gr_name).to_owned();
            
            let new_node = Box::new(UserId {
                id: Id::Gid(gid),
                name,
                next: group_alist.take(),
            });
            *group_alist = Some(new_node);
            group_alist.as_ref().map(|n| n.name.clone())
        }
    }
}

pub fn getgidbyname(group: &CStr) -> Option<gid_t> {
    let mut group_alist = GROUP_ALIST.lock().unwrap();
    let mut nogroup_alist = NOGROUP_ALIST.lock().unwrap();

    let mut current = &mut *group_alist;
    while let Some(ref mut node) = current {
        if node.name.as_bytes() == group.to_bytes() {
            if let Id::Gid(gid) = node.id {
                return Some(gid);
            }
        }
        current = &mut node.next;
    }

    let mut current = &mut *nogroup_alist;
    while let Some(ref mut node) = current {
        if node.name.as_bytes() == group.to_bytes() {
            return None;
        }
        current = &mut node.next;
    }

    unsafe {
        let grent = libc::getgrnam(group.as_ptr());
        let name = group.to_owned();
        
        if grent.is_null() {
            let new_node = Box::new(UserId {
                id: Id::Gid(0),
                name,
                next: nogroup_alist.take(),
            });
            *nogroup_alist = Some(new_node);
            None
        } else {
            let gid = (*grent).gr_gid;
            let new_node = Box::new(UserId {
                id: Id::Gid(gid),
                name,
                next: group_alist.take(),
            });
            *group_alist = Some(new_node);
            Some(gid)
        }
    }
}