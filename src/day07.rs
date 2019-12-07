use common::aoc::{load_input, run_many, print_time, print_result, run_many_mut};
use common::intcode::{VM, StepResult};
use common::math::Permutations;

fn main() {
    let input = load_input("day07");

    let ((mut vm1, mut vm2), dur_parse) = run_many(10000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many_mut(1000, move || part1(&mut vm1));
    let (res_part2, dur_part2) = run_many_mut(1000, move || part2(&mut vm2));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

fn parse_input(input: &str) -> (VM, VM) {
    let vm = VM::parse(input.trim_end_matches('\n'));

    (vm.clone(), vm.clone())
}

fn part1(vm: &mut VM) -> i32 {
    let mut best_signal = 0;

    let mut perm = Permutations::new(&[0, 1, 2, 3, 4]);
    while let Some(phases) = perm.next() {
        let mut signal = 0;

        for phase in phases {
            vm.reset();
            vm.push_input(*phase);
            vm.push_input(signal);
            vm.run();
            signal = *vm.output().last().unwrap()
        }

        if signal > best_signal {
            best_signal = signal;
        }
    }

    best_signal
}

fn part2(vm: &mut VM) -> i32 {
    let mut best_signal = 0;
    let mut perm = Permutations::new(&[5, 6, 7, 8, 9]);
    let mut vms: Vec<VM> = (0..5).map(|_| vm.clone()).collect();

    while let Some(phases) = perm.next() {
        for (i, phase) in phases.iter().enumerate() {
            vms[i].reset();
            vms[i].push_input(*phase);
        }
        vms[0].push_input(0);

        loop {
            let mut last_result = StepResult::Continue;

            for i in 0..5 {
                let output = vms[(i + 4) % 5].read_output().first().cloned();
                if let Some(output) = output {
                    vms[i].push_input(output);
                }
                last_result = vms[i].run();
            }

            if last_result == StepResult::Exit {
                break;
            }
        }

        let signal = *vms[4].read_output().last().unwrap();
        if signal > best_signal {
            best_signal = signal;
        }
    }

    best_signal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let mut vm1 = VM::parse("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        let mut vm2 = VM::parse("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
        let mut vm3 = VM::parse("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");

        assert_eq!(part1(&mut vm1), 43210);
        assert_eq!(part1(&mut vm2), 54321);
        assert_eq!(part1(&mut vm3), 65210);
    }

    #[test]
    fn test_part2() {
        let mut vm1 = VM::parse("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
        let mut vm2 = VM::parse("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");

        assert_eq!(part2(&mut vm1), 139629729);
        assert_eq!(part2(&mut vm2), 18216);
    }
}