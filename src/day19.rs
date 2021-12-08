use common::aoc::{load_input, run_many, run_many_mut, print_result, print_time};
use common::intcode::VM;
use common::grid::{Grid, BigGrid};

fn main() {
    let input = load_input("day19");

    let (mut vm, dur_parse) = run_many(1000, || VM::parse(&input.trim_end_matches("\n")));
    let (res_part1, dur_part1) = run_many_mut(100, || part1(&mut vm));
    let (res_part2, dur_part2) = run_many_mut(1, || part2(&mut vm));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}


fn part1(vm: &mut VM) -> usize {
    let mut count = 0;

    for y in 0..50 {
        let mut found = false;

        for x in 0..50 {
            vm.reset();
            vm.push_input(x);
            vm.push_input(y);
            vm.run();

            if *vm.read_output().last().unwrap() == 1 {
                count += 1;
                found = true;
            } else if found {
                break;
            }
        }
    }

    count
}

fn part2(vm: &mut VM) -> i64 {
    for y in 100.. {
        if let Some(v) = part2_check(vm, y) {
            return v;
        }
    }

    -1
}

fn part2_check(vm: &mut VM, y: i64) -> Option<i64> {
    for x in 0.. {
        vm.reset();
        vm.push_input(x);
        vm.push_input(y);
        vm.run();

        if *vm.output().last().unwrap() == 1 {
            vm.reset();
            vm.push_input(x);
            vm.push_input(y-99);
            vm.run();
            if *vm.output().last().unwrap() == 0 {
                return None;
            }
            vm.reset();
            vm.push_input(x+99);
            vm.push_input(y-99);
            vm.run();
            if *vm.output().last().unwrap() == 0 {
                return None;
            }
            vm.reset();
            vm.push_input(x+99);
            vm.push_input(y);
            vm.run();
            if *vm.output().last().unwrap() == 0 {
                return None;
            }

            return Some(x * 10000 + (y - 99));
        }
    }

    None
}
