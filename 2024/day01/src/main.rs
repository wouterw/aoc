fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];

    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    });

    left.sort();
    right.sort();

    (left, right)
}

fn part_one(input: &str) -> usize {
    let input = parse(input);

    let distance = input
        .0
        .iter()
        .zip(input.1)
        .fold(0, |acc, (l, r)| acc + l.abs_diff(r));

    distance
}

fn part_two(input: &str) -> usize {
    let input = parse(input);

    let similarity = input.0.iter().fold(0, |acc, n| {
        let occurences = input.1.iter().filter(|&x| x == n).count();
        acc + n * occurences
    });

    similarity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        assert_eq!(part_one(input), 11);
    }

    #[test]
    fn test_part_two() {
        let input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

        assert_eq!(part_two(input), 31);
    }
}
