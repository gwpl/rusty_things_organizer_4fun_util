// cmd column_name key - greps rows in a way that they contain/include "key"
// i.e. abcde "includes/contains" key "bcd"

use std::error::Error;
use std::io;
use std::process;
use std::ffi::OsString;
use std::env;

fn run() -> Result<(), Box<dyn Error>> {
    let (column_name, key) = get_first_two_args()?;
    let key = key.to_string_lossy().to_string();
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut col_idx: usize = usize::MAX;
    {
        // We nest this call in its own scope because of lifetimes.
        let headers = rdr.headers()?;
        eprintln!("{:?}", headers);
        for (idx,col) in headers.iter().enumerate() {
            eprintln!("{} {:?}", idx, col);
            if col.eq(&column_name) {
                eprintln!("MATCH");
                col_idx = idx;
            }
        }
        wtr.write_record(headers)?;
    }
    if col_idx == usize::MAX {
        return Err(From::from("Column name didn't match any column in header"));
    }
    for result in rdr.records() {
        let record = result?;
        eprintln!("{:?} does it match agains `{:?}` ?", record, &record[col_idx]);
        if record[col_idx].to_string().contains(&key) {
            eprintln!("Matches!");
            wtr.write_record(&record)?;
        }
    }
    // We can ask for the headers at any time. There's no need to nest this
    // call in its own scope because we never try to borrow the reader again.
    let headers = rdr.headers()?;
    eprintln!("{:?}", headers);
    Ok(())
}

fn get_first_two_args() -> Result<(OsString,OsString), Box<dyn Error>> {
     match env::args_os().nth(1) {
        None => Err(From::from("expected 2 arguments")),
             Some(arg1) => {
                    match env::args_os().nth(2) {
                        None => Err(From::from("expected 2 arguments")),
                        Some(arg2) => Ok((arg1, arg2)),
                    }
             },
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        process::exit(1);
    }
}
