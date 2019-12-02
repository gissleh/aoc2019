use common::aoc::{load_input, run_many, print_result, print_time};

fn main() {
    let input = load_input("day02");

    let (program, dur_parse) = run_many(10000, || parse_input(&input));
    let (res_part1, dur_part1) = run_many(10000, || part1(&program));
    let (res_part2, dur_part2) = run_many(10000, || part2(&program));
    let (res_part2_bs, dur_part2_bs) = run_many(10000, || part2_bs(&program));
    let (res_part2_reduce, dur_part2_reduce) = run_many(10000, || part2_reduce(&program));

    print_result("P1", res_part1);
    print_result("P2: Pattern Exploit", res_part2);
    print_result("P2: Binary Search", res_part2_bs);
    print_result("P2: Reduce [not my idea]", res_part2_reduce);
    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2: Pattern Exploit", dur_part2);
    print_time("P2: Binary Search", dur_part2_bs);
    print_time("P2: Reduce [not my idea]", dur_part2_reduce);
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

const PART2_TARGET: u32 = 19690720;
const PART2_TARGET_DIV100: u32 = 196907;

fn part2(initial_program: &[u32]) -> u32 {
    let mut program = initial_program.to_vec();

    for noun in 0..100 {
        program[1] = noun;
        program[2] = 0;

        run_intcode(&mut program);

        let hundred = program[0] / 100;
        if hundred == PART2_TARGET_DIV100 || hundred == PART2_TARGET_DIV100 - 1 {
            let verb = PART2_TARGET - program[0];

            return (noun * 100) + verb;
        }

        program.copy_from_slice(initial_program);
    }

    panic!("Answer not found for noun-verb pairs in range 0..100")
}

fn part2_bs(initial_program: &[u32]) -> u32 {
    let mut program = initial_program.to_vec();

    let mut current = 5000;
    let mut next_jump_weight = 2500;

    loop {
        program[1]  = current / 100;
        program[2] = current % 100;

        run_intcode(&mut program);

        let result = program[0];
        if result == PART2_TARGET {
            return current;
        } else if result < PART2_TARGET {
            current += next_jump_weight;
        } else {
            current -= next_jump_weight;
        }

        if next_jump_weight > 1 {
            next_jump_weight /= 2;
        }

        program.copy_from_slice(initial_program);
    }
}

fn part2_reduce(initial_program: &[u32]) -> u32 {
    let mut program1 = initial_program.to_vec();
    let mut program2 = initial_program.to_vec();
    let mut program3 = initial_program.to_vec();

    program1[1] = 25;
    program1[2] = 11;
    program2[1] = 26;
    program2[2] = 11;
    program3[1] = 25;
    program3[2] = 12;

    run_intcode(&mut program1);
    run_intcode(&mut program2);
    run_intcode(&mut program3);

    let res1 = program1[0];
    let res2 = program2[0];
    let res3 = program3[0];

    let x = res2 - res1;
    let y = res3 - res1;
    let n = res1 - ((25 * x) + (11 * y));

    let nx = (PART2_TARGET - n) / x;
    let ny = (PART2_TARGET - n - (nx * x)) / y;

    (nx * 100) + ny
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
        let expected = vec![2,0,0,0,99];
        run_intcode(&mut program);
        assert_eq!(program, expected);

        let mut program = vec![2,3,0,3,99];
        let expected = vec![2,3,0,6,99];
        run_intcode(&mut program);
        assert_eq!(program, expected);

        let mut program = vec![2,4,4,5,99,0];
        let expected = vec![2,4,4,5,99,9801];
        run_intcode(&mut program);
        assert_eq!(program, expected);

        let mut program = vec![1,1,1,4,99,5,6,0,99];
        let expected = vec![30,1,1,4,2,5,6,0,99];
        run_intcode(&mut program);
        assert_eq!(program, expected);
    }
}