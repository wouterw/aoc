use std::{collections::HashMap, fmt};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1), // UP-LEFT
    (-1, 0),  // UP
    (-1, 1),  // UP-RIGHT
    (0, 1),   // RIGHT
    (1, 1),   // DOWN-RIGHT
    (1, 0),   // DOWN
    (1, -1),  // DOWN-LEFT
    (0, -1),  // LEFT
];

type Schematic = Vec<Vec<Element>>;

enum Element {
    Number(usize),
    Symbol(char),
    Dot,
}

impl fmt::Debug for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Element::Number(n) => write!(f, "{}", n),
            Element::Symbol(c) => write!(f, "{}", c),
            Element::Dot => write!(f, "."),
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn parse(input: &str) -> Schematic {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '0'..='9' => Element::Number(c.to_digit(10).unwrap() as usize),
                    '.' => Element::Dot,
                    _ => Element::Symbol(c),
                })
                .collect()
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    let schematic = parse(input);

    println!("{:?}", schematic);

    let h = schematic.len();
    let l: usize = schematic[0].len();

    println!("bounds: {}x{}", h, l);

    let mut parts: Vec<usize> = Vec::new();

    let mut digits: Vec<usize> = Vec::new();
    let mut touched: bool = false;

    for i in 0..h {
        for j in 0..l {
            if let Element::Number(n) = schematic[i][j] {
                digits.push(n);

                for (di, dj) in DIRECTIONS.iter() {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;

                    if ni >= 0 && ni < h as isize && nj >= 0 && nj < l as isize {
                        if let Element::Symbol(_) = schematic[ni as usize][nj as usize] {
                            touched = true;
                        }
                    }
                }
            } else {
                if touched {
                    let number: usize = digits.iter().fold(0, |acc, d| acc * 10 + d);
                    parts.push(number);
                }

                touched = false;
                digits.clear();
            }
        }
    }

    println!("{:?}", parts);

    parts.iter().sum()
}

fn part_two(input: &str) -> usize {
    let schematic = parse(input);

    let h = schematic.len();
    let l: usize = schematic[0].len();

    let mut hitbox: HashMap<(usize, usize), usize> = HashMap::new();

    for i in 0..h {
        let mut digits: Vec<usize> = Vec::new();

        for j in 0..l {
            if let Element::Number(n) = schematic[i][j] {
                digits.push(n);

                let mut clear = false;

                if j + 1 >= l {
                    clear = true;
                } else {
                    match schematic[i][j + 1] {
                        Element::Number(_) => (),
                        Element::Symbol(_) | Element::Dot => clear = true,
                    }
                }

                if clear {
                    let number: usize = digits.iter().fold(0, |acc, d| acc * 10 + d);

                    for (ii, _) in digits.iter().enumerate() {
                        hitbox.insert((i, j - ii), number);
                    }

                    digits.clear();
                }
            }
        }
    }

    let mut gears: Vec<(usize, usize)> = Vec::new();

    for i in 0..h {
        for j in 0..l {
            if let Element::Symbol('*') = schematic[i][j] {
                let mut neighbouring_parts: HashMap<(usize, usize), usize> = HashMap::new();

                for (di, dj) in DIRECTIONS.iter() {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;

                    if ni >= 0 && ni < h as isize && nj >= 0 && nj < l as isize {
                        if let Element::Number(_) = schematic[ni as usize][nj as usize] {
                            let number = hitbox.get(&(ni as usize, nj as usize)).unwrap();
                            neighbouring_parts.insert((ni as usize, nj as usize), *number);
                        }
                    }
                }

                let p = neighbouring_parts.clone();
                let pts: &mut Vec<_> = &mut p.keys().collect();

                pts.sort_by(|a, b| {
                    if a.0 == b.0 {
                        a.1.cmp(&b.1)
                    } else {
                        a.0.cmp(&b.0)
                    }
                });

                println!("{:?}", pts);

                let mut to_remove = Vec::new();

                for (a, b) in pts.iter().zip(pts.iter().skip(1)) {
                    if a.0 == b.0 && a.1 + 1 == b.1 {
                        to_remove.push(*b);
                    }
                }

                for a in to_remove {
                    neighbouring_parts.remove(&a);
                }

                println!("{:?}", neighbouring_parts);

                if neighbouring_parts.values().len() == 2 {
                    let x: Vec<_> = neighbouring_parts.iter().collect();
                    gears.push((*x[0].1, *x[1].1));
                }

                neighbouring_parts.clear()
            }
        }
    }

    println!("{:?}", gears);

    gears.iter().map(|(a, b)| a * b).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("test.txt");
        assert_eq!(part_one(input), 4361);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("test.txt");
        assert_eq!(part_two(input), 467835);
    }
}
