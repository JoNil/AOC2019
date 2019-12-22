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
            &Operation::DealIntoNewStack => {
                len - tracked_pos - 1
            }
            &Operation::DealWithIncrement(num) => {

                let num = num as i64;

                //(num*x) % len = tracked_pos

                let mut res = 0;

                for i in 0.. {
                    if (tracked_pos + len*i) % num == 0 {
                        res = (tracked_pos + len*i) / num;
                        break;
                    }
                }

                res
            }
            &Operation::Cut(num) => {

                let num = -num as i64;

                if num > 0 {

                    if tracked_pos > num {
                        tracked_pos + len - num
                    } else {
                        tracked_pos - num
                    }
                    
                } else {

                    let num = len - num.abs();

                    if tracked_pos > num  {
                        tracked_pos + len - num
                    } else {
                        tracked_pos - num
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

    for i in 0..101_741_582_076_661_i64 {
        for operation in &operations {
            tracked_pos = operation.apply(tracked_pos, stack_len);
        }
    }

    println!("{:#?}", tracked_pos);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{parse_operations};

    #[test]
    fn test_22_a() {
        {
            let input = "deal with increment 7
                deal into new stack
                deal into new stack";

            let mut operations = parse_operations(&input);
            operations.reverse();

            let mut res = 9;
            let stack_len = 10;

            for operation in &operations {
                res = operation.apply(res, stack_len);
            }

            assert_eq!(res, 7);
        }

        {
            let input = "cut 6
                deal with increment 7
                deal into new stack";

            let mut operations = parse_operations(&input);
            operations.reverse();

            let mut res = 9;
            let stack_len = 10;

            for operation in &operations {
                res = operation.apply(res, stack_len);
            }

            assert_eq!(res, 6);
        }

        {
            let input = "deal with increment 7
                deal with increment 9
                cut -2";

            let mut operations = parse_operations(&input);
            operations.reverse();

            let mut res = 9;
            let stack_len = 10;

            for operation in &operations {
                res = operation.apply(res, stack_len);
            }

            assert_eq!(res, 9);
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

            let mut res = 9;
            let stack_len = 10;

            for operation in &operations {
                res = operation.apply(res, stack_len);
            }

            assert_eq!(res, 6);
        }
    }
}