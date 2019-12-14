use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let reaction_text = get_contents("input");

    dbg!(get_num_ore(&reaction_text));
}

fn get_contents(filename: &str) -> String {
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    contents.trim().to_string()
}

#[derive(Debug)]
struct Reaction {
    reagants: HashMap<String, i64>,
    products: HashMap<String, i64>,
}

impl Reaction {
    fn from_string(reaction: &str) -> Reaction {
        let mut reagants: HashMap<String, i64> = HashMap::new();
        let mut products: HashMap<String, i64> = HashMap::new();

        let mut spl = reaction.split(" => ");
        let str_reagants = spl.next().unwrap();
        let str_products = spl.next().unwrap();

        for reagant in str_reagants.split(", ") {
            let mut spl_reagant = reagant.split(" ");
            let amount = spl_reagant.next().unwrap();
            let name = spl_reagant.next().unwrap();
            reagants.insert(name.to_string(), amount.parse::<i64>().unwrap());
        }

        for product in str_products.split(", ") {
            let mut spl_product = product.split(" ");
            let amount = spl_product.next().unwrap();
            let name = spl_product.next().unwrap();
            products.insert(name.to_string(), amount.parse::<i64>().unwrap());
        }

        Reaction {
            reagants: reagants,
            products: products,
        }
    }
}

fn get_num_ore(reaction_text: &str) -> i64 {
    let reactions: Vec<Reaction> = reaction_text
        .lines()
        .map(|x| Reaction::from_string(x))
        .collect();

    let mut elements: HashMap<String, i64> = HashMap::new();
    elements.insert("FUEL".to_string(), 1);

    loop {
        for (element, element_amount) in elements.clone() {
            if element_amount < 0 {
                continue;
            }
            for reaction in &reactions {
                if reaction.products.contains_key(&element) {
                    let amount_needed = reaction.products.get(&element).unwrap();
                    let num_reactions =
                        (element_amount as f64 / *amount_needed as f64).ceil() as i64;
                    for (reagant, amount) in &reaction.reagants {
                        *elements.entry(reagant.to_string()).or_insert(0) +=
                            num_reactions * *amount;
                    }
                    let element_amount_entry = elements.entry(element.clone()).or_insert(0);
                    *element_amount_entry -= num_reactions * amount_needed;
                    if *elements.get(&element).unwrap() == 0 {
                        elements.remove(&element);
                    }
                }
            }
        }
        if elements
            .iter()
            .filter(|&(k, _)| *k != "ORE")
            .all(|(_, v)| *v < 0)
        {
            dbg!(&elements);
            break;
        }
    }

    *elements.get("ORE").unwrap()
}

mod tests {
    #[test]
    fn test() {
        use indoc::indoc;

        use super::*;

        let reaction_text = indoc!(
            "10 ORE => 10 A
             1 ORE => 1 B
             7 A, 1 B => 1 C
             7 A, 1 C => 1 D
             7 A, 1 D => 1 E
             7 A, 1 E => 1 FUEL"
        );

        assert!(get_num_ore(reaction_text) == 31);

        let reaction_text = indoc!(
            "9 ORE => 2 A
             8 ORE => 3 B
             7 ORE => 5 C
             3 A, 4 B => 1 AB
             5 B, 7 C => 1 BC
             4 C, 1 A => 1 CA
             2 AB, 3 BC, 4 CA => 1 FUEL"
        );

        assert!(get_num_ore(reaction_text) == 165);

        let reaction_text = indoc!(
            "157 ORE => 5 NZVS
             165 ORE => 6 DCFZ
             44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
             12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
             179 ORE => 7 PSHF
             177 ORE => 5 HKGWZ
             7 DCFZ, 7 PSHF => 2 XJWVT
             165 ORE => 2 GPVTF
             3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
        );

        assert!(get_num_ore(reaction_text) == 13312);

        let reaction_text = indoc!(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
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
             176 ORE => 6 VJHF"
        );

        assert!(get_num_ore(reaction_text) == 180697);

        let reaction_text = indoc!(
            "171 ORE => 8 CNZTR
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
             5 BHXH, 4 VRPVC => 5 LTCX"
        );

        assert!(get_num_ore(reaction_text) == 2210736);
    }
}
