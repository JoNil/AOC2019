use std::error::Error;
use std::fs;

#[derive(Copy, Clone, Debug)]
enum Operation {
    DealIntoNewStack,
    DealWithIncrement(i32),
    Cut(i32),
}

impl Operation {
    fn apply(&self, deck: &mut [i32]) {
        match self {
            &Operation::DealIntoNewStack => {
                deck.reverse();
            }
            &Operation::DealWithIncrement(num) => {
                let old = deck.to_vec();

                let mut pos = 0;
                for card in &old {
                    deck[pos] = *card;
                    pos = (pos + num as usize) % old.len();
                }
            }
            &Operation::Cut(num) => {
                if num > 0 {
                    let index = num as usize;
                    let cut = deck[..index].iter().copied();
                    let rest = deck[index..].iter().copied();
                    let res = rest.chain(cut).collect::<Vec<_>>();
                    deck.copy_from_slice(&res);
                } else {
                    let index = deck.len() - num.abs() as usize;
                    let cut = deck[index..].iter().copied();
                    let rest = deck[..index].iter().copied();
                    let res = cut.chain(rest).collect::<Vec<_>>();
                    deck.copy_from_slice(&res);
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

    let operations = parse_operations(&input);

    let mut stack = (0..10007).collect::<Vec<_>>();

    for operation in operations {
        operation.apply(&mut stack);
    }

    println!(
        "{:#?}",
        stack.iter().position(|n| *n == 2019).ok_or("Not found")?
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::parse_operations;

    #[test]
    fn test_22_a() {
        {
            let input = "deal with increment 7
                deal into new stack
                deal into new stack";

            let operations = parse_operations(&input);

            let mut stack = (0..10).collect::<Vec<_>>();

            for operation in operations {
                operation.apply(&mut stack);
            }

            assert_eq!(stack, [0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
        }

        {
            let input = "cut 6
                deal with increment 7
                deal into new stack";

            let operations = parse_operations(&input);

            let mut stack = (0..10).collect::<Vec<_>>();

            for operation in operations {
                operation.apply(&mut stack);
            }

            assert_eq!(stack, [3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
        }

        {
            let input = "deal with increment 7
                deal with increment 9
                cut -2";

            let operations = parse_operations(&input);

            let mut stack = (0..10).collect::<Vec<_>>();

            for operation in operations {
                operation.apply(&mut stack);
            }

            assert_eq!(stack, [6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
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

            let operations = parse_operations(&input);

            let mut stack = (0..10).collect::<Vec<_>>();

            for operation in operations {
                operation.apply(&mut stack);
            }

            assert_eq!(stack, [9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
        }
    }
}
