use std::env;
use std::ffi::{CString, OsStr};
use std::fs::{File, remove_file, remove_dir};
use std::io;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use tempfile::{Builder, NamedTempFile};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Maker {
    NotMade,
    Real,
    Effective,
}

#[derive(Debug)]
struct Sff {
    filename: Option<PathBuf>,
    disposition: Maker,
}

impl Sff {
    fn new() -> Self {
        Sff {
            filename: None,
            disposition: Maker::NotMade,
        }
    }
}

#[derive(Debug)]
struct EphemStuff {
    standard: Option<PathBuf>,
    tpnames: Vec<Sff>,
}

impl EphemStuff {
    fn new() -> Self {
        EphemStuff {
            standard: None,
            tpnames: vec![Sff::new(); 5],
        }
    }
}

#[derive(Debug)]
struct Behavior {
    ephemstuff: EphemStuff,
    sff: Vec<Sff>,
}

impl Behavior {
    fn new() -> Self {
        Behavior {
            ephemstuff: EphemStuff::new(),
            sff: vec![Sff::new(); 2],
        }
    }
}

struct Top {
    behavior: Behavior,
}

impl Top {
    fn new() -> Self {
        Top {
            behavior: Behavior::new(),
        }
    }
}

fn init_ephemstuff(top: &mut Top) {
    top.behavior.ephemstuff = EphemStuff::new();
}

fn jam_sff(sff: &mut Sff, prefix: Option<&Path>) -> io::Result<()> {
    let prefix = match prefix {
        Some(p) => p.to_owned(),
        None => {
            let dir = env::var_os("TMPDIR")
                .or_else(|| env::var_os("TMP"))
                .or_else(|| env::var_os("TEMP"))
                .unwrap_or_else(|| OsStr::new("/tmp").to_os_string());
            
            let mut path = PathBuf::from(dir);
            if !path.ends_with("/") {
                path.push("/");
            }
            path.push("XXXXXX");
            path
        }
    };

    let mut builder = Builder::new();
    builder.prefix(&prefix);
    let tempfile = builder.tempfile()?;
    let path = tempfile.path().to_owned();
    drop(tempfile);

    sff.filename = Some(path);
    sff.disposition = Maker::Real;
    Ok(())
}

fn maketemp(top: &mut Top, n: usize) -> io::Result<&Path> {
    if top.behavior.ephemstuff.tpnames[n].filename.is_none() {
        jam_sff(&mut top.behavior.ephemstuff.tpnames[n], None)?;
    }
    Ok(top.behavior.ephemstuff.tpnames[n].filename.as_ref().unwrap())
}

fn makedirtemp(top: &mut Top, isworkfile: bool) -> io::Result<&Path> {
    let slot = if isworkfile { 1 } else { 0 };
    let prefix = if isworkfile {
        // Assuming manifestation.filename exists in original
        None
    } else {
        // Assuming repository.filename exists in original
        None
    };
    jam_sff(&mut top.behavior.sff[slot], prefix)?;
    Ok(top.behavior.sff[slot].filename.as_ref().unwrap())
}

fn keepdirtemp(top: &mut Top, name: &Path) {
    for sff in &mut top.behavior.sff {
        if let Some(ref filename) = sff.filename {
            if filename == name {
                sff.disposition = Maker::NotMade;
                return;
            }
        }
    }
    panic!("keepdirtemp: temp file not found");
}

fn reap(sffs: &mut [Sff], is_dir: bool) -> io::Result<()> {
    for sff in sffs {
        if sff.disposition != Maker::NotMade {
            if let Some(ref path) = sff.filename {
                if sff.disposition == Maker::Effective {
                    // seteid() equivalent would go here
                }

                if is_dir {
                    remove_dir(path)?;
                } else {
                    remove_file(path)?;
                }

                if sff.disposition == Maker::Effective {
                    // setrid() equivalent would go here
                }

                sff.filename = None;
                sff.disposition = Maker::NotMade;
            }
        }
    }
    Ok(())
}

fn tempunlink(top: &mut Top) -> io::Result<()> {
    reap(&mut top.behavior.ephemstuff.tpnames, false)
}

fn dirtempunlink(top: &mut Top) -> io::Result<()> {
    reap(&mut top.behavior.sff, true)
}