use std::env;
use std::ffi::{CString, OsStr, OsString};
use std::fs::{self, File};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process;
use std::sync::Once;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Disposition {
    Real,
    Effective,
    NotMade,
}

struct Sff {
    filename: Option<PathBuf>,
    disposition: Disposition,
}

struct EphemStuff {
    standard: Option<PathBuf>,
    tpnames: Vec<Sff>,
}

static mut BE: Option<Box<EphemStuff>> = None;
static INIT: Once = Once::new();

fn init_ephemstuff() {
    INIT.call_once(|| {
        unsafe {
            BE = Some(Box::new(EphemStuff {
                standard: None,
                tpnames: vec![Sff {
                    filename: None,
                    disposition: Disposition::NotMade,
                }; TEMPNAMES],
            }));
        }
    });
}

const TEMPNAMES: usize = 5;
const SLASH: char = '/';
const SFFI_NEWDIR: usize = 0;

fn jam_sff(sff: &mut Sff, prefix: Option<&Path>) -> io::Result<()> {
    let mut template = if let Some(p) = prefix {
        p.to_path_buf()
    } else {
        let dir = env::var_os("TMPDIR")
            .or_else(|| env::var_os("TMP"))
            .or_else(|| env::var_os("TEMP"))
            .unwrap_or_else(|| OsString::from("/tmp"));
        
        let mut path = PathBuf::from(dir);
        path.push(env::current_exe()?.file_name().unwrap());
        path
    };

    let mut file_name = template.file_name().unwrap().to_os_string();
    file_name.push("XXXXXX");
    template.set_file_name(file_name);

    let path = tempfile::Builder::new()
        .prefix(template.as_os_str().to_str().unwrap())
        .tempfile()?
        .into_temp_path();
    
    sff.filename = Some(path.to_path_buf());
    sff.disposition = Disposition::Real;
    Ok(())
}

fn maketemp(n: usize) -> io::Result<&'static Path> {
    unsafe {
        if BE.is_none() {
            init_ephemstuff();
        }
        
        if BE.as_ref().unwrap().tpnames[n].filename.is_none() {
            jam_sff(&mut BE.as_mut().unwrap().tpnames[n], None)?;
        }
        
        Ok(BE.as_ref().unwrap().tpnames[n].filename.as_ref().unwrap())
    }
}

fn makedirtemp(is_workfile: bool) -> io::Result<&'static Path> {
    unsafe {
        if BE.is_none() {
            init_ephemstuff();
        }
        
        let slot = SFFI_NEWDIR + if is_workfile { 1 } else { 0 };
        let prefix = if is_workfile {
            Path::new("filename") // TODO: Replace with actual workfile path
        } else {
            Path::new("filename") // TODO: Replace with actual RCSfile path
        };
        
        jam_sff(&mut BE.as_mut().unwrap().tpnames[slot], Some(prefix))?;
        Ok(BE.as_ref().unwrap().tpnames[slot].filename.as_ref().unwrap())
    }
}

fn keepdirtemp(name: &Path) {
    unsafe {
        if let Some(be) = BE.as_mut() {
            for sff in be.tpnames.iter_mut() {
                if let Some(ref filename) = sff.filename {
                    if filename == name {
                        sff.disposition = Disposition::NotMade;
                        return;
                    }
                }
            }
        }
        eprintln!("keepdirtemp: file not found");
        process::exit(1);
    }
}

fn reap<F>(items: &mut [Sff], mut unlink_fn: F) 
where
    F: FnMut(&Path) -> io::Result<()>,
{
    for item in items.iter_mut() {
        if item.disposition != Disposition::NotMade {
            if let Some(ref path) = item.filename {
                if let Err(e) = unlink_fn(path) {
                    eprintln!("Failed to unlink {}: {}", path.display(), e);
                }
            }
            item.filename = None;
            item.disposition = Disposition::NotMade;
        }
    }
}

fn tempunlink() {
    unsafe {
        if let Some(be) = BE.as_mut() {
            reap(&mut be.tpnames, |path| fs::remove_file(path));
        }
    }
}

fn dirtempunlink() {
    unsafe {
        if let Some(be) = BE.as_mut() {
            reap(&mut be.tpnames, |path| fs::remove_file(path));
        }
    }
}