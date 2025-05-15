use std::ffi::CString;
use std::io::{self, Write};
use std::process;

struct Group {
    gr_name: String,
    gr_passwd: String,
    gr_gid: u32,
    gr_mem: Vec<String>,
}

fn get_groups() -> Vec<Group> {
    let mut groups = Vec::new();
    
    unsafe {
        loop {
            let g = libc::getgrent();
            if g.is_null() {
                break;
            }
            
            let gr_name = unsafe { CString::from_raw((*g).gr_name) }.into_string().unwrap();
            let gr_passwd = unsafe { CString::from_raw((*g).gr_passwd) }.into_string().unwrap();
            let gr_gid = (*g).gr_gid;
            
            let mut members = Vec::new();
            let mut i = 0;
            while !(*((*g).gr_mem).offset(i as isize)).is_null() {
                let member = unsafe { CString::from_raw(*((*g).gr_mem).offset(i as isize)) }
                    .into_string()
                    .unwrap();
                members.push(member);
                i += 1;
            }
            
            groups.push(Group {
                gr_name,
                gr_passwd,
                gr_gid,
                gr_mem: members,
            });
        }
        libc::endgrent();
    }
    
    groups
}

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    for group in get_groups() {
        write!(
            handle,
            "{}:{}:{}:",
            group.gr_name,
            group.gr_passwd,
            group.gr_gid
        )?;
        
        for (i, member) in group.gr_mem.iter().enumerate() {
            write!(handle, "{}", member)?;
            if i < group.gr_mem.len() - 1 {
                write!(handle, ",")?;
            }
        }
        
        writeln!(handle)?;
    }
    
    Ok(())
}