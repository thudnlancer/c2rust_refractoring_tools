use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::time::SystemTime;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KwSub {
    Kv,
    Kvl,
    K,
    V,
    O,
    B,
}

#[derive(Debug, Clone)]
pub struct Delta {
    pub num: String,
    pub date: String,
    pub author: String,
    pub lockedby: String,
    pub state: String,
    pub log: Vec<String>,
    pub text: Vec<String>,
    pub name: String,
    pub pretty_log: String,
    pub branches: Vec<String>,
    pub commitid: String,
    pub ilk: Option<Box<Delta>>,
    pub selector: bool,
    pub neck: u64,
}

#[derive(Debug)]
pub struct RcsFile {
    file: File,
    size: u64,
    pos: u64,
}

impl RcsFile {
    pub fn open<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let size = file.metadata()?.size();
        Ok(Self { file, size, pos: 0 })
    }

    pub fn try_get_byte(&mut self) -> std::io::Result<Option<u8>> {
        let mut buf = [0u8; 1];
        match self.file.read(&mut buf)? {
            0 => Ok(None),
            1 => {
                self.pos += 1;
                Ok(Some(buf[0]))
            }
            _ => unreachable!(),
        }
    }

    pub fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        let new_pos = self.file.seek(pos)?;
        self.pos = new_pos;
        Ok(new_pos)
    }
}

pub fn rcsfcmp(
    xfile: &mut RcsFile,
    xstat: &std::fs::Metadata,
    uname: &str,
    delta: &Delta,
) -> std::io::Result<i32> {
    let mut ufile = RcsFile::open(uname)?;
    let ustat = ufile.file.metadata()?;

    if xstat.size() != ustat.size() {
        return Ok(1);
    }

    let mut xc = 0u8;
    let mut uc = 0u8;
    let mut leaderlen = 0usize;
    let mut result = 0;

    loop {
        if xc != b'$' {
            xc = match xfile.try_get_byte()? {
                Some(b) => b,
                None => break,
            };
            uc = match ufile.try_get_byte()? {
                Some(b) => b,
                None => break,
            };
            if xc != uc {
                result = 1;
                break;
            }
        } else {
            let mut keyword = Vec::with_capacity(10);
            loop {
                xc = match xfile.try_get_byte()? {
                    Some(b) => b,
                    None => break,
                };
                uc = match ufile.try_get_byte()? {
                    Some(b) => b,
                    None => break,
                };
                if xc != uc {
                    break;
                }
                match xc {
                    b'\n' | b'$' | b':' => break,
                    _ => {
                        if keyword.len() < 8 {
                            keyword.push(xc);
                        } else {
                            break;
                        }
                    }
                }
            }

            if (xc == b'$' || xc == b':') && (uc == b'$' || uc == b':') {
                let keyword_str = String::from_utf8_lossy(&keyword);
                if is_keyword(&keyword_str) {
                    result = -1;
                    let mut eqkeyvals = false;
                    loop {
                        if xc != uc {
                            xc = discard_keyval(xc, xfile)?;
                            uc = discard_keyval(uc, &mut ufile)?;
                            if xc == 0xff || uc == 0xff {
                                break;
                            }
                            eqkeyvals = false;
                            break;
                        } else {
                            match xc {
                                b'\n' | b'$' => {
                                    eqkeyvals = true;
                                    break;
                                }
                                _ => {
                                    xc = match xfile.try_get_byte()? {
                                        Some(b) => b,
                                        None => 0xff,
                                    };
                                    uc = match ufile.try_get_byte()? {
                                        Some(b) => b,
                                        None => 0xff,
                                    };
                                    if xc == 0xff || uc == 0xff {
                                        break;
                                    }
                                }
                            }
                        }
                    }

                    if xc != uc {
                        result = 1;
                        break;
                    }

                    if xc == b'$' {
                        xc = match xfile.try_get_byte()? {
                            Some(b) => b,
                            None => 0xff,
                        };
                        uc = match ufile.try_get_byte()? {
                            Some(b) => b,
                            None => 0xff,
                        };
                        if xc == 0xff || uc == 0xff {
                            break;
                        }

                        if keyword_str == "Log" {
                            let mut lncnt = 0;
                            let mut ls = delta.pretty_log.len();
                            let mut sp = delta.pretty_log.as_bytes();

                            if !delta.pretty_log.starts_with("ciklog") {
                                let mut c1 = 1;
                                let mut ccnt = 0; // TODO: Replace with actual log_lead size
                                for _ in 0..ccnt {
                                    c1 += if sp[0] == b'\n' { 1 } else { 0 };
                                    sp = &sp[1..];
                                }
                                lncnt = 2 * c1 + 1;

                                for _ in 0..ls {
                                    if sp[0] == b'\n' {
                                        lncnt += c1;
                                    }
                                    sp = &sp[1..];
                                }

                                while lncnt > 0 {
                                    if xc == b'\n' {
                                        lncnt -= 1;
                                        if lncnt == 0 {
                                            break;
                                        }
                                    }
                                    xc = match xfile.try_get_byte()? {
                                        Some(b) => b,
                                        None => break,
                                    };
                                }

                                ccnt = 0; // TODO: Replace with actual conditional value
                                for _ in 0..ccnt {
                                    xc = match xfile.try_get_byte()? {
                                        Some(b) => b,
                                        None => break,
                                    };
                                    if xc == b'\n' {
                                        c1 -= 1;
                                        if c1 == 0 {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    } else if !eqkeyvals {
                        result = 1;
                        break;
                    }
                }
            }
        }

        if xc != uc {
            result = 1;
            break;
        }

        if xc == b'\n' {
            leaderlen = 0;
        } else {
            leaderlen += 1;
        }
    }

    Ok(result)
}

fn discard_keyval(mut c: u8, file: &mut RcsFile) -> std::io::Result<u8> {
    loop {
        match c {
            b'$' | b'\n' => return Ok(c),
            _ => {
                c = match file.try_get_byte()? {
                    Some(b) => b,
                    None => return Ok(0xff),
                };
            }
        }
    }
}

fn is_keyword(keyword: &str) -> bool {
    matches!(
        keyword,
        "Author" | "Date" | "Header" | "Id" | "Locker" | "Log" | "Name" | "RCSfile" | "Revision" | "Source" | "State"
    )
}