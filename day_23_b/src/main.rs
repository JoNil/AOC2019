use int_comp::{IntcodeComputer, IntcodeOutput};
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut computers = Vec::new();
    let mut queues = Vec::new();
    let mut nat = (0, 0);
    let mut last_nat = (0, 0);

    for i in 0..50 {
        computers.push(IntcodeComputer::new(&program));
        queues.push(vec![i]);
    }

    let mut done = false;
    let mut idle = false;
    let mut last_idle;

    loop {
        last_idle = idle;
        idle = true;

        for (i, incode) in computers.iter_mut().enumerate() {
            let mut input = queues[i].drain(..).collect::<Vec<_>>();

            if input.len() == 0 {
                if i == 0 && last_idle {
                    if nat == last_nat {
                        println!("First nat repeate {:?}", nat);
                        done = true;
                    }

                    last_nat = nat;

                    input.push(nat.0);
                    input.push(nat.1);
                } else {
                    input.push(-1);
                }
            } else {
                idle = false;
            }

            let output = incode.run(&input, Some(3))?;

            match output {
                IntcodeOutput::Interrupt(out) => {
                    let address = out[0];
                    let x = out[1];
                    let y = out[2];

                    if address == 255 {
                        nat = (x, y);
                    } else {
                        queues[address as usize].push(x);
                        queues[address as usize].push(y);
                    }
                }
                _ => continue,
            }
        }

        if done {
            break;
        }
    }

    Ok(())
}
