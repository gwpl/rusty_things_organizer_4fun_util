#![allow(unused_mut)]
use std::path::Path;
use std::{fs, io};

use whatwhere::WhatWhereMemDB;

pub mod cli;
pub mod whatwhere;
pub mod things_tree;

pub fn run_with_path_to_csv<R, W, P>(
    args: &[String],
    input: R,
    output: &mut W,
    things_csv_db_path: P,
) -> Result<(), Box<dyn std::error::Error>>
where
    R: io::Read,
    W: io::Write,
    P: AsRef<Path>,
{
    let mut thingscsvdb = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(things_csv_db_path)?;
    run_with_csv_file(&args, input, output, &mut thingscsvdb)
}

pub fn run_with_csv_file<R, W, F>(
    args: &[String],
    input: R,
    mut output: &mut W,
    mut thingscsvdb: &mut F,
    //mut thingscsvdb: &mut fs::File,
) -> Result<(), Box<dyn std::error::Error>>
where
    R: io::Read,
    W: io::Write,
    F: io::Read + io::Write + io::Seek,
{
    //use std::io::Seek;
    if args.len() < 2 {
        cli::display_help(&args, &mut io::stderr());
        return Ok(());
    }
    let mut memdb = WhatWhereMemDB::new();
    memdb.add_from_csv(&mut thingscsvdb)?;
    cli::parse_and_execute_updates(&args, &mut memdb, input, output)?;
    thingscsvdb
        .seek(io::SeekFrom::Start(0))
        .expect("Error in saving, tried to seek to beginning to file to save results");
    memdb.into_csv_from_db(thingscsvdb)?;
    //let current_position: u64 = thingscsvdb.stream_position()?;
    //thingscsvdb.set_len(current_position)?; // NOTE: Require File
    Ok(())
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use std::io::{self, Read, Seek, Write};
    use tempfile::NamedTempFile;

    #[test]
    fn batch_mode_with_csv_file_00() -> Result<(), Box<dyn Error>> {
        let mut outbuf = Vec::new();
        let mut csvf_tempfile_base = NamedTempFile::new()?;
        let csv_before = "What,Container,LastUpdate
a,A,TODO
b,A,TODO
c,A,TODO
x,X,TODO
y,X,TODO
z,X,TODO
";
        csvf_tempfile_base
            .reopen()
            .unwrap()
            .write_all(csv_before.as_bytes())?;
        // Arguments and stdin
        let args: Vec<String> = ["cmd", "b"].iter().map(|s| s.to_string()).collect();
        let input = io::Cursor::new(
            "B
b

C
c



Y
y

Z
z
",
        );
        let mut csv_file = csvf_tempfile_base.reopen()?;
        run_with_csv_file(&args[..], input, &mut outbuf, &mut csv_file)?;
        let csv_after_expected = "Container,What,LastUpdate
A,a,TODO
B,b,TODO
C,c,TODO
X,x,TODO
Y,y,TODO
Z,z,TODO
";
        let mut csv_after_result = String::new();
        csvf_tempfile_base
            .reopen()
            .unwrap()
            .read_to_string(&mut csv_after_result)?;
        assert_eq!(csv_after_result, csv_after_expected);
        Ok(())
    }

    #[test]
    fn search_mode_with_csv_file_00() -> Result<(), Box<dyn Error>> {
        let mut outbuf = Vec::new();
        let mut csvf_tempfile_base = NamedTempFile::new()?;
        let csv_before = "What,Container,LastUpdate
a,A,TODO
b,B,TODO
c,C,TODO
x,X,TODO
y,Y,TODO
z,Z,TODO
";
        csvf_tempfile_base
            .reopen()
            .unwrap()
            .write_all(csv_before.as_bytes())?;
        // Arguments and stdin
        let args: Vec<String> = ["cmd", "s"].iter().map(|s| s.to_string()).collect();
        let input = io::Cursor::new(
            "a
b
ItemNotInDB
y



z
",
        );
        let mut csv_file = csvf_tempfile_base.reopen()?;
        run_with_csv_file(&args[..], input, &mut outbuf, &mut csv_file)?;
        let output_expected = "A
B
Error: Not found!
Y
Z
";
        assert_eq!(String::from_utf8(outbuf)?, output_expected);
        Ok(())
    }
}
