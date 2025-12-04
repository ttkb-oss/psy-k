// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

use std::path::PathBuf;

mod common;

use common::{compare_output, round_trip};

const CMD_DATA_PREFIX: &str = "tests/data/cmd/psy-q-saturn";
const PSYQ_PREFIX: &str = "target/.private/tests/data/psy-q-saturn";

#[inline]
fn path_sat(file: &str) -> PathBuf {
    PathBuf::from(format!("{PSYQ_PREFIX}/{file}"))
}

#[test]
pub fn test_roundtrip() {
    round_trip(&path_sat("dos/GNUSHLIB/LIB/LIBSN.LIB"));
    round_trip(&path_sat("dos/GNUSHLIB/LIB/LIBG.LIB"));
    round_trip(&path_sat("dos/GNUSHLIB/LIB/LIBC.LIB"));
    round_trip(&path_sat("dos/GNUSHLIB/LIB/LIBSTDCX.LIB"));
    round_trip(&path_sat("dos/GNUSHLIB/LIB/LIBM.LIB"));
    round_trip(&path_sat("dos/GNUSHLIB/LIB/LIBGXX.LIB"));
    round_trip(&path_sat("dos/GNUSHLIB/LIB/LIBGCC.LIB"));
}

fn compare_lib_output(prefix: &str) {
    compare_output(
        &path_sat(&format!("{prefix}.LIB")),
        &PathBuf::from(format!("{CMD_DATA_PREFIX}/{prefix}.TXT")),
        3,
    );
}

#[test]
pub fn test_output() {
    compare_lib_output("dos/GNUSHLIB/LIB/LIBC");
    compare_lib_output("dos/GNUSHLIB/LIB/LIBG");
    compare_lib_output("dos/GNUSHLIB/LIB/LIBGCC");
    compare_lib_output("dos/GNUSHLIB/LIB/LIBGXX");
    compare_lib_output("dos/GNUSHLIB/LIB/LIBM");
    compare_lib_output("dos/GNUSHLIB/LIB/LIBSN");
    compare_lib_output("dos/GNUSHLIB/LIB/LIBSTDCX");
}
