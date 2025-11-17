`psy-x`
========


`psy-x` is a pure-safe library and utility for parsing PSY-Q LIB & OBJ files.

As a utility, `psyx` will print the contents of LIB or OBJ files. `psyx` can split `LIB` files into `OBJ`s or combine
`OBJ`s into `LIB`s; your choice, really.

As a library, `psy-x` parses `LIB` and `OBJ` files for programmatic manipulation.

`psy-x` is used by `mipsmatch`.

Commands
--------

*default*/*info* - dump a `LIB` or `OBJ` file

```bash
$> psyx PSX/LIB/LIBCARD.LIB
Module     Date     Time   Externals defined
C112     12-26-95 17:43:08 _bu_init
C171     12-26-95 17:43:08 _card_info
C172     12-26-95 17:43:08 _card_load
C173     12-26-95 17:43:08 _card_auto
A74      12-26-95 17:43:10 InitCARD
A75      12-26-95 17:43:10 StartCARD
A76      12-26-95 17:43:10 StopCARD
A78      12-26-95 17:43:10 _card_write
A79      12-26-95 17:43:10 _card_read
A80      12-26-95 17:43:10 _new_card
A92      12-26-95 17:43:12 _card_status
A93      12-26-95 17:43:12 _card_wait
CARD     12-26-95 17:43:12 _card_clear
```

*split* - split a `LIB` file into `OBJ`s

```bash
$> psyx split PSX/LIB/LIBCARD.LIB
psyx version 0.0.0

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

*join* - join several `OBJ`s into a `LIB`

`unimplemented`

*add* - add another `OBJ` to an existing `LIB`

`unimplemented`

References
----------

sozud's [psy-q-splitter](https://github.com/sozud/psy-q-splitter) includes a `LIB` parser
and extractor.

[spirit t0aster's worklog](https://web.archive.org/web/20230428082811/https://www.psxdev.net/forum/viewtopic.php?t=1582)
provided additional information for several expression operators that are present in Psy-Q for Saturn but were not in
Psy-Q for Playstation.
