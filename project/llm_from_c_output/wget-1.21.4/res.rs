use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::ptr;
use std::str;

#[derive(Debug)]
struct PathInfo {
    path: String,
    allowed: bool,
    user_agent_exact: bool,
}

#[derive(Debug)]
struct RobotSpecs {
    paths: Vec<PathInfo>,
}

impl RobotSpecs {
    fn new() -> Self {
        RobotSpecs { paths: Vec::new() }
    }

    fn add_path(&mut self, path: &str, allowed: bool, exact: bool) {
        let path = if path.starts_with('/') {
            &path[1..]
        } else {
            path
        };
        self.paths.push(PathInfo {
            path: path.to_string(),
            allowed,
            user_agent_exact: exact,
        });
    }

    fn prune_non_exact(&mut self) {
        self.paths.retain(|p| p.user_agent_exact);
    }
}

fn match_user_agent(agent: &str) -> (bool, bool) {
    if agent == "*" {
        (true, false)
    } else if agent.eq_ignore_ascii_case("wget") {
        (true, true)
    } else {
        (false, false)
    }
}

fn res_parse(source: &str) -> RobotSpecs {
    let mut specs = RobotSpecs::new();
    let mut user_agent_applies = false;
    let mut user_agent_exact = false;
    let mut found_exact = false;
    let mut record_count = 0;

    for line in source.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let mut parts = line.splitn(2, ':');
        let field = parts.next().unwrap().trim();
        let value = parts.next().unwrap_or("").trim();

        match field.to_lowercase().as_str() {
            "user-agent" => {
                let (applies, exact) = match_user_agent(value);
                if record_count != 0 || !user_agent_applies {
                    user_agent_applies = applies;
                    user_agent_exact = exact;
                }
                if exact {
                    found_exact = true;
                }
                record_count = 0;
            }
            "allow" => {
                if user_agent_applies {
                    specs.add_path(value, true, user_agent_exact);
                }
                record_count += 1;
            }
            "disallow" => {
                if user_agent_applies {
                    let allowed = value.is_empty();
                    specs.add_path(value, allowed, user_agent_exact);
                }
                record_count += 1;
            }
            _ => continue,
        }
    }

    if found_exact {
        specs.prune_non_exact();
    }

    specs
}

fn res_parse_from_file(filename: &str) -> Option<RobotSpecs> {
    let mut file = File::open(filename).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;
    Some(res_parse(&contents))
}

fn matches(record_path: &str, url_path: &str) -> bool {
    let mut rp = record_path.chars();
    let mut up = url_path.chars();

    loop {
        match (rp.next(), up.next()) {
            (None, _) => return true,
            (_, None) => return false,
            (Some(rc), Some(uc)) if rc == uc => continue,
            _ => return false,
        }
    }
}

fn res_match_path(specs: &RobotSpecs, path: &str) -> bool {
    for p in &specs.paths {
        if matches(&p.path, path) {
            return p.allowed;
        }
    }
    true
}

struct ResRegistry {
    specs: HashMap<String, RobotSpecs>,
}

impl ResRegistry {
    fn new() -> Self {
        ResRegistry {
            specs: HashMap::new(),
        }
    }

    fn register_specs(&mut self, host: &str, port: u16, specs: RobotSpecs) {
        let key = format!("{}:{}", host, port);
        self.specs.insert(key, specs);
    }

    fn get_specs(&self, host: &str, port: u16) -> Option<&RobotSpecs> {
        let key = format!("{}:{}", host, port);
        self.specs.get(&key)
    }
}

fn res_retrieve_file(url: &str) -> Option<String> {
    let robots_url = format!("{}/robots.txt", url.trim_end_matches('/'));
    // In a real implementation, we'd use a proper HTTP client here
    // This is simplified for demonstration
    Some(robots_url)
}

fn is_robots_txt_url(url: &str) -> bool {
    url.ends_with("/robots.txt")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_robots_txt_url() {
        assert!(is_robots_txt_url("http://example.com/robots.txt"));
        assert!(!is_robots_txt_url("http://example.com/"));
        assert!(!is_robots_txt_url("http://example.com/path/robots.txt"));
    }

    #[test]
    fn test_res_parse() {
        let input = r#"
User-Agent: *
Disallow: /private/

User-Agent: wget
Allow: /public/
"#;
        let specs = res_parse(input);
        assert_eq!(specs.paths.len(), 1);
        assert!(specs.paths[0].allowed);
        assert_eq!(specs.paths[0].path, "public/");
    }

    #[test]
    fn test_res_match_path() {
        let mut specs = RobotSpecs::new();
        specs.add_path("/private/", false, false);
        specs.add_path("/public/", true, true);

        assert!(!res_match_path(&specs, "/private/file"));
        assert!(res_match_path(&specs, "/public/file"));
        assert!(res_match_path(&specs, "/other/path"));
    }
}