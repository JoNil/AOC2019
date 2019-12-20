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
    Portal((i32, i32), [u8; 2], i32),
}

impl Tile {
    fn is_ground(&self) -> bool {
        match self {
            Tile::Wall => false,
            Tile::Ground => true,
            Tile::Portal(..) => false,
        }
    }

    fn is_wall(&self) -> Option<Self> {
        match self {
            Tile::Wall => Some(Tile::Wall),
            Tile::Ground => None,
            Tile::Portal(..) => None,
        }
    }

    fn get_portal(&self) -> Option<((i32, i32), [u8; 2], i32)> {
        match self {
            Tile::Wall => None,
            Tile::Ground => None,
            Tile::Portal(pos, name, z_dir) => Some((*pos, *name, *z_dir)),
        }
    }

    fn get_destination(&self, pos: (i32, i32, i32)) -> Option<(i32, i32, i32)> {
        match self {
            Tile::Wall => None,
            Tile::Ground => Some(pos),
            Tile::Portal(dest, name, z_dir) => {
                if name == b"AA" || name == b"ZZ" {
                    None
                } else {
                    if *z_dir == -1 && pos.2 == 0 {
                        None
                    } else {
                        Some((dest.0, dest.1, pos.2 + z_dir))
                    }
                }
            }
        }
    }

    fn get_destination_if_name_matches(&self, name: &[u8]) -> Option<(i32, i32, i32)> {
        match self {
            Tile::Wall => None,
            Tile::Ground => None,
            Tile::Portal(dest, portal_name, _) => {
                if portal_name == name {
                    Some((dest.0, dest.1, 0))
                } else {
                    None
                }
            }
        }
    }

    fn to_styled_char(&self) -> StyledContent<char> {
        match self {
            Tile::Wall => style('#').with(Color::DarkGrey),
            Tile::Ground => style('.').with(Color::Grey),
            Tile::Portal(_, name, _) => style(name[0] as char).with(Color::Green),
        }
    }
}

fn reconstruct_path(
    came_from: &HashMap<(i32, i32, i32), (i32, i32, i32)>,
    mut current: (i32, i32, i32),
) -> Vec<(i32, i32, i32)> {
    let mut total_path = Vec::new();
    total_path.push(current);
    while came_from.contains_key(&current) {
        current = came_from[&current];
        total_path.push(current);
    }
    total_path.reverse();
    total_path
}

fn neighbors(pos: (i32, i32, i32), map: &HashMap<(i32, i32), Tile>) -> Vec<(i32, i32, i32)> {
    [
        (pos.0 + 1, pos.1),
        (pos.0 - 1, pos.1),
        (pos.0, pos.1 + 1),
        (pos.0, pos.1 - 1),
    ]
    .iter()
    .map(|candidate| (candidate, map.get(candidate).unwrap_or(&Tile::Wall)))
    .filter_map(|(candidate, tile)| tile.get_destination((candidate.0, candidate.1, pos.2)))
    .collect::<Vec<_>>()
}

fn a_star(
    start: (i32, i32, i32),
    goal: (i32, i32, i32),
    map: &HashMap<(i32, i32), Tile>,
) -> Option<Vec<(i32, i32, i32)>> {
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

        for neighbor in &neighbors(current, map) {
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

fn abs_dist(a: (i32, i32), b: (i32, i32)) -> i32 {
    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}

fn parse_map(input: &str) -> Result<HashMap<(i32, i32), Tile>, Box<dyn Error>> {
    let mut map = HashMap::<(i32, i32), Tile>::new();
    let mut portal_part = HashMap::<(i32, i32), u8>::new();
    let mut x = 0;
    let mut y = 0;

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
            ' ' => {
                x += 1;
            }
            ch => {
                portal_part.insert((x, y), ch as u8);
                x += 1;
            }
        }
    }

    let max_wall = map
        .iter()
        .filter_map(|(pos, tile)| tile.is_wall().map(|_| pos))
        .copied()
        .max_by(|a, b| (a.0 + a.1).cmp(&(b.0 + b.1)))
        .ok_or("Error")?;
    let min_wall = map
        .iter()
        .filter_map(|(pos, tile)| tile.is_wall().map(|_| pos))
        .copied()
        .min_by(|a, b| (a.0 + a.1).cmp(&(b.0 + b.1)))
        .ok_or("Error")?;

    let is_on_outer_edge = |pos: (i32, i32)| pos.0 == max_wall.0
        || pos.0 == min_wall.0
        || pos.1 == max_wall.1
        || pos.1 == min_wall.1;

    for (pos, part) in portal_part.iter() {
        for (other_pos, other_part) in [
            (pos.0 + 1, pos.1),
            (pos.0 - 1, pos.1),
            (pos.0, pos.1 + 1),
            (pos.0, pos.1 - 1),
        ]
        .iter()
        .filter_map(|maybe_pos| portal_part.get_key_value(maybe_pos))
        {
            let dir = ((pos.0 - other_pos.0).abs(), (pos.1 - other_pos.1).abs());

            let candidates = [
                ((pos.0 + dir.0), (pos.1 + dir.1)),
                ((pos.0 - dir.0), (pos.1 - dir.1)),
                ((other_pos.0 + dir.0), (other_pos.1 + dir.1)),
                ((other_pos.0 - dir.0), (other_pos.1 - dir.1)),
            ]
            .iter()
            .filter_map(|maybe_dest| map.get_key_value(&maybe_dest))
            .map(|(k, v)| (*k, *v))
            .collect::<Vec<_>>();

            for (dest_pos, _) in candidates.iter().filter(|(_, tile)| tile.is_ground()) {
                let (portal_pos, portal_part, far_part) = match (abs_dist(*pos, *dest_pos), abs_dist(*other_pos, *dest_pos)) {
                    (1, 2) => (pos, part, other_part),
                    (2, 1) => (other_pos, other_part, part),
                    _ => return Err("Bad map")?,
                };

                let name = match portal_pos.0 - dest_pos.0 + portal_pos.1 - dest_pos.1 {
                    1 => [*portal_part, *far_part],
                    -1 => [*far_part, *portal_part],
                    _ => return Err("Bad map")?,
                };

                let z_diff = if is_on_outer_edge(*dest_pos) {
                    -1
                } else {
                    1
                };

                map.insert(*portal_pos, Tile::Portal(*dest_pos, name, z_diff));
            }
        }
    }

    let mut portals = map
        .iter()
        .filter_map(|(pos, tile)| tile.get_portal().map(|portal| (*pos, portal)))
        .collect::<Vec<_>>();

    for (a, (a_pos, (a_dest, a_name, _))) in portals.clone().into_iter().enumerate() {
        for (b, (b_pos, (b_dest, b_name, _))) in portals.clone().into_iter().enumerate() {
            if a_pos != b_pos && a_name == b_name && a > b {
                (portals[a].1).0 = b_dest;
                (portals[b].1).0 = a_dest;
            }
        }
    }

    for (pos, (dest, name, z_diff)) in portals {
        map.insert(pos, Tile::Portal(dest, name, z_diff));
    }

    Ok(map)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let map = parse_map(&input)?;

    let start_pos = map
        .values()
        .find_map(|tile| tile.get_destination_if_name_matches(b"AA"))
        .ok_or("No Start")?;
    let end_pos = map
        .values()
        .find_map(|tile| tile.get_destination_if_name_matches(b"ZZ"))
        .ok_or("No End")?;

    let path = a_star(start_pos, end_pos, &map).ok_or("No Path")?;

    stdout().execute(terminal::Clear(terminal::ClearType::All))?;

    for (pos, tile) in &map {
        stdout()
            .execute(cursor::MoveTo(2*pos.0 as u16, pos.1 as u16))?
            .execute(PrintStyledContent(tile.to_styled_char()))?;
    }

    for pos in &path {
        stdout()
            .execute(cursor::MoveTo(2*pos.0 as u16, pos.1 as u16))?
            .execute(PrintStyledContent(style(pos.2.to_string()).with(Color::Blue)))?;
    }

    {
        let mut drawn = HashSet::new();
        for (portal_pos, tile) in &map {
            if let Tile::Portal(pos, name, z_diff) = tile {
                if *z_diff == 1 {
                    stdout()
                        .execute(cursor::MoveTo((2*portal_pos.0 - 1) as u16, portal_pos.1 as u16))?
                        .execute(PrintStyledContent(
                            style(name[0] as char).with(Color::Green),
                        ))?
                        .execute(cursor::MoveTo(2*portal_pos.0 as u16, portal_pos.1 as u16))?
                        .execute(PrintStyledContent(
                            style(name[1] as char).with(Color::Green),
                        ))?
                        .execute(cursor::MoveTo((2*pos.0 - 1) as u16, pos.1 as u16))?
                        .execute(PrintStyledContent(
                            style(name[0].to_ascii_lowercase() as char).with(Color::Green),
                        ))?
                        .execute(cursor::MoveTo(2*pos.0 as u16, pos.1 as u16))?
                        .execute(PrintStyledContent(
                            style(name[1].to_ascii_lowercase() as char).with(Color::Green),
                        ))?;
                } else if *z_diff == -1 {
                    stdout()
                        .execute(cursor::MoveTo((2*portal_pos.0 - 1) as u16, portal_pos.1 as u16))?
                        .execute(PrintStyledContent(
                            style(name[0] as char).with(Color::Red),
                        ))?
                        .execute(cursor::MoveTo(2*portal_pos.0 as u16, portal_pos.1 as u16))?
                        .execute(PrintStyledContent(
                            style(name[1] as char).with(Color::Red),
                        ))?
                        .execute(cursor::MoveTo((2*pos.0 - 1) as u16, pos.1 as u16))?
                        .execute(PrintStyledContent(
                            style(name[0].to_ascii_lowercase() as char).with(Color::Red),
                        ))?
                        .execute(cursor::MoveTo(2*pos.0 as u16, pos.1 as u16))?
                        .execute(PrintStyledContent(
                            style(name[1].to_ascii_lowercase() as char).with(Color::Red),
                        ))?;

                    drawn.insert(*name);
                }
            }
        }
    }

    {
        let max_pos = map.keys().max_by(|a, b| a.1.cmp(&b.1)).ok_or("Error")?;
        stdout()
            .execute(cursor::MoveTo(0, max_pos.1 as u16))?
            .execute(Print('\n'))?;
    }

    println!("Steps: {}", path.len() - 1);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{a_star, parse_map};

    #[test]
    fn test_20_b() {
        {
            let input = "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";

            let map = parse_map(&input).unwrap();

            let start_pos = map
                .values()
                .find_map(|tile| tile.get_destination_if_name_matches(b"AA"))
                .unwrap();
            let end_pos = map
                .values()
                .find_map(|tile| tile.get_destination_if_name_matches(b"ZZ"))
                .unwrap();

            let path = a_star(start_pos, end_pos, &map).unwrap();

            assert_eq!(path.len() - 1, 26)
        }

        {
            let input = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";

            let map = parse_map(&input).unwrap();

            let start_pos = map
                .values()
                .find_map(|tile| tile.get_destination_if_name_matches(b"AA"))
                .unwrap();
            let end_pos = map
                .values()
                .find_map(|tile| tile.get_destination_if_name_matches(b"ZZ"))
                .unwrap();

            let path = a_star(start_pos, end_pos, &map).unwrap();

            assert_eq!(path.len() - 1, 396)
        }
    }
}
