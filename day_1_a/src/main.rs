use std::fs;
use std::error::Error;

fn calculate_fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut total_fuel = 0;

    for line in input.lines() {

        let mass = line.parse::<i32>()?;

        total_fuel += calculate_fuel(mass);
    }

    println!("Toral Fuel: {}", total_fuel);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::calculate_fuel;

    #[test]
    fn test_add() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }
}