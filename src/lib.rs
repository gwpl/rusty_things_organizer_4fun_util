#![allow(unused_mut)]
use std::{io,fs};
use std::path::Path;

use whatwhere::WhatWhereMemDB;

mod whatwhere;
mod cli;

pub fn run_with_path_to_csv<R, W, P>(
    input: R,
    mut output: W,
    things_csv_db_path: P,
) -> Result<(), Box<dyn std::error::Error>> 
where
    R: io::Read,
    W: io::Write,
    P: AsRef<Path>
{
    let mut thingscsvdb = fs::OpenOptions::new().read(true).write(true).open(things_csv_db_path)?;
    run_with_csv_file(input, output, &mut thingscsvdb)
}

pub fn run_with_csv_file<R, W, F>(
    input: R,
    mut output: W,
    thingscsvdb: &mut F,
) -> Result<(), Box<dyn std::error::Error>>
where
    R: io::Read,
    W: io::Write,
    F: io::Read+io::Write+io::Seek,
{
    let args: Vec<String> = std::env::args().collect();
    let mut memdb = WhatWhereMemDB::new();
    memdb.add_from_csv(thingscsvdb)?;
    cli::parse_and_execute_updates(&args, &mut memdb, input, output)?;
    Ok(())
}

#[cfg(test)]
mod tests {}
