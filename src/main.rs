use rusty_things_organizer_4fun_util::*;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let things_db_path = "./thingsdb.csv";
    let input = io::stdin();
    let mut output = io::stdout();
    run_with_path_to_csv(&args, input, &mut output, things_db_path)
}
