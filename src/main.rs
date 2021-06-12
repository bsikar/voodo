#![allow(dead_code)]
mod parser;

#[macro_use]
extern crate clap;

use clap::{Arg, SubCommand};
use std::fmt::{self, Display, Formatter};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

struct Voodo {
    file: std::fs::File,
    path: PathBuf,
}

impl Voodo {
    fn open(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();

        let file = OpenOptions::new()
            .write(true)
            .read(true)
            .create(true)
            .open(path)
            .expect("cannot open ~/.voodo");

        Voodo::reorder(&file);

        Voodo {
            file,
            path: path.to_owned(),
        }
    }

    fn reorder(file: &File) {}

    fn add_todo(&self) {}
}

impl Display for Voodo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "peepee")
    }
}

fn main() {
    let matches = app_from_crate!()
        .subcommand(
            SubCommand::with_name("add").arg(Arg::with_name("name").index(1).help("Add new todo")),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .arg(Arg::with_name("name").index(1).help("Remove todo at index"))
                .arg(
                    Arg::with_name("first")
                        .help("Removes the first <lines> lines")
                        .short("f")
                        .long("first")
                        .value_name("lines")
                        .default_value("1"),
                ),
        )
        .arg(Arg::with_name("edit").help("Enter a edit repl"))
        .arg(Arg::with_name("undo").help("Undo last change"))
        .arg(Arg::with_name("soon").help("Get what's due soon"))
        .get_matches();

    let voodo = Voodo::open(
        dirs::home_dir()
            .expect("failed to resolve $HOME")
            .join(".voodo"),
    );

    if let Some(matches) = matches.subcommand_matches("add") {
        // shit out a todo
    } else if let Some(matches) = matches.subcommand_matches("rm") {
        if let Some(value) = matches.value_of("name") {
            // parsing
        } else if let Some(value) = matches.value_of("first") {
            let n: usize = value.parse().expect("failed to parse argument of first");

            let lines: Vec<String> = BufReader::new(&voodo.file)
                .lines()
                .skip(n)
                .map(|x| x.unwrap())
                .collect();

            fs::write(voodo.path, lines.join("\n")).expect("failed to write to file");
        }
    } else {
        dbg!("else branch");
        // check edit
        // check undo
        // check soon
    }

    
}


/*
voodo add "English: I need to make a thing: @"

voodo rm "E: asgn*
voodo rm "E: as1" "E: as2"
rm -e *=
life: school: class:
life: work:
life: programming:
gaming: rotmg:
gaming: minecraft: build house

english:
gaming: roblox:
hist:

add "G: my todo @ 16:20"
add "l:s:h: my todo @ 4:20 PM"

rm E: asd_*

voodo rm -e asd*

voodo undo
rm oopsy
rm undo
a:s:t:*

rm engling: assignment
rm * // important sidenote disallow creating todo named "*"

voodo rm *

for i in {{0..123}}; do voodo rm $i; done

voodo add a b c d e f g
[a,b,c,d,e,f,g]

voodo add "english      : essay @ 6/11 10:00"
voodo add rust tetris 6/15 21:00
voodo soon
voodo add -i 1ly@45mph 1h 10ns 10us 10ps 10


-- another project: make notifcations based on this (voodod)
    voodo add english esssay 6/11 10:00 -w3h -w1d -1w

CRATES:
pdf / latex? -> for printing
time parsing

*/
