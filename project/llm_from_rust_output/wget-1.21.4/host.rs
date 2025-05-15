use std::ffi::{CStr, CString};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::ptr;
use std::os::raw::{c_char, c_int, c_void};
use std::mem;
use std::time::Duration;
use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;
use libc::{addrinfo, sockaddr, sockaddr_in, sockaddr_in6, AF_INET, AF_INET6, SOCK_STREAM};
use crate::options::{Options, PreferFamily};

#[derive(Debug, Clone)]
pub struct IpAddress {
    family: c_int,
    data: IpAddr,
    ipv6_scope: c_int,
}

#[derive(Debug, Clone)]
pub struct AddressList {
    addresses: Vec<IpAddress>,
    faulty: c_int,
    connected: bool,
    refcount: c_int,
}

lazy_static! {
    static ref HOST_NAME_ADDRESSES_MAP: Mutex<HashMap<String, AddressList>> = Mutex::new(HashMap::new());
}

impl AddressList {
    pub fn get_bounds(&self) -> (c_int, c_int) {
        (self.faulty, self.addresses.len() as c_int)
    }

    pub fn address_at(&self, pos: c_int) -> Option<&IpAddress> {
        self.addresses.get(pos as usize)
    }

    pub fn contains(&self, ip: &IpAddress) -> bool {
        self.addresses.iter().any(|addr| addr == ip)
    }

    pub fn set_faulty(&mut self, index: c_int) {
        if index != self.faulty {
            log::error!("Error in handling the address list. Please report this issue.");
            panic!("Invalid faulty index");
        }
        self.faulty += 1;
        if self.faulty >= self.addresses.len() as c_int {
            self.faulty = 0;
        }
    }

    pub fn set_connected(&mut self) {
        self.connected = true;
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }

    pub fn release(&mut self) {
        self.refcount -= 1;
        if self.refcount <= 0 {
            if Options::global().debug {
                log::debug!("Deleting unused address list");
            }
        }
    }
}

pub fn lookup_host(host: &str, flags: c_int) -> Option<AddressList> {
    let silent = flags & LH_SILENT != 0;
    let use_cache = Options::global().dns_cache && (flags & LH_BIND == 0) && !is_valid_ip_address(host);
    
    if use_cache && (flags & LH_REFRESH == 0) {
        if let Some(al) = cache_query(host) {
            return Some(al);
        }
    } else if use_cache {
        cache_remove(host);
    }

    if !silent && !is_valid_ip_address(host) {
        log::info!("Resolving {}...", host);
    }

    let hints = create_hints(flags);
    let timeout = if is_valid_ip_address(host) {
        Duration::from_secs(0)
    } else {
        Duration::from_secs_f64(Options::global().dns_timeout)
    };

    match getaddrinfo_with_timeout(host, None, &hints, timeout) {
        Ok(res) => {
            let mut al = address_list_from_addrinfo(&res);
            if al.addresses.is_empty() {
                log::error!("No IPv4/IPv6 addresses for host");
                return None;
            }

            if al.addresses.len() > 1 {
                sort_addresses(&mut al.addresses);
            }

            if !silent && !is_valid_ip_address(host) {
                log_addresses(&al);
            }

            if use_cache {
                cache_store(host, &al);
            }

            Some(al)
        }
        Err(e) => {
            if !silent {
                log::error!("Failed to resolve host: {}", e);
            }
            None
        }
    }
}

fn create_hints(flags: c_int) -> addrinfo {
    let mut hints = unsafe { mem::zeroed::<addrinfo>() };
    hints.ai_socktype = SOCK_STREAM;
    hints.ai_family = if Options::global().ipv4_only {
        AF_INET
    } else if Options::global().ipv6_only {
        AF_INET6
    } else {
        0
    };

    if flags & LH_BIND != 0 {
        hints.ai_flags |= libc::AI_PASSIVE;
    }

    if is_valid_ip_address(host) {
        hints.ai_flags |= libc::AI_NUMERICHOST;
    }

    hints
}

fn sort_addresses(addresses: &mut [IpAddress]) {
    match Options::global().prefer_family {
        PreferFamily::Ipv4 => {
            addresses.sort_by(|a, b| {
                (a.family != AF_INET).cmp(&(b.family != AF_INET))
            });
        }
        PreferFamily::Ipv6 => {
            addresses.sort_by(|a, b| {
                (a.family != AF_INET6).cmp(&(b.family != AF_INET6))
            });
        }
        PreferFamily::None => {}
    }
}

fn log_addresses(al: &AddressList) {
    let print_max = if Options::global().show_all_dns_entries {
        al.addresses.len()
    } else {
        std::cmp::min(3, al.addresses.len())
    };

    let addresses_str: Vec<String> = al.addresses[..print_max]
        .iter()
        .map(|addr| addr.to_string())
        .collect();

    log::info!("{}", addresses_str.join(", "));
    if print_max < al.addresses.len() {
        log::info!(", ...");
    }
}

fn cache_query(host: &str) -> Option<AddressList> {
    HOST_NAME_ADDRESSES_MAP.lock().unwrap().get(host).cloned()
}

fn cache_store(host: &str, al: &AddressList) {
    let mut map = HOST_NAME_ADDRESSES_MAP.lock().unwrap();
    let mut al = al.clone();
    al.refcount += 1;
    map.insert(host.to_lowercase(), al);
}

fn cache_remove(host: &str) {
    HOST_NAME_ADDRESSES_MAP.lock().unwrap().remove(&host.to_lowercase());
}

pub fn is_valid_ip_address(name: &str) -> bool {
    name.parse::<IpAddr>().is_ok()
}

const LH_SILENT: c_int = 1;
const LH_BIND: c_int = 2;
const LH_REFRESH: c_int = 4;