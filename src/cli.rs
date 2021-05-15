use crate::whatwhere::{SearchableDB, UpdatableDB};
use std::error::Error;
use std::io;

#[allow(unused_variables)]
pub fn parse_and_execute_updates<UDB, R, W>(
    args: &[String],
    db: &mut UDB,
    input: R,
    mut output: W,
) -> Result<(), Box<dyn Error>>
where
    R: io::Read,
    W: io::Write,
    UDB: UpdatableDB + SearchableDB,
{
    Ok(())
}

#[cfg(test)]
#[allow(unused_imports)]
mod cli_tests {
    use crate::whatwhere::WhatWhereMemDB;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn add_things_from_cli() -> Result<(), Box<dyn Error>> {
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
}
