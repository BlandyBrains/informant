use std::env;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    io::stdout().write_all(b"test output")?;

    Ok(())
}