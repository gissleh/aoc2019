use common::aoc::{load_input, run_many, run_many_mut, print_result, print_time};
use common::intcode::VM;

fn main() {
    let input = load_input("day15");

    let (mut vm, dur_parse) = run_many(1000, || VM::parse(&input.trim_end_matches("\n")));
    let (res_part1, dur_part1) = run_many_mut(100, || part1_dfs(&mut vm));
    let (res_part2, dur_part2) = run_many(100, || part2_dfs(vm.clone()));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

const OPPOSITE_DIRS: [i64; 5] = [0, 2, 1, 4, 3];

fn part1_dfs(vm: &mut VM) -> usize {
    let mut stack: Vec<Search> = Vec::with_capacity(128);

    vm.reset();

    stack.push(Search{dir: 0, prev: 0});

    loop {
        let mut last = stack.pop().unwrap();

        last.dir += 1;
        if last.dir == last.prev {
            last.dir += 1;
        }
        if last.dir == 5 {
            vm.push_input(last.prev);
            continue;
        }
        vm.push_input(last.dir);
        vm.run();

        let output = vm.read_output().last().unwrap();

        match output {
            0 => {
                stack.push(last);
            }
            1 => {
                let opposite_dir = OPPOSITE_DIRS[last.dir as usize];

                stack.push(last);
                stack.push(Search{prev: opposite_dir, dir: 0});
            }
            2 => {
                return stack.len() + 1;
            }
            n => panic!("invalid response: {}", n)
        }
    }
}

fn part2_dfs(mut vm: VM) -> usize {
    let mut stack: Vec<Search> = Vec::with_capacity(128);
    let mut greatest_distance = 0;

    stack.push(Search{dir: 0, prev: 0});

    while stack.len() > 0 {
        let mut last = stack.pop().unwrap();

        last.dir += 1;
        if last.dir == last.prev {
            last.dir += 1;
        }
        if last.dir == 5 {
            vm.push_input(last.prev);
            continue;
        }
        vm.push_input(last.dir);
        vm.run();

        let output = vm.read_output().last().unwrap();

        match output {
            0 | 2 => {
                stack.push(last);
            }
            1 => {
                let opposite_dir = OPPOSITE_DIRS[last.dir as usize];

                stack.push(last);
                stack.push(Search{prev: opposite_dir, dir: 0});

                if stack.len() > greatest_distance {
                    greatest_distance = stack.len() - 1;
                }
            }
            n => panic!("invalid response: {}", n)
        }
    }

    greatest_distance
}

#[derive(Clone, Copy, std::fmt::Debug)]
struct Search {
    prev: i64,
    dir: i64,
}