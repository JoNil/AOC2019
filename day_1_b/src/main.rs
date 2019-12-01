use std::error::Error;
use std::fs;

fn calculate_fuel_inner(mass: i32) -> i32 {
    mass / 3 - 2
}

fn calculate_fuel(mass: i32) -> i32 {
    let mut fuel = calculate_fuel_inner(mass);
    let mut total_fuel = fuel;

    while fuel > 0 {
        let fuel_for_fuel = calculate_fuel_inner(fuel);

        total_fuel += fuel_for_fuel.max(0);

        fuel = fuel_for_fuel;
    }

    total_fuel
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut total_fuel = 0;

    for line in input.lines() {
        let mass = line.parse::<i32>()?;

        let new_fuel = calculate_fuel(mass);

        total_fuel += new_fuel;
    }

    println!("Toral Fuel: {}", total_fuel);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::calculate_fuel;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(1969), 966);
        assert_eq!(calculate_fuel(100756), 50346);
    }
}
