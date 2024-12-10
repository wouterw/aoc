fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

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

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_one(input: &str) -> u32 {
    let grid = parse_input(input);

    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    let mut found = 0;

    for i in 0..h {
        for j in 0..w {
            if grid[i as usize][j as usize] != 'X' {
                continue;
            }

            for (dx, dy) in DIRECTIONS.iter() {
                let mut x = i as isize + dx;
                let mut y = j as isize + dy;

                let mut n = 1;

                while x >= 0 && x < h && y >= 0 && y < w {
                    if grid[x as usize][y as usize] != XMAS[n] {
                        break;
                    }

                    if n == 3 {
                        found += 1;
                        break;
                    }

                    x += dx;
                    y += dy;
                    n += 1;
                }
            }
        }
    }

    found
}

fn part_two(input: &str) -> u32 {
    let grid = parse_input(input);

    let h = grid.len() as isize;
    let w = grid[0].len() as isize;

    let mut found = 0;

    for i in 0..h {
        for j in 0..w {
            if grid[i as usize][j as usize] != 'A' {
                continue;
            }

            if !(i >= 1 && i < h - 1 && j >= 1 && j < w - 1) {
                continue;
            }

            // top-left to bottom-right diagonal
            let top_left = grid[(i - 1) as usize][(j - 1) as usize];
            let bottom_right = grid[(i + 1) as usize][(j + 1) as usize];

            // top-right and bottom-left diagonal
            let top_right = grid[(i - 1) as usize][(j + 1) as usize];
            let bottom_left = grid[(i + 1) as usize][(j - 1) as usize];

            if ((top_left == 'M' && bottom_right == 'S')
                || (top_left == 'S' && bottom_right == 'M'))
                && ((top_right == 'M' && bottom_left == 'S')
                    || (top_right == 'S' && bottom_left == 'M'))
            {
                found += 1;
            }
        }
    }

    found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        assert_eq!(part_one(input), 18);
    }

    #[test]
    fn test_part_two() {
        let input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

        assert_eq!(part_two(input), 9);
    }
}
