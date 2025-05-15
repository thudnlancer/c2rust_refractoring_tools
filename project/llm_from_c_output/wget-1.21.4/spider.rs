//! Spider module for tracking visited URLs in spider mode.
//!
//! This module provides functionality to track and report broken links
//! encountered during spidering operations.

use std::collections::HashSet;
use std::ffi::CStr;
use std::os::raw::c_char;

use crate::url::is_robots_txt_url;
use crate::utils::logprintf;
use crate::res::{LOG_NOTQUIET, _};

static mut NONEXISTING_URLS_SET: Option<HashSet<String>> = None;

/// Cleanup the data structures associated with this module.
#[cfg(any(debug_assertions, test))]
pub fn spider_cleanup() {
    unsafe {
        NONEXISTING_URLS_SET = None;
    }
}

/// Remember broken links.
pub fn nonexisting_url(url: &str) {
    // Ignore robots.txt URLs
    if is_robots_txt_url(url) {
        return;
    }

    unsafe {
        if NONEXISTING_URLS_SET.is_none() {
            NONEXISTING_URLS_SET = Some(HashSet::new());
        }
        
        if let Some(set) = &mut NONEXISTING_URLS_SET {
            set.insert(url.to_string());
        }
    }
}

/// Print all broken links found during spidering.
pub fn print_broken_links() {
    unsafe {
        if NONEXISTING_URLS_SET.is_none() || NONEXISTING_URLS_SET.as_ref().unwrap().is_empty() {
            logprintf(LOG_NOTQUIET, "Found no broken links.\n\n");
            return;
        }

        let set = NONEXISTING_URLS_SET.as_ref().unwrap();
        let num_elems = set.len();

        logprintf(
            LOG_NOTQUIET,
            if num_elems == 1 {
                "Found {} broken link.\n\n"
            } else {
                "Found {} broken links.\n\n"
            },
            num_elems,
        );

        for url in set {
            logprintf(LOG_NOTQUIET, "{}\n", url);
        }
        
        logprintf(LOG_NOTQUIET, "\n");
    }
}