// Copyright 2016 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0.
// If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.


/// Represents an entry in the SQLite db
pub struct Entry {
    /// Primary key of SQLite table entry
    pub id: u64,
    /// Tag associated with this item
    pub tag: String,
    /// Item saved
    pub item: String
}
