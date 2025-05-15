use std::ffi::CStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum Token {
    Delim = 0,
    Digit = 1,
    IdChar = 2,
    Newln = 3,
    Letter = 4,
    LowerLetter = 5,
    Period = 6,
    Sbegin = 7,
    Space = 8,
    Unkn = 9,
    Colon = 10,
    Id = 11,
    Num = 12,
    Semi = 13,
    String = 14,
}

pub static CTAB: [Token; 256] = [
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Space,
    Token::Space,
    Token::Newln,
    Token::Space,
    Token::Space,
    Token::Space,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Space,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::Delim,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::Delim,
    Token::IdChar,
    Token::Period,
    Token::IdChar,
    Token::Digit,
    Token::Digit,
    Token::Digit,
    Token::Digit,
    Token::Digit,
    Token::Digit,
    Token::Digit,
    Token::Digit,
    Token::Digit,
    Token::Digit,
    Token::Colon,
    Token::Semi,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::Sbegin,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::Unkn,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::IdChar,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::IdChar,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::Letter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::IdChar,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
    Token::LowerLetter,
];

fn check_identifier(id: &CStr, delimiter: Option<char>, dot_ok: bool) -> Result<(), String> {
    let bytes = id.to_bytes();
    let mut is_id = false;
    let mut pos = 0;

    while pos < bytes.len() {
        let c = bytes[pos];
        match CTAB[c as usize] {
            Token::Digit | Token::IdChar | Token::Letter | Token::LowerLetter => {
                is_id = true;
                pos += 1;
            }
            Token::Period if dot_ok => {
                pos += 1;
            }
            _ => break,
        }
    }

    if !is_id
        || (pos < bytes.len()
            && delimiter.map_or(true, |d| {
                let c = bytes[pos] as char;
                c != d && c != ' ' && c != '\t' && c != '\n'
            })
    {
        while pos < bytes.len() {
            let c = bytes[pos] as char;
            if c == '\0'
                || c == ' '
                || c == '\t'
                || c == '\n'
                || delimiter.map_or(false, |d| c == d)
            {
                break;
            }
            pos += 1;
        }

        let invalid_part = &bytes[..pos];
        let invalid_str = String::from_utf8_lossy(invalid_part);
        let kind = if dot_ok { "identifier" } else { "symbol" };
        return Err(format!("invalid {} `{}`", kind, invalid_str));
    }

    Ok(())
}

pub fn check_id(id: &CStr, delimiter: Option<char>) -> Result<(), String> {
    check_identifier(id, delimiter, true)
}

pub fn check_sym(sym: &CStr, delimiter: Option<char>) -> Result<(), String> {
    check_identifier(sym, delimiter, false)
}

pub fn check_sid(id: &CStr) -> Result<(), String> {
    check_id(id, None)
}

pub fn check_ssym(sym: &CStr) -> Result<(), String> {
    check_sym(sym, None)
}