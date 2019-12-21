use crossterm::{
    cursor,
    style::{style, Color, Print, PrintStyledContent, StyledContent},
    terminal, ExecutableCommand,
};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::io::stdout;
use std::rc::Rc;

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

    let mut score = HashMap::new();
    score.insert(start, 0);

    while !open_set.is_empty() {
        let current = *open_set
            .iter()
            .min_by(|x, y| {
                score
                    .get(x)
                    .unwrap_or(&std::i32::MAX)
                    .cmp(score.get(y).unwrap_or(&std::i32::MAX))
            })
            .unwrap();

        if current == goal {
            return Some(reconstruct_path(&came_from, current));
        }

        open_set.remove(&current);

        for neighbor in &neighbors(current, map, keys) {
            let new_score = score.get(&current).unwrap_or(&std::i32::MAX) + 1;

            if new_score < *score.get(neighbor).unwrap_or(&std::i32::MAX) {
                came_from.insert(*neighbor, current);
                score.insert(*neighbor, new_score);

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
    [(i32, i32); 4],
    Vec<(char, (i32, i32))>,
    Vec<(char, (i32, i32))>,
) {
    let mut map = HashMap::<(i32, i32), Tile>::new();
    let mut keys = Vec::new();
    let mut doors = Vec::new();
    let mut x = 0;
    let mut y = 0;
    let mut start_pos = [(0, 0); 4];
    let mut start_pos_index = 0;

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
                start_pos[start_pos_index] = (x, y);
                start_pos_index += 1;
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

#[derive(Clone, Hash, Eq, PartialEq)]
struct CacheKey {
    pos: (i32, i32),
    aquired_keys: Vec<char>,
}

#[derive(Copy, Clone)]
struct Path {
    key: char,
    end_pos: (i32, i32),
    len: i32,
}

fn calculate_paths_to_reachable_keys<'a>(
    map: &HashMap<(i32, i32), Tile>,
    cache: &mut HashMap<CacheKey, Rc<Vec<Path>>>,
    start_pos: (i32, i32),
    remaning_keys: &[(char, (i32, i32))],
    aquired_keys: &[char],
) -> Rc<Vec<Path>> {
    let mut sorted_aquired_keys = aquired_keys.to_owned();
    sorted_aquired_keys.sort();

    let cache_key = CacheKey {
        pos: start_pos,
        aquired_keys: sorted_aquired_keys,
    };

    if let Some(res) = cache.get(&cache_key) {
        return res.clone();
    }

    let mut res = Vec::new();

    for (key, key_pos) in remaning_keys {
        if let Some(path) = a_star(start_pos, *key_pos, map, aquired_keys) {
            let res_path = Path {
                key: *key,
                end_pos: *path.last().unwrap(),
                len: path.len() as i32 - 1,
            };

            res.push(res_path);
        }
    }

    res.sort_by(|a, b| a.len.cmp(&b.len));

    cache.insert(cache_key.clone(), Rc::new(res));

    cache.get(&cache_key).unwrap().clone()
}

fn calculate_shortest_path(
    map: &HashMap<(i32, i32), Tile>,
    cache: &mut HashMap<CacheKey, Rc<Vec<Path>>>,
    pos: [(i32, i32); 4],
    remaning_keys: Vec<(char, (i32, i32))>,
    aquired_keys: Vec<char>,
) -> i32 {
    let mut possible_paths = Vec::new();

    for i in 0..4 {
        let paths =
            calculate_paths_to_reachable_keys(map, cache, pos[i], &remaning_keys, &aquired_keys);
        possible_paths.reserve(paths.len());

        for path in paths.iter() {
            possible_paths.push((i, *path));
        }
    }

    if possible_paths.len() == 0 {
        return 0;
    }

    let mut shortest_path = std::i32::MAX;

    for (i, path) in possible_paths.iter().take(if aquired_keys.len() < 10 {
        if aquired_keys.len() < 5 {
            5
        } else {
            3
        }
    } else {
        if possible_paths.len() == remaning_keys.len() {
            1
        } else {
            2
        }
    }) {
        let new_remaning_keys = remaning_keys
            .iter()
            .copied()
            .filter(|(key, _)| *key != path.key)
            .collect::<Vec<_>>();
        let new_aquired_keys = aquired_keys
            .iter()
            .chain(&[path.key])
            .copied()
            .collect::<Vec<_>>();

        if remaning_keys.len() > 16 {
            println!("{:?}", new_aquired_keys);
        }

        let mut new_pos = pos;
        new_pos[*i] = path.end_pos;

        let new_steps =
            calculate_shortest_path(map, cache, new_pos, new_remaning_keys, new_aquired_keys)
                + path.len;

        if new_steps < shortest_path {
            shortest_path = new_steps;
        }
    }

    shortest_path
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let (map, pos, keys, _) = parse_map(&input);
    let mut cache = HashMap::new();

    let shortest_path = calculate_shortest_path(&map, &mut cache, pos, keys, Vec::new());

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
    use std::collections::HashMap;

    #[test]
    fn test_calculate_shortest_path_b() {
        {
            let input = "#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#Ab#
#######";

            let (map, pos, keys, _) = parse_map(&input);
            let mut cache = HashMap::new();

            let shortest_path = calculate_shortest_path(&map, &mut cache, pos, keys, Vec::new());

            assert_eq!(shortest_path, 8)
        }
    }
}
