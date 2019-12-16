use common::aoc::{load_input, run_many, print_time, print_result, run_once};
use num::abs;

fn main() {
    let input = load_input("day16").repeat(16);

    let (numbers, dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_once(|| part1(&numbers));

    print_result("P1", res_part1);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
}

const PATTERN: [i64; 4] = [0, 1, 0, -1];

fn parse_input(input: &str) -> Vec<i64> {
    input.chars()
        .filter(|ch| *ch >= '0' && *ch <= '9')
        .map(|ch| (ch as u8 - '0' as u8) as i64)
        .collect()
}

fn part1(numbers: &[i64]) -> i64 {
    let mut prev = numbers.to_vec();
    let mut curr = vec![0; numbers.len()];

    for _ in 0..100 {
        handle_signal(numbers.len(), &mut prev, &mut curr);
    }

    let mut result = 0;
    for n in curr.iter().take(8) {
        result *= 10;
        result += *n;
    }

    result
}

fn handle_signal(len: usize, prev: &mut Vec<i64>, curr: &mut Vec<i64>) {
    for i in 0..len {
        let pat_size = i + 1;
        let mut sum = 0;

        for j in i..len {
            let pat_index = ((j + 1) / pat_size) % PATTERN.len();

            sum += PATTERN[pat_index] * prev[j];
        }

        curr[i] = abs(sum) % 10;
    }

    prev.copy_from_slice(&curr);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input("80871224585914546619083218645595")), 24176176);
        assert_eq!(part1(&parse_input("19617804207202209144916044189917")), 73745418);
        assert_eq!(part1(&parse_input("69317163492948606335995924319873")), 52432133);
    }

    #[test]
    fn test_part1_repeat() {
        let mut input1 = parse_input(&"12345678".repeat(1));
        let mut input2 = parse_input(&"12345678".repeat(2));
        let mut input3 = parse_input(&"12345678".repeat(3));
        let mut input4 = parse_input(&"12345678".repeat(4));

        let mut curr1 = input1.clone();
        let mut curr2 = input2.clone();
        let mut curr3 = input3.clone();
        let mut curr4 = input4.clone();

        println!("{:?}", curr1);
        println!("{:?}", curr2);
        println!("{:?}", curr3);
        println!("{:?}", curr4);

        handle_signal(input1.len(), &mut input1, &mut curr1);
        handle_signal(input2.len(), &mut input2, &mut curr2);
        handle_signal(input3.len(), &mut input3, &mut curr3);
        handle_signal(input4.len(), &mut input4, &mut curr4);

        println!("{:?}", curr1);
        println!("{:?}", curr2);
        println!("{:?}", curr3);
        println!("{:?}", curr4);

        handle_signal(input1.len(), &mut input1, &mut curr1);
        handle_signal(input2.len(), &mut input2, &mut curr2);
        handle_signal(input3.len(), &mut input3, &mut curr3);
        handle_signal(input4.len(), &mut input4, &mut curr4);

        println!("{:?}", curr1);
        println!("{:?}", curr2);
        println!("{:?}", curr3);
        println!("{:?}", curr4);

        handle_signal(input1.len(), &mut input1, &mut curr1);
        handle_signal(input2.len(), &mut input2, &mut curr2);
        handle_signal(input3.len(), &mut input3, &mut curr3);
        handle_signal(input4.len(), &mut input4, &mut curr4);

        println!("{:?}", curr1);
        println!("{:?}", curr2);
        println!("{:?}", curr3);
        println!("{:?}", curr4);

        handle_signal(input1.len(), &mut input1, &mut curr1);
        handle_signal(input2.len(), &mut input2, &mut curr2);
        handle_signal(input3.len(), &mut input3, &mut curr3);
        handle_signal(input4.len(), &mut input4, &mut curr4);

        println!("{:?}", curr1);
        println!("{:?}", curr2);
        println!("{:?}", curr3);
        println!("{:?}", curr4);

        assert!(false);
    }
}