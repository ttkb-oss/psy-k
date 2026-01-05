// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::{File, FileTimes};
use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::bail;
use anyhow::Result;
use clap::crate_version;

use psyk::display;
use psyk::io::{read, read_lib, write_lib, write_obj};
use psyk::{Module, LIB};

/// Prints information about an [OBJ](super::OBJ) or [LIB].
pub fn info<P: AsRef<Path>>(
    write: &mut impl Write,
    lib_or_obj: P,
    code: bool,
    disassembly: bool,
    recursive: bool,
) -> Result<()> {
    let o = read(&lib_or_obj)?;
    let mut options = display::Options::default();
    if disassembly {
        options.code_format = display::CodeFormat::Disassembly;
    } else if code {
        options.code_format = display::CodeFormat::Hex;
    }
    options.recursive = recursive;
    writeln!(write, "{}", display::PsyKDisplayable::wrap(&o, options))?;
    Ok(())
}

pub fn split<P: AsRef<Path>>(lib_path: P) -> Result<()> {
    let lib = read_lib(&lib_path)?;
    println!("psyk version {}\n", crate_version!());
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

pub fn delete<P: AsRef<Path>>(lib_path: P, obj_names: &[String]) -> Result<()> {
    let lib = read_lib(&lib_path)?;

    let module_names: HashSet<&String> = HashSet::from_iter(obj_names);

    let new_modules: Vec<Module> = lib
        .into_modules()
        .into_iter()
        .filter(|m| !module_names.contains(&m.name()))
        .collect::<Vec<Module>>();
    let lib = LIB::new(new_modules);

    let mut file = File::create(&lib_path)?;
    Ok(write_lib(&lib, &mut file)?)
}

pub fn join<P: AsRef<Path>, O: AsRef<Path>>(lib_path: P, obj_paths: &[O]) -> Result<()> {
    let modules = obj_paths
        .iter()
        .map(|path| Module::new_from_path(path).expect("module"))
        .collect::<Vec<Module>>();

    let lib = LIB::new(modules);

    let mut file = File::create(lib_path)?;
    Ok(write_lib(&lib, &mut file)?)
}

pub fn add<P: AsRef<Path>, O: AsRef<Path>>(lib_path: P, obj_path: O) -> Result<()> {
    let lib = read_lib(&lib_path)?;

    let module = Module::new_from_path(obj_path)?;
    let mut modules: Vec<Module> = lib.modules().clone();
    modules.push(module);

    let lib = LIB::new(modules);

    let mut file = File::create(lib_path)?;
    Ok(write_lib(&lib, &mut file)?)
}

pub fn update<P: AsRef<Path>, O: AsRef<Path>>(lib_path: P, obj_paths: &[O]) -> Result<()> {
    let lib = read_lib(&lib_path)?;

    let mut updated_module_paths: HashMap<String, PathBuf> = HashMap::new();
    for path in obj_paths {
        let p = path.as_ref();
        if !Path::exists(p) {
            bail!(format!("File not found: {}", p.display()));
        }

        let module_name = String::from(p.file_stem().expect("file").to_string_lossy());
        updated_module_paths.insert(module_name, p.to_path_buf());
    }

    let new_modules = lib
        .into_modules()
        .into_iter()
        .map({
            |m| {
                if let Some(module_path) = updated_module_paths.get(&m.name()) {
                    let Ok(new_mod) = Module::new_from_path(module_path) else {
                        eprintln!("could not read: {module_path:?}. Skipping.");
                        return m.clone();
                    };
                    new_mod
                } else {
                    m.clone()
                }
            }
        })
        .collect::<Vec<Module>>();
    let lib = LIB::new(new_modules);

    let mut file = File::create(lib_path)?;
    Ok(write_lib(&lib, &mut file)?)
}
