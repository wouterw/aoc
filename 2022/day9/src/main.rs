use std::collections::HashSet;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    let motions = parse(input);

    let rope = simulate(&motions, 2);
    println!("Part 1: {:?}", rope.visited.len());

    let rope = simulate(&motions, 10);
    println!("Part 2: {:?}", rope.visited.len());
}

fn parse(input: &str) -> Vec<Motion> {
    input.lines().filter_map(|line| line.parse().ok()).collect()
}

fn simulate(motions: &[Motion], knots: usize) -> Rope {
    let mut rope = Rope::new(knots);

    for motion in motions {
        let (steps, delta) = match motion {
            Motion::Up(steps) => (steps, (-1, 0)),
            Motion::Down(steps) => (steps, (1, 0)),
            Motion::Left(steps) => (steps, (0, -1)),
            Motion::Right(steps) => (steps, (0, 1)),
        };

        for _ in 0..*steps {
            rope.r#move(delta);
        }
    }

    rope
}

#[derive(Debug)]
enum Motion {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

struct ParseMotionError;

impl FromStr for Motion {
    type Err = ParseMotionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, steps) = s.split_once(' ').ok_or(ParseMotionError)?;
        let steps = match steps.parse() {
            Ok(i) => i,
            Err(_) => return Err(ParseMotionError),
        };

        match direction {
            "U" => Ok(Self::Up(steps)),
            "D" => Ok(Self::Down(steps)),
            "L" => Ok(Self::Left(steps)),
            "R" => Ok(Self::Right(steps)),
            _ => panic!("Invalid direction {s}"),
        }
    }
}

#[derive(Debug)]
struct Rope {
    seg: Vec<(i32, i32)>,
    visited: HashSet<(i32, i32)>,
}

impl Rope {
    fn new(knots: usize) -> Self {
        Self {
            seg: vec![(0, 0); knots],
            visited: HashSet::new(),
        }
    }

    fn r#move(&mut self, delta: (i32, i32)) {
        self.seg[0].0 += delta.0;
        self.seg[0].1 += delta.1;

        for i in 1..self.seg.len() {
            let dr = self.seg[i - 1].0 - self.seg[i].0;
            let dc = self.seg[i - 1].1 - self.seg[i].1;

            if dr.abs() > 1 || dc.abs() > 1 {
                self.seg[i].0 += dr.signum();
                self.seg[i].1 += dc.signum();
            }
        }

        self.visited.insert(self.seg[self.seg.len() - 1]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("test.txt");
        let motions = parse(input);
        let rope = simulate(&motions, 2);

        assert_eq!(rope.visited.len(), 13);
    }

    #[test]
    fn test_part_tow() {
        let input = include_str!("test.txt");
        let motions = parse(input);
        let rope = simulate(&motions, 10);

        assert_eq!(rope.visited.len(), 1);
    }
}
