use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

   
    //println!("{}", ore);

    Ok(())
}

#[cfg(test)]
mod tests {

    //use super::{find_fuel_for_ore, parse_reactions};

    #[test]
    fn test() {
    }
}
