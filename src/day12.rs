use common::aoc::{load_input, run_many, print_time, print_result, run_once};
use num::abs;
use std::ops::{Add, Neg, AddAssign, SubAssign};
use std::collections::{HashSet, HashMap};

fn main() {
    let input = load_input("day12");

    let (simulation, dur_parse) = run_many(10000, || Simulation::parse(&input));
    let (res_part1, dur_part1) = run_many(10000, || part1(simulation.clone(), 1000));
    let (res_part2, dur_part2) = run_once(|| part2(simulation.clone()));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

fn part1(mut simulation: Simulation, count: usize) -> i32 {
    for _ in 0..count {
        simulation.simulate_step();
    }

    simulation.total_energy()
}

fn part2(mut simulation: Simulation) -> usize {
    let mut sets: Vec<HashMap<(Point, Point), usize>> = vec![HashMap::with_capacity(3068208); simulation.moons.len()];
    let mut cycle_start: Vec<usize> = vec![0; simulation.moons.len()];
    let mut cycle_lengths: Vec<usize> = vec![0; simulation.moons.len()];
    let mut remaining = 4;
    let mut step_index = 0;

    while remaining > 0 {
        for (i, moon) in simulation.moons.iter().enumerate() {
            if cycle_lengths[i] != 0 {
                continue
            }

            let state = (moon.position, moon.velocity);

            if sets[i].get(&state).is_some() {
                cycle_start[i] = sets[i][&state];
                cycle_lengths[i] = 1 + step_index - sets[i][&state];

                println!("C {} {} {}", i, cycle_start[i], cycle_lengths[i]);
                remaining -= 1;

                sets[i].clear();
            } else {
                sets[i].insert(state, step_index);
            }
        }

        simulation.simulate_step();

        step_index += 1;
    }

    let step_size = 1; // cycle_lengths.iter().min().cloned().unwrap();

    let mut pos = cycle_lengths.iter().max().cloned().unwrap();;
    while pos < 10000 {
        let first = cycle_start[0] + ((pos - cycle_start[0]) % cycle_lengths[0]);
        let mut failed = false;

        print!("P {} {}", pos, first);
        for i in 1..cycle_lengths.len() {
            let cycle_pos = cycle_start[i] + ((pos - cycle_start[i]) % cycle_lengths[i]);

            print!(" {}", cycle_pos);

            if first != cycle_pos {
                failed = true;
                break;
            }
        }
        println!();

        if !failed {
            break;
        }

        //C 3 47 522
        //C 2 7 616
        //C 1 201 522
        //C 0 0 924

        pos += step_size;
    }

    pos
}

#[derive(Clone)]
struct Simulation {
    moons: Vec<Moon>,
}

impl Simulation {
    fn simulate_step(&mut self) {
        let moon_count = self.moons.len();

        for i in 0..moon_count {
            for j in (i+1)..moon_count {
                let velocity = self.moons[i].simulate_gravity(&self.moons[j]);

                self.moons[i].velocity += velocity;
                self.moons[j].velocity -= velocity;
            }
        }

        for moon in self.moons.iter_mut() {
            moon.position += moon.velocity;
        }
    }

    fn total_energy(&self) -> i32 {
        let mut total = 0;

        for moon in self.moons.iter() {
            total += moon.kin() * moon.pot();
        }

        total
    }

    fn parse(str: &str) -> Simulation {
        Simulation {
            moons: str.lines().filter(|l| l.len() > 1).map(|l| Moon::parse(l)).collect(),
        }
    }
}

fn cycle_pos(mut pos: usize, start: usize, length: usize) -> usize {
    if pos < start {
        pos += length;
    }

    start + ((pos - start) % length)
}

#[derive(Clone)]
struct Moon {
    position: Point,
    velocity: Point,
}

impl Moon {
    fn pot(&self) -> i32 {
        let Point(x, y, z) = self.position;

        abs(x) + abs(y) + abs(z)
    }

    fn kin(&self) -> i32 {
        let Point(x, y, z) = self.velocity;

        abs(x) + abs(y) + abs(z)
    }

    fn simulate_gravity(&self, other: &Moon) -> Point {
        let Point(sx, sy, sz) = self.position;
        let Point(ox, oy, oz) = other.position;

        let mut velocity = Point(0, 0, 0);
        let Point(vx, vy, vz) = &mut velocity;

        if sx < ox {
            *vx += 1;
        } else if sx > ox {
            *vx -= 1;
        }
        if sy < oy {
            *vy += 1;
        } else if sy > oy {
            *vy -= 1;
        }
        if sz < oz {
            *vz += 1;
        } else if sz > oz {
            *vz -= 1;
        }

        velocity
    }

    fn parse(line: &str) -> Moon {
        let mut arr = [0; 3];
        let mut parsed = 0;
        let mut sign = 1;
        let mut idx = 0;

        for ch in line.chars() {
            match ch {
                '0'..='9' => {
                    parsed *= 10;
                    parsed += ((ch as u8) - ('0' as u8)) as i32;
                }
                '-' => {
                    sign = -1;
                }
                ',' => {
                    arr[idx] = parsed * sign;
                    sign = 1;
                    parsed = 0;
                    idx += 1;
                },
                _ => {}
            }
        }

        Moon {
            position: Point(arr[0], arr[1], parsed * sign),
            velocity: Point(0, 0, 0),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, std::fmt::Debug)]
struct Point (i32, i32, i32);

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let Point(sx, sy, sz) = self;
        let Point(ox, oy, oz) = rhs;

        Point(sx+ox, sy+oy, sz+oz)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        let Point(sx, sy, sz) = self;
        let Point(ox, oy, oz) = rhs;

        *sx += ox;
        *sy += oy;
        *sz += oz;
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        let Point(sx, sy, sz) = self;
        let Point(ox, oy, oz) = rhs;

        *sx -= ox;
        *sy -= oy;
        *sz -= oz;
    }
}


impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        let Point(x, y, z) = self;

        Point(-x, -y, -z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moon_parse() {
        let moon = Moon::parse("<x=2, y=-10, z=-7>");
        let moon2 = Moon::parse("<x=2432, y=1110, z=-17>");

        assert_eq!(moon.position, Point(2, -10, -7));
        assert_eq!(moon2.position, Point(2432, 1110, -17));
    }

    #[test]
    fn test_moon_simulate_gravity() {
        let moon = Moon::parse("<x=-1, y=  5, z= 1>");
        let moon2 = Moon::parse("<x=1, y=  5, z= -1>");

        let velocity = moon.simulate_gravity(&moon2);

        assert_eq!(velocity, Point(1, 0, -1));
    }

    const TEST_DATA: &str = "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>";
    #[test]
    fn test_part1() {
        let mut sim = Simulation::parse(TEST_DATA);

        for i in 0..100 {
            println!("Step {}", i);
            for moon in sim.moons.iter() {
                println!("pos={:?} vel={:?} pot={} kin={}", moon.position, moon.velocity, moon.pot(), moon.kin());
            }

            sim.simulate_step();
        }
        println!("Step 100");
        for moon in sim.moons.iter() {
            println!("pos={:?} vel={:?} pot={} kin={}", moon.position, moon.velocity, moon.pot(), moon.kin());
        }

        assert_eq!(sim.total_energy(), 1940);
    }

    const TEST_DATA_EASY: &str = "<x= -1, y=  0, z=  2>\n<x=  2, y=-10, z= -7>\n<x=  4, y= -8, z=  8>\n<x=  3, y=  5, z= -1>\n";

    #[test]
    fn test_cycle_pos() {
        assert_eq!(cycle_pos(0, 2, 5), 5);
        assert_eq!(cycle_pos(0, 201, 522), 522 - 201)
    }

    #[test]
    fn test_part2() {
        let amount = part2(Simulation::parse(TEST_DATA_EASY));

        assert_eq!(amount, 2772);
    }
}