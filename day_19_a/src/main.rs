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

    for x in 0..50 {
        for y in 0..50 {
            let output = IntcodeComputer::new(&program).run(&[x, y], None)?;

            if output.data().len() > 0 {
                map.insert((x, y), output.data()[0]);
            }
        }
    }

    stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    for (pos, value) in &map {
        stdout()
            .execute(cursor::MoveTo(pos.0 as u16, pos.1 as u16))?
            .execute(PrintStyledContent(if *value == 1 {
                style('#').with(Color::Red)
            } else {
                style('.').with(Color::DarkGrey)
            }))?;
    }

    {
        let max_pos = map.keys().max_by(|a, b| a.1.cmp(&b.1)).ok_or("Error")?;
        stdout()
            .execute(cursor::MoveTo(max_pos.0 as u16, max_pos.1 as u16))?
            .execute(Print('\n'))?;
    }

    println!("{}", map.values().filter(|v| **v == 1).count());

    Ok(())
}
