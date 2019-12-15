use common::aoc::{load_input, run_many, print_time, print_result};
use num::abs;
use std::ops::{Add, Neg, AddAssign, SubAssign};
use std::collections::{BTreeMap};
use std::hash::Hash;
use num::Integer;

fn main() {
    let input = load_input("day12");

    let (simulation, dur_parse) = run_many(10000, || Simulation::parse(&input));
    let (res_part1, dur_part1) = run_many(10000, || part1(simulation.clone(), 1000));
    let (res_part2, dur_part2) = run_many(10, || part2(simulation.clone()));

    print_result("P1", res_part1);
    print_result("P2", res_part2);

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
    print_time("P2", dur_part2);
}

fn part1(mut simulation: Simulation, count: usize) -> i64 {
    for _ in 0..count {
        simulation.simulate_step();
    }

    simulation.total_energy()
}

fn part2(mut simulation: Simulation) -> usize {
    if simulation.moons.len() != 4 {
        panic!("Only 4 moons supported");
    }

    let mut sets: Vec<BTreeMap<[(i64, i64); 4], usize>> = vec![BTreeMap::new(); 3];
    let mut cycle_lengths = [0usize; 3];
    let mut remaining = 3;
    let mut step_index = 0;

    let mut velocities: Vec<Vec<i64>> = vec![Vec::with_capacity(simulation.moons.len()); 3];
    let mut positions: Vec<Vec<i64>> = vec![Vec::with_capacity(simulation.moons.len()); 3];
    let mut states = [(0, 0); 4];

    while remaining > 0 {
        for vel_axis in velocities.iter_mut() {
            vel_axis.clear();
        }
        for pos_axis in positions.iter_mut() {
            pos_axis.clear();
        }

        for moon_index in 0..4 {
            let moon = &simulation.moons[moon_index];

            let Point(px, py, pz) = moon.position;
            let Point(vx, vy, vz) = moon.velocity;

            positions[0].push(px);
            positions[1].push(py);
            positions[2].push(pz);
            velocities[0].push(vx);
            velocities[1].push(vy);
            velocities[2].push(vz);
        }

        for i in 0..3 {
            if cycle_lengths[i] != 0 {
                continue;
            }

            for j in 0..positions.len() {
                states[j] = (positions[i][j], velocities[i][j]);
            }

            if sets[i].get(&states).is_some() {
                cycle_lengths[i] = step_index - sets[i][&states];
                remaining -= 1;
            } else {
                sets[i].insert(states.clone(), step_index);
            }
        }

        simulation.simulate_step();
        step_index += 1;
    }

    let mut current: usize = cycle_lengths[0];
    for i in 1..3 {
        current = current.lcm(&cycle_lengths[i]);
    }

    current
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

    fn total_energy(&self) -> i64 {
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

#[derive(Clone)]
struct Moon {
    position: Point,
    velocity: Point,
}

impl Moon {
    fn pot(&self) -> i64 {
        let Point(x, y, z) = self.position;

        abs(x) + abs(y) + abs(z)
    }

    fn kin(&self) -> i64 {
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
        let mut arr = [0i64; 3];
        let mut parsed = 0i64;
        let mut sign = 1;
        let mut idx = 0;

        for ch in line.chars() {
            match ch {
                '0'..='9' => {
                    parsed *= 10;
                    parsed += ((ch as u8) - ('0' as u8)) as i64;
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
struct Point (i64, i64, i64);

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
    fn test_part2() {
        let amount = part2(Simulation::parse(TEST_DATA_EASY));

        assert_eq!(amount, 2772);
    }
}