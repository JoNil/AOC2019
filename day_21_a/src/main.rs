use int_comp::IntcodeComputer;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let program = input
        .split(",")
        .map(|v| v.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let input = "NOT A T
NOT T T
AND B T
AND C T
NOT D J
OR J T
NOT T J
WALK
";

    let mut incode = IntcodeComputer::new(&program);

    let output = incode.run(&input.chars().map(|c| c as i64).collect::<Vec<_>>(), None)?;

    for data in output.data() {
        print!("{}", *data as u8 as char);
    }

    println!("");

    println!("{}", output.data().last().ok_or("No Output")?);

    Ok(())
}
