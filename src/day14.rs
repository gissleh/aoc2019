use common::aoc::{load_input, run_many, print_time, print_result};
use std::collections::HashMap;

fn main() {
    let input = load_input("day14");

    let (chain, dur_parse) = run_many(1000, || ReactionChain::parse(&input));
    let (res_part1, dur_part1) = run_many(100, || chain.min_opf(1));
    let (res_part2, dur_part2) = run_many(100, || chain.max_fpo(1_000_000_000_000));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

struct ReactionChain {
    list: Vec<Material>,
    map: HashMap<String, usize>,
}

impl ReactionChain {
    fn ensure(&mut self, name: &str) -> usize {
        if let Some(index) = self.map.get(name) {
            return *index;
        };

        let index = self.list.len();

        self.list.push(Material{
            amount: 0,
            dependencies: Vec::with_capacity(4),
            dependents: Vec::with_capacity(4),
        });
        self.map.insert(name.to_owned(), index);

        index
    }

    fn min_opf(&self, fuel_count: u64) -> u64 {
        let mut amounts = vec![0; self.list.len()];
        let mut completed = vec![false; self.list.len()];
        let mut deferred: Vec<usize> = Vec::with_capacity(self.list.len());
        let mut remaining: Vec<usize> = (0..self.list.len()).collect();
        let ore_index = self.map["ORE"];
        let fuel_index = self.map["FUEL"];

        amounts[fuel_index] = fuel_count;

        remaining.swap_remove(ore_index);

        while remaining.len() > 0 {
            let mat_index = remaining.pop().unwrap();

            // Fabricate not until all of the material is present.
            let mut mise_en_place = true;
            for dep_index in self.list[mat_index].dependents.iter().cloned() {
                if !completed[dep_index] {
                    mise_en_place = false;
                    break;
                }
            }
            if !mise_en_place {
                // Save it for later.
                deferred.push(mat_index);

                // Before quitting, make sure that there aren't any deferred materials.
                if remaining.len() == 0 {
                    remaining.extend(deferred.iter());
                    deferred.clear();
                }

                continue;
            }

            // Break it up into dependencies.
            let material = &self.list[mat_index];
            let amount = amounts[mat_index];
            let needed = if amount % material.amount == 0 { amount / material.amount } else { (amount / material.amount) + 1 };
            for dep in material.dependencies.iter() {
                amounts[dep.index] += dep.amount * needed;
            }

            // Mark as completed, thus allowing dependencies to be manufactured.
            completed[mat_index] = true;

            // Before quitting, make sure that there aren't any deferred materials.
            if remaining.len() == 0 {
                remaining.extend(deferred.iter());
                deferred.clear();
            }
        }

        amounts[ore_index]
    }

    fn max_fpo(&self, ore_count: u64) -> u64 {
        let mut last_good = 0;
        let mut step = 1000000;
        let mut current = 1;

        loop {
            let result = self.min_opf(current);
            if result > ore_count {
                current -= step;
                step /= 2;

                if step == 0 {
                    break;
                }
            } else {
                last_good = current;
            }

            current += step;
        }

        last_good
    }

    fn new() -> ReactionChain {
        ReactionChain {
            map: HashMap::with_capacity(128),
            list: Vec::with_capacity(128),
        }
    }

    fn parse(input: &str) -> ReactionChain {
        let mut chain = Self::new();

        let ore_index = chain.ensure("ORE");
        chain.list[ore_index].amount = 1;

        for line in input.lines() {
            if line.len() == 0 {
                continue;
            }

            let (left, right) = {
                let mut tokens = line.split("=>");
                (tokens.next().unwrap(), tokens.next().unwrap())
            };

            let (result_name, result_amount) = parse_mat_qty(right);
            let result_index = chain.ensure(result_name);

            chain.list[result_index].amount = result_amount;

            for token in left.split(',') {
                let (dependency_name, dependency_amount) = parse_mat_qty(token);
                let dependency_index = chain.ensure(dependency_name);

                chain.list[result_index].dependencies.push(Dependency{
                    index: dependency_index,
                    amount: dependency_amount,
                });

                chain.list[dependency_index].dependents.push(result_index);
            }
        }

        chain
    }
}

struct Material {
    amount: u64,
    dependencies: Vec<Dependency>,
    dependents: Vec<usize>
}

struct Dependency {
    index: usize,
    amount: u64,
}

fn parse_mat_qty(s: &str) -> (&str, u64) {
    let mut amount = 0;
    let mut start_index = 0;
    let mut end_index = 0;

    for (i, ch) in s.chars().enumerate() {
        match ch {
            'A'..='Z' => {
                if start_index == 0 {
                    start_index = i;
                    end_index = i;
                } else {
                    end_index = i;
                }
            }
            '0'..='9' => {
                amount *= 10;
                amount += (ch as u8 - '0' as u8) as u64;
            }
            _ => {}
        }
    }

    (&s[start_index..=end_index], amount)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mat_qty() {
        assert_eq!(parse_mat_qty("9 ORE"), ("ORE", 9));
        assert_eq!(parse_mat_qty(" 1 GPVTF"), ("GPVTF", 1));
        assert_eq!(parse_mat_qty(" 7 PSHF "), ("PSHF", 7));
        assert_eq!(parse_mat_qty("179 ORE "), ("ORE", 179));
        assert_eq!(parse_mat_qty(" 2 A"), ("A", 2));
    }

    const TEST_INPUT1: &str = "9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL\n";
    const TEST_INPUT2: &str = "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT\n";
    const TEST_INPUT3: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n17 NVRVD, 3 JNWZP => 8 VPVL\n53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n22 VJHF, 37 MNCFX => 5 FWMGM\n139 ORE => 4 NVRVD\n144 ORE => 7 JNWZP\n5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n145 ORE => 6 MNCFX\n1 NVRVD => 8 CXFTF\n1 VJHF, 6 MNCFX => 4 RFSQX\n176 ORE => 6 VJHF";
    const TEST_INPUT4: &str = "171 ORE => 8 CNZTR\n7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n114 ORE => 4 BHXH\n14 VRPVC => 6 BMBT\n6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n5 BMBT => 4 WPTQ\n189 ORE => 9 KTJDG\n1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n12 VRPVC, 27 CNZTR => 2 XDBXC\n15 KTJDG, 12 BHXH => 5 XCVML\n3 BHXH, 2 VRPVC => 7 MZWV\n121 ORE => 7 VRPVC\n7 XCVML => 6 RJRHP\n5 BHXH, 4 VRPVC => 5 LTCX";

    #[test]
    fn test_part1() {
        let test1 = ReactionChain::parse(TEST_INPUT1);
        let test2 = ReactionChain::parse(TEST_INPUT2);
        let test3 = ReactionChain::parse(TEST_INPUT3);
        let test4 = ReactionChain::parse(TEST_INPUT4);

        assert_eq!(test1.min_opf(1), 165);
        assert_eq!(test2.min_opf(1), 13312);
        assert_eq!(test3.min_opf(1), 180697);
        assert_eq!(test4.min_opf(1), 2210736);
    }

    const PART2_GOAL: u64 = 1_000_000_000_000;

    #[test]
    fn test_part2() {
        let test2 = ReactionChain::parse(TEST_INPUT2);
        let test3 = ReactionChain::parse(TEST_INPUT3);
        let test4 = ReactionChain::parse(TEST_INPUT4);

        assert_eq!(test2.max_fpo(PART2_GOAL), 82892753);
        assert_eq!(test3.max_fpo(PART2_GOAL), 5586022);
        assert_eq!(test4.max_fpo(PART2_GOAL), 460664);
    }
}



















