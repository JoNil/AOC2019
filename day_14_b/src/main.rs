use std::collections::VecDeque;
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

fn find_fuel_for_ore(reactions: &[Reaction], mut ore_amt: i64) -> i32 {

    let mut fuel_produced = 0;

    let mut remaining_ingridients = Vec::<Ingredient>::new();

    let mut outstanding_requests = VecDeque::new();
    outstanding_requests.push_back(Ingredient {
        amount: 1,
        name: "FUEL".to_owned(),
    });

    loop {

        if outstanding_requests.len() == 0 {
            fuel_produced += 1;
            outstanding_requests.push_back(Ingredient {
                amount: 1,
                name: "FUEL".to_owned(),
            });
        }

        let mut request = outstanding_requests.pop_back().unwrap();

        {
            let mut remaining_ingridient_to_delete = None;

            for ingredient in &mut remaining_ingridients {
                if ingredient.name == request.name {
                    if request.amount > ingredient.amount {
                        request.amount -= ingredient.amount;
                        remaining_ingridient_to_delete = Some(ingredient.name.clone());
                    } else {
                        ingredient.amount -= request.amount;
                        request.amount = 0;
                        if ingredient.amount == 0 {
                            remaining_ingridient_to_delete = Some(ingredient.name.clone());
                        }
                    }

                    break;
                }
            }

            if let Some(to_delete) = remaining_ingridient_to_delete {
                remaining_ingridients = remaining_ingridients.into_iter().filter(|i| i.name != to_delete).collect::<Vec<_>>();
            }   
        }

        if request.amount == 0 {
            continue;
        }

        for reaction in reactions {
            if reaction.output.name == request.name {

                let times = (request.amount - 1) / reaction.output.amount + 1;
                let rest = times * reaction.output.amount - request.amount;

                if rest > 0 {

                    let mut found_ingredient = false;

                    for ingredient in &mut remaining_ingridients {
                        if ingredient.name == request.name {
                            ingredient.amount += rest;
                            found_ingredient = true;
                            break;
                        }
                    }

                    if !found_ingredient {
                        remaining_ingridients.push(Ingredient {
                            name: request.name.clone(),
                            amount: rest,
                        });
                    }
                }

                for input in &reaction.inputs {

                    if input.name == "ORE" {
                        if ore_amt < (times * input.amount) as i64 {
                            return fuel_produced;
                        } else {
                            ore_amt -= (times * input.amount) as i64;
                        }
                    } else {
                        outstanding_requests.push_back(Ingredient {
                            name: input.name.clone(),
                            amount: times * input.amount,
                        });
                    }
                }

                break;
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let input = fs::read_to_string("input")?;

    let reactions = parse_reactions(&input)?;

    let ore = find_fuel_for_ore(&reactions, 1000000000000);

    println!("{}", ore);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::{find_fuel_for_ore, parse_reactions};

    #[test]
    fn test_find_fuel_for_ore() {
        {
            let input = "157 ORE => 5 NZVS
                165 ORE => 6 DCFZ
                44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
                12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
                179 ORE => 7 PSHF
                177 ORE => 5 HKGWZ
                7 DCFZ, 7 PSHF => 2 XJWVT
                165 ORE => 2 GPVTF
                3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

            let reactions = parse_reactions(&input).unwrap();

            assert_eq!(find_fuel_for_ore(&reactions, 1000000000000), 82892753);
        }

        {
            let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
                17 NVRVD, 3 JNWZP => 8 VPVL
                53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
                22 VJHF, 37 MNCFX => 5 FWMGM
                139 ORE => 4 NVRVD
                144 ORE => 7 JNWZP
                5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
                5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
                145 ORE => 6 MNCFX
                1 NVRVD => 8 CXFTF
                1 VJHF, 6 MNCFX => 4 RFSQX
                176 ORE => 6 VJHF";

            let reactions = parse_reactions(&input).unwrap();

            assert_eq!(find_fuel_for_ore(&reactions, 1000000000000), 5586022);
        }

        {
            let input = "171 ORE => 8 CNZTR
                7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
                114 ORE => 4 BHXH
                14 VRPVC => 6 BMBT
                6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
                6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
                15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
                13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
                5 BMBT => 4 WPTQ
                189 ORE => 9 KTJDG
                1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
                12 VRPVC, 27 CNZTR => 2 XDBXC
                15 KTJDG, 12 BHXH => 5 XCVML
                3 BHXH, 2 VRPVC => 7 MZWV
                121 ORE => 7 VRPVC
                7 XCVML => 6 RJRHP
                5 BHXH, 4 VRPVC => 5 LTCX";

            let reactions = parse_reactions(&input).unwrap();

            assert_eq!(find_fuel_for_ore(&reactions, 1000000000000), 460664);
        }
    }
}
