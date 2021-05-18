use crate::whatwhere::{SearchableDB, UpdatableDB};
use std::io;
use std::{error::Error, io::BufRead};

pub enum ModeCommands {
    BatchMode,
    SearchMode,
    //ContentsMode, //TODO
    UnknownMode,
}

pub fn display_help(args: &[String]) {
    let cmd: &str = if args.len() >= 1 { &args[0] } else { "command" };
    eprintln!(
        "Usage:
{0} b # to enter batch mode
{0} s # to search for things

Batch mode:
Provide container code folowed by codes of things to assign to it (in separate lines).
To enter new container, provide one or more empty lines.

Search mode:
each line specify code of thing to search.
",
        cmd
    );
}

//TODO: refactor argument parsing to use impl IntoIterator<Item=AsRef<str>> and return custom struct
pub fn parse_mode_command(cmd: &str) -> ModeCommands {
    use ModeCommands::*;
    match cmd.as_ref() {
        "b" => BatchMode,
        "s" => SearchMode,
        // "c" => ContentsMode,
        _ => UnknownMode,
    }
    //if cmd == "b" {
    //    BatchMode
    //} else if cmd == "s" {
    //    SearchMode
    //} else {
    //    UnknownMode
    //}
}

pub fn parse_and_execute_updates<DB, R, W>(
    args: &[String],
    db: &mut DB,
    input: R,
    mut output: &mut W,
) -> Result<(), Box<dyn Error>>
where
    R: io::Read,
    W: io::Write,
    DB: UpdatableDB + SearchableDB,
{
    use ModeCommands::*;
    if args.len() < 2 {
        return Err("No command provided".into());
    }
    match parse_mode_command(&args[1]) {
        BatchMode => process_batch(&args, db, input, output),
        SearchMode => process_search(&args, db, input, output),
        UnknownMode => Err("Unknown command".into()),
    }
}

fn process_batch<UDB, R, W>(
    _args: &[String],
    db: &mut UDB,
    input: R,
    mut _output: &W,
) -> Result<(), Box<dyn Error>>
where
    R: io::Read,
    W: io::Write,
    UDB: UpdatableDB,
{
    let current_update = "TODO"; // timestamp marker of this session
    let input = io::BufReader::new(input);
    let mut container = String::new();
    for line in input.lines() {
        let line: String = match line {
            Ok(string) => string,
            Err(_) => continue,
        };
        if line.trim() == "" {
            eprintln!("Empty line detected. Waiting for new container code.");
            container = "".into();
        }
        if container == "" {
            //if container != "" {
            // TODO: contianer contents
            //    let contents : Vec<String> = db.contents(container);
            //    eprintln!("Current `{}` container contents : {:?}", container, contents);
            //}
            container = line;
            eprintln!("Waiting for things to assign to container: {:?}", container);
        } else {
            let thing = line;
            eprintln!("Thing `{}` assined to container `{}`", thing, container);
            db.update(&container, &thing, current_update);
        }
    }
    Ok(())
}

#[allow(warnings)]
fn process_search<SDB, R, W>(
    _args: &[String],
    db: &mut SDB,
    input: R,
    mut output: &mut W,
) -> Result<(), Box<dyn Error>>
where
    R: io::Read,
    W: io::Write,
    SDB: SearchableDB,
{
    let input = io::BufReader::new(input);
    for line in input.lines() {
        let line: String = match line {
            Ok(string) => string.trim().into(),
            Err(_) => continue,
        };
        if line == "" {
            continue;
        };
        match db.search_by_thing_code(&line) {
            Some(container) => writeln!(&mut output, "{}", container),
            None => writeln!(&mut output, "Error: Not found!"),
        };
    }
    Ok(())
}

#[cfg(test)]
#[allow(unused_imports)]
mod cli_tests {
    use crate::whatwhere::WhatWhereMemDB;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn cli_batch_mode_test_00() -> Result<(), Box<dyn Error>> {
        let input = "container0x
thing00
thing01
thing02



container1x
thing10
thing11
thing12

container2x
thing20

container3x";
        //let args: Vec<String> = vec!["rustythings_foo_bar.bin".to_string(), "b".to_string()];
        let args: Vec<String> = ["rustythings_foo_bar.bin", "b"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let mut memdb = WhatWhereMemDB::new();
        let mut output: Vec<_> = Vec::new();
        let cursor_input = io::Cursor::new(input);
        parse_and_execute_updates(&args, &mut memdb, cursor_input, &mut output)?;
        let i = [
            ("container0x", ["thing00", "thing01", "thing02"]),
            ("container1x", ["thing10", "thing11", "thing12"]),
            ("container2x", ["thing20", "", ""]),
            ("container3x", ["", "", ""]),
        ];
        let v: Vec<_> = i.iter().fold(Vec::<(&str, &str)>::new(), |acc, (a, l)| {
            l.iter().fold(acc, |mut acc2, b| {
                if *b != "" {
                    acc2.push((a, b))
                };
                acc2
            })
        });
        for (container_code, thing_code) in v {
            assert_eq!(memdb.search_by_thing_code(thing_code), Some(container_code));
        }
        Ok(())
    }

    #[test]
    fn cli_search_mode_test_00() -> Result<(), Box<dyn Error>> {
        let input = "thing01
thing02
thingABC

thing03
";
        //let args: Vec<String> = vec!["rustythings_foo_bar.bin".to_string(), "b".to_string()];
        let args: Vec<String> = ["rustythings_foo_bar.bin", "s"]
            .iter()
            .map(|x| x.to_string())
            .collect();
        let mut memdb = WhatWhereMemDB::new();
        let mut output: Vec<_> = Vec::new();
        let cursor_input = io::Cursor::new(input);
        memdb.update("container01", "thing01", "TimestampX");
        memdb.update("container02", "thing02", "TimestampX");
        memdb.update("container03", "thing03", "TimestampX");
        parse_and_execute_updates(&args, &mut memdb, cursor_input, &mut output)?;
        let output_expected = "container01
container02
Error: Not found!
container03
";
        assert_eq!(String::from_utf8(output)?, output_expected);
        Ok(())
    }
}
