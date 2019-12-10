use common::aoc::{load_input, run_many, print_time, print_result};
use common::math::{Grid, grid_direction, direction_atan2, grid_direction_len, cmp_f64};

fn main() {
    let input = load_input("day10");

    let (field, dur_parse) = run_many(1000, || AsteroidField::parse(&input));
    let ((x, y, res_part1), dur_part1) = run_many(100, || field.find_location());
    let ((x2, y2), dur_part2) = run_many(1000, || field.destroy_asteroids(x, y, 200));

    print_result("P1", res_part1);
    print_result("P2", format!("{},{}", x2, y2));

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

#[derive(Clone)]
struct AsteroidField {
    width: usize,
    height: usize,
    asteroids: Grid<char>,
}

impl AsteroidField {
    fn find_location(&self) -> (isize, isize, u32) {
        let mut best_score = 0;
        let mut best_x = 0;
        let mut best_y = 0;
        let mut observations = Grid::new(
            self.width * 2, self.height * 2,
            self.width as isize, self.height as isize,
            0,
        );

        let asteroids = self.index_asteroids();

        for (i, (x, y)) in asteroids.iter().cloned().enumerate() {
            let mut score = 0;

            for (j, (x2, y2)) in asteroids.iter().cloned().enumerate() {
                if i == j {
                    continue;
                }

                let (dx, dy) = grid_direction(x, y, x2, y2);
                if observations.get(dx, dy) == 0 {
                    observations.set(dx, dy, 1);
                    score += 1;
                }
            }

            if score > best_score {
                best_score = score;
                best_x = x;
                best_y = y;
            }

            observations.clear();
        }

        (best_x, best_y, best_score)
    }

    fn destroy_asteroids(&self, x: isize, y: isize, bet_number: isize) -> (isize, isize) {
        let asteroids = self.index_asteroids();
        let mut directions: Vec<(usize, (isize, isize, isize))> = asteroids.iter().enumerate()
            .map(|(i, (x2, y2))| (i, grid_direction_len(x, y, *x2, *y2)))
            .collect();
        let mut destroyed = vec![false; directions.len()];

        let atan2s: Vec<f64> = directions.iter().map(|(_, (dx, dy, _))| direction_atan2(*dx, *dy)).collect();
        directions.sort_by(|(i, (adx, ady, am)), (j, (bdx, bdy, bm))| {
            if adx == bdx && ady == bdy {
                am.cmp(bm)
            } else {
                cmp_f64(atan2s[*i], atan2s[*j])
            }
        });

        let mut number = 0;
        let mut prev_direction: (isize, isize) = (-999, -999);

        loop {
            for (i, (dx, dy, _)) in directions.iter() {
                if destroyed[*i] {
                    continue;
                }
                if prev_direction == (*dx, *dy) {
                    continue;
                }

                number += 1;
                destroyed[*i] = true;
                prev_direction = (*dx, *dy);

                if number == bet_number {
                    return asteroids[*i];
                }
            }
        }
    }

    fn index_asteroids(&self) -> Vec<(isize, isize)> {
        let mut result: Vec<(isize, isize)> = Vec::with_capacity(self.width);

        for x in 0..self.width as isize {
            for y in 0..self.height as isize {
                if self.asteroids.get(x, y) == '#' {
                    result.push((x, y))
                }
            }
        }

        result
    }

    fn parse(str: &str) -> AsteroidField {
        let data: Vec<char> = str.chars().filter(|c| *c != '\n' && *c != ' ').collect();
        let width = str.lines().find(|_| true).unwrap().len();
        let height = data.len() / width;

        let mut grid = Grid::new(width, height, 0, 0, '.');
        grid.set_data(&data);

        return AsteroidField{
            width, height,
            asteroids: grid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####\n";
    const TEST_INPUT2: &str = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##\n";

    #[test]
    fn test_part1() {
        let field1 = AsteroidField::parse(TEST_INPUT1);
        let field2 = AsteroidField::parse(TEST_INPUT2);

        assert_eq!(field1.find_location(), (5, 8, 33));
        assert_eq!(field2.find_location(), (11,13, 210));
    }

    #[test]
    fn test_part2() {
        let field2 = AsteroidField::parse(TEST_INPUT2);

        assert_eq!(field2.destroy_asteroids(11, 13, 1), (11, 12));
        assert_eq!(field2.destroy_asteroids(11, 13, 50), (16, 9));
        assert_eq!(field2.destroy_asteroids(11, 13, 200), (8, 2));
        assert_eq!(field2.destroy_asteroids(11, 13, 299), (11, 1));
    }
}