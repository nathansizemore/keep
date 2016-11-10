// Copyright 2016 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.


use std::env;
use std::fs::OpenOptions;
use std::mem;
use std::path::Path;
use std::process;

use pad::PadStr;
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


pub fn init() {
    create_db_if_not_exists();
}

pub fn insert(item: &String) {
    let query = format!("INSERT INTO stuff (tag, item) VALUES ('', '{}');",
                        item);
    execute_query(query);
}

pub fn insert_with_tag(tag: &String, item: &String) {
    let query = format!("INSERT INTO stuff (tag, item) VALUES ('{}', '{}');",
                        tag,
                        item);
    execute_query(query);
}

pub fn list_all() {
    let query = format!("SELECT * FROM stuff;");
    run_list_query(&query);
}

pub fn list_with_tag(tag: &String) {
    let query = format!("SELECT * FROM stuff WHERE tag='{}';", tag);
    run_list_query(&query);
}

fn run_list_query(query: &String) {
    let mut buf = get_all_matching_query(&query);
    let (pad_id, pad_tag, pad_item) = get_padding(&buf);
    pad_list(&mut buf, pad_id, pad_tag, pad_item);
    print_buf(&buf);
}

fn get_all_matching_query(query: &String) -> Vec<(String, String, String)> {
    let c = sqlite::open(&Path::new(&*DB_PATH_STR)).unwrap();
    let mut cursor = c.prepare(query).unwrap().cursor();

    let mut buf = Vec::<(String, String, String)>::new();
    buf.push(("id".to_owned(), "tag".to_owned(), "item".to_owned()));

    while let Some(row) = cursor.next().unwrap() {
        let id = format!("{}", row[0].as_integer().unwrap());
        let tag = row[1].as_string().unwrap().to_owned();
        let item = row[2].as_string().unwrap().to_owned();

        buf.push((id, tag, item));
    }

    buf
}

fn get_padding(buf: &Vec<(String, String, String)>) -> (usize, usize, usize) {
    let mut pad_id = 0;
    let mut pad_tag = 0;
    let mut pad_item = 0;

    for &(ref id, ref tag, ref item) in buf {
        if id.len() > pad_id { pad_id = id.len(); }
        if tag.len() > pad_tag { pad_tag = tag.len(); }
        if item.len() > pad_item { pad_item = item.len(); }
    }

    (pad_id, pad_tag, pad_item)
}

fn pad_list(buf: &mut Vec<(String, String, String)>,
            pad_id: usize,
            pad_tag: usize,
            pad_item: usize)
{
    for &mut (ref mut id, ref mut tag, ref mut item) in buf {
        let new_id = id.pad_to_width(pad_id);
        let new_tag = tag.pad_to_width(pad_tag);
        let new_item = item.pad_to_width(pad_item);

        mem::replace(id, new_id);
        mem::replace(tag, new_tag);
        mem::replace(item, new_item);
    }
}

fn print_buf(buf: &Vec<(String, String, String)>) {
    for &(ref id, ref tag, ref item) in buf {
        println!("| {} | {} | {} |", id, tag, item);
    }
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
    create_table_version();
    create_table_stuff();
}

fn create_table_version() {
    let statement_version = "CREATE TABLE version(
        id         INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
        version    TEXT NOT NULL);".to_owned();

    execute_query(statement_version);
}

fn create_table_stuff() {
    let statement_stuff = "CREATE TABLE stuff(
	id	INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT UNIQUE,
	tag	TEXT NOT NULL,
	item	TEXT NOT NULL);".to_owned();

    execute_query(statement_stuff);
}

fn execute_query(q: String) {
    let c = sqlite::open(&Path::new(&*DB_PATH_STR)).unwrap();
    let _ = c.execute(q).unwrap();
}
