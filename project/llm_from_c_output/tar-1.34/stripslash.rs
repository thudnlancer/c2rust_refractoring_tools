/// Remove trailing slashes from a file path. Returns true if a trailing slash
/// was removed. This is useful when using file name completion from a
/// shell that adds a "/" after directory names (such as tcsh and bash),
/// because on symlinks to directories, several system calls have different
/// semantics according to whether a trailing slash is present.
///
/// # Arguments
/// * `file` - A mutable reference to a String containing the file path
///
/// # Returns
/// * `bool` - True if trailing slashes were removed, false otherwise
pub fn strip_trailing_slashes(file: &mut String) -> bool {
    // Get the last component of the path
    let last_component = file.rsplit('/').next().unwrap_or("");
    
    // Handle filesystem roots (like "///" which should become "/")
    let base = if last_component.is_empty() {
        file.as_str()
    } else {
        last_component
    };
    
    // Calculate the position where the base ends
    let base_len = base.len();
    let base_end = file.len() - (last_component.len() - base_len);
    
    // Check if there were trailing slashes
    let had_slash = file.len() > base_end;
    
    // Truncate the string at the end of the base component
    file.truncate(base_end);
    
    had_slash
}