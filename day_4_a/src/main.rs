use std::error::Error;

fn segment_num(number: i32) -> Vec<i32> {
    number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .collect()
}

fn follows_rules(number: i32) -> bool {
    let arr = segment_num(number);

    if arr.len() != 6 {
        return false;
    }

    {
        let mut found_adjacent = false;
        let mut last = arr[0];

        for digit in &arr[1..] {
            if *digit == last {
                found_adjacent = true;
                break;
            }
            last = *digit;
        }

        if !found_adjacent {
            return false;
        }
    }

    {
        let mut is_increasing = true;
        let mut last = arr[0];

        for digit in &arr[1..] {
            if *digit < last {
                is_increasing = false;
                break;
            }
            last = *digit;
        }

        if !is_increasing {
            return false;
        }
    }

    return true;
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut num_possible = 0;

    for i in 236491..=713787 {
        if follows_rules(i) {
            num_possible += 1;
        }
    }

    println!("Solution: {}", num_possible);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::follows_rules;

    #[test]
    fn test_follows_rules() {
        assert_eq!(follows_rules(111111), true);
        assert_eq!(follows_rules(223450), false);
        assert_eq!(follows_rules(123789), false);
    }
}
