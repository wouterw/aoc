fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn part_one(input: &str) -> isize {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

#[test]
fn test_part_one() {
    assert_eq!(part_one("(())"), 0);
    assert_eq!(part_one("()()"), 0);
    assert_eq!(part_one("((("), 3);
    assert_eq!(part_one("(()(()("), 3);
    assert_eq!(part_one("))((((("), 3);
    assert_eq!(part_one("())"), -1);
    assert_eq!(part_one("))("), -1);
    assert_eq!(part_one(")))"), -3);
    assert_eq!(part_one(")())())"), -3);
}

fn part_two(input: &str) -> isize {
    let mut floor = 0;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if floor == -1 {
            return (i + 1) as isize;
        }
    }

    -1
}

#[test]
fn test_part_two() {
    assert_eq!(part_two(")"), 1);
    assert_eq!(part_two("()())"), 5);
}
