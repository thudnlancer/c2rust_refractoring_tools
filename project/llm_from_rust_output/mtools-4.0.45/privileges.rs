use std::env;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::io::AsRawFd;
use nix::unistd::{getuid, geteuid, getgid, getegid, setuid, seteuid, setgid, Uid, Gid};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};

#[derive(Debug)]
pub struct Privileges {
    no_privileges: bool,
    rgid: Gid,
    egid: Gid,
    ruid: Uid,
    euid: Uid,
}

impl Privileges {
    pub fn new() -> Self {
        let euid = geteuid();
        let ruid = getuid();
        let egid = getegid();
        let rgid = getgid();
        
        if euid != ruid {
            env::remove_var("SOURCE_DATE_EPOCH");
        }
        
        if euid.is_root() && !ruid.is_root() {
            setuid(Uid::from_raw(0)).expect("Failed to setuid to root");
        }
        
        let privs = Privileges {
            no_privileges: false,
            rgid,
            egid,
            ruid,
            euid,
        };
        
        privs.drop_privs();
        privs
    }
    
    fn setuid(&self, uid: Uid) -> nix::Result<()> {
        if self.euid.is_root() {
            seteuid(uid)
        } else {
            setuid(uid)
        }
    }
    
    pub fn reclaim_privs(&self) {
        if self.no_privileges {
            return;
        }
        setgid(self.egid).expect("Failed to setgid");
        self.setuid(self.euid).expect("Failed to seteuid");
    }
    
    pub fn drop_privs(&self) {
        self.setuid(self.ruid).expect("Failed to setuid");
        setgid(self.rgid).expect("Failed to setgid");
    }
    
    pub fn destroy_privs(&mut self) {
        if self.euid.is_root() {
            setuid(Uid::from_raw(0)).expect("Failed to setuid to root");
            setuid(self.ruid).expect("Failed to setuid");
            seteuid(self.ruid).expect("Failed to seteuid");
        }
        self.drop_privs();
        self.no_privileges = true;
    }
    
    pub fn get_real_uid(&self) -> Uid {
        self.ruid
    }
    
    pub fn close_exec<T: AsRawFd>(fd: &T) -> nix::Result<()> {
        fcntl(fd.as_raw_fd(), FcntlArg::F_SETFD(FdFlag::FD_CLOEXEC))
    }
}