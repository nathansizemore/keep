// Copyright 2016 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.


use std::env;
use std::fs::OpenOptions;
use std::path::Path;
use std::process;

use entry::Entry;

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

pub fn get_all() -> Vec<Entry> {
    let query = format!("SELECT * FROM stuff;");
    get_all_matching_query(&query)
}

pub fn get_with_tag(tag: &String) -> Vec<Entry> {
    let query = format!("SELECT * FROM stuff WHERE tag='{}';", tag);
    get_all_matching_query(&query)
}

pub fn rm_with_tag(tag: &String) {
    let query = format!("DELETE FROM stuff WHERE tag='{}';", tag);
    execute_query(query);
}

pub fn rm_with_id(id: u64) {
    let query = format!("DELETE FROM stuff WHERE id={};", id);
    execute_query(query);
}

pub fn rm_all() {
    let query = format!("DELETE FROM stuff;");
    execute_query(query);
}

fn get_all_matching_query(query: &String) -> Vec<Entry> {
    let c = sqlite::open(&Path::new(&*DB_PATH_STR)).unwrap();
    let mut cursor = c.prepare(query).unwrap().cursor();

    let mut buf = Vec::<Entry>::new();
    while let Some(row) = cursor.next().unwrap() {
        buf.push(Entry {
            id: row[0].as_integer().unwrap() as u64,
            tag: row[1].as_string().unwrap().to_owned(),
            item: row[2].as_string().unwrap().to_owned()
        });
    }

    buf
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
    execute_query("INSERT INTO version (version) VALUES ('0.1.0');".to_owned());
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
