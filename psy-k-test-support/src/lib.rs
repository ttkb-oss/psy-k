// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

use std::ffi::OsString;

pub fn unsafe_path_name() -> OsString {
    // b"\u{C0}invalid.obj"
    let s: OsString;
    unsafe {
        s = OsString::from_encoded_bytes_unchecked(vec![
            0xC0, 0x69, 0x6E, 0x76, 0x61, 0x6C, 0x69, 0x64, 0x2e, 0x6f, 0x62, 0x6a,
        ]);
    }
    s
}
