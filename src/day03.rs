use common::aoc::{load_input, run_many, print_time, print_result};

fn main() {
    let input = load_input("day03");


    let (wires, dur_parse) = run_many(1000, || parse_input(&input));
    let w1 = &wires[0];
    let w2 = &wires[1];

    let (res_part1, dur_part1) = run_many(1000, || w1.closest_intersection(&w2));

    print_result("P1", res_part1.unwrap_or_default());

    print_time("Parse", dur_parse);
    print_time("P1", dur_part1);
}

fn parse_input(input: &str) -> Vec<Wire> {
    let mut results = Vec::with_capacity(2);

    for line in input.lines() {
        if line.len() < 1 {
            continue;
        }

        results.push(Wire::parse(line));
    }

    results
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Wire {
    points: Vec<Point>,
}

impl Wire {
    fn closest_intersection(&self, other: &Wire) -> Option<i32> {
        let mut winner: Option<i32> = None;

        let mut prev = &self.points[0];
        for point in self.points.iter().skip(1) {
            let line = Line(prev, point);
            prev = point;

            let mut other_prev = &other.points[0];
            for other_point in other.points.iter().skip(1) {
                let other_line = Line(other_prev, other_point);
                other_prev = other_point;

                if let Some(point) = line.intersects(&other_line) {
                    if point.x == 0 && point.y == 0 {
                        continue;
                    }

                    let dist = point.x.abs() + point.y.abs();

                    if let Some(winner_dist) = winner {
                        if dist < winner_dist {
                            winner = Some(dist);
                        }
                    } else {
                        winner = Some(dist);
                    }
                }
            }
        }

        winner
    }

    fn parse(str: &str) -> Wire {
        let mut wire = Wire {
            points: Vec::with_capacity(str.len() / 3),
        };

        let mut current = Point::new(0, 0);
        wire.points.push(current.clone());

        for token in str.split(',') {
            let dir: char = token.chars().next().unwrap();
            let len_str: &str = &token[1..];
            let len: i32 = len_str.parse().unwrap();

            current = current.next(dir, len);
            wire.points.push(current.clone());
        }

        wire
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn next(&self, dir: char, len: i32) -> Point {
        match dir {
            'L' => Point{x: self.x - len, y: self.y},
            'R' => Point{x: self.x + len, y: self.y},
            'U' => Point{x: self.x, y: self.y - len},
            'D' => Point{x: self.x, y: self.y + len},
            _ => panic!("invalid direction {}", dir),
        }
    }

    fn new(x: i32, y: i32) -> Point {
        Point{x, y}
    }
}

#[derive(Debug)]
struct Line<'a> (&'a Point, &'a Point);

impl<'a> Line<'a> {
    fn len(&self) -> i32 {
        let Line(a, b) = *self;
        
        (b.x - a.x).abs() + (b.y - a.y).abs()
    }

    fn intersects(&self, other: &Line) -> Option<Point> {
        let Line(mut a1, mut a2) = *self;
        let Line(mut b1, mut b2) = *other;

        // Parallel lines
        if a1.x == a2.x && b1.x == b2.x {
            return None
        }
        if a1.y == a2.y && b1.y == b2.y {
            return None
        }

        if a1.x != a2.x {
            let t1 = a1;
            let t2 = a2;
            a1 = b1;
            a2 = b2;
            b1 = t1;
            b2 = t2;
        }

        let (at, ab) = if a1.y < a2.y { (a1, a2) } else { (a2, a1) };
        let (bl, br) = if b1.x < b2.x { (b1, b2) } else { (b2, b1) };

        if bl.y >= at.y && bl.y <= ab.y && at.x >= bl.x && at.x <= br.x {
            Some(Point{x: at.x, y: bl.y})
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_parse() {
        let actual = Wire::parse("U32,R64,D48,L96,U20");
        let expected = Wire{
            points: vec!{
                Point{x: 0, y: 0},
                Point{x: 0, y: -32},
                Point{x: 64, y: -32},
                Point{x: 64, y: 16},
                Point{x: -32, y: 16},
                Point{x: -32, y: -4},
            }
        };

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_wire_closest_intersection() {
        let w1 = Wire::parse("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let w2 = Wire::parse("U62,R66,U55,R34,D71,R55,D58,R83");
        let w3 = Wire::parse("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let w4 = Wire::parse("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");

        assert_eq!(w1.closest_intersection(&w2), Some(159));
        assert_eq!(w3.closest_intersection(&w4), Some(135));
    }

    #[test]
    fn test_line_intersect() {
        let a = Point{x: 0, y: 0};
        let b = Point{x: 32, y: 0};
        let c = Point{x: 16, y: 4};
        let d = Point{x: 16, y: -4};
        let e = Point{x: 16, y: -16};
        let f = Point{x: 16, y: -6};
        let g = Point{x: 48, y: 0};
        let h = Point{x: 56, y: 0};
        let i = Point{x: 48, y: 16};
        let j = Point{x: 56, y: 16};

        let ab = Line(&a, &b);
        let cd = Line(&c, &d);
        let ef = Line(&e, &f);
        let gh = Line(&g, &h);
        let gi = Line(&g, &i);
        let ji = Line(&j, &i);

        assert_eq!(ab.intersects(&cd), Some(Point{x: 16, y: 0}));
        assert_eq!(gh.intersects(&gi), Some(g.clone()));
        assert_eq!(gi.intersects(&ji), Some(i.clone()));
        assert_eq!(ab.intersects(&ef), None);
        assert_eq!(ab.intersects(&gh), None);
    }
}