use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;


    Ok(())
}

#[cfg(test)]
mod tests {
    //use super::{count_orbits, parse_orbits};

    #[test]
    fn test() {
        
    }
}
