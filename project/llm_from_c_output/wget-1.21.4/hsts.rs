use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use libc::{flock, LOCK_EX};
use url::Url;

#[derive(Debug, Clone)]
struct HstsKh {
    host: String,
    explicit_port: i32,
}

#[derive(Debug, Clone)]
struct HstsKhInfo {
    created: i64,
    max_age: i64,
    include_subdomains: bool,
}

#[derive(Debug, PartialEq)]
enum HstsKhMatch {
    NoMatch,
    SuperdomainMatch,
    CongruentMatch,
}

#[derive(Debug)]
pub struct HstsStore {
    table: HashMap<HstsKh, HstsKhInfo>,
    last_mtime: i64,
    changed: bool,
}

impl HstsKh {
    fn new(host: String, explicit_port: i32) -> Self {
        HstsKh { host, explicit_port }
    }
}

impl PartialEq for HstsKh {
    fn eq(&self, other: &Self) -> bool {
        self.host == other.host && self.explicit_port == other.explicit_port
    }
}

impl Eq for HstsKh {}

impl std::hash::Hash for HstsKh {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.host.hash(state);
        self.explicit_port.hash(state);
    }
}

impl HstsStore {
    pub fn open(filename: &str) -> io::Result<Self> {
        let mut store = HstsStore {
            table: HashMap::new(),
            last_mtime: 0,
            changed: false,
        };

        if Path::new(filename).exists() {
            if !is_file_access_valid(filename)? {
                return Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "HSTS database must be a regular and non-world-writable file",
                ));
            }

            let file = File::open(filename)?;
            let metadata = file.metadata()?;
            store.last_mtime = metadata.modified()?
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let reader = BufReader::new(file);
            if let Err(e) = store.read_database(reader, false) {
                return Err(e);
            }
        }

        Ok(store)
    }

    pub fn save(&mut self, filename: &str) -> io::Result<()> {
        if self.table.is_empty() {
            return Ok(());
        }

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o644)
            .open(filename)?;

        unsafe {
            flock(file.as_raw_fd(), LOCK_EX);
        }

        if self.last_mtime != 0 {
            if let Ok(metadata) = file.metadata() {
                let mtime = metadata.modified()?
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                if mtime > self.last_mtime {
                    let reader = BufReader::new(&file);
                    self.read_database(reader, true)?;
                }
            }
        }

        file.set_len(0)?;
        file.seek(io::SeekFrom::Start(0))?;

        self.dump(&mut file)?;
        Ok(())
    }

    pub fn has_changed(&self) -> bool {
        self.changed
    }

    pub fn match_url(&mut self, url: &mut Url) -> bool {
        if !is_scheme_valid(url.scheme()) {
            return false;
        }

        let port = make_explicit_port(url.scheme(), url.port().unwrap_or(0));
        let host = url.host_str().unwrap_or("");
        let mut match_type = HstsKhMatch::NoMatch;
        let kh = HstsKh::new(host.to_string(), port);

        if let Some(entry) = self.find_entry(host, port, &mut match_type, None) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            if entry.created + entry.max_age >= now {
                if match_type == HstsKhMatch::CongruentMatch
                    || (match_type == HstsKhMatch::SuperdomainMatch && entry.include_subdomains)
                {
                    url.set_scheme("https").unwrap();
                    if url.port() == Some(80) {
                        url.set_port(Some(443)).unwrap();
                    }
                    self.changed = true;
                    return true;
                }
            } else {
                self.table.remove(&kh);
                self.changed = true;
            }
        }

        false
    }

    pub fn store_entry(
        &mut self,
        scheme: &str,
        host: &str,
        port: i32,
        max_age: i64,
        include_subdomains: bool,
    ) -> bool {
        if !is_host_eligible(scheme, host) {
            return false;
        }

        let port = make_explicit_port(scheme, port);
        let mut match_type = HstsKhMatch::NoMatch;
        let kh = HstsKh::new(host.to_string(), port);

        if let Some(entry) = self.find_entry(host, port, &mut match_type, None) {
            if match_type == HstsKhMatch::CongruentMatch {
                if max_age == 0 {
                    self.table.remove(&kh);
                    self.changed = true;
                    return false;
                } else if max_age > 0 {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64;

                    if now != entry.created {
                        let new_entry = HstsKhInfo {
                            created: now,
                            max_age,
                            include_subdomains,
                        };
                        self.table.insert(kh, new_entry);
                        self.changed = true;
                    }
                }
                return false;
            }
        }

        if max_age > 0 {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            let new_entry = HstsKhInfo {
                created: now,
                max_age,
                include_subdomains,
            };
            self.table.insert(kh, new_entry);
            self.changed = true;
            true
        } else {
            false
        }
    }

    fn find_entry(
        &self,
        host: &str,
        explicit_port: i32,
        match_type: &mut HstsKhMatch,
        kh_out: Option<&mut HstsKh>,
    ) -> Option<&HstsKhInfo> {
        let mut k = HstsKh::new(host.to_lowercase(), explicit_port);
        let original_host = k.host.clone();

        if let Some(entry) = self.table.get(&k) {
            *match_type = HstsKhMatch::CongruentMatch;
            if let Some(kh) = kh_out {
                *kh = k.clone();
            }
            return Some(entry);
        }

        let mut pos = k.host.find('.');
        while *match_type == HstsKhMatch::NoMatch && pos.is_some() && pos.unwrap() > 0 {
            k.host = k.host[pos.unwrap() + 1..].to_string();
            if let Some(entry) = self.table.get(&k) {
                *match_type = HstsKhMatch::SuperdomainMatch;
                if let Some(kh) = kh_out {
                    kh.host = original_host.clone();
                    kh.explicit_port = explicit_port;
                }
                return Some(entry);
            }
            pos = k.host.find('.');
        }

        None
    }

    fn read_database<R: io::Read>(&mut self, reader: BufReader<R>, merge: bool) -> io::Result<()> {
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 5 {
                let host = parts[0].to_string();
                let port = parts[1].parse().unwrap_or(0);
                let include_subdomains = parts[2].parse().unwrap_or(0) != 0;
                let created = parts[3].parse().unwrap_or(0);
                let max_age = parts[4].parse().unwrap_or(0);

                if merge {
                    self.merge_entry(&host, port, created, max_age, include_subdomains);
                } else {
                    self.new_entry(&host, port, created, max_age, include_subdomains);
                }
            }
        }
        Ok(())
    }

    fn dump<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "# HSTS 1.0 Known Hosts database for GNU Wget.")?;
        writeln!(writer, "# Edit at your own risk.")?;
        writeln!(
            writer,
            "# <hostname>\t<port>\t<incl. subdomains>\t<created>\t<max-age>"
        )?;

        for (kh, khi) in &self.table {
            writeln!(
                writer,
                "{}\t{}\t{}\t{}\t{}",
                kh.host, kh.explicit_port, khi.include_subdomains as i32, khi.created, khi.max_age
            )?;
        }
        Ok(())
    }

    fn new_entry(
        &mut self,
        host: &str,
        port: i32,
        created: i64,
        max_age: i64,
        include_subdomains: bool,
    ) -> bool {
        if !is_host_name_valid(host) || (created + max_age) < created {
            return false;
        }

        let kh = HstsKh::new(host.to_lowercase(), port);
        if self.table.contains_key(&kh) {
            return false;
        }

        self.table.insert(
            kh,
            HstsKhInfo {
                created,
                max_age,
                include_subdomains,
            },
        );
        true
    }

    fn merge_entry(
        &mut self,
        host: &str,
        port: i32,
        created: i64,
        max_age: i64,
        include_subdomains: bool,
    ) -> bool {
        let mut match_type = HstsKhMatch::NoMatch;
        let kh = HstsKh::new(host.to_string(), port);

        if let Some(entry) = self.find_entry(host, port, &mut match_type, None) {
            if match_type == HstsKhMatch::CongruentMatch && created > entry.created {
                let new_entry = HstsKhInfo {
                    created,
                    max_age,
                    include_subdomains,
                };
                self.table.insert(kh, new_entry);
                return true;
            }
            false
        } else {
            self.new_entry(host, port, created, max_age, include_subdomains)
        }
    }
}

fn is_file_access_valid(filename: &str) -> io::Result<bool> {
    let metadata = std::fs::metadata(filename)?;
    Ok(metadata.is_file() && metadata.permissions().readonly())
}

fn is_scheme_valid(scheme: &str) -> bool {
    scheme == "https"
}

fn is_host_name_valid(host: &str) -> bool {
    !host.parse::<std::net::IpAddr>().is_ok()
}

fn is_host_eligible(scheme: &str, host: &str) -> bool {
    is_scheme_valid(scheme) && is_host_name_valid(host)
}

fn make_explicit_port(scheme: &str, port: i32) -> i32 {
    match scheme {
        "https" => if port == 443 { 0 } else { port },
        _ => if port == 80 { 0 } else { port },
    }
}