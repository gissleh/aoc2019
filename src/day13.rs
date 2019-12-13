use common::aoc::{load_input, run_many, print_time, print_result};
use common::grid::{BigGrid, Grid};
use common::intcode::VM;
use num::clamp;
use std::io::Write;
use term::stdout;

fn main() {
    let input = load_input("day13");

    let (game, dur_parse) = run_many(100, || Game::new(&input.trim_end_matches("\n")));
    let ((game, res_part1), dur_part1) = run_many(10, || part1(game.clone()));
    let (res_part2, dur_part2) = run_many(1, || part2(game.clone()));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

fn part1(mut game: Game) -> (Game, usize) {
    game.setup();

    let count_blocks = game.block_count;
    (game, count_blocks)
}

fn part2(mut game: Game) -> i64 {
    game.reset();

    while game.block_count > 0 {
        game.run();
    }

    game.score
}

#[derive(Clone)]
struct Game {
    vm: VM,
    game_grid: Grid<u8>,
    block_count: usize,
    score: i64,
    ball_pos: (isize, isize),
    paddle_pos: (isize, isize),
}

impl Game {
    fn setup(&mut self) {
        self.vm.run();

        let mut source_grid = BigGrid::new(1024, 1024, 0u8);

        let output = self.vm.read_output();
        for i in 0..(output.len() / 3) {
            let i = i * 3;
            let x = output[i] as isize;
            let y = output[i + 1] as isize;
            let v = output[i + 2] as u8;

            let cell = source_grid.get_mut(x, y);
            if *cell == 2 && v != 2 {
                self.block_count -= 1;
            } else if v == 2 && *cell != 2 {
                self.block_count += 1;
            }

            if v == 4 {
                self.ball_pos = (x, y);
            }

            if v == 3 {
                self.paddle_pos = (x, y);
            }

            *cell = v;
        }

        self.game_grid = source_grid.to_exact();
    }

    fn reset(&mut self) {
        self.vm.reset();
        self.vm.set_memory(0, 2);
    }

    fn run(&mut self) {
        let (px, _) = self.paddle_pos;
        let (bx, _) = self.ball_pos;

        self.vm.push_input(clamp((bx - px) as i64, -1, 1));
        self.vm.run();

        let output = self.vm.read_output();
        for i in 0..(output.len() / 3) {
            let i = i * 3;
            let x = output[i] as isize;
            let y = output[i + 1] as isize;

            if x == -1 && y == 0 {
                self.score = output[i + 2];
                continue;
            }

            let v = output[i + 2] as u8;
            let cell = self.game_grid.get_mut(x, y);
            if *cell == 2 && v != 2 {
                self.block_count -= 1;
            } else if v == 2 && *cell != 2 {
                self.block_count += 1;
            }

            if v == 4 {
                self.ball_pos = (x, y);
            }

            if v == 3 {
                self.paddle_pos = (x, y);
            }

            *cell = v;
        }
    }

    #[allow(dead_code)]
    fn visualize(&self) -> String {
        let mut s = String::with_capacity(20+20*42);
        for y in 0..=20 {
            for x in 0..=42 {
                match self.game_grid.get(x, y) {
                    0 => s.push(' '),
                    1 => s.push('#'),
                    2 => s.push('%'),
                    3 => s.push('='),
                    4 => s.push('¤'),
                    _ => s.push(' '),
                };
            }

            s.push('\n');
        }

        s
    }

    fn new(input: &str) -> Game {
        Game{
            vm: VM::parse(input),
            game_grid: Grid::empty(0),
            block_count: 0,
            ball_pos: (0, 0),
            paddle_pos: (0, 0),
            score: 0,
        }
    }
}