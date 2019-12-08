use std::collections::HashMap;
use std::error::Error;
use std::fs;

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

fn count_to_com<'a>(
    bodys: &'a HashMap<String, String>,
    mut body: &'a str,
) -> Result<i32, Box<dyn Error>> {
    let mut res = 0;

    while body != "COM" {
        res += 1;
        body = bodys.get(body).ok_or("Path not found")?;
    }

    Ok(res)
}

fn count_orbits(bodys: &HashMap<String, String>) -> Result<i32, Box<dyn Error>> {
    let mut count = 0;

    for body in bodys.keys() {
        count += count_to_com(bodys, body)?;
    }

    Ok(count)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let bodys = parse_orbits(&input)?;

    let result = count_orbits(&bodys)?;

    println!("Result: {}", result);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{count_orbits, parse_orbits};

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
            K)L";

        let bodys = parse_orbits(input).unwrap();

        assert_eq!(count_orbits(&bodys).unwrap(), 42);
    }
}
