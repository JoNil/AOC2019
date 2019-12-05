use std::error::Error;

fn segment_num(number: i32) -> Vec<i32> {
    number
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap() as i32)
        .collect()
}

fn follows_rules_b(number: i32) -> bool {
    let arr = segment_num(number);

    if arr.len() != 6 {
        return false;
    }

    {
        let mut found_pair = false;
        let mut is_long_series = false;
        let mut last = arr[0];
        let mut found_ok_pair = false;

        for digit in &arr[1..] {
            if *digit == last {
                if !is_long_series {
                    if found_pair {
                        found_pair = false;
                        is_long_series = true;
                    } else {
                        found_pair = true;
                    }
                }
            } else {
                if found_pair {
                    found_ok_pair = true;
                }
                is_long_series = false;
            }
            last = *digit;
        }

        if found_ok_pair {
            found_pair = true;
        }

        if !found_pair {
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
        if follows_rules_b(i) {
            num_possible += 1;
        }
    }

    println!("Solution: {}", num_possible);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::follows_rules_b;

    #[test]
    fn test_follows_rules_b() {
        assert_eq!(follows_rules_b(112233), true);
        assert_eq!(follows_rules_b(123444), false);
        assert_eq!(follows_rules_b(111122), true);
        assert_eq!(follows_rules_b(111222), false);
        assert_eq!(follows_rules_b(112222), true);
        assert_eq!(follows_rules_b(113322), false);
    }
}
