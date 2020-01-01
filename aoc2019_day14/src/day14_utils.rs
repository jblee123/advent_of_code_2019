use std::collections::HashMap;
use std::str::FromStr;

pub type Reactions = HashMap<String, Reaction>;
pub type ReactionDepths = HashMap<String, u32>;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Chem {
    name: String,
    amount: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Reaction {
    needed: Vec<Chem>,
    result: Chem,
}

fn chem_from_str(s: &str) -> Chem {
    let mut parts = s.split(" ");
    let amount = usize::from_str(parts.next().unwrap()).unwrap();
    let name = parts.next().unwrap().to_string();
    Chem {
        name: name,
        amount: amount,
    }
}

pub fn parse_input(input: &str) -> Reactions {
    input.lines().map(|line| {
        let mut parts = line.split(" => ");
        let lhs = parts.next().unwrap();
        let rhs = parts.next().unwrap();

        let needed = lhs.split(", ").map(|part| {
            chem_from_str(part)
        })
        .collect::<Vec<Chem>>();

        let result = chem_from_str(rhs);

        (
            result.name.clone(),
            Reaction {
                needed: needed,
                result: result,
            },
        )
    })
    .collect::<Reactions>()
}

fn do_build_depths(
    chem_name: &str,
    reactions: &Reactions,
    depths: &mut HashMap<String, u32>
) {
    if depths.contains_key(chem_name) {
        return;
    }

    let needed_chems = &reactions.get(chem_name).unwrap().needed;
    for needed_chem in needed_chems {
        do_build_depths(&needed_chem.name, reactions, depths);
    }

    let depth = {
        let max_child_depth = needed_chems.iter().map(|needed_chem| {
                depths.get(&needed_chem.name).unwrap()
            })
            .max()
            .unwrap();
        *max_child_depth + 1
    };

    depths.insert(chem_name.to_string(), depth);
}

pub fn build_depths(reactions: &Reactions) -> ReactionDepths {
    let mut depths = ReactionDepths::new();
    depths.insert("ORE".to_string(), 0);
    do_build_depths("FUEL", reactions, &mut depths);
    depths
}

fn get_max_reaction_depth(
    ingredients: &HashMap<String, usize>,
    depths: &ReactionDepths,
) -> u32 {
    ingredients.keys()
        .map(|chem_name| {
            *(depths.get(chem_name).unwrap())
        })
        .max()
        .unwrap()
}

fn get_chem_with_depth(
    ingredients: &HashMap<String, usize>,
    depths: &ReactionDepths,
    target_depth: u32,
) -> String {
    ingredients.keys()
        .filter(|chem_name| {
            *(depths.get(*chem_name).unwrap()) == target_depth
        })
        .take(1)
        .next()
        .unwrap()
        .clone()
}

pub fn get_needed_ore_for_fuel(
    fuel_amount: usize,
    reactions: &Reactions,
    depths: &ReactionDepths,
) -> usize {
    let mut ingredients = HashMap::new();
    ingredients.insert("FUEL".to_string(), fuel_amount);

    loop {
        let max_depth = get_max_reaction_depth(&ingredients, &depths);
        if max_depth == 0 {
            break;
        }

        let chem_name = get_chem_with_depth(&ingredients, &depths, max_depth);
        let cur_chem_amount = ingredients.remove(&chem_name).unwrap();
        let reaction = &reactions.get(&chem_name).unwrap();
        let needed_chems = &reaction.needed;
        for needed_chem in needed_chems {
            let new_name = needed_chem.name.clone();
            let amount = ingredients.entry(new_name).or_insert(0);
            let mut multiplier = cur_chem_amount / reaction.result.amount;
            if (cur_chem_amount % reaction.result.amount) != 0 {
                multiplier += 1;
            }
            *amount += multiplier * needed_chem.amount;
        }
    }

    *ingredients.values().next().unwrap()
}

pub fn get_max_fuel_for_ore(
    mined_ore: usize,
    reactions: &Reactions,
    depths: &ReactionDepths,
) -> usize {
    let mut min_fuel = 0;
    let mut max_fuel = 0;
    loop {
        let ore = get_needed_ore_for_fuel(max_fuel, &reactions, &depths);
        if ore > mined_ore {
            break;
        }
        min_fuel = max_fuel;
        max_fuel = if max_fuel > 0 { max_fuel * 2 } else { 1 };
    };

    while min_fuel < max_fuel - 1 {
        let fuel = (min_fuel + max_fuel) / 2;
        let ore = get_needed_ore_for_fuel(fuel, &reactions, &depths);
        if ore > mined_ore {
            max_fuel = fuel;
        } else {
            min_fuel = fuel;
        }
    }

    min_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT_1: &str = concat!(
        "10 ORE => 10 A\n",
        "1 ORE => 1 B\n",
        "7 A, 1 B => 1 C\n",
        "7 A, 1 C => 1 D\n",
        "7 A, 1 D => 1 E\n",
        "7 A, 1 E => 1 FUEL\n",
    );

    const SAMPLE_INPUT_2: &str = concat!(
        "9 ORE => 2 A\n",
        "8 ORE => 3 B\n",
        "7 ORE => 5 C\n",
        "3 A, 4 B => 1 AB\n",
        "5 B, 7 C => 1 BC\n",
        "4 C, 1 A => 1 CA\n",
        "2 AB, 3 BC, 4 CA => 1 FUEL\n",
    );

    const SAMPLE_INPUT_3: &str = concat!(
        "157 ORE => 5 NZVS\n",
        "165 ORE => 6 DCFZ\n",
        "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n",
        "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n",
        "179 ORE => 7 PSHF\n",
        "177 ORE => 5 HKGWZ\n",
        "7 DCFZ, 7 PSHF => 2 XJWVT\n",
        "165 ORE => 2 GPVTF\n",
        "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT\n",
    );

    const SAMPLE_INPUT_4: &str = concat!(
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n",
        "17 NVRVD, 3 JNWZP => 8 VPVL\n",
        "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n",
        "22 VJHF, 37 MNCFX => 5 FWMGM\n",
        "139 ORE => 4 NVRVD\n",
        "144 ORE => 7 JNWZP\n",
        "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n",
        "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n",
        "145 ORE => 6 MNCFX\n",
        "1 NVRVD => 8 CXFTF\n",
        "1 VJHF, 6 MNCFX => 4 RFSQX\n",
        "176 ORE => 6 VJHF\n",
    );

    const SAMPLE_INPUT_5: &str = concat!(
        "171 ORE => 8 CNZTR\n",
        "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n",
        "114 ORE => 4 BHXH\n",
        "14 VRPVC => 6 BMBT\n",
        "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n",
        "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n",
        "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n",
        "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n",
        "5 BMBT => 4 WPTQ\n",
        "189 ORE => 9 KTJDG\n",
        "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n",
        "12 VRPVC, 27 CNZTR => 2 XDBXC\n",
        "15 KTJDG, 12 BHXH => 5 XCVML\n",
        "3 BHXH, 2 VRPVC => 7 MZWV\n",
        "121 ORE => 7 VRPVC\n",
        "7 XCVML => 6 RJRHP\n",
        "5 BHXH, 4 VRPVC => 5 LTCX\n",
    );

    fn make_reaction(needed: Vec<(&str, usize)>, result: (&str, usize))
    -> Reaction {
        Reaction {
            needed: needed.iter().map(|(name, amount)| {
                Chem {
                    name: name.to_string(),
                    amount: *amount,
                }
            }).collect(),
            result: Chem {
                name: result.0.to_string(),
                amount: result.1,
            },
        }
    }

    #[test]
    fn test_parse_input() {
        let mut target = Reactions::new();
        target.insert(
            "A".to_string(),
            make_reaction(vec![("ORE", 10)], ("A", 10)),
        );
        target.insert(
            "B".to_string(),
            make_reaction(vec![("ORE", 1)], ("B", 1)),
        );
        target.insert(
            "C".to_string(),
            make_reaction(vec![("A", 7), ("B", 1)], ("C", 1)),
        );
        target.insert(
            "D".to_string(),
            make_reaction(vec![("A", 7), ("C", 1)], ("D", 1)),
        );
        target.insert(
            "E".to_string(),
            make_reaction(vec![("A", 7), ("D", 1)], ("E", 1)),
        );
        target.insert(
            "FUEL".to_string(),
            make_reaction(vec![("A", 7), ("E", 1)], ("FUEL", 1)),
        );

        let reactions = parse_input(SAMPLE_INPUT_1);
        assert_eq!(reactions, target);
    }

    #[test]
    fn test_build_depths() {
        let mut target = ReactionDepths::new();
        target.insert("ORE".to_string(), 0);
        target.insert("A".to_string(), 1);
        target.insert("B".to_string(), 1);
        target.insert("C".to_string(), 2);
        target.insert("D".to_string(), 3);
        target.insert("E".to_string(), 4);
        target.insert("FUEL".to_string(), 5);
        let reactions = parse_input(SAMPLE_INPUT_1);
        let depths = build_depths(&reactions);
        assert_eq!(depths, target);
    }

    #[test]
    fn test_get_needed_ore_for_fuel() {
        let reactions = parse_input(SAMPLE_INPUT_1);
        let depths = build_depths(&reactions);

        let result = get_needed_ore_for_fuel(1, &reactions, &depths);
        assert_eq!(result, 31);

        let reactions = parse_input(SAMPLE_INPUT_2);
        let depths = build_depths(&reactions);

        let result = get_needed_ore_for_fuel(1, &reactions, &depths);
        assert_eq!(result, 165);

        let reactions = parse_input(SAMPLE_INPUT_3);
        let depths = build_depths(&reactions);

        let result = get_needed_ore_for_fuel(1, &reactions, &depths);
        assert_eq!(result, 13312);

        let reactions = parse_input(SAMPLE_INPUT_4);
        let depths = build_depths(&reactions);

        let result = get_needed_ore_for_fuel(1, &reactions, &depths);
        assert_eq!(result, 180697);

        let reactions = parse_input(SAMPLE_INPUT_5);
        let depths = build_depths(&reactions);

        let result = get_needed_ore_for_fuel(1, &reactions, &depths);
        assert_eq!(result, 2210736);
    }

    #[test]
    fn test_get_max_fuel_for_ore() {
        const COLLECTED_ORE: usize = 1000000000000;

        let reactions = parse_input(SAMPLE_INPUT_3);
        let depths = build_depths(&reactions);

        let result = get_max_fuel_for_ore(COLLECTED_ORE, &reactions, &depths);
        assert_eq!(result, 82892753);

        let reactions = parse_input(SAMPLE_INPUT_4);
        let depths = build_depths(&reactions);

        let result = get_max_fuel_for_ore(COLLECTED_ORE, &reactions, &depths);
        assert_eq!(result, 5586022);

        let reactions = parse_input(SAMPLE_INPUT_5);
        let depths = build_depths(&reactions);

        let result = get_max_fuel_for_ore(COLLECTED_ORE, &reactions, &depths);
        assert_eq!(result, 460664);
    }
}
