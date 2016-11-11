// Copyright 2016 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.


use pad::PadStr;

use entry::Entry;


pub fn print_entries(entries: Vec<Entry>) {
    if entries.len() == 0 { return; }

    let (pad_id, pad_tag, pad_item) = get_padding(&entries);
    print_heading(pad_id, pad_tag, pad_item);

    for entry in entries {
        println!("| {} | {} | {} |",
                 format!("{}", entry.id).pad_to_width(pad_id),
                 entry.tag.pad_to_width(pad_tag),
                 entry.item.pad_to_width(pad_item));
    }
}

fn get_padding(entries: &Vec<Entry>) -> (usize, usize, usize) {
    // Their default padding values are at least the length of their string
    // representation in the column headers.
    // | id | tag | item |
    let mut pad_tag = 3;
    let mut pad_item = 4;

    for entry in entries {
        if entry.tag.len() > pad_tag { pad_tag = entry.tag.len();}
        if entry.item.len() > pad_item { pad_item = entry.item.len(); }
    }

    let mut pad_id = 1;
    let mut id_len = entries.len();
    if id_len >= 100000000 { pad_id += 8; id_len /= 100000000; }
    if id_len >= 10000 { pad_id += 4; id_len /= 10000; }
    if id_len >= 100 { pad_id += 2; id_len /= 100; }
    if id_len >= 10 { pad_id += 1; }

    // Our "id" string is at least 2 in len
    if pad_id < 2 { pad_id = 2; }

    (pad_id, pad_tag, pad_item)
}

fn print_heading(pad_id: usize, pad_tag: usize, pad_item: usize) {
    let heading = format!("| {} | {} | {} |",
                          "id".pad_to_width(pad_id),
                          "tag".pad_to_width(pad_tag),
                          "item".pad_to_width(pad_item));

    let total_len = heading.len();

    println!("{}", heading);
    for _ in 0..total_len {
        print!("-");
    }
    println!("");
}
