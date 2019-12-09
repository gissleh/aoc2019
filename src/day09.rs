use common::aoc::{load_input, run_many, print_time, print_result};
use common::intcode::VM;

fn main() {
    let input = load_input("day09");

    let (vm, dur_parse) = run_many(10000, || VM::parse(input.trim_end_matches('\n')));
    let (res_part1, dur_part1) = run_many(10000, || part1(&mut vm.clone()));
    let (res_part2, dur_part2) = run_many(300, || part2(&mut vm.clone()));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

fn part1(vm: &mut VM) -> i64 {
    vm.push_input(1);
    vm.run();

    if vm.output().len() > 1 {
        panic!("[BOOST] check opcode: {:?}", vm.output())
    }

    *vm.output().first().unwrap()
}

fn part2(vm: &mut VM) -> i64 {
    vm.push_input(2);
    vm.run();

    *vm.output().first().unwrap()
}