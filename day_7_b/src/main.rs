use std::error::Error;
use std::fs;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;

enum ParameterMode {
    Position = 0,
    Immediate = 1,
}

impl From<i32> for ParameterMode {
    fn from(value: i32) -> Self {
        match value % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("Unknown parameter mode"),
        }
    }
}

enum Opcode {
    Add(ParameterMode, ParameterMode),
    Mult(ParameterMode, ParameterMode),
    Input,
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThen(ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode),
    Halt,
}

impl From<i32> for Opcode {
    fn from(value: i32) -> Self {
        match value % 100 {
            1 => Opcode::Add((value / 100).into(), (value / 1000).into()),
            2 => Opcode::Mult((value / 100).into(), (value / 1000).into()),
            3 => Opcode::Input,
            4 => Opcode::Output((value / 100).into()),
            5 => Opcode::JumpIfTrue((value / 100).into(), (value / 1000).into()),
            6 => Opcode::JumpIfFalse((value / 100).into(), (value / 1000).into()),
            7 => Opcode::LessThen((value / 100).into(), (value / 1000).into()),
            8 => Opcode::Equals((value / 100).into(), (value / 1000).into()),
            99 => Opcode::Halt,
            _ => panic!("Unknown opcode"),
        }
    }
}

fn get_parameter(memory: &[i32], value: i32, mode: ParameterMode) -> i32 {
    match mode {
        ParameterMode::Position => memory[value as usize],
        ParameterMode::Immediate => value,
    }
}

fn run_program(
    memory: &mut [i32],
    input: mpsc::Receiver<i32>,
    output: mpsc::Sender<i32>,
    output_side_channel: Option<Arc<AtomicI32>>,
) -> Result<(), Box<dyn Error>> {
    let mut pc = 0;

    loop {
        let opcode = Opcode::from(memory[pc]);

        match opcode {
            Opcode::Add(a_mode, b_mode) => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let res = memory[pc + 3] as usize;

                memory[res] = get_parameter(memory, a, a_mode) + get_parameter(memory, b, b_mode);

                pc += 4
            }
            Opcode::Mult(a_mode, b_mode) => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let res = memory[pc + 3] as usize;

                memory[res] = get_parameter(memory, a, a_mode) * get_parameter(memory, b, b_mode);

                pc += 4
            }
            Opcode::Input => {
                let address = memory[pc + 1] as usize;

                memory[address] = input.recv()?;

                pc += 2
            }
            Opcode::Output(mode) => {
                let value = memory[pc + 1];

                let output_value = get_parameter(memory, value, mode);

                output.send(output_value).ok();

                if let Some(side_channel) = &output_side_channel {
                    side_channel.store(output_value, Ordering::SeqCst);
                }

                pc += 2
            }
            Opcode::JumpIfTrue(a_mode, b_mode) => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];

                if get_parameter(memory, a, a_mode) != 0 {
                    pc = get_parameter(memory, b, b_mode) as usize;
                } else {
                    pc += 3;
                }
            }
            Opcode::JumpIfFalse(a_mode, b_mode) => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];

                if get_parameter(memory, a, a_mode) == 0 {
                    pc = get_parameter(memory, b, b_mode) as usize;
                } else {
                    pc += 3;
                }
            }
            Opcode::LessThen(a_mode, b_mode) => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let res = memory[pc + 3] as usize;

                if get_parameter(memory, a, a_mode) < get_parameter(memory, b, b_mode) {
                    memory[res] = 1;
                } else {
                    memory[res] = 0;
                }

                pc += 4
            }
            Opcode::Equals(a_mode, b_mode) => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let res = memory[pc + 3] as usize;

                if get_parameter(memory, a, a_mode) == get_parameter(memory, b, b_mode) {
                    memory[res] = 1;
                } else {
                    memory[res] = 0;
                }

                pc += 4
            }
            Opcode::Halt => {
                return Ok(());
            }
        }
    }
}

fn run_amplifiers(program: &[i32], &phase_settings: &[i32; 5]) -> Result<i32, Box<dyn Error>> {
    let (a_out, b_in) = mpsc::channel();
    let (b_out, c_in) = mpsc::channel();
    let (c_out, d_in) = mpsc::channel();
    let (d_out, e_in) = mpsc::channel();
    let (e_out, a_in) = mpsc::channel();

    let mut a_prg = program.to_owned();
    let mut b_prg = program.to_owned();
    let mut c_prg = program.to_owned();
    let mut d_prg = program.to_owned();
    let mut e_prg = program.to_owned();

    e_out.send(phase_settings[0])?;
    a_out.send(phase_settings[1])?;
    b_out.send(phase_settings[2])?;
    c_out.send(phase_settings[3])?;
    d_out.send(phase_settings[4])?;

    e_out.send(0)?;

    let output_side_channel = Arc::new(AtomicI32::new(0));
    let output_side_channel_thread = output_side_channel.clone();

    let a = thread::spawn(move || {
        run_program(&mut a_prg, a_in, a_out, None).unwrap();
    });

    let b = thread::spawn(move || {
        run_program(&mut b_prg, b_in, b_out, None).unwrap();
    });

    let c = thread::spawn(move || {
        run_program(&mut c_prg, c_in, c_out, None).unwrap();
    });

    let d = thread::spawn(move || {
        run_program(&mut d_prg, d_in, d_out, None).unwrap();
    });

    let e = thread::spawn(move || {
        run_program(&mut e_prg, e_in, e_out, Some(output_side_channel_thread)).unwrap();
    });

    e.join().ok();
    d.join().ok();
    c.join().ok();
    b.join().ok();
    a.join().ok();

    Ok(output_side_channel.load(Ordering::SeqCst))
}

fn find_max_output(program: &[i32]) -> Result<i32, Box<dyn Error>> {
    let mut max_output = std::i32::MIN;

    for a in 5..=9 {
        for b in (5..=9).filter(|b| ![a].contains(b)) {
            for c in (5..=9).filter(|c| ![a, b].contains(c)) {
                for d in (5..=9).filter(|d| ![a, b, c].contains(d)) {
                    for e in (5..=9).filter(|e| ![a, b, c, d].contains(e)) {
                        max_output = max_output.max(run_amplifiers(program, &[a, b, c, d, e])?);
                    }
                }
            }
        }
    }

    Ok(max_output)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let output = find_max_output(&program)?;

    println!("{:?}", output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::find_max_output;

    #[test]
    fn test_find_max_output_b() {
        {
            let program = [
                3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28,
                -1, 28, 1005, 28, 6, 99, 0, 0, 5,
            ];
            assert_eq!(find_max_output(&program).unwrap(), 139629729);
        }

        {
            let program = [
                3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001,
                54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53,
                55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
            ];
            assert_eq!(find_max_output(&program).unwrap(), 18216);
        }
    }
}
