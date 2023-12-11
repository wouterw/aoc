use std::{
    collections::{HashSet, VecDeque},
    ops::Add,
};

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl Tile {
    fn directions(&self) -> Vec<Point> {
        match self {
            Tile::Vertical => vec![UP, DOWN],
            Tile::Horizontal => vec![LEFT, RIGHT],
            Tile::NorthEast => vec![UP, RIGHT],
            Tile::NorthWest => vec![UP, LEFT],
            Tile::SouthWest => vec![DOWN, LEFT],
            Tile::SouthEast => vec![DOWN, RIGHT],
            Tile::Start => vec![UP, DOWN, LEFT, RIGHT],
            Tile::Ground => vec![],
        }
    }

    fn is_valid_start(&self, next_tile: &Tile, direction: &Point) -> bool {
        if !matches!(self, Tile::Start) {
            return true;
        };

        matches!(
            (next_tile, direction),
            (Tile::Horizontal, &RIGHT)
                | (Tile::Horizontal, &LEFT)
                | (Tile::Vertical, &UP)
                | (Tile::Vertical, &DOWN)
                | (Tile::NorthEast, &LEFT)
                | (Tile::NorthEast, &DOWN)
                | (Tile::NorthWest, &DOWN)
                | (Tile::NorthWest, &RIGHT)
                | (Tile::SouthEast, &LEFT)
                | (Tile::SouthEast, &UP)
                | (Tile::SouthWest, &RIGHT)
                | (Tile::SouthWest, &UP)
        )
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NorthEast,
            'J' => Tile::NorthWest,
            '7' => Tile::SouthWest,
            'F' => Tile::SouthEast,
            '.' => Tile::Ground,
            'S' => Tile::Start,
            _ => panic!("Invalid pipe: {}", value),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Point { x, y }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

const UP: Point = Point { x: 0, y: -1 };
const DOWN: Point = Point { x: 0, y: 1 };
const LEFT: Point = Point { x: -1, y: 0 };
const RIGHT: Point = Point { x: 1, y: 0 };

type Grid = Vec<Vec<Tile>>;

fn parse(input: &str) -> (Point, Grid) {
    let mut grid = Vec::new();
    let mut start = Point::new(0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();

        for (x, c) in line.chars().enumerate() {
            let tile = Tile::from(c);

            if tile == Tile::Start {
                start = Point::new(x as isize, y as isize);
            }

            row.push(tile);
        }

        grid.push(row);
    }

    (start, grid)
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_one(input));
}

fn part_one(input: &str) -> usize {
    let (start, grid) = parse(input);

    let max_y = grid.len() as isize;
    let max_x = grid[0].len() as isize;

    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue = VecDeque::new();

    let mut furthest = 0;

    queue.push_back((start, 0));

    while let Some((point, steps)) = queue.pop_front() {
        visited.insert(point);

        let tile = &grid[point.y as usize][point.x as usize];

        for dir in tile.directions() {
            let next = point + dir;

            if next.x < 0 || next.x > max_x || next.y < 0 || next.y > max_y {
                continue;
            }

            if !tile.is_valid_start(&grid[next.y as usize][next.x as usize], &dir) {
                continue;
            }

            if visited.contains(&next) {
                continue;
            }

            let next_steps = steps + 1;

            if furthest == 0 || next_steps > furthest {
                furthest = next_steps
            }

            queue.push_back((next, next_steps));
        }
    }

    println!("{:?}", visited);

    furthest
}

#[test]
fn test_part_one_a() {
    let input = ".....
.S-7.
.|.|.
.L-J.
.....";

    assert_eq!(part_one(input), 4);
}

#[test]
fn test_part_one_a_complex() {
    let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    assert_eq!(part_one(input), 4);
}

#[test]
fn test_part_one_b() {
    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    assert_eq!(part_one(input), 8);
}

#[test]
fn test_part_one_b_complex() {
    let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    assert_eq!(part_one(input), 8);
}
