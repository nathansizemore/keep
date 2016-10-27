// Copyright 2016 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.


use std::env;
use std::fs::OpenOptions;
use std::path::Path;

use sqlite;


const DB_NAME: &'static str = "keep.db";
const CUST_KEEP_DB: &'static str = "KEEP_DB";


pub fn insert(item: &String) {
    if !init() { return; }

    let query = format!("INSERT INTO stuff (tag, item) VALUES ('', '{}');",
                        item);
    execute_query(query);
}

pub fn insert_with_tag(tag: &String, item: &String) {
    if !init() { return; }

    let query = format!("INSERT INTO stuff (tag, item) VALUES ('{}', '{}');",
                        tag,
                        item);
    execute_query(query);
}

pub fn list_all() {
    if !init() { return; }

    let db_dir = get_home_dir().unwrap();
    let db_path_str = get_db_path(&db_dir);
    let db_path = Path::new(&db_path_str);

    let query = format!("SELECT * FROM stuff;");
    let c = sqlite::open(&db_path).unwrap();
    let mut cursor = c.prepare(query).unwrap().cursor();
    while let Some(row) = cursor.next().unwrap() {
        println!("{}", row[1].as_string().unwrap());
    }
}

pub fn list_with_tag(tag: &String) {
    if !init() { return; }

    let db_dir = get_home_dir().unwrap();
    let db_path_str = get_db_path(&db_dir);
    let db_path = Path::new(&db_path_str);

    let query = format!("SELECT * FROM stuff WHERE tag='{}';", tag);
    let c = sqlite::open(&db_path).unwrap();
    let mut cursor = c.prepare(query).unwrap().cursor();
    while let Some(row) = cursor.next().unwrap() {
        println!("{}", row[1].as_string().unwrap());
    }
}

fn init() -> bool {
    let db_dir_res = get_home_dir();
    if db_dir_res.is_err() { return false; }

    let db_dir = db_dir_res.unwrap();
    create_db_if_not_exists(&db_dir);

    true
}

fn get_home_dir() -> Result<String, ()> {
    // Check for custom install location
    let custom_result = env::var(CUST_KEEP_DB);
    if custom_result.is_ok() {
        let home = custom_result.unwrap();
        return Ok(home);
    }

    // UNIX users first
    let unix_result = env::var("HOME");
    if unix_result.is_ok() {
        let home = unix_result.unwrap();
        return Ok(home);
    }

    // Windows users second
    let win_result = env::var("HOMEPATH");
    if win_result.is_ok() {
        let home_drive = env::var("HOMEDRIVE").unwrap();
        let home = win_result.unwrap() + &home_drive;
        return Ok(home);
    }

    // Display error message and close
    println!("Could not locate HOME, HOMEPATH, or KEEP_DB");
    println!("Please define KEEP_DB as the storage directory.");
    Err(())
}


fn create_db_if_not_exists(db_dir: &String) {
    let mut db_path_s = String::new();
    db_path_s.push_str(db_dir);
    db_path_s.push_str("/");
    db_path_s.push_str(DB_NAME);

    let db_path = Path::new(&db_path_s);
    if db_path.exists() { return; }
    create_db_file(&db_path);
    initialize_db(&db_path);
}

fn create_db_file(path: &Path) {
    let mut opts = OpenOptions::new();
    let _ = opts.append(true).create(true).open(path).map_err(|e| {
        println!("Error creating db: {}", e)
    });
}

fn initialize_db(path: &Path) {
    let c = sqlite::open(path).unwrap();
    let _ = c.execute("CREATE TABLE stuff (tag TEXT, item TEXT);").unwrap();
}

fn get_db_path(db_dir: &String) -> String {
    let mut db_path_s = String::new();
    db_path_s.push_str(db_dir);
    db_path_s.push_str("/");
    db_path_s.push_str(DB_NAME);
    db_path_s
}

fn execute_query(q: String) {
    let db_dir = get_home_dir().unwrap();
    let db_path_str = get_db_path(&db_dir);
    let db_path = Path::new(&db_path_str);

    let c = sqlite::open(&db_path).unwrap();
    let _ = c.execute(q).unwrap();
}
