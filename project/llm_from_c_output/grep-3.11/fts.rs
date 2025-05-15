use std::{
    ffi::{CStr, CString, OsStr, OsString},
    fs::{self, DirBuilder, File, Metadata, OpenOptions, ReadDir},
    io::{self, Error, ErrorKind},
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    ptr,
    sync::Arc,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtsOptions {
    Physical,
    Logical,
    NoChdir,
    CwdFd,
    NameOnly,
    DeferStat,
    Xdev,
    Seedot,
    TightCycleCheck,
    Comfollow,
    Nostat,
    LeafOptimization,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtsInstr {
    NoInstr,
    Again,
    Follow,
    Skip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtsInfo {
    Init,
    D,
    Dc,
    Dnr,
    Dp,
    Dot,
    Err,
    F,
    Ns,
    Nsok,
    Sl,
    Slnone,
    Default,
}

#[derive(Debug)]
pub struct FtsEnt {
    pub fts_accpath: PathBuf,
    pub fts_path: PathBuf,
    pub fts_name: OsString,
    pub fts_namelen: usize,
    pub fts_level: i32,
    pub fts_parent: Option<Arc<FtsEnt>>,
    pub fts_link: Option<Arc<FtsEnt>>,
    pub fts_fts: Arc<Fts>,
    pub fts_errno: i32,
    pub fts_dirp: Option<ReadDir>,
    pub fts_flags: u32,
    pub fts_instr: FtsInstr,
    pub fts_number: u64,
    pub fts_pointer: *mut libc::c_void,
    pub fts_statp: Metadata,
    pub fts_info: FtsInfo,
}

#[derive(Debug)]
pub struct Fts {
    pub fts_path: PathBuf,
    pub fts_pathlen: usize,
    pub fts_nitems: usize,
    pub fts_cur: Option<Arc<FtsEnt>>,
    pub fts_child: Option<Arc<FtsEnt>>,
    pub fts_array: Vec<Arc<FtsEnt>>,
    pub fts_options: u32,
    pub fts_dev: u64,
    pub fts_rfd: i32,
    pub fts_cwd_fd: i32,
    pub fts_compar: Option<Box<dyn Fn(&FtsEnt, &FtsEnt) -> i32>>,
}

impl Fts {
    pub fn open(
        argv: &[&str],
        options: u32,
        compar: Option<Box<dyn Fn(&FtsEnt, &FtsEnt) -> i32>>,
    ) -> io::Result<Arc<Self>> {
        let mut sp = Arc::new(Fts {
            fts_path: PathBuf::new(),
            fts_pathlen: 0,
            fts_nitems: 0,
            fts_cur: None,
            fts_child: None,
            fts_array: Vec::new(),
            fts_options: options,
            fts_dev: 0,
            fts_rfd: -1,
            fts_cwd_fd: -1,
            fts_compar: compar,
        });

        if options & !0xFFFF != 0 {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid options"));
        }

        if (options & FtsOptions::NoChdir as u32 != 0) && (options & FtsOptions::CwdFd as u32 != 0) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "FTS_NOCHDIR and FTS_CWDFD are mutually exclusive",
            ));
        }

        if (options & FtsOptions::Physical as u32 == 0)
            && (options & FtsOptions::Logical as u32 == 0)
        {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "must specify either FTS_PHYSICAL or FTS_LOGICAL",
            ));
        }

        if options & FtsOptions::Logical as u32 != 0 {
            sp.fts_options |= FtsOptions::NoChdir as u32;
            sp.fts_options &= !(FtsOptions::CwdFd as u32);
        }

        sp.fts_cwd_fd = -1;

        let maxarglen = argv.iter().map(|s| s.len()).max().unwrap_or(0);
        let maxpathlen = std::cmp::max(maxarglen, 1024);
        if !sp.fts_palloc(maxpathlen) {
            return Err(Error::new(ErrorKind::Other, "memory allocation failed"));
        }

        let mut parent = None;
        if !argv.is_empty() {
            parent = Some(Arc::new(FtsEnt {
                fts_accpath: PathBuf::new(),
                fts_path: PathBuf::new(),
                fts_name: OsString::new(),
                fts_namelen: 0,
                fts_level: -1,
                fts_parent: None,
                fts_link: None,
                fts_fts: Arc::clone(&sp),
                fts_errno: 0,
                fts_dirp: None,
                fts_flags: 0,
                fts_instr: FtsInstr::NoInstr,
                fts_number: 0,
                fts_pointer: ptr::null_mut(),
                fts_statp: Metadata::default(),
                fts_info: FtsInfo::Init,
            }));
        }

        let defer_stat = sp.fts_compar.is_none() || options & FtsOptions::DeferStat as u32 != 0;

        let mut root = None;
        let mut nitems = 0;
        let mut tmp = None;
        for arg in argv {
            let len = arg.len();
            let mut p = arg.to_string();
            if options & FtsOptions::Verbatim as u32 == 0 {
                while len > 1 && p.ends_with('/') && p[p.len() - 2] == '/' {
                    p.pop();
                }
            }

            let ent = Arc::new(FtsEnt {
                fts_accpath: PathBuf::from(&p),
                fts_path: PathBuf::from(&p),
                fts_name: OsString::from(&p),
                fts_namelen: p.len(),
                fts_level: 0,
                fts_parent: parent.clone(),
                fts_link: None,
                fts_fts: Arc::clone(&sp),
                fts_errno: 0,
                fts_dirp: None,
                fts_flags: 0,
                fts_instr: FtsInstr::NoInstr,
                fts_number: 0,
                fts_pointer: ptr::null_mut(),
                fts_statp: Metadata::default(),
                fts_info: if defer_stat && root.is_some() {
                    FtsInfo::Nsok
                } else {
                    FtsInfo::Init
                },
            });

            if sp.fts_compar.is_some() {
                ent.fts_link = root.clone();
                root = Some(ent);
            } else {
                if root.is_none() {
                    root = Some(ent.clone());
                    tmp = Some(ent);
                } else {
                    if let Some(t) = tmp {
                        t.fts_link = Some(ent.clone());
                        tmp = Some(ent);
                    }
                }
            }
            nitems += 1;
        }

        if sp.fts_compar.is_some() && nitems > 1 {
            root = Some(sp.fts_sort(root.unwrap(), nitems));
        }

        let cur = Arc::new(FtsEnt {
            fts_accpath: PathBuf::new(),
            fts_path: PathBuf::new(),
            fts_name: OsString::new(),
            fts_namelen: 0,
            fts_level: 1,
            fts_parent: None,
            fts_link: root,
            fts_fts: Arc::clone(&sp),
            fts_errno: 0,
            fts_dirp: None,
            fts_flags: 0,
            fts_instr: FtsInstr::NoInstr,
            fts_number: 0,
            fts_pointer: ptr::null_mut(),
            fts_statp: Metadata::default(),
            fts_info: FtsInfo::Init,
        });

        sp.fts_cur = Some(cur);

        if !sp.setup_dir() {
            return Err(Error::new(ErrorKind::Other, "failed to setup directory"));
        }

        if options & FtsOptions::NoChdir as u32 == 0 && options & FtsOptions::CwdFd as u32 == 0 {
            sp.fts_rfd = match File::open(".") {
                Ok(f) => f.into_raw_fd(),
                Err(_) => {
                    sp.fts_options |= FtsOptions::NoChdir as u32;
                    -1
                }
            };
        }

        Ok(sp)
    }

    fn fts_palloc(&mut self, more: usize) -> bool {
        let new_len = self.fts_pathlen + more + 256;
        if new_len < self.fts_pathlen {
            return false;
        }
        self.fts_pathlen = new_len;
        self.fts_path.reserve(new_len);
        true
    }

    fn setup_dir(&self) -> bool {
        true
    }

    fn fts_sort(&self, head: Arc<FtsEnt>, nitems: usize) -> Arc<FtsEnt> {
        head
    }
}

impl Drop for Fts {
    fn drop(&mut self) {
        if self.fts_rfd != -1 {
            unsafe {
                libc::close(self.fts_rfd);
            }
        }
    }
}