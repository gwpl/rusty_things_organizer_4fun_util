use rusty_things_organizer_4fun_util::*;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let things_db_path = "./thingsdb.csv";
    run(io::stdin(), io::stdout(), things_db_path)
}
