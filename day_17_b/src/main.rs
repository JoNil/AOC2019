use int_comp::IntcodeComputer;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

#[derive(Copy, Clone, Debug)]
enum Dir {
    Right,
    Left,
    Up,
    Down,
}

impl Dir {
    fn get_next(&self, pos: (i32, i32)) -> (i32, i32) {
        match *self {
            Dir::Right => (pos.0 + 1, pos.1),
            Dir::Left => (pos.0 - 1, pos.1),
            Dir::Up => (pos.0, pos.1 - 1),
            Dir::Down => (pos.0, pos.1 + 1),
        }
    }

    fn get_line(&self, start_pos: (i32, i32), len: i32) -> Line {
        match *self {
            Dir::Right => Line::Horiz(start_pos, len),
            Dir::Left => Line::Horiz(start_pos, -len),
            Dir::Up => Line::Verti(start_pos, -len),
            Dir::Down => Line::Verti(start_pos, len),
        }
    }

    fn get_possible_out_dirs(&self) -> [Dir; 3] {
        match *self {
            Dir::Right => [Dir::Right, Dir::Up, Dir::Down],
            Dir::Left => [Dir::Left, Dir::Up, Dir::Down],
            Dir::Up => [Dir::Left, Dir::Right, Dir::Up],
            Dir::Down => [Dir::Left, Dir::Right, Dir::Down],
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Line {
    Horiz((i32, i32), i32),
    Verti((i32, i32), i32),
}

impl Line {
    fn calculate(mut start_pos: (i32, i32), map: &HashMap<(i32, i32), char>) -> Vec<Line> {
        let mut lines = Vec::new();

        let mut current_pos = start_pos;
        let mut current_len = 0;
        let mut current_dir = Dir::Left;

        loop {
            let next_pos = current_dir.get_next(current_pos);
            let next_char = *map.get(&next_pos).unwrap_or(&'.');

            if next_char == '#' {
                current_pos = next_pos;
                current_len += 1;
                continue;
            } else {
                lines.push(current_dir.get_line(start_pos, current_len));

                start_pos = current_pos;
                current_len = 0;

                let mut found_new_dir = false;

                for maybe_dir in &current_dir.get_possible_out_dirs() {
                    let maybe_pos = maybe_dir.get_next(current_pos);
                    let maybe_char = *map.get(&maybe_pos).unwrap_or(&'.');

                    if maybe_char == '#' {
                        current_dir = *maybe_dir;
                        found_new_dir = true;
                        break;
                    }
                }

                if !found_new_dir {
                    break;
                }
            }
        }

        lines
    }
}

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Left(i32),
    Right(i32),
}

impl Instruction {
    fn calculate(segments: &[Line]) -> Vec<Instruction> {
        let mut res = Vec::new();

        let mut current_dir = Dir::Up;

        for segment in segments {
            match (segment, current_dir) {
                (Line::Horiz(_, len), Dir::Up) => {
                    if *len > 0 {
                        res.push(Instruction::Right(len.abs()));
                        current_dir = Dir::Right;
                    } else {
                        res.push(Instruction::Left(len.abs()));
                        current_dir = Dir::Left;
                    }
                }
                (Line::Horiz(_, len), Dir::Down) => {
                    if *len > 0 {
                        res.push(Instruction::Left(len.abs()));
                        current_dir = Dir::Right;
                    } else {
                        res.push(Instruction::Right(len.abs()));
                        current_dir = Dir::Left;
                    }
                }
                (Line::Verti(_, len), Dir::Left) => {
                    if *len > 0 {
                        res.push(Instruction::Left(len.abs()));
                        current_dir = Dir::Down;
                    } else {
                        res.push(Instruction::Right(len.abs()));
                        current_dir = Dir::Up;
                    }
                }
                (Line::Verti(_, len), Dir::Right) => {
                    if *len > 0 {
                        res.push(Instruction::Right(len.abs()));
                        current_dir = Dir::Down;
                    } else {
                        res.push(Instruction::Left(len.abs()));
                        current_dir = Dir::Up;
                    }
                }
                _ => panic!("Bad dir"),
            }
        }

        res
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut map = HashMap::<(i32, i32), char>::new();
    let mut start_pos = (0, 0);

    {
        let mut int_comp = IntcodeComputer::new(&program);
        let output = int_comp.run(&[], None)?;

        {
            let mut x = 0;
            let mut y = 0;

            for ch in output.data().iter().map(|c| *c as u8 as char) {
                match ch {
                    '\n' => {
                        x = 0;
                        y += 1;
                    }
                    ch => {
                        if let '^' | 'v' | 'V' | '<' | '>' = ch {
                            start_pos = (x, y);
                        }

                        map.insert((x, y), ch);
                        x += 1;
                    }
                }
            }
        }
    }

    let segments = Line::calculate(start_pos, &map);

    let instr = Instruction::calculate(&segments);

    program[0] = 2;

    let input = "A,B,B,C,A,B,C,A,B,C
L,6,R,12,L,4,L,6
R,6,L,6,R,12
L,6,L,10,L,10,R,6
y
";

    let mut int_comp = IntcodeComputer::new(&program);

    let output = int_comp.run(&input.chars().map(|c| c as i64).collect::<Vec<_>>(), None)?;

    for ch in output.data().iter().map(|c| *c as u8 as char) {
        print!("{}", ch);
    }

    println!("\n{}", output.data().last().ok_or("No output")?);

    println!("{:?}", instr);

    Ok(())
}
