/*  Copyright 1998-2002,2009 Alain Knaff.
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

use std::fs::{self, DirEntry, File, Metadata};
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

struct DirStream {
    pathname: PathBuf,
    dir: fs::ReadDir,
    statbuf: Metadata,
}

impl DirStream {
    fn get_dir_data(&self, date: Option<&mut SystemTime>, size: Option<&mut u64>, 
                   type_: Option<&mut i32>, address: Option<&mut u32>) -> io::Result<()> {
        if let Some(date) = date {
            *date = self.statbuf.modified()?;
        }
        if let Some(size) = size {
            *size = self.statbuf.size();
        }
        if let Some(type_) = type_ {
            *type_ = 1;
        }
        if let Some(address) = address {
            *address = 0;
        }
        Ok(())
    }
}

fn unix_dir_loop(stream: &DirStream, mp: &MainParam) -> io::Result<i32> {
    let mut ret = 0;
    
    for entry in stream.dir {
        if got_signal() {
            break;
        }
        
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_str().ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid filename"))?;
        
        if is_special(file_name) {
            continue;
        }
        
        let new_name = stream.pathname.join(file_name);
        
        ret |= unix_loop(stream, mp, &new_name, false)?;
    }
    
    Ok(ret)
}

fn open_dir(filename: &str) -> io::Result<DirStream> {
    let path = Path::new(filename);
    let statbuf = fs::metadata(path)?;
    let dir = fs::read_dir(path)?;
    
    Ok(DirStream {
        pathname: path.to_path_buf(),
        dir,
        statbuf,
    })
}

struct MainParam;

fn unix_loop(_stream: &DirStream, _mp: &MainParam, _path: &Path, _flag: bool) -> io::Result<i32> {
    // Implementation would go here
    Ok(0)
}

fn got_signal() -> bool {
    // Signal handling implementation would go here
    false
}

fn is_special(_name: &str) -> bool {
    // Special file check implementation would go here
    false
}