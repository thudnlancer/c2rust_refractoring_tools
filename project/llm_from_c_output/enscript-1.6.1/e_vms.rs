/*
 * AFM vms encoding.
 *
 * This file is automatically generated from file `vms.txt'.  If you
 * have any corrections to this file, please, edit file `vms.txt' instead.
 */

/*
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; see the file COPYING.  If not, write to
 * the Free Software Foundation, 59 Temple Place - Suite 330,
 * Boston, MA 02111-1307, USA.
 */

#[derive(Debug, Clone, Copy)]
pub enum AfmEnc {
    None,
    Named(&'static str),
}

pub struct AfmEncodingEntry {
    pub code: i32,
    pub enc: AfmEnc,
}

pub const AFM_VMS_ENCODING: &[AfmEncodingEntry] = &[
    AfmEncodingEntry { code: 0x00, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x01, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x02, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x03, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x04, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x05, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x06, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x07, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x08, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x09, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x0a, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x0b, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x0c, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x0d, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x0e, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x0f, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x10, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x11, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x12, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x13, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x14, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x15, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x16, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x17, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x18, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x19, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x1a, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x1b, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x1c, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x1d, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x1e, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x1f, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x20, enc: AfmEnc::Named("space") },
    AfmEncodingEntry { code: 0x21, enc: AfmEnc::Named("exclam") },
    AfmEncodingEntry { code: 0x22, enc: AfmEnc::Named("quotedbl") },
    AfmEncodingEntry { code: 0x23, enc: AfmEnc::Named("numbersign") },
    AfmEncodingEntry { code: 0x24, enc: AfmEnc::Named("dollar") },
    AfmEncodingEntry { code: 0x25, enc: AfmEnc::Named("percent") },
    AfmEncodingEntry { code: 0x26, enc: AfmEnc::Named("ampersand") },
    AfmEncodingEntry { code: 0x27, enc: AfmEnc::Named("quoteright") },
    AfmEncodingEntry { code: 0x28, enc: AfmEnc::Named("parenleft") },
    AfmEncodingEntry { code: 0x29, enc: AfmEnc::Named("parenright") },
    AfmEncodingEntry { code: 0x2a, enc: AfmEnc::Named("asterisk") },
    AfmEncodingEntry { code: 0x2b, enc: AfmEnc::Named("plus") },
    AfmEncodingEntry { code: 0x2c, enc: AfmEnc::Named("comma") },
    AfmEncodingEntry { code: 0x2d, enc: AfmEnc::Named("hyphen") },
    AfmEncodingEntry { code: 0x2e, enc: AfmEnc::Named("period") },
    AfmEncodingEntry { code: 0x2f, enc: AfmEnc::Named("slash") },
    AfmEncodingEntry { code: 0x30, enc: AfmEnc::Named("zero") },
    AfmEncodingEntry { code: 0x31, enc: AfmEnc::Named("one") },
    AfmEncodingEntry { code: 0x32, enc: AfmEnc::Named("two") },
    AfmEncodingEntry { code: 0x33, enc: AfmEnc::Named("three") },
    AfmEncodingEntry { code: 0x34, enc: AfmEnc::Named("four") },
    AfmEncodingEntry { code: 0x35, enc: AfmEnc::Named("five") },
    AfmEncodingEntry { code: 0x36, enc: AfmEnc::Named("six") },
    AfmEncodingEntry { code: 0x37, enc: AfmEnc::Named("seven") },
    AfmEncodingEntry { code: 0x38, enc: AfmEnc::Named("eight") },
    AfmEncodingEntry { code: 0x39, enc: AfmEnc::Named("nine") },
    AfmEncodingEntry { code: 0x3a, enc: AfmEnc::Named("colon") },
    AfmEncodingEntry { code: 0x3b, enc: AfmEnc::Named("semicolon") },
    AfmEncodingEntry { code: 0x3c, enc: AfmEnc::Named("less") },
    AfmEncodingEntry { code: 0x3d, enc: AfmEnc::Named("equal") },
    AfmEncodingEntry { code: 0x3e, enc: AfmEnc::Named("greater") },
    AfmEncodingEntry { code: 0x3f, enc: AfmEnc::Named("question") },
    AfmEncodingEntry { code: 0x40, enc: AfmEnc::Named("at") },
    AfmEncodingEntry { code: 0x41, enc: AfmEnc::Named("A") },
    AfmEncodingEntry { code: 0x42, enc: AfmEnc::Named("B") },
    AfmEncodingEntry { code: 0x43, enc: AfmEnc::Named("C") },
    AfmEncodingEntry { code: 0x44, enc: AfmEnc::Named("D") },
    AfmEncodingEntry { code: 0x45, enc: AfmEnc::Named("E") },
    AfmEncodingEntry { code: 0x46, enc: AfmEnc::Named("F") },
    AfmEncodingEntry { code: 0x47, enc: AfmEnc::Named("G") },
    AfmEncodingEntry { code: 0x48, enc: AfmEnc::Named("H") },
    AfmEncodingEntry { code: 0x49, enc: AfmEnc::Named("I") },
    AfmEncodingEntry { code: 0x4a, enc: AfmEnc::Named("J") },
    AfmEncodingEntry { code: 0x4b, enc: AfmEnc::Named("K") },
    AfmEncodingEntry { code: 0x4c, enc: AfmEnc::Named("L") },
    AfmEncodingEntry { code: 0x4d, enc: AfmEnc::Named("M") },
    AfmEncodingEntry { code: 0x4e, enc: AfmEnc::Named("N") },
    AfmEncodingEntry { code: 0x4f, enc: AfmEnc::Named("O") },
    AfmEncodingEntry { code: 0x50, enc: AfmEnc::Named("P") },
    AfmEncodingEntry { code: 0x51, enc: AfmEnc::Named("Q") },
    AfmEncodingEntry { code: 0x52, enc: AfmEnc::Named("R") },
    AfmEncodingEntry { code: 0x53, enc: AfmEnc::Named("S") },
    AfmEncodingEntry { code: 0x54, enc: AfmEnc::Named("T") },
    AfmEncodingEntry { code: 0x55, enc: AfmEnc::Named("U") },
    AfmEncodingEntry { code: 0x56, enc: AfmEnc::Named("V") },
    AfmEncodingEntry { code: 0x57, enc: AfmEnc::Named("W") },
    AfmEncodingEntry { code: 0x58, enc: AfmEnc::Named("X") },
    AfmEncodingEntry { code: 0x59, enc: AfmEnc::Named("Y") },
    AfmEncodingEntry { code: 0x5a, enc: AfmEnc::Named("Z") },
    AfmEncodingEntry { code: 0x5b, enc: AfmEnc::Named("bracketleft") },
    AfmEncodingEntry { code: 0x5c, enc: AfmEnc::Named("backslash") },
    AfmEncodingEntry { code: 0x5d, enc: AfmEnc::Named("bracketright") },
    AfmEncodingEntry { code: 0x5e, enc: AfmEnc::Named("asciicircum") },
    AfmEncodingEntry { code: 0x5f, enc: AfmEnc::Named("underscore") },
    AfmEncodingEntry { code: 0x60, enc: AfmEnc::Named("quoteleft") },
    AfmEncodingEntry { code: 0x61, enc: AfmEnc::Named("a") },
    AfmEncodingEntry { code: 0x62, enc: AfmEnc::Named("b") },
    AfmEncodingEntry { code: 0x63, enc: AfmEnc::Named("c") },
    AfmEncodingEntry { code: 0x64, enc: AfmEnc::Named("d") },
    AfmEncodingEntry { code: 0x65, enc: AfmEnc::Named("e") },
    AfmEncodingEntry { code: 0x66, enc: AfmEnc::Named("f") },
    AfmEncodingEntry { code: 0x67, enc: AfmEnc::Named("g") },
    AfmEncodingEntry { code: 0x68, enc: AfmEnc::Named("h") },
    AfmEncodingEntry { code: 0x69, enc: AfmEnc::Named("i") },
    AfmEncodingEntry { code: 0x6a, enc: AfmEnc::Named("j") },
    AfmEncodingEntry { code: 0x6b, enc: AfmEnc::Named("k") },
    AfmEncodingEntry { code: 0x6c, enc: AfmEnc::Named("l") },
    AfmEncodingEntry { code: 0x6d, enc: AfmEnc::Named("m") },
    AfmEncodingEntry { code: 0x6e, enc: AfmEnc::Named("n") },
    AfmEncodingEntry { code: 0x6f, enc: AfmEnc::Named("o") },
    AfmEncodingEntry { code: 0x70, enc: AfmEnc::Named("p") },
    AfmEncodingEntry { code: 0x71, enc: AfmEnc::Named("q") },
    AfmEncodingEntry { code: 0x72, enc: AfmEnc::Named("r") },
    AfmEncodingEntry { code: 0x73, enc: AfmEnc::Named("s") },
    AfmEncodingEntry { code: 0x74, enc: AfmEnc::Named("t") },
    AfmEncodingEntry { code: 0x75, enc: AfmEnc::Named("u") },
    AfmEncodingEntry { code: 0x76, enc: AfmEnc::Named("v") },
    AfmEncodingEntry { code: 0x77, enc: AfmEnc::Named("w") },
    AfmEncodingEntry { code: 0x78, enc: AfmEnc::Named("x") },
    AfmEncodingEntry { code: 0x79, enc: AfmEnc::Named("y") },
    AfmEncodingEntry { code: 0x7a, enc: AfmEnc::Named("z") },
    AfmEncodingEntry { code: 0x7b, enc: AfmEnc::Named("braceleft") },
    AfmEncodingEntry { code: 0x7c, enc: AfmEnc::Named("bar") },
    AfmEncodingEntry { code: 0x7d, enc: AfmEnc::Named("braceright") },
    AfmEncodingEntry { code: 0x7e, enc: AfmEnc::Named("tilde") },
    AfmEncodingEntry { code: 0x7f, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x80, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x81, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x82, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x83, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x84, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x85, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x86, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x87, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x88, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x89, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x8a, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x8b, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x8c, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x8d, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x8e, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x8f, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x90, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x91, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x92, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x93, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x94, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x95, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x96, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x97, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x98, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x99, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x9a, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x9b, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x9c, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x9d, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x9e, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0x9f, enc: AfmEnc::None },
    AfmEncodingEntry { code: 0xa0, enc: AfmEnc::Named("space") },
    AfmEncodingEntry { code: 0xa1, enc: AfmEnc::Named("exclamdown") },
    AfmEncodingEntry { code: 0xa2, enc: AfmEnc::Named("cent") },
    AfmEncoding