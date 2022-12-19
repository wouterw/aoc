use std::collections::HashSet;

static DIRECTIONS: [(isize, isize); 4] = [
    (-1, 0), // UP
    (0, 1),  // RIGHT
    (1, 0),  // DOWN
    (0, -1), // LEFT
];

fn main() {
    let input = include_str!("input.txt");
    let height_map = parse(input);
    println!("Part 1: {:?}", number_of_visible_trees(&height_map));
    println!("Part 2: {:?}", highest_scenic_score(&height_map));
}

type Heightmap = Vec<Vec<Tree>>;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Tree {
    r: usize,
    c: usize,
    h: usize,
}

fn parse(input: &str) -> Heightmap {
    input
        .lines()
        .enumerate()
        .map(|(r, l)| {
            l.chars()
                .enumerate()
                .map(|(i, c)| Tree {
                    r: r,
                    c: i,
                    h: c.to_digit(10).unwrap() as usize,
                })
                .collect()
        })
        .collect()
}

fn dimensions(height_map: &Heightmap) -> (usize, usize) {
    // (columns, rows)
    (height_map.len(), height_map[0].len())
}

fn number_of_visible_trees(height_map: &Heightmap) -> usize {
    let mut trees_in_line_of_sight: HashSet<&Tree> = HashSet::new();

    let (columns, rows) = dimensions(height_map);

    // west (left-to-right)
    for row in 0..rows {
        let mut h = 0;

        for column in 0..columns {
            let tree = &height_map[row][column];

            if tree.h > h || column == 0 {
                h = tree.h;
                trees_in_line_of_sight.insert(tree);
            }
        }
    }

    // east (right-to-left)
    for row in 0..rows {
        let mut h = 0;

        for column in (0..columns).rev() {
            let tree = &height_map[row][column];

            if tree.h > h || column == columns - 1 {
                h = tree.h;
                trees_in_line_of_sight.insert(tree);
            }
        }
    }

    // north (top-to-bottom)
    for column in 0..columns {
        let mut h = 0;

        for row in 0..rows {
            let tree = &height_map[row][column];

            if tree.h > h || row == 0 {
                h = tree.h;
                trees_in_line_of_sight.insert(tree);
            }
        }
    }

    // south (bottom-to-top)
    for column in 0..columns {
        let mut h = 0;

        for row in (0..rows).rev() {
            let tree = &height_map[row][column];

            if tree.h > h || row == rows - 1 {
                h = tree.h;
                trees_in_line_of_sight.insert(tree);
            }
        }
    }

    trees_in_line_of_sight.len()
}

fn senic_score(height_map: &Heightmap, tree: &Tree) -> usize {
    let (columns, rows) = dimensions(height_map);

    DIRECTIONS
        .iter()
        .map(|direction| {
            let mut trees_in_line_of_sight = 0;

            let mut p: (isize, isize) = (tree.r as isize, tree.c as isize);
            let mut h;

            loop {
                p = (p.0 + direction.0, p.1 + direction.1);

                if p.0 < 0 || p.0 >= rows as isize || p.1 < 0 || p.1 >= columns as isize {
                    break;
                }

                trees_in_line_of_sight += 1;

                h = height_map[p.0 as usize][p.1 as usize].h;

                if h >= tree.h {
                    break;
                }
            }

            trees_in_line_of_sight
        })
        .product()
}

fn highest_scenic_score(height_map: &Heightmap) -> usize {
    height_map
        .iter()
        .flat_map(|row| row)
        .map(|tree| senic_score(height_map, tree))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let input = include_str!("test.txt");
        let height_map = parse(input);
        assert_eq!(number_of_visible_trees(&height_map), 21);
    }

    #[test]
    fn part_two() {
        let input = include_str!("test.txt");
        let height_map = parse(input);
        assert_eq!(highest_scenic_score(&height_map), 8);
    }
}
