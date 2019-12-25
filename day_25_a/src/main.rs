use int_comp::{IntcodeComputer, IntcodeOutput};
use rustyline::Editor;
use std::error::Error;
use std::fs;
use std::iter;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut incode = IntcodeComputer::new(&program);

    let mut rl = Editor::<()>::new();
    let mut input = Vec::new();
    let mut run = true;

    while run {
        let mut got_output = true;

        while got_output {
            match incode.run(&input.drain(..).collect::<Vec<_>>(), Some(1))? {
                IntcodeOutput::Halt(_) => {
                    got_output = false;
                    run = false;
                }
                IntcodeOutput::Interrupt(output) => {
                    for ch in output.iter().map(|c| *c as u8 as char) {
                        print!("{}", ch);
                    }
                }
                IntcodeOutput::NeedMoreInput => {
                    got_output = false;
                }
            }
        }

        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                input.extend(line.chars().chain(iter::once('\n')).map(|ch| ch as i64));
            }
            Err(err) => {
                println!("Error: {:?}", err);
                run = false;
            }
        }
    }

    Ok(())
}
