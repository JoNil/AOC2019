use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    style::{style, Color, Print, PrintStyledContent},
    terminal, ExecutableCommand,
};
use int_comp::{IntcodeComputer, IntcodeOutput};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::stdout;

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
enum Tile {
    Wall,
    Ground,
    GroundOxygen,
}

impl Tile {
    fn from_i32(input: i32) -> Self {
        match input {
            0 => Tile::Wall,
            1 => Tile::Ground,
            2 => Tile::GroundOxygen,
            _ => panic!(),
        }
    }
}

fn reconstruct_path(
    came_from: &HashMap<(i32, i32), (i32, i32)>,
    mut current: (i32, i32),
) -> Vec<(i32, i32)> {
    let mut total_path = Vec::new();
    total_path.push(current);
    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.push(current);
    }
    total_path.reverse();
    total_path
}

fn huristic(pos: (i32, i32), goal: (i32, i32)) -> i32 {
    (goal.0 - pos.0).abs() + (goal.1 - pos.1).abs()
}

fn neighbors(pos: (i32, i32), map: &HashMap<(i32, i32), Tile>) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    for candidate in &[
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ] {
        let tile = *map.get(candidate).unwrap_or(&Tile::Wall);
        if tile == Tile::Ground || tile == Tile::GroundOxygen {
            res.push(*candidate);
        }
    }

    res
}

fn a_star(
    start: (i32, i32),
    goal: (i32, i32),
    map: &HashMap<(i32, i32), Tile>,
) -> Option<Vec<(i32, i32)>> {
    let mut open_set = HashSet::new();
    open_set.insert(start);

    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start, 0);

    let mut f_score = HashMap::new();
    f_score.insert(start, huristic(start, goal));

    while !open_set.is_empty() {
        let current = *open_set
            .iter()
            .min_by(|x, y| {
                f_score
                    .get(x)
                    .unwrap_or(&std::i32::MAX)
                    .cmp(f_score.get(y).unwrap_or(&std::i32::MAX))
            })
            .unwrap();

        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }

        open_set.remove(&current);

        for neighbor in &neighbors(current, map) {
            let tentative_g_score = g_score.get(&current).unwrap_or(&std::i32::MAX) + 1;

            if tentative_g_score < *g_score.get(neighbor).unwrap_or(&std::i32::MAX) {
                came_from.insert(*neighbor, current);
                g_score.insert(*neighbor, tentative_g_score);
                f_score.insert(*neighbor, tentative_g_score + huristic(*neighbor, goal));

                open_set.insert(*neighbor);
            }
        }
    }

    None
}

fn update(
    pos: &mut (i32, i32),
    map: &mut HashMap<(i32, i32), Tile>,
    last_path_to_home: &mut Vec<(i32, i32)>,
    input: Input,
    output: Tile,
) -> Option<Vec<((i32, i32), char, Color)>> {
    let mut res = Vec::new();

    let old_pos = *pos;

    let new_pos = match input {
        Input::North => (pos.0, pos.1 + 1),
        Input::South => (pos.0, pos.1 - 1),
        Input::East => (pos.0 + 1, pos.1),
        Input::West => (pos.0 - 1, pos.1),
    };

    map.insert(new_pos, output);

    if output == Tile::Wall {
        res.push((new_pos, '#', Color::Grey));
    } else {
        res.push((old_pos, '.', Color::Grey));
        res.push((new_pos, 'D', Color::Grey));
        *pos = new_pos;
    }

    if let Some(path_to_home) = a_star((0, 0), new_pos, map) {
        for pos in last_path_to_home.iter() {
            res.push((*pos, '.', Color::Grey));
        }

        for pos in path_to_home.iter() {
            res.push((*pos, '.', Color::Blue));
        }

        *last_path_to_home = path_to_home;
    }

    Some(res)
}

const DRAW_OFFSET: (i32, i32) = (50, 20);

fn calculate_fill_time(map: &HashMap<(i32, i32), Tile>) -> Option<i32> {
    let mut map = map.clone();

    let mut open_list = Vec::new();

    for (pos, tile) in map.iter() {
        if *tile == Tile::GroundOxygen {
            open_list.push(*pos);
            break;
        }
    }

    let mut time = 0;

    while !open_list.is_empty() {
        let mut new_open_list = Vec::new();

        for current in open_list {
            map.insert(current, Tile::Wall);

            for neighbor in &neighbors(current, &map) {
                new_open_list.push(*neighbor);
            }
        }

        open_list = new_open_list;

        time += 1;
    }

    Some(time - 1)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut int_comp = IntcodeComputer::new(&program);
    let mut map = HashMap::new();
    let mut pos = (0, 0);
    let mut last_path_to_home = Vec::new();

    map.insert(pos, Tile::Ground);

    stdout()
        .execute(terminal::Clear(terminal::ClearType::All))?
        .execute(cursor::MoveTo(
            (pos.0 + DRAW_OFFSET.0) as u16,
            (pos.1 + DRAW_OFFSET.1) as u16,
        ))?
        .execute(Print('D'))?;

    loop {
        let input = match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Up, ..
            }) => Input::South,
            Event::Key(KeyEvent {
                code: KeyCode::Down,
                ..
            }) => Input::North,
            Event::Key(KeyEvent {
                code: KeyCode::Left,
                ..
            }) => Input::West,
            Event::Key(KeyEvent {
                code: KeyCode::Right,
                ..
            }) => Input::East,
            Event::Key(KeyEvent {
                code: KeyCode::PageDown,
                ..
            }) => {
                println!("Fill Time: {:?}", calculate_fill_time(&map));
                continue;
            }
            _ => continue,
        };

        let output = match int_comp.run(&[input.as_i32() as i64], Some(1))? {
            IntcodeOutput::Interrupt(output) => Tile::from_i32(output[0] as i32),
            IntcodeOutput::Halt(_) => {
                return Err("Halt".into());
            }
        };

        if let Some(draw_instructions) =
            update(&mut pos, &mut map, &mut last_path_to_home, input, output)
        {
            for (pos, ch, color) in draw_instructions {
                stdout()
                    .execute(cursor::MoveTo(
                        (pos.0 + DRAW_OFFSET.0) as u16,
                        (pos.1 + DRAW_OFFSET.1) as u16,
                    ))?
                    .execute(PrintStyledContent(style(ch).with(color)))?;
            }
        } else {
            println!("Done: {}", last_path_to_home.len());
            break;
        }
    }

    Ok(())
}
