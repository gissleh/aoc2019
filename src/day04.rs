use common::aoc::{load_input, run_many, print_time, print_result};

fn main() {
    let input = load_input("day04");

    let ((start, end), dur_parse) = run_many(10000, || parse_input(&input));
    let (res_part1_bf, dur_part1_bf) = run_many(100, || part1_bf(start, end));
    let (res_part2_bf, dur_part2_bf) = run_many(100, || part2_bf(start, end));

    print_result("P1: Brute Force", res_part1_bf);
    print_result("P2: Brute Force", res_part2_bf);

    print_time("Parse", dur_parse);
    print_time("P1: Brute Force", dur_part1_bf);
    print_time("P2: Brute Force", dur_part2_bf);
}

fn part1_bf(start: u32, end: u32) -> u32 {
    let mut count = 0;

    for i in start..=end {
        if password_valid(i) {
            count += 1;
        }
    }

    count
}

fn part2_bf(start: u32, end: u32) -> u32 {
    let mut count = 0;

    for i in start..=end {
        if password_valid_p2(i) {
            count += 1;
        }
    }

    count
}

fn parse_input(input: &str) -> (u32, u32) {
    let mut iter = input.trim_end().split('-');
    let a: u32 = iter.next().unwrap().parse().unwrap();
    let b: u32 = iter.next().unwrap().parse().unwrap();

    (a, b)
}

fn password_valid(n: u32) -> bool {
    let mut has_double = false;
    let mut prev_digit = n % 10;
    let mut n = n / 10;

    loop {
        let curr_digit = n % 10;
        if curr_digit > prev_digit {
            return false;
        } else if curr_digit == prev_digit {
            has_double = true;
        }

        prev_digit = curr_digit;
        n /= 10;

        if n == 0 {
            break;
        }
    }

    has_double
}

fn password_valid_p2(n: u32) -> bool {
    let mut has_double = 10;
    let mut bad_double = 10;
    let mut prev_double = false;
    let mut prev_digit = n % 10;
    let mut n = n / 10;

    loop {
        let curr_digit = n % 10;
        if curr_digit > prev_digit {
            return false;
        } else if curr_digit == prev_digit {
            if prev_double && curr_digit == has_double {
                bad_double = curr_digit;
            } else if has_double == 10 || has_double == bad_double {
                has_double = curr_digit;
            }

            prev_double = true;
        } else {
            prev_double = false;
        }

        prev_digit = curr_digit;
        n /= 10;

        if n == 0 {
            break;
        }
    }

    has_double != 10 && has_double != bad_double
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_valid() {
        assert_eq!(password_valid(123456), false);
        assert_eq!(password_valid(123556), true);
        assert_eq!(password_valid(122556), true);
        assert_eq!(password_valid(111111), true);
        assert_eq!(password_valid(223450), false);
        assert_eq!(password_valid(123789), false);
    }

    #[test]
    fn test_password_valid_p2() {
        assert_eq!(password_valid_p2(123456), false);
        assert_eq!(password_valid_p2(123556), true);
        assert_eq!(password_valid_p2(122556), true);
        assert_eq!(password_valid_p2(111111), false);
        assert_eq!(password_valid_p2(223450), false);
        assert_eq!(password_valid_p2(123789), false);
        assert_eq!(password_valid_p2(112233), true);
        assert_eq!(password_valid_p2(123444), false);
        assert_eq!(password_valid_p2(111122), true);
        assert_eq!(password_valid_p2(111455), true);
        assert_eq!(password_valid_p2(114555), true);
        assert_eq!(password_valid_p2(112345), true);
    }
}