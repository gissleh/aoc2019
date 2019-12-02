use common::aoc::{load_input, run_many, print_result, print_time, run_once};

fn main() {
    let input = load_input("day02");

    let (program, dur_parse) = run_many(10000, || parse_input(&input));
    let (res_part1, dur_part1) = run_once(|| part1(&program));
    let (res_part2, dur_part2) = run_once(|| part2(&program));

    print_result("P1", res_part1);
    print_result("P2", res_part2);
    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

fn parse_input(input: &str) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::with_capacity(input.len() / 2);
    let mut next: u32 = 0;
    let zero_char = '0' as u32;

    for ch in input.chars() {
        if ch == '\r' || ch == '\n' {
            continue;
        } else if ch == ',' {
            result.push(next);
            next = 0;
            continue;
        }

        next *= 10;
        next += (ch as u32) - zero_char;
    }

    result.push(next);

    result
}

fn part1(initial_program: &[u32]) -> u32 {
    let mut program = initial_program.to_vec();
    program[1] = 12;
    program[2] = 2;
    run_intcode(&mut program);

    program[0]
}

fn part2(initial_program: &[u32]) -> u32 {
    let mut program = initial_program.to_vec();

    for noun in 0..100 {
        program[1] = noun;
        program[2] = 0;

        run_intcode(&mut program);

        if (program[0] / 100) == 196907 {
            let verb = 19690720 - program[0];

            return (noun * 100) + verb;
        }

        program.copy_from_slice(initial_program);
    }

    panic!("Answer not found for noun-verb pairs in range 0..100")
}

fn run_intcode(program: &mut Vec<u32>) {
    let mut position: usize = 0;

    loop {
        let opcode = program[position];

        match opcode {
            1 => {
                let target = program[position + 3] as usize;
                let left = program[position + 1] as usize;
                let right = program[position + 2] as usize;

                program[target] = program[left] + program[right];
            }
            2 => {
                let target = program[position + 3] as usize;
                let left = program[position + 1] as usize;
                let right = program[position + 2] as usize;

                program[target] = program[left] * program[right];
            }
            99 => {
                break
            }
            _ => {
                panic!("Unknown opcode: {}", opcode)
            }
        }

        position += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_intcode() {
        let mut program = vec![1,0,0,0,99];
        let mut expected = vec![2,0,0,0,99];
        run_intcode(&mut program);
        assert_eq!(program, expected);

        let mut program = vec![2,3,0,3,99];
        let mut expected = vec![2,3,0,6,99];
        run_intcode(&mut program);
        assert_eq!(program, expected);

        let mut program = vec![2,4,4,5,99,0];
        let mut expected = vec![2,4,4,5,99,9801];
        run_intcode(&mut program);
        assert_eq!(program, expected);

        let mut program = vec![1,1,1,4,99,5,6,0,99];
        let mut expected = vec![30,1,1,4,2,5,6,0,99];
        run_intcode(&mut program);
        assert_eq!(program, expected);
    }
}