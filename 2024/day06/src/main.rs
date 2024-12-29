use std::collections::HashMap;
use std::collections::HashSet;

type Map = Vec<Vec<char>>;
type Position = (usize, usize);
type Path = HashMap<Position, u32>;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn parse(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn find_start_position(map: &Map) -> Option<(Position, char)> {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let char = map[i][j];
            let position = (i, j);

            if char == '^' || char == 'v' || char == '<' || char == '>' {
                return Some((position, char));
            }
        }
    }

    None
}

fn print_map_with_path(map: &Map, path: &Path, start: (Position, char)) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if (i, j) == start.0 {
                print!("{}", start.1);
                continue;
            }

            match path.get(&(i, j)) {
                Some(n) => {
                    if n > &1 {
                        print!("+");
                    } else {
                        print!("{}", map[i][j]);
                    }
                }
                _ => {
                    print!("{}", map[i][j]);
                }
            }
        }

        println!();
    }
}

fn print_map(map: &Map) {
    println!();

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j]);
        }

        println!();
    }

    println!();
}

fn walk_map(map: &Map) -> (Map, Path) {
    let mut map = map.clone();

    let mut path: Path = HashMap::new();

    'outer: loop {
        // print_map(&map);

        let mut did_move = false;

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                let char = map[i][j];

                if char == '^' || char == 'v' || char == '<' || char == '>' {
                    did_move = true;

                    let pos = (i, j);
                    let number_of_visits = path.get(&pos).unwrap_or(&0) + 1;
                    path.insert(pos, number_of_visits);

                    if number_of_visits > 5 {
                        // stuck in loop
                        break 'outer;
                    }
                }

                match char {
                    '^' => {
                        if i == 0 {
                            // guard walks off the map
                            map[i][j] = '|';
                            break 'outer;
                        }

                        if i > 0 && (map[i - 1][j] == '#' || map[i - 1][j] == 'O') {
                            // turn right
                            map[i][j] = '>';
                        }

                        if i > 0 && map[i - 1][j] != '#' && map[i - 1][j] != 'O' {
                            // move forward
                            map[i][j] = '|';
                            map[i - 1][j] = '^';
                        }

                        continue 'outer;
                    }
                    'v' => {
                        if i == map.len() - 1 {
                            // guard walks off the map
                            map[i][j] = '|';
                            break 'outer;
                        }

                        if i < map.len() - 1 && (map[i + 1][j] == '#' || map[i + 1][j] == 'O') {
                            // turn right
                            map[i][j] = '<';
                        }

                        if i < map.len() - 1 && map[i + 1][j] != '#' && map[i + 1][j] != 'O' {
                            // move forward
                            map[i][j] = '|';
                            map[i + 1][j] = 'v';
                        }

                        continue 'outer;
                    }
                    '>' => {
                        if j == map[i].len() - 1 {
                            // guard walks off the map
                            map[i][j] = '-';
                            break 'outer;
                        }

                        if j < map[i].len() - 1 && (map[i][j + 1] == '#' || map[i][j + 1] == 'O') {
                            // turn right
                            map[i][j] = 'v';
                        }

                        if j < map[i].len() - 1 && map[i][j + 1] != '#' && map[i][j + 1] != 'O' {
                            // move forward
                            map[i][j] = '-';
                            map[i][j + 1] = '>';
                        }

                        continue 'outer;
                    }
                    '<' => {
                        if j == 0 {
                            // guard walks off the map
                            map[i][j] = '-';
                            break 'outer;
                        }

                        if j > 0 && (map[i][j - 1] == '#' || map[i][j - 1] == 'O') {
                            // turn right
                            map[i][j] = '^';
                        }

                        if j > 0 && map[i][j - 1] != '#' && map[i][j - 1] != 'O' {
                            // move forward
                            map[i][j] = '-';
                            map[i][j - 1] = '<';
                        }

                        continue 'outer;
                    }
                    _ => {}
                }
            }
        }

        if !did_move {
            break;
        }
    }

    (map, path)
}

fn is_stuck_in_loop(map: &Map) -> bool {
    let (m, path) = walk_map(&map);
    let loops = path.iter().filter(|(_, v)| **v > 2).count();

    if loops >= 4 {
        // println!("Loops: {}", loops);
        // print_map(&m);
        return true;
    } else {
        return false;
    }
}

fn neighbor_positions(pos: Position) -> Vec<Position> {
    let (i, j) = pos;

    // (0, 0) is a special case
    if i == 0 && j == 0 {
        return vec![(i + 1, j), (i, j), (i, j + 1)];
    }

    // (0, n) is a special case
    if i == 0 && j > 0 {
        return vec![(i + 1, j), (i, j), (i, j + 1), (i, j - 1)];
    }

    // (n, 0) is a special case
    if i > 0 && j == 0 {
        return vec![(i - 1, j), (i + 1, j), (i, j), (i, j + 1)];
    }

    vec![(i - 1, j), (i + 1, j), (i, j), (i, j - 1), (i, j + 1)]
}

fn part_one(input: &str) -> u32 {
    let map = parse(input);
    let start = find_start_position(&map).unwrap();
    let (map, path) = walk_map(&map);
    // print_map_with_path(&map, &path, start);
    path.len() as u32
}

fn part_two(input: &str) -> u32 {
    let map = parse(input);
    let start = find_start_position(&map).unwrap();
    let (m, path) = walk_map(&map);

    // print_map_with_path(&m, &path, start);

    let obstacles: HashSet<_> = path
        .iter()
        .flat_map(|(pos, _)| neighbor_positions(*pos))
        .filter(|p| path.contains_key(p) && p != &start.0)
        .collect();

    let obstacles_that_cause_loops: Vec<_> = obstacles
        .iter()
        .filter_map(|p| {
            let mut m = map.clone();
            let (x, y) = *p;
            m[x][y] = 'O';

            if is_stuck_in_loop(&m) {
                Some(*p)
            } else {
                None
            }
        })
        .collect();

    obstacles_that_cause_loops.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        assert_eq!(part_one(input), 41);
    }

    #[test]
    fn test_part_two() {
        let input = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

        assert_eq!(part_two(input), 6);
    }
}
