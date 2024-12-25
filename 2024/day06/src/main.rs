use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
}

fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn print_map(map: &Vec<Vec<char>>) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            print!("{}", map[i][j]);
        }
        println!();
    }
}

fn print_map_with_path(map: &Vec<Vec<char>>, path: &HashSet<(usize, usize)>) {
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if path.contains(&(i, j)) {
                print!("X");
            } else {
                print!("{}", map[i][j]);
            }
        }
        println!();
    }
}

fn part_one(input: &str) -> u32 {
    let mut map = parse(input);

    let mut path: HashSet<(usize, usize)> = HashSet::new();
    let mut n = 0;

    'outer: loop {
        println!("{:?}", n);
        print_map(&map);

        for i in 0..map.len() {
            for j in 0..map[i].len() {
                match map[i][j] {
                    '^' => {
                        path.insert((i, j));

                        if i == 0 {
                            // guard walks off the map
                            break 'outer;
                        }

                        if i > 0 && map[i - 1][j] == '#' {
                            // turn right
                            map[i][j] = '>';
                        }

                        if i > 0 && map[i - 1][j] == '.' {
                            // move forward
                            map[i][j] = '.';
                            map[i - 1][j] = '^';
                        }

                        n += 1;
                        continue 'outer;
                    }
                    'v' => {
                        path.insert((i, j));

                        if i == map.len() - 1 {
                            // guard walks off the map
                            break 'outer;
                        }

                        if i < map.len() - 1 && map[i + 1][j] == '#' {
                            // turn right
                            map[i][j] = '<';
                        }

                        if i < map.len() - 1 && map[i + 1][j] == '.' {
                            // move forward
                            map[i][j] = '.';
                            map[i + 1][j] = 'v';
                        }

                        n += 1;
                        continue 'outer;
                    }
                    '>' => {
                        path.insert((i, j));

                        if j == map[i].len() - 1 {
                            // guard walks off the map
                            break 'outer;
                        }

                        if j < map[i].len() - 1 && map[i][j + 1] == '#' {
                            // turn right
                            map[i][j] = 'v';
                        }

                        if j < map[i].len() - 1 && map[i][j + 1] == '.' {
                            // move forward
                            map[i][j] = '.';
                            map[i][j + 1] = '>';
                        }

                        n += 1;
                        continue 'outer;
                    }
                    '<' => {
                        path.insert((i, j));

                        if j == 0 {
                            // guard walks off the map
                            break 'outer;
                        }

                        if j > 0 && map[i][j - 1] == '#' {
                            // turn right
                            map[i][j] = '^';
                        }

                        if j > 0 && map[i][j - 1] == '.' {
                            // move forward
                            map[i][j] = '.';
                            map[i][j - 1] = '<';
                        }

                        n += 1;
                        continue 'outer;
                    }
                    _ => {}
                }
            }
        }
    }

    println!("{:?}", path);
    print_map_with_path(&map, &path);

    path.len() as u32
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
}
