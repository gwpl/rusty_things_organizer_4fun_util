#![allow(unused_imports)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::error::Error;
use std::io;
use std::vec::Vec;
use crate::things_tree::{ThingsTree,IntoAsciiTree};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WhatWhereRecord {
    pub container: String, // because "where" is reserved keyword and previxing r# may suck
    pub what: String,
    pub last_update: String, // iso-8601 string //TODO: make custom type Iso8601String with TryFrom,Into,AsRef,custom serde visitor and serialize impl
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ThingProperties {
    pub container: String,
    pub last_update: String,
}

impl ThingProperties {
    pub fn new(container: String, last_update: String) -> ThingProperties {
        ThingProperties {
            container,
            last_update,
        }
    }
}

// TODO: make Key a custom type (?)
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct WhatWhereMemDB {
    db: BTreeMap<String, ThingProperties>, // key is what , value is "where" == "container"
    tree: ThingsTree, // to keep track of tree structure for ascii_tree rendering (TODO make it as feature "ascii_tree")
}

impl WhatWhereRecord {
    pub fn new(what: &str, container: &str, last_update: &str) -> WhatWhereRecord {
        WhatWhereRecord {
            what: what.to_string(),
            container: container.to_string(),
            last_update: last_update.to_string(),
        }
    }
    pub fn from_things_properties(
        what: &str,
        thing_properties: &ThingProperties,
    ) -> WhatWhereRecord {
        WhatWhereRecord {
            what: what.to_string(),
            container: thing_properties.container.to_string(),
            last_update: thing_properties.last_update.to_string(),
        }
    }
}

fn load_from_csv_to_vec<R>(input: R) -> Result<Vec<WhatWhereRecord>, Box<dyn Error>>
where
    R: io::Read,
{
    let mut v: Vec<_> = Vec::new();
    let mut rdr = csv::Reader::from_reader(input);
    for result in rdr.deserialize() {
        let record: WhatWhereRecord = result?;
        v.push(record);
    }
    Ok(v)
}

fn save_to_csv_from_vec<'a, W, I>(output: &mut W, iterator: I) -> Result<(), Box<dyn Error>>
where
    W: io::Write,
    I: Iterator<Item = &'a WhatWhereRecord>,
{
    let mut wtr = csv::Writer::from_writer(output);
    for record in iterator {
        wtr.serialize(record)?;
    }
    Ok(())
}

pub trait UpdatableDB {
    fn update(&mut self, container: &str, what: &str, current_update: &str);
}

pub trait SearchableDB {
    fn search_by_thing_code(&self, what: &str) -> Option<&str>;
}

impl UpdatableDB for WhatWhereMemDB {
    fn update(&mut self, container: &str, what: &str, current_update: &str) {
        self.db.insert(
            what.to_string(),
            ThingProperties::new(container.to_string(), current_update.to_string()),
        );
        self.tree.insert(container, what);
    }
}

impl SearchableDB for WhatWhereMemDB {
    fn search_by_thing_code(&self, what: &str) -> Option<&str> {
        match self.db.get(what) {
            Some(tprop) => Some(&tprop.container),
            None => None,
        }
    }
}

impl IntoAsciiTree for WhatWhereMemDB {
    fn into_ascii_tree(&self, root: &str) -> ascii_tree::Tree {
        self.tree.into_ascii_tree(root)
    }
    fn into_ascii_forest(&self) -> String {
        self.tree.to_string()
    }
}

impl WhatWhereMemDB {
    pub fn new() -> WhatWhereMemDB {
        WhatWhereMemDB {
            db: BTreeMap::new(),
            tree: ThingsTree::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.db.len()
    }

    // Adds elements from csv reader (e.g. file).
    // If elements alredy exised, they will be updated with new values
    // and `last_updated` entry is ignored in decision making.
    pub fn add_from_csv<R>(&mut self, input: R) -> Result<(), Box<dyn Error>>
    where
        R: io::Read,
    {
        let mut rdr = csv::Reader::from_reader(input);
        for result in rdr.deserialize() {
            let record: WhatWhereRecord = result?;
            self.update(&record.container, &record.what, &record.last_update);
        }
        Ok(())
    }

    // dumps as csv into writer
    pub fn into_csv_from_db<W>(&self, output: &mut W) -> Result<(), Box<dyn Error>>
    where
        W: io::Write,
    {
        let mut wrt = csv::Writer::from_writer(output);
        for (what, tprop) in &self.db {
            let record: WhatWhereRecord = WhatWhereRecord::from_things_properties(what, tprop);
            wrt.serialize(&record)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod whatwheretests {
    use super::*;
    use pretty_assertions::assert_eq;

    const CSV00IN: &str = "LastUpdate,Container,What
1901-01-01 12:53,c01,t01
\"2001-01-01 12:54\",c02,t02
";

    const CSV00OUT: &str = "Container,What,LastUpdate
c01,t01,1901-01-01 12:53
c02,t02,2001-01-01 12:54
";

    fn records00() -> [WhatWhereRecord; 2] {
        [
            WhatWhereRecord::new("t01", "c01", "1901-01-01 12:53"),
            WhatWhereRecord::new("t02", "c02", "2001-01-01 12:54"),
        ]
    }

    #[test]
    fn test_load_from_csv_to_vec_00() -> Result<(), Box<dyn Error>> {
        let mut buff = io::Cursor::new(CSV00IN);
        let v = load_from_csv_to_vec(buff)?;
        assert_eq!(&v, &records00());
        Ok(())
    }

    #[test]
    fn test_save_to_csv_from_vec_00() -> Result<(), Box<dyn Error>> {
        let mut output_as_bytes: Vec<u8> = Vec::new();
        let records = records00();
        save_to_csv_from_vec(&mut output_as_bytes, records.iter())?;
        let output_string = String::from_utf8(output_as_bytes).expect("Not UTF-8");
        assert_eq!(output_string, CSV00OUT);
        Ok(())
    }

    #[test]
    fn test_add_from_csv_to_db_00() -> Result<(), Box<dyn Error>> {
        let mut db = WhatWhereMemDB::new();
        let mut buff = io::Cursor::new(CSV00IN);
        db.add_from_csv(buff)?;
        assert_eq!(db.search_by_thing_code("t01"), Some("c01"));
        assert_eq!(db.search_by_thing_code("t02"), Some("c02"));
        assert_eq!(db.len(), 2);
        Ok(())
    }

    #[test]
    fn test_add_from_csv_to_ascii_tree_forest_string_00() -> Result<(), Box<dyn Error>> {
        let mut db = WhatWhereMemDB::new();
        let mut buff = io::Cursor::new(CSV00IN);
        db.add_from_csv(buff)?;
        let forest_string = db.tree.to_string();
        assert_eq!(
forest_string.trim(),
"
c01
 ?????? t01
 c02
 ?????? t02
".trim()
);
        Ok(())
    }

    #[test]
    fn test_to_ascii_tree_forest_string_00() -> Result<(), Box<dyn Error>> {
        let mut db = WhatWhereMemDB::new();
        db.update("box_ABC", "thing_A","TODO");
        db.update("box_ABC", "thing_B","TODO");
        db.update("box_ABC", "thing_C","TODO");
        db.update("storage","box_ABC", "TODO");
        db.update("storage", "box_X", "TODO");
        db.update("box_X", "thing_Y","TODO");
        db.update("box_X", "thing_Z","TODO");
        db.update("box_W", "thing_V","TODO");
        let forest_string = db.tree.to_string();
        assert_eq!(
forest_string.trim(),
"
 box_W
 ?????? thing_V
 storage
 ?????? box_ABC
 ???  ?????? thing_A
 ???  ?????? thing_B
 ???  ?????? thing_C
 ?????? box_X
    ?????? thing_Y
    ?????? thing_Z
".trim()
);
        Ok(())
    }

    #[test]
    fn test_into_csv_from_db_00() -> Result<(), Box<dyn Error>> {
        let mut output_as_bytes: Vec<u8> = Vec::new();
        let mut db = WhatWhereMemDB::new();
        for r in &records00() {
            db.update(&r.container, &r.what, &r.last_update);
        }
        db.into_csv_from_db(&mut output_as_bytes)?;
        let output_string = String::from_utf8(output_as_bytes)?;
        assert_eq!(output_string, CSV00OUT);
        Ok(())
    }
}
