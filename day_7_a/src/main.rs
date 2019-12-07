use std::error::Error;
use std::fs;

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

fn run_program(memory: &mut [i32], input: &[i32]) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut pc = 0;
    let mut input_counter = 0;
    let mut output = Vec::new();

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

                memory[address] = input[input_counter];

                input_counter += 1;
                pc += 2
            }
            Opcode::Output(mode) => {
                let value = memory[pc + 1];

                output.push(get_parameter(memory, value, mode));

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
                return Ok(output);
            }
        }
    }
}

fn run_amplifiers(program: &[i32], &phase_settings: &[i32; 5]) -> Result<i32, Box<dyn Error>> {
    let mut last_output = 0;

    for i in 0..5 {
        let input = [phase_settings[i], last_output];
        let output = run_program(&mut program.to_owned(), &input)?;
        last_output = *output.get(0).ok_or("Bad program".to_string())?;
    }

    Ok(last_output)
}

fn find_max_output(program: &[i32]) -> Result<i32, Box<dyn Error>> {
    let mut max_output = std::i32::MIN;

    for a in 0..5 {
        for b in (0..5).filter(|b| ![a].contains(b)) {
            for c in (0..5).filter(|c| ![a, b].contains(c)) {
                for d in (0..5).filter(|d| ![a, b, c].contains(d)) {
                    for e in (0..5).filter(|e| ![a, b, c, d].contains(e)) {
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
mod tests;

#[cfg(test)]
mod day7_tests {
    use super::find_max_output;

    #[test]
    fn test_find_max_output() {
        {
            let program = [
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
            ];
            assert_eq!(find_max_output(&program).unwrap(), 43210);
        }

        {
            let program = [
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0,
            ];
            assert_eq!(find_max_output(&program).unwrap(), 54321);
        }

        {
            let program = [
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
            ];
            assert_eq!(find_max_output(&program).unwrap(), 65210);
        }
    }
}
