/*
 * The default font.
 * Copyright (c) 1995, 1996, 1997 Markku Rossi.
 *
 * Author: Markku Rossi <mtr@iki.fi>
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

use std::collections::HashMap;
use std::ffi::CString;

#[derive(Debug)]
struct AFMEncodingTable {
    code: i32,
    character: &'static str,
}

#[derive(Debug)]
struct AFMIndividualCharacterMetrics {
    name: String,
    character_code: i32,
    w0x: f64,
    w0y: f64,
}

#[derive(Debug)]
struct AFMWritingDirectionMetrics {
    is_valid: bool,
    is_fixed_pitch: bool,
    char_width_x: f64,
    char_width_y: f64,
}

#[derive(Debug)]
struct AFMGlobalInfo {
    font_name: String,
    font_bbox_llx: f64,
    font_bbox_lly: f64,
    font_bbox_urx: f64,
    font_bbox_ury: f64,
}

#[derive(Debug)]
struct AFMFontPrivateData {
    fontnames: HashMap<String, AFMIndividualCharacterMetrics>,
}

#[derive(Debug)]
struct AFMFont {
    version: f64,
    global_info: AFMGlobalInfo,
    writing_direction_metrics: [AFMWritingDirectionMetrics; 1],
    character_metrics: Vec<AFMIndividualCharacterMetrics>,
    private: AFMFontPrivateData,
}

#[derive(Debug)]
enum AFMError {
    Success,
    MemoryError,
}

static BUILTIN_COURIER: &[AFMEncodingTable] = &[
    AFMEncodingTable { code: 32, character: "space" },
    AFMEncodingTable { code: 33, character: "exclam" },
    AFMEncodingTable { code: 34, character: "quotedbl" },
    AFMEncodingTable { code: 35, character: "numbersign" },
    AFMEncodingTable { code: 36, character: "dollar" },
    AFMEncodingTable { code: 37, character: "percent" },
    AFMEncodingTable { code: 38, character: "ampersand" },
    AFMEncodingTable { code: 39, character: "quoteright" },
    AFMEncodingTable { code: 40, character: "parenleft" },
    AFMEncodingTable { code: 41, character: "parenright" },
    AFMEncodingTable { code: 42, character: "asterisk" },
    AFMEncodingTable { code: 43, character: "plus" },
    AFMEncodingTable { code: 44, character: "comma" },
    AFMEncodingTable { code: 45, character: "hyphen" },
    AFMEncodingTable { code: 46, character: "period" },
    AFMEncodingTable { code: 47, character: "slash" },
    AFMEncodingTable { code: 48, character: "zero" },
    AFMEncodingTable { code: 49, character: "one" },
    AFMEncodingTable { code: 50, character: "two" },
    AFMEncodingTable { code: 51, character: "three" },
    AFMEncodingTable { code: 52, character: "four" },
    AFMEncodingTable { code: 53, character: "five" },
    AFMEncodingTable { code: 54, character: "six" },
    AFMEncodingTable { code: 55, character: "seven" },
    AFMEncodingTable { code: 56, character: "eight" },
    AFMEncodingTable { code: 57, character: "nine" },
    AFMEncodingTable { code: 58, character: "colon" },
    AFMEncodingTable { code: 59, character: "semicolon" },
    AFMEncodingTable { code: 60, character: "less" },
    AFMEncodingTable { code: 61, character: "equal" },
    AFMEncodingTable { code: 62, character: "greater" },
    AFMEncodingTable { code: 63, character: "question" },
    AFMEncodingTable { code: 64, character: "at" },
    AFMEncodingTable { code: 65, character: "A" },
    AFMEncodingTable { code: 66, character: "B" },
    AFMEncodingTable { code: 67, character: "C" },
    AFMEncodingTable { code: 68, character: "D" },
    AFMEncodingTable { code: 69, character: "E" },
    AFMEncodingTable { code: 70, character: "F" },
    AFMEncodingTable { code: 71, character: "G" },
    AFMEncodingTable { code: 72, character: "H" },
    AFMEncodingTable { code: 73, character: "I" },
    AFMEncodingTable { code: 74, character: "J" },
    AFMEncodingTable { code: 75, character: "K" },
    AFMEncodingTable { code: 76, character: "L" },
    AFMEncodingTable { code: 77, character: "M" },
    AFMEncodingTable { code: 78, character: "N" },
    AFMEncodingTable { code: 79, character: "O" },
    AFMEncodingTable { code: 80, character: "P" },
    AFMEncodingTable { code: 81, character: "Q" },
    AFMEncodingTable { code: 82, character: "R" },
    AFMEncodingTable { code: 83, character: "S" },
    AFMEncodingTable { code: 84, character: "T" },
    AFMEncodingTable { code: 85, character: "U" },
    AFMEncodingTable { code: 86, character: "V" },
    AFMEncodingTable { code: 87, character: "W" },
    AFMEncodingTable { code: 88, character: "X" },
    AFMEncodingTable { code: 89, character: "Y" },
    AFMEncodingTable { code: 90, character: "Z" },
    AFMEncodingTable { code: 91, character: "bracketleft" },
    AFMEncodingTable { code: 92, character: "backslash" },
    AFMEncodingTable { code: 93, character: "bracketright" },
    AFMEncodingTable { code: 94, character: "asciicircum" },
    AFMEncodingTable { code: 95, character: "underscore" },
    AFMEncodingTable { code: 96, character: "quoteleft" },
    AFMEncodingTable { code: 97, character: "a" },
    AFMEncodingTable { code: 98, character: "b" },
    AFMEncodingTable { code: 99, character: "c" },
    AFMEncodingTable { code: 100, character: "d" },
    AFMEncodingTable { code: 101, character: "e" },
    AFMEncodingTable { code: 102, character: "f" },
    AFMEncodingTable { code: 103, character: "g" },
    AFMEncodingTable { code: 104, character: "h" },
    AFMEncodingTable { code: 105, character: "i" },
    AFMEncodingTable { code: 106, character: "j" },
    AFMEncodingTable { code: 107, character: "k" },
    AFMEncodingTable { code: 108, character: "l" },
    AFMEncodingTable { code: 109, character: "m" },
    AFMEncodingTable { code: 110, character: "n" },
    AFMEncodingTable { code: 111, character: "o" },
    AFMEncodingTable { code: 112, character: "p" },
    AFMEncodingTable { code: 113, character: "q" },
    AFMEncodingTable { code: 114, character: "r" },
    AFMEncodingTable { code: 115, character: "s" },
    AFMEncodingTable { code: 116, character: "t" },
    AFMEncodingTable { code: 117, character: "u" },
    AFMEncodingTable { code: 118, character: "v" },
    AFMEncodingTable { code: 119, character: "w" },
    AFMEncodingTable { code: 120, character: "x" },
    AFMEncodingTable { code: 121, character: "y" },
    AFMEncodingTable { code: 122, character: "z" },
    AFMEncodingTable { code: 123, character: "braceleft" },
    AFMEncodingTable { code: 124, character: "bar" },
    AFMEncodingTable { code: 125, character: "braceright" },
    AFMEncodingTable { code: 126, character: "asciitilde" },
    AFMEncodingTable { code: 161, character: "exclamdown" },
    AFMEncodingTable { code: 162, character: "cent" },
    AFMEncodingTable { code: 163, character: "sterling" },
    AFMEncodingTable { code: 164, character: "fraction" },
    AFMEncodingTable { code: 165, character: "yen" },
    AFMEncodingTable { code: 166, character: "florin" },
    AFMEncodingTable { code: 167, character: "section" },
    AFMEncodingTable { code: 168, character: "currency" },
    AFMEncodingTable { code: 169, character: "quotesingle" },
    AFMEncodingTable { code: 170, character: "quotedblleft" },
    AFMEncodingTable { code: 171, character: "guillemotleft" },
    AFMEncodingTable { code: 172, character: "guilsinglleft" },
    AFMEncodingTable { code: 173, character: "guilsinglright" },
    AFMEncodingTable { code: 174, character: "fi" },
    AFMEncodingTable { code: 175, character: "fl" },
    AFMEncodingTable { code: 177, character: "endash" },
    AFMEncodingTable { code: 178, character: "dagger" },
    AFMEncodingTable { code: 179, character: "daggerdbl" },
    AFMEncodingTable { code: 180, character: "periodcentered" },
    AFMEncodingTable { code: 182, character: "paragraph" },
    AFMEncodingTable { code: 183, character: "bullet" },
    AFMEncodingTable { code: 184, character: "quotesinglbase" },
    AFMEncodingTable { code: 185, character: "quotedblbase" },
    AFMEncodingTable { code: 186, character: "quotedblright" },
    AFMEncodingTable { code: 187, character: "guillemotright" },
    AFMEncodingTable { code: 188, character: "ellipsis" },
    AFMEncodingTable { code: 189, character: "perthousand" },
    AFMEncodingTable { code: 191, character: "questiondown" },
    AFMEncodingTable { code: 193, character: "grave" },
    AFMEncodingTable { code: 194, character: "acute" },
    AFMEncodingTable { code: 195, character: "circumflex" },
    AFMEncodingTable { code: 196, character: "tilde" },
    AFMEncodingTable { code: 197, character: "macron" },
    AFMEncodingTable { code: 198, character: "breve" },
    AFMEncodingTable { code: 199, character: "dotaccent" },
    AFMEncodingTable { code: 200, character: "dieresis" },
    AFMEncodingTable { code: 202, character: "ring" },
    AFMEncodingTable { code: 203, character: "cedilla" },
    AFMEncodingTable { code: 205, character: "hungarumlaut" },
    AFMEncodingTable { code: 206, character: "ogonek" },
    AFMEncodingTable { code: 207, character: "caron" },
    AFMEncodingTable { code: 208, character: "emdash" },
    AFMEncodingTable { code: 225, character: "AE" },
    AFMEncodingTable { code: 227, character: "ordfeminine" },
    AFMEncodingTable { code: 232, character: "Lslash" },
    AFMEncodingTable { code: 233, character: "Oslash" },
    AFMEncodingTable { code: 234, character: "OE" },
    AFMEncodingTable { code: 235, character: "ordmasculine" },
    AFMEncodingTable { code: 241, character: "ae" },
    AFMEncodingTable { code: 245, character: "dotlessi" },
    AFMEncodingTable { code: 248, character: "lslash" },
    AFMEncodingTable { code: 249, character: "oslash" },
    AFMEncodingTable { code: 250, character: "oe" },
    AFMEncodingTable { code: 251, character: "germandbls" },
    AFMEncodingTable { code: -1, character: "Aacute" },
    AFMEncodingTable { code: -1, character: "Acircumflex" },
    AFMEncodingTable { code: -1, character: "Adieresis" },
    AFMEncodingTable { code: -1, character: "Agrave" },
    AFMEncodingTable { code: -1, character: "Aring" },
    AFMEncodingTable { code: -1, character: "Atilde" },
    AFMEncodingTable { code: -1, character: "Ccedilla" },
    AFMEncodingTable { code: -1, character: "Eacute" },
    AFMEncodingTable { code: -1, character: "Ecircumflex" },
    AFMEncodingTable { code: -1, character: "Edieresis" },
    AFMEncodingTable { code: -1, character: "Egrave" },
    AFMEncodingTable { code: -1, character: "Eth" },
    AFMEncodingTable { code: -1, character: "Gcaron" },
    AFMEncodingTable { code: -1, character: "IJ" },
    AFMEncodingTable { code: -1, character: "Iacute" },
    AFMEncodingTable { code: -1, character: "Icircumflex" },
    AFMEncodingTable { code: -1, character: "Idieresis" },
    AFMEncodingTable { code: -1, character: "Idot" },
    AFMEncodingTable { code: -1, character: "Igrave" },
    AFMEncodingTable { code: -1, character: "LL" },
    AFMEncodingTable { code: -1, character: "Ntilde" },
    AFMEncodingTable { code: -1, character: "Oacute" },
    AFMEncodingTable { code: -1, character: "Ocircumflex" },
    AFMEncodingTable { code: -1, character: "Odieresis" },
    AFMEncodingTable { code: -1, character: "Ograve" },
    AFMEncodingTable { code: -1, character: "Otilde" },
    AFMEncodingTable { code: -1, character: "Scaron" },
    AFMEncodingTable { code: -1, character: "Scedilla" },
    AFMEncodingTable { code: -1, character: "Thorn" },
    AFMEncodingTable { code: -1, character: "Uacute" },
    AFMEncodingTable { code: -1, character: "Ucircumflex" },
    AFMEncodingTable { code: -1, character: "Udieresis" },
    AFMEncodingTable { code: -1, character: "Ugrave" },
    AFMEncodingTable { code: -1, character: "Yacute" },
    AFMEncodingTable { code: -1, character: "Ydieresis" },
    AFMEncodingTable { code: -1, character: "Zcaron" },
    AFMEncodingTable { code: -1, character: "aacute" },
    AFMEncodingTable { code: -1, character: "acircumflex" },
    AFMEncodingTable { code: -1, character: "adieresis" },
    AFMEncodingTable { code: -1, character: "agrave" },
    AFMEncodingTable { code: -1, character: "aring" },
    AFMEncodingTable { code: -1, character: "arrowboth" },
    AFMEncodingTable { code: -1, character: "arrowdown" },
    AFMEncodingTable { code: -1, character: "arrowleft" },
    AFMEncodingTable { code: -1, character: "arrowright" },
    AFMEncodingTable { code: -1, character: "arrowup" },
    AFMEncodingTable { code: -1, character: "atilde" },
    AFMEncodingTable { code: -1, character: "brokenbar" },
    AFMEncodingTable { code: -1, character: "ccedilla" },
    AFMEncodingTable { code: -1, character: "center" },
    AFMEncodingTable { code: -1, character