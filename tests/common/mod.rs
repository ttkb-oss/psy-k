// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

use std::fs::read_to_string;
use std::path::Path;

use binrw::io::Cursor;
use binrw::BinWrite;
use psyk::io;

pub fn round_trip(path: &Path) {
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

pub fn compare_output(lib_path: &Path, txt_path: &Path, skip_lines: usize) {
    let bin = io::read(lib_path).expect("lib");
    let psyk_output = format!("{bin}");
    let psyq_output = read_to_string(txt_path).unwrap();

    // Compare line by line
    for (line_num, (psyk_line, dump_line)) in psyk_output
        .lines()
        .zip(
            psyq_output
                .lines()
                .skip(skip_lines)
                // TODO: wrapped lines aren't supported is psy-x
                .filter(|l| !l.starts_with("        ")),
        )
        .enumerate()
    {
        println!("{line_num}: {dump_line}");
        println!("{line_num}: {psyk_line}");
        println!();
        if psyk_line != dump_line {
            // TODO: currently psyk doesn't handle line wrapping
            if psyk_line.len() > 70 {
                continue;
            }
            // TODO: not specifying locale
            if dump_line.contains("Uninitialised") {
                continue;
            }
            println!(
                "Diff at line {}: \n  psyk: {}\n  dump: {}",
                line_num, psyk_line, dump_line
            );
            assert_eq!(dump_line, psyk_line);
        }
    }
}
