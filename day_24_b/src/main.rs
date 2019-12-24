use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq)]
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

#[derive(Clone, Eq, PartialEq)]
struct Map {
    data: HashMap<i32, [Tile; MAP_SIZE]>,
}

impl Map {
    fn get_adjacent(&self, x: i32, y: i32, z: i32, from: (i32, i32)) -> Vec<Tile> {
        if x < 0 {
            return vec![self.get(1, 2, z - 1)];
        }

        if x >= MAP_WIDTH as i32 {
            return vec![self.get(3, 2, z - 1)];
        }

        if y < 0 {
            return vec![self.get(2, 1, z - 1)];
        }

        if y >= MAP_HEIGHT as i32 {
            return vec![self.get(2, 3, z - 1)];
        }

        if x == 2 && y == 2 {
            if from.0 == 1 && from.1 == 2 {
                let mut res = Vec::new();
                for y in 0..(MAP_HEIGHT as i32) {
                    res.push(self.get(0, y, z + 1));
                }
                return res;
            }

            if from.0 == 2 && from.1 == 1 {
                let mut res = Vec::new();
                for x in 0..(MAP_WIDTH as i32) {
                    res.push(self.get(x, 0, z + 1));
                }
                return res;
            }

            if from.0 == 3 && from.1 == 2 {
                let mut res = Vec::new();
                for y in 0..(MAP_HEIGHT as i32) {
                    res.push(self.get(MAP_WIDTH as i32 - 1, y, z + 1));
                }
                return res;
            }

            if from.0 == 2 && from.1 == 3 {
                let mut res = Vec::new();
                for x in 0..(MAP_WIDTH as i32) {
                    res.push(self.get(x, MAP_HEIGHT as i32 - 1, z + 1));
                }
                return res;
            }
        }

        if !self.data.contains_key(&z) {
            return vec![Tile::Ground];
        }

        vec![self.data.get(&z).unwrap()[x as usize + MAP_WIDTH * (y as usize)]]
    }

    fn get(&self, x: i32, y: i32, z: i32) -> Tile {
        if x < 0 || y < 0 || x >= MAP_WIDTH as i32 || y >= MAP_HEIGHT as i32 {
            panic!("Outside");
        }

        if !self.data.contains_key(&z) {
            return Tile::Ground;
        }

        self.data.get(&z).unwrap()[x as usize + MAP_WIDTH * (y as usize)]
    }

    fn set(&mut self, x: i32, y: i32, z: i32, tile: Tile) {
        match self.data.get_mut(&z) {
            Some(map) => {
                map[(x as usize + MAP_WIDTH * (y as usize))] = tile;
            }
            None => {}
        };
    }

    fn count_adjacent_bugs(&self, x: i32, y: i32, z: i32) -> i32 {
        [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .iter()
            .map(|pos| {
                self.get_adjacent(pos.0, pos.1, z, (x, y))
                    .iter()
                    .filter(|tile| **tile == Tile::Bug)
                    .count()
            })
            .sum::<usize>() as i32
    }

    fn next(&mut self) {
        {
            let min_level = *self.data.keys().min().unwrap();
            let max_level = *self.data.keys().max().unwrap();

            if self
                .data
                .get(&min_level)
                .unwrap()
                .iter()
                .filter(|tile| **tile == Tile::Bug)
                .count()
                > 0
            {
                self.data.insert(min_level - 1, [Tile::Ground; MAP_SIZE]);
            }

            if self
                .data
                .get(&max_level)
                .unwrap()
                .iter()
                .filter(|tile| **tile == Tile::Bug)
                .count()
                > 0
            {
                self.data.insert(max_level + 1, [Tile::Ground; MAP_SIZE]);
            }
        }

        let mut adjacent_bugs = HashMap::new();

        for z in self.data.keys().copied() {
            for y in 0..(MAP_HEIGHT as i32) {
                for x in 0..(MAP_WIDTH as i32) {
                    if x == 2 && y == 2 {
                        continue;
                    }

                    adjacent_bugs.insert((x, y, z), self.count_adjacent_bugs(x, y, z));
                }
            }
        }

        for z in self
            .data
            .keys()
            .copied()
            .collect::<Vec<_>>()
            .iter()
            .copied()
        {
            for y in 0..(MAP_HEIGHT as i32) {
                for x in 0..(MAP_WIDTH as i32) {
                    if x == 2 && y == 2 {
                        continue;
                    }

                    let adjacent_bugs = *adjacent_bugs.get(&(x, y, z)).unwrap_or(&0);

                    self.set(
                        x,
                        y,
                        z,
                        match self.get(x, y, z) {
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
        }
    }
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let min_level = *self.data.keys().min().unwrap();
        let max_level = *self.data.keys().max().unwrap();

        for z in min_level..=max_level {

            writeln!(f, "Level: {}", z)?;

            for y in 0..MAP_HEIGHT {
                for x in 0..MAP_WIDTH {
                    write!(f, "{:?}", self.get(x as i32, y as i32, z as i32))?;
                }

                if y != MAP_HEIGHT - 1 {
                    write!(f, "\n")?;
                }
            }

            write!(f, "\n\n")?;
        }

        Ok(())
    }
}

fn parse_map(input: &str) -> Result<Map, Box<dyn Error>> {
    let mut map = Map {
        data: HashMap::new(),
    };

    map.data.insert(0, [Tile::Ground; MAP_SIZE]);

    let mut x = 0;
    let mut y = 0;

    for ch in input.chars() {
        match ch {
            '\n' => {
                x = 0;
                y += 1;
            }
            '#' => {
                map.set(x, y, 0, Tile::Bug);
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

    for _ in 0..200 {
        map.next();
    }

    println!(
        "Bugs: {}",
        map.data
            .values()
            .map(|map| map.iter().filter(|tile| **tile == Tile::Bug).count())
            .sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_map, Tile};

    #[test]
    fn test_map_next_2() {
        let input = "....#
            #..#.
            #.?##
            ..#..
            #....";

        let mut map = parse_map(&input).unwrap();

        for _ in 0..10 {
            map.next();
        }

        assert_eq!(
            map.data
                .values()
                .map(|map| map.iter().filter(|tile| **tile == Tile::Bug).count())
                .sum::<usize>(),
            99
        );
    }
}
