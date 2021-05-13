#![allow(unused_mut)]
use std::{io,fs};
use std::path::Path;

mod whatwhere;

pub fn run_with_path<R, W, P>(
    input: R,
    mut output: W,
    things_db_path: P,
) -> Result<(), Box<dyn std::error::Error>> 
where
    R: io::Read,
    W: io::Write,
    P: AsRef<Path>
{
    let mut thingsdb = fs::OpenOptions::new().read(true).write(true).open(things_db_path)?;
    run_with_file(input, output, &mut thingsdb)
}

pub fn run_with_file<R, W, F>(
    _input: R,
    mut output: W,
    _thingsdb: &mut F,
) -> Result<(), Box<dyn std::error::Error>>
where
    R: io::Read,
    W: io::Write,
    F: io::Read+io::Write+io::Seek,
{
    writeln!(output, "Hello, world!")?;
    Ok(())
}

#[cfg(test)]
mod tests {}
