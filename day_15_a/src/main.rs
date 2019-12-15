use crossterm::{
    cursor,
    style::{style, Color, PrintStyledContent, Print},
    event::{read, Event, KeyCode, KeyEvent},
    terminal, ExecutableCommand,
};
use int_comp::{IntcodeComputer, IntcodeOutput};
use std::error::Error;
use std::io::stdout;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Input {
    North,
    South,
    West,
    East,
}

impl Input {

    fn as_i32(self) -> i32 {
        match self {
            Input::North => 1,
            Input::South => 2,
            Input::West => 3,
            Input::East => 4,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Output {
    HitWall,
    Moved,
    MovedDone,
}

impl Output {
    fn from_i32(input: i32) -> Self {
        match input {
            0 => Output::HitWall,
            1 => Output::Moved,
            2 => Output::MovedDone,
            _ => panic!(),
        }
    }
}

fn update(pos: &mut (i32, i32), input: Input, output: Output) -> Vec<((i32, i32), char)> {

    let mut res = Vec::new();

    let old_pos = *pos;

    let new_pos = match input {
        Input::North => (pos.0, pos.1 + 1),
        Input::South => (pos.0, pos.1 - 1),
        Input::East => (pos.0 + 1, pos.1),
        Input::West => (pos.0 - 1, pos.1),
    };

    if output == Output::MovedDone {
        panic!("Done: {:?}", new_pos);
    }

    if output == Output::HitWall {
        res.push((new_pos, '#'));
    } else {
        res.push((old_pos, '.'));
        res.push((new_pos, 'D'));
        *pos = new_pos;
    }

    res
}

const DRAW_OFFSET: (i32, i32) = (50, 50);

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut int_comp = IntcodeComputer::new(&program);

    let mut pos = (0, 0);

    stdout()
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::MoveTo((pos.0 + DRAW_OFFSET.0) as u16, (pos.1 + DRAW_OFFSET.1) as u16))?
        .execute(Print('D'))?;

    loop {

        let input = match read()? {
            Event::Key(KeyEvent { code: KeyCode::Up, .. }) => Input::South,
            Event::Key(KeyEvent { code: KeyCode::Down, .. }) => Input::North,
            Event::Key(KeyEvent { code: KeyCode::Left, .. }) => Input::West,
            Event::Key(KeyEvent { code: KeyCode::Right, .. }) => Input::East,
            _ => continue,
        };

        let output = match int_comp.run(&[input.as_i32() as i64], Some(1))? {
            IntcodeOutput::Interrupt(output) => {
                Output::from_i32(output[0] as i32)
            }
            IntcodeOutput::Halt(_) => {
                panic!();
            }
        };

        let draw_instructions = update(&mut pos, input, output);

        for (pos, ch) in draw_instructions {

            stdout()
                .execute(cursor::MoveTo((pos.0 + DRAW_OFFSET.0) as u16, (pos.1 + DRAW_OFFSET.1) as u16))?
                .execute(Print(ch))?;
        }
    }

    Ok(())
}