/* RCS map of character types

   Copyright (C) 2010-2020 Thien-Thi Nguyen
   Copyright (C) 1990, 1991, 1995 Paul Eggert
   Copyright (C) 1982, 1988, 1989 Walter Tichy

   This file is part of GNU RCS.

   GNU RCS is free software: you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   GNU RCS is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty
   of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
   See the GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Unkn,
    Space,
    Newln,
    IdChar,
    Delim,
    Period,
    Digit,
    Colon,
    Semi,
    Sbegin,
    Letter,
    LowerLetter,
}

const CTAB: [Token; 256] = [
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,   
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,
    Token::Space,  Token::Space,  Token::Newln,  Token::Space,  
    Token::Space,  Token::Space,  Token::Unkn,   Token::Unkn,
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,   
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,   
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,
    Token::Space,  Token::IdChar, Token::IdChar, Token::IdChar, 
    Token::Delim,  Token::IdChar, Token::IdChar, Token::IdChar,
    Token::IdChar, Token::IdChar, Token::IdChar, Token::IdChar, 
    Token::Delim,  Token::IdChar, Token::Period, Token::IdChar,
    Token::Digit,  Token::Digit,  Token::Digit,  Token::Digit,  
    Token::Digit,  Token::Digit,  Token::Digit,  Token::Digit,
    Token::Digit,  Token::Digit,  Token::Colon,  Token::Semi,   
    Token::IdChar, Token::IdChar, Token::IdChar, Token::IdChar,
    Token::Sbegin, Token::Letter, Token::Letter, Token::Letter, 
    Token::Letter, Token::Letter, Token::Letter, Token::Letter,
    Token::Letter, Token::Letter, Token::Letter, Token::Letter, 
    Token::Letter, Token::Letter, Token::Letter, Token::Letter,
    Token::Letter, Token::Letter, Token::Letter, Token::Letter, 
    Token::Letter, Token::Letter, Token::Letter, Token::Letter,
    Token::Letter, Token::Letter, Token::Letter, Token::IdChar, 
    Token::IdChar, Token::IdChar, Token::IdChar, Token::IdChar,
    Token::IdChar, Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, 
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::LowerLetter,
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, 
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::LowerLetter,
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, 
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::LowerLetter,
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::IdChar, 
    Token::IdChar, Token::IdChar, Token::IdChar, Token::Unkn,
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,   
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,   
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,   
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,   
    Token::Unkn,   Token::Unkn,   Token::Unkn,   Token::Unkn,
    Token::IdChar, Token::IdChar, Token::IdChar, Token::IdChar, 
    Token::IdChar, Token::IdChar, Token::IdChar, Token::IdChar,
    Token::IdChar, Token::IdChar, Token::IdChar, Token::IdChar, 
    Token::IdChar, Token::IdChar, Token::IdChar, Token::IdChar,
    Token::IdChar, Token::IdChar, Token::IdChar, Token::IdChar, 
    Token::IdChar, Token::IdChar, Token::IdChar, Token::IdChar,
    Token::IdChar, Token::IdChar, Token::IdChar, Token::IdChar, 
    Token::IdChar, Token::IdChar, Token::IdChar, Token::IdChar,
    Token::Letter, Token::Letter, Token::Letter, Token::Letter, 
    Token::Letter, Token::Letter, Token::Letter, Token::Letter,
    Token::Letter, Token::Letter, Token::Letter, Token::Letter, 
    Token::Letter, Token::Letter, Token::Letter, Token::Letter,
    Token::Letter, Token::Letter, Token::Letter, Token::Letter, 
    Token::Letter, Token::Letter, Token::Letter, Token::IdChar,
    Token::Letter, Token::Letter, Token::Letter, Token::Letter, 
    Token::Letter, Token::Letter, Token::Letter, Token::LowerLetter,
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, 
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::LowerLetter,
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, 
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::LowerLetter,
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, 
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::IdChar,
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, 
    Token::LowerLetter, Token::LowerLetter, Token::LowerLetter, Token::LowerLetter
];

fn check_identifier(id: &str, delimiter: Option<char>, dot_ok: bool) -> Result<usize, String> {
    let mut is_id = false;
    let mut pos = 0;

    for (i, c) in id.chars().enumerate() {
        if c as usize >= CTAB.len() {
            return Err(format!("Invalid character at position {}", i));
        }
        match CTAB[c as usize] {
            Token::Digit | Token::IdChar | Token::Letter | Token::LowerLetter => {
                is_id = true;
                pos = i + 1;
            }
            Token::Period if dot_ok => {
                pos = i + 1;
            }
            _ => break,
        }
    }

    if !is_id {
        return Err("Invalid identifier".to_string());
    }

    if let Some(c) = id.chars().nth(pos) {
        if let Some(delim) = delimiter {
            if c != delim && c != ' ' && c != '\t' && c != '\n' {
                return Err(format!("Invalid delimiter at position {}", pos));
            }
        } else {
            return Err("Unexpected character after identifier".to_string());
        }
    }

    Ok(pos)
}

pub fn check_id(id: &str, delimiter: Option<char>) -> Result<usize, String> {
    check_identifier(id, delimiter, true)
}

pub fn check_sym(sym: &str, delimiter: Option<char>) -> Result<usize, String> {
    check_identifier(sym, delimiter, false)
}

pub fn check_sid(id: &str) -> Result<(), String> {
    check_id(id, None).map(|_| ())
}

pub fn check_ssym(sym: &str) -> Result<(), String> {
    check_sym(sym, None).map(|_| ())
}