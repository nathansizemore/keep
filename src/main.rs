// Copyright 2016 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.


extern crate docopt;
extern crate rustc_serialize;
extern crate sqlite;


use docopt::Docopt;

mod db;


const USAGE: &'static str = "
keep - it keeps shit, so you can look at it later

Usage:
    keep save [--tag=<t>] <item>
    keep list [--tag=<t>]
    keep -h | --help
    keep --version

Options:
    -t --tag=<t>    Name to help identify item.
    -h --help       Show this screen.
    --version       Show version.
";

const VERSION_STR: &'static str = "keep 0.1.0

Copyright (C) 2016 Nathan Sizemore <nathanrsizemore@gmail.com>
License: MPL-2.0 https://www.mozilla.org/en-US/MPL/2.0
This is free software: you are free to change and redistribute it.";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub flag_tag: String,
    pub flag_version: bool,
    pub cmd_save: bool,
    pub cmd_list: bool,
    pub arg_item: String,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
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
        // Tagged
        if args.flag_tag.len() > 0 {
            db::list_with_tag(&args.flag_tag);
        } else {
            db::list_all();
        }
        return;
    }
}
