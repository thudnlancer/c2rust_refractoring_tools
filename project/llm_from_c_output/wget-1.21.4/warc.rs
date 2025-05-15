use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use sha1::{Sha1, Digest};
use base32;
use std::net::IpAddr;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::os::unix::fs::OpenOptionsExt;
use libc::O_TEMPORARY;

pub struct WarcWriter {
    log_file: Option<File>,
    manifest_file: Option<File>,
    current_file: Option<File>,
    current_filename: Option<String>,
    current_file_number: i32,
    write_ok: bool,
    cdx_file: Option<File>,
    warcinfo_uuid: String,
    cdx_dedup_table: HashMap<Vec<u8>, WarcCdxRecord>,
}

struct WarcCdxRecord {
    url: String,
    uuid: String,
    digest: Vec<u8>,
}

impl WarcWriter {
    pub fn new() -> Self {
        WarcWriter {
            log_file: None,
            manifest_file: None,
            current_file: None,
            current_filename: None,
            current_file_number: -1,
            write_ok: true,
            cdx_file: None,
            warcinfo_uuid: String::new(),
            cdx_dedup_table: HashMap::new(),
        }
    }

    pub fn init(&mut self) -> io::Result<()> {
        self.write_ok = true;
        self.start_new_file(false)
    }

    pub fn close(&mut self) -> io::Result<()> {
        if let Some(file) = self.current_file.take() {
            self.write_metadata()?;
            self.warcinfo_uuid.clear();
        }
        if let Some(file) = self.cdx_file.take() {
            // file will be closed when dropped
        }
        Ok(())
    }

    fn start_new_file(&mut self, meta: bool) -> io::Result<()> {
        let extension = if meta { "meta.warc.gz" } else { "warc.gz" };
        let filename = format!("{}-{:05}.{}", "base", self.current_file_number + 1, extension);
        
        self.current_file_number += 1;
        self.current_filename = Some(filename.clone());
        
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&filename)?;
            
        self.current_file = Some(file);
        self.write_warcinfo_record(&filename)
    }

    fn write_warcinfo_record(&mut self, filename: &str) -> io::Result<()> {
        let timestamp = self.timestamp();
        self.warcinfo_uuid = self.uuid_str();
        
        self.write_start_record()?;
        self.write_header("WARC-Type", "warcinfo")?;
        self.write_header("Content-Type", "application/warc-fields")?;
        self.write_header("WARC-Date", &timestamp)?;
        self.write_header("WARC-Record-ID", &self.warcinfo_uuid)?;
        
        let basename = Path::new(filename).file_name()
            .and_then(OsStr::to_str)
            .unwrap_or(filename);
        self.write_header("WARC-Filename", basename)?;

        let mut tmpfile = self.tempfile()?;
        writeln!(tmpfile, "software: Wget/1.0 (Rust)")?;
        writeln!(tmpfile, "format: WARC File Format 1.0")?;
        writeln!(tmpfile, "conformsTo: http://bibnum.bnf.fr/WARC/WARC_ISO_28500_version1_latestdraft.pdf")?;

        self.write_digest_headers(&mut tmpfile, -1)?;
        self.write_block_from_file(&mut tmpfile)?;
        self.write_end_record()?;
        
        Ok(())
    }

    fn write_metadata(&mut self) -> io::Result<()> {
        let manifest_uuid = self.uuid_str();
        
        if let Some(ref mut manifest) = self.manifest_file {
            self.write_metadata_record(
                &manifest_uuid,
                "metadata://gnu.org/software/wget/warc/MANIFEST.txt",
                None,
                None,
                None,
                "text/plain",
                manifest,
                -1,
            )?;
        }

        let mut tmpfile = self.tempfile()?;
        writeln!(tmpfile, "arguments")?;
        self.write_resource_record(
            None,
            "metadata://gnu.org/software/wget/warc/wget_arguments.txt",
            None,
            &manifest_uuid,
            None,
            "text/plain",
            &mut tmpfile,
            -1,
        )?;

        if let Some(ref mut log) = self.log_file {
            self.write_resource_record(
                None,
                "metadata://gnu.org/software/wget/warc/wget.log",
                None,
                &manifest_uuid,
                None,
                "text/plain",
                log,
                -1,
            )?;
        }
        
        Ok(())
    }

    fn tempfile(&self) -> io::Result<File> {
        let mut opts = OpenOptions::new();
        opts.read(true)
            .write(true)
            .create(true)
            .truncate(true);
        
        #[cfg(unix)]
        opts.custom_flags(O_TEMPORARY);
        
        tempfile::tempfile()
    }

    fn timestamp(&self) -> String {
        let start = SystemTime::now();
        let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        format!("{}", since_epoch.as_secs())
    }

    fn uuid_str(&self) -> String {
        format!("<urn:uuid:{}>", Uuid::new_v4())
    }

    fn write_start_record(&mut self) -> io::Result<()> {
        if !self.write_ok {
            return Ok(());
        }
        self.write_string("WARC/1.0\r\n")
    }

    fn write_header(&mut self, name: &str, value: &str) -> io::Result<()> {
        if !value.is_empty() {
            self.write_string(name)?;
            self.write_string(": ")?;
            self.write_string(value)?;
            self.write_string("\r\n")?;
        }
        Ok(())
    }

    fn write_header_uri(&mut self, name: &str, value: &str) -> io::Result<()> {
        if !value.is_empty() {
            self.write_string(name)?;
            self.write_string(": <")?;
            self.write_string(value)?;
            self.write_string(">\r\n")?;
        }
        Ok(())
    }

    fn write_string(&mut self, s: &str) -> io::Result<()> {
        if self.write_ok {
            if let Some(ref mut file) = self.current_file {
                file.write_all(s.as_bytes())?;
            }
        }
        Ok(())
    }

    fn write_block_from_file(&mut self, file: &mut File) -> io::Result<()> {
        let len = file.seek(SeekFrom::End(0))?;
        self.write_header("Content-Length", &len.to_string())?;
        self.write_string("\r\n")?;
        
        file.seek(SeekFrom::Start(0))?;
        let mut buffer = vec![0; 8192];
        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            if let Some(ref mut warc_file) = self.current_file {
                warc_file.write_all(&buffer[..n])?;
            }
        }
        Ok(())
    }

    fn write_end_record(&mut self) -> io::Result<()> {
        self.write_string("\r\n\r\n")
    }

    fn write_digest_headers(&mut self, file: &mut File, payload_offset: i64) -> io::Result<()> {
        let mut sha_block = Sha1::new();
        let mut sha_payload = Sha1::new();
        
        file.seek(SeekFrom::Start(0))?;
        let mut buffer = vec![0; 8192];
        let mut pos = 0;
        
        loop {
            let n = file.read(&mut buffer)?;
            if n == 0 {
                break;
            }
            
            sha_block.update(&buffer[..n]);
            if payload_offset >= 0 && pos >= payload_offset as u64 {
                sha_payload.update(&buffer[..n]);
            }
            pos += n as u64;
        }
        
        let block_digest = sha_block.finalize();
        let payload_digest = if payload_offset >= 0 {
            Some(sha_payload.finalize())
        } else {
            None
        };
        
        self.write_header("WARC-Block-Digest", &format!("sha1:{}", base32::encode(base32::Alphabet::RFC4648 { padding: true }, &block_digest)))?;
        
        if let Some(digest) = payload_digest {
            self.write_header("WARC-Payload-Digest", &format!("sha1:{}", base32::encode(base32::Alphabet::RFC4648 { padding: true }, &digest)))?;
        }
        
        Ok(())
    }

    pub fn write_request_record(
        &mut self,
        url: &str,
        timestamp: &str,
        record_uuid: &str,
        ip: Option<IpAddr>,
        mut body: File,
        payload_offset: u64,
    ) -> io::Result<()> {
        self.write_start_record()?;
        self.write_header("WARC-Type", "request")?;
        self.write_header_uri("WARC-Target-URI", url)?;
        self.write_header("Content-Type", "application/http;msgtype=request")?;
        self.write_header("WARC-Date", timestamp)?;
        self.write_header("WARC-Record-ID", record_uuid)?;
        
        if let Some(ip) = ip {
            self.write_header("WARC-IP-Address", &ip.to_string())?;
        }
        
        self.write_header("WARC-Warcinfo-ID", &self.warcinfo_uuid)?;
        self.write_digest_headers(&mut body, payload_offset as i64)?;
        self.write_block_from_file(&mut body)?;
        self.write_end_record()
    }

    pub fn write_response_record(
        &mut self,
        url: &str,
        timestamp: &str,
        concurrent_to_uuid: &str,
        ip: Option<IpAddr>,
        mut body: File,
        payload_offset: u64,
        mime_type: &str,
        response_code: i32,
        redirect_location: Option<&str>,
    ) -> io::Result<()> {
        let response_uuid = self.uuid_str();
        let offset = self.current_file.as_ref().map_or(0, |f| f.seek(SeekFrom::Current(0)).unwrap_or(0);

        self.write_start_record()?;
        self.write_header("WARC-Type", "response")?;
        self.write_header("WARC-Record-ID", &response_uuid)?;
        self.write_header("WARC-Warcinfo-ID", &self.warcinfo_uuid)?;
        self.write_header("WARC-Concurrent-To", concurrent_to_uuid)?;
        self.write_header_uri("WARC-Target-URI", url)?;
        self.write_header("WARC-Date", timestamp)?;
        
        if let Some(ip) = ip {
            self.write_header("WARC-IP-Address", &ip.to_string())?;
        }
        
        self.write_header("Content-Type", "application/http;msgtype=response")?;
        self.write_digest_headers(&mut body, payload_offset as i64)?;
        self.write_block_from_file(&mut body)?;
        self.write_end_record()?;

        if self.cdx_file.is_some() {
            self.write_cdx_record(
                url,
                timestamp,
                mime_type,
                response_code,
                None, // digest would be calculated in write_digest_headers
                redirect_location,
                offset,
                self.current_filename.as_deref().unwrap_or(""),
                &response_uuid,
            )?;
        }

        Ok(())
    }

    fn write_cdx_record(
        &mut self,
        url: &str,
        timestamp: &str,
        mime_type: &str,
        response_code: i32,
        payload_digest: Option<&str>,
        redirect_location: Option<&str>,
        offset: u64,
        warc_filename: &str,
        response_uuid: &str,
    ) -> io::Result<()> {
        if let Some(ref mut cdx_file) = self.cdx_file {
            let cdx_timestamp = if timestamp.len() >= 14 {
                format!("{}{}{}{}{}{}",
                    &timestamp[0..4], &timestamp[5..7], &timestamp[8..10],
                    &timestamp[11..13], &timestamp[14..16], &timestamp[17..19])
            } else {
                String::new()
            };

            writeln!(
                cdx_file,
                "{} {} {} {} {} {} {} - {} {} {}",
                url,
                cdx_timestamp,
                url,
                mime_type,
                response_code,
                payload_digest.unwrap_or("-"),
                redirect_location.unwrap_or("-"),
                offset,
                warc_filename,
                response_uuid
            )?;
        }
        Ok(())
    }

    fn write_record(
        &mut self,
        record_type: &str,
        resource_uuid: Option<&str>,
        url: &str,
        timestamp: &str,
        concurrent_to_uuid: Option<&str>,
        ip: Option<IpAddr>,
        content_type: &str,
        mut body: File,
        payload_offset: i64,
    ) -> io::Result<()> {
        let uuid = resource_uuid.unwrap_or_else(|| {
            let u = self.uuid_str();
            &u
        });

        self.write_start_record()?;
        self.write_header("WARC-Type", record_type)?;
        self.write_header("WARC-Record-ID", uuid)?;
        self.write_header("WARC-Warcinfo-ID", &self.warcinfo_uuid)?;
        
        if let Some(ct) = concurrent_to_uuid {
            self.write_header("WARC-Concurrent-To", ct)?;
        }
        
        self.write_header_uri("WARC-Target-URI", url)?;
        self.write_header("WARC-Date", timestamp)?;
        
        if let Some(ip) = ip {
            self.write_header("WARC-IP-Address", &ip.to_string())?;
        }
        
        self.write_header("Content-Type", content_type)?;
        self.write_digest_headers(&mut body, payload_offset)?;
        self.write_block_from_file(&mut body)?;
        self.write_end_record()
    }

    pub fn write_resource_record(
        &mut self,
        resource_uuid: Option<&str>,
        url: &str,
        timestamp: Option<&str>,
        concurrent_to_uuid: Option<&str>,
        ip: Option<IpAddr>,
        content_type: &str,
        body: &mut File,
        payload_offset: i64,
    ) -> io::Result<()> {
        self.write_record(
            "resource",
            resource_uuid,
            url,
            timestamp.unwrap_or(&self.timestamp()),
            concurrent_to_uuid,
            ip,
            content_type,
            body,
            payload_offset,
        )
    }

    pub fn write_metadata_record(
        &mut self,
        record_uuid: Option<&str>,
        url: &str,
        timestamp: Option<&str>,
        concurrent_to_uuid: Option<&str>,
        ip: Option<IpAddr>,
        content_type: &str,
        body: &mut File,
        payload_offset: i64,
    ) -> io::Result<()> {
        self.write_record(
            "metadata",
            record_uuid,
            url,
            timestamp.unwrap_or(&self.timestamp()),
            concurrent_to_uuid,
            ip,
            content_type,
            body,
            payload_offset,
        )
    }
}