use std::error::Error;
use std::fs;

#[derive(Copy, Clone, Debug)]
enum Operation {
    DealIntoNewStack,
    DealWithIncrement(i32),
    Cut(i32),
}

impl Operation {
    fn apply(&self, tracked_pos: i64, len: i64) -> i64 {
        match self {
            &Operation::DealIntoNewStack => len - tracked_pos - 1,
            &Operation::DealWithIncrement(num) => {
                let num = num as i64;

                //(num*x) % len = tracked_pos

                let mut res = 0;

                for i in 0.. {
                    if (tracked_pos + len * i) % num == 0 {
                        res = (tracked_pos + len * i) / num;
                        break;
                    }
                }

                //[0, 3, 6, 9, 2, 5, 8, 1, 4, 7]

                dbg!(num);
                dbg!(tracked_pos);
                dbg!(len);

                dbg!(res)
            }
            &Operation::Cut(val) => {
                let num = -val as i64;

                if num > 0 {
                    if tracked_pos > num {
                        tracked_pos + len - num
                    } else {
                        tracked_pos - num
                    }
                } else {
                    let inv_num = num.abs();

                    if len - dbg!(inv_num) > tracked_pos {
                        tracked_pos + inv_num
                    } else {
                        tracked_pos - len + inv_num
                    }
                }
            }
        }
    }
}

fn parse_operations(input: &str) -> Vec<Operation> {
    let mut res = Vec::new();

    for line in input.lines() {
        let line = line.trim();

        if line.starts_with("deal with increment ") {
            if let Some(num) = line.split("deal with increment ").nth(1) {
                if let Ok(num) = num.parse::<i32>() {
                    res.push(Operation::DealWithIncrement(num));
                }
            }
        } else if line.starts_with("cut ") {
            if let Some(num) = line.split("cut ").nth(1) {
                if let Ok(num) = num.parse::<i32>() {
                    res.push(Operation::Cut(num));
                }
            }
        } else if line.starts_with("deal into new stack") {
            res.push(Operation::DealIntoNewStack);
        }
    }

    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let mut operations = parse_operations(&input);
    operations.reverse();

    let mut tracked_pos = 2020;
    let stack_len = 119_315_717_514_047_i64;

    for _i in 0..101_741_582_076_661_i64 {
        for operation in &operations {
            tracked_pos = operation.apply(tracked_pos, stack_len);
        }
    }

    println!("{:#?}", tracked_pos);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_operations;

    #[test]
    fn test_22_b() {
        {
            let input = "deal with increment 7
                deal into new stack
                deal into new stack";

            let mut operations = parse_operations(&input);
            operations.reverse();

            let inputs = [0, 3, 6, 9, 2, 5, 8, 1, 4, 7];

            for (index, input) in inputs.iter().enumerate() {
                let mut res = *input;
                for operation in &operations {
                    res = operation.apply(res, inputs.len() as i64);
                }

                assert_eq!(res, index as i64);
            }
        }

        {
            let input = "cut 6
                deal with increment 7
                deal into new stack";

            let mut operations = parse_operations(&input);
            operations.reverse();

            let inputs = [3, 0, 7, 4, 1, 8, 5, 2, 9, 6];

            for (index, input) in inputs.iter().enumerate() {
                let mut res = *input;
                for operation in &operations {
                    res = operation.apply(res, inputs.len() as i64);
                }

                assert_eq!(res, index as i64);
            }
        }

        {
            let input = "deal with increment 7
                deal with increment 9
                cut -2";

            let mut operations = parse_operations(&input);
            operations.reverse();

            let inputs = [6, 3, 0, 7, 4, 1, 8, 5, 2, 9];

            for (index, input) in inputs.iter().enumerate() {
                let mut res = *input;
                for operation in &operations {
                    res = operation.apply(res, inputs.len() as i64);
                }

                assert_eq!(res, index as i64);
            }
        }

        {
            let input = "deal into new stack
                cut -2
                deal with increment 7
                cut 8
                cut -4
                deal with increment 7
                cut 3
                deal with increment 9
                deal with increment 3
                cut -1";

            let mut operations = parse_operations(&input);
            operations.reverse();

            let inputs = [9, 2, 5, 8, 1, 4, 7, 0, 3, 6];

            for (index, input) in inputs.iter().enumerate() {
                let mut res = *input;
                for operation in &operations {
                    res = operation.apply(res, inputs.len() as i64);
                }

                assert_eq!(res, index as i64);
            }
        }
    }
}
