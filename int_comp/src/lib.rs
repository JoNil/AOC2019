use std::convert::TryFrom;
use std::error::Error;

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl TryFrom<i64> for ParameterMode {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value % 10 {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            2 => Ok(ParameterMode::Relative),
            _ => Err("Unknown parameter mode"),
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

impl TryFrom<i64> for Opcode {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        Ok(match value % 100 {
            1 => Opcode::Add(
                ParameterMode::try_from(value / 100)?,
                ParameterMode::try_from(value / 1000)?,
                ParameterMode::try_from(value / 10000)?,
            ),
            2 => Opcode::Mult(
                ParameterMode::try_from(value / 100)?,
                ParameterMode::try_from(value / 1000)?,
                ParameterMode::try_from(value / 10000)?,
            ),
            3 => Opcode::Input(ParameterMode::try_from(value / 100)?),
            4 => Opcode::Output(ParameterMode::try_from(value / 100)?),
            5 => Opcode::JumpIfTrue(
                ParameterMode::try_from(value / 100)?,
                ParameterMode::try_from(value / 1000)?,
            ),
            6 => Opcode::JumpIfFalse(
                ParameterMode::try_from(value / 100)?,
                ParameterMode::try_from(value / 1000)?,
            ),
            7 => Opcode::LessThen(
                ParameterMode::try_from(value / 100)?,
                ParameterMode::try_from(value / 1000)?,
                ParameterMode::try_from(value / 10000)?,
            ),
            8 => Opcode::Equals(
                ParameterMode::try_from(value / 100)?,
                ParameterMode::try_from(value / 1000)?,
                ParameterMode::try_from(value / 10000)?,
            ),
            9 => Opcode::RelativeBaseOffset(ParameterMode::try_from(value / 100)?),
            99 => Opcode::Halt,
            _ => return Err("Unknown opcode"),
        })
    }
}

fn get_parameter(memory: &[i64], value: i64, mode: ParameterMode, relative_base: i64) -> i64 {
    match mode {
        ParameterMode::Position => memory[value as usize],
        ParameterMode::Immediate => value,
        ParameterMode::Relative => memory[(relative_base + value) as usize],
    }
}

fn get_address(
    value: i64,
    mode: ParameterMode,
    relative_base: i64,
) -> Result<usize, Box<dyn Error>> {
    match mode {
        ParameterMode::Position => Ok(value as usize),
        ParameterMode::Immediate => Err("Invalid address mode for write".into()),
        ParameterMode::Relative => Ok((relative_base + value) as usize),
    }
}

#[derive(Debug)]
pub enum IntcodeOutput {
    Halt(Vec<i64>),
    Interrupt(Vec<i64>),
}

pub struct IntcodeComputer {
    pc: usize,
    relative_base: i64,
    memory: Box<[i64]>,
}

impl IntcodeComputer {
    pub fn new(program: &[i64]) -> IntcodeComputer {
        let memory = {
            let mut memory = Vec::new();
            memory.resize_with(1 * 1024 * 1024, Default::default);
            memory[..program.len()].copy_from_slice(program);
            memory.into_boxed_slice()
        };

        IntcodeComputer {
            pc: 0,
            relative_base: 0,
            memory: memory,
        }
    }
}

impl IntcodeComputer {
    pub fn run(
        &mut self,
        input: &[i64],
        outputs_before_interrupt: Option<i32>,
    ) -> Result<IntcodeOutput, Box<dyn Error>> {
        let mut input_counter = 0;
        let mut output = Vec::new();

        loop {
            let opcode = Opcode::try_from(self.memory[self.pc])?;

            match opcode {
                Opcode::Add(a_mode, b_mode, res_mode) => {
                    let a = self.memory[self.pc + 1];
                    let b = self.memory[self.pc + 2];
                    let res = self.memory[self.pc + 3];

                    self.memory[get_address(res, res_mode, self.relative_base)?] =
                        get_parameter(&self.memory, a, a_mode, self.relative_base)
                            + get_parameter(&self.memory, b, b_mode, self.relative_base);

                    self.pc += 4;
                }
                Opcode::Mult(a_mode, b_mode, res_mode) => {
                    let a = self.memory[self.pc + 1];
                    let b = self.memory[self.pc + 2];
                    let res = self.memory[self.pc + 3];

                    self.memory[get_address(res, res_mode, self.relative_base)?] =
                        get_parameter(&self.memory, a, a_mode, self.relative_base)
                            * get_parameter(&self.memory, b, b_mode, self.relative_base);

                    self.pc += 4;
                }
                Opcode::Input(mode) => {
                    let value = self.memory[self.pc + 1];

                    self.memory[get_address(value, mode, self.relative_base)?] =
                        input[input_counter];

                    input_counter += 1;
                    self.pc += 2;
                }
                Opcode::Output(mode) => {
                    let value = self.memory[self.pc + 1];

                    output.push(get_parameter(&self.memory, value, mode, self.relative_base));

                    self.pc += 2;

                    if let Some(outputs_before_interrupt) = outputs_before_interrupt {
                        if output.len() == outputs_before_interrupt as usize {
                            return Ok(IntcodeOutput::Interrupt(output));
                        }
                    }
                }
                Opcode::JumpIfTrue(a_mode, b_mode) => {
                    let a = self.memory[self.pc + 1];
                    let b = self.memory[self.pc + 2];

                    if get_parameter(&self.memory, a, a_mode, self.relative_base) != 0 {
                        self.pc =
                            get_parameter(&self.memory, b, b_mode, self.relative_base) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Opcode::JumpIfFalse(a_mode, b_mode) => {
                    let a = self.memory[self.pc + 1];
                    let b = self.memory[self.pc + 2];

                    if get_parameter(&self.memory, a, a_mode, self.relative_base) == 0 {
                        self.pc =
                            get_parameter(&self.memory, b, b_mode, self.relative_base) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Opcode::LessThen(a_mode, b_mode, res_mode) => {
                    let a = self.memory[self.pc + 1];
                    let b = self.memory[self.pc + 2];
                    let res = self.memory[self.pc + 3];

                    if get_parameter(&self.memory, a, a_mode, self.relative_base)
                        < get_parameter(&self.memory, b, b_mode, self.relative_base)
                    {
                        self.memory[get_address(res, res_mode, self.relative_base)?] = 1;
                    } else {
                        self.memory[get_address(res, res_mode, self.relative_base)?] = 0;
                    }

                    self.pc += 4;
                }
                Opcode::Equals(a_mode, b_mode, res_mode) => {
                    let a = self.memory[self.pc + 1];
                    let b = self.memory[self.pc + 2];
                    let res = self.memory[self.pc + 3];

                    if get_parameter(&self.memory, a, a_mode, self.relative_base)
                        == get_parameter(&self.memory, b, b_mode, self.relative_base)
                    {
                        self.memory[get_address(res, res_mode, self.relative_base)?] = 1;
                    } else {
                        self.memory[get_address(res, res_mode, self.relative_base)?] = 0;
                    }

                    self.pc += 4;
                }
                Opcode::RelativeBaseOffset(mode) => {
                    let value = self.memory[self.pc + 1];

                    self.relative_base +=
                        get_parameter(&self.memory, value, mode, self.relative_base);

                    self.pc += 2;
                }
                Opcode::Halt => {
                    return Ok(IntcodeOutput::Halt(output));
                }
            }
        }
    }
}
