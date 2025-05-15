use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs::{self, DirEntry, File, Metadata};
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static COMMAND_COUNT: AtomicU64 = AtomicU64::new(0);
static MAX_OPEN_DIRECTORIES: usize = 10;
static mut OPEN_DIRECTORIES: usize = 0;

#[derive(Debug, Clone)]
struct DirectoryContents {
    dev: u64,
    ino: u64,
    dirfiles: HashMap<String, DirFile>,
    counter: u64,
    dirstream: Option<fs::ReadDir>,
}

#[derive(Debug, Clone)]
struct DirFile {
    name: String,
    length: usize,
    impossible: bool,
    file_type: Option<fs::FileType>,
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    counter: u64,
    contents: Option<DirectoryContents>,
}

struct DirStream {
    contents: DirectoryContents,
    entries: Vec<DirEntry>,
    current: usize,
}

fn directory_contents_hash(dc: &DirectoryContents) -> u64 {
    (dc.dev << 4) ^ dc.ino
}

fn find_directory(name: &str) -> Directory {
    let mut directories: HashMap<String, Directory> = HashMap::new();
    
    let dir_key = name.to_string();
    let dir = directories.entry(dir_key.clone()).or_insert_with(|| {
        let len = name.len();
        Directory {
            name: dir_key,
            counter: COMMAND_COUNT.load(Ordering::SeqCst),
            contents: None,
        }
    });

    if let Some(contents) = &dir.contents {
        if contents.counter == COMMAND_COUNT.load(Ordering::SeqCst) {
            return dir.clone();
        }
    }

    let metadata = match fs::metadata(name) {
        Ok(m) => m,
        Err(_) => return dir.clone(),
    };

    let dev = metadata.dev();
    let ino = metadata.ino();

    let dc_key = DirectoryContents {
        dev,
        ino,
        dirfiles: HashMap::new(),
        counter: 0,
        dirstream: None,
    };

    let dc = directories.entry(name.to_string())
        .or_insert_with(|| dc_key.clone())
        .contents
        .get_or_insert(dc_key);

    dc.counter = COMMAND_COUNT.load(Ordering::SeqCst);

    if dc.dirstream.is_none() {
        dc.dirstream = match fs::read_dir(name) {
            Ok(rd) => {
                unsafe { OPEN_DIRECTORIES += 1 };
                Some(rd)
            },
            Err(_) => None,
        };
    }

    dir.clone()
}

fn dir_contents_file_exists_p(dir: &Directory, filename: Option<&str>) -> bool {
    let dc = match &dir.contents {
        Some(dc) => dc,
        None => return false,
    };

    if let Some(fname) = filename {
        if let Some(df) = dc.dirfiles.get(fname) {
            return !df.impossible;
        }
    }

    if dc.dirstream.is_none() {
        return false;
    }

    let mut entries = match dc.dirstream.as_ref().unwrap().collect::<io::Result<Vec<_>>>() {
        Ok(e) => e,
        Err(_) => return false,
    };

    for entry in entries {
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let len = name.len();
        
        let df = DirFile {
            name: name.clone(),
            length: len,
            impossible: false,
            file_type: Some(entry.file_type().unwrap()),
        };

        dc.dirfiles.insert(name.clone(), df);

        if let Some(fname) = filename {
            if name == fname {
                return true;
            }
        }
    }

    unsafe { OPEN_DIRECTORIES -= 1; }
    dc.dirstream = None;
    false
}

fn dir_file_exists_p(dirname: &str, filename: &str) -> bool {
    dir_contents_file_exists_p(&find_directory(dirname), Some(filename))
}

fn file_exists_p(name: &str) -> bool {
    let path = Path::new(name);
    let parent = path.parent().unwrap_or(Path::new("."));
    let file_name = path.file_name().unwrap().to_str().unwrap();
    dir_file_exists_p(parent.to_str().unwrap(), file_name)
}

fn file_impossible(filename: &str) {
    let path = Path::new(filename);
    let parent = path.parent().unwrap_or(Path::new("."));
    let file_name = path.file_name().unwrap().to_str().unwrap();
    
    let mut dir = find_directory(parent.to_str().unwrap());
    if dir.contents.is_none() {
        dir.contents = Some(DirectoryContents {
            dev: 0,
            ino: 0,
            dirfiles: HashMap::new(),
            counter: 0,
            dirstream: None,
        });
    }

    let dc = dir.contents.as_mut().unwrap();
    let len = file_name.len();
    
    let df = DirFile {
        name: file_name.to_string(),
        length: len,
        impossible: true,
        file_type: None,
    };

    dc.dirfiles.insert(file_name.to_string(), df);
}

fn file_impossible_p(filename: &str) -> bool {
    let path = Path::new(filename);
    let parent = path.parent().unwrap_or(Path::new("."));
    let file_name = path.file_name().unwrap().to_str().unwrap();
    
    let dir = find_directory(parent.to_str().unwrap());
    let dc = match dir.contents {
        Some(ref dc) => dc,
        None => return false,
    };

    match dc.dirfiles.get(file_name) {
        Some(df) => df.impossible,
        None => false,
    }
}

fn dir_name(dir: &str) -> String {
    find_directory(dir).name
}

fn open_dirstream(directory: &str) -> Option<DirStream> {
    let dir = find_directory(directory);
    let dc = match dir.contents {
        Some(ref dc) => dc,
        None => return None,
    };

    let entries = match dc.dirstream.as_ref() {
        Some(rd) => match rd.collect::<io::Result<Vec<_>>>() {
            Ok(e) => e,
            Err(_) => return None,
        },
        None => return None,
    };

    Some(DirStream {
        contents: dc.clone(),
        entries,
        current: 0,
    })
}

fn read_dirstream(stream: &mut DirStream) -> Option<DirEntry> {
    if stream.current < stream.entries.len() {
        let entry = stream.entries[stream.current].clone();
        stream.current += 1;
        Some(entry)
    } else {
        None
    }
}

fn hash_init_directories() {
    // Initialization handled by HashMap in Rust
}

fn main() {
    // Example usage
    hash_init_directories();
    println!("File exists: {}", file_exists_p("Makefile"));
}