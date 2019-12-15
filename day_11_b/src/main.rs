use int_comp::{IntcodeComputer, IntcodeOutput};
use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn get_new_dir(dir: (i32, i32), change: i32) -> Result<(i32, i32), Box<dyn Error>> {
    Ok(match (dir, change) {
        ((0, 1), 0) => (-1, 0),
        ((-1, 0), 0) => (0, -1),
        ((0, -1), 0) => (1, 0),
        ((1, 0), 0) => (0, 1),

        ((0, 1), 1) => (1, 0),
        ((-1, 0), 1) => (0, 1),
        ((0, -1), 1) => (-1, 0),
        ((1, 0), 1) => (0, -1),

        _ => return Err("Bad direction".into()),
    })
}

fn paint(int_comp: &mut IntcodeComputer) -> Result<HashMap<(i32, i32), i32>, Box<dyn Error>> {
    let mut pos = (0, 0);
    let mut dir = (0, 1);
    let mut painted_squares: HashMap<(i32, i32), i32> = HashMap::new();

    painted_squares.insert((0, 0), 1);

    loop {
        let input = match painted_squares.get(&pos) {
            Some(1) => [1],
            _ => [0],
        };

        let (color_to_paint, dir_change) = match int_comp.run(&input, Some(2))? {
            IntcodeOutput::Halt(_) => {
                return Ok(painted_squares);
            }
            IntcodeOutput::Interrupt(output) => (output[0], output[1]),
        };

        painted_squares.insert(pos, color_to_paint as i32);

        dir = get_new_dir(dir, dir_change as i32)?;

        pos.0 += dir.0;
        pos.1 += dir.1;
    }
}

fn output_pain(painted_squares: HashMap<(i32, i32), i32>) {
    let mut max_x = std::i32::MIN;
    let mut min_x = std::i32::MAX;

    let mut max_y = std::i32::MIN;
    let mut min_y = std::i32::MAX;

    for (x, y) in painted_squares.keys() {
        max_x = max_x.max(*x);
        min_x = min_x.min(*x);

        max_y = max_y.max(*y);
        min_y = min_y.min(*y);
    }

    let width = (max_x - min_x) + 1;
    let height = (max_y - min_y) + 1;

    let mut bitmap = Vec::new();
    bitmap.resize_with((width * height) as usize, Default::default);

    for ((x, y), value) in painted_squares {
        let y = y.abs();

        bitmap[(x + width * y) as usize] = value;
    }

    for chunk in bitmap.chunks(width as usize) {
        for pixel in chunk {
            if *pixel == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }

        println!("");
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut int_comp = IntcodeComputer::new(&program);

    let output = paint(&mut int_comp)?;

    output_pain(output);

    Ok(())
}
