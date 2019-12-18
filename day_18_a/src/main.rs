use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    style::{style, Color, Print, PrintStyledContent, StyledContent},
    terminal, ExecutableCommand,
};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::stdout;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Wall,
    Ground,
    Key(char),
    Door(char),
}

impl Tile {
    fn is_passable(&self, state: &State) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Ground => true,
            Tile::Key(_) => true,
            Tile::Door(door) => state.keys.contains(door),
        }
    }

    fn to_styled_char(&self) -> StyledContent<char> {
        match self {
            Tile::Wall => style('#').with(Color::Grey),
            Tile::Ground => style('.').with(Color::DarkGrey),
            Tile::Key(ch) => style(*ch).with(Color::Green),
            Tile::Door(ch) => style(*ch).with(Color::Red),
        }
    }

    fn get_key(&self) -> Option<char> {
        if let Tile::Key(ch) = self {
            Some(*ch)
        } else {
            None
        }
    }
}

struct State {
    map: HashMap<(i32, i32), Tile>,
    keys: Vec<char>,
}

impl State {
    fn new(map: HashMap<(i32, i32), Tile>) -> State {
        State {
            map: map,
            keys: Vec::new(),
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

fn neighbors(pos: (i32, i32), state: &State) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    for candidate in &[
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ] {
        let tile = *state.map.get(candidate).unwrap_or(&Tile::Wall);
        if tile.is_passable(&state) {
            res.push(*candidate);
        }
    }

    res
}

fn a_star(start: (i32, i32), goal: (i32, i32), state: &State) -> Option<Vec<(i32, i32)>> {
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

        for neighbor in &neighbors(current, state) {
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

fn parse_map(input: &str) -> (HashMap<(i32, i32), Tile>, (i32, i32)) {
    let mut map = HashMap::<(i32, i32), Tile>::new();
    let mut x = 0;
    let mut y = 0;
    let mut start_pos = (0, 0);

    for ch in input.chars() {
        match ch {
            '\n' => {
                x = 0;
                y += 1;
            }
            '#' => {
                map.insert((x, y), Tile::Wall);
                x += 1;
            }
            '.' => {
                map.insert((x, y), Tile::Ground);
                x += 1;
            }
            '@' => {
                map.insert((x, y), Tile::Ground);
                start_pos = (x, y);
                x += 1;
            }
            ch => {
                if ch.is_ascii_uppercase() {
                    map.insert((x, y), Tile::Door(ch));
                }

                if ch.is_ascii_lowercase() {
                    map.insert((x, y), Tile::Key(ch.to_ascii_uppercase()));
                }

                x += 1;
            }
        }
    }

    (map, start_pos)
}

fn calculate_paths_to_reachable_keys(state: &State, start_pos: (i32, i32)) -> Vec<(char, Vec<(i32, i32)>)> {

    let mut res = Vec::new();

    for (key_pos, key) in state.map.iter().filter_map(|(pos, tile)| tile.get_key().map(|key| (pos, key))) {

        if let Some(path) = a_star(start_pos, *key_pos, state) {
            res.push((key, path));
        }
    }

    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let (map, start_pos) = parse_map(&input);

    let state = State::new(map.clone());

    let keys = map.values().filter_map(|tile| tile.get_key()).collect::<Vec<_>>();

    let path_to_keys = calculate_paths_to_reachable_keys(&state, start_pos);

    stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    for (pos, tile) in &map {
        stdout()
            .execute(cursor::MoveTo(pos.0 as u16, pos.1 as u16))?
            .execute(PrintStyledContent(tile.to_styled_char()))?;
    }

    if let Some((key, path)) = path_to_keys.get(0) {
        for pos in path {
            stdout()
                .execute(cursor::MoveTo(pos.0 as u16, pos.1 as u16))?
                .execute(PrintStyledContent(style(key).with(Color::Blue)))?;
        }
    }

    {
        let max_pos = map.keys().max_by(|a, b| a.1.cmp(&b.1)).ok_or("Error")?;
        stdout()
            .execute(cursor::MoveTo(max_pos.0 as u16, max_pos.1 as u16))?
            .execute(Print('\n'))?;
    }

    println!("keys: {:?}", keys);

    Ok(())
}
