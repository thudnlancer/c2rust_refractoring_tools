/*  Copyright 1996,1997,1999,2001,2002,2008,2009 Alain Knaff.
 *  This file is part of mtools.
 *
 *  Mtools is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Mtools is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Mtools.  If not, see <http://www.gnu.org/licenses/>.
 *
 */

#[derive(Debug)]
pub struct Codepage {
    pub id: u32,
    pub data: [&'static str; 8],
}

pub const CODEPAGES: &[Codepage] = &[
    Codepage {
        id: 437,
        data: [
            "",
            "ܢPf",
            "Ѫr",
            "_______________",
            "________________",
            "________________",
            "abgpSstftod_N",
            "=<>||~Vn__",
        ],
    },
    Codepage {
        id: 819,
        data: [
            "________________",
            "________________",
            "",
            "",
            "",
            "",
            "",
            "",
        ],
    },
    Codepage {
        id: 850,
        data: [
            "",
            "_",
            "Ѫ",
            "_________",
            "_____________",
            "i____|I_",
            "յޯ",
            "___",
        ],
    },
    Codepage {
        id: 852,
        data: [
            "uclZC",
            "LlLlSsTtLc",
            "AaZzEe zCs",
            "_____ES____Zz",
            "______Aa_______",
            "Dde_r__TU_",
            "NnSsRrUt",
            "~.~~uRr_",
        ],
    },
    Codepage {
        id: 860,
        data: [
            "",
            "ܢP",
            "ѪҬ",
            "_______________",
            "________________",
            "________________",
            "abgpSstftod_N",
            "=<>||~Vn__",
        ],
    },
    Codepage {
        id: 863,
        data: [
            "_",
            "ܢf",
            "| r",
            "_______________",
            "________________",
            "________________",
            "abgpSstftod_N",
            "=<>||~Vn__",
        ],
    },
    Codepage {
        id: 865,
        data: [
            "",
            "Pf",
            "Ѫr",
            "_______________",
            "________________",
            "________________",
            "abgpSstftod_N",
            "=<>||~Vn__",
        ],
    },
    Codepage {
        id: 950,
        data: ["", "", "", "", "", "", "", ""],
    },
    Codepage {
        id: 0,
        data: ["", "", "", "", "", "", "", ""],
    },
];