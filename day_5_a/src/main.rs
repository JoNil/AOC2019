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
    Halt,
}

impl From<i32> for Opcode {
    fn from(value: i32) -> Self {
        match value % 100 {
            1 => Opcode::Add((value / 100).into(), (value / 1000).into()),
            2 => Opcode::Mult((value / 100).into(), (value / 1000).into()),
            3 => Opcode::Input,
            4 => Opcode::Output((value / 100).into()),
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
            Opcode::Halt => {
                return Ok(output);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut program = input
        .split(",")
        .map(|v| v.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let input = [1];

    let output = run_program(&mut program, &input)?;

    println!("{:?}", output);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run_program;

    #[test]
    fn test_run() {
        {
            let mut program = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
            let result = [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

            run_program(&mut program, &[]).unwrap();
            assert_eq!(&program, &result);
        }

        {
            let mut program = [1, 0, 0, 0, 99];
            let result = [2, 0, 0, 0, 99];

            run_program(&mut program, &[]).unwrap();
            assert_eq!(&program, &result);
        }

        {
            let mut program = [2, 3, 0, 3, 99];
            let result = [2, 3, 0, 6, 99];

            run_program(&mut program, &[]).unwrap();
            assert_eq!(&program, &result);
        }

        {
            let mut program = [2, 4, 4, 5, 99, 0];
            let result = [2, 4, 4, 5, 99, 9801];

            run_program(&mut program, &[]).unwrap();
            assert_eq!(&program, &result);
        }

        {
            let mut program = [1, 1, 1, 4, 99, 5, 6, 0, 99];
            let result = [30, 1, 1, 4, 2, 5, 6, 0, 99];

            run_program(&mut program, &[]).unwrap();
            assert_eq!(&program, &result);
        }

        {
            let mut program = [3, 0, 4, 0, 99];
            let result = [100, 0, 4, 0, 99];
            let input = [100];

            let output = run_program(&mut program, &input).unwrap();

            assert_eq!(&program, &result);
            assert_eq!(&output, &[100]);
        }

        {
            let mut program = [1101, 100, -1, 4, 0];
            let result = [1101, 100, -1, 4, 99];

            run_program(&mut program, &[]).unwrap();

            assert_eq!(&program, &result);
        }
    }
}
