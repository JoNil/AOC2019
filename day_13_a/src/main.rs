use int_comp::{IntcodeComputer, IntcodeOutput};
use std::convert::TryFrom;
use std::error::Error;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TryFrom<i32> for TileType {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        Ok(match value {
            0 => TileType::Empty,
            1 => TileType::Wall,
            2 => TileType::Block,
            3 => TileType::Paddle,
            4 => TileType::Ball,
            _ => return Err("Unknown Tile"),
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Tile {
    x: i32,
    y: i32,
    ty: TileType,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut int_comp = IntcodeComputer::new(&program);

    let mut tiles = Vec::new();

    loop {
        match int_comp.run(&[], Some(3))? {
            IntcodeOutput::Interrupt(output) => {
                tiles.push(Tile {
                    x: output[0] as i32,
                    y: output[1] as i32,
                    ty: TileType::try_from(output[2] as i32)?,
                });
            }
            IntcodeOutput::Halt(_) => {
                break;
            }
        }
    }

    let res = tiles.iter().filter(|t| t.ty == TileType::Block).count();

    println!("{}", res);

    Ok(())
}
