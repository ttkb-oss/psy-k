// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

use std::env;
use std::fs::{File, FileTimes};
use std::path::PathBuf;

use anyhow::{bail, Result};
use clap::{crate_version, CommandFactory, Parser, Subcommand};

use psyx::io::{read, read_lib, read_obj, write_obj};

/// Inspect, extract, and create PSY-Q LIB and OBJ files.
#[derive(Debug, Parser)]
#[clap(name = env!("CARGO_CRATE_NAME"), version)]
#[command(version, about, long_about = None)]
pub struct App {
    #[arg(required = false)]
    lib_or_obj: Option<PathBuf>,

    #[clap(subcommand)]
    command: Option<CLICommand>,
}

#[derive(Debug, Subcommand)]
enum CLICommand {
    /// prints information about the file
    Info {
        /// a LIB or OBJ file
        #[arg(required = true)]
        lib_or_obj: PathBuf,

        /// enable a listing of code in the dump
        #[clap(short, long)]
        code: bool,

        /// show disassembly of code for known architectures
        #[clap(short, long)]
        disassemble: bool,
    },

    /// splits a [LIB] into multiple [OBJ]s
    Split {
        /// the [LIB] to split
        #[arg(required = true)]
        lib: PathBuf,
    },

    /// join OBJs into a [LIB]
    Join {
        /// the [LIB] to create
        #[arg(required = true)]
        lib: PathBuf,
        /// the [OBJ]s to include
        #[arg(num_args=1..)]
        objs: Vec<PathBuf>,
    },

    /// Adds an [OBJ] into an existing [LIB]
    Add {
        /// the [LIB] to create
        #[arg(required = true)]
        lib: PathBuf,
        /// the [OBJ] to add
        #[arg(required = true)]
        obj: PathBuf,
    },
}

fn main() -> Result<()> {
    let args = App::parse();

    match args.command {
        Some(command) => match command {
            CLICommand::Info {
                lib_or_obj,
                code,
                disassemble,
            } => info(lib_or_obj, code, disassemble)?,
            CLICommand::Split { lib } => split(lib)?,
            CLICommand::Join { lib, objs } => join(lib, objs)?,
            CLICommand::Add { lib, obj } => add(lib, obj)?,
        },
        None => match args.lib_or_obj {
            Some(lib_or_obj) => info(lib_or_obj, false, false)?,
            None => {
                let a = App::command().render_help();
                eprintln!("{}", a);
            }
        },
    }

    Ok(())
}

fn info(lib_or_obj: PathBuf, code: bool, disassembly: bool) -> Result<()> {
    let o = read(&lib_or_obj)?;
    if disassembly {
        unsafe {
            env::set_var("DUMP", "DISASSEMBLE");
        }
    } else if code {
        unsafe {
            env::set_var("DUMP", "CODE");
        }
    }
    println!("{o}");
    Ok(())
}

fn split(lib_path: PathBuf) -> Result<()> {
    let lib = read_lib(&lib_path)?;
    println!("psyx version {}\n", crate_version!());
    for module in lib.modules() {
        let object_filename = format!("{}.OBJ", module.name());
        let time = module.created_at().expect("created timestamp");
        let mut file = File::create(&object_filename)?;
        let times = FileTimes::new().set_accessed(time).set_modified(time);
        file.set_times(times)?;
        write_obj(module.object(), &mut file)?;

        println!("Extracted object file {}", object_filename);
    }
    Ok(())
}

fn join(lib_path: PathBuf, _obj_paths: Vec<PathBuf>) -> Result<()> {
    let _lib = read_lib(&lib_path)?;
    bail!("unimplemented");
}

fn add(lib_path: PathBuf, obj_path: PathBuf) -> Result<()> {
    let _lib = read_lib(&lib_path)?;
    let _obj = read_obj(&obj_path)?;

    bail!("unimplemented");
    // get name from path
    // get created from metadata
    // offset?
    // size from metadata
}
