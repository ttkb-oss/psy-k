// Copyright (c)\x2025 joaoviictorti
// Licensed under the MIT License. See LICENSE file in the project root for details.

use std::fmt::{Debug, Display, Formatter};
use std::io::{Read, Seek, Write};
use std::path::Path;

use crate::{display, LIB, OBJ};
use binrw::io::Cursor;
use binrw::{meta::ReadMagic, BinRead, BinWrite};
use thiserror::Error;

/// `IOError`s should be considered unstable for the time being.
#[derive(Debug, Error)]
pub enum IOError {
    #[error("{0}: {1}")]
    FileNotFound(String, String),

    #[error("Bad magic: {0}")]
    BadMagic(String),

    #[error(transparent)]
    ParseError(#[from] binrw::Error),

    #[error(transparent)]
    SerializeError(binrw::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

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

fn read_bytes<P: AsRef<Path>>(path: P) -> std::result::Result<Vec<u8>, IOError> {
    if !Path::exists(path.as_ref()) {
        let p = path.as_ref().display();
        return Err(IOError::FileNotFound(
            format!("File not found: {p}"),
            p.to_string(),
        ));
    }

    Ok(std::fs::read(path.as_ref())?)
}

/// Reads a Psy-Q [LIB] or [OBJ]. If the file cannot be found or if the file
/// does not contain valid data an error will be returned.
pub fn read<P: AsRef<Path>>(lib_or_obj_path: P) -> std::result::Result<Type, IOError> {
    // binrw can operate on a File or BufReader directly, but
    // performance on either of those is significantly lower
    // than operating on a vec directly. OBJ and LIB files are
    // typically less than 1MB and the resulting structure is
    // completely in memory and not streamed.
    let bytes = read_bytes(lib_or_obj_path)?;
    read_from_memory(&bytes)
}

pub fn read_from_memory(bytes: &[u8]) -> std::result::Result<Type, IOError> {
    if bytes.len() < 3 {
        return Err(IOError::BadMagic(
            "File too small to contain valid PSY-Q magic number".to_string(),
        ));
    }

    let mut magic: [u8; 3] = [0; 3];
    magic.copy_from_slice(&bytes[0..3]);
    match magic {
        LIB::MAGIC => Ok(read_lib_from_memory(bytes).map(Type::LIB)?),
        OBJ::MAGIC => Ok(read_obj_from_memory(bytes).map(Type::OBJ)?),
        _ => Err(IOError::BadMagic(format!(
            "Unrecognized magic {:?}",
            &magic
        ))),
    }
}

/// Reads a Psy-Q [OBJ]. If the file cannot be found or if the file
/// does not contain valid data an error will be returned.
pub fn read_obj<P: AsRef<Path>>(obj_path: P) -> std::result::Result<OBJ, IOError> {
    let bytes = read_bytes(obj_path)?;
    let mut data = Cursor::new(&bytes);
    read_obj_from_reader(&mut data)
}

pub fn read_obj_from_memory(bytes: &[u8]) -> std::result::Result<OBJ, IOError> {
    read_obj_from_reader(Cursor::new(bytes))
}

/// Reads a Psy-Q [OBJ].
pub fn read_obj_from_reader<R: Read + Seek>(mut reader: R) -> std::result::Result<OBJ, IOError> {
    OBJ::read(&mut reader).map_err(IOError::ParseError)
}

/// Reads a Psy-Q [LIB]. If the file cannot be found or if the file
/// does not contain valid data an error will be returned.
pub fn read_lib<P: AsRef<Path>>(lib_path: P) -> std::result::Result<LIB, IOError> {
    let bytes = read_bytes(lib_path)?;
    let mut data = Cursor::new(&bytes);
    read_lib_from_reader(&mut data)
}

pub fn read_lib_from_memory(bytes: &[u8]) -> std::result::Result<LIB, IOError> {
    read_lib_from_reader(Cursor::new(bytes))
}

/// Reads a Psy-Q [LIB].
pub fn read_lib_from_reader<R: Read + Seek>(mut reader: R) -> std::result::Result<LIB, IOError> {
    LIB::read(&mut reader).map_err(IOError::ParseError)
}

/// Writes a Psy-Q [OBJ]. If the file cannot be written an error will
/// be returned.
pub fn write_obj<W: Write + Seek>(obj: &OBJ, writer: &mut W) -> std::result::Result<(), IOError> {
    obj.write(writer).map_err(IOError::SerializeError)
}

/// Writes a Psy-Q [LIB]. If the file cannot be written an error will
/// be returned.
pub fn write_lib<W: Write + Seek>(lib: &LIB, writer: &mut W) -> std::result::Result<(), IOError> {
    lib.write(writer).map_err(IOError::SerializeError)
}
