use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressFamily {
    IPv4,
    IPv6,
}

#[derive(Debug, Clone)]
pub struct IpAddress {
    family: AddressFamily,
    addr: IpAddr,
    ipv6_scope: Option<u32>,
}

impl IpAddress {
    pub fn new_v4(addr: Ipv4Addr) -> Self {
        IpAddress {
            family: AddressFamily::IPv4,
            addr: IpAddr::V4(addr),
            ipv6_scope: None,
        }
    }

    pub fn new_v6(addr: Ipv6Addr, scope: Option<u32>) -> Self {
        IpAddress {
            family: AddressFamily::IPv6,
            addr: IpAddr::V6(addr),
            ipv6_scope: scope,
        }
    }

    pub fn family(&self) -> AddressFamily {
        self.family
    }

    pub fn as_v4(&self) -> Option<&Ipv4Addr> {
        match self.addr {
            IpAddr::V4(addr) => Some(&addr),
            _ => None,
        }
    }

    pub fn as_v6(&self) -> Option<&Ipv6Addr> {
        match self.addr {
            IpAddr::V6(addr) => Some(&addr),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AddressList {
    addresses: Vec<IpAddress>,
    faulty: usize,
    connected: bool,
}

impl AddressList {
    pub fn new(addresses: Vec<IpAddress>) -> Self {
        AddressList {
            addresses,
            faulty: 0,
            connected: false,
        }
    }

    pub fn get_bounds(&self) -> (usize, usize) {
        (self.faulty, self.addresses.len())
    }

    pub fn address_at(&self, pos: usize) -> Option<&IpAddress> {
        if pos >= self.faulty && pos < self.addresses.len() {
            Some(&self.addresses[pos])
        } else {
            None
        }
    }

    pub fn contains(&self, ip: &IpAddress) -> bool {
        self.addresses.iter().any(|addr| match (addr, ip) {
            (IpAddress { family: AddressFamily::IPv4, addr: IpAddr::V4(a), .. },
             IpAddress { family: AddressFamily::IPv4, addr: IpAddr::V4(b), .. }) => a == b,
            (IpAddress { family: AddressFamily::IPv6, addr: IpAddr::V6(a), scope: s1, .. },
             IpAddress { family: AddressFamily::IPv6, addr: IpAddr::V6(b), scope: s2, .. }) => {
                a == b && s1 == s2
            }
            _ => false,
        })
    }

    pub fn set_faulty(&mut self, index: usize) {
        assert_eq!(index, self.faulty);
        self.faulty += 1;
        if self.faulty >= self.addresses.len() {
            self.faulty = 0;
        }
    }

    pub fn set_connected(&mut self) {
        self.connected = true;
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

pub fn print_address(addr: &IpAddress) -> String {
    addr.addr.to_string()
}

pub fn is_valid_ipv4_address(s: &str) -> bool {
    Ipv4Addr::from_str(s).is_ok()
}

pub fn is_valid_ipv6_address(s: &str) -> bool {
    Ipv6Addr::from_str(s).is_ok()
}

pub fn is_valid_ip_address(s: &str) -> bool {
    is_valid_ipv4_address(s) || is_valid_ipv6_address(s)
}

#[derive(Debug, Clone)]
pub struct HostCache {
    map: HashMap<String, Arc<AddressList>>,
}

impl HostCache {
    pub fn new() -> Self {
        HostCache {
            map: HashMap::new(),
        }
    }

    pub fn query(&self, host: &str) -> Option<Arc<AddressList>> {
        self.map.get(host).cloned()
    }

    pub fn store(&mut self, host: String, al: AddressList) {
        self.map.insert(host, Arc::new(al));
    }

    pub fn remove(&mut self, host: &str) {
        self.map.remove(host);
    }
}

#[derive(Debug)]
pub enum LookupError {
    Timeout,
    NotFound,
    Other(String),
}

impl fmt::Display for LookupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LookupError::Timeout => write!(f, "DNS lookup timed out"),
            LookupError::NotFound => write!(f, "Host not found"),
            LookupError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl Error for LookupError {}

pub fn lookup_host(
    host: &str,
    prefer_ipv4: bool,
    timeout: Option<Duration>,
) -> Result<AddressList, LookupError> {
    if is_valid_ip_address(host) {
        let addr = if is_valid_ipv4_address(host) {
            IpAddress::new_v4(Ipv4Addr::from_str(host).unwrap())
        } else {
            IpAddress::new_v6(Ipv6Addr::from_str(host).unwrap(), None)
        };
        return Ok(AddressList::new(vec![addr]));
    }

    // TODO: Implement actual DNS lookup
    Err(LookupError::NotFound)
}

pub fn accept_domain(url_host: &str, allowed_domains: &[String], excluded_domains: &[String]) -> bool {
    if !allowed_domains.is_empty() && !sufmatch(allowed_domains, url_host) {
        return false;
    }
    if !excluded_domains.is_empty() && sufmatch(excluded_domains, url_host) {
        return false;
    }
    true
}

pub fn sufmatch(patterns: &[String], s: &str) -> bool {
    let s_lower = s.to_lowercase();
    let s_len = s_lower.len();

    for pattern in patterns {
        let pattern_lower = pattern.to_lowercase();
        let pattern_len = pattern_lower.len();

        if s_len < pattern_len {
            continue;
        }

        if pattern_lower == s_lower {
            return true;
        }

        if s_lower.ends_with(&pattern_lower) {
            let pos = s_len - pattern_len;
            if pos == 0 || s_lower.as_bytes()[pos - 1] == b'.' {
                return true;
            }
        }

        if pattern_lower.starts_with('.') {
            let pattern_without_dot = &pattern_lower[1..];
            if s_lower.ends_with(pattern_without_dot) {
                let pos = s_len - pattern_without_dot.len();
                if pos == 0 || s_lower.as_bytes()[pos - 1] == b'.' {
                    return true;
                }
            }
        }
    }

    false
}