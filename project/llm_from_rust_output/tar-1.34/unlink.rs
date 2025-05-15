use std::ffi::{CString, CStr};
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::fs::{remove_file, remove_dir, metadata};
use std::io::{Error, ErrorKind};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::collections::LinkedList;

#[derive(Debug)]
struct DeferredUnlink {
    dir_idx: i32,
    file_name: PathBuf,
    is_dir: bool,
    records_written: i64,
}

static DUNLINK_COUNT: AtomicUsize = AtomicUsize::new(0);
static DEFERRED_UNLINK_DELAY: AtomicUsize = AtomicUsize::new(0);
static DUNLINK_LIST: Mutex<LinkedList<DeferredUnlink>> = Mutex::new(LinkedList::new());

fn flush_deferred_unlinks(force: bool) -> Result<(), Error> {
    let mut list = DUNLINK_LIST.lock().unwrap();
    let records_written = 0; // TODO: Replace with actual records_written value
    let delay = DEFERRED_UNLINK_DELAY.load(Ordering::Relaxed);

    list.retain(|item| {
        if force || records_written as usize > (item.records_written as usize).wrapping_add(delay) {
            let result = if item.is_dir {
                remove_dir(&item.file_name)
            } else {
                remove_file(&item.file_name)
            };

            match result {
                Ok(_) => {
                    DUNLINK_COUNT.fetch_sub(1, Ordering::Relaxed);
                    false
                },
                Err(e) if e.kind() == ErrorKind::NotFound => false,
                Err(e) if e.raw_os_error() == Some(17) || e.raw_os_error() == Some(39) => true,
                Err(_) => {
                    // TODO: Handle error reporting (rmdir_error/unlink_error)
                    false
                }
            }
        } else {
            true
        }
    });

    Ok(())
}

fn finish_deferred_unlinks() -> Result<(), Error> {
    flush_deferred_unlinks(true)?;
    let mut list = DUNLINK_LIST.lock().unwrap();
    list.clear();
    DUNLINK_COUNT.store(0, Ordering::Relaxed);
    Ok(())
}

fn queue_deferred_unlink(name: &Path, is_dir: bool) -> Result<(), Error> {
    let records_written = 0; // TODO: Replace with actual records_written value
    let delay = DEFERRED_UNLINK_DELAY.load(Ordering::Relaxed);

    {
        let list = DUNLINK_LIST.lock().unwrap();
        if let Some(head) = list.front() {
            if records_written as usize > (head.records_written as usize).wrapping_add(delay) {
                flush_deferred_unlinks(false)?;
            }
        }
    }

    let mut normalized_name = name.to_path_buf();
    // TODO: Implement normalize_filename_x equivalent
    // normalize_filename_x(&mut normalized_name);

    let item = DeferredUnlink {
        dir_idx: 0, // TODO: Replace with actual chdir_current value
        file_name: normalized_name,
        is_dir,
        records_written,
    };

    let mut list = DUNLINK_LIST.lock().unwrap();
    if is_dir && (name.as_os_str().is_empty() || name == Path::new(".")) {
        let mut insert_pos = None;
        for (i, existing) in list.iter().enumerate() {
            if existing.is_dir 
                && (existing.file_name.as_os_str().is_empty() || existing.file_name == Path::new("."))
                && existing.dir_idx < item.dir_idx
            {
                insert_pos = Some(i);
                break;
            }
        }
        
        if let Some(pos) = insert_pos {
            let mut tail = list.split_off(pos);
            list.push_back(item);
            list.append(&mut tail);
        } else {
            list.push_back(item);
        }
    } else {
        list.push_back(item);
    }

    DUNLINK_COUNT.fetch_add(1, Ordering::Relaxed);
    Ok(())
}