use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Ingredient {
    amount: i32,
    name: String,
}

#[derive(Debug)]
struct Reaction {
    inputs: Vec<Ingredient>,
    output: Ingredient,
}

fn parse_reactions(input: &str) -> Result<Vec<Reaction>, Box<dyn Error>> {
    let mut res = Vec::new();

    for line in input.lines() {
        if let [inputs_str, output_str] = line.split("=>").collect::<Vec<_>>().as_slice() {
            let mut inputs = Vec::new();

            for input_str in inputs_str.split(",") {
                if let [amount, name] = input_str.trim().split(" ").collect::<Vec<_>>().as_slice() {
                    inputs.push(Ingredient {
                        amount: amount.parse()?,
                        name: name.to_string(),
                    })
                }
            }

            if let [amount, name] = output_str.trim().split(" ").collect::<Vec<_>>().as_slice() {
                let output = Ingredient {
                    amount: amount.parse()?,
                    name: name.to_string(),
                };

                res.push(Reaction {
                    inputs: inputs,
                    output: output,
                });
            }
        }
    }

    Ok(res)
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("input")?;

    let reactions = parse_reactions(&input)?;

    println!("{:#?}", reactions);

    Ok(())
}
