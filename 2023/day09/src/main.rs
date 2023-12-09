fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn deltas(input: Vec<Vec<i64>>) -> Vec<Vec<Vec<i64>>> {
    input
        .iter()
        .map(|history| {
            let mut deltas: Vec<i64> = history.clone();
            let mut sequences: Vec<Vec<i64>> = vec![deltas.clone()];

            while !deltas.iter().all(|n| *n == 0) {
                deltas = deltas
                    .iter()
                    .zip(deltas.iter().skip(1))
                    .map(|(a, b)| b - a)
                    .collect::<Vec<i64>>();

                sequences.push(deltas.clone());
            }

            sequences
        })
        .collect()
}

fn part_one(input: &str) -> i64 {
    let histories = parse(input);
    let series = deltas(histories);

    series
        .iter()
        .map(|serie| {
            serie
                .windows(2)
                .rev()
                .fold(0, |acc, w| acc + w[0].last().unwrap())
        })
        .sum()
}

#[test]
fn test_part_one() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    assert_eq!(part_one(input), 114);
}

fn part_two(input: &str) -> i64 {
    let histories = parse(input);
    let series = deltas(histories);

    series
        .iter()
        .map(|serie| {
            serie
                .windows(2)
                .rev()
                .fold(0, |acc, w| w[0].first().unwrap() - acc)
        })
        .sum()
}

#[test]
fn test_part_two() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    assert_eq!(part_two(input), 2);
}
