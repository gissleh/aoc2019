use common::aoc::{load_input, run_many, print_time, print_result};
use common::intcode::VM;
use num::clamp;

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
    block_count: usize,
    score: i64,
    ball_pos: (i64, i64),
    paddle_pos: (i64, i64),
}

impl Game {
    fn setup(&mut self) {
        self.vm.run();

        let output = self.vm.read_output();
        for i in 0..(output.len() / 3) {
            let i = i * 3;

            match output[i + 2]  {
                2 => self.block_count += 1,
                3 => self.paddle_pos = (output[i], output[i + 1]),
                4 => self.ball_pos = (output[i], output[i + 1]),
                _ => {}
            };
        }
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
            let x = output[i];
            let y = output[i + 1];

            if x == -1 && y == 0 {
                if self.score != output[i + 2] {
                    self.block_count -= 1;
                }

                self.score = output[i + 2];
                continue;
            }

            match output[i + 2] {
                3 => self.paddle_pos = (x, y),
                4 => self.ball_pos = (x, y),
                _ => {}
            };
        }
    }

    fn new(input: &str) -> Game {
        Game{
            vm: VM::parse(input),
            block_count: 0,
            ball_pos: (0, 0),
            paddle_pos: (0, 0),
            score: 0,
        }
    }
}