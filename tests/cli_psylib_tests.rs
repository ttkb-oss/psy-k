// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

use std::process::Command;

use assert_cmd::cargo;
use assert_cmd::prelude::*;
use predicates::prelude::*;

#[inline]
fn psylib() -> Command {
    Command::new(cargo::cargo_bin!("psylib"))
}

#[test]
fn test_psylib_help() {
    psylib()
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"))
        .stderr(predicate::str::contains(
            "psylib /u <library.lib> <obj1> [obj2...]",
        ))
        .stderr(predicate::str::contains("psylib /l <library.lib>"));
}

#[test]
fn test_psylib_list_valid_file() {
    psylib()
        .arg("/l")
        .arg("tests/data/psy-q/3.5/PSX/LIB/LIBCARD.LIB")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "A74      15-05-96 16:12:06 InitCARD",
        ));
}

#[test]
fn test_psylib_list_file_not_found() {
    psylib()
        .arg("/l")
        .arg("non_existent_file.lib")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_psylib_list_missing_args() {
    psylib()
        .arg("/l")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_psylib_add_missing_args() {
    psylib()
        .arg("/a")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_psylib_add_file_not_found() {
    psylib()
        .arg("/a")
        .arg("non_existent_file.lib")
        .arg("non_existent_file.obj")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));

    // real LIB
    psylib()
        .arg("/a")
        .arg("tests/data/psy-q/3.5/PSX/LIB/LIBCARD.LIB")
        .arg("non_existent_file.obj")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_psylib_deletem_issing_args() {
    psylib()
        .arg("/d")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_psylib_delete_file_not_found() {
    psylib()
        .arg("/d")
        .arg("non_existent_file.lib")
        .arg("FOO")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_psylib_update_missing_args() {
    psylib()
        .arg("/u")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_psylib_update_file_not_found() {
    psylib()
        .arg("/u")
        .arg("non_existent_file.lib")
        .arg("non_existent_file.obj")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));

    // real LIB
    psylib()
        .arg("/u")
        .arg("tests/data/psy-q/3.5/PSX/LIB/LIBCARD.LIB")
        .arg("non_existent_file.obj")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_psylib_extract_missing_args() {
    psylib()
        .arg("/x")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn test_psylib_extract_file_not_found() {
    psylib()
        .arg("/x")
        .arg("non_existent_file.lib")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error"));
}

#[test]
fn test_psylib_extract_bad_subcommand() {
    psylib()
        .arg("/?")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid option: /?"))
        .stderr(predicate::str::contains("Usage"));
}
