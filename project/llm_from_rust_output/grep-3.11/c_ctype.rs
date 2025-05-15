pub fn c_isalnum(c: i32) -> bool {
    match c as u8 as char {
        '0'..='9' | 'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
}

pub fn c_isalpha(c: i32) -> bool {
    match c as u8 as char {
        'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
}

pub fn c_isascii(c: i32) -> bool {
    (c as u8).is_ascii()
}

pub fn c_isblank(c: i32) -> bool {
    match c as u8 as char {
        ' ' | '\t' => true,
        _ => false,
    }
}

pub fn c_iscntrl(c: i32) -> bool {
    (c as u8).is_ascii_control()
}

pub fn c_isdigit(c: i32) -> bool {
    match c as u8 as char {
        '0'..='9' => true,
        _ => false,
    }
}

pub fn c_isgraph(c: i32) -> bool {
    (c as u8).is_ascii_graphic()
}

pub fn c_islower(c: i32) -> bool {
    match c as u8 as char {
        'a'..='z' => true,
        _ => false,
    }
}

pub fn c_isprint(c: i32) -> bool {
    (c as u8).is_ascii() && !(c as u8).is_ascii_control()
}

pub fn c_ispunct(c: i32) -> bool {
    match c as u8 as char {
        '!'..='/' | ':'..='@' | '['..='`' | '{'..='~' => true,
        _ => false,
    }
}

pub fn c_isspace(c: i32) -> bool {
    (c as u8).is_ascii_whitespace()
}

pub fn c_isupper(c: i32) -> bool {
    match c as u8 as char {
        'A'..='Z' => true,
        _ => false,
    }
}

pub fn c_isxdigit(c: i32) -> bool {
    match c as u8 as char {
        '0'..='9' | 'a'..='f' | 'A'..='F' => true,
        _ => false,
    }
}

pub fn c_tolower(c: i32) -> i32 {
    match c as u8 as char {
        'A'..='Z' => (c as u8).to_ascii_lowercase() as i32,
        _ => c,
    }
}

pub fn c_toupper(c: i32) -> i32 {
    match c as u8 as char {
        'a'..='z' => (c as u8).to_ascii_uppercase() as i32,
        _ => c,
    }
}