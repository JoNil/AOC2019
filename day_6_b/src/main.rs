use std::error::Error;
use std::fs;
use std::collections::HashMap;

fn parse_orbits(input: &str) -> Result<HashMap<String, String>, Box<dyn Error>> {
    
    let mut res = HashMap::new();
    
    for line in input.lines() {
        
        let split = line.split(")").collect::<Vec<&str>>();
        let orbits = split.get(0).ok_or("Bad input")?.trim();
        let name = split.get(1).ok_or("Bad input")?.trim();

        res.insert(name.to_owned(), orbits.to_owned());
    }

    Ok(res)
}

fn path_to_com<'a>(bodys: &'a HashMap<String, String>, mut body: &'a str) -> Result<Vec<String>, Box<dyn Error>> {

    let mut res = Vec::new();

    while body != "COM" {
        body = bodys.get(body).ok_or("Path not found")?;
        res.push(body.to_owned());
    }

    Ok(res)
}

fn find_transfer(bodys: &HashMap<String, String>, a: &str, b: &str) -> Result<i32, Box<dyn Error>> {

    let a_to_com = path_to_com(&bodys, a)?;
    let b_to_com = path_to_com(&bodys, b)?;

    for (index_a, body_a) in a_to_com.iter().enumerate() {

        for (index_b, body_b) in b_to_com.iter().enumerate() {
            
            if body_a == body_b {

                return Ok(index_a as i32 + index_b as i32);
            }
        }
    }

    return Err("Path not found".into());
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let bodys = parse_orbits(&input)?;

    let transfer = find_transfer(&bodys, "YOU", "SAN").unwrap();

    println!("Result: {}", transfer);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_orbits, find_transfer};

    #[test]
    fn test_count_orbits() {
        let input = "COM)B
            B)C
            C)D
            D)E
            E)F
            B)G
            G)H
            D)I
            E)J
            J)K
            K)L
            K)YOU
            I)SAN";

        let bodys = parse_orbits(input).unwrap();

        let transfer = find_transfer(&bodys, "YOU", "SAN").unwrap();

        assert_eq!(transfer, 4)
    }
}
