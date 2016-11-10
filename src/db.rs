// Copyright 2016 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.


use std::env;
use std::fs::OpenOptions;
use std::path::Path;
use std::process;

use pad::{PadStr, Alignment};
use sqlite;


const DB_NAME: &'static str = "keep.db";
const CUST_KEEP_DB: &'static str = "KEEP_DB";


lazy_static! {
    static ref DB_PATH_STR: String = {
        // Check for custom install location
        let custom_result = env::var(CUST_KEEP_DB);
        if custom_result.is_ok() {
            return custom_result.unwrap() + "/" + DB_NAME;
        }

        // UNIX users first
        let unix_result = env::var("HOME");
        if unix_result.is_ok() {
            return unix_result.unwrap() + "/" + DB_NAME;
        }

        // Windows users second
        let win_result = env::var("HOMEPATH");
        if win_result.is_ok() {
            let home_drive = env::var("HOMEDRIVE").unwrap();
            let home = win_result.unwrap() + &home_drive + "/" + DB_NAME;
            return home;
        }

        // Display error message and close
        println!("Could not locate HOME, HOMEPATH, or KEEP_DB");
        println!("Please define KEEP_DB as the storage directory.");
        process::exit(0);
    };
}


pub fn insert(item: &String) {
    init();

    let query = format!("INSERT INTO stuff (tag, item) VALUES ('', '{}');",
                        item);
    execute_query(query);
}

pub fn insert_with_tag(tag: &String, item: &String) {
    init();

    let query = format!("INSERT INTO stuff (tag, item) VALUES ('{}', '{}');",
                        tag,
                        item);
    execute_query(query);
}

pub fn list_all() {
    init();

    let query = format!("SELECT * FROM stuff;");
    let c = sqlite::open(&Path::new(&*DB_PATH_STR)).unwrap();
    let mut cursor = c.prepare(query).unwrap().cursor();

    let mut pad_id = 2; // "id"
    let mut pad_tag = 3; // "tag"
    let mut pad_item = 4; // "item"
    let mut buf = Vec::<(String, String, String)>::new();
    while let Some(row) = cursor.next().unwrap() {
        let id = format!("{}", row[0].as_integer().unwrap());
        let tag = row[1].as_string().unwrap().to_owned();
        let item = row[2].as_string().unwrap().to_owned();

        if id.len() > pad_id { pad_id = id.len(); }
        if tag.len() > pad_tag { pad_tag = tag.len(); }
        if item.len() > pad_item { pad_item = item.len(); }

        buf.push((id, tag, item));
    }

    if buf.len() > 0 {
        println!("| {} | {} | {} |",
                 "id".pad_to_width_with_alignment(pad_id, Alignment::Middle),
                 "tag".pad_to_width_with_alignment(pad_tag, Alignment::Middle),
                 "item".pad_to_width_with_alignment(pad_item, Alignment::Middle));
    }

    for (id, tag, item) in buf {
        println!("| {} | {} | {} |",
                 id.pad_to_width(pad_id),
                 tag.pad_to_width(pad_tag),
                 item.pad_to_width(pad_item));
    }
}

pub fn list_with_tag(tag: &String) {
    init();

    let query = format!("SELECT * FROM stuff WHERE tag='{}';", tag);
    let c = sqlite::open(&Path::new(&*DB_PATH_STR)).unwrap();
    let mut cursor = c.prepare(query).unwrap().cursor();

    let mut pad_id = 2; // "id"
    let mut pad_item = 4; // "item"
    let mut buf = Vec::<(String, String)>::new();
    while let Some(row) = cursor.next().unwrap() {
        let id = format!("{}", row[0].as_integer().unwrap());
        let item = row[2].as_string().unwrap().to_owned();

        if id.len() > pad_id { pad_id = id.len(); }
        if item.len() > pad_item { pad_item = item.len(); }

        buf.push((id, item));
    }

    if buf.len() > 0 {
        println!("| {} | {} |",
                 "id".pad_to_width_with_alignment(pad_id, Alignment::Middle),
                 "item".pad_to_width_with_alignment(pad_item, Alignment::Middle));
    }

    for (id, item) in buf {
        println!("| {} | {} |",
                 id.pad_to_width(pad_id),
                 item.pad_to_width(pad_item));
    }
}

fn init() {
    create_db_if_not_exists();
}

fn create_db_if_not_exists() {
    let db_path = Path::new(&*DB_PATH_STR);
    if db_path.exists() { return; }

    create_db_file();
    initialize_db();
}

fn create_db_file() {
    let mut opts = OpenOptions::new();
    let _ = opts.append(true).create(true).open(&Path::new(&*DB_PATH_STR)).map_err(|e| {
        println!("Error creating db: {}", e)
    });
}

fn initialize_db() {
    let statement = "CREATE TABLE stuff (
	id	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	tag	TEXT NOT NULL,
	item	TEXT NOT NULL);";

    let c = sqlite::open(Path::new(&*DB_PATH_STR)).unwrap();
    let _ = c.execute(statement).unwrap();
}

fn execute_query(q: String) {
    let c = sqlite::open(&Path::new(&*DB_PATH_STR)).unwrap();
    let _ = c.execute(q).unwrap();
}
