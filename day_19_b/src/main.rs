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

fn does_square_fit(pos: (i32, i32), map: &HashMap<(i32, i32), i32>, size: i32) -> bool {

    for x in pos.0..(pos.0 + size) {
        for y in pos.1..(pos.1 + size) {
            if *map.get(&(x, y)).unwrap_or(&0) != 1 {
                return false;
            }
        }
    }

    true
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    for x in 0..50 {
        for y in 0..50 {
            let output = IntcodeComputer::new(&program).run(&[x as i64, y as i64], None)?;

            if output.data().len() > 0 {
                map.insert((x, y), output.data()[0] as i32);
            }
        }
    }

    let mut dist = std::i32::MAX;
    let mut closest_pos = (std::i32::MAX, std::i32::MAX);

    for pos in map.keys() {
        if does_square_fit(*pos, &map, 3) {
            if pos.0 + pos.1 < dist {
                dist = pos.0 + pos.1;
                closest_pos = *pos;
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

    for x in closest_pos.0..(closest_pos.0 + 3) {
        for y in closest_pos.1..(closest_pos.1 + 3) {
            stdout()
                .execute(cursor::MoveTo(x as u16, y as u16))?
                .execute(PrintStyledContent(style('#').with(Color::Blue)))?;
        }
    }

    {
        let max_pos = map.keys().max_by(|a, b| a.1.cmp(&b.1)).ok_or("Error")?;
        stdout()
            .execute(cursor::MoveTo(max_pos.0 as u16, max_pos.1 as u16))?
            .execute(Print('\n'))?;
    }

    println!("{:?}", closest_pos);

    Ok(())
}
