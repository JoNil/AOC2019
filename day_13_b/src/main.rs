use int_comp::{IntcodeComputer, IntcodeOutput};
use pancurses::{initscr, endwin, noecho};
use std::convert::TryFrom;
use std::error::Error;
use std::fs;
use std::thread;
use std::time::Duration;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum TileType {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl TileType {
    fn to_char(&self) -> char {
        match *self {
            TileType::Empty => ' ',
            TileType::Wall => '#',
            TileType::Block => '=',
            TileType::Paddle => '_',
            TileType::Ball => 'o',
        }
    }
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

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    program[0] = 2;

    let mut int_comp = IntcodeComputer::new(&program);
    let mut score = 0;
    let mut last_ball_x = 0;
    let mut last_paddle_x = 0;

    let window = initscr();
    window.nodelay(true);
    noecho();

    loop {
        window.getch();

        let input = if last_ball_x < last_paddle_x {
            -1
        } else if last_ball_x > last_paddle_x {
            1
        } else {
            0
        };

        match int_comp.run(&[input], Some(3))? {
            IntcodeOutput::Interrupt(output) => {
                if output[0] == -1 && output[1] == 0 {
                    score = output[2];
                } else {
                    let x = output[0] as i32;
                    let y = output[1] as i32;
                    let ty = TileType::try_from(output[2] as i32)?;

                    if ty == TileType::Ball {
                        last_ball_x = x;
                    }

                    if ty == TileType::Paddle {
                        last_paddle_x = x;
                    }

                    window.mv(y, x);
                    window.addch(ty.to_char());
                }
            }
            IntcodeOutput::Halt(_) => {
                break;
            }
        }

        thread::sleep(Duration::from_millis(10));
    }

    endwin();

    println!("{}", score);

    Ok(())
}
