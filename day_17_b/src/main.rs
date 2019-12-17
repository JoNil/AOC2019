use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    style::{style, Color, Print, PrintStyledContent},
    terminal, ExecutableCommand,
};
use int_comp::IntcodeComputer;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::stdout;

fn neighbors(pos: (i32, i32), map: &HashSet<(i32, i32)>) -> impl Iterator<Item = (i32, i32)> {
    [
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ]
    .into_iter()
    .filter(|candidate| map.contains(candidate))
    .copied()
    .collect::<Vec<_>>()
    .into_iter()
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut int_comp = IntcodeComputer::new(&program);

    let output = int_comp.run(&[], None)?;

    let mut map = HashSet::<(i32, i32)>::new();
    let mut start_pos = (0, 0);
    let mut x = 0;
    let mut y = 0;

    for ch in output.data().iter().map(|c| *c as u8 as char) {
        match ch {
            '\n' => {
                x = 0;
                y += 1;
            }
            '#' => {
                map.insert((x, y));
                x += 1;
            }
            '.' => {
                x += 1;
            }
            '^' | 'v' | 'V' | '<' | '>' => {
                start_pos = (x, y);
                x += 1;
            }
            c => {
                panic!("Bad Char: {}", c);
            }
        }
    }

    let intersections = map
        .iter()
        .copied()
        .filter(|pos| neighbors(*pos, &map).count() == 4)
        .collect::<Vec<_>>();

    stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    for pos in map {
        stdout()
            .execute(cursor::MoveTo(pos.0 as u16, pos.1 as u16))?
            .execute(PrintStyledContent(style('#').with(Color::Blue)))?;
    }

    for pos in &intersections {
        stdout()
            .execute(cursor::MoveTo(pos.0 as u16, pos.1 as u16))?
            .execute(PrintStyledContent(style('O').with(Color::Red)))?;
    }

    stdout().execute(cursor::MoveTo(0, 0))?;

    println!("{}", intersections.iter().map(|(x, y)| x*y).sum::<i32>());

    Ok(())
}
