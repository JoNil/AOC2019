use crossterm::{
    cursor,
    style::{style, Color, Print, PrintStyledContent, StyledContent},
    terminal, ExecutableCommand,
};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::stdout;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Tile {
    Wall,
    Ground,
    Key(char),
    Door(char),
}

impl Tile {
    fn is_passable(&self, keys: &[char]) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Ground => true,
            Tile::Key(_) => true,
            Tile::Door(door) => keys.contains(door),
        }
    }

    fn to_styled_char(&self) -> StyledContent<char> {
        match self {
            Tile::Wall => style('#').with(Color::Grey),
            Tile::Ground => style('.').with(Color::DarkGrey),
            Tile::Key(ch) => style(*ch).with(Color::Blue),
            Tile::Door(ch) => style(*ch).with(Color::Red),
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

fn neighbors(pos: (i32, i32), map: &HashMap<(i32, i32), Tile>, keys: &[char]) -> Vec<(i32, i32)> {
    let mut res = Vec::new();
    for candidate in &[
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ] {
        let tile = *map.get(candidate).unwrap_or(&Tile::Wall);
        if tile.is_passable(keys) {
            res.push(*candidate);
        }
    }

    res
}

fn a_star(
    start: (i32, i32),
    goal: (i32, i32),
    map: &HashMap<(i32, i32), Tile>,
    keys: &[char],
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

        for neighbor in &neighbors(current, map, keys) {
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

fn parse_map(
    input: &str,
) -> (
    HashMap<(i32, i32), Tile>,
    (i32, i32),
    Vec<(char, (i32, i32))>,
    Vec<(char, (i32, i32))>,
) {
    let mut map = HashMap::<(i32, i32), Tile>::new();
    let mut keys = Vec::new();
    let mut doors = Vec::new();
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
            ' ' => {}
            ch => {
                if ch.is_ascii_uppercase() {
                    map.insert((x, y), Tile::Door(ch));
                    doors.push((ch, (x, y)));
                }

                if ch.is_ascii_lowercase() {
                    map.insert((x, y), Tile::Key(ch.to_ascii_uppercase()));
                    keys.push((ch.to_ascii_uppercase(), (x, y)));
                }

                x += 1;
            }
        }
    }

    (map, start_pos, keys, doors)
}

fn calculate_paths_to_reachable_keys(
    map: &HashMap<(i32, i32), Tile>,
    start_pos: (i32, i32),
    remaning_keys: &[(char, (i32, i32))],
    aquired_keys: &[char],
) -> Vec<(char, Vec<(i32, i32)>)> {
    let mut res = Vec::new();

    for (key, key_pos) in remaning_keys {
        if let Some(path) = a_star(start_pos, *key_pos, map, aquired_keys) {
            res.push((*key, path));
        }
    }

    res
}

fn calculate_shortest_path(
    map: &HashMap<(i32, i32), Tile>,
    pos: (i32, i32),
    remaning_keys: Vec<(char, (i32, i32))>,
    remaning_doors: Vec<(char, (i32, i32))>,
    aquired_keys: Vec<char>,
) -> i32 {
    let mut possible_paths =
        calculate_paths_to_reachable_keys(map, pos, &remaning_keys, &aquired_keys);

    let visible_doors =
        calculate_paths_to_reachable_keys(map, pos, &remaning_doors, &aquired_keys);

    if possible_paths.len() == 0 {
        return 0;
    }

    possible_paths.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

    let mut shortest_path = std::i32::MAX;

    for (new_key, path) in possible_paths.iter().filter(|(key, _)| {
        if visible_doors.len() == 0 {
            return true;
        }

        for (door, _) in &visible_doors {
            if door == key {
                return true;
            }
        }

        false
    }).take(if aquired_keys.len() > 4 { 1 } else { if aquired_keys.len() > 2 { 2 } else { 4 }}) {
        let new_remaning_keys = remaning_keys
            .iter()
            .copied()
            .filter(|(key, _)| *key != *new_key)
            .collect::<Vec<_>>();
        let new_remaning_doors = remaning_doors
            .iter()
            .copied()
            .filter(|(door, _)| *door != *new_key)
            .collect::<Vec<_>>();
        let new_aquired_keys = aquired_keys
            .iter()
            .chain(&[*new_key])
            .copied()
            .collect::<Vec<_>>();

        println!("{:?}", new_aquired_keys);

        let new_pos = *path.last().unwrap();

        let new_steps = calculate_shortest_path(map, new_pos, new_remaning_keys, new_remaning_doors, new_aquired_keys)
            + (path.len() as i32 - 1);

        if new_steps < shortest_path {
            shortest_path = new_steps;
        }
    }

    shortest_path
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let (map, pos, keys, doors) = parse_map(&input);

    let shortest_path = calculate_shortest_path(&map, pos, keys, doors, Vec::new());

    stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    for (pos, tile) in &map {
        stdout()
            .execute(cursor::MoveTo(pos.0 as u16, pos.1 as u16))?
            .execute(PrintStyledContent(tile.to_styled_char()))?;
    }

    {
        let max_pos = map.keys().max_by(|a, b| a.1.cmp(&b.1)).ok_or("Error")?;
        stdout()
            .execute(cursor::MoveTo(max_pos.0 as u16, max_pos.1 as u16))?
            .execute(Print('\n'))?;
    }

    println!("Steps: {}", shortest_path);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{calculate_shortest_path, parse_map};

    #[test]
    fn test_find_best_monitoring_position() {
        {
            let input = "#########
#b.A.@.a#
#########";

            let (map, pos, keys, doors) = parse_map(&input);

            let shortest_path = calculate_shortest_path(&map, pos, keys, doors, Vec::new());

            assert_eq!(shortest_path, 8)
        }

        {
            let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

            let (map, pos, keys, doors) = parse_map(&input);

            let shortest_path = calculate_shortest_path(&map, pos, keys, doors, Vec::new());

            assert_eq!(shortest_path, 86)
        }

        {
            let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

            let (map, pos, keys, doors) = parse_map(&input);

            let shortest_path = calculate_shortest_path(&map, pos, keys, doors, Vec::new());

            assert_eq!(shortest_path, 132)
        }

        {
            let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

            let (map, pos, keys, doors) = parse_map(&input);

            let shortest_path = calculate_shortest_path(&map, pos, keys, doors, Vec::new());

            assert_eq!(shortest_path, 136)
        }

        {
            let input = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

            let (map, pos, keys, doors) = parse_map(&input);

            let shortest_path = calculate_shortest_path(&map, pos, keys, doors, Vec::new());

            assert_eq!(shortest_path, 81)
        }
    }
}
