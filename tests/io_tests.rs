// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

use std::path::Path;

use psyx::io;
use psyx::LIB;

const PSYQ_PREFIX: &str = "tests/data/psy-q";

#[test]
fn test_lib_creation() {
    let lib =
        io::read_lib(Path::new(&format!("{PSYQ_PREFIX}/3.5/PSX/LIB/LIBCD.LIB"))).expect("lib");

    let modules = lib.modules();

    let new_lib = LIB::new(modules.clone());

    assert_eq!(lib, new_lib);
}
