use std::fmt::Write;

pub fn imaxtostr(value: i64) -> Result<String, std::fmt::Error> {
    let mut buffer = String::new();
    write!(&mut buffer, "{}", value)?;
    Ok(buffer)
}