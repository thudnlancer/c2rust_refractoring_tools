/*  Copyright 1997,1999,2001,2002,2007,2009 Alain Knaff.
 *  This file is part of mtools.
 *
 *  Mtools is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Mtools is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::env;
use std::os::unix::io::AsRawFd;
use std::process;
use nix::unistd::{getuid, geteuid, getgid, getegid, setuid, seteuid, setgid};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};

static mut NO_PRIVILEGES: bool = false;

#[cfg(target_os = "windows")]
mod priv_windows {
    pub fn reclaim_privs() {}
    pub fn drop_privs() {}
    pub fn destroy_privs() {}
    pub fn get_real_uid() -> u32 { 0 }
    pub fn init_privs() {}
    pub fn close_exec(fd: i32) {}
}

#[cfg(not(target_os = "windows"))]
mod priv_unix {
    use super::*;
    use nix::unistd::Uid;
    use nix::unistd::Gid;

    static mut RGID: Option<Gid> = None;
    static mut EGID: Option<Gid> = None;
    static mut RUID: Option<Uid> = None;
    static mut EUID: Option<Uid> = None;

    #[cfg(feature = "priv_debug")]
    fn print_privs(message: &str) {
        eprintln!("{} egid={:?} rgid={:?}", message, getegid(), getgid());
        eprintln!("{} euid={:?} ruid={:?}", message, geteuid(), getuid());
    }

    #[cfg(not(feature = "priv_debug"))]
    fn print_privs(_message: &str) {}

    fn set_uid(uid: Uid) -> Result<(), nix::Error> {
        unsafe {
            if EUID == Some(Uid::from_raw(0)) {
                seteuid(uid)
            } else {
                setuid(uid)
            }
        }
    }

    pub fn reclaim_privs() -> Result<(), nix::Error> {
        unsafe {
            if NO_PRIVILEGES {
                return Ok(());
            }
            setgid(EGID.unwrap())?;
            set_uid(EUID.unwrap())?;
            print_privs("after reclaim privs, both uids should be 0 ");
            Ok(())
        }
    }

    pub fn drop_privs() -> Result<(), nix::Error> {
        unsafe {
            set_uid(RUID.unwrap())?;
            setgid(RGID.unwrap())?;
            print_privs("after drop_privs, real should be 0, effective should not ");
            Ok(())
        }
    }

    pub fn destroy_privs() -> Result<(), nix::Error> {
        unsafe {
            if EUID == Some(Uid::from_raw(0)) {
                setuid(Uid::from_raw(0))?;
                setuid(RUID.unwrap())?;
                seteuid(RUID.unwrap())?;
            }
            
            drop_privs()?;
            print_privs("destroy_privs, no uid should be zero");
            Ok(())
        }
    }

    pub fn get_real_uid() -> u32 {
        unsafe { RUID.unwrap().as_raw() }
    }

    pub fn init_privs() -> Result<(), nix::Error> {
        unsafe {
            EUID = Some(geteuid());
            RUID = Some(getuid());
            EGID = Some(getegid());
            RGID = Some(getgid());

            if EUID != RUID {
                env::remove_var("SOURCE_DATE_EPOCH");
            }

            if EUID == Some(Uid::from_raw(0)) && RUID != Some(Uid::from_raw(0)) {
                setuid(Uid::from_raw(0))?;
            }

            drop_privs()?;
            print_privs("after init, real should be 0, effective should not ");
            Ok(())
        }
    }

    pub fn close_exec(fd: i32) -> Result<(), nix::Error> {
        fcntl(fd, FcntlArg::F_SETFD(FdFlag::FD_CLOEXEC))?;
        Ok(())
    }
}

#[cfg(target_os = "windows")]
use priv_windows::*;

#[cfg(not(target_os = "windows"))]
use priv_unix::*;