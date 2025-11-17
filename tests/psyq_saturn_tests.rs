// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

use std::path::{Path, PathBuf};

use binrw::io::Cursor;
use binrw::BinWrite;
use psyx::io;

fn round_trip(path: &Path) {
    eprintln!("roundtripping {}", path.display());
    let bin = io::read(path);
    let mut writer = Cursor::new(Vec::new());

    match bin {
        Ok(io::Type::OBJ(ref lnk)) => lnk.write(&mut writer).unwrap(),
        Ok(io::Type::LIB(ref lib)) => lib.write(&mut writer).unwrap(),
        Err(e) => panic!("{}", e),
    }

    let bytes = std::fs::read(path).expect("file");
    let gen = writer.into_inner();
    if bytes != gen {
        eprintln!(
            "{}",
            match bin {
                Ok(io::Type::OBJ(ref lnk)) => lnk as &dyn std::fmt::Display,
                Ok(io::Type::LIB(ref lib)) => lib as &dyn std::fmt::Display,
                Err(_) => &"error" as &dyn std::fmt::Display,
            }
        );
    }
    assert_eq!(bytes.len(), gen.len());
    assert_eq!(bytes, gen);
}

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
