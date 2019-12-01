use common::aoc::{run_once, load_input, run_many, print_time, print_result};

fn main() {
    let (list, dur_input_load) = run_once(|| {
        let mut list: Vec<u32> = Vec::with_capacity(128);

        for line in load_input("day01").lines() {
            if line.len() == 0 {
                continue;
            }

            list.push(line.parse().unwrap());
        }

        list
    });

    let (res_part1, dur_part1) = run_many(100000, || part1(&list));
    let (res_part2, dur_part2) = run_many(100000, || part2(&list));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
    print_time("Input", dur_input_load);
}

fn part1(list: &[u32]) -> u32 {
    let mut total: u32 = 0;
    for item in list.iter() {
        total += calc_fuel_required(*item)
    }

    total
}

fn part2(list: &[u32]) -> u32 {
    let mut total: u32 = 0;
    for item in list.iter() {
        total += calc_fuel_required_re(*item)
    }

    total
}

fn calc_fuel_required(mass: u32) -> u32 {
    (mass / 3) - 2
}

fn calc_fuel_required_re(mass: u32) -> u32 {
    let mut fuel = (mass / 3) - 2;
    let mut fuel_accounted_for: u32 = 0;

    loop {
        let fuel_diff = fuel - fuel_accounted_for;
        if fuel_diff < 6 {
            break
        }

        fuel_accounted_for = fuel;
        fuel += (fuel_diff / 3) - 2;
    }

    fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_fuel_required() {
        assert_eq!(calc_fuel_required(12), 2);
        assert_eq!(calc_fuel_required(14), 2);
        assert_eq!(calc_fuel_required(1969), 654);
        assert_eq!(calc_fuel_required(100756), 33583);
    }

    #[test]
    fn test_calc_fuel_required_re() {
        assert_eq!(calc_fuel_required_re(14), 2);
        assert_eq!(calc_fuel_required_re(1969), 966);
        assert_eq!(calc_fuel_required_re(100756), 50346);
    }
}
