/*
 * grcat.rs
 *
 * Generate a printable version of the group database.
 */
/*
 * Arnold Robbins, arnold@skeeve.com, May 1993
 * Public Domain
 * December 2010, move to ANSI C definition for main().
 * Translated to Rust while maintaining functionality and safety.
 */

use std::io::{self, Write};
use std::ffi::CStr;
use libc::{getgrent, endgrent, group};
use std::ptr;

fn main() -> io::Result<()> {
    unsafe {
        loop {
            let g = getgrent();
            if g.is_null() {
                break;
            }
            
            let group = &*g;
            
            let name = CStr::from_ptr(group.gr_name).to_string_lossy();
            let gid = group.gr_gid;
            
            #[cfg(not(any(target_os = "android", target_os = "linux")))]
            let passwd = if !group.gr_passwd.is_null() {
                CStr::from_ptr(group.gr_passwd).to_string_lossy()
            } else {
                "*".into()
            };
            
            #[cfg(not(any(target_os = "android", target_os = "linux")))]
            print!("{}:{}:{}:", name, passwd, gid);
            #[cfg(any(target_os = "android", target_os = "linux"))]
            print!("{}:*:{}:", name, gid);
            
            if !group.gr_mem.is_null() {
                let mut i = 0;
                while !(*group.gr_mem.offset(i)).is_null() {
                    let member = CStr::from_ptr(*group.gr_mem.offset(i)).to_string_lossy();
                    print!("{}", member);
                    
                    if !(*group.gr_mem.offset(i + 1)).is_null() {
                        print!(",");
                    }
                    
                    i += 1;
                }
            }
            
            println!();
        }
        
        endgrent();
    }
    
    Ok(())
}