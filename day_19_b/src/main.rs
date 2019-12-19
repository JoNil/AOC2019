use crossterm::{
    cursor,
    style::{style, Color, PrintStyledContent},
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

fn count_line(pos: (i32, i32), map: &HashMap<(i32, i32), i32>) -> i32 {

    let mut count = 0;

    for i in 1..  {
        if *map.get(&(pos.0 - i, pos.1)).unwrap_or(&0) != 1 {
            return count;
        }

        count += 1;
    }

    count
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    let mut pos = (8, 9);
    let mut going_down = true;
    let mut found_blocks = true;
    let mut consecutive_blocks = 0;

    loop {
        let output = IntcodeComputer::new(&program).run(&[pos.0 as i64, pos.1 as i64], None)?;

        let out = output.data()[0];

        map.insert(pos, out as i32);

        if out == 0 && found_blocks {

            if !going_down {
                if dbg!(count_line(pos, &map)) > 205 && dbg!(consecutive_blocks) > 205 {
                    break;
                }
            }

            pos.0 += 1;
            going_down = !going_down;
            consecutive_blocks = 0;
            found_blocks = false;
        } else {
        
            if out == 1 {
                found_blocks = true;
                consecutive_blocks += 1;
            } 
            
            if going_down {
                pos.1 += 1;
            } else {
                pos.1 -= 1;
            }
        }
    }

    let mut dist = std::i32::MAX;
    let mut closest_pos = (std::i32::MAX, std::i32::MAX);

    for pos in map.keys() {
        if does_square_fit(*pos, &map, 100) {
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

    println!("{:?}: {:?}", closest_pos, closest_pos.0 * 10000 + closest_pos.1);

    Ok(())
}
