use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::os::unix::fs::OpenOptionsExt;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UrlScheme {
    Http,
    Https,
    Ftp,
    Ftps,
    Invalid,
}

#[derive(Debug, Clone)]
pub struct Url {
    pub url: String,
    pub scheme: UrlScheme,
    pub host: String,
    pub port: u16,
    pub path: String,
    pub params: String,
    pub query: String,
    pub fragment: String,
    pub dir: String,
    pub file: String,
    pub user: String,
    pub passwd: String,
}

#[derive(Debug, Clone)]
struct HstsKey {
    host: String,
    explicit_port: u16,
}

#[derive(Debug, Clone)]
struct HstsInfo {
    created: i64,
    max_age: i64,
    include_subdomains: bool,
}

#[derive(Debug, Clone)]
pub struct HstsStore {
    table: HashMap<HstsKey, HstsInfo>,
    last_mtime: i64,
    changed: bool,
}

#[derive(Debug, PartialEq, Eq)]
enum HstsMatchType {
    NoMatch,
    SuperdomainMatch,
    CongruentMatch,
}

impl HstsStore {
    pub fn new() -> Self {
        HstsStore {
            table: HashMap::new(),
            last_mtime: 0,
            changed: false,
        }
    }

    fn hash_key(key: &HstsKey) -> u64 {
        let mut hash = key.explicit_port as u64;
        for c in key.host.chars() {
            hash = hash.wrapping_mul(31).wrapping_add(c as u64);
        }
        hash
    }

    fn find_entry(
        &self,
        host: &str,
        explicit_port: u16,
        match_type: &mut HstsMatchType,
    ) -> Option<(&HstsKey, &HstsInfo)> {
        let search_key = HstsKey {
            host: host.to_lowercase(),
            explicit_port,
        };

        if let Some((k, v)) = self.table.get_key_value(&search_key) {
            *match_type = HstsMatchType::CongruentMatch;
            return Some((k, v));
        }

        let mut current_host = search_key.host.clone();
        while let Some(dot_pos) = current_host.find('.') {
            if dot_pos == 0 || current_host[dot_pos + 1..].find('.').is_none() {
                break;
            }

            current_host = current_host[dot_pos + 1..].to_string();
            let super_key = HstsKey {
                host: current_host.clone(),
                explicit_port,
            };

            if let Some((k, v)) = self.table.get_key_value(&super_key) {
                *match_type = HstsMatchType::SuperdomainMatch;
                return Some((k, v));
            }
        }

        None
    }

    fn new_entry_internal(
        &mut self,
        host: &str,
        port: u16,
        created: i64,
        max_age: i64,
        include_subdomains: bool,
        check_validity: bool,
        check_expired: bool,
        check_duplicates: bool,
    ) -> bool {
        if check_validity && is_valid_ip_address(host) {
            return false;
        }

        if check_expired && created + max_age < created {
            return false;
        }

        let explicit_port = match port {
            443 => 0,
            80 => 0,
            _ => port,
        };

        let key = HstsKey {
            host: host.to_lowercase(),
            explicit_port,
        };

        if check_duplicates && self.table.contains_key(&key) {
            return false;
        }

        let info = HstsInfo {
            created,
            max_age,
            include_subdomains,
        };

        self.table.insert(key, info);
        self.changed = true;
        true
    }

    pub fn add_entry(
        &mut self,
        host: &str,
        port: u16,
        max_age: i64,
        include_subdomains: bool,
    ) -> bool {
        let created = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        self.new_entry_internal(
            host,
            port,
            created,
            max_age,
            include_subdomains,
            false,
            true,
            false,
        )
    }

    pub fn new_entry(
        &mut self,
        host: &str,
        port: u16,
        created: i64,
        max_age: i64,
        include_subdomains: bool,
    ) -> bool {
        self.new_entry_internal(
            host,
            port,
            created,
            max_age,
            include_subdomains,
            true,
            true,
            true,
        )
    }

    fn remove_entry(&mut self, key: &HstsKey) {
        self.table.remove(key);
        self.changed = true;
    }

    pub fn store_merge(
        &mut self,
        host: &str,
        port: u16,
        created: i64,
        max_age: i64,
        include_subdomains: bool,
    ) -> bool {
        let explicit_port = match port {
            443 => 0,
            80 => 0,
            _ => port,
        };

        let mut match_type = HstsMatchType::NoMatch;
        if let Some((k, v)) = self.find_entry(host, explicit_port, &mut match_type) {
            if match_type == HstsMatchType::CongruentMatch && created > v.created {
                let new_info = HstsInfo {
                    created,
                    max_age,
                    include_subdomains,
                };
                self.table.insert(k.clone(), new_info);
                self.changed = true;
                return true;
            }
        } else {
            return self.new_entry(host, port, created, max_age, include_subdomains);
        }

        false
    }

    pub fn read_database(&mut self, path: &Path, merge: bool) -> bool {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(_) => return false,
        };

        let reader = BufReader::new(file);
        let func = if merge {
            HstsStore::store_merge
        } else {
            HstsStore::new_entry
        };

        for line in reader.lines().filter_map(|l| l.ok()) {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 5 {
                continue;
            }

            let host = parts[0];
            let port = parts[1].parse().unwrap_or(0);
            let include_subdomains = parts[2].parse().unwrap_or(0) != 0;
            let created = parts[3].parse().unwrap_or(0);
            let max_age = parts[4].parse().unwrap_or(0);

            func(self, host, port, created, max_age, include_subdomains);
        }

        true
    }

    pub fn store_dump(&self, path: &Path) -> std::io::Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(path)?;

        writeln!(file, "# HSTS 1.0 Known Hosts database for GNU Wget.")?;
        writeln!(file, "# Edit at your own risk.")?;
        writeln!(
            file,
            "# <hostname>\t<port>\t<incl. subdomains>\t<created>\t<max-age>"
        )?;

        for (key, info) in &self.table {
            writeln!(
                file,
                "{}\t{}\t{}\t{}\t{}",
                key.host,
                key.explicit_port,
                info.include_subdomains as i32,
                info.created,
                info.max_age
            )?;
        }

        Ok(())
    }

    pub fn match_url(&mut self, url: &mut Url) -> bool {
        if url.scheme == UrlScheme::Https {
            return false;
        }

        let port = match url.port {
            80 => 0,
            _ => url.port,
        };

        let mut match_type = HstsMatchType::NoMatch;
        if let Some((key, info)) = self.find_entry(&url.host, port, &mut match_type) {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as i64;

            if info.created + info.max_age >= now {
                if match_type == HstsMatchType::CongruentMatch
                    || (match_type == HstsMatchType::SuperdomainMatch
                        && info.include_subdomains)
                {
                    url.scheme = UrlScheme::Https;
                    if url.port == 80 {
                        url.port = 443;
                    }
                    self.changed = true;
                    return true;
                }
            } else {
                self.remove_entry(key);
                self.changed = true;
            }
        }

        false
    }

    pub fn store_entry(
        &mut self,
        scheme: UrlScheme,
        host: &str,
        port: u16,
        max_age: i64,
        include_subdomains: bool,
    ) -> bool {
        if scheme != UrlScheme::Https || is_valid_ip_address(host) {
            return false;
        }

        let explicit_port = match port {
            443 => 0,
            80 => 0,
            _ => port,
        };

        let mut match_type = HstsMatchType::NoMatch;
        let mut result = false;

        if let Some((key, info)) = self.find_entry(host, explicit_port, &mut match_type) {
            if match_type == HstsMatchType::CongruentMatch {
                if max_age == 0 {
                    self.remove_entry(key);
                    self.changed = true;
                } else if max_age > 0 {
                    let now = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs() as i64;

                    if now != info.created {
                        let new_info = HstsInfo {
                            created: now,
                            max_age,
                            include_subdomains,
                        };
                        self.table.insert(key.clone(), new_info);
                        self.changed = true;
                    }
                }
            }
        } else if match_type == HstsMatchType::NoMatch
            || match_type == HstsMatchType::SuperdomainMatch
        {
            result = self.add_entry(host, port, max_age, include_subdomains);
            if result {
                self.changed = true;
            }
        }

        result
    }

    pub fn open(path: &Path) -> Option<Self> {
        let mut store = HstsStore::new();

        if !path.exists() {
            return Some(store);
        }

        if !is_file_access_valid(path) {
            eprintln!("Will not apply HSTS. The HSTS database must be a regular and non-world-writable file.");
            return None;
        }

        if let Ok(metadata) = path.metadata() {
            store.last_mtime = metadata.modified().ok()?.duration_since(UNIX_EPOCH).ok()?.as_secs() as i64;
        }

        if !store.read_database(path, false) {
            return None;
        }

        Some(store)
    }

    pub fn save(&mut self, path: &Path) -> std::io::Result<()> {
        if self.table.is_empty() {
            return Ok(());
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .mode(0o600)
            .open(path)?;

        if self.last_mtime != 0 {
            if let Ok(metadata) = path.metadata() {
                let mtime = metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs() as i64;
                if mtime > self.last_mtime {
                    let mut temp_store = HstsStore::new();
                    temp_store.read_database(path, true);
                    self.table.extend(temp_store.table);
                }
            }
        }

        self.store_dump(path)?;
        self.changed = false;
        Ok(())
    }

    pub fn has_changed(&self) -> bool {
        self.changed
    }
}

fn is_valid_ip_address(host: &str) -> bool {
    host.parse::<std::net::IpAddr>().is_ok()
}

fn is_file_access_valid(path: &Path) -> bool {
    let metadata = match path.metadata() {
        Ok(m) => m,
        Err(_) => return false,
    };

    let mode = metadata.permissions().mode();
    mode & 0o002 == 0 && metadata.is_file()
}