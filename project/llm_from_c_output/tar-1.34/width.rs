// Determine display width of Unicode character.
// Copyright (C) 2001-2002, 2006-2021 Free Software Foundation, Inc.
// Written by Bruno Haible <bruno@clisp.org>, 2002.

// This program is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published
// by the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

const NONSPACING_TABLE_DATA: [u8; 38*64] = [
    /* 0x0000-0x01ff */
    0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, /* 0x0000-0x003f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, /* 0x0040-0x007f */
    0xff, 0xff, 0xff, 0xff, 0x00, 0x20, 0x00, 0x00, /* 0x0080-0x00bf */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x00c0-0x00ff */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0100-0x013f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0140-0x017f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0180-0x01bf */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x01c0-0x01ff */
    /* 0x0200-0x03ff */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0200-0x023f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0240-0x027f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0280-0x02bf */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x02c0-0x02ff */
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, /* 0x0300-0x033f */
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, /* 0x0340-0x037f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0380-0x03bf */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x03c0-0x03ff */
    /* 0x0400-0x05ff */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0400-0x043f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0440-0x047f */
    0xf8, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0480-0x04bf */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x04c0-0x04ff */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0500-0x053f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0540-0x057f */
    0x00, 0x00, 0xfe, 0xff, 0xff, 0xff, 0xff, 0xbf, /* 0x0580-0x05bf */
    0xb6, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x05c0-0x05ff */
    /* 0x0600-0x07ff */
    0x3f, 0x00, 0xff, 0x17, 0x00, 0x00, 0x00, 0x00, /* 0x0600-0x063f */
    0x00, 0xf8, 0xff, 0xff, 0x00, 0x00, 0x01, 0x00, /* 0x0640-0x067f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0680-0x06bf */
    0x00, 0x00, 0xc0, 0xbf, 0x9f, 0x3d, 0x00, 0x00, /* 0x06c0-0x06ff */
    0x00, 0x80, 0x02, 0x00, 0x00, 0x00, 0xff, 0xff, /* 0x0700-0x073f */
    0xff, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0740-0x077f */
    0x00, 0x00, 0x00, 0x00, 0xc0, 0xff, 0x01, 0x00, /* 0x0780-0x07bf */
    0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x0f, 0x00, /* 0x07c0-0x07ff */
    /* 0x0800-0x09ff */
    0x00, 0x00, 0xc0, 0xfb, 0xef, 0x3e, 0x00, 0x00, /* 0x0800-0x083f */
    0x00, 0x00, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x00, /* 0x0840-0x087f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0880-0x08bf */
    0x00, 0x00, 0xf0, 0xff, 0xff, 0xff, 0xff, 0xff, /* 0x08c0-0x08ff */
    0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x14, /* 0x0900-0x093f */
    0xfe, 0x21, 0xfe, 0x00, 0x0c, 0x00, 0x00, 0x00, /* 0x0940-0x097f */
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, /* 0x0980-0x09bf */
    0x1e, 0x20, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, /* 0x09c0-0x09ff */
    /* 0x0a00-0x0bff */
    0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, /* 0x0a00-0x0a3f */
    0x86, 0x39, 0x02, 0x00, 0x00, 0x00, 0x23, 0x00, /* 0x0a40-0x0a7f */
    0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, /* 0x0a80-0x0abf */
    0xbe, 0x21, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, /* 0x0ac0-0x0aff */
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x90, /* 0x0b00-0x0b3f */
    0x1e, 0x20, 0x40, 0x00, 0x0c, 0x00, 0x00, 0x00, /* 0x0b40-0x0b7f */
    0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0b80-0x0bbf */
    0x01, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0bc0-0x0bff */
    /* 0x0c00-0x0dff */
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc0, /* 0x0c00-0x0c3f */
    0xc1, 0x3d, 0x60, 0x00, 0x0c, 0x00, 0x00, 0x00, /* 0x0c40-0x0c7f */
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, /* 0x0c80-0x0cbf */
    0x00, 0x30, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, /* 0x0cc0-0x0cff */
    0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0d00-0x0d3f */
    0x1e, 0x20, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, /* 0x0d40-0x0d7f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0d80-0x0dbf */
    0x00, 0x04, 0x5c, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0dc0-0x0dff */
    /* 0x0e00-0x0fff */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf2, 0x07, /* 0x0e00-0x0e3f */
    0x80, 0x7f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0e40-0x0e7f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf2, 0x1b, /* 0x0e80-0x0ebf */
    0x00, 0x3f, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0ec0-0x0eff */
    0x00, 0x00, 0x00, 0x03, 0x00, 0x00, 0xa0, 0x02, /* 0x0f00-0x0f3f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xfe, 0x7f, /* 0x0f40-0x0f7f */
    0xdf, 0xe0, 0xff, 0xfe, 0xff, 0xff, 0xff, 0x1f, /* 0x0f80-0x0fbf */
    0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x0fc0-0x0fff */
    /* 0x1000-0x11ff */
    0x00, 0x00, 0x00, 0x00, 0x00, 0xe0, 0xfd, 0x66, /* 0x1000-0x103f */
    0x00, 0x00, 0x00, 0xc3, 0x01, 0x00, 0x1e, 0x00, /* 0x1040-0x107f */
    0x64, 0x20, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, /* 0x1080-0x10bf */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x10c0-0x10ff */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x1100-0x113f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x1140-0x117f */
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, /* 0x1180-0x11bf */
    0x00, 0x00, 0x00, 0x00