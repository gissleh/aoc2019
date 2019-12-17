use common::aoc::{load_input, run_many, print_time, print_result, run_once};
use common::intcode::VM;
use common::grid::Grid;

const SCAFFOLD: i64 = '#' as u8 as i64;
const ROBOT_UP: char = '^';
const ROBOT_UP_I64: i64 = ROBOT_UP as u8 as i64;
const ROBOT_RIGHT: char = '>';
const ROBOT_RIGHT_I64: i64 = ROBOT_RIGHT as u8 as i64;
const ROBOT_LEFT: char = '<';
const ROBOT_LEFT_I64: i64 = ROBOT_LEFT as u8 as i64;
const ROBOT_DOWN: char = 'v';
const ROBOT_DOWN_I64: i64 = ROBOT_DOWN as u8 as i64;
const ROBOT_DIRECTIONS: [char; 4] = [ROBOT_LEFT, ROBOT_UP, ROBOT_RIGHT, ROBOT_DOWN];
const LEFT: [(isize, isize); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];
const RIGHT: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const FORWARD: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn main() {
    let input = load_input("day17");

    let (vm, dur_parse) = run_many(1000, || VM::parse(&input.trim_end_matches('\n')));
    let ((grid, res_part1), dur_part1) = run_many(1000, || part1(vm.clone()));
    let (res_part2, dur_part2) = run_once(|| part2(vm.clone(), &grid));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

fn part1(mut vm: VM) -> (Grid<char>, usize) {
    vm.run();
    let output = vm.read_output();

    let (width, _) = output.iter().enumerate().find(|(_, p)| **p == 10).unwrap();
    let height = output.iter().filter(|p| **p == 10).count() - 1;

    let mut grid = Grid::new(width, height, 0, 0, '.');
    let mut x = 0;
    let mut y = 0;

    for n in output.iter() {
        match *n {
            SCAFFOLD | ROBOT_DOWN_I64 | ROBOT_LEFT_I64 | ROBOT_RIGHT_I64 | ROBOT_UP_I64 => {
                grid.set(x, y, *n as u8 as char);
                x += 1;
            }
            10 => {
                x = 0;
                y += 1;
            }
            _ => {
                x += 1;
            }
        }
    }

    let width = width as isize;
    let height = height as isize;

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                continue;
            }

            if grid.get(x, y) == '#' {
                if grid.get(x - 1, y) == '#' && grid.get(x + 1, y) == '#' && grid.get(x, y - 1) == '#' && grid.get(x, y + 1) == '#' {
                    grid.set(x, y, 'O');
                    sum += x * y;
                }
            }
        }
    }

    (grid, sum as usize)
}

fn part2(mut vm: VM, grid: &Grid<char>) -> i64 {
    let (width, height) = grid.size();

    let mut robot_x = -1;
    let mut robot_y = -1;
    let mut robot_dir = 0;

    for y in 0..(height as isize) {
        for x in 0..(width as isize) {
            let c = grid.get(x, y);
            for (i, d) in ROBOT_DIRECTIONS.iter().enumerate() {
                if c == *d {
                    robot_x = x;
                    robot_y = y;
                    robot_dir = i;
                    break;
                }
            }

            if robot_x != -1 {
                break;
            }
        }

        if robot_x != -1 {
            break;
        }
    }

    let mut path: Vec<i64> = Vec::with_capacity(64);
    let mut traveled = 0;
    loop {
        let (forward_x, forward_y) = FORWARD[robot_dir];
        let (left_x, left_y) = LEFT[robot_dir];
        let (right_x, right_y) = RIGHT[robot_dir];
        let forward = grid.get_oob(robot_x + forward_x, robot_y + forward_y);
        let left = grid.get_oob(robot_x + left_x, robot_y + left_y);
        let right = grid.get_oob(robot_x + right_x, robot_y + right_y);

        if forward == 'O' {
            traveled += 2;
            robot_x += forward_x * 2;
            robot_y += forward_y * 2;
        } else if forward == '#' {
            traveled += 1;
            robot_x += forward_x;
            robot_y += forward_y;
        } else if right == '#' {
            if traveled > 0 {
                path.push(44);
                if traveled >= 10 {
                    path.push(48 + (traveled / 10));
                }
                path.push(48 + (traveled % 10));
            }
            if path.len() > 0 {
                path.push(44);
            }
            path.push(82);

            traveled = 0;
            robot_dir += 1;
            if robot_dir > 3 {
                robot_dir -= 4;
            }
        } else if left == '#' {
            if traveled > 0 {
                path.push(44);
                if traveled >= 10 {
                    path.push(48 + (traveled / 10));
                }
                path.push(48 + (traveled % 10));
            }
            if path.len() > 0 {
                path.push(44);
            }
            path.push(76);

            traveled = 0;

            if robot_dir == 0 {
                robot_dir = 3;
            } else {
                robot_dir -= 1;
            }
        } else {
            if traveled > 0 {
                path.push(44);
                if traveled >= 10 {
                    path.push(48 + (traveled / 10));
                }
                path.push(48 + (traveled % 10));
            }
            break
        }
    }

    let chars: String = path.iter().map(|n| (*n as u8 as char)).collect();
    let tokens: Vec<String> = chars.split(",").map(|s| String::from(s)).collect();
    let (indices, lengths) = find_patterns(&tokens).unwrap();
    
    let mut offset = 0;
    while offset < tokens.len() {
        for i in 0..3 {
            let pattern: &[String] = &tokens[indices[i]..indices[i]+lengths[i]];

            if has_pattern(&tokens, &pattern, offset) {
                if offset > 0 {
                    vm.push_input(44);
                }
                vm.push_input(65 + i as i64);

                offset += lengths[i];
                break;
            }
        }
    }
    vm.push_input(10);

    for i in 0..3 {
        for j in indices[i]..(indices[i] + lengths[i]) {
            if j > indices[i] {
                vm.push_input(44);
            }
            for ch in tokens[j].chars() {
                vm.push_input(ch as u8 as i64);
            }
        }
        vm.push_input(10);
    }

    vm.set_memory(0, 2);
    vm.push_input('n' as u8 as i64);
    vm.push_input(10);
    vm.run();

    *vm.read_output().last().unwrap()
}

fn find_patterns(arr: &[String]) -> Option<([usize; 3], [usize; 3])>  {
    for i in 3..=10 {
        for j in 3..=10 {
            for k in 3..=10 {
                for io in 0..arr.len() - (i - 1) {
                    for jo in 0..arr.len() - (j - 1) {
                        for ko in 0..arr.len() - (k - 1) {
                            if check_patterns(arr, 0, [io, jo, ko], [i, j, k]) {
                                return Some(([io, jo, ko], [i, j, k]));
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

fn check_patterns(arr: &[String], offset: usize, offsets: [usize; 3], lengths: [usize; 3]) -> bool {
    if offset < arr.len() {
        let mut success = false;
        for i in 0..3 {
            let pattern = &arr[offsets[i]..offsets[i]+lengths[i]];

            if has_pattern(arr, pattern, offset) {
                if check_patterns(arr, offset + lengths[i], offsets, lengths) {
                    success = true;
                    break;
                }
            }
        }

        success
    } else {
        true
    }
}

fn has_pattern(arr: &[String], pattern: &[String], offset: usize) -> bool {
    if offset + pattern.len() <= arr.len() {
        for i in 0..pattern.len() {
            if pattern[i] != arr[offset + i] {
                return false;
            }
        }

        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_pattern() {
        let arr = [
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
            String::from("e"),
            String::from("c"),
            String::from("d"),
            String::from("e"),
        ];

        assert!(has_pattern(&arr, &[
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ], 0));

        assert!(!has_pattern(&arr, &[
            String::from("a"),
            String::from("b"),
            String::from("c"),
        ], 1));

        assert!(has_pattern(&arr, &[
            String::from("c"),
            String::from("d"),
            String::from("e"),
        ], 2));

        assert!(has_pattern(&arr, &[
            String::from("c"),
            String::from("d"),
            String::from("e"),
        ], 5));

        assert!(!has_pattern(&arr, &[
            String::from("c"),
            String::from("d"),
            String::from("e"),
        ], 6));
    }
}
