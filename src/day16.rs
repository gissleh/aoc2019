use common::aoc::{load_input, run_many, print_time, print_result, run_once};
use num::{abs, range_step};

fn main() {
    let input = load_input("day16");

    let (numbers, dur_parse) = run_many(1000, || parse_input(&input));
    let (res_part1, dur_part1) = run_once(|| part1(&numbers));
    let (res_part2, dur_part2) = run_once(|| part2(&numbers));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
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
    let len = curr.len();

    for _ in 0..100 {
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

    let mut result = 0;
    for n in curr.iter().take(8) {
        result *= 10;
        result += *n;
    }

    result
}

fn part2(numbers: &[i64]) -> i64 {
    let huge_len = numbers.len() * 10000;
    let mut message_offset = 0;
    for n in numbers.iter().take(7) {
        message_offset *= 10;
        message_offset += *n;
    }
    let message_offset = message_offset as usize;

    let mut curr = Vec::with_capacity(huge_len - message_offset + 2);
    for i in message_offset-1..huge_len {
        curr.push(numbers[i % numbers.len()]);
    }

    for _ in 0..100 {
        let mut count = 0;
        for i in range_step(curr.len() as isize - 1, 0, -1) {
            let i = i as usize;

            let num = curr[i];
            count += num;
            curr[i] = abs(count) % 10;
        }
    }

    let mut result = 0;
    for n in curr.iter().skip(1).take(8) {
        result *= 10;
        result += *n;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input("80871224585914546619083218645595")), 24176176);
        assert_eq!(part1(&parse_input("19617804207202209144916044189917")), 73745418);
        assert_eq!(part1(&parse_input("69317163492948606335995924319873")), 52432133);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input("03036732577212944063491565474664")), 84462026);
        assert_eq!(part2(&parse_input("02935109699940807407585447034323")), 78725270);
        assert_eq!(part2(&parse_input("03081770884921959731165446850517")), 53553731);
    }
}