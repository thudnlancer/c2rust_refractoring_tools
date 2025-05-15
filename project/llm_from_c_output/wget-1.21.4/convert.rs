use std::collections::HashMap;
use std::ffi::CString;
use std::fs::{File, rename};
use std::io::{self, Write, Read};
use std::path::{Path, PathBuf};
use std::ptr;
use std::str;
use std::sync::Once;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConvertOptions {
    NoConvert,
    ConvertToRelative,
    ConvertBasenameOnly,
    ConvertToComplete,
    NullifyBase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadedFileType {
    NotAlreadyDownloaded,
    DownloadedNormally,
    DownloadedAndHtmlExtensionAdded,
    CheckForFile,
}

pub struct UrlPos {
    pub url: String,
    pub local_name: Option<String>,
    pub ignore_when_downloading: bool,
    pub link_relative_p: bool,
    pub link_complete_p: bool,
    pub link_base_p: bool,
    pub link_inline_p: bool,
    pub link_css_p: bool,
    pub link_noquote_html_p: bool,
    pub link_expect_html: bool,
    pub link_expect_css: bool,
    pub link_refresh_p: bool,
    pub refresh_timeout: i32,
    pub convert: ConvertOptions,
    pub pos: i32,
    pub size: i32,
    pub next: Option<Box<UrlPos>>,
}

pub struct Converter {
    dl_file_url_map: HashMap<String, String>,
    dl_url_file_map: HashMap<String, String>,
    downloaded_html_set: HashMap<String, ()>,
    downloaded_css_set: HashMap<String, ()>,
    converted_files: HashMap<String, ()>,
    downloaded_files_hash: HashMap<String, DownloadedFileType>,
}

impl Converter {
    pub fn new() -> Self {
        Self {
            dl_file_url_map: HashMap::new(),
            dl_url_file_map: HashMap::new(),
            downloaded_html_set: HashMap::new(),
            downloaded_css_set: HashMap::new(),
            converted_files: HashMap::new(),
            downloaded_files_hash: HashMap::new(),
        }
    }

    pub fn convert_links_in_hashtable(
        &mut self,
        downloaded_set: &HashMap<String, ()>,
        is_css: bool,
        file_count: &mut i32,
    ) {
        if downloaded_set.is_empty() {
            return;
        }

        for file in downloaded_set.keys() {
            let url = match self.dl_file_url_map.get(file) {
                Some(url) => url,
                None => {
                    log::debug!("Apparently {} has been removed.", file);
                    continue;
                }
            };

            log::debug!("Scanning {} (from {})", file, url);

            // Parse the file...
            let urls = if is_css {
                Self::get_urls_css_file(file, url)
            } else {
                Self::get_urls_html(file, url, None, None)
            };

            for cur_url in urls.iter() {
                if cur_url.link_base_p {
                    cur_url.convert = ConvertOptions::NullifyBase;
                    continue;
                }

                let local_name = self.dl_url_file_map.get(&cur_url.url).cloned();

                match local_name {
                    Some(local_name) => {
                        cur_url.convert = if opt.convert_file_only {
                            ConvertOptions::ConvertBasenameOnly
                        } else {
                            ConvertOptions::ConvertToRelative
                        };
                        cur_url.local_name = Some(local_name);
                        log::debug!(
                            "will convert url {} to local {}",
                            cur_url.url,
                            local_name
                        );
                    }
                    None => {
                        if !cur_url.link_complete_p {
                            cur_url.convert = ConvertOptions::ConvertToComplete;
                        }
                        cur_url.local_name = None;
                        log::debug!("will convert url {} to complete", cur_url.url);
                    }
                }
            }

            self.convert_links(file, &urls);
            *file_count += 1;
        }
    }

    pub fn convert_all_links(&mut self) {
        let timer = std::time::Instant::now();
        let mut file_count = 0;

        self.convert_links_in_hashtable(&self.downloaded_html_set, false, &mut file_count);
        self.convert_links_in_hashtable(&self.downloaded_css_set, true, &mut file_count);

        let secs = timer.elapsed().as_secs_f64();
        log::info!(
            "Converted links in {} files in {:.2} seconds.",
            file_count,
            secs
        );
    }

    pub fn convert_links(&mut self, file: &str, links: &[UrlPos]) {
        log::info!("Converting links in {}... ", file);

        let dry_count = links
            .iter()
            .filter(|link| link.convert != ConvertOptions::NoConvert)
            .count();

        if dry_count == 0 {
            log::info!("nothing to do.");
            return;
        }

        log::info!("{}", dry_count);

        let fm = match std::fs::read(file) {
            Ok(content) => content,
            Err(e) => {
                log::error!("Cannot convert links in {}: {}", file, e);
                return;
            }
        };

        let downloaded_file_return = self.downloaded_file(DownloadedFileType::CheckForFile, file);
        if opt.backup_converted && downloaded_file_return == DownloadedFileType::DownloadedNormally {
            self.write_backup_file(file, downloaded_file_return);
        }

        if let Err(e) = std::fs::remove_file(file) {
            if e.kind() != std::io::ErrorKind::NotFound {
                log::error!("Unable to delete {}: {}", file, e);
                return;
            }
        }

        let mut fp = match File::create(file) {
            Ok(file) => file,
            Err(e) => {
                log::error!("Cannot convert links in {}: {}", file, e);
                return;
            }
        };

        let mut p = fm.as_slice();
        let mut to_file_count = 0;
        let mut to_url_count = 0;

        for link in links {
            if link.pos >= fm.len() as i32 {
                log::debug!("Something strange is going on. Please investigate.");
                break;
            }

            if link.convert == ConvertOptions::NoConvert {
                log::debug!("Skipping {} at position {}.", link.url, link.pos);
                continue;
            }

            let url_start = link.pos as usize;
            fp.write_all(&p[..url_start]).unwrap();
            p = &p[url_start..];

            match link.convert {
                ConvertOptions::ConvertToRelative => {
                    if let Some(local_name) = &link.local_name {
                        let newname = self.construct_relative(file, local_name);
                        let quoted_newname =
                            self.local_quote_string(&newname, link.link_css_p);

                        if link.link_css_p || link.link_noquote_html_p {
                            p = self.replace_plain(p, link.size as usize, &mut fp, &quoted_newname);
                        } else if !link.link_refresh_p {
                            p = self.replace_attr(p, link.size as usize, &mut fp, &quoted_newname);
                        } else {
                            p = self.replace_attr_refresh_hack(
                                p,
                                link.size as usize,
                                &mut fp,
                                &quoted_newname,
                                link.refresh_timeout,
                            );
                        }

                        log::debug!(
                            "TO_RELATIVE: {} to {} at position {} in {}.",
                            link.url,
                            newname,
                            link.pos,
                            file
                        );

                        to_file_count += 1;
                    }
                }
                ConvertOptions::ConvertBasenameOnly => {
                    let newname = self.convert_basename(
                        unsafe { std::str::from_utf8_unchecked(p) },
                        link,
                    );
                    let quoted_newname = self.local_quote_string(&newname, link.link_css_p);

                    if link.link_css_p || link.link_noquote_html_p {
                        p = self.replace_plain(p, link.size as usize, &mut fp, &quoted_newname);
                    } else if !link.link_refresh_p {
                        p = self.replace_attr(p, link.size as usize, &mut fp, &quoted_newname);
                    } else {
                        p = self.replace_attr_refresh_hack(
                            p,
                            link.size as usize,
                            &mut fp,
                            &quoted_newname,
                            link.refresh_timeout,
                        );
                    }

                    log::debug!(
                        "Converted file part only: {} to {} at position {} in {}.",
                        link.url,
                        newname,
                        link.pos,
                        file
                    );

                    to_file_count += 1;
                }
                ConvertOptions::ConvertToComplete => {
                    let quoted_newlink = self.html_quote_string(&link.url);

                    if link.link_css_p || link.link_noquote_html_p {
                        p = self.replace_plain(p, link.size as usize, &mut fp, &link.url);
                    } else if !link.link_refresh_p {
                        p = self.replace_attr(p, link.size as usize, &mut fp, &quoted_newlink);
                    } else {
                        p = self.replace_attr_refresh_hack(
                            p,
                            link.size as usize,
                            &mut fp,
                            &quoted_newlink,
                            link.refresh_timeout,
                        );
                    }

                    log::debug!(
                        "TO_COMPLETE: <something> to {} at position {} in {}.",
                        link.url,
                        link.pos,
                        file
                    );

                    to_url_count += 1;
                }
                ConvertOptions::NullifyBase => {
                    p = self.replace_attr(p, link.size as usize, &mut fp, "");
                }
                ConvertOptions::NoConvert => unreachable!(),
            }
        }

        if !p.is_empty() {
            fp.write_all(p).unwrap();
        }

        log::info!("{}-{}", to_file_count, to_url_count);
    }

    fn construct_relative(&self, basefile: &str, linkfile: &str) -> String {
        let base = Path::new(basefile);
        let link = Path::new(linkfile);

        let mut base_components = base.components();
        let mut link_components = link.components();

        let mut common_prefix = 0;
        let mut base_iter = base.components();
        let mut link_iter = link.components();

        while let (Some(base_comp), Some(link_comp)) = (base_iter.next(), link_iter.next()) {
            if base_comp == link_comp {
                common_prefix += 1;
            } else {
                break;
            }
        }

        let base_depth = base.components().count() - common_prefix;
        let mut result = String::new();

        for _ in 0..base_depth {
            result.push_str("../");
        }

        for (i, comp) in link.components().enumerate() {
            if i >= common_prefix {
                result.push_str(&comp.as_os_str().to_string_lossy());
                if i < link.components().count() - 1 {
                    result.push('/');
                }
            }
        }

        result
    }

    fn convert_basename(&self, p: &str, link: &UrlPos) -> String {
        let len = link.size as usize;
        let url = if p.starts_with('"') || p.starts_with('\'') {
            &p[1..len - 1]
        } else {
            &p[..len]
        };

        let org_basename = url.rsplit('/').next().unwrap_or(url);
        let local_basename = link
            .local_name
            .as_ref()
            .and_then(|name| name.rsplit('/').next())
            .unwrap_or(url);

        if org_basename == local_basename {
            url.to_string()
        } else {
            let mut result = url.rsplitn(2, '/').nth(1).unwrap_or("").to_string();
            result.push('/');
            result.push_str(local_basename);
            result
        }
    }

    fn write_backup_file(&mut self, file: &str, downloaded_file_return: DownloadedFileType) {
        if self.converted_files.contains_key(file) {
            return;
        }

        let backup_file = if downloaded_file_return == DownloadedFileType::DownloadedAndHtmlExtensionAdded
        {
            let mut path = PathBuf::from(file);
            path.set_extension("orig");
            path.to_string_lossy().into_owned()
        } else {
            format!("{}.orig", file)
        };

        if let Err(e) = rename(file, &backup_file) {
            log::error!("Cannot back up {} as {}: {}", file, backup_file, e);
        }

        self.converted_files.insert(file.to_string(), ());
    }

    fn replace_plain(&self, p: &[u8], size: usize, fp: &mut File, new_text: &str) -> &[u8] {
        fp.write_all(new_text.as_bytes()).unwrap();
        &p[size..]
    }

    fn replace_attr(
        &self,
        p: &[u8],
        size: usize,
        fp: &mut File,
        new_text: &str,
    ) -> &[u8] {
        let quote_char = if p[0] == b'"' || p[0] == b'\'' {
            p[0] as char
        } else {
            '"'
        };

        fp.write_all(&[quote_char as u8]).unwrap();
        fp.write_all(new_text.as_bytes()).unwrap();

        if let Some((frag_beg, frag_end)) = self.find_fragment(p, size) {
            fp.write_all(&p[frag_beg..frag_end]).unwrap();
        }

        fp.write_all(&[quote_char as u8]).unwrap();

        if quote_char != '"' {
            &p[size + 2..]
        } else {
            &p[size..]
        }
    }

    fn replace_attr_refresh_hack(
        &self,
        p: &[u8],
        size: usize,
        fp: &mut File,
        new_text: &str,
        timeout: i32,
    ) -> &[u8] {
        let new_with_timeout = format!("{}; URL={}", timeout, new_text);
        self.replace_attr(p, size, fp, &new_with_timeout)
    }

    fn find_fragment(&self, beg: &[u8], size: usize) -> Option<(usize, usize)> {
        let end = size;
        let mut saw_amp = false;

        for i in 0..end {
            match beg[i] {
                b'&' => saw_amp = true,
                b'#' if !saw_amp => return Some((i, end)),
                _ => saw_amp = false,
            }
        }

        None
    }

    fn local_quote_string(&self, file: &str, no_html_quote: bool) -> String {
        if !file.contains(|c| c == '?' || c == '#' || c == '%' || c == ';') {
            return if no_html_quote {
                file.to_string()
            } else {
                self.html_quote_string(file)
            };
        }

        let mut result = String::new();
        for c in file.chars() {
            match c {
                '%' => result.push_str("%25"),
                '#' => result.push_str("%23"),
                ';' => result.push_str("%3B"),
                '?' if opt.adjust_extension => result.push_str("%3F"),
                _ => result.push(c),
            }
        }

        if no_html_quote {
            result
        } else {
            self.html_quote_string(&result)
        }
    }

    pub fn register_download(&mut self, url: &str, file: &str) {
        if let Some((old_file, old_url)) = self.dl_file_url_map.get_key_value(file) {
            if url == old_url {
                return;
            }

            if self.match_except_index(url, old_url) && !self.dl_url_file_map.contains_key(url) {
                self.dl_url_file_map.insert(url.to_string(), file.to_string());
                return;
            }

            self.dl_file_url_map.remove(file);
            self.dissociate_urls_from_file(file);
        }

        self.dl_file_url_map.insert(file.to_string(), url.to_string());

        if let Some(old_file) = self.dl_url_file_map.remove(url) {
            self.dl_url_file_map.insert(url.to_string(), file.to_string());
        } else {
            self.dl_url_file_map.insert(url.to_string(), file.to_string());
        }
    }

    pub fn register_redirection(&mut self, from: &str, to: &str) {
        if let Some(file) = self.dl_url_file_map.get(to) {
            if !self.dl_url_file_map.contains_key(from) {
                self.dl_url_file_map.insert(from.to_string(), file.clone());
            }
        }
    }

    pub fn register_delete_file(&mut self, file: &str) {
        if let Some(url) = self.dl_file_url_map.remove(file) {
            self.dissociate_urls_from_file(file);
        }
    }

    pub fn register_html(&mut self, file: &str) {
        self.downloaded_html_set.insert(file.to_string(), ());
    }

    pub fn register_css(&mut self, file: &str) {
        self.downloaded_css_set.insert(file.to_string(), ());
    }

    fn match_except_index(&self, s1: &str, s2: &str) -> bool {
        let mut i = 0;
        let (mut b, mut l) = (s1.chars(), s2.chars());

        while let (Some(bc), Some(lc)) = (b.next(), l.next()) {
            if bc != lc {
                break;
            }
            i += 1;
        }

        if i == 0 {
            return false;
        }

        let lng = if b.next().is_some() { s1 } else { s2 };
        let lng = &lng[i..];

        lng.is_empty()
            || lng == "/"
            || lng == "/index.html"
            || (lng.starts_with('/') && lng[1..].is_empty())
    }

    fn dissociate_urls_from_file(&mut self, file: &str) {
        self.dl_url_file_map
            .retain(|_, v| v != file);
    }

    pub fn downloaded_file(
        &mut self,
        mode: