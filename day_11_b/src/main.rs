use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    //println!("{:?}", res);

    Ok(())
}

#[cfg(test)]
mod tests {
    //use super::;

    #[test]
    fn test() {
        let input = "";
    }
}
