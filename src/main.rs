use rusty_things_organizer_4fun_util::*;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(io::stdin(), io::stdout())
}
