`psy-k`
========

[![codecov](https://codecov.io/github/ttkb-oss/psy-k/graph/badge.svg?token=G837GZY5VW)](https://codecov.io/github/ttkb-oss/psy-k)
[![Latest version](https://img.shields.io/crates/v/psy-k.svg)](https://crates.io/crates/psy-k)

`psy-k` is a pure-safe library and utility for parsing PSY-Q LIB & OBJ files.

Several utilities are included in the `psy-k-bin` crate. `psyk` will print the contents of LIB or OBJ files. `psyk` can split `LIB` files into `OBJ`s or combine
`OBJ`s into `LIB`s; your choice, really.

As a library, `psy-k` parses `LIB` and `OBJ` files for programmatic manipulation.

`psy-k` is used by `mipsmatch`.

Commands
--------

**(no command)/`list` - dump a `LIB` or `OBJ` file**

```bash
$> psyk PSX/LIB/LIBCARD.LIB
Module     Date     Time   Externals defined
C112     26-12-95 17:43:08 _bu_init
C171     26-12-95 17:43:08 _card_info
C172     26-12-95 17:43:08 _card_load
C173     26-12-95 17:43:08 _card_auto
A74      26-12-95 17:43:10 InitCARD
A75      26-12-95 17:43:10 StartCARD
A76      26-12-95 17:43:10 StopCARD
A78      26-12-95 17:43:10 _card_write
A79      26-12-95 17:43:10 _card_read
A80      26-12-95 17:43:10 _new_card
A92      26-12-95 17:43:12 _card_status
A93      26-12-95 17:43:12 _card_wait
CARD     26-12-95 17:43:12 _card_clear
```

Additional options allow for deeper inspection:

```
# Show hex dump of all sections
psyk list --code PLAYER.OBJ

# Show MIPS disassembly
psyk list --disassemble MAIN.OBJ

# Recursively list all modules and their exports inside a library
psyk list --recursive LIBSN.LIB
```

**`extract` - extract `OBJ`s from a a `LIB` file**

```bash
$> psyk extract PSX/LIB/LIBCARD.LIB
psyk version 0.1.0

Extracted object file C112.OBJ
Extracted object file C171.OBJ
Extracted object file C172.OBJ
Extracted object file C173.OBJ
Extracted object file A74.OBJ
Extracted object file A75.OBJ
Extracted object file A76.OBJ
Extracted object file A78.OBJ
Extracted object file A79.OBJ
Extracted object file A80.OBJ
Extracted object file A92.OBJ
Extracted object file A93.OBJ
Extracted object file CARD.OBJ
```

**`create` - Create a new library**

Combines one or more object files into a new .LIB archive.

```bash
psyk create NEW.LIB C1.OBJ C2.OBJ A1.OBJ
```

**`add` - Appends a new object file to an existing library**

Appends a new object file to an existing library.

```bash
psyk add UTILS.LIB NEW_FUNC.OBJ
```

**update** - Update objects in a library

Replaces existing modules in a library with new versions from disk if the filenames match.

```
psyk update MATH.LIB VECTOR.OBJ MATRIX.OBJ
```

**delete** - Delete objects from a library

Removes specific modules from a library by name.

```bash
psyk delete PROJECT.LIB OLD_FUNC DEPRECATED_STUB
```

### DOS Compatibility

`psyk` can also provides `dumpobj` or `psylib` utilities that are drop in replacements for the original DOS Psy-Q
versions. This is primarily useful if you have a toolchain that uses those utilities and want to migrate to a Psy-K.

```bash
psylib /x MYLIB.LIB  # Equivalent to `psyk extract MYLIB.LIB`
```

Library
-------

`psy-k` can be used to programmatically read and modify `LIB` and `OBJ` structures as well.

```rust
use std::path::Path;
use psyk::io;
use anyhow::Result;

fn main() -> Result<()> {
    let lib = io::read_lib(Path::new("LIBAPI.LIB"))?;

    for module in lib.modules() {
        println!("Module: {}", module.name());
        println!("Created: {}", module.created());
        println!("Exports: {:?}", module.exports());
    }

    Ok(())
}
```

References
----------

sozud's [psy-q-splitter](https://github.com/sozud/psy-q-splitter) includes a `LIB` parser
and extractor.

[spirit t0aster's worklog](https://web.archive.org/web/20230428082811/https://www.psxdev.net/forum/viewtopic.php?t=1582)
provided additional information for several expression operators that are present in Psy-Q for Saturn but were not in
Psy-Q for Playstation.
