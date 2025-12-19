// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

use std::process::Command;

use assert_cmd::cargo;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[inline]
fn dumpobj() -> Command {
    Command::new(cargo::cargo_bin!("dumpobj"))
}

#[test]
fn test_dumpobj_help() {
    let mut cmd = dumpobj();

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"))
        .stderr(predicate::str::contains("/c    Show code listing"))
        .stderr(predicate::str::contains("/d    Show disassembly"));
}

#[test]
fn test_dumpobj_valid_file() {
    // Note: You should include a small sample .obj file in tests/fixtures/
    let mut cmd = dumpobj();
    cmd.arg("tests/data/psy-q/3.5/PSX/LIB/MALLOC.OBJ");

    cmd.assert()
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains(
            "28 : Define file number 59a7 as \"C:\\PSX.NEW\\SRC\\C\\MALLOC4.C\"",
        ))
        // no code listing is configured, a new section immediately follows code.
        .stdout(predicate::str::contains(
            "\
            2 : Code 1548 bytes\n\
            10 : Patch type 82 at offset 8 with (sectbase(557f)+$8)\n\
        ",
        ));

    dumpobj()
        .arg("tests/data/psy-q/3.5/PSX/LIB/MALLOC.OBJ")
        .arg("/c")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "28 : Define file number 59a7 as \"C:\\PSX.NEW\\SRC\\C\\MALLOC4.C\"",
        ))
        // no code listing is configured, a new section immediately follows code.
        .stdout(predicate::str::contains(
            "\
            2 : Code 1548 bytes\n\
            \n\
            0000: 1d 00 80 10 00 00 00 00 00 00 03 3c 00 00 63 8c\n\
        ",
        ));

    dumpobj()
        .arg("tests/data/psy-q/3.5/PSX/LIB/MALLOC.OBJ")
        .arg("/d")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "28 : Define file number 59a7 as \"C:\\PSX.NEW\\SRC\\C\\MALLOC4.C\"",
        ))
        // no code listing is configured, a new section immediately follows code.
        .stdout(predicate::str::contains(
            "\
            2 : Code 1548 bytes\n\
            \n    \
            /* 1080001d */   beqz        $a0, . + 4 + (0x1D << 2)\n\
        ",
        ));
}

#[test]
fn test_dumpobj_file_not_found() {
    dumpobj()
        .arg("non_existent_file.obj")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_dumpobj_bad_option() {
    dumpobj()
        .arg("tests/data/psy-q/3.5/PSX/LIB/MALLOC.OBJ")
        .arg("/?")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid option: /?"))
        .stderr(predicate::str::contains("Usage"));
}
