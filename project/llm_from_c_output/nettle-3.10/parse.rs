// parse.rs

// Copyright (C) 2002, 2003 Niels Möller
// 
// This file is part of GNU Nettle.
// 
// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:
// 
//     * the GNU Lesser General Public License as published by the Free
//       Software Foundation; either version 3 of the License, or (at your
//       option) any later version.
// 
// or
// 
//     * the GNU General Public License as published by the Free
//       Software Foundation; either version 2 of the License, or (at your
//       option) any later version.
// 
// or both in parallel, as here.
// 
// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
// 
// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SexpToken {
    ListStart,
    ListEnd,
    DisplayStart,
    DisplayEnd,
    String,
    Comment,
    TransportStart,
    CodingEnd,
    Display,
    Eof,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SexpMode {
    Canonical,
    Advanced,
}

pub struct SexpCompoundToken {
    pub token_type: SexpToken,
    pub display: Vec<u8>,
    pub string: Vec<u8>,
}

impl SexpCompoundToken {
    pub fn new() -> Self {
        SexpCompoundToken {
            token_type: SexpToken::Eof,
            display: Vec::new(),
            string: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.display.clear();
        self.string.clear();
    }
}

pub struct SexpParser<'a> {
    pub input: &'a mut dyn SexpInput,
    pub mode: SexpMode,
    pub level: u32,
    pub transport: u32,
}

impl<'a> SexpParser<'a> {
    pub fn new(input: &'a mut dyn SexpInput, mode: SexpMode) -> Self {
        SexpParser {
            input,
            mode,
            level: 1,
            transport: 0,
        }
    }

    fn check_token(&mut self, expected: SexpToken, buffer: &mut Vec<u8>) -> Result<(), ParseError> {
        self.input.get_token(
            if self.transport != 0 {
                SexpMode::Canonical
            } else {
                self.mode
            },
            buffer,
        )?;

        if self.input.current_token() != expected {
            Err(ParseError::SyntaxError)
        } else {
            Ok(())
        }
    }

    pub fn parse(&mut self, token: &mut SexpCompoundToken) -> Result<(), ParseError> {
        loop {
            self.input.get_token(
                if self.transport != 0 {
                    SexpMode::Canonical
                } else {
                    self.mode
                },
                &mut token.string,
            )?;

            match self.input.current_token() {
                SexpToken::ListEnd => {
                    if self.level == self.transport {
                        return Err(ParseError::UnmatchedListEnd);
                    }
                    self.level -= 1;

                    if self.level == 0 {
                        return Err(ParseError::UnmatchedListEnd);
                    }

                    token.token_type = SexpToken::ListEnd;

                    if self.level == self.transport {
                        self.check_token(SexpToken::CodingEnd, &mut token.string)?;
                        assert_ne!(self.transport, 0);
                        assert_eq!(self.level, self.transport);

                        self.level -= 1;
                        self.transport = 0;
                    }
                    return Ok(());
                }

                SexpToken::Eof => {
                    if self.level > 1 {
                        return Err(ParseError::UnexpectedEof);
                    }

                    token.token_type = SexpToken::Eof;
                    return Ok(());
                }

                SexpToken::ListStart => {
                    self.level += 1;
                    token.token_type = SexpToken::ListStart;
                    return Ok(());
                }

                SexpToken::DisplayStart => {
                    self.check_token(SexpToken::String, &mut token.display)?;
                    self.check_token(SexpToken::DisplayEnd, &mut token.display)?;
                    self.check_token(SexpToken::String, &mut token.string)?;

                    token.token_type = SexpToken::Display;

                    if self.level == self.transport {
                        self.check_token(SexpToken::CodingEnd, &mut token.string)?;
                        assert_ne!(self.transport, 0);
                        assert_eq!(self.level, self.transport);

                        self.level -= 1;
                        self.transport = 0;
                    }
                    return Ok(());
                }

                SexpToken::String => {
                    token.token_type = SexpToken::String;

                    if self.level == self.transport {
                        self.check_token(SexpToken::CodingEnd, &mut token.string)?;
                        assert_ne!(self.transport, 0);
                        assert_eq!(self.level, self.transport);

                        self.level -= 1;
                        self.transport = 0;
                    }
                    return Ok(());
                }

                SexpToken::Comment => {
                    token.token_type = SexpToken::Comment;
                    return Ok(());
                }

                SexpToken::TransportStart => {
                    if self.mode == SexpMode::Canonical {
                        return Err(ParseError::Base64InCanonical);
                    }
                    self.level += 1;
                    self.transport = self.level;
                    continue;
                }

                SexpToken::CodingEnd => return Err(ParseError::UnexpectedCodingEnd),
                SexpToken::DisplayEnd => return Err(ParseError::UnexpectedDisplayEnd),
                SexpToken::Display => unreachable!("Internal error: unexpected DISPLAY token"),
            }
        }
    }
}

pub trait SexpInput {
    fn get_token(&mut self, mode: SexpMode, buffer: &mut Vec<u8>) -> Result<(), ParseError>;
    fn current_token(&self) -> SexpToken;
}

#[derive(Debug)]
pub enum ParseError {
    SyntaxError,
    UnmatchedListEnd,
    UnexpectedEof,
    Base64InCanonical,
    UnexpectedCodingEnd,
    UnexpectedDisplayEnd,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::SyntaxError => write!(f, "Syntax error"),
            ParseError::UnmatchedListEnd => write!(f, "Unmatched end of list"),
            ParseError::UnexpectedEof => write!(f, "Unexpected end of file"),
            ParseError::Base64InCanonical => write!(f, "Base64 not allowed in canonical mode"),
            ParseError::UnexpectedCodingEnd => write!(f, "Unexpected end of transport encoding"),
            ParseError::UnexpectedDisplayEnd => write!(f, "Unexpected end of display tag"),
        }
    }
}

impl std::error::Error for ParseError {}