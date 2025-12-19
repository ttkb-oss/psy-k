// Copyright (c)\x2025 joaoviictorti
// Licensed under the MIT License. See LICENSE file in the project root for details.

use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::{display, LIB, OBJ};
use anyhow::{bail, Result};
use binrw::io::Cursor;
use binrw::{meta::ReadMagic, BinRead, BinWrite};

#[derive(Debug)]
pub enum Type {
    OBJ(OBJ),
    LIB(LIB),
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::OBJ(obj) => obj as &dyn Display,
            Self::LIB(lib) => lib as &dyn Display,
        }
        .fmt(f)
    }
}

impl display::DisplayWithOptions for Type {
    fn fmt_with_options(&self, f: &mut Formatter, options: &display::Options) -> std::fmt::Result {
        match self {
            Self::OBJ(obj) => obj as &dyn display::DisplayWithOptions,
            Self::LIB(lib) => lib as &dyn display::DisplayWithOptions,
        }
        .fmt_with_options(f, options)
    }
}

pub fn read_bytes(path: &Path) -> Result<Vec<u8>> {
    if !Path::exists(path) {
        bail!(format!("File not found: {}", path.display()));
    }

    Ok(std::fs::read(path)?)
}

/// Reads a Psy-Q [LIB] or [OBJ]. If the file cannot be found or if the file
/// does not contain valid data an error will be returned.
pub fn read(lib_or_obj_path: &Path) -> Result<Type> {
    let bytes = read_bytes(lib_or_obj_path)?;

    if bytes.len() < 3 {
        bail!("File too small to contain valid PSY-Q magic number");
    }

    let mut magic: [u8; 3] = [0; 3];
    magic.clone_from_slice(&bytes[0..3]);
    let mut data = Cursor::new(&bytes);

    match magic {
        LIB::MAGIC => Ok(Type::LIB(LIB::read(&mut data)?)),
        OBJ::MAGIC => Ok(Type::OBJ(OBJ::read(&mut data)?)),
        _ => bail!(format!("Unrecognized magic {:?}", &bytes[0..3])),
    }
}

/// Reads a Psy-Q [OBJ]. If the file cannot be found or if the file
/// does not contain valid data an error will be returned.
pub fn read_obj(obj_path: &Path) -> Result<OBJ> {
    let bytes = read_bytes(obj_path)?;
    let mut data = Cursor::new(&bytes);
    Ok(OBJ::read(&mut data)?)
}

/// Reads a Psy-Q [LIB]. If the file cannot be found or if the file
/// does not contain valid data an error will be returned.
pub fn read_lib(lib_path: &Path) -> Result<LIB> {
    let bytes = read_bytes(lib_path)?;
    let mut data = Cursor::new(&bytes);
    Ok(LIB::read(&mut data)?)
}

/// Writes a Psy-Q [OBJ]. If the file cannot be written an error will
/// be returned.
pub fn write_obj(obj: &OBJ, file: &mut File) -> Result<()> {
    let mut writer = Cursor::new(Vec::new());
    obj.write(&mut writer)?;
    let gen = writer.into_inner();
    file.write_all(&gen)?;
    Ok(())
}

/// Writes a Psy-Q [LIB]. If the file cannot be written an error will
/// be returned.
pub fn write_lib(lib: &LIB, file: &mut File) -> Result<()> {
    let mut writer = Cursor::new(Vec::new());
    lib.write(&mut writer)?;
    let gen = writer.into_inner();
    file.write_all(&gen)?;
    Ok(())
}
