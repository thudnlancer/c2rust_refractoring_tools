/*
 * AFM mac encoding.
 *
 * This file is automatically generated from file `mac.txt'.  If you
 * have any corrections to this file, please, edit file `mac.txt' instead.
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

#[derive(Debug, Clone, PartialEq)]
pub enum AFMEncoding {
    None,
    Named(&'static str),
}

#[derive(Debug, Clone, PartialEq)]
pub struct AFMEncodingEntry {
    pub code: i32,
    pub encoding: AFMEncoding,
}

pub const AFM_MAC_ENCODING: &[AFMEncodingEntry] = &[
    AFMEncodingEntry { code: 0x00, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x01, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x02, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x03, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x04, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x05, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x06, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x07, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x08, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x09, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x0a, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x0b, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x0c, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x0d, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x0e, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x0f, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x10, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x11, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x12, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x13, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x14, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x15, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x16, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x17, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x18, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x19, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x1a, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x1b, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x1c, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x1d, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x1e, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x1f, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x20, encoding: AFMEncoding::Named("space") },
    AFMEncodingEntry { code: 0x21, encoding: AFMEncoding::Named("exclam") },
    AFMEncodingEntry { code: 0x22, encoding: AFMEncoding::Named("quotedbl") },
    AFMEncodingEntry { code: 0x23, encoding: AFMEncoding::Named("numbersign") },
    AFMEncodingEntry { code: 0x24, encoding: AFMEncoding::Named("dollar") },
    AFMEncodingEntry { code: 0x25, encoding: AFMEncoding::Named("percent") },
    AFMEncodingEntry { code: 0x26, encoding: AFMEncoding::Named("ampersand") },
    AFMEncodingEntry { code: 0x27, encoding: AFMEncoding::Named("quoteright") },
    AFMEncodingEntry { code: 0x28, encoding: AFMEncoding::Named("parenleft") },
    AFMEncodingEntry { code: 0x29, encoding: AFMEncoding::Named("parenright") },
    AFMEncodingEntry { code: 0x2a, encoding: AFMEncoding::Named("asterisk") },
    AFMEncodingEntry { code: 0x2b, encoding: AFMEncoding::Named("plus") },
    AFMEncodingEntry { code: 0x2c, encoding: AFMEncoding::Named("comma") },
    AFMEncodingEntry { code: 0x2d, encoding: AFMEncoding::Named("hyphen") },
    AFMEncodingEntry { code: 0x2e, encoding: AFMEncoding::Named("period") },
    AFMEncodingEntry { code: 0x2f, encoding: AFMEncoding::Named("slash") },
    AFMEncodingEntry { code: 0x30, encoding: AFMEncoding::Named("zero") },
    AFMEncodingEntry { code: 0x31, encoding: AFMEncoding::Named("one") },
    AFMEncodingEntry { code: 0x32, encoding: AFMEncoding::Named("two") },
    AFMEncodingEntry { code: 0x33, encoding: AFMEncoding::Named("three") },
    AFMEncodingEntry { code: 0x34, encoding: AFMEncoding::Named("four") },
    AFMEncodingEntry { code: 0x35, encoding: AFMEncoding::Named("five") },
    AFMEncodingEntry { code: 0x36, encoding: AFMEncoding::Named("six") },
    AFMEncodingEntry { code: 0x37, encoding: AFMEncoding::Named("seven") },
    AFMEncodingEntry { code: 0x38, encoding: AFMEncoding::Named("eight") },
    AFMEncodingEntry { code: 0x39, encoding: AFMEncoding::Named("nine") },
    AFMEncodingEntry { code: 0x3a, encoding: AFMEncoding::Named("colon") },
    AFMEncodingEntry { code: 0x3b, encoding: AFMEncoding::Named("semicolon") },
    AFMEncodingEntry { code: 0x3c, encoding: AFMEncoding::Named("less") },
    AFMEncodingEntry { code: 0x3d, encoding: AFMEncoding::Named("equal") },
    AFMEncodingEntry { code: 0x3e, encoding: AFMEncoding::Named("greater") },
    AFMEncodingEntry { code: 0x3f, encoding: AFMEncoding::Named("question") },
    AFMEncodingEntry { code: 0x40, encoding: AFMEncoding::Named("at") },
    AFMEncodingEntry { code: 0x41, encoding: AFMEncoding::Named("A") },
    AFMEncodingEntry { code: 0x42, encoding: AFMEncoding::Named("B") },
    AFMEncodingEntry { code: 0x43, encoding: AFMEncoding::Named("C") },
    AFMEncodingEntry { code: 0x44, encoding: AFMEncoding::Named("D") },
    AFMEncodingEntry { code: 0x45, encoding: AFMEncoding::Named("E") },
    AFMEncodingEntry { code: 0x46, encoding: AFMEncoding::Named("F") },
    AFMEncodingEntry { code: 0x47, encoding: AFMEncoding::Named("G") },
    AFMEncodingEntry { code: 0x48, encoding: AFMEncoding::Named("H") },
    AFMEncodingEntry { code: 0x49, encoding: AFMEncoding::Named("I") },
    AFMEncodingEntry { code: 0x4a, encoding: AFMEncoding::Named("J") },
    AFMEncodingEntry { code: 0x4b, encoding: AFMEncoding::Named("K") },
    AFMEncodingEntry { code: 0x4c, encoding: AFMEncoding::Named("L") },
    AFMEncodingEntry { code: 0x4d, encoding: AFMEncoding::Named("M") },
    AFMEncodingEntry { code: 0x4e, encoding: AFMEncoding::Named("N") },
    AFMEncodingEntry { code: 0x4f, encoding: AFMEncoding::Named("O") },
    AFMEncodingEntry { code: 0x50, encoding: AFMEncoding::Named("P") },
    AFMEncodingEntry { code: 0x51, encoding: AFMEncoding::Named("Q") },
    AFMEncodingEntry { code: 0x52, encoding: AFMEncoding::Named("R") },
    AFMEncodingEntry { code: 0x53, encoding: AFMEncoding::Named("S") },
    AFMEncodingEntry { code: 0x54, encoding: AFMEncoding::Named("T") },
    AFMEncodingEntry { code: 0x55, encoding: AFMEncoding::Named("U") },
    AFMEncodingEntry { code: 0x56, encoding: AFMEncoding::Named("V") },
    AFMEncodingEntry { code: 0x57, encoding: AFMEncoding::Named("W") },
    AFMEncodingEntry { code: 0x58, encoding: AFMEncoding::Named("X") },
    AFMEncodingEntry { code: 0x59, encoding: AFMEncoding::Named("Y") },
    AFMEncodingEntry { code: 0x5a, encoding: AFMEncoding::Named("Z") },
    AFMEncodingEntry { code: 0x5b, encoding: AFMEncoding::Named("bracketleft") },
    AFMEncodingEntry { code: 0x5c, encoding: AFMEncoding::Named("backslash") },
    AFMEncodingEntry { code: 0x5d, encoding: AFMEncoding::Named("bracketright") },
    AFMEncodingEntry { code: 0x5e, encoding: AFMEncoding::Named("asciicircum") },
    AFMEncodingEntry { code: 0x5f, encoding: AFMEncoding::Named("underscore") },
    AFMEncodingEntry { code: 0x60, encoding: AFMEncoding::Named("quoteleft") },
    AFMEncodingEntry { code: 0x61, encoding: AFMEncoding::Named("a") },
    AFMEncodingEntry { code: 0x62, encoding: AFMEncoding::Named("b") },
    AFMEncodingEntry { code: 0x63, encoding: AFMEncoding::Named("c") },
    AFMEncodingEntry { code: 0x64, encoding: AFMEncoding::Named("d") },
    AFMEncodingEntry { code: 0x65, encoding: AFMEncoding::Named("e") },
    AFMEncodingEntry { code: 0x66, encoding: AFMEncoding::Named("f") },
    AFMEncodingEntry { code: 0x67, encoding: AFMEncoding::Named("g") },
    AFMEncodingEntry { code: 0x68, encoding: AFMEncoding::Named("h") },
    AFMEncodingEntry { code: 0x69, encoding: AFMEncoding::Named("i") },
    AFMEncodingEntry { code: 0x6a, encoding: AFMEncoding::Named("j") },
    AFMEncodingEntry { code: 0x6b, encoding: AFMEncoding::Named("k") },
    AFMEncodingEntry { code: 0x6c, encoding: AFMEncoding::Named("l") },
    AFMEncodingEntry { code: 0x6d, encoding: AFMEncoding::Named("m") },
    AFMEncodingEntry { code: 0x6e, encoding: AFMEncoding::Named("n") },
    AFMEncodingEntry { code: 0x6f, encoding: AFMEncoding::Named("o") },
    AFMEncodingEntry { code: 0x70, encoding: AFMEncoding::Named("p") },
    AFMEncodingEntry { code: 0x71, encoding: AFMEncoding::Named("q") },
    AFMEncodingEntry { code: 0x72, encoding: AFMEncoding::Named("r") },
    AFMEncodingEntry { code: 0x73, encoding: AFMEncoding::Named("s") },
    AFMEncodingEntry { code: 0x74, encoding: AFMEncoding::Named("t") },
    AFMEncodingEntry { code: 0x75, encoding: AFMEncoding::Named("u") },
    AFMEncodingEntry { code: 0x76, encoding: AFMEncoding::Named("v") },
    AFMEncodingEntry { code: 0x77, encoding: AFMEncoding::Named("w") },
    AFMEncodingEntry { code: 0x78, encoding: AFMEncoding::Named("x") },
    AFMEncodingEntry { code: 0x79, encoding: AFMEncoding::Named("y") },
    AFMEncodingEntry { code: 0x7a, encoding: AFMEncoding::Named("z") },
    AFMEncodingEntry { code: 0x7b, encoding: AFMEncoding::Named("braceleft") },
    AFMEncodingEntry { code: 0x7c, encoding: AFMEncoding::Named("bar") },
    AFMEncodingEntry { code: 0x7d, encoding: AFMEncoding::Named("braceright") },
    AFMEncodingEntry { code: 0x7e, encoding: AFMEncoding::Named("tilde") },
    AFMEncodingEntry { code: 0x7f, encoding: AFMEncoding::None },
    AFMEncodingEntry { code: 0x80, encoding: AFMEncoding::Named("Adieresis") },
    AFMEncodingEntry { code: 0x81, encoding: AFMEncoding::Named("Aring") },
    AFMEncodingEntry { code: 0x82, encoding: AFMEncoding::Named("Ccedilla") },
    AFMEncodingEntry { code: 0x83, encoding: AFMEncoding::Named("Eacute") },
    AFMEncodingEntry { code: 0x84, encoding: AFMEncoding::Named("Ntilde") },
    AFMEncodingEntry { code: 0x85, encoding: AFMEncoding::Named("Odieresis") },
    AFMEncodingEntry { code: 0x86, encoding: AFMEncoding::Named("Udieresis") },
    AFMEncodingEntry { code: 0x87, encoding: AFMEncoding::Named("aacute") },
    AFMEncodingEntry { code: 0x88, encoding: AFMEncoding::Named("agrave") },
    AFMEncodingEntry { code: 0x89, encoding: AFMEncoding::Named("acircumflex") },
    AFMEncodingEntry { code: 0x8a, encoding: AFMEncoding::Named("adieresis") },
    AFMEncodingEntry { code: 0x8b, encoding: AFMEncoding::Named("atilde") },
    AFMEncodingEntry { code: 0x8c, encoding: AFMEncoding::Named("aring") },
    AFMEncodingEntry { code: 0x8d, encoding: AFMEncoding::Named("ccedilla") },
    AFMEncodingEntry { code: 0x8e, encoding: AFMEncoding::Named("eacute") },
    AFMEncodingEntry { code: 0x8f, encoding: AFMEncoding::Named("egrave") },
    AFMEncodingEntry { code: 0x90, encoding: AFMEncoding::Named("ecircumflex") },
    AFMEncodingEntry { code: 0x91, encoding: AFMEncoding::Named("edieresis") },
    AFMEncodingEntry { code: 0x92, encoding: AFMEncoding::Named("iacute") },
    AFMEncodingEntry { code: 0x93, encoding: AFMEncoding::Named("igrave") },
    AFMEncodingEntry { code: 0x94, encoding: AFMEncoding::Named("icircumflex") },
    AFMEncodingEntry { code: 0x95, encoding: AFMEncoding::Named("idieresis") },
    AFMEncodingEntry { code: 0x96, encoding: AFMEncoding::Named("ntilde") },
    AFMEncodingEntry { code: 0x97, encoding: AFMEncoding::Named("oacute") },
    AFMEncodingEntry { code: 0x98, encoding: AFMEncoding::Named("ograve") },
    AFMEncodingEntry { code: 0x99, encoding: AFMEncoding::Named("ocircumflex") },
    AFMEncodingEntry { code: 0x9a, encoding: AFMEncoding::Named("odieresis") },
    AFMEncodingEntry { code: 0x9b, encoding: AFMEncoding::Named("otilde") },
    AFMEncodingEntry { code: 0x9c, encoding: AFMEncoding::Named("uacute") },
    AFM