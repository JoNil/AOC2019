use crossterm::{
    cursor,
    style::{style, Color, Print, PrintStyledContent},
    terminal, ExecutableCommand,
};
use int_comp::IntcodeComputer;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::stdout;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut map: HashMap<(i64, i64), i64> = HashMap::new();

    IntcodeComputer::new(&program).run(&[x, y], None)?;

    Ok(())
}
