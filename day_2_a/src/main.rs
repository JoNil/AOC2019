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

fn run(memory: &mut [i32]) -> Result<(), Box<dyn Error>> {
    let mut pc = 0;

    loop {
        let opcode = Opcode::from(memory[pc]);

        match opcode {
            Opcode::Add => {
                let a_address = memory[pc + 1];
                let b_address = memory[pc + 2];
                let res_address = memory[pc + 3];

                memory[res_address as usize] =
                    memory[a_address as usize] + memory[b_address as usize];
            }
            Opcode::Mult => {
                let a_address = memory[pc + 1];
                let b_address = memory[pc + 2];
                let res_address = memory[pc + 3];

                memory[res_address as usize] =
                    memory[a_address as usize] * memory[b_address as usize];
            }
            Opcode::Halt => {
                return Ok(());
            }
        }

        pc += 4;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut program = input
        .split(",")
        .map(|v| v.trim().parse::<i32>())
        .collect::<Result<Vec<_>, _>>()?;
    program[1] = 12;
    program[2] = 2;

    run(&mut program)?;

    println!("program[0] => {}", program[0]);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn test_run() {
        {
            let mut program = [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
            let result = [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50];

            run(&mut program).unwrap();
            assert_eq!(&program, &result);
        }

        {
            let mut program = [1, 0, 0, 0, 99];
            let result = [2, 0, 0, 0, 99];

            run(&mut program).unwrap();
            assert_eq!(&program, &result);
        }

        {
            let mut program = [2, 3, 0, 3, 99];
            let result = [2, 3, 0, 6, 99];

            run(&mut program).unwrap();
            assert_eq!(&program, &result);
        }

        {
            let mut program = [2, 4, 4, 5, 99, 0];
            let result = [2, 4, 4, 5, 99, 9801];

            run(&mut program).unwrap();
            assert_eq!(&program, &result);
        }

        {
            let mut program = [1, 1, 1, 4, 99, 5, 6, 0, 99];
            let result = [30, 1, 1, 4, 2, 5, 6, 0, 99];

            run(&mut program).unwrap();
            assert_eq!(&program, &result);
        }
    }
}
