use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let _input = fs::read_to_string("input")?;

    Ok(())
}
