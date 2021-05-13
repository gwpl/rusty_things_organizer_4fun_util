#![allow(unused_imports)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io;
use std::vec::Vec;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "PascalCase")]
pub struct WhatWhereRecord {
    pub what: String,
    pub container: String, // because "where" is reserved keyword and previxing r# may suck
    pub last_update: String, // iso-8601 string
}

impl WhatWhereRecord {
    fn new(what: &str, container: &str, last_update: &str) -> WhatWhereRecord {
        WhatWhereRecord {
            what: what.to_string(),
            container: container.to_string(),
            last_update: last_update.to_string(),
        }
    }
}

pub fn load_from_csv<R>(input: R) -> Result<Vec<WhatWhereRecord>, Box<dyn Error>>
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

pub fn save_to_csv<'a, W, I>(output: W, iterator: I) -> Result<(), Box<dyn Error>>
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

#[cfg(test)]
mod whatwheretests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_load_from_csv_00() -> Result<(), Box<dyn Error>> {
        let mut buff = io::Cursor::new(
            "Container,What,LastUpdate
c01,t01,1901-01-01 12:53
c02,t02,\"2001-01-01 12:54\"
",
        );
        let v = load_from_csv(buff)?;
        assert_eq!(
            &v,
            &[
                WhatWhereRecord::new("t01", "c01", "1901-01-01 12:53"),
                WhatWhereRecord::new("t02", "c02", "2001-01-01 12:54"),
            ]
        );
        Ok(())
    }

    #[test]
    fn test_save_to_csv_00() -> Result<(), Box<dyn Error>> {
        let mut output_as_bytes: Vec<u8> = Vec::new();
        let records = [
            WhatWhereRecord::new("t01", "c01", "1901-01-01 12:53"),
            WhatWhereRecord::new("t02", "c02", "2001-01-01 12:54"),
        ];
        save_to_csv(&mut output_as_bytes, records.iter())?;
        let output_string = String::from_utf8(output_as_bytes).expect("Not UTF-8");
        assert_eq!(
            output_string,
            "What,Container,LastUpdate
t01,c01,1901-01-01 12:53
t02,c02,2001-01-01 12:54
"
        );
        Ok(())
    }
}
