use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    style::{style, Color, Print, PrintStyledContent},
    terminal, ExecutableCommand,
};
use int_comp::{IntcodeComputer, IntcodeOutput};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut int_comp = IntcodeComputer::new(&program);

    Ok(())
}
