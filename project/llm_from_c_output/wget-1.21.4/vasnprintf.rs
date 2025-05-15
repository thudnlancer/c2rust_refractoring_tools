use std::fmt;
use std::io::{self, Write};
use std::mem;
use std::ptr;
use std::slice;
use std::str;

#[derive(Debug)]
enum Directive {
    Literal(String),
    Format {
        arg_index: usize,
        flags: u32,
        width: Option<usize>,
        precision: Option<usize>,
        conversion: char,
    },
}

struct Arguments {
    args: Vec<fmt::ArgumentV1<'static>>,
}

impl Arguments {
    fn new() -> Self {
        Arguments { args: Vec::new() }
    }

    fn add<T: fmt::Display + 'static>(&mut self, arg: T) {
        self.args.push(fmt::ArgumentV1::new(&arg, fmt::Display::fmt));
    }
}

fn parse_format_string(format: &str) -> Vec<Directive> {
    let mut directives = Vec::new();
    let mut chars = format.chars().peekable();
    let mut literal_start = 0;

    while let Some(c) = chars.next() {
        if c == '%' {
            // Add previous literal if any
            if literal_start < chars.offset() - 1 {
                directives.push(Directive::Literal(
                    format[literal_start..chars.offset() - 1].to_string(),
                ));
            }

            // Parse format directive
            let mut flags = 0;
            let mut width = None;
            let mut precision = None;
            let mut arg_index = 0;
            let mut conversion = '\0';

            // Flags
            loop {
                match chars.peek() {
                    Some('-') => {
                        flags |= 1;
                        chars.next();
                    }
                    Some('+') => {
                        flags |= 2;
                        chars.next();
                    }
                    Some(' ') => {
                        flags |= 4;
                        chars.next();
                    }
                    Some('#') => {
                        flags |= 8;
                        chars.next();
                    }
                    Some('0') => {
                        flags |= 16;
                        chars.next();
                    }
                    _ => break,
                }
            }

            // Width
            if let Some(&c) = chars.peek() {
                if c.is_ascii_digit() {
                    let mut w = 0;
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() {
                            w = w * 10 + c.to_digit(10).unwrap() as usize;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    width = Some(w);
                } else if c == '*' {
                    // TODO: Handle dynamic width
                    chars.next();
                }
            }

            // Precision
            if chars.peek() == Some(&'.') {
                chars.next();
                if let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() {
                        let mut p = 0;
                        while let Some(&c) = chars.peek() {
                            if c.is_ascii_digit() {
                                p = p * 10 + c.to_digit(10).unwrap() as usize;
                                chars.next();
                            } else {
                                break;
                            }
                        }
                        precision = Some(p);
                    } else if c == '*' {
                        // TODO: Handle dynamic precision
                        chars.next();
                    }
                }
            }

            // Length modifier
            // TODO: Handle length modifiers (h, l, L, etc.)

            // Conversion specifier
            if let Some(c) = chars.next() {
                conversion = c;
            }

            directives.push(Directive::Format {
                arg_index,
                flags,
                width,
                precision,
                conversion,
            });

            literal_start = chars.offset();
        }
    }

    // Add remaining literal if any
    if literal_start < format.len() {
        directives.push(Directive::Literal(format[literal_start..].to_string()));
    }

    directives
}

fn vasprintf(
    resultbuf: Option<&mut Vec<u8>>,
    format: &str,
    args: &Arguments,
) -> io::Result<Vec<u8>> {
    let directives = parse_format_string(format);
    let mut output = Vec::new();
    let mut arg_idx = 0;

    for directive in directives {
        match directive {
            Directive::Literal(s) => {
                output.extend_from_slice(s.as_bytes());
            }
            Directive::Format {
                arg_index,
                flags,
                width,
                precision,
                conversion,
            } => {
                if arg_index >= args.args.len() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Not enough arguments",
                    ));
                }

                let arg = &args.args[arg_index];
                let mut formatted = Vec::new();
                write!(&mut formatted, "{}", arg)?;

                // Apply width and padding
                let width = width.unwrap_or(0);
                if formatted.len() < width {
                    let padding = width - formatted.len();
                    if flags & 1 != 0 {
                        // Left-justified
                        output.extend_from_slice(&formatted);
                        output.extend(vec![b' '; padding]);
                    } else if flags & 16 != 0 {
                        // Zero-padded
                        output.extend(vec![b'0'; padding]);
                        output.extend_from_slice(&formatted);
                    } else {
                        // Right-justified with spaces
                        output.extend(vec![b' '; padding]);
                        output.extend_from_slice(&formatted);
                    }
                } else {
                    output.extend_from_slice(&formatted);
                }

                arg_idx += 1;
            }
        }
    }

    if let Some(buf) = resultbuf {
        buf.clear();
        buf.extend_from_slice(&output);
    }

    Ok(output)
}

pub fn asprintf(
    resultbuf: Option<&mut Vec<u8>>,
    format: &str,
    args: fmt::Arguments<'_>,
) -> io::Result<Vec<u8>> {
    let mut arguments = Arguments::new();
    for arg in args.as_str().split_whitespace() {
        arguments.add(arg);
    }
    vasprintf(resultbuf, format, &arguments)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_formatting() {
        let mut buf = Vec::new();
        let result = asprintf(
            Some(&mut buf),
            "Hello %s, your score is %d",
            format_args!("world", 42),
        )
        .unwrap();
        assert_eq!(result, b"Hello world, your score is 42");
    }
}