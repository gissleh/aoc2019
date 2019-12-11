use common::aoc::{load_input, run_many, print_result, print_time, print_result_multiline};
use common::intcode::{VM, StepResult};
use std::collections::{HashSet, HashMap};
use common::math::Grid;

fn main() {
    let input = load_input("day11");

    let (vm, dur_parse) = run_many(10000, || VM::parse(input.trim_end_matches('\n')));
    let ((_, res_part1), dur_part1) = run_many(10, || part1(vm.clone(), 0));
    let (res_part2, dur_part2) = run_many(10, || part2(vm.clone()));

    print_result("P1", res_part1);
    print_result_multiline("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

const DIRECTIONS: [(isize, isize); 4] = [
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
];

const DEFAULT_PAINT: i64 = 0;

fn part1(mut vm: VM, starting_color: i64) -> (HashMap<(isize, isize), i64>, usize) {
    let mut paint_map: HashMap<(isize, isize), i64> = HashMap::with_capacity(128);

    let mut x = 0;
    let mut y = 0;
    let mut dir_index = 0;

    vm.push_input(starting_color);

    loop {
        let result = vm.run();
        if result == StepResult::Exit {
            break;
        }

        let output = vm.read_output();

        let color = output[0];
        let dir_change = output[1];

        dir_index = (dir_index + if dir_change == 1 { 1 } else { 3 }) % 4;

        match paint_map.get_mut(&(x, y)) {
            Some(v) => *v = color,
            None => {
                paint_map.insert((x, y), color);
            }
        }

        let (dx, dy) = DIRECTIONS[dir_index];
        x += dx;
        y += dy;

        vm.push_input(*paint_map.get(&(x, y)).unwrap_or(&DEFAULT_PAINT));
    }

    let count = paint_map.len();

    (paint_map, count)
}

fn part2(mut vm: VM) -> String {
    let (mut paint_map, _) = part1(vm, 1);

    // Find boundaries
    let mut tlx = 0;
    let mut tly = 0;
    let mut brx = 0;
    let mut bry = 0;
    for (x, y) in paint_map.keys() {
        if *x < tlx {
            tlx = *x;
        }
        if *x > brx {
            brx = *x;
        }
        if *y < tly {
            tly = *y;
        }
        if *y > bry {
            bry = *y;
        }
    }
    let width = (brx - tlx) as usize;
    let height = (bry - tly) as usize;

    let mut grid = Grid::new(width+1, height+2, -tlx, -tly, '#');

    for ((x, y), color) in paint_map.iter() {
        if *color == 0 {
            grid.set(*x, *y, '.');
        }
    }

    let mut result = String::with_capacity((width * height) + height);

    for y in 0..=(height as isize) {
        for x in 0..=(width as isize) {
            result.push(grid.get(x + tlx, y + tly));
        }

        result.push('\n');
    }

    result
}