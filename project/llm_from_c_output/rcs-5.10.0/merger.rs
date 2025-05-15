/* three-way file merge internals

   Copyright (C) 2010-2020 Thien-Thi Nguyen
   Copyright (C) 1991, 1992, 1993, 1994, 1995 Paul Eggert

   This file is part of GNU RCS.

   GNU RCS is free software: you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   GNU RCS is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty
   of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
   See the GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;

const DIFF_SUCCESS: i32 = 0;
const DIFF_FAILURE: i32 = 1;
const DIFF_TROUBLE: i32 = 2;

struct Symdef {
    meaningful: String,
    underlying: String,
}

fn merge(tostdout: bool, edarg: Option<&str>, three_manifestations: [Symdef; 3]) -> io::Result<i32> {
    let mut a = [PathBuf::new(), PathBuf::new(), PathBuf::new()];
    
    for i in (0..3).rev() {
        let filename = Path::new(&three_manifestations[i].underlying);
        
        if filename.starts_with("-") {
            let mut new_path = PathBuf::from(".");
            new_path.push(filename);
            a[i] = new_path;
        } else {
            a[i] = filename.to_path_buf();
        }
    }

    let edarg = edarg.unwrap_or("-E");
    let mut s = DIFF_SUCCESS;

    if has_diff3() {
        let output_file = if !tostdout {
            Some(maketemp(0)?)
        } else {
            None
        };

        let status = Command::new("diff3")
            .arg(edarg)
            .arg("-am")
            .arg("-L").arg(&three_manifestations[0].meaningful)
            .arg("-L").arg(&three_manifestations[1].meaningful)
            .arg("-L").arg(&three_manifestations[2].meaningful)
            .arg(&a[0])
            .arg(&a[1])
            .arg(&a[2])
            .status()?;

        s = status.code().unwrap_or(DIFF_TROUBLE);

        if s == DIFF_TROUBLE {
            return Err(io::Error::new(io::ErrorKind::Other, "diff3 trouble"));
        }

        if s == DIFF_FAILURE {
            eprintln!("conflicts during merge");
        }

        if let Some(ref temp_path) = output_file {
            let mut output = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&a[0])?;

            let mut temp_file = File::open(temp_path)?;
            io::copy(&mut temp_file, &mut output)?;
        }
    } else {
        let mut d = [PathBuf::new(), PathBuf::new()];
        
        for i in 0..2 {
            let temp_path = maketemp(i)?;
            let status = Command::new("diff")
                .arg(&a[i])
                .arg(&a[2])
                .status()?;
                
            if status.code().unwrap_or(DIFF_TROUBLE) == DIFF_TROUBLE {
                return Err(io::Error::new(io::ErrorKind::Other, "diff failed"));
            }
            d[i] = temp_path;
        }

        let t = maketemp(2)?;
        let status = Command::new("diff3")
            .arg(edarg)
            .arg(&d[0])
            .arg(&d[1])
            .arg(&a[0])
            .arg(&a[1])
            .arg(&a[2])
            .arg(&three_manifestations[0].meaningful)
            .arg(&three_manifestations[2].meaningful)
            .status()?;

        s = status.code().unwrap_or(DIFF_FAILURE);
        if s != DIFF_SUCCESS {
            s = DIFF_FAILURE;
            eprintln!("overlaps or other problems during merge");
        }

        let mut f = OpenOptions::new()
            .append(true)
            .read(true)
            .open(&t)?;

        writeln!(f, "{}", if tostdout { "1,$p" } else { "w" })?;
        f.seek(SeekFrom::Start(0))?;
        f.sync_all()?;

        let status = Command::new("ed")
            .arg("-")
            .arg(&a[0])
            .stdin(f)
            .status()?;

        if !status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "ed command failed"));
        }
    }

    tempunlink()?;
    Ok(s)
}

fn has_diff3() -> bool {
    Command::new("which").arg("diff3").output().map(|o| o.status.success()).unwrap_or(false)
}

fn maketemp(_num: i32) -> io::Result<PathBuf> {
    tempfile::Builder::new().tempfile()?.path().to_path_buf()
}

fn tempunlink() -> io::Result<()> {
    // Implementation depends on how temp files are managed
    Ok(())
}