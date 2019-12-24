use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Ground,
    Bug,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Ground => write!(f, "."),
            Tile::Bug => write!(f, "#"),
        }
    }
}

const MAP_WIDTH: usize = 5;
const MAP_HEIGHT: usize = 5;
const MAP_SIZE: usize = MAP_WIDTH * MAP_HEIGHT;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Map {
    data: [Tile; MAP_SIZE],
}

impl Map {
    fn new() -> Map {
        Map {
            data: [Tile::Ground; MAP_SIZE],
        }
    }

    fn get(&self, x: i32, y: i32) -> Tile {
        if x < 0 || y < 0 || x >= MAP_WIDTH as i32 || y >= MAP_HEIGHT as i32 {
            return Tile::Ground;
        }

        self.data[x as usize + MAP_WIDTH * (y as usize)]
    }

    fn set(&mut self, x: i32, y: i32, tile: Tile) {
        self.data[(x as usize + MAP_WIDTH * (y as usize))] = tile;
    }

    fn count_adjacent_bugs(&self, x: i32, y: i32) -> i32 {
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .iter()
            .map(|pos| self.get(pos.0, pos.1))
            .filter(|tile| *tile == Tile::Bug)
            .count() as i32
    }

    fn biodiversity_rating(&self) -> usize {
        let mut res = 0;

        for i in 0..MAP_SIZE {
            if self.data[i] == Tile::Bug {
                res += 2usize.pow(i as u32);
            }
        }

        res
    }

    fn next(&self) -> Map {
        let mut new = Map::new();

        for y in 0..(MAP_HEIGHT as i32) {
            for x in 0..(MAP_WIDTH as i32) {
                let adjacent_bugs = self.count_adjacent_bugs(x, y);

                new.set(
                    x,
                    y,
                    match self.get(x, y) {
                        Tile::Ground => {
                            if adjacent_bugs == 1 || adjacent_bugs == 2 {
                                Tile::Bug
                            } else {
                                Tile::Ground
                            }
                        }
                        Tile::Bug => {
                            if adjacent_bugs == 1 {
                                Tile::Bug
                            } else {
                                Tile::Ground
                            }
                        }
                    },
                );
            }
        }

        new
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                write!(f, "{:?}", self.get(x as i32, y as i32))?;
            }

            if y != MAP_HEIGHT - 1 {
                write!(f, "\n")?;
            }
        }

        Ok(())
    }
}

fn parse_map(input: &str) -> Result<Map, Box<dyn Error>> {
    let mut map = Map {
        data: [Tile::Ground; MAP_SIZE],
    };

    let mut x = 0;
    let mut y = 0;

    for ch in input.chars() {
        match ch {
            '\n' => {
                x = 0;
                y += 1;
            }
            '#' => {
                map.set(x, y, Tile::Bug);
                x += 1;
            }
            ' ' => {}
            _ => {
                x += 1;
            }
        }
    }

    Ok(map)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut map = parse_map(&input)?;

    let mut previous_maps = HashSet::new();
    previous_maps.insert(map);

    loop {
        map = map.next();

        if previous_maps.contains(&map) {
            break;
        }

        previous_maps.insert(map);
    }

    println!("Biodiversity Rating {:?}", map.biodiversity_rating());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_map;

    #[test]
    fn test_map_next() {
        let input = "....#
            #..#.
            #..##
            ..#..
            #....";

        let mut map = parse_map(&input).unwrap();

        map = map.next();
        assert_eq!(
            format!("{:?}", map),
            "#..#.
####.
###.#
##.##
.##.."
        );

        map = map.next();
        assert_eq!(
            format!("{:?}", map),
            "#####
....#
....#
...#.
#.###"
        );

        map = map.next();
        assert_eq!(
            format!("{:?}", map),
            "#....
####.
...##
#.##.
.##.#"
        );

        map = map.next();
        assert_eq!(
            format!("{:?}", map),
            "####.
....#
##..#
.....
##..."
        );
    }
}
