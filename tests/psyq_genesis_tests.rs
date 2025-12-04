// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

use std::path::PathBuf;

mod common;

use common::{compare_output, round_trip};

const CMD_DATA_PREFIX: &str = "tests/data/cmd/psy-q-genesis";
const PSYQ_PREFIX: &str = "target/.private/tests/data/psy-q-genesis";

#[inline]
fn path_gen(file: &str) -> PathBuf {
    PathBuf::from(format!("{PSYQ_PREFIX}/{file}"))
}

#[test]
pub fn test_roundtrip() {
    round_trip(&path_gen("LIBSN68/LIBSN.LIB"));
}

fn compare_lib_output(prefix: &str) {
    compare_output(
        &path_gen(&format!("{prefix}.LIB")),
        &PathBuf::from(format!("{CMD_DATA_PREFIX}/{prefix}.TXT")),
        3,
    );
}

#[test]
pub fn test_output() {
    compare_lib_output("LIBSN68/LIBSN");
}
