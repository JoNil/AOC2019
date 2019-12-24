use int_comp::IntcodeComputer;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut incode = IntcodeComputer::new(&program);

    let _output = incode.run(&[], None)?;

    Ok(())
}
