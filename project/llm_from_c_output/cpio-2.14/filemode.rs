use std::os::unix::fs::FileTypeExt;
use std::os::unix::prelude::MetadataExt;

/// Return a character indicating the type of file described by file mode bits:
/// 'd' for directories
/// 'b' for block special files
/// 'c' for character special files
/// 'm' for multiplexor files
/// 'l' for symbolic links
/// 's' for sockets
/// 'p' for fifos
/// '-' for regular files
/// '?' for any other file type.
fn ftypelet(mode: u32) -> char {
    let file_type = mode & libc::S_IFMT;

    match file_type {
        libc::S_IFBLK => 'b',
        libc::S_IFCHR => 'c',
        libc::S_IFDIR => 'd',
        libc::S_IFREG => '-',
        libc::S_IFIFO => 'p',
        libc::S_IFLNK => 'l',
        libc::S_IFSOCK => 's',
        #[cfg(any(target_os = "v7", target_os = "unix"))]
        libc::S_IFMPB | libc::S_IFMPC => 'm',
        #[cfg(target_os = "hpux")]
        libc::S_IFNWK => 'n',
        _ => '?',
    }
}

/// Look at read, write, and execute bits in bits and set flags in chars accordingly.
fn rwx(bits: u32, chars: &mut [char; 3]) {
    chars[0] = if bits & libc::S_IRUSR != 0 { 'r' } else { '-' };
    chars[1] = if bits & libc::S_IWUSR != 0 { 'w' } else { '-' };
    chars[2] = if bits & libc::S_IXUSR != 0 { 'x' } else { '-' };
}

/// Set the 's' and 't' flags in file attributes string chars, according to the file mode bits.
fn setst(bits: u32, chars: &mut [char; 10]) {
    if bits & libc::S_ISUID != 0 {
        chars[3] = if chars[3] != 'x' { 'S' } else { 's' };
    }
    if bits & libc::S_ISGID != 0 {
        chars[6] = if chars[6] != 'x' { 'S' } else { 's' };
    }
    if bits & libc::S_ISVTX != 0 {
        chars[9] = if chars[9] != 'x' { 'T' } else { 't' };
    }
}

/// Like filemodestring, but only the relevant part of the mode is given as an argument.
pub fn mode_string(mode: u32) -> [char; 10] {
    let mut str = ['-'; 10];
    str[0] = ftypelet(mode);

    let mut owner = ['-'; 3];
    rwx((mode & 0o700) >> 6, &mut owner);
    str[1..4].copy_from_slice(&owner);

    let mut group = ['-'; 3];
    rwx((mode & 0o70) >> 3, &mut group);
    str[4..7].copy_from_slice(&group);

    let mut other = ['-'; 3];
    rwx(mode & 0o7, &mut other);
    str[7..10].copy_from_slice(&other);

    setst(mode, &mut str);
    str
}

/// Fill in string with an ls-style ASCII representation of the st_mode field.
/// 10 characters are returned in the array:
/// 0: File type ('d', 'c', 'b', 'm', 'l', 's', 'p', '-', '?')
/// 1-3: Owner permissions ('r', 'w', 'x'/'s'/'S')
/// 4-6: Group permissions ('r', 'w', 'x'/'s'/'S')
/// 7-9: Other permissions ('r', 'w', 'x'/'t'/'T')
pub fn filemodestring(metadata: &std::fs::Metadata) -> [char; 10] {
    mode_string(metadata.mode())
}