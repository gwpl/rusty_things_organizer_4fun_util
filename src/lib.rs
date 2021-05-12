use std::io;
use std::path::Path;

pub fn run<R, W, P>(
    _input: R,
    mut output: W,
    things_db_path: P,
) -> Result<(), Box<dyn std::error::Error>>
where
    R: io::Read,
    W: io::Write,
    P: AsRef<Path>,
{
    let _thingsdb = std::fs::File::open(things_db_path);
    writeln!(output, "Hello, world!")?;
    Ok(())
}

#[cfg(test)]
mod tests {}
