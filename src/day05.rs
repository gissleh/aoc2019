use common::aoc::{load_input, run_many, print_time, print_result, run_many_mut};
use common::intcode::VM;

fn main() {
    let input = load_input("day05");

    let ((mut vm1, mut vm2), dur_parse) = run_many(10000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many_mut(10000, move || part1(&mut vm1));
    //let (res_part2, dur_part2) = run_many_mut(1000, move || part2(&mut vm2));

    print_result("P1", res_part1);
    //print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    //print_time("P2", dur_part2);
}

fn parse_input(input: &str) -> (VM, VM) {
    let vm = VM::parse(input.trim_end_matches('\n'));

    (vm.clone(), vm.clone())
}

fn part1(vm: &mut VM) -> i32 {
    vm.reset();
    vm.push_input(1);
    vm.run();
    *vm.output().last().unwrap()
}

fn part2(vm: &mut VM) -> i32 {
    vm.reset();
    vm.push_input(5);
    vm.run();
    *vm.output().last().unwrap()
}