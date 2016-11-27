// Copyright 2016 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.


extern crate docopt;
#[macro_use] extern crate lazy_static;
extern crate pad;
extern crate rustc_serialize;
extern crate sqlite;


use docopt::Docopt;

mod db;
mod entry;
mod print;


const USAGE: &'static str = "
keep - it keeps shit, so you can look at it later.

Usage:
    keep save [--tag=<t>] <item>
    keep list [--tag=<t>]
    keep rm [--all] [--tag=<t>] [<id>]
    keep -h | --help
    keep --version

Options:
    -a --all        Apply command to every entry.
    -t --tag=<t>    Name to help identify item.
    -h --help       Show this screen.
    --version       Show version.
";

const VERSION_STR: &'static str = "keep 0.1.1

Copyright (C) 2016 Nathan Sizemore <nathanrsizemore@gmail.com>
License: MPL-2.0 https://www.mozilla.org/en-US/MPL/2.0
This is free software: you are free to change and redistribute it.";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub flag_all: bool,
    pub flag_tag: String,
    pub flag_version: bool,
    pub cmd_save: bool,
    pub cmd_list: bool,
    pub cmd_rm: bool,
    pub arg_item: String,
    pub arg_id: u64
}


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    db::init();

    handle_input(&args);
}

fn handle_input(args: &Args) {
    // println!("{:?}", args);

    if args.flag_version {
        println!("{}", VERSION_STR);
        return;
    }

    // Insert op
    if args.cmd_save {
        if args.flag_tag.len() > 0 {
            db::insert_with_tag(&args.flag_tag, &args.arg_item);
        } else {
            db::insert(&args.arg_item);
        }
        return;
    }

    // List op
    if args.cmd_list {
        let entries = if args.flag_tag.len() > 0 {
            db::get_with_tag(&args.flag_tag)
        } else {
            db::get_all()
        };

        print::print_entries(entries);
        return;
    }

    // Remove op
    if args.cmd_rm {
        if args.flag_tag.len() > 0 { // By tag
            db::rm_with_tag(&args.flag_tag);
        } else if args.flag_all { // Everything
            db::rm_all();
        } else { // By id
            db::rm_with_id(args.arg_id);
        }
    }
}
