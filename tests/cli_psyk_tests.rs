// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

use std::process::Command;

use assert_cmd::cargo;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[inline]
fn psyk() -> Command {
    Command::new(cargo::cargo_bin!("psyk"))
}

#[test]
fn test_psyk_help() {
    psyk()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"))
        .stderr(predicate::str::contains("  list   "))
        .stderr(predicate::str::contains("  help   "));
}

#[test]
fn test_psyk_list_help() {
    psyk()
        .arg("list")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"))
        .stderr(predicate::str::contains("--help"));
}

#[test]
fn test_psyk_list_valid_file() {
    // no command variant
    psyk()
        .arg("tests/data/psy-q/3.5/PSX/LIB/LIBCARD.LIB")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "A74      15-05-96 16:12:06 InitCARD",
        ));

    psyk()
        .arg("list")
        .arg("tests/data/psy-q/3.5/PSX/LIB/LIBCARD.LIB")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "A74      15-05-96 16:12:06 InitCARD",
        ));

    psyk()
        .arg("list")
        .arg("tests/data/psy-q/3.5/PSX/LIB/MALLOC.OBJ")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "\
            2 : Code 1548 bytes\n\
            10 : Patch type 82 at offset 8 with (sectbase(557f)+$8)\n\
        ",
        ));

    psyk()
        .arg("list")
        .arg("--code")
        .arg("tests/data/psy-q/3.5/PSX/LIB/MALLOC.OBJ")
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

    psyk()
        .arg("list")
        .arg("--disassemble")
        .arg("tests/data/psy-q/3.5/PSX/LIB/MALLOC.OBJ")
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
fn test_psyk_list_file_not_found() {
    psyk()
        .arg("list")
        .arg("non_existent_file.lib")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_psyk_add_missing_args() {
    psyk()
        .arg("add")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_psyk_add_file_not_found() {
    psyk()
        .arg("add")
        .arg("non_existent_file.lib")
        .arg("non_existent_file.obj")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));

    // real LIB
    psyk()
        .arg("add")
        .arg("tests/data/psy-q/3.5/PSX/LIB/LIBCARD.LIB")
        .arg("non_existent_file.obj")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_psyk_create_missing_args() {
    psyk()
        .arg("create")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_psyk_create_file_not_found() {
    psyk()
        .arg("create")
        .arg("non_existent_file.lib")
        .arg("non_existent_file.obj")
        .assert()
        .failure()
        .stderr(predicate::str::contains("File not found"));
}

#[test]
fn test_psyk_deletem_issing_args() {
    psyk()
        .arg("delete")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_psyk_delete_file_not_found() {
    psyk()
        .arg("delete")
        .arg("non_existent_file.lib")
        .arg("FOO")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_psyk_update_missing_args() {
    psyk()
        .arg("update")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_psyk_update_file_not_found() {
    psyk()
        .arg("update")
        .arg("non_existent_file.lib")
        .arg("non_existent_file.obj")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));

    // real LIB
    psyk()
        .arg("update")
        .arg("tests/data/psy-q/3.5/PSX/LIB/LIBCARD.LIB")
        .arg("non_existent_file.obj")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_psyk_extract_missing_args() {
    psyk()
        .arg("extract")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_psyk_extract_file_not_found() {
    psyk()
        .arg("extract")
        .arg("non_existent_file.lib")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}
