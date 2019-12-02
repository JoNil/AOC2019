use std::error::Error;
use std::fs;

enum Opcode {
    Add = 1,
    Mult = 2,
    Halt = 99,
}

impl From<i32> for Opcode {
    fn from(value: i32) -> Self {
        match value {
            1 => Opcode::Add,
            2 => Opcode::Mult,
            99 => Opcode::Halt,
            _ => panic!("Unknown opcode"),
        }
    }
}

fn run_program(memory: &mut [i32]) -> Result<(), Box<dyn Error>> {
    let mut pc = 0;

    loop {
        let opcode = Opcode::from(memory[pc]);

        pc = match opcode {
            Opcode::Add => {
                let a_address = memory[pc + 1];
                let b_address = memory[pc + 2];
                let res_address = memory[pc + 3];

                memory[res_address as usize] =
                    memory[a_address as usize] + memory[b_address as usize];

                pc + 4
            }
            Opcode::Mult => {
                let a_address = memory[pc + 1];
                let b_address = memory[pc + 2];
                let res_address = memory[pc + 3];

                memory[res_address as usize] =
                    memory[a_address as usize] * memory[b_address as usize];
                
                pc + 4
            }
            Opcode::Halt => {
                return Ok(());
            }
        }
    }
}

fn find_verb_noun(initial_program: &[i32], result: i32) -> Result<(i32, i32), Box<dyn Error>> {

    for verb in 0..=99 {
        for noun in 0..=99 {

            let mut program = initial_program.to_owned();

            program[1] = verb;
            program[2] = noun;

            run_program(&mut program)?;

            if program[0] == result {
                return Ok((verb, noun));
            }
        }
    }

    Err("Answer not found".into())
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;

    let (verb, noun) = find_verb_noun(&program, 19690720)?;

    println!("program[0] => {}", 100 * verb + noun);

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

            run_program(&mut program).unwrap();
            assert_eq!(&program, &result);
        }

        {
            let mut program = [1, 0, 0, 0, 99];
            let result = [2, 0, 0, 0, 99];

            run_program(&mut program).unwrap();
            assert_eq!(&program, &result);
        }

        {
            let mut program = [2, 3, 0, 3, 99];
            let result = [2, 3, 0, 6, 99];

            run_program(&mut program).unwrap();
            assert_eq!(&program, &result);
        }

        {
            let mut program = [2, 4, 4, 5, 99, 0];
            let result = [2, 4, 4, 5, 99, 9801];

            run_program(&mut program).unwrap();
            assert_eq!(&program, &result);
        }

        {
            let mut program = [1, 1, 1, 4, 99, 5, 6, 0, 99];
            let result = [30, 1, 1, 4, 2, 5, 6, 0, 99];

            run_program(&mut program).unwrap();
            assert_eq!(&program, &result);
        }
    }
}
