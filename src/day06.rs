use std::collections::BTreeMap;
use common::aoc::{load_input, run_many, print_time, print_result};

fn main() {
    let input = load_input("day06");

    let (set, dur_parse) = run_many(1000, || OrbiterSet::parse(&input));
    let (res_part1, dur_part1) = run_many(1000, || set.checksum());
    let (res_part2, dur_part2) = run_many(1000, || set.num_transfers("YOU", "SAN"));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

struct OrbiterSet {
    list: Vec<Orbiter>,
    map: BTreeMap<u32, usize>,
}

impl OrbiterSet {
    fn ensure_orbiter(&mut self, name: &str) -> usize {
        let name_key = str_to_key(name);

        if let Some(index) = self.map.get(&name_key) {
            *index
        } else {
            let index = self.list.len();
            self.list.push(Orbiter{
                name_key,
                parent: 0,
            });

            self.map.insert(name_key, index);

            index
        }
    }

    fn find_orbiter(&self, name: &str) -> usize {
        let name_key = str_to_key(name);
        *self.map.get(&name_key).unwrap()
    }

    fn checksum(&self) -> u32 {
        let mut levels: Vec<u32> = vec![0; self.list.len()];
        let mut stack: Vec<usize> = Vec::with_capacity(16);

        for (index, orbiter) in self.list.iter().enumerate() {
            if orbiter.parent == index || levels[index] != 0 {
                continue;
            }

            stack.clear();

            let offset: u32;
            let mut current_index = orbiter.parent;

            loop {
                let orbiter = &self.list[current_index];
                if orbiter.parent == current_index {
                    offset = 0;
                    break;
                }

                let level = levels[current_index];
                if level != 0 {
                    offset = level;
                    break;
                }

                stack.push(current_index);

                current_index = orbiter.parent;
            }

            for (i, current_index) in stack.iter().enumerate() {
                levels[*current_index] = offset + (stack.len() - i) as u32;
            }

            levels[index] = 1 + offset + stack.len() as u32;
        };

        levels.iter().sum()
    }

    fn num_transfers(&self, from: &str, to: &str) -> u32 {
        let from_index = self.find_orbiter(from);
        let to_index = self.find_orbiter(to);
        let infinite_distance = self.list.len() as u32;
        let mut distances: Vec<u32> = vec![infinite_distance; self.list.len()];

        let mut current_index = self.list[from_index].parent;
        let mut distance = 0;
        loop {
            distances[current_index] = distance;

            let parent = self.list[current_index].parent;
            if parent == current_index {
                break;
            }

            current_index = parent;
            distance += 1;
        }

        let mut current_index = self.list[to_index].parent;
        let mut distance = 0;
        loop {
            let other_distance = distances[current_index];
            if other_distance != infinite_distance {
                return distance + other_distance;
            }

            let parent = self.list[current_index].parent;
            if parent == current_index {
                break;
            }

            current_index = parent;
            distance += 1;
        }

        panic!("No orbit!");
    }

    fn new() -> OrbiterSet {
        OrbiterSet {
            list: Vec::with_capacity(512),
            map: BTreeMap::new(),
        }
    }

    fn parse(str: &str) -> OrbiterSet {
        let mut om = Self::new();

        for line in str.lines() {
            if line.len() < 2 {
                continue;
            }

            let mut tokens = line.split(')');
            let left: &str = tokens.next().unwrap();
            let right: &str = tokens.next().unwrap();

            let left_index = om.ensure_orbiter(left);
            let right_index = om.ensure_orbiter(right);

            om.list[right_index].parent = left_index;
        }

        let com_index = om.find_orbiter("COM");
        om.list[com_index].parent = com_index;

        om
    }
}

fn str_to_key(str: &str) -> u32 {
    if str.len() == 3 {
        let mut chars = str.chars();
        (chars.next().unwrap() as u32 * 256 * 256) +
        (chars.next().unwrap() as u32 * 256) +
        (chars.next().unwrap() as u32)
    } else {
        str.chars().next().unwrap() as u32
    }
}

#[allow(dead_code)]
fn key_to_str(key: u32) -> String {
    if key > 256 {
        [
            ((key / 256 / 256) % 256) as u8 as char,
            ((key / 256) % 256) as u8 as char,
            (key % 256) as u8 as char,
        ].iter().collect()
    } else {
        [key as u8 as char].iter().collect()
    }
}

struct Orbiter {
    parent: usize,
    #[allow(dead_code)]
    name_key: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_STR_P1: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L";
    const INPUT_STR_P2: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN\n";

    #[test]
    fn test_checksum() {
        let set = OrbiterSet::parse(INPUT_STR_P1);

        assert_eq!(set.checksum(), 42);
    }

    #[test]
    fn test_num_transfers() {
        let set = OrbiterSet::parse(INPUT_STR_P2);

        assert_eq!(set.num_transfers("YOU", "SAN"), 4);
    }
}