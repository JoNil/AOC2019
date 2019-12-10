use std::error::Error;
use std::fs;

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl From<i64> for ParameterMode {
    fn from(value: i64) -> Self {
        match value % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unknown parameter mode"),
        }
    }
}

#[derive(Debug)]
enum Opcode {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Mult(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThen(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    RelativeBaseOffset(ParameterMode),
    Halt,
}

impl From<i64> for Opcode {
    fn from(value: i64) -> Self {
        match value % 100 {
            1 => Opcode::Add(
                (value / 100).into(),
                (value / 1000).into(),
                (value / 10000).into(),
            ),
            2 => Opcode::Mult(
                (value / 100).into(),
                (value / 1000).into(),
                (value / 10000).into(),
            ),
            3 => Opcode::Input((value / 100).into()),
            4 => Opcode::Output((value / 100).into()),
            5 => Opcode::JumpIfTrue((value / 100).into(), (value / 1000).into()),
            6 => Opcode::JumpIfFalse((value / 100).into(), (value / 1000).into()),
            7 => Opcode::LessThen(
                (value / 100).into(),
                (value / 1000).into(),
                (value / 10000).into(),
            ),
            8 => Opcode::Equals(
                (value / 100).into(),
                (value / 1000).into(),
                (value / 10000).into(),
            ),
            9 => Opcode::RelativeBaseOffset((value / 100).into()),
            99 => Opcode::Halt,
            _ => panic!("Unknown opcode"),
        }
    }
}

fn get_parameter(memory: &[i64], value: i64, mode: ParameterMode, relative_base: i64) -> i64 {
    match mode {
        ParameterMode::Position => memory[value as usize],
        ParameterMode::Immediate => value,
        ParameterMode::Relative => memory[(relative_base + value) as usize],
    }
}

fn get_address(value: i64, mode: ParameterMode, relative_base: i64) -> usize {
    match mode {
        ParameterMode::Position => value as usize,
        ParameterMode::Immediate => panic!(),
        ParameterMode::Relative => (relative_base + value) as usize,
    }
}

fn run_program(program: &[i64], input: &[i64]) -> Result<Vec<i64>, Box<dyn Error>> {
    let mut memory = {
        let mut mem = Vec::new();
        mem.resize_with(1 * 1024 * 1024, Default::default);
        mem[..program.len()].copy_from_slice(program);
        mem.into_boxed_slice()
    };

    let mut pc = 0;
    let mut relative_base = 0;
    let mut input_counter = 0;
    let mut output = Vec::new();

    loop {
        let opcode = Opcode::from(memory[pc]);

        match opcode {
            Opcode::Add(a_mode, b_mode, res_mode) => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let res = memory[pc + 3];

                memory[get_address(res, res_mode, relative_base)] =
                    get_parameter(&memory, a, a_mode, relative_base)
                        + get_parameter(&memory, b, b_mode, relative_base);

                pc += 4
            }
            Opcode::Mult(a_mode, b_mode, res_mode) => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let res = memory[pc + 3];

                memory[get_address(res, res_mode, relative_base)] =
                    get_parameter(&memory, a, a_mode, relative_base)
                        * get_parameter(&memory, b, b_mode, relative_base);

                pc += 4
            }
            Opcode::Input(mode) => {
                let value = memory[pc + 1];

                memory[get_address(value, mode, relative_base)] = input[input_counter];

                input_counter += 1;
                pc += 2
            }
            Opcode::Output(mode) => {
                let value = memory[pc + 1];

                output.push(get_parameter(&memory, value, mode, relative_base));

                pc += 2
            }
            Opcode::JumpIfTrue(a_mode, b_mode) => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];

                if get_parameter(&memory, a, a_mode, relative_base) != 0 {
                    pc = get_parameter(&memory, b, b_mode, relative_base) as usize;
                } else {
                    pc += 3;
                }
            }
            Opcode::JumpIfFalse(a_mode, b_mode) => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];

                if get_parameter(&memory, a, a_mode, relative_base) == 0 {
                    pc = get_parameter(&memory, b, b_mode, relative_base) as usize;
                } else {
                    pc += 3;
                }
            }
            Opcode::LessThen(a_mode, b_mode, res_mode) => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let res = memory[pc + 3];

                if get_parameter(&memory, a, a_mode, relative_base)
                    < get_parameter(&memory, b, b_mode, relative_base)
                {
                    memory[get_address(res, res_mode, relative_base)] = 1;
                } else {
                    memory[get_address(res, res_mode, relative_base)] = 0;
                }

                pc += 4
            }
            Opcode::Equals(a_mode, b_mode, res_mode) => {
                let a = memory[pc + 1];
                let b = memory[pc + 2];
                let res = memory[pc + 3];

                if get_parameter(&memory, a, a_mode, relative_base)
                    == get_parameter(&memory, b, b_mode, relative_base)
                {
                    memory[get_address(res, res_mode, relative_base)] = 1;
                } else {
                    memory[get_address(res, res_mode, relative_base)] = 0;
                }

                pc += 4
            }
            Opcode::RelativeBaseOffset(mode) => {
                let value = memory[pc + 1];

                relative_base += get_parameter(&memory, value, mode, relative_base);

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

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let output = run_program(&program, &[2])?;

    println!("{:?}", output);

    Ok(())
}

#[cfg(test)]
mod tests;
