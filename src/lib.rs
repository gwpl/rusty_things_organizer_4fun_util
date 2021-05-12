use std::io;

pub fn run<R: io::Read, W: io::Write> (_input: R, mut output: W) -> Result<(), Box<dyn std::error::Error>> {
    writeln!(output, "Hello, world!")?;
    Ok(())
}

#[cfg(test)]
mod tests {
}
