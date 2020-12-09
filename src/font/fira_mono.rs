// Copyright (c) 2012-2013, The Mozilla Corporation and Telefonica S.A.

// This Font Software is licensed under the SIL Open Font License, Version 1.1.
// This license is copied below, and is also available with a FAQ at:
// http://scripts.sil.org/OFL

// -----------------------------------------------------------
// SIL OPEN FONT LICENSE Version 1.1 - 26 February 2007
// -----------------------------------------------------------

// PREAMBLE
// The goals of the Open Font License (OFL) are to stimulate worldwide
// development of collaborative font projects, to support the font creation
// efforts of academic and linguistic communities, and to provide a free and
// open framework in which fonts may be shared and improved in partnership
// with others.

// The OFL allows the licensed fonts to be used, studied, modified and
// redistributed freely as long as they are not sold by themselves. The
// fonts, including any derivative works, can be bundled, embedded,
// redistributed and/or sold with any software provided that any reserved
// names are not used by derivative works. The fonts and derivatives,
// however, cannot be released under any other type of license. The
// requirement for fonts to remain under this license does not apply
// to any document created using the fonts or their derivatives.

// DEFINITIONS
// "Font Software" refers to the set of files released by the Copyright
// Holder(s) under this license and clearly marked as such. This may
// include source files, build scripts and documentation.

// "Reserved Font Name" refers to any names specified as such after the
// copyright statement(s).

// "Original Version" refers to the collection of Font Software components as
// distributed by the Copyright Holder(s).

// "Modified Version" refers to any derivative made by adding to, deleting,
// or substituting -- in part or in whole -- any of the components of the
// Original Version, by changing formats or by porting the Font Software to a
// new environment.

// "Author" refers to any designer, engineer, programmer, technical
// writer or other person who contributed to the Font Software.

// PERMISSION & CONDITIONS
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of the Font Software, to use, study, copy, merge, embed, modify,
// redistribute, and sell modified and unmodified copies of the Font
// Software, subject to the following conditions:

// 1) Neither the Font Software nor any of its individual components,
// in Original or Modified Versions, may be sold by itself.

// 2) Original or Modified Versions of the Font Software may be bundled,
// redistributed and/or sold with any software, provided that each copy
// contains the above copyright notice and this license. These can be
// included either as stand-alone text files, human-readable headers or
// in the appropriate machine-readable metadata fields within text or
// binary files as long as those fields can be easily viewed by the user.

// 3) No Modified Version of the Font Software may use the Reserved Font
// Name(s) unless explicit written permission is granted by the corresponding
// Copyright Holder. This restriction only applies to the primary font name as
// presented to the users.

// 4) The name(s) of the Copyright Holder(s) or the Author(s) of the Font
// Software shall not be used to promote, endorse or advertise any
// Modified Version, except to acknowledge the contribution(s) of the
// Copyright Holder(s) and the Author(s) or with their explicit written
// permission.

// 5) The Font Software, modified or unmodified, in part or in whole,
// must be distributed entirely under this license, and must not be
// distributed under any other license. The requirement for fonts to
// remain under this license does not apply to any document created
// using the Font Software.

// TERMINATION
// This license becomes null and void if any of the above conditions are
// not met.

// DISCLAIMER
// THE FONT SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO ANY WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT
// OF COPYRIGHT, PATENT, TRADEMARK, OR OTHER RIGHT. IN NO EVENT SHALL THE
// COPYRIGHT HOLDER BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
// INCLUDING ANY GENERAL, SPECIAL, INDIRECT, INCIDENTAL, OR CONSEQUENTIAL
// DAMAGES, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF THE USE OR INABILITY TO USE THE FONT SOFTWARE OR FROM
// OTHER DEALINGS IN THE FONT SOFTWARE.

use std::collections::HashMap;

use super::CharData;
use super::FontData;
use crate::math::Vec2;
use crate::math::Vec4;

pub fn fira_mono() -> FontData<'static> {
    let mut font_data = FontData {
        height: 1.0,
        line_gap: 0.0,
        ascender: 0.77916664,
        descender: -0.22083332,
        char_data: HashMap::new(),
        texture_data: DATA,
        texture_width: 970,
        texture_height: 23,
    };

    font_data.char_data.insert(
        '!',
        CharData {
            uvs: Vec4::new(0.0010309279, 0.04347826, 0.0051546395, 0.6956522),
            bounds: Vec2::new(0.16666667, 0.625),
            bearing: Vec2::new(0.20833333, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '\"',
        CharData {
            uvs: Vec4::new(0.006185567, 0.04347826, 0.012371134, 0.3043478),
            bounds: Vec2::new(0.25, 0.25),
            bearing: Vec2::new(0.125, -0.375),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '#',
        CharData {
            uvs: Vec4::new(0.013402062, 0.04347826, 0.023711339, 0.6521739),
            bounds: Vec2::new(0.41666666, 0.5833333),
            bearing: Vec2::new(0.041666668, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '$',
        CharData {
            uvs: Vec4::new(0.024742268, 0.04347826, 0.035051547, 0.95652175),
            bounds: Vec2::new(0.41666666, 0.875),
            bearing: Vec2::new(0.041666668, 0.16666667),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '%',
        CharData {
            uvs: Vec4::new(0.036082473, 0.04347826, 0.048453607, 0.73913044),
            bounds: Vec2::new(0.5, 0.6666667),
            bearing: Vec2::new(0.0, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '&',
        CharData {
            uvs: Vec4::new(0.049484536, 0.04347826, 0.060824744, 0.6956522),
            bounds: Vec2::new(0.45833334, 0.625),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '\'',
        CharData {
            uvs: Vec4::new(0.06185567, 0.04347826, 0.065979384, 0.3043478),
            bounds: Vec2::new(0.16666667, 0.25),
            bearing: Vec2::new(0.20833333, -0.375),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '(',
        CharData {
            uvs: Vec4::new(0.067010306, 0.04347826, 0.074226804, 0.91304344),
            bounds: Vec2::new(0.29166666, 0.8333333),
            bearing: Vec2::new(0.125, 0.125),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        ')',
        CharData {
            uvs: Vec4::new(0.07525773, 0.04347826, 0.08247423, 0.91304344),
            bounds: Vec2::new(0.29166666, 0.8333333),
            bearing: Vec2::new(0.125, 0.125),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '*',
        CharData {
            uvs: Vec4::new(0.08350515, 0.04347826, 0.09381443, 0.4347826),
            bounds: Vec2::new(0.41666666, 0.375),
            bearing: Vec2::new(0.083333336, -0.083333336),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '+',
        CharData {
            uvs: Vec4::new(0.09484536, 0.04347826, 0.10309278, 0.39130434),
            bounds: Vec2::new(0.33333334, 0.33333334),
            bearing: Vec2::new(0.083333336, -0.083333336),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        ',',
        CharData {
            uvs: Vec4::new(0.10412371, 0.04347826, 0.10824742, 0.39130434),
            bounds: Vec2::new(0.16666667, 0.33333334),
            bearing: Vec2::new(0.16666667, 0.16666667),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '-',
        CharData {
            uvs: Vec4::new(0.10927835, 0.04347826, 0.11752577, 0.13043478),
            bounds: Vec2::new(0.33333334, 0.083333336),
            bearing: Vec2::new(0.125, -0.20833333),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '.',
        CharData {
            uvs: Vec4::new(0.1185567, 0.04347826, 0.12268041, 0.26086956),
            bounds: Vec2::new(0.16666667, 0.20833333),
            bearing: Vec2::new(0.16666667, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '/',
        CharData {
            uvs: Vec4::new(0.12371134, 0.04347826, 0.13402061, 0.91304344),
            bounds: Vec2::new(0.41666666, 0.8333333),
            bearing: Vec2::new(0.041666668, 0.125),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '0',
        CharData {
            uvs: Vec4::new(0.13505155, 0.04347826, 0.14536083, 0.6956522),
            bounds: Vec2::new(0.41666666, 0.625),
            bearing: Vec2::new(0.083333336, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '1',
        CharData {
            uvs: Vec4::new(0.14639175, 0.04347826, 0.15670103, 0.6521739),
            bounds: Vec2::new(0.41666666, 0.5833333),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '2',
        CharData {
            uvs: Vec4::new(0.15773197, 0.04347826, 0.16701032, 0.6521739),
            bounds: Vec2::new(0.375, 0.5833333),
            bearing: Vec2::new(0.041666668, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '3',
        CharData {
            uvs: Vec4::new(0.16804124, 0.04347826, 0.17835052, 0.6956522),
            bounds: Vec2::new(0.41666666, 0.625),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '4',
        CharData {
            uvs: Vec4::new(0.17938145, 0.04347826, 0.18969072, 0.6521739),
            bounds: Vec2::new(0.41666666, 0.5833333),
            bearing: Vec2::new(0.041666668, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '5',
        CharData {
            uvs: Vec4::new(0.19072165, 0.04347826, 0.20103092, 0.6956522),
            bounds: Vec2::new(0.41666666, 0.625),
            bearing: Vec2::new(0.083333336, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '6',
        CharData {
            uvs: Vec4::new(0.20206186, 0.04347826, 0.21237114, 0.6956522),
            bounds: Vec2::new(0.41666666, 0.625),
            bearing: Vec2::new(0.083333336, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '7',
        CharData {
            uvs: Vec4::new(0.21340206, 0.04347826, 0.22371134, 0.6956522),
            bounds: Vec2::new(0.41666666, 0.625),
            bearing: Vec2::new(0.083333336, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '8',
        CharData {
            uvs: Vec4::new(0.22474226, 0.04347826, 0.23505154, 0.6956522),
            bounds: Vec2::new(0.41666666, 0.625),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '9',
        CharData {
            uvs: Vec4::new(0.23608248, 0.04347826, 0.24639176, 0.6956522),
            bounds: Vec2::new(0.41666666, 0.625),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        ':',
        CharData {
            uvs: Vec4::new(0.24742268, 0.04347826, 0.25154638, 0.5652174),
            bounds: Vec2::new(0.16666667, 0.5),
            bearing: Vec2::new(0.16666667, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        ';',
        CharData {
            uvs: Vec4::new(0.2525773, 0.04347826, 0.25670102, 0.6956522),
            bounds: Vec2::new(0.16666667, 0.625),
            bearing: Vec2::new(0.16666667, 0.16666667),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '<',
        CharData {
            uvs: Vec4::new(0.25773194, 0.04347826, 0.26804122, 0.5652174),
            bounds: Vec2::new(0.41666666, 0.5),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '=',
        CharData {
            uvs: Vec4::new(0.26907218, 0.04347826, 0.2773196, 0.3043478),
            bounds: Vec2::new(0.33333334, 0.25),
            bearing: Vec2::new(0.083333336, -0.125),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '>',
        CharData {
            uvs: Vec4::new(0.2783505, 0.04347826, 0.28865978, 0.5652174),
            bounds: Vec2::new(0.41666666, 0.5),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '?',
        CharData {
            uvs: Vec4::new(0.28969073, 0.04347826, 0.2989691, 0.6956522),
            bounds: Vec2::new(0.375, 0.625),
            bearing: Vec2::new(0.083333336, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '@',
        CharData {
            uvs: Vec4::new(0.3, 0.04347826, 0.31237113, 0.73913044),
            bounds: Vec2::new(0.5, 0.6666667),
            bearing: Vec2::new(0.0, 0.083333336),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'A',
        CharData {
            uvs: Vec4::new(0.31340206, 0.04347826, 0.32577318, 0.6521739),
            bounds: Vec2::new(0.5, 0.5833333),
            bearing: Vec2::new(0.0, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'B',
        CharData {
            uvs: Vec4::new(0.32680413, 0.04347826, 0.3371134, 0.6521739),
            bounds: Vec2::new(0.41666666, 0.5833333),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'C',
        CharData {
            uvs: Vec4::new(0.33814433, 0.04347826, 0.34948453, 0.6956522),
            bounds: Vec2::new(0.45833334, 0.625),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'D',
        CharData {
            uvs: Vec4::new(0.35051546, 0.04347826, 0.36185566, 0.6521739),
            bounds: Vec2::new(0.45833334, 0.5833333),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'E',
        CharData {
            uvs: Vec4::new(0.3628866, 0.04347826, 0.37216496, 0.6521739),
            bounds: Vec2::new(0.375, 0.5833333),
            bearing: Vec2::new(0.125, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'F',
        CharData {
            uvs: Vec4::new(0.3731959, 0.04347826, 0.38247424, 0.6521739),
            bounds: Vec2::new(0.375, 0.5833333),
            bearing: Vec2::new(0.125, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'G',
        CharData {
            uvs: Vec4::new(0.38350517, 0.04347826, 0.39484537, 0.6956522),
            bounds: Vec2::new(0.45833334, 0.625),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'H',
        CharData {
            uvs: Vec4::new(0.3958763, 0.04347826, 0.40618557, 0.6521739),
            bounds: Vec2::new(0.41666666, 0.5833333),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'I',
        CharData {
            uvs: Vec4::new(0.4072165, 0.04347826, 0.41546392, 0.6521739),
            bounds: Vec2::new(0.33333334, 0.5833333),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'J',
        CharData {
            uvs: Vec4::new(0.41649485, 0.04347826, 0.4257732, 0.6956522),
            bounds: Vec2::new(0.375, 0.625),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'K',
        CharData {
            uvs: Vec4::new(0.42680413, 0.04347826, 0.4371134, 0.6521739),
            bounds: Vec2::new(0.41666666, 0.5833333),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'L',
        CharData {
            uvs: Vec4::new(0.43814433, 0.04347826, 0.44742268, 0.6521739),
            bounds: Vec2::new(0.375, 0.5833333),
            bearing: Vec2::new(0.125, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'M',
        CharData {
            uvs: Vec4::new(0.4484536, 0.04347826, 0.46082473, 0.6521739),
            bounds: Vec2::new(0.5, 0.5833333),
            bearing: Vec2::new(0.041666668, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'N',
        CharData {
            uvs: Vec4::new(0.46185568, 0.04347826, 0.47216496, 0.6521739),
            bounds: Vec2::new(0.41666666, 0.5833333),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'O',
        CharData {
            uvs: Vec4::new(0.47319588, 0.04347826, 0.485567, 0.6956522),
            bounds: Vec2::new(0.5, 0.625),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'P',
        CharData {
            uvs: Vec4::new(0.48659793, 0.04347826, 0.4969072, 0.6521739),
            bounds: Vec2::new(0.41666666, 0.5833333),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'Q',
        CharData {
            uvs: Vec4::new(0.49793816, 0.04347826, 0.5103093, 0.82608694),
            bounds: Vec2::new(0.5, 0.75),
            bearing: Vec2::new(0.041666668, 0.16666667),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'R',
        CharData {
            uvs: Vec4::new(0.5113402, 0.04347826, 0.5216495, 0.6521739),
            bounds: Vec2::new(0.41666666, 0.5833333),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'S',
        CharData {
            uvs: Vec4::new(0.5226804, 0.04347826, 0.5340206, 0.6956522),
            bounds: Vec2::new(0.45833334, 0.625),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'T',
        CharData {
            uvs: Vec4::new(0.5350515, 0.04347826, 0.54742265, 0.6521739),
            bounds: Vec2::new(0.5, 0.5833333),
            bearing: Vec2::new(0.041666668, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'U',
        CharData {
            uvs: Vec4::new(0.5484536, 0.04347826, 0.5587629, 0.6956522),
            bounds: Vec2::new(0.41666666, 0.625),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'V',
        CharData {
            uvs: Vec4::new(0.55979383, 0.04347826, 0.57216495, 0.6521739),
            bounds: Vec2::new(0.5, 0.5833333),
            bearing: Vec2::new(0.041666668, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'W',
        CharData {
            uvs: Vec4::new(0.5731959, 0.04347826, 0.585567, 0.6521739),
            bounds: Vec2::new(0.5, 0.5833333),
            bearing: Vec2::new(0.0, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'X',
        CharData {
            uvs: Vec4::new(0.5865979, 0.04347826, 0.59896904, 0.6521739),
            bounds: Vec2::new(0.5, 0.5833333),
            bearing: Vec2::new(0.041666668, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'Y',
        CharData {
            uvs: Vec4::new(0.6, 0.04347826, 0.61237115, 0.6521739),
            bounds: Vec2::new(0.5, 0.5833333),
            bearing: Vec2::new(0.0, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'Z',
        CharData {
            uvs: Vec4::new(0.61340207, 0.04347826, 0.62371135, 0.6521739),
            bounds: Vec2::new(0.41666666, 0.5833333),
            bearing: Vec2::new(0.041666668, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '[',
        CharData {
            uvs: Vec4::new(0.62474227, 0.04347826, 0.63092786, 0.91304344),
            bounds: Vec2::new(0.25, 0.8333333),
            bearing: Vec2::new(0.16666667, 0.125),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '\\',
        CharData {
            uvs: Vec4::new(0.6319588, 0.04347826, 0.64226806, 0.91304344),
            bounds: Vec2::new(0.41666666, 0.8333333),
            bearing: Vec2::new(0.041666668, 0.125),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        ']',
        CharData {
            uvs: Vec4::new(0.643299, 0.04347826, 0.6494846, 0.91304344),
            bounds: Vec2::new(0.25, 0.8333333),
            bearing: Vec2::new(0.125, 0.125),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '^',
        CharData {
            uvs: Vec4::new(0.65051544, 0.04347826, 0.6608247, 0.34782606),
            bounds: Vec2::new(0.41666666, 0.29166666),
            bearing: Vec2::new(0.041666668, -0.41666666),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '_',
        CharData {
            uvs: Vec4::new(0.6618557, 0.04347826, 0.672165, 0.13043478),
            bounds: Vec2::new(0.41666666, 0.083333336),
            bearing: Vec2::new(0.041666668, 0.16666667),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '`',
        CharData {
            uvs: Vec4::new(0.6731959, 0.04347826, 0.6793815, 0.21739131),
            bounds: Vec2::new(0.25, 0.16666667),
            bearing: Vec2::new(0.16666667, -0.5),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'a',
        CharData {
            uvs: Vec4::new(0.68041235, 0.04347826, 0.69072163, 0.5652174),
            bounds: Vec2::new(0.41666666, 0.5),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'b',
        CharData {
            uvs: Vec4::new(0.69175255, 0.04347826, 0.7010309, 0.73913044),
            bounds: Vec2::new(0.375, 0.6666667),
            bearing: Vec2::new(0.083333336, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'c',
        CharData {
            uvs: Vec4::new(0.70206183, 0.04347826, 0.7123711, 0.5652174),
            bounds: Vec2::new(0.41666666, 0.5),
            bearing: Vec2::new(0.083333336, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'd',
        CharData {
            uvs: Vec4::new(0.71340203, 0.04347826, 0.7226804, 0.73913044),
            bounds: Vec2::new(0.375, 0.6666667),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'e',
        CharData {
            uvs: Vec4::new(0.7237113, 0.04347826, 0.7340206, 0.5652174),
            bounds: Vec2::new(0.41666666, 0.5),
            bearing: Vec2::new(0.083333336, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'f',
        CharData {
            uvs: Vec4::new(0.7350516, 0.04347826, 0.7463918, 0.6956522),
            bounds: Vec2::new(0.45833334, 0.625),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'g',
        CharData {
            uvs: Vec4::new(0.7474227, 0.04347826, 0.7587629, 0.7826087),
            bounds: Vec2::new(0.45833334, 0.7083333),
            bearing: Vec2::new(0.041666668, 0.20833333),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'h',
        CharData {
            uvs: Vec4::new(0.7597938, 0.04347826, 0.7690722, 0.6956522),
            bounds: Vec2::new(0.375, 0.625),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'i',
        CharData {
            uvs: Vec4::new(0.7701031, 0.04347826, 0.77938145, 0.73913044),
            bounds: Vec2::new(0.375, 0.6666667),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'j',
        CharData {
            uvs: Vec4::new(0.7804124, 0.04347826, 0.7886598, 0.95652175),
            bounds: Vec2::new(0.33333334, 0.875),
            bearing: Vec2::new(0.083333336, 0.20833333),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'k',
        CharData {
            uvs: Vec4::new(0.78969073, 0.04347826, 0.8, 0.6956522),
            bounds: Vec2::new(0.41666666, 0.625),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'l',
        CharData {
            uvs: Vec4::new(0.80103093, 0.04347826, 0.8113402, 0.73913044),
            bounds: Vec2::new(0.41666666, 0.6666667),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'm',
        CharData {
            uvs: Vec4::new(0.81237113, 0.04347826, 0.8226804, 0.5217391),
            bounds: Vec2::new(0.41666666, 0.45833334),
            bearing: Vec2::new(0.041666668, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'n',
        CharData {
            uvs: Vec4::new(0.82371134, 0.04347826, 0.8329897, 0.5217391),
            bounds: Vec2::new(0.375, 0.45833334),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'o',
        CharData {
            uvs: Vec4::new(0.8340206, 0.04347826, 0.8443299, 0.5652174),
            bounds: Vec2::new(0.41666666, 0.5),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'p',
        CharData {
            uvs: Vec4::new(0.8453608, 0.04347826, 0.8546392, 0.73913044),
            bounds: Vec2::new(0.375, 0.6666667),
            bearing: Vec2::new(0.083333336, 0.20833333),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'q',
        CharData {
            uvs: Vec4::new(0.8556701, 0.04347826, 0.86494845, 0.73913044),
            bounds: Vec2::new(0.375, 0.6666667),
            bearing: Vec2::new(0.041666668, 0.20833333),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'r',
        CharData {
            uvs: Vec4::new(0.8659794, 0.04347826, 0.87525773, 0.5217391),
            bounds: Vec2::new(0.375, 0.45833334),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        's',
        CharData {
            uvs: Vec4::new(0.87628865, 0.04347826, 0.88659793, 0.5652174),
            bounds: Vec2::new(0.41666666, 0.5),
            bearing: Vec2::new(0.041666668, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        't',
        CharData {
            uvs: Vec4::new(0.88762885, 0.04347826, 0.89793813, 0.6956522),
            bounds: Vec2::new(0.41666666, 0.625),
            bearing: Vec2::new(0.083333336, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'u',
        CharData {
            uvs: Vec4::new(0.89896905, 0.04347826, 0.9072165, 0.5652174),
            bounds: Vec2::new(0.33333334, 0.5),
            bearing: Vec2::new(0.083333336, 0.041666668),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'v',
        CharData {
            uvs: Vec4::new(0.9082474, 0.04347826, 0.9185567, 0.5217391),
            bounds: Vec2::new(0.41666666, 0.45833334),
            bearing: Vec2::new(0.041666668, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'w',
        CharData {
            uvs: Vec4::new(0.9195876, 0.04347826, 0.93195873, 0.5217391),
            bounds: Vec2::new(0.5, 0.45833334),
            bearing: Vec2::new(0.0, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'x',
        CharData {
            uvs: Vec4::new(0.9329897, 0.04347826, 0.943299, 0.5217391),
            bounds: Vec2::new(0.41666666, 0.45833334),
            bearing: Vec2::new(0.041666668, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'y',
        CharData {
            uvs: Vec4::new(0.9443299, 0.04347826, 0.9546392, 0.73913044),
            bounds: Vec2::new(0.41666666, 0.6666667),
            bearing: Vec2::new(0.041666668, 0.20833333),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        'z',
        CharData {
            uvs: Vec4::new(0.9556701, 0.04347826, 0.96391755, 0.5217391),
            bounds: Vec2::new(0.33333334, 0.45833334),
            bearing: Vec2::new(0.083333336, 0.0),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '{',
        CharData {
            uvs: Vec4::new(0.9649485, 0.04347826, 0.9731959, 0.91304344),
            bounds: Vec2::new(0.33333334, 0.8333333),
            bearing: Vec2::new(0.083333336, 0.125),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '|',
        CharData {
            uvs: Vec4::new(0.97422683, 0.04347826, 0.9762887, 0.91304344),
            bounds: Vec2::new(0.083333336, 0.8333333),
            bearing: Vec2::new(0.20833333, 0.125),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '}',
        CharData {
            uvs: Vec4::new(0.9773196, 0.04347826, 0.98556703, 0.91304344),
            bounds: Vec2::new(0.33333334, 0.8333333),
            bearing: Vec2::new(0.125, 0.125),
            advance: 0.5,
        },
    );

    font_data.char_data.insert(
        '~',
        CharData {
            uvs: Vec4::new(0.98659796, 0.04347826, 0.9989691, 0.26086956),
            bounds: Vec2::new(0.5, 0.20833333),
            bearing: Vec2::new(0.041666668, -0.16666667),
            advance: 0.5,
        },
    );

    font_data
}

const DATA: &[u8] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 193, 177, 0, 0, 139, 199, 71, 71, 199, 139, 0, 0, 0, 4,
    197, 81, 0, 0, 134, 148, 0, 0, 0, 0, 0, 0, 108, 225, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 5, 0, 0, 0, 0, 101, 188, 236, 207, 131, 5, 0, 0, 0, 0, 7, 199, 199, 7, 0, 0, 0, 0, 0, 7,
    139, 23, 0, 23, 139, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 113, 112, 0, 0, 0, 0, 0, 0, 0, 0, 184, 174,
    0, 0, 0, 0, 0, 28, 28, 0, 0, 125, 250, 250, 250, 250, 250, 250, 125, 0, 0, 30, 27, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 30, 164, 225, 225, 164, 30, 0, 0, 0, 0, 0, 0, 5, 119, 199,
    95, 0, 0, 0, 0, 0, 41, 174, 233, 248, 218, 129, 8, 0, 0, 0, 33, 159, 212, 241, 195, 127, 5, 0,
    0, 0, 0, 0, 0, 0, 173, 123, 0, 0, 0, 0, 0, 0, 111, 199, 199, 199, 199, 199, 199, 133, 0, 0, 0,
    0, 2, 97, 205, 244, 226, 158, 29, 0, 0, 12, 199, 199, 199, 199, 199, 199, 199, 199, 20, 0, 0,
    0, 51, 177, 229, 231, 182, 64, 0, 0, 0, 0, 1, 102, 203, 240, 217, 168, 34, 0, 0, 0, 0, 16, 14,
    0, 0, 0, 16, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 76, 147, 0, 0, 188, 214, 214, 214, 214, 214, 214,
    188, 0, 0, 144, 79, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 85, 174, 229, 227, 173, 57, 0, 0, 0, 0, 72,
    150, 198, 242, 229, 196, 100, 6, 0, 0, 0, 0, 0, 0, 0, 45, 199, 199, 45, 0, 0, 0, 0, 0, 16, 199,
    199, 199, 193, 167, 120, 27, 0, 0, 0, 0, 0, 0, 78, 185, 233, 241, 204, 144, 22, 0, 0, 68, 199,
    199, 196, 167, 131, 65, 0, 0, 0, 0, 0, 95, 199, 199, 199, 199, 199, 199, 199, 57, 0, 48, 199,
    199, 199, 199, 199, 199, 199, 164, 0, 0, 0, 0, 16, 125, 212, 245, 220, 177, 57, 0, 0, 68, 199,
    80, 0, 0, 0, 0, 80, 199, 68, 0, 187, 199, 199, 199, 199, 199, 199, 187, 0, 0, 0, 0, 175, 199,
    199, 199, 199, 187, 0, 179, 167, 0, 0, 0, 0, 15, 181, 185, 19, 0, 56, 199, 88, 0, 0, 0, 0, 0,
    0, 0, 0, 128, 199, 178, 0, 0, 0, 0, 181, 199, 128, 0, 0, 68, 199, 199, 35, 0, 0, 0, 56, 199,
    68, 0, 0, 0, 0, 60, 185, 236, 226, 169, 59, 0, 0, 0, 0, 135, 199, 199, 198, 178, 148, 79, 2, 0,
    0, 0, 0, 0, 0, 64, 187, 236, 227, 170, 60, 0, 0, 0, 0, 191, 199, 199, 196, 173, 143, 56, 0, 0,
    0, 0, 0, 0, 7, 114, 184, 233, 231, 190, 126, 11, 0, 0, 40, 199, 199, 199, 199, 199, 199, 199,
    199, 199, 199, 26, 0, 111, 199, 36, 0, 0, 0, 0, 40, 199, 111, 0, 74, 199, 96, 0, 0, 0, 0, 0, 0,
    72, 199, 74, 0, 168, 177, 0, 0, 0, 0, 0, 0, 0, 0, 150, 167, 0, 0, 155, 199, 40, 0, 0, 0, 0, 37,
    199, 131, 0, 0, 65, 199, 117, 0, 0, 0, 0, 0, 0, 93, 199, 65, 0, 0, 167, 199, 199, 199, 199,
    199, 199, 199, 111, 0, 24, 76, 76, 76, 76, 54, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 76, 76,
    76, 76, 24, 0, 0, 0, 0, 11, 190, 190, 11, 0, 0, 0, 0, 214, 214, 214, 214, 214, 214, 214, 214,
    214, 214, 0, 0, 107, 46, 0, 0, 0, 0, 0, 46, 120, 165, 186, 164, 105, 6, 0, 0, 0, 214, 163, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 53, 148, 183, 164, 117, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 163, 214,
    0, 0, 0, 8, 104, 171, 176, 127, 23, 0, 0, 0, 0, 0, 0, 0, 35, 163, 224, 234, 186, 133, 2, 0, 0,
    0, 0, 0, 0, 0, 0, 5, 72, 97, 0, 0, 207, 157, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 66, 69, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 33, 91, 10, 0, 187, 190, 0, 0, 0, 0, 0, 0, 0, 0, 0, 159, 199, 199, 199,
    199, 24, 0, 0, 0, 0, 0, 135, 48, 82, 171, 136, 8, 61, 163, 162, 30, 0, 138, 64, 6, 98, 160,
    170, 107, 1, 0, 0, 0, 0, 16, 118, 168, 170, 122, 20, 0, 0, 0, 138, 64, 17, 116, 175, 153, 71,
    0, 0, 0, 0, 0, 42, 137, 180, 140, 42, 65, 138, 0, 124, 138, 138, 8, 5, 101, 172, 179, 87, 0, 0,
    0, 37, 138, 177, 178, 142, 72, 0, 0, 0, 0, 0, 0, 2, 14, 0, 0, 0, 0, 0, 0, 138, 94, 0, 0, 0, 0,
    94, 138, 0, 97, 138, 13, 0, 0, 0, 0, 7, 137, 97, 0, 81, 138, 14, 0, 0, 0, 0, 0, 0, 3, 137, 82,
    0, 32, 138, 103, 0, 0, 0, 0, 95, 138, 32, 0, 92, 138, 15, 0, 0, 0, 0, 9, 138, 92, 0, 69, 138,
    138, 138, 138, 138, 138, 124, 0, 0, 0, 0, 0, 63, 142, 171, 4, 0, 27, 26, 0, 4, 171, 142, 63, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 243, 223, 0, 0, 165, 255, 77, 77, 255,
    165, 0, 0, 0, 35, 255, 72, 0, 0, 203, 159, 0, 0, 0, 0, 0, 0, 112, 235, 0, 0, 0, 0, 0, 0, 79,
    214, 241, 182, 30, 0, 0, 0, 86, 211, 7, 0, 0, 128, 252, 138, 83, 126, 241, 157, 0, 0, 0, 0, 0,
    250, 250, 0, 0, 0, 0, 0, 12, 191, 236, 47, 0, 47, 236, 191, 12, 0, 0, 0, 0, 0, 0, 0, 0, 192,
    190, 0, 0, 0, 0, 0, 0, 0, 0, 204, 194, 0, 0, 0, 0, 66, 251, 251, 69, 0, 59, 117, 117, 117, 117,
    117, 117, 59, 0, 91, 252, 251, 83, 0, 0, 0, 0, 0, 0, 0, 0, 0, 186, 165, 0, 0, 32, 224, 231,
    130, 130, 231, 224, 32, 0, 0, 0, 0, 58, 210, 255, 255, 122, 0, 0, 0, 0, 74, 240, 222, 148, 119,
    182, 255, 187, 2, 0, 61, 243, 212, 132, 103, 159, 252, 189, 9, 0, 0, 0, 0, 0, 42, 254, 128, 0,
    0, 0, 0, 0, 0, 143, 255, 154, 148, 148, 148, 148, 82, 0, 0, 0, 0, 148, 255, 178, 109, 122, 196,
    24, 0, 0, 9, 158, 158, 158, 158, 158, 158, 224, 252, 16, 0, 0, 86, 247, 187, 100, 100, 189,
    252, 101, 0, 0, 0, 156, 255, 180, 112, 133, 229, 240, 48, 0, 0, 61, 245, 243, 57, 0, 61, 245,
    243, 57, 0, 0, 0, 0, 0, 0, 29, 174, 255, 215, 19, 0, 135, 153, 153, 153, 153, 153, 153, 135, 0,
    16, 212, 255, 176, 30, 0, 0, 0, 0, 0, 0, 5, 146, 255, 193, 124, 126, 221, 248, 75, 0, 31, 193,
    255, 227, 177, 135, 152, 215, 255, 190, 6, 0, 0, 0, 0, 0, 0, 132, 249, 254, 132, 0, 0, 0, 0, 0,
    20, 255, 224, 163, 164, 186, 246, 249, 74, 0, 0, 0, 3, 153, 255, 223, 157, 130, 169, 246, 216,
    6, 0, 87, 255, 200, 163, 186, 235, 255, 187, 39, 0, 0, 0, 122, 255, 189, 168, 168, 168, 168,
    168, 31, 0, 61, 255, 212, 168, 168, 168, 168, 168, 122, 0, 0, 0, 35, 217, 254, 184, 132, 155,
    225, 252, 76, 0, 87, 255, 102, 0, 0, 0, 0, 102, 255, 87, 0, 149, 158, 158, 243, 243, 158, 158,
    149, 0, 0, 0, 0, 144, 163, 163, 163, 237, 240, 0, 230, 214, 0, 0, 0, 4, 184, 251, 73, 0, 0, 71,
    255, 112, 0, 0, 0, 0, 0, 0, 0, 0, 177, 254, 255, 27, 0, 0, 28, 255, 254, 177, 0, 0, 87, 255,
    255, 138, 0, 0, 0, 71, 255, 87, 0, 0, 0, 83, 252, 222, 148, 148, 222, 246, 79, 0, 0, 0, 173,
    255, 157, 153, 168, 217, 255, 206, 16, 0, 0, 0, 0, 91, 253, 222, 148, 148, 222, 247, 80, 0, 0,
    0, 245, 232, 158, 159, 178, 229, 255, 163, 1, 0, 0, 0, 2, 186, 255, 196, 132, 140, 188, 253,
    225, 25, 0, 37, 184, 184, 184, 184, 248, 244, 184, 184, 184, 184, 8, 0, 143, 255, 46, 0, 0, 0,
    0, 51, 255, 143, 0, 22, 252, 194, 0, 0, 0, 0, 0, 0, 164, 252, 22, 0, 184, 251, 4, 0, 3, 46, 46,
    4, 0, 0, 224, 180, 0, 0, 61, 254, 181, 0, 0, 0, 0, 177, 247, 37, 0, 0, 3, 210, 245, 25, 0, 0,
    0, 0, 11, 230, 210, 3, 0, 0, 154, 184, 184, 184, 184, 184, 212, 255, 133, 0, 82, 255, 255, 255,
    255, 178, 0, 166, 181, 0, 0, 0, 0, 0, 0, 0, 0, 0, 179, 255, 255, 255, 255, 82, 0, 0, 0, 0, 134,
    248, 251, 134, 0, 0, 0, 0, 163, 163, 163, 163, 163, 163, 163, 163, 163, 163, 0, 26, 245, 250,
    130, 7, 0, 0, 0, 159, 236, 186, 171, 211, 255, 198, 4, 0, 0, 255, 173, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 127, 254, 237, 185, 200, 249, 233, 22, 0, 0, 0, 0, 0, 0, 0, 0, 173, 255, 0, 0, 16, 200,
    253, 193, 176, 238, 229, 48, 0, 0, 0, 0, 0, 28, 232, 234, 134, 116, 163, 147, 0, 0, 0, 0, 47,
    124, 177, 200, 213, 245, 255, 235, 3, 0, 255, 173, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 24, 253, 254,
    31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 194, 255, 112, 0, 224, 204, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 143,
    143, 206, 255, 31, 0, 0, 0, 0, 0, 250, 192, 220, 176, 253, 178, 229, 169, 245, 179, 0, 255,
    162, 196, 231, 167, 216, 255, 132, 0, 0, 0, 40, 220, 246, 186, 184, 245, 226, 46, 0, 0, 255,
    164, 211, 221, 171, 233, 253, 104, 0, 0, 0, 58, 239, 250, 186, 193, 237, 201, 255, 0, 174, 208,
    255, 41, 173, 255, 226, 255, 135, 0, 0, 74, 250, 231, 172, 169, 209, 255, 158, 0, 0, 0, 0, 0,
    248, 173, 0, 0, 0, 0, 0, 0, 255, 173, 0, 0, 0, 0, 173, 255, 0, 108, 255, 89, 0, 0, 0, 0, 78,
    255, 107, 0, 115, 255, 53, 0, 0, 47, 51, 5, 0, 33, 255, 117, 0, 0, 180, 254, 59, 0, 0, 48, 251,
    177, 0, 0, 102, 255, 92, 0, 0, 0, 0, 80, 255, 102, 0, 120, 240, 240, 240, 240, 242, 255, 225,
    0, 0, 0, 0, 129, 255, 221, 174, 3, 0, 194, 189, 0, 3, 174, 221, 255, 129, 0, 0, 0, 0, 0, 0,
    127, 235, 234, 146, 9, 0, 0, 60, 107, 4, 0, 0, 0, 237, 218, 0, 0, 150, 255, 61, 61, 255, 150,
    0, 0, 0, 70, 255, 37, 0, 0, 238, 124, 0, 0, 0, 0, 0, 0, 112, 235, 0, 0, 0, 0, 0, 42, 251, 170,
    86, 217, 201, 1, 0, 25, 236, 132, 0, 0, 6, 242, 160, 0, 0, 0, 39, 19, 0, 0, 0, 0, 0, 235, 235,
    0, 0, 0, 0, 6, 186, 238, 45, 0, 0, 0, 45, 238, 186, 6, 0, 0, 0, 0, 66, 20, 0, 170, 167, 0, 21,
    66, 0, 0, 0, 0, 0, 204, 194, 0, 0, 0, 0, 133, 255, 255, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 173,
    255, 255, 163, 0, 0, 0, 0, 0, 0, 0, 0, 55, 255, 111, 0, 0, 155, 250, 53, 0, 0, 53, 250, 155, 0,
    0, 3, 151, 253, 209, 115, 255, 122, 0, 0, 0, 0, 62, 166, 15, 0, 0, 0, 159, 255, 51, 0, 5, 104,
    7, 0, 0, 0, 117, 255, 79, 0, 0, 0, 0, 0, 151, 250, 26, 0, 0, 0, 0, 0, 0, 143, 255, 15, 0, 0, 0,
    0, 0, 0, 0, 0, 76, 255, 138, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 240, 172, 0, 0, 1, 232,
    204, 3, 0, 0, 3, 205, 239, 2, 0, 63, 255, 160, 0, 0, 0, 30, 247, 178, 0, 0, 131, 255, 255, 126,
    0, 131, 255, 255, 126, 0, 0, 0, 0, 4, 114, 243, 246, 123, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 5, 120, 245, 243, 115, 4, 0, 0, 0, 0, 3, 131, 115, 0, 0, 0, 21, 244, 190, 0, 130, 238, 91,
    2, 0, 0, 0, 4, 141, 255, 129, 0, 0, 0, 0, 0, 0, 215, 181, 202, 215, 0, 0, 0, 0, 0, 20, 255,
    168, 0, 0, 0, 30, 233, 237, 1, 0, 0, 116, 255, 162, 10, 0, 0, 0, 32, 45, 0, 0, 87, 255, 102, 0,
    0, 0, 85, 250, 220, 17, 0, 0, 122, 255, 61, 0, 0, 0, 0, 0, 0, 0, 61, 255, 128, 0, 0, 0, 0, 0,
    0, 0, 0, 5, 205, 245, 79, 0, 0, 0, 11, 102, 3, 0, 87, 255, 102, 0, 0, 0, 0, 102, 255, 87, 0, 0,
    0, 0, 224, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 204, 240, 0, 230, 214, 0, 0, 0, 147, 255, 107,
    0, 0, 0, 71, 255, 112, 0, 0, 0, 0, 0, 0, 0, 0, 191, 214, 255, 87, 0, 0, 86, 249, 217, 191, 0,
    0, 87, 255, 203, 234, 7, 0, 0, 71, 255, 87, 0, 0, 19, 242, 210, 17, 0, 0, 17, 209, 241, 14, 0,
    0, 173, 255, 10, 0, 0, 2, 123, 255, 169, 0, 0, 0, 24, 246, 208, 17, 0, 0, 17, 209, 242, 15, 0,
    0, 245, 194, 0, 0, 0, 5, 165, 255, 96, 0, 0, 0, 79, 255, 152, 0, 0, 0, 0, 57, 105, 0, 0, 0, 0,
    0, 0, 0, 230, 214, 0, 0, 0, 0, 0, 0, 143, 255, 46, 0, 0, 0, 0, 51, 255, 143, 0, 0, 188, 252,
    21, 0, 0, 0, 0, 5, 239, 188, 0, 0, 148, 255, 31, 0, 38, 255, 255, 47, 0, 6, 252, 142, 0, 0, 0,
    158, 255, 71, 0, 0, 68, 255, 127, 0, 0, 0, 0, 73, 255, 149, 0, 0, 0, 0, 122, 255, 72, 0, 0, 0,
    0, 0, 0, 0, 0, 5, 205, 234, 21, 0, 82, 255, 97, 36, 36, 25, 0, 111, 254, 50, 0, 0, 0, 0, 0, 0,
    0, 0, 25, 36, 36, 97, 255, 82, 0, 0, 0, 39, 249, 137, 146, 249, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 26, 136, 239, 213, 22, 0, 0, 14, 3, 0, 0, 0, 135, 255, 88, 0, 0, 255, 173, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 78, 255, 186, 15, 0, 0, 29, 88, 0, 0, 0, 0, 0, 0, 0, 0, 0, 173, 255, 0,
    0, 155, 253, 75, 0, 0, 28, 226, 195, 0, 0, 0, 0, 0, 121, 255, 66, 0, 0, 0, 0, 0, 0, 0, 86, 247,
    199, 135, 161, 243, 151, 49, 14, 0, 0, 255, 173, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 176, 180, 5,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 109, 203, 51, 0, 224, 204, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 143,
    255, 31, 0, 0, 0, 0, 0, 250, 208, 14, 0, 213, 238, 40, 0, 157, 233, 0, 255, 255, 133, 2, 0, 13,
    234, 218, 0, 0, 0, 188, 245, 52, 0, 0, 49, 243, 196, 0, 0, 255, 255, 114, 0, 0, 28, 233, 236,
    8, 0, 0, 206, 250, 48, 0, 0, 57, 239, 255, 0, 0, 61, 255, 171, 235, 55, 0, 255, 118, 0, 0, 203,
    242, 16, 0, 0, 0, 59, 45, 0, 0, 0, 0, 0, 255, 173, 0, 0, 0, 0, 0, 0, 255, 173, 0, 0, 0, 0, 173,
    255, 0, 21, 250, 175, 0, 0, 0, 0, 163, 249, 20, 0, 69, 255, 90, 0, 13, 253, 255, 55, 0, 69,
    255, 73, 0, 0, 24, 235, 211, 5, 3, 201, 232, 22, 0, 0, 19, 250, 175, 0, 0, 0, 0, 162, 250, 19,
    0, 0, 0, 0, 0, 0, 138, 255, 93, 0, 0, 0, 2, 240, 189, 5, 0, 0, 0, 194, 189, 0, 0, 0, 5, 189,
    240, 2, 0, 0, 0, 0, 101, 253, 170, 149, 250, 207, 46, 62, 228, 181, 1, 0, 0, 0, 232, 213, 0, 0,
    135, 255, 45, 45, 255, 135, 0, 22, 112, 167, 255, 118, 112, 118, 255, 166, 105, 0, 0, 0, 64,
    163, 237, 254, 205, 136, 29, 0, 0, 129, 252, 9, 0, 100, 255, 37, 0, 179, 207, 6, 0, 0, 39, 255,
    132, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 219, 219, 0, 0, 0, 0, 139, 253, 67, 0, 0, 0, 0, 0, 67, 253,
    139, 0, 0, 0, 1, 227, 247, 161, 186, 184, 163, 248, 227, 1, 0, 160, 235, 235, 251, 250, 235,
    235, 160, 0, 36, 254, 255, 84, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 79, 245, 244, 73, 0, 0, 0, 0, 0,
    0, 0, 0, 178, 233, 10, 0, 12, 242, 189, 0, 0, 0, 0, 189, 242, 12, 0, 0, 152, 112, 4, 56, 255,
    122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 73, 255, 114, 0, 0, 0, 0, 0, 0, 0, 39, 255, 138, 0, 0, 0, 0,
    16, 244, 168, 0, 0, 0, 0, 0, 0, 0, 143, 255, 15, 0, 0, 0, 0, 0, 0, 0, 0, 196, 236, 7, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110, 255, 61, 0, 0, 21, 255, 146, 0, 0, 0, 0, 147, 255, 21, 0,
    142, 255, 57, 0, 0, 0, 0, 174, 254, 13, 0, 38, 208, 205, 35, 0, 38, 208, 205, 35, 0, 0, 0, 56,
    208, 255, 181, 34, 0, 0, 0, 0, 9, 10, 10, 10, 10, 10, 10, 9, 0, 0, 0, 0, 32, 178, 255, 208, 56,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 234, 219, 0, 2, 26, 0, 0, 0, 0, 0, 0, 3, 207, 224, 2, 0, 0, 0, 0,
    44, 255, 102, 124, 255, 42, 0, 0, 0, 0, 20, 255, 168, 0, 0, 0, 0, 164, 255, 36, 0, 15, 246,
    224, 6, 0, 0, 0, 0, 0, 0, 0, 0, 87, 255, 102, 0, 0, 0, 0, 131, 255, 117, 0, 0, 122, 255, 61, 0,
    0, 0, 0, 0, 0, 0, 61, 255, 128, 0, 0, 0, 0, 0, 0, 0, 0, 99, 255, 138, 0, 0, 0, 0, 0, 0, 0, 0,
    87, 255, 102, 0, 0, 0, 0, 102, 255, 87, 0, 0, 0, 0, 224, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    204, 240, 0, 230, 214, 0, 0, 107, 255, 145, 0, 0, 0, 0, 71, 255, 112, 0, 0, 0, 0, 0, 0, 0, 0,
    205, 174, 236, 147, 0, 0, 144, 198, 204, 205, 0, 0, 87, 255, 118, 255, 90, 0, 0, 71, 255, 87,
    0, 0, 122, 255, 81, 0, 0, 0, 0, 80, 255, 110, 0, 0, 173, 255, 10, 0, 0, 0, 1, 227, 243, 2, 0,
    0, 132, 255, 78, 0, 0, 0, 0, 80, 255, 114, 0, 0, 245, 194, 0, 0, 0, 0, 51, 255, 159, 0, 0, 0,
    122, 255, 78, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 230, 214, 0, 0, 0, 0, 0, 0, 143, 255, 46,
    0, 0, 0, 0, 51, 255, 143, 0, 0, 103, 255, 98, 0, 0, 0, 0, 70, 255, 103, 0, 0, 113, 255, 62, 0,
    79, 249, 255, 86, 0, 39, 255, 103, 0, 0, 0, 20, 236, 212, 4, 4, 211, 218, 7, 0, 0, 0, 0, 0,
    188, 250, 35, 0, 0, 19, 240, 187, 0, 0, 0, 0, 0, 0, 0, 0, 0, 124, 255, 85, 0, 0, 82, 255, 71,
    0, 0, 0, 0, 10, 233, 174, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 255, 82, 0, 0, 0, 187, 232, 14,
    18, 237, 187, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 96, 1, 0, 0, 0, 0, 0, 0,
    0, 47, 255, 134, 0, 0, 255, 173, 0, 0, 0, 0, 0, 0, 0, 0, 0, 198, 249, 31, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 173, 255, 0, 17, 250, 172, 0, 0, 0, 0, 112, 255, 48, 0, 0, 0, 0, 157, 255,
    21, 0, 0, 0, 0, 0, 0, 31, 251, 179, 1, 0, 0, 73, 255, 97, 0, 0, 0, 255, 173, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 204, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 143, 255, 31, 0, 0, 0, 0, 0, 250, 143, 0, 0, 200, 199, 0, 0, 144, 250, 0, 255,
    188, 0, 0, 0, 0, 189, 253, 4, 0, 51, 255, 158, 0, 0, 0, 0, 155, 255, 55, 0, 255, 183, 0, 0, 0,
    0, 144, 255, 77, 0, 57, 255, 156, 0, 0, 0, 0, 173, 255, 0, 0, 61, 255, 255, 59, 0, 0, 255, 101,
    0, 0, 220, 243, 18, 0, 0, 0, 0, 0, 0, 0, 58, 138, 138, 255, 217, 138, 138, 138, 41, 0, 0, 255,
    173, 0, 0, 0, 0, 173, 255, 0, 0, 178, 247, 14, 0, 0, 7, 240, 177, 0, 0, 23, 255, 126, 0, 64,
    251, 235, 104, 0, 106, 255, 29, 0, 0, 0, 85, 255, 120, 109, 255, 77, 0, 0, 0, 0, 180, 246, 12,
    0, 0, 6, 239, 180, 0, 0, 0, 0, 0, 0, 75, 254, 158, 0, 0, 0, 0, 7, 254, 142, 0, 0, 0, 0, 194,
    189, 0, 0, 0, 0, 142, 254, 7, 0, 0, 0, 5, 205, 134, 0, 0, 63, 229, 255, 255, 222, 21, 0, 0, 0,
    0, 226, 208, 0, 0, 121, 255, 30, 30, 255, 121, 0, 45, 224, 242, 249, 224, 224, 232, 255, 229,
    211, 0, 0, 82, 250, 236, 199, 244, 155, 235, 241, 65, 0, 140, 248, 8, 0, 85, 255, 48, 99, 248,
    45, 0, 0, 0, 6, 231, 215, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 203, 203, 0, 0, 0, 50, 252, 171, 0, 0,
    0, 0, 0, 0, 0, 171, 252, 50, 0, 0, 2, 59, 117, 175, 253, 254, 176, 117, 59, 2, 0, 83, 122, 122,
    228, 223, 122, 122, 83, 0, 42, 255, 237, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 14, 0, 0, 0,
    0, 0, 0, 0, 0, 47, 254, 119, 0, 0, 65, 255, 121, 0, 0, 0, 0, 121, 255, 65, 0, 0, 0, 0, 0, 56,
    255, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 91, 255, 118, 0, 0, 0, 0, 0, 0, 0, 70, 255, 97, 0, 0,
    0, 0, 115, 255, 61, 0, 0, 0, 0, 0, 0, 0, 143, 255, 15, 0, 0, 0, 0, 0, 0, 0, 16, 252, 151, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 221, 205, 0, 0, 0, 5, 246, 200, 1, 0, 0, 0, 205, 226, 1, 0,
    167, 255, 29, 0, 0, 0, 0, 138, 255, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 152, 253, 227, 83,
    0, 0, 0, 0, 0, 0, 224, 255, 255, 255, 255, 255, 255, 224, 0, 0, 0, 0, 0, 0, 80, 225, 253, 151,
    13, 0, 0, 0, 0, 0, 0, 0, 106, 255, 148, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 114, 255, 62, 0, 0, 0, 0,
    127, 253, 25, 45, 255, 125, 0, 0, 0, 0, 20, 255, 168, 0, 0, 0, 0, 188, 253, 17, 0, 94, 255,
    112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 255, 102, 0, 0, 0, 0, 33, 255, 199, 0, 0, 122, 255, 61, 0,
    0, 0, 0, 0, 0, 0, 61, 255, 128, 0, 0, 0, 0, 0, 0, 0, 0, 187, 255, 26, 0, 0, 0, 0, 0, 0, 0, 0,
    87, 255, 102, 0, 0, 0, 0, 102, 255, 87, 0, 0, 0, 0, 224, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    204, 240, 0, 230, 214, 0, 70, 251, 180, 3, 0, 0, 0, 0, 71, 255, 112, 0, 0, 0, 0, 0, 0, 0, 0,
    220, 171, 172, 208, 0, 0, 202, 137, 201, 220, 0, 0, 87, 255, 53, 229, 193, 0, 0, 71, 255, 87,
    0, 0, 191, 247, 9, 0, 0, 0, 0, 9, 246, 194, 0, 0, 173, 255, 10, 0, 0, 0, 0, 194, 255, 24, 0, 0,
    201, 244, 7, 0, 0, 0, 0, 9, 246, 199, 0, 0, 245, 194, 0, 0, 0, 0, 43, 255, 171, 0, 0, 0, 84,
    255, 168, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 230, 214, 0, 0, 0, 0, 0, 0, 143, 255, 46, 0,
    0, 0, 0, 51, 255, 143, 0, 0, 22, 252, 177, 0, 0, 0, 0, 150, 252, 22, 0, 0, 77, 255, 93, 0, 121,
    209, 230, 125, 0, 74, 255, 65, 0, 0, 0, 0, 99, 255, 107, 106, 255, 68, 0, 0, 0, 0, 0, 0, 50,
    253, 164, 0, 0, 140, 253, 49, 0, 0, 0, 0, 0, 0, 0, 0, 46, 249, 170, 0, 0, 0, 82, 255, 71, 0, 0,
    0, 0, 0, 119, 253, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 255, 82, 0, 0, 86, 255, 101, 0, 0,
    108, 255, 86, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 52, 81, 101,
    123, 255, 143, 0, 0, 255, 173, 15, 120, 176, 152, 75, 0, 0, 0, 13, 254, 202, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 43, 152, 182, 142, 41, 173, 255, 0, 76, 255, 111, 0, 0, 0, 0, 64, 255, 86, 0, 0, 0,
    0, 158, 255, 20, 0, 0, 0, 0, 0, 0, 93, 255, 93, 0, 0, 0, 3, 246, 184, 0, 0, 0, 255, 173, 4, 97,
    160, 170, 107, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 204, 0,
    0, 0, 0, 83, 138, 76, 0, 0, 0, 0, 0, 143, 255, 31, 0, 0, 0, 0, 0, 250, 143, 0, 0, 199, 199, 0,
    0, 143, 250, 0, 255, 173, 0, 0, 0, 0, 172, 255, 5, 0, 96, 255, 101, 0, 0, 0, 0, 100, 255, 98,
    0, 255, 173, 0, 0, 0, 0, 96, 255, 107, 0, 100, 255, 111, 0, 0, 0, 0, 173, 255, 0, 0, 61, 255,
    196, 0, 0, 0, 10, 4, 0, 0, 125, 255, 230, 131, 48, 0, 0, 0, 0, 0, 84, 199, 199, 255, 237, 199,
    199, 199, 37, 0, 0, 255, 173, 0, 0, 0, 0, 173, 255, 0, 0, 86, 255, 92, 0, 0, 77, 255, 84, 0, 0,
    0, 232, 163, 0, 116, 211, 184, 154, 0, 142, 240, 1, 0, 0, 0, 0, 165, 245, 242, 153, 0, 0, 0, 0,
    0, 91, 255, 86, 0, 0, 71, 255, 91, 0, 0, 0, 0, 0, 30, 236, 212, 10, 0, 0, 0, 0, 0, 235, 164, 0,
    0, 0, 0, 194, 189, 0, 0, 0, 0, 164, 235, 0, 0, 0, 0, 0, 3, 4, 0, 0, 0, 17, 86, 87, 12, 0, 0, 0,
    0, 0, 221, 203, 0, 0, 46, 107, 8, 8, 107, 46, 0, 0, 0, 170, 188, 0, 0, 83, 255, 20, 0, 0, 0,
    218, 230, 16, 112, 235, 0, 19, 125, 7, 0, 83, 255, 74, 2, 154, 243, 37, 241, 119, 0, 0, 0, 0,
    0, 65, 231, 216, 129, 102, 102, 102, 102, 102, 49, 0, 0, 81, 81, 0, 0, 0, 166, 254, 46, 0, 0,
    0, 0, 0, 0, 0, 46, 254, 166, 0, 0, 0, 0, 0, 119, 237, 229, 139, 0, 0, 0, 0, 0, 0, 0, 204, 194,
    0, 0, 0, 0, 103, 255, 135, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    170, 237, 13, 0, 0, 89, 255, 94, 0, 12, 13, 0, 94, 255, 89, 0, 0, 0, 0, 0, 56, 255, 122, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 149, 255, 63, 0, 0, 0, 0, 0, 1, 51, 216, 197, 5, 0, 0, 0, 3, 222, 208,
    0, 0, 106, 119, 0, 0, 0, 0, 143, 255, 81, 147, 163, 123, 25, 0, 0, 0, 68, 255, 110, 72, 178,
    229, 189, 115, 4, 0, 0, 0, 0, 0, 0, 0, 81, 255, 95, 0, 0, 0, 0, 128, 255, 175, 44, 4, 140, 244,
    66, 0, 0, 148, 255, 47, 0, 0, 0, 0, 129, 255, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 61, 255,
    166, 13, 0, 0, 0, 0, 0, 0, 0, 90, 102, 102, 102, 102, 102, 102, 90, 0, 0, 0, 0, 0, 0, 0, 12,
    162, 255, 66, 0, 0, 0, 0, 0, 1, 124, 254, 187, 13, 0, 0, 0, 61, 159, 199, 180, 107, 13, 0, 61,
    255, 116, 0, 0, 0, 0, 210, 199, 0, 0, 222, 208, 0, 0, 0, 0, 20, 255, 168, 0, 0, 10, 100, 250,
    140, 0, 0, 150, 255, 67, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 255, 102, 0, 0, 0, 0, 0, 234, 237, 0,
    0, 122, 255, 61, 0, 0, 0, 0, 0, 0, 0, 61, 255, 128, 0, 0, 0, 0, 0, 0, 0, 0, 242, 239, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 87, 255, 102, 0, 0, 0, 0, 102, 255, 87, 0, 0, 0, 0, 224, 224, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 204, 240, 0, 230, 214, 41, 240, 208, 13, 0, 0, 0, 0, 0, 71, 255, 112, 0, 0,
    0, 0, 0, 0, 0, 0, 234, 167, 107, 253, 16, 10, 250, 76, 192, 234, 0, 0, 87, 255, 57, 131, 255,
    43, 0, 71, 255, 87, 0, 0, 239, 222, 0, 0, 0, 0, 0, 0, 221, 224, 0, 0, 173, 255, 10, 0, 0, 0, 0,
    224, 242, 1, 0, 0, 249, 217, 0, 0, 0, 0, 0, 0, 221, 229, 0, 0, 245, 194, 0, 0, 0, 0, 120, 255,
    110, 0, 0, 0, 6, 199, 255, 203, 89, 13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 230, 214, 0, 0, 0, 0, 0,
    0, 143, 255, 46, 0, 0, 0, 0, 51, 255, 143, 0, 0, 0, 188, 246, 11, 0, 0, 2, 229, 188, 0, 0, 0,
    42, 255, 125, 0, 162, 170, 192, 165, 0, 109, 255, 26, 0, 0, 0, 0, 1, 196, 235, 235, 166, 0, 0,
    0, 0, 0, 0, 0, 0, 163, 253, 46, 29, 247, 161, 0, 0, 0, 0, 0, 0, 0, 0, 5, 205, 234, 22, 0, 0, 0,
    82, 255, 71, 0, 0, 0, 0, 0, 13, 237, 166, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 255, 82, 0, 12,
    227, 208, 3, 0, 0, 4, 213, 227, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    84, 222, 249, 215, 195, 202, 255, 143, 0, 0, 255, 198, 210, 221, 171, 233, 254, 113, 0, 0, 37,
    255, 173, 0, 0, 0, 0, 0, 0, 0, 0, 0, 68, 247, 242, 177, 192, 238, 224, 255, 0, 99, 255, 248,
    245, 245, 245, 245, 247, 255, 112, 0, 22, 184, 184, 228, 255, 189, 184, 184, 109, 0, 0, 0, 69,
    255, 101, 0, 0, 0, 3, 249, 186, 0, 0, 0, 255, 191, 187, 230, 163, 205, 255, 132, 0, 0, 66, 138,
    138, 138, 138, 47, 0, 0, 0, 0, 0, 22, 138, 138, 138, 138, 138, 124, 0, 224, 204, 0, 0, 0, 94,
    253, 191, 10, 0, 0, 0, 0, 0, 143, 255, 31, 0, 0, 0, 0, 0, 250, 143, 0, 0, 199, 199, 0, 0, 143,
    250, 0, 255, 173, 0, 0, 0, 0, 168, 255, 5, 0, 127, 255, 79, 0, 0, 0, 0, 78, 255, 128, 0, 255,
    173, 0, 0, 0, 0, 78, 255, 130, 0, 128, 255, 81, 0, 0, 0, 0, 173, 255, 0, 0, 61, 255, 116, 0, 0,
    0, 0, 0, 0, 0, 1, 90, 202, 254, 255, 216, 101, 2, 0, 0, 0, 0, 0, 255, 173, 0, 0, 0, 0, 0, 0,
    255, 173, 0, 0, 0, 0, 173, 255, 0, 0, 8, 240, 178, 0, 0, 162, 238, 7, 0, 0, 0, 187, 199, 0,
    169, 162, 139, 203, 0, 179, 196, 0, 0, 0, 0, 0, 45, 255, 255, 47, 0, 0, 0, 0, 0, 12, 245, 169,
    0, 0, 153, 245, 12, 0, 0, 0, 0, 5, 197, 245, 43, 0, 0, 0, 0, 0, 0, 207, 189, 0, 0, 0, 0, 194,
    189, 0, 0, 0, 0, 189, 207, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 215, 198,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 206, 155, 0, 0, 119, 242, 0, 0, 0, 12, 255, 192, 0, 112, 235,
    0, 0, 0, 0, 0, 2, 170, 244, 220, 253, 86, 191, 197, 3, 0, 0, 0, 0, 0, 19, 128, 248, 248, 240,
    240, 253, 250, 240, 115, 0, 0, 0, 0, 0, 0, 9, 249, 184, 0, 0, 0, 0, 0, 0, 0, 0, 0, 184, 249, 9,
    0, 0, 0, 79, 253, 110, 85, 255, 117, 0, 0, 0, 0, 0, 0, 204, 194, 0, 0, 0, 0, 164, 250, 28, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 253, 128, 0, 0, 0, 113, 255,
    77, 21, 240, 241, 24, 77, 255, 113, 0, 0, 0, 0, 0, 56, 255, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15,
    231, 209, 2, 0, 0, 0, 0, 138, 244, 254, 163, 11, 0, 0, 0, 0, 78, 255, 101, 0, 0, 183, 199, 0,
    0, 0, 0, 143, 255, 225, 175, 187, 250, 234, 47, 0, 0, 89, 255, 168, 237, 168, 120, 186, 255,
    162, 0, 0, 0, 0, 0, 0, 0, 194, 232, 7, 0, 0, 0, 0, 0, 109, 254, 255, 234, 245, 53, 0, 0, 0, 80,
    255, 135, 0, 0, 0, 58, 243, 255, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 242, 244, 116, 4, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 113, 242, 243, 45, 0, 0, 0, 0, 0,
    136, 255, 138, 3, 0, 0, 0, 85, 251, 184, 121, 169, 255, 87, 0, 29, 255, 140, 0, 0, 0, 39, 255,
    120, 0, 0, 143, 255, 36, 0, 0, 0, 20, 255, 248, 235, 235, 253, 242, 97, 0, 0, 0, 174, 255, 38,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 255, 102, 0, 0, 0, 0, 0, 215, 254, 5, 0, 122, 255, 197, 178,
    178, 178, 178, 64, 0, 0, 61, 255, 194, 133, 133, 133, 133, 119, 0, 0, 11, 255, 211, 0, 0, 0,
    113, 138, 138, 138, 107, 0, 87, 255, 249, 245, 245, 245, 245, 249, 255, 87, 0, 0, 0, 0, 224,
    224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 204, 240, 0, 230, 215, 213, 255, 48, 0, 0, 0, 0, 0, 0,
    71, 255, 112, 0, 0, 0, 0, 0, 0, 0, 0, 249, 156, 42, 255, 74, 62, 253, 17, 182, 249, 0, 0, 87,
    255, 66, 31, 253, 146, 0, 71, 255, 87, 0, 4, 254, 202, 0, 0, 0, 0, 0, 0, 201, 250, 2, 0, 173,
    255, 10, 0, 0, 0, 98, 255, 175, 0, 0, 14, 255, 197, 0, 0, 0, 0, 0, 0, 201, 253, 5, 0, 245, 210,
    66, 67, 88, 153, 250, 194, 6, 0, 0, 0, 0, 12, 138, 246, 255, 247, 168, 62, 0, 0, 0, 0, 0, 0, 0,
    0, 230, 214, 0, 0, 0, 0, 0, 0, 143, 255, 46, 0, 0, 0, 0, 51, 255, 143, 0, 0, 0, 103, 255, 81,
    0, 0, 56, 255, 103, 0, 0, 0, 8, 253, 156, 0, 203, 131, 153, 204, 0, 145, 242, 1, 0, 0, 0, 0, 0,
    58, 255, 255, 39, 0, 0, 0, 0, 0, 0, 0, 0, 31, 247, 179, 158, 246, 29, 0, 0, 0, 0, 0, 0, 0, 0,
    125, 255, 86, 0, 0, 0, 0, 82, 255, 71, 0, 0, 0, 0, 0, 0, 128, 252, 38, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 71, 255, 82, 0, 45, 117, 48, 0, 0, 0, 0, 51, 117, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 54, 250, 193, 24, 0, 0, 36, 255, 143, 0, 0, 255, 255, 114, 0, 0, 26, 227,
    244, 16, 0, 19, 255, 185, 0, 0, 0, 0, 0, 0, 0, 0, 8, 225, 237, 41, 0, 0, 57, 239, 255, 0, 80,
    255, 151, 82, 82, 82, 82, 82, 82, 35, 0, 19, 158, 158, 218, 255, 166, 158, 158, 79, 0, 0, 0,
    11, 234, 206, 14, 0, 0, 104, 255, 112, 0, 0, 0, 255, 255, 131, 2, 0, 2, 215, 218, 0, 0, 98,
    204, 204, 221, 255, 87, 0, 0, 0, 0, 0, 33, 204, 204, 204, 204, 244, 229, 0, 224, 204, 0, 0, 89,
    252, 192, 11, 0, 0, 0, 0, 0, 0, 143, 255, 31, 0, 0, 0, 0, 0, 250, 143, 0, 0, 199, 199, 0, 0,
    143, 250, 0, 255, 173, 0, 0, 0, 0, 168, 255, 5, 0, 113, 255, 89, 0, 0, 0, 0, 90, 255, 111, 0,
    255, 173, 0, 0, 0, 0, 89, 255, 115, 0, 118, 255, 88, 0, 0, 0, 0, 173, 255, 0, 0, 61, 255, 112,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 113, 216, 255, 134, 0, 0, 0, 0, 0, 255, 173, 0, 0, 0, 0, 0,
    0, 255, 173, 0, 0, 0, 0, 173, 255, 0, 0, 0, 156, 248, 16, 7, 240, 153, 0, 0, 0, 0, 141, 236, 0,
    221, 113, 93, 247, 5, 215, 152, 0, 0, 0, 0, 1, 187, 226, 240, 197, 3, 0, 0, 0, 0, 0, 169, 243,
    9, 3, 233, 169, 0, 0, 0, 0, 0, 138, 255, 97, 0, 0, 0, 0, 0, 0, 0, 179, 214, 0, 0, 0, 0, 194,
    189, 0, 0, 0, 0, 214, 179, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 210, 193,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 241, 122, 0, 0, 154, 209, 0, 0, 0, 0, 224, 249, 32, 112, 235,
    0, 0, 0, 0, 0, 0, 2, 52, 92, 28, 112, 244, 36, 0, 0, 0, 0, 0, 26, 220, 220, 68, 7, 0, 0, 219,
    173, 0, 0, 0, 0, 0, 0, 0, 0, 64, 255, 125, 0, 0, 0, 0, 0, 0, 0, 0, 0, 125, 255, 64, 0, 0, 0,
    169, 216, 5, 0, 182, 175, 3, 0, 0, 0, 0, 0, 94, 89, 0, 0, 0, 0, 123, 110, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 162, 242, 18, 0, 0, 0, 118, 255, 73, 36, 254, 254,
    40, 73, 255, 118, 0, 0, 0, 0, 0, 56, 255, 122, 0, 0, 0, 0, 0, 0, 0, 0, 0, 162, 251, 58, 0, 0,
    0, 0, 0, 74, 115, 158, 243, 198, 24, 0, 0, 0, 188, 239, 11, 0, 0, 193, 199, 0, 0, 0, 0, 14, 24,
    1, 0, 0, 55, 251, 192, 0, 0, 102, 255, 236, 41, 0, 0, 1, 179, 255, 44, 0, 0, 0, 0, 0, 52, 255,
    128, 0, 0, 0, 0, 0, 59, 219, 178, 89, 179, 252, 234, 68, 0, 0, 2, 189, 254, 163, 116, 173, 233,
    202, 255, 32, 0, 0, 0, 0, 0, 0, 0, 33, 30, 0, 0, 0, 28, 175, 255, 208, 56, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 207, 255, 178, 30, 0, 0, 0, 0, 0, 30, 253, 175, 0, 0, 0,
    0, 3, 220, 187, 1, 0, 36, 255, 87, 0, 16, 255, 164, 0, 0, 0, 122, 255, 40, 0, 0, 65, 255, 119,
    0, 0, 0, 20, 255, 213, 133, 133, 148, 203, 243, 130, 0, 0, 179, 255, 32, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 87, 255, 102, 0, 0, 0, 0, 0, 217, 254, 6, 0, 122, 255, 201, 184, 184, 184, 184, 66, 0, 0,
    61, 255, 240, 224, 224, 224, 224, 202, 0, 0, 13, 255, 203, 0, 0, 0, 170, 235, 235, 254, 199, 0,
    87, 255, 172, 117, 117, 117, 117, 172, 255, 87, 0, 0, 0, 0, 224, 224, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 204, 240, 0, 230, 214, 103, 255, 182, 2, 0, 0, 0, 0, 0, 71, 255, 112, 0, 0, 0, 0, 0,
    0, 0, 8, 255, 144, 1, 231, 135, 120, 209, 0, 168, 255, 8, 0, 87, 255, 71, 0, 181, 239, 10, 67,
    255, 87, 0, 6, 254, 196, 0, 0, 0, 0, 0, 0, 197, 253, 6, 0, 173, 255, 133, 128, 144, 194, 255,
    221, 21, 0, 0, 19, 255, 191, 0, 0, 0, 0, 0, 0, 197, 255, 12, 0, 245, 255, 255, 255, 255, 221,
    116, 5, 0, 0, 0, 0, 0, 0, 0, 14, 98, 194, 254, 254, 149, 0, 0, 0, 0, 0, 0, 0, 230, 214, 0, 0,
    0, 0, 0, 0, 143, 255, 46, 0, 0, 0, 0, 51, 255, 143, 0, 0, 0, 21, 251, 161, 0, 0, 136, 251, 21,
    0, 0, 0, 0, 226, 187, 1, 243, 92, 114, 242, 1, 180, 204, 0, 0, 0, 0, 0, 0, 141, 250, 253, 141,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 138, 255, 253, 135, 0, 0, 0, 0, 0, 0, 0, 0, 46, 249, 171, 0, 0, 0,
    0, 0, 82, 255, 71, 0, 0, 0, 0, 0, 0, 18, 242, 158, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 255, 82, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 131,
    255, 69, 0, 0, 0, 36, 255, 143, 0, 0, 255, 183, 0, 0, 0, 0, 126, 255, 98, 0, 0, 237, 225, 2, 0,
    0, 0, 0, 0, 0, 0, 78, 255, 137, 0, 0, 0, 0, 173, 255, 0, 44, 255, 139, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 158, 255, 20, 0, 0, 0, 0, 0, 0, 0, 36, 230, 232, 177, 196, 254, 162, 3, 0, 0, 0, 255,
    187, 0, 0, 0, 0, 171, 253, 4, 0, 0, 0, 0, 87, 255, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 199, 229,
    0, 224, 204, 0, 85, 251, 194, 12, 0, 0, 0, 0, 0, 0, 0, 143, 255, 31, 0, 0, 0, 0, 0, 250, 143,
    0, 0, 199, 199, 0, 0, 143, 250, 0, 255, 173, 0, 0, 0, 0, 168, 255, 5, 0, 79, 255, 121, 0, 0, 0,
    0, 121, 255, 77, 0, 255, 173, 0, 0, 0, 0, 120, 255, 85, 0, 92, 255, 116, 0, 0, 0, 0, 173, 255,
    0, 0, 61, 255, 112, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 239, 234, 0, 0, 0, 0, 0, 255, 173,
    0, 0, 0, 0, 0, 0, 255, 173, 0, 0, 0, 0, 173, 255, 0, 0, 0, 63, 255, 95, 77, 255, 60, 0, 0, 0,
    0, 95, 255, 37, 254, 64, 47, 255, 50, 249, 108, 0, 0, 0, 0, 112, 255, 73, 103, 255, 120, 0, 0,
    0, 0, 0, 80, 255, 81, 63, 255, 81, 0, 0, 0, 0, 76, 254, 162, 0, 0, 0, 0, 0, 0, 0, 0, 160, 235,
    0, 0, 0, 0, 194, 189, 0, 0, 0, 0, 235, 162, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 204, 188, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 255, 89, 0, 0, 190, 175, 0, 0, 0, 0, 80, 251,
    237, 197, 235, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 246, 106, 18, 67, 43, 0, 0, 0, 151, 253, 41, 0,
    0, 0, 0, 219, 173, 0, 0, 0, 0, 0, 0, 0, 0, 101, 255, 89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 89, 255,
    101, 0, 0, 0, 0, 26, 0, 0, 16, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 34, 251, 136, 0, 0, 0, 0, 94, 255, 91, 0, 55, 55,
    0, 91, 255, 94, 0, 0, 0, 0, 0, 56, 255, 122, 0, 0, 0, 0, 0, 0, 0, 0, 117, 255, 109, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 54, 253, 171, 0, 0, 43, 254, 142, 0, 0, 0, 202, 199, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 181, 254, 11, 0, 83, 255, 97, 0, 0, 0, 0, 94, 255, 106, 0, 0, 0, 0, 0, 165, 248, 24, 0, 0,
    0, 0, 34, 238, 177, 0, 0, 0, 51, 221, 248, 38, 0, 0, 9, 136, 220, 225, 164, 52, 186, 236, 3, 0,
    38, 208, 206, 36, 0, 73, 253, 252, 67, 0, 0, 0, 0, 79, 226, 253, 151, 16, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 17, 151, 253, 227, 82, 0, 0, 0, 0, 0, 0, 0, 80, 255, 98, 0, 0, 0, 0, 49,
    255, 101, 0, 0, 36, 255, 87, 0, 7, 255, 181, 0, 0, 0, 205, 216, 0, 0, 0, 4, 237, 202, 0, 0, 0,
    20, 255, 168, 0, 0, 0, 0, 139, 255, 99, 0, 155, 255, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 255,
    102, 0, 0, 0, 0, 1, 246, 239, 0, 0, 122, 255, 61, 0, 0, 0, 0, 0, 0, 0, 61, 255, 128, 0, 0, 0,
    0, 0, 0, 0, 0, 239, 226, 0, 0, 0, 0, 0, 0, 245, 199, 0, 87, 255, 102, 0, 0, 0, 0, 102, 255, 87,
    0, 0, 0, 0, 224, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 204, 240, 0, 230, 214, 0, 163, 255, 126,
    0, 0, 0, 0, 0, 71, 255, 112, 0, 0, 0, 0, 0, 0, 0, 23, 255, 131, 0, 166, 195, 178, 148, 0, 153,
    255, 23, 0, 87, 255, 71, 0, 79, 255, 98, 56, 255, 87, 0, 0, 232, 217, 0, 0, 0, 0, 0, 0, 217,
    231, 0, 0, 173, 255, 226, 224, 204, 173, 101, 7, 0, 0, 0, 3, 253, 212, 0, 0, 0, 0, 0, 0, 217,
    248, 0, 0, 245, 200, 26, 54, 247, 197, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 215, 255, 90, 0,
    0, 0, 0, 0, 0, 230, 214, 0, 0, 0, 0, 0, 0, 143, 255, 46, 0, 0, 0, 0, 51, 255, 143, 0, 0, 0, 0,
    188, 236, 4, 0, 217, 188, 0, 0, 0, 0, 0, 190, 219, 30, 255, 53, 75, 255, 27, 215, 166, 0, 0, 0,
    0, 0, 41, 250, 142, 164, 250, 41, 0, 0, 0, 0, 0, 0, 0, 0, 16, 242, 242, 15, 0, 0, 0, 0, 0, 0,
    0, 5, 205, 234, 22, 0, 0, 0, 0, 0, 82, 255, 71, 0, 0, 0, 0, 0, 0, 0, 136, 250, 32, 0, 0, 0, 0,
    0, 0, 0, 0, 71, 255, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 135, 255, 74, 0, 0, 0, 75, 255, 145, 0, 0, 255, 173, 0, 0, 0, 0, 73, 255,
    131, 0, 0, 148, 255, 92, 0, 0, 0, 0, 7, 0, 0, 135, 255, 77, 0, 0, 0, 0, 173, 255, 0, 2, 208,
    234, 18, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 158, 255, 20, 0, 0, 0, 0, 0, 0, 0, 85, 242, 89, 108, 99,
    48, 0, 0, 0, 0, 0, 255, 173, 0, 0, 0, 0, 168, 255, 5, 0, 0, 0, 0, 87, 255, 87, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 199, 229, 0, 224, 204, 80, 250, 195, 12, 0, 0, 0, 0, 0, 0, 0, 0, 143, 255, 31,
    0, 0, 0, 0, 0, 250, 143, 0, 0, 199, 199, 0, 0, 143, 250, 0, 255, 173, 0, 0, 0, 0, 168, 255, 5,
    0, 16, 238, 210, 2, 0, 0, 2, 210, 234, 12, 0, 255, 202, 6, 0, 0, 2, 208, 246, 20, 0, 37, 254,
    199, 1, 0, 0, 28, 225, 255, 0, 0, 61, 255, 112, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 2, 228,
    234, 0, 0, 0, 0, 0, 255, 173, 0, 0, 0, 0, 0, 0, 241, 192, 0, 0, 0, 29, 224, 255, 0, 0, 0, 2,
    224, 181, 162, 221, 1, 0, 0, 0, 0, 49, 255, 125, 254, 16, 7, 250, 130, 255, 64, 0, 0, 0, 43,
    247, 164, 0, 1, 193, 249, 47, 0, 0, 0, 0, 7, 239, 164, 145, 240, 7, 0, 0, 0, 30, 236, 215, 12,
    0, 0, 0, 0, 0, 0, 0, 18, 221, 212, 0, 0, 0, 0, 194, 189, 0, 0, 0, 0, 208, 227, 28, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 92, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 163, 188, 255,
    195, 173, 173, 242, 222, 173, 35, 0, 0, 0, 61, 205, 255, 255, 196, 78, 0, 0, 0, 0, 0, 0, 4,
    201, 186, 71, 240, 247, 255, 168, 2, 0, 213, 214, 0, 0, 0, 0, 0, 219, 173, 0, 0, 0, 0, 0, 0, 0,
    0, 119, 255, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 255, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    154, 245, 22, 0, 0, 0, 0, 70, 255, 112, 0, 0, 0, 0, 112, 255, 70, 0, 0, 0, 0, 0, 56, 255, 122,
    0, 0, 0, 0, 0, 0, 0, 100, 254, 134, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 214, 241, 1, 0, 133, 255,
    200, 178, 178, 178, 241, 238, 178, 100, 0, 0, 0, 0, 0, 0, 0, 0, 151, 255, 42, 0, 59, 255, 105,
    0, 0, 0, 0, 65, 255, 120, 0, 0, 0, 0, 28, 250, 161, 0, 0, 0, 0, 0, 132, 255, 60, 0, 0, 0, 0,
    86, 255, 143, 0, 0, 0, 0, 0, 0, 0, 19, 239, 168, 0, 0, 131, 255, 255, 127, 0, 140, 255, 255,
    133, 0, 0, 0, 0, 0, 12, 142, 251, 231, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 91, 232, 252,
    145, 13, 0, 0, 0, 0, 0, 0, 0, 0, 42, 117, 38, 0, 0, 0, 0, 78, 255, 72, 0, 0, 36, 255, 87, 0, 7,
    255, 176, 0, 0, 34, 255, 252, 245, 245, 245, 245, 253, 254, 31, 0, 0, 20, 255, 168, 0, 0, 0, 0,
    36, 255, 172, 0, 112, 255, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 255, 102, 0, 0, 0, 0, 26, 255,
    205, 0, 0, 122, 255, 61, 0, 0, 0, 0, 0, 0, 0, 61, 255, 128, 0, 0, 0, 0, 0, 0, 0, 0, 209, 248,
    5, 0, 0, 0, 0, 0, 245, 199, 0, 87, 255, 102, 0, 0, 0, 0, 102, 255, 87, 0, 0, 0, 0, 224, 224, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 212, 233, 0, 230, 214, 0, 11, 212, 253, 70, 0, 0, 0, 0, 71, 255,
    112, 0, 0, 0, 0, 0, 0, 0, 37, 255, 119, 0, 101, 248, 237, 87, 0, 138, 255, 37, 0, 87, 255, 71,
    0, 4, 228, 201, 45, 255, 87, 0, 0, 204, 240, 2, 0, 0, 0, 0, 2, 240, 202, 0, 0, 173, 255, 10, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 221, 237, 1, 0, 0, 0, 0, 2, 240, 213, 0, 0, 245, 194, 0, 0, 123, 255,
    116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 255, 160, 0, 0, 0, 0, 0, 0, 230, 214, 0, 0, 0, 0,
    0, 0, 140, 255, 49, 0, 0, 0, 0, 54, 255, 140, 0, 0, 0, 0, 103, 255, 65, 42, 255, 103, 0, 0, 0,
    0, 0, 155, 248, 73, 254, 14, 37, 255, 69, 248, 127, 0, 0, 0, 0, 0, 186, 237, 18, 32, 247, 186,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 224, 224, 0, 0, 0, 0, 0, 0, 0, 0, 125, 255, 86, 0, 0, 0, 0, 0, 0,
    82, 255, 71, 0, 0, 0, 0, 0, 0, 0, 22, 245, 151, 0, 0, 0, 0, 0, 0, 0, 0, 71, 255, 82, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 254, 199,
    46, 33, 107, 239, 245, 202, 11, 0, 255, 173, 0, 0, 0, 0, 53, 255, 155, 0, 0, 20, 232, 250, 135,
    72, 87, 160, 226, 21, 0, 156, 255, 54, 0, 0, 0, 0, 173, 255, 0, 0, 67, 251, 217, 91, 57, 92,
    192, 166, 0, 0, 0, 0, 0, 158, 255, 20, 0, 0, 0, 0, 0, 0, 0, 190, 226, 5, 0, 0, 0, 0, 0, 0, 0,
    0, 255, 173, 0, 0, 0, 0, 168, 255, 5, 0, 0, 0, 0, 87, 255, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    199, 229, 0, 224, 204, 187, 255, 102, 0, 0, 0, 0, 0, 0, 0, 0, 0, 143, 255, 31, 0, 0, 0, 0, 0,
    250, 143, 0, 0, 199, 199, 0, 0, 143, 250, 0, 255, 173, 0, 0, 0, 0, 168, 255, 5, 0, 0, 118, 255,
    174, 68, 68, 174, 255, 108, 0, 0, 255, 255, 190, 85, 69, 172, 255, 146, 0, 0, 0, 181, 255, 157,
    64, 108, 220, 240, 255, 0, 69, 119, 255, 155, 77, 44, 0, 0, 0, 0, 58, 251, 176, 91, 51, 70,
    175, 255, 148, 0, 0, 0, 0, 0, 255, 173, 0, 0, 0, 0, 0, 0, 186, 252, 91, 40, 104, 222, 226, 255,
    0, 0, 0, 0, 134, 249, 241, 129, 0, 0, 0, 0, 0, 7, 251, 214, 221, 0, 0, 211, 215, 255, 20, 0, 0,
    6, 206, 235, 21, 0, 0, 39, 247, 209, 7, 0, 0, 0, 0, 158, 240, 227, 159, 0, 0, 0, 0, 195, 255,
    156, 112, 112, 112, 112, 104, 0, 118, 195, 246, 219, 66, 0, 0, 0, 0, 194, 189, 0, 0, 0, 0, 57,
    208, 249, 199, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 144, 192, 255, 158, 153, 158, 255, 189, 153, 31, 0, 0, 0, 0, 0, 132, 250, 241, 255, 150,
    0, 0, 0, 0, 0, 125, 239, 33, 228, 180, 14, 80, 255, 90, 0, 210, 203, 0, 0, 0, 0, 0, 219, 173,
    0, 0, 0, 0, 0, 0, 0, 0, 119, 255, 74, 0, 0, 0, 0, 0, 0, 0, 0, 0, 74, 255, 119, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 28, 249, 144, 0, 0, 0, 0, 0, 23, 251, 174, 0, 0, 0, 0, 174, 251, 23, 0, 0, 0,
    0, 0, 56, 255, 122, 0, 0, 0, 0, 0, 0, 93, 253, 139, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 204,
    248, 2, 0, 94, 168, 168, 168, 168, 168, 241, 236, 168, 94, 0, 0, 0, 0, 0, 0, 0, 0, 175, 255,
    26, 0, 23, 252, 144, 0, 0, 0, 0, 89, 255, 95, 0, 0, 0, 0, 136, 255, 51, 0, 0, 0, 0, 0, 168,
    255, 33, 0, 0, 0, 0, 36, 255, 168, 0, 0, 0, 0, 0, 0, 0, 171, 252, 53, 0, 0, 62, 245, 244, 57,
    0, 41, 255, 255, 79, 0, 0, 0, 0, 0, 0, 0, 49, 202, 229, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14,
    225, 205, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 95, 255, 61, 0, 0, 36, 255,
    87, 0, 10, 255, 169, 0, 0, 117, 255, 141, 122, 122, 122, 122, 154, 255, 113, 0, 0, 20, 255,
    168, 0, 0, 0, 0, 28, 255, 181, 0, 30, 254, 201, 0, 0, 0, 0, 0, 0, 0, 0, 0, 87, 255, 102, 0, 0,
    0, 0, 136, 255, 125, 0, 0, 122, 255, 61, 0, 0, 0, 0, 0, 0, 0, 61, 255, 128, 0, 0, 0, 0, 0, 0,
    0, 0, 148, 255, 82, 0, 0, 0, 0, 0, 245, 199, 0, 87, 255, 102, 0, 0, 0, 0, 102, 255, 87, 0, 0,
    0, 0, 224, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 243, 199, 0, 230, 214, 0, 0, 41, 243, 235, 30,
    0, 0, 0, 71, 255, 112, 0, 0, 0, 0, 0, 0, 0, 51, 255, 106, 0, 36, 255, 255, 27, 0, 123, 255, 51,
    0, 87, 255, 71, 0, 0, 129, 255, 83, 255, 87, 0, 0, 140, 255, 62, 0, 0, 0, 0, 61, 255, 134, 0,
    0, 173, 255, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 255, 59, 0, 0, 0, 0, 61, 255, 143, 0, 0, 245,
    194, 0, 0, 5, 210, 247, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 255, 177, 0, 0, 0, 0, 0, 0,
    230, 214, 0, 0, 0, 0, 0, 0, 116, 255, 76, 0, 0, 0, 0, 82, 255, 115, 0, 0, 0, 0, 21, 251, 144,
    123, 251, 21, 0, 0, 0, 0, 0, 119, 255, 139, 229, 0, 4, 249, 136, 255, 89, 0, 0, 0, 0, 82, 255,
    112, 0, 0, 139, 255, 81, 0, 0, 0, 0, 0, 0, 0, 0, 224, 224, 0, 0, 0, 0, 0, 0, 0, 46, 249, 171,
    0, 0, 0, 0, 0, 0, 0, 82, 255, 71, 0, 0, 0, 0, 0, 0, 0, 0, 144, 248, 27, 0, 0, 0, 0, 0, 0, 0,
    71, 255, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 111, 244, 255, 255, 215, 55, 101, 248, 66, 0, 255, 173, 0, 0, 0, 0, 65, 255, 139,
    0, 0, 0, 24, 176, 249, 255, 255, 228, 107, 4, 0, 142, 255, 63, 0, 0, 0, 0, 173, 255, 0, 0, 0,
    57, 212, 254, 255, 253, 199, 68, 0, 0, 0, 0, 0, 158, 255, 20, 0, 0, 0, 0, 0, 0, 0, 138, 255,
    242, 210, 209, 187, 137, 28, 0, 0, 0, 255, 173, 0, 0, 0, 0, 168, 255, 5, 0, 0, 0, 0, 87, 255,
    87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 199, 229, 0, 224, 204, 16, 210, 250, 74, 0, 0, 0, 0, 0, 0, 0,
    0, 143, 255, 31, 0, 0, 0, 0, 0, 250, 143, 0, 0, 199, 199, 0, 0, 143, 250, 0, 255, 173, 0, 0, 0,
    0, 168, 255, 5, 0, 0, 0, 105, 237, 255, 255, 235, 97, 0, 0, 0, 255, 185, 156, 252, 255, 248,
    149, 4, 0, 0, 0, 17, 182, 253, 255, 241, 104, 173, 255, 0, 230, 255, 255, 255, 255, 148, 0, 0,
    0, 0, 2, 99, 222, 254, 255, 255, 235, 132, 4, 0, 0, 0, 0, 0, 255, 173, 0, 0, 0, 0, 0, 0, 42,
    217, 255, 255, 224, 80, 125, 255, 0, 0, 0, 0, 41, 255, 254, 37, 0, 0, 0, 0, 0, 0, 212, 255,
    172, 0, 0, 166, 255, 231, 0, 0, 0, 135, 255, 92, 0, 0, 0, 0, 123, 255, 137, 0, 0, 0, 0, 70,
    255, 255, 70, 0, 0, 0, 0, 245, 255, 255, 255, 255, 255, 255, 212, 0, 118, 196, 246, 220, 71, 0,
    0, 0, 0, 194, 189, 0, 0, 0, 0, 65, 206, 248, 198, 119, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 67, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 122, 244, 1, 0, 35, 255, 69, 0, 0, 0, 0, 0, 0,
    0, 112, 235, 21, 199, 255, 87, 0, 0, 0, 49, 250, 93, 40, 255, 93, 0, 0, 240, 152, 0, 150, 249,
    36, 0, 0, 0, 1, 224, 173, 0, 0, 0, 0, 0, 0, 0, 0, 101, 255, 90, 0, 0, 0, 0, 0, 0, 0, 0, 0, 90,
    255, 101, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 145, 248, 27, 0, 0, 0, 0, 0, 0, 177, 242, 26, 0,
    0, 26, 242, 177, 0, 0, 0, 0, 0, 0, 56, 255, 122, 0, 0, 0, 0, 0, 98, 253, 128, 0, 0, 0, 0, 0, 0,
    23, 65, 0, 0, 0, 0, 41, 249, 188, 0, 0, 0, 0, 0, 0, 0, 0, 214, 199, 0, 0, 0, 0, 57, 4, 0, 0, 0,
    23, 242, 213, 0, 0, 0, 181, 230, 8, 0, 0, 0, 174, 252, 25, 0, 0, 0, 11, 238, 195, 0, 0, 0, 0,
    0, 0, 125, 255, 97, 0, 0, 0, 0, 106, 255, 120, 0, 0, 0, 0, 0, 25, 175, 255, 130, 0, 0, 0, 0,
    16, 14, 0, 0, 48, 255, 233, 8, 0, 0, 0, 0, 0, 0, 0, 0, 3, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 31, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 91, 22, 0, 0, 0, 0, 74, 255, 77, 0, 0, 36, 255,
    87, 0, 18, 255, 149, 0, 0, 200, 228, 1, 0, 0, 0, 0, 10, 245, 196, 0, 0, 20, 255, 168, 0, 0, 0,
    0, 119, 255, 127, 0, 0, 154, 255, 111, 0, 0, 0, 0, 4, 47, 0, 0, 87, 255, 102, 0, 0, 0, 72, 245,
    233, 26, 0, 0, 122, 255, 61, 0, 0, 0, 0, 0, 0, 0, 61, 255, 128, 0, 0, 0, 0, 0, 0, 0, 0, 32,
    250, 201, 13, 0, 0, 0, 0, 245, 199, 0, 87, 255, 102, 0, 0, 0, 0, 102, 255, 87, 0, 0, 0, 0, 224,
    224, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 99, 255, 136, 0, 229, 214, 0, 0, 0, 89, 255, 200, 6, 0, 0,
    71, 255, 112, 0, 0, 0, 0, 0, 0, 0, 66, 255, 93, 0, 0, 125, 122, 0, 0, 109, 255, 66, 0, 87, 255,
    71, 0, 0, 30, 252, 176, 255, 87, 0, 0, 33, 252, 178, 3, 0, 0, 3, 176, 250, 28, 0, 0, 173, 255,
    10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 255, 176, 3, 0, 0, 3, 176, 247, 37, 0, 0, 245, 194, 0, 0, 0,
    54, 252, 200, 3, 0, 0, 0, 72, 79, 0, 0, 0, 0, 0, 128, 255, 124, 0, 0, 0, 0, 0, 0, 230, 214, 0,
    0, 0, 0, 0, 0, 47, 255, 168, 0, 0, 0, 0, 175, 255, 44, 0, 0, 0, 0, 0, 187, 223, 203, 187, 0, 0,
    0, 0, 0, 0, 84, 255, 211, 190, 0, 0, 214, 210, 255, 50, 0, 0, 0, 9, 223, 218, 6, 0, 0, 17, 237,
    223, 9, 0, 0, 0, 0, 0, 0, 0, 224, 224, 0, 0, 0, 0, 0, 0, 5, 206, 235, 22, 0, 0, 0, 0, 0, 0, 0,
    82, 255, 71, 0, 0, 0, 0, 0, 0, 0, 0, 27, 248, 143, 0, 0, 0, 0, 0, 0, 0, 71, 255, 82, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 43,
    20, 0, 0, 0, 19, 3, 0, 255, 173, 0, 0, 0, 0, 100, 255, 107, 0, 0, 0, 0, 0, 5, 41, 23, 0, 0, 0,
    0, 114, 255, 95, 0, 0, 0, 0, 173, 255, 0, 0, 0, 0, 0, 15, 45, 11, 0, 0, 0, 0, 0, 0, 0, 158,
    255, 20, 0, 0, 0, 0, 0, 0, 0, 4, 97, 152, 163, 165, 209, 253, 234, 47, 0, 0, 255, 173, 0, 0, 0,
    0, 168, 255, 5, 0, 0, 0, 0, 87, 255, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 199, 229, 0, 224, 204,
    0, 27, 225, 245, 57, 0, 0, 0, 0, 0, 0, 0, 143, 255, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 32, 31, 0, 0, 0, 0, 0, 255, 173, 0, 11, 43,
    7, 0, 0, 0, 0, 0, 0, 0, 14, 40, 3, 0, 173, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13,
    47, 30, 4, 0, 0, 0, 0, 0, 0, 0, 234, 193, 0, 0, 0, 0, 0, 0, 0, 3, 35, 29, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 197, 222, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 222, 214, 0, 0, 0,
    0, 194, 189, 0, 0, 0, 0, 217, 227, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51,
    255, 255, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 154, 210, 0, 0, 68, 255, 37, 0, 0, 0, 0, 0, 0, 0, 112,
    235, 0, 75, 255, 162, 0, 0, 7, 211, 174, 0, 22, 255, 122, 0, 2, 245, 134, 0, 48, 248, 221, 88,
    47, 78, 182, 254, 113, 0, 0, 0, 0, 0, 0, 0, 0, 64, 255, 127, 0, 0, 0, 0, 0, 0, 0, 0, 0, 127,
    255, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 23, 246, 152, 0, 0, 0, 0, 0, 0, 0, 57, 245, 203, 74, 74,
    203, 245, 57, 0, 0, 0, 55, 92, 92, 128, 255, 170, 92, 92, 31, 0, 86, 253, 209, 112, 112, 112,
    112, 112, 93, 0, 171, 253, 151, 79, 53, 102, 227, 245, 51, 0, 0, 0, 0, 0, 0, 0, 0, 214, 199, 0,
    0, 0, 44, 250, 199, 94, 53, 86, 214, 253, 74, 0, 0, 0, 65, 249, 191, 72, 55, 145, 255, 137, 0,
    0, 0, 0, 107, 255, 84, 0, 0, 0, 0, 0, 0, 21, 227, 241, 114, 52, 55, 125, 246, 222, 18, 0, 0, 0,
    52, 143, 239, 243, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 110, 255, 129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 163, 255, 184, 0,
    0, 0, 0, 31, 254, 132, 0, 0, 113, 254, 99, 0, 49, 255, 121, 0, 29, 254, 150, 0, 0, 0, 0, 0, 0,
    176, 253, 25, 0, 20, 255, 206, 112, 112, 128, 186, 254, 203, 13, 0, 0, 15, 200, 255, 177, 105,
    80, 120, 209, 247, 31, 0, 87, 255, 163, 102, 124, 179, 254, 224, 60, 0, 0, 0, 122, 255, 139,
    102, 102, 102, 102, 102, 53, 0, 61, 255, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0, 126, 255, 209, 111,
    71, 95, 174, 254, 199, 0, 87, 255, 102, 0, 0, 0, 0, 102, 255, 87, 0, 91, 97, 97, 236, 236, 97,
    97, 91, 0, 58, 241, 145, 84, 71, 134, 250, 237, 19, 0, 229, 214, 0, 0, 0, 0, 149, 255, 148, 0,
    0, 71, 255, 189, 138, 138, 138, 138, 138, 97, 0, 80, 255, 80, 0, 0, 0, 0, 0, 0, 94, 255, 80, 0,
    87, 255, 71, 0, 0, 0, 179, 249, 255, 87, 0, 0, 0, 134, 255, 181, 92, 92, 179, 255, 123, 0, 0,
    0, 173, 255, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 155, 255, 180, 92, 92, 179, 251, 101, 0, 0, 0,
    245, 194, 0, 0, 0, 0, 143, 255, 120, 0, 0, 4, 204, 255, 178, 112, 69, 82, 163, 253, 219, 18, 0,
    0, 0, 0, 0, 0, 230, 214, 0, 0, 0, 0, 0, 0, 0, 163, 255, 172, 91, 93, 177, 255, 158, 0, 0, 0, 0,
    0, 0, 102, 255, 255, 102, 0, 0, 0, 0, 0, 0, 48, 255, 255, 151, 0, 0, 175, 255, 254, 13, 0, 0,
    0, 127, 255, 81, 0, 0, 0, 0, 113, 255, 127, 0, 0, 0, 0, 0, 0, 0, 224, 224, 0, 0, 0, 0, 0, 0,
    112, 255, 192, 128, 128, 128, 128, 128, 128, 65, 0, 82, 255, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    152, 245, 22, 0, 0, 0, 0, 0, 0, 71, 255, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 202, 5, 0, 0, 1,
    195, 251, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 255, 182, 0, 0, 0, 26, 225, 255, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 255, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 49,
    254, 153, 0, 0, 255, 173, 0, 0, 0, 0, 168, 255, 5, 0, 0, 0, 0, 87, 255, 87, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 199, 229, 0, 224, 204, 0, 0, 41, 236, 237, 43, 0, 0, 0, 0, 0, 0, 132, 255, 49, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 255, 173, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 173, 255, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 159, 254, 118, 56, 96, 140, 3, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 85, 255, 107, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 160, 235, 0, 0, 0, 0, 194, 189, 0, 0, 0, 0, 237, 162, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 30, 244, 243, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 187, 175, 0, 0, 101, 253,
    7, 0, 0, 0, 0, 0, 0, 0, 112, 235, 0, 32, 255, 179, 0, 0, 138, 233, 22, 0, 0, 199, 211, 39, 130,
    252, 59, 0, 0, 60, 207, 255, 255, 255, 223, 85, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 249, 188, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 188, 249, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 137, 251, 33, 0, 0, 0, 0, 0,
    0, 0, 0, 65, 220, 255, 255, 220, 65, 0, 0, 0, 0, 153, 255, 255, 255, 255, 255, 255, 255, 87, 0,
    138, 255, 255, 255, 255, 255, 255, 255, 186, 0, 12, 136, 223, 255, 255, 254, 193, 50, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 214, 199, 0, 0, 0, 0, 58, 203, 251, 255, 255, 213, 73, 0, 0, 0, 0, 0, 69,
    222, 255, 255, 242, 128, 2, 0, 0, 0, 0, 182, 225, 4, 0, 0, 0, 0, 0, 0, 0, 33, 176, 250, 255,
    255, 248, 169, 28, 0, 0, 0, 81, 255, 227, 135, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 172, 248, 24,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 132, 255, 155, 0, 0, 0, 0, 0, 184, 243, 113, 138, 218, 138, 148, 0, 99, 255, 66, 0,
    111, 255, 70, 0, 0, 0, 0, 0, 0, 96, 255, 106, 0, 20, 255, 255, 255, 253, 230, 196, 106, 12, 0,
    0, 0, 0, 9, 133, 235, 255, 255, 252, 207, 66, 0, 0, 87, 255, 255, 254, 228, 188, 119, 9, 0, 0,
    0, 0, 122, 255, 255, 255, 255, 255, 255, 255, 133, 0, 61, 255, 128, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 88, 220, 254, 255, 255, 235, 144, 37, 0, 87, 255, 102, 0, 0, 0, 0, 102, 255, 87, 0, 240,
    255, 255, 255, 255, 255, 255, 240, 0, 17, 136, 242, 255, 255, 253, 194, 34, 0, 0, 229, 214, 0,
    0, 0, 0, 7, 203, 255, 90, 0, 71, 255, 255, 255, 255, 255, 255, 255, 151, 0, 95, 255, 68, 0, 0,
    0, 0, 0, 0, 79, 255, 95, 0, 87, 255, 71, 0, 0, 0, 77, 255, 255, 87, 0, 0, 0, 0, 112, 227, 255,
    255, 225, 106, 0, 0, 0, 0, 173, 255, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 132, 245, 255, 255,
    255, 174, 33, 0, 0, 0, 245, 194, 0, 0, 0, 0, 12, 223, 248, 44, 0, 0, 7, 123, 219, 252, 255,
    255, 243, 159, 24, 0, 0, 0, 0, 0, 0, 0, 230, 214, 0, 0, 0, 0, 0, 0, 0, 6, 139, 243, 255, 255,
    241, 134, 5, 0, 0, 0, 0, 0, 0, 21, 251, 251, 21, 0, 0, 0, 0, 0, 0, 13, 255, 255, 111, 0, 0,
    137, 255, 228, 0, 0, 0, 31, 246, 193, 0, 0, 0, 0, 0, 7, 222, 246, 31, 0, 0, 0, 0, 0, 0, 224,
    224, 0, 0, 0, 0, 0, 0, 143, 255, 255, 255, 255, 255, 255, 255, 255, 104, 0, 82, 255, 71, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 33, 251, 136, 0, 0, 0, 0, 0, 0, 71, 255, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    255, 253, 184, 75, 60, 165, 255, 155, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 184, 255, 143,
    49, 90, 214, 233, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 255, 20, 0, 0, 0, 0,
    0, 0, 143, 226, 2, 0, 0, 0, 0, 12, 250, 171, 0, 0, 255, 173, 0, 0, 0, 0, 168, 255, 5, 0, 0, 0,
    0, 87, 255, 87, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 199, 229, 0, 224, 204, 0, 0, 0, 58, 245, 227, 30,
    0, 0, 0, 0, 0, 74, 255, 184, 72, 79, 89, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 173, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 173, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    17, 172, 255, 255, 255, 212, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 143, 251, 184,
    2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 178, 214, 0, 0, 0, 0, 194, 189, 0, 0, 0,
    0, 214, 178, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 14, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 142, 19, 0, 112, 235, 0, 102, 255, 127, 0, 35,
    252, 81, 0, 0, 0, 27, 194, 252, 228, 102, 0, 0, 0, 0, 0, 27, 50, 32, 1, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 166, 254, 50, 0, 0, 0, 0, 0, 0, 0, 50, 254, 166, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 18,
    242, 160, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 26, 26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 47, 20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 9, 46, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 44, 9, 0, 0, 0, 0, 0, 0, 0, 16, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 14, 45, 43, 12, 0, 0, 0, 0, 0, 13, 59, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 133, 109, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 17, 165, 218, 167, 48, 39, 242, 160,
    229, 218, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 4, 32, 43, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 46, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 32, 48, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 5, 39, 74, 132, 241, 250, 116, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 45, 38,
    7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 42, 41, 8, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 82, 255,
    71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 160, 242, 18, 0, 0, 0, 0, 0, 71, 255, 82, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 255, 144, 156, 251, 255, 248, 150, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14,
    178, 252, 255, 239, 100, 137, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 158, 255, 20,
    0, 0, 0, 0, 0, 0, 90, 255, 160, 65, 39, 45, 82, 194, 253, 78, 0, 0, 255, 173, 0, 0, 0, 0, 168,
    255, 5, 0, 50, 87, 87, 144, 255, 144, 87, 87, 26, 0, 0, 0, 0, 0, 0, 0, 207, 218, 0, 224, 204,
    0, 0, 0, 0, 77, 251, 215, 20, 0, 0, 0, 0, 0, 135, 251, 255, 255, 211, 1, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 173, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 173, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 47, 24, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    197, 242, 138, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 206, 189, 0, 0, 0, 0,
    194, 189, 0, 0, 0, 0, 189, 206, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 246, 227, 130, 155, 241, 136,
    246, 225, 15, 0, 0, 45, 0, 0, 0, 0, 0, 0, 12, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 50, 252, 177, 0, 0, 0, 0, 0, 0, 0, 177, 252, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 129, 252, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 71, 180, 171, 38, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 243, 254, 71, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 82, 255, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 252, 128, 0,
    0, 0, 0, 0, 71, 255, 82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 43, 6, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 40, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 130, 238, 255, 255, 255, 255, 202, 81, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 148, 255, 255, 255, 255, 255, 255, 255, 77, 0, 0, 0, 0, 0, 0, 5, 248, 173, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 48, 32, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 46, 14, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 14, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 28, 4, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 234, 164, 0, 0, 0, 0, 194, 189, 0, 0, 0, 0,
    164, 234, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 172, 237, 255, 255, 243, 167, 23, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 139,
    254, 72, 0, 0, 0, 0, 0, 72, 254, 139, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 238, 168, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 118, 255, 166, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 82, 255, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 168, 238, 14, 0, 0, 0, 0, 71, 255, 82, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
    32, 54, 43, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 100, 255, 113, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 254, 141, 0, 0, 0, 0, 194,
    189, 0, 0, 0, 0, 141, 254, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 131, 237, 6, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6,
    186, 240, 47, 0, 0, 0, 47, 240, 186, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 121, 254, 46, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 69, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 82, 255, 93, 31, 31, 21, 0, 0, 0, 0, 0, 0, 0, 0, 46, 254, 121, 0, 21, 31, 31, 93, 255, 82,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 2, 91, 247, 196, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 240, 182, 1, 0, 0, 0, 194,
    189, 0, 0, 0, 1, 182, 240, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112, 235, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    12, 191, 237, 48, 0, 48, 237, 191, 12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 147, 172, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    82, 255, 255, 255, 255, 178, 0, 0, 0, 0, 0, 0, 0, 0, 0, 172, 147, 0, 179, 255, 255, 255, 255,
    82, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 1, 53, 119, 207, 255, 172, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 129, 255, 218,
    169, 3, 0, 194, 189, 0, 3, 169, 218, 255, 129, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 112,
    235, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 139, 23, 0, 23, 139, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 24, 76, 76, 76, 76, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 54, 76, 76, 76, 76, 24,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3,
    242, 228, 161, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 142, 171, 4, 0,
    8, 8, 0, 4, 171, 142, 63, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 14, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
