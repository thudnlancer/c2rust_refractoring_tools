pub fn c_isalnum(c: i32) -> bool {
    match c {
        b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z' => true,
        _ => false,
    }
}

pub fn c_isalpha(c: i32) -> bool {
    match c {
        b'a'..=b'z' | b'A'..=b'Z' => true,
        _ => false,
    }
}

pub fn c_isascii(c: i32) -> bool {
    (0..=127).contains(&c)
}

pub fn c_isblank(c: i32) -> bool {
    c == b' ' as i32 || c == b'\t' as i32
}

pub fn c_iscntrl(c: i32) -> bool {
    match c {
        0..=31 | 127 => true,
        _ => false,
    }
}

pub fn c_isdigit(c: i32) -> bool {
    match c {
        b'0'..=b'9' => true,
        _ => false,
    }
}

pub fn c_isgraph(c: i32) -> bool {
    match c {
        b'!'..=b'~' => true,
        _ => false,
    }
}

pub fn c_islower(c: i32) -> bool {
    match c {
        b'a'..=b'z' => true,
        _ => false,
    }
}

pub fn c_isprint(c: i32) -> bool {
    match c {
        b' '..=b'~' => true,
        _ => false,
    }
}

pub fn c_ispunct(c: i32) -> bool {
    match c {
        b'!'..=b'/' | b':'..=b'@' | b'['..=b'`' | b'{'..=b'~' => true,
        _ => false,
    }
}

pub fn c_isspace(c: i32) -> bool {
    match c {
        b' ' | b'\t'..=b'\r' => true,
        _ => false,
    }
}

pub fn c_isupper(c: i32) -> bool {
    match c {
        b'A'..=b'Z' => true,
        _ => false,
    }
}

pub fn c_isxdigit(c: i32) -> bool {
    match c {
        b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => true,
        _ => false,
    }
}

pub fn c_tolower(c: i32) -> i32 {
    match c {
        b'A'..=b'Z' => c + 32,
        _ => c,
    }
}

pub fn c_toupper(c: i32) -> i32 {
    match c {
        b'a'..=b'z' => c - 32,
        _ => c,
    }
}