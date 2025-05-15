use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ffi::{CString, OsString};
use std::os::unix::ffi::OsStringExt;
use tempfile::NamedTempFile;

struct SymDef {
    meaningful: String,
    underlying: PathBuf,
}

fn merge(
    to_stdout: bool,
    ed_arg: Option<&str>,
    three_manifestations: [SymDef; 3],
) -> io::Result<i32> {
    let mut args = vec![
        OsString::from("-am"),
        OsString::from("-L"),
        OsString::from(&three_manifestations[0].meaningful),
        OsString::from("-L"),
        OsString::from(&three_manifestations[1].meaningful),
        OsString::from("-L"),
        OsString::from(&three_manifestations[2].meaningful),
    ];

    let mut paths = Vec::new();
    for manifest in &three_manifestations {
        let path = if manifest.underlying.to_str() == Some("-") {
            PathBuf::from("/dev/stdin")
        } else {
            manifest.underlying.clone()
        };
        paths.push(path);
    }

    let ed_arg = ed_arg.unwrap_or("-E");
    args.insert(0, OsString::from(ed_arg));

    let output_file = if to_stdout {
        None
    } else {
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path().to_owned();
        Some((temp_file, path))
    };

    let mut cmd = Command::new("diff3");
    cmd.args(args)
        .arg(&paths[0])
        .arg(&paths[1])
        .arg(&paths[2]);

    if let Some((_, ref output_path)) = output_file {
        cmd.arg(output_path);
    }

    let status = cmd.status()?;
    let exit_code = status.code().unwrap_or(1);

    if exit_code == 2 {
        std::process::exit(0);
    } else if exit_code == 1 {
        eprintln!("conflicts during merge");
    }

    if let Some((temp_file, output_path)) = output_file {
        let mut output = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&three_manifestations[0].underlying)?;

        let mut content = Vec::new();
        File::open(output_path)?.read_to_end(&mut content)?;
        output.write_all(&content)?;
    }

    Ok(exit_code)
}