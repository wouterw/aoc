use std::fmt;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn parse(input: &str) -> (Vec<Point>, Vec<usize>, Vec<usize>) {
    let mut galaxies: Vec<Point> = vec![];

    for (m, row) in input.lines().enumerate() {
        for (n, col) in row.chars().enumerate() {
            match col {
                '#' => galaxies.push(Point::new(m, n)),
                '.' => (),
                _ => panic!("Invalid character: {}", col),
            }
        }
    }

    // bounds of our universe
    let max_rows = galaxies.iter().map(|galaxy| galaxy.x).max().unwrap();
    let max_columns = galaxies.iter().map(|galaxy| galaxy.y).max().unwrap();

    let empty_space_rows = (0..max_rows)
        .filter(|m| galaxies.iter().all(|p| p.x != *m))
        .collect::<Vec<_>>();

    let empty_space_columns = (0..max_columns)
        .filter(|n| galaxies.iter().all(|p| p.y != *n))
        .collect::<Vec<_>>();

    (galaxies, empty_space_rows, empty_space_columns)
}

fn overlaps(value: &usize, a: usize, b: usize) -> bool {
    let max = a.max(b);
    let min = a.min(b);

    (min..max).contains(value)
}

fn shortest_paths_between_galaxies(input: &str, cosmic_expansion_factor: usize) -> usize {
    let (galaxies, empty_space_rows, empty_space_columns) = parse(input);

    let mut sum = 0;

    for (i, galaxy) in galaxies.iter().enumerate() {
        for other_galaxy in galaxies.iter().skip(i + 1) {
            let empty_columns_count = empty_space_columns
                .iter()
                .filter(|n| overlaps(n, other_galaxy.y, galaxy.y))
                .count();

            let empty_rows_count = empty_space_rows
                .iter()
                .filter(|m| overlaps(m, other_galaxy.x, galaxy.x))
                .count();

            let distance = galaxy.manhattan_distance(other_galaxy)
                + (empty_columns_count + empty_rows_count) * (cosmic_expansion_factor - 1);

            sum += distance;
        }
    }

    // The sum of the shortest path between all pairs of galaxies.
    sum
}

fn part_one(input: &str) -> usize {
    shortest_paths_between_galaxies(input, 2)
}

#[test]
fn test_part_one() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    assert_eq!(part_one(input), 374);
}

fn part_two(input: &str) -> usize {
    shortest_paths_between_galaxies(input, 1_000_000)
}

#[test]
fn test_part_two() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    assert_eq!(shortest_paths_between_galaxies(input, 10), 1030);
}
