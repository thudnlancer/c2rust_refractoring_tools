use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::os::unix::io::FromRawFd;
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};
use libc::{c_char, c_int, c_void, size_t, off_t, FILE, time_t, tm};
use sha1::{Sha1, Digest};
use base32;
use tempfile::tempfile;

static mut WARC_LOG_FP: Option<File> = None;
static mut WARC_MANIFEST_FP: Option<File> = None;
static mut WARC_CURRENT_FILE: Option<File> = None;
static mut WARC_CURRENT_GZFILE: Option<File> = None;
static mut WARC_CURRENT_GZFILE_OFFSET: off_t = 0;
static mut WARC_CURRENT_GZFILE_UNCOMPRESSED_SIZE: off_t = 0;
static mut WARC_WRITE_OK: bool = true;
static mut WARC_CURRENT_CDX_FILE: Option<File> = None;
static mut WARC_CURRENT_WARCINFO_UUID_STR: [c_char; 48] = [0; 48];
static mut WARC_CURRENT_FILENAME: Option<CString> = None;
static mut WARC_CURRENT_FILE_NUMBER: c_int = 0;
static mut WARC_CDX_DEDUP_TABLE: Option<HashMap<Vec<u8>, WarcCdxRecord>> = None;

struct WarcCdxRecord {
    url: CString,
    uuid: CString,
    digest: [u8; 20],
}

fn warc_write_buffer(buffer: &[u8]) -> bool {
    unsafe {
        if let Some(gzfile) = &mut WARC_CURRENT_GZFILE {
            WARC_CURRENT_GZFILE_UNCOMPRESSED_SIZE += buffer.len() as off_t;
            gzfile.write_all(buffer).is_ok()
        } else if let Some(file) = &mut WARC_CURRENT_FILE {
            file.write_all(buffer).is_ok()
        } else {
            false
        }
    }
}

fn warc_write_string(s: &str) -> bool {
    if !unsafe { WARC_WRITE_OK } {
        return false;
    }
    warc_write_buffer(s.as_bytes())
}

fn warc_write_start_record() -> bool {
    unsafe {
        if !WARC_WRITE_OK {
            return false;
        }

        if let Some(file) = &mut WARC_CURRENT_FILE {
            file.flush().ok();
            if opt.warc_maxsize > 0 && file.metadata().unwrap().len() >= opt.warc_maxsize as u64 {
                warc_start_new_file(false);
            }

            if opt.warc_compression_enabled {
                WARC_CURRENT_GZFILE_OFFSET = file.seek(SeekFrom::Current(14)).unwrap() as off_t;
                let dup_fd = libc::dup(file.as_raw_fd());
                if dup_fd < 0 {
                    WARC_WRITE_OK = false;
                    return false;
                }
                WARC_CURRENT_GZFILE = Some(unsafe { File::from_raw_fd(dup_fd) });
                WARC_CURRENT_GZFILE_UNCOMPRESSED_SIZE = 0;
            }
        }

        warc_write_string("WARC/1.0\r\n")
    }
}

fn warc_write_header(name: &str, value: &str) -> bool {
    if value.is_empty() {
        return unsafe { WARC_WRITE_OK };
    }
    warc_write_string(name) && 
    warc_write_string(": ") && 
    warc_write_string(value) && 
    warc_write_string("\r\n")
}

fn warc_write_header_uri(name: &str, value: &str) -> bool {
    if value.is_empty() {
        return unsafe { WARC_WRITE_OK };
    }
    warc_write_string(name) && 
    warc_write_string(": <") && 
    warc_write_string(value) && 
    warc_write_string(">\r\n")
}

fn warc_write_block_from_file(mut file: File) -> bool {
    let mut content_length = String::new();
    file.seek(SeekFrom::End(0)).unwrap();
    content_length = file.seek(SeekFrom::Current(0)).unwrap().to_string();
    
    warc_write_header("Content-Length", &content_length) &&
    warc_write_string("\r\n") &&
    file.seek(SeekFrom::Start(0)).is_ok() && {
        let mut buffer = [0; 8192];
        while let Ok(n) = file.read(&mut buffer) {
            if n == 0 {
                break;
            }
            if !warc_write_buffer(&buffer[..n]) {
                unsafe { WARC_WRITE_OK = false; }
                break;
            }
        }
        unsafe { WARC_WRITE_OK }
    }
}

fn warc_write_end_record() -> bool {
    if !warc_write_buffer(b"\r\n\r\n") {
        unsafe { WARC_WRITE_OK = false; }
        return false;
    }

    unsafe {
        if let Some(gzfile) = WARC_CURRENT_GZFILE.take() {
            // Handle gzip closing and extra header writing
            // ... (implementation omitted for brevity)
        }
        WARC_WRITE_OK
    }
}

fn warc_write_date_header(timestamp: Option<&str>) -> bool {
    let current_timestamp = if let Some(ts) = timestamp {
        ts.to_string()
    } else {
        warc_timestamp()
    };
    warc_write_header("WARC-Date", &current_timestamp)
}

fn warc_write_ip_header(ip: Option<&IpAddress>) -> bool {
    if let Some(ip_addr) = ip {
        warc_write_header("WARC-IP-Address", &ip_addr.to_string())
    } else {
        unsafe { WARC_WRITE_OK }
    }
}

fn warc_sha1_stream_with_payload(
    mut stream: File,
    res_block: &mut [u8; 20],
    res_payload: &mut [u8; 20],
    payload_offset: off_t,
) -> bool {
    let mut hasher_block = Sha1::new();
    let mut hasher_payload = Sha1::new();
    let mut pos: off_t = 0;
    let mut buffer = vec![0; 32768];

    loop {
        let n = match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => n,
            Err(_) => return false,
        } as off_t;

        hasher_block.update(&buffer[..n as usize]);
        if payload_offset >= 0 && payload_offset < pos + n {
            let start = if payload_offset > pos {
                (payload_offset - pos) as usize
            } else {
                0
            };
            hasher_payload.update(&buffer[start..n as usize]);
        }
        pos += n;
    }

    res_block.copy_from_slice(&hasher_block.finalize());
    if payload_offset >= 0 {
        res_payload.copy_from_slice(&hasher_payload.finalize());
    }
    true
}

fn warc_base32_sha1_digest(sha1_digest: &[u8; 20]) -> String {
    let mut result = String::from("sha1:");
    let encoded = base32::encode(base32::Alphabet::RFC4648 { padding: true }, sha1_digest);
    result.push_str(&encoded);
    result
}

fn warc_write_digest_headers(file: &mut File, payload_offset: off_t) {
    if opt.warc_digests_enabled {
        let mut sha1_res_block = [0; 20];
        let mut sha1_res_payload = [0; 20];
        file.seek(SeekFrom::Start(0)).unwrap();
        
        if warc_sha1_stream_with_payload(
            file.try_clone().unwrap(),
            &mut sha1_res_block,
            &mut sha1_res_payload,
            payload_offset,
        ) {
            let digest = warc_base32_sha1_digest(&sha1_res_block);
            warc_write_header("WARC-Block-Digest", &digest);
            
            if payload_offset >= 0 {
                let payload_digest = warc_base32_sha1_digest(&sha1_res_payload);
                warc_write_header("WARC-Payload-Digest", &payload_digest);
            }
        }
    }
}

fn warc_timestamp() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let tm = unsafe { *gmtime(&(now as time_t)) };
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        tm.tm_year + 1900,
        tm.tm_mon + 1,
        tm.tm_mday,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec
    )
}

fn warc_uuid_str() -> String {
    let mut uuid = [0u8; 16];
    getrandom::getrandom(&mut uuid).unwrap();
    
    uuid[6] = (uuid[6] & 0x0f) | 0x40;
    uuid[8] = (uuid[8] & 0x3f) | 0x80;
    
    format!(
        "<urn:uuid:{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}>",
        uuid[0], uuid[1], uuid[2], uuid[3],
        uuid[4], uuid[5], uuid[6], uuid[7],
        uuid[8], uuid[9], uuid[10], uuid[11],
        uuid[12], uuid[13], uuid[14], uuid[15]
    )
}

// ... (additional functions would follow similar patterns)