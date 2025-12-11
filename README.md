`psy-x`
========


`psy-x` is a pure-safe library and utility for parsing PSY-Q LIB & OBJ files.

As a utility, `psyx` will print the contents of LIB or OBJ files. `psyx` can split `LIB` files into `OBJ`s or combine
`OBJ`s into `LIB`s; your choice, really.

As a library, `psy-x` parses `LIB` and `OBJ` files for programmatic manipulation.

`psy-x` is used by `mipsmatch`.

Commands
--------

*default*/*list* - dump a `LIB` or `OBJ` file

```bash
$> psyx PSX/LIB/LIBCARD.LIB
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

*extract* - extract `OBJ`s from a a `LIB` file

```bash
$> psyx extract PSX/LIB/LIBCARD.LIB
psyx version 0.1.0

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

*create* - create a new `LIB` from one or more `OBJ`s

*add* - add another `OBJ` to an existing `LIB`

*update* - update an existing `OBJ` in a `LIB`

*delete* - delete an `OBJ` from a `LIB`

Library
-------

`psy-x` can be used to programmatically read and modify `LIB` and `OBJ` structures as well.

```rust
use std::path::Path;
use psyx::io;
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
