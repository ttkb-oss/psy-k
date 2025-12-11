// SPDX-FileCopyrightText: © 2025 TTKB, LLC
// SPDX-License-Identifier: BSD-3-CLAUSE

use std::path::Path;

use psyx::io;
use psyx::link;
use psyx::link::Command;
use psyx::link::Comment;

fn get_command_at(lines: &[(Option<Command>, Option<Comment>)], line: usize) -> &Command {
    let Some((Some(command), _)) = lines.get(line - 1) else {
        panic!("line {}: {:?}", line, lines.get(6));
    };
    command
}

fn read_lnk(path: &str) -> Vec<(Option<Command>, Option<Comment>)> {
    io::read_bytes(Path::new(path))
        // poor man's ISO-8859-1 to Unicode converter
        .map(|bytes| bytes.iter().map(|&c| c as char).collect::<String>())
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(|line| {
            let mut line = line;
            link::parse_line(&mut line).unwrap()
        })
        .collect::<Vec<(Option<Command>, Option<Comment>)>>()
}

#[test]
fn test_menu_lnk_33() {
    let path = "tests/data/psy-q/3.3/PSX/SAMPLE/MODULE/OVERMENU/MENU.LNK";
    let lnk_script = read_lnk(path);

    assert!(matches!(lnk_script.first(), Some((None, Some(_)))));

    let Command::Origin { address } = get_command_at(&lnk_script, 7) else {
        panic!("line: {:?}", lnk_script.get(6));
    };
    assert_eq!(0x80010000, *address);

    let Command::Group {
        name,
        attributes: _,
    } = get_command_at(&lnk_script, 10)
    else {
        panic!("line: {:?}", lnk_script.get(6));
    };
    assert_eq!("bss".to_string(), *name);
}

#[test]
fn test_lnk_parse_sanity() {
    read_lnk("tests/data/psy-q/3.3/PSX/SAMPLE/MODULE/OVERMENU/MENU.LNK");
    read_lnk("tests/data/psy-q/3.3/PSX/SAMPLE/ETC/CARD/MAKECARD/MAKECARD.LNK");
    read_lnk("tests/data/psy-q/3.3/PSX/SAMPLE/ETC/CARD/MAX/MENU.LNK");
    read_lnk("tests/data/psy-q/3.3/PSX/SAMPLE/ETC/CARD/MAX/MAIN.LNK");
    read_lnk("tests/data/psy-q/3.3/PSX/SAMPLE/CMPLR/SCRATCH/MAIN.LNK");
    read_lnk("tests/data/psy-q/3.3/PSX/SAMPLE/CMPLR/SCRATCH/SCRATCH.LNK");
    read_lnk("tests/data/psy-q/3.3/PSX/UTILITY/MENU/CDMENU2.LNK");
    read_lnk("tests/data/psy-q/3.3/PSX/UTILITY/MENU/CDMENU.LNK");
    read_lnk("tests/data/psy-q/3.3/PSX/UTILITY/MENU/PCMENU.LNK");
    read_lnk("tests/data/psy-q/3.5/PSYQ/SAMPLE/DEBUGGER/OVERLAY/MAIN.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/MODULE/OVERMENU/MENU.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/ETC/CARD/MAKECARD/MAKECARD.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/ETC/CARD/MAX/MENU.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/ETC/CARD/MAX/MAIN.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/SCEE/ETC/CARDCONF/MAIN.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/SCEE/PAL/MAIN.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/SCEE/UTILS/EXCEPT/TEST.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/SCEE/UTILS/PROFILER/PROTEST.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/SCEE/DEMODISC/DEMO/EXAMPLE/HARNESS.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/SCEE/DEMODISC/DEMO/BS/BS.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/SCEE/SUBDIV/MAIN.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/SCEE/KCHEATS/MAIN.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/CMPLR/SCRATCH/MAIN.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/SAMPLE/CMPLR/SCRATCH/SCRATCH.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/UTILITY/MENU/CDMENU2.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/UTILITY/MENU/CDMENU.LNK");
    read_lnk("tests/data/psy-q/3.6/PSX/UTILITY/MENU/PCMENU.LNK");
    read_lnk("tests/data/psy-q/3.6/PSYQ/SAMPLE/DEBUGGER/OVERLAY/MAIN.LNK");
    read_lnk("tests/data/psy-q/4.0/PSX/SAMPLE/MODULE/OVERMENU/MENU.LNK");
    read_lnk("tests/data/psy-q/4.0/PSX/SAMPLE/OLD/ETC/CARD/MAKECARD/MAKECARD.LNK");
    read_lnk("tests/data/psy-q/4.0/PSX/SAMPLE/OLD/ETC/CARD/MAX/MENU.LNK");
    read_lnk("tests/data/psy-q/4.0/PSX/SAMPLE/OLD/ETC/CARD/MAX/MAIN.LNK");
    read_lnk("tests/data/psy-q/4.0/PSX/SAMPLE/CMPLR/SCRATCH/APDL.LNK");
    read_lnk("tests/data/psy-q/4.0/PSX/SAMPLE/CMPLR/SCRATCH/MAIN.LNK");
    read_lnk("tests/data/psy-q/4.0/PSX/SAMPLE/CMPLR/SCRATCH/APDS.LNK");
    read_lnk("tests/data/psy-q/4.0/PSX/SAMPLE/CMPLR/SCRATCH/SCRATCH.LNK");
    read_lnk("tests/data/psy-q/4.0/PSX/UTILITY/MENU/CDMENU2.LNK");
    read_lnk("tests/data/psy-q/4.0/PSX/UTILITY/MENU/CDMENU.LNK");
    read_lnk("tests/data/psy-q/4.0/PSX/UTILITY/MENU/PCMENU.LNK");
    read_lnk("tests/data/psy-q/4.0/PSYQ/PREFSMPL/MENU.LNK");
    read_lnk("tests/data/psy-q/4.0/PSYQ/PREFSMPL/SN/MENU.LNK");
}
