use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn parse(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>) {
    let (directions, nodes) = input.split_once("\n\n").unwrap();

    let directions = directions.chars().collect::<Vec<_>>();

    let nodes = nodes
        .lines()
        .map(|line| {
            let loc = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];

            (loc, (left, right))
        })
        .collect::<HashMap<_, _>>();

    (directions, nodes)
}

fn part_one(input: &str) -> usize {
    let (directions, nodes) = parse(input);

    let mut steps = 0;
    let mut loc = "AAA";

    while loc != "ZZZ" {
        let dir = directions[steps % directions.len()];

        let (left, right) = nodes.get(loc).unwrap();

        match dir {
            'L' => loc = left,
            'R' => loc = right,
            _ => panic!("Unknown direction {}", dir),
        }

        steps += 1;
    }

    steps
}

fn part_two(input: &str) -> i64 {
    let (directions, nodes) = parse(input);

    let locs = nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    let steps = locs.iter().map(|l| {
        let mut loc = l;
        let mut steps = 0;

        while !loc.ends_with('Z') {
            let dir = directions[steps % directions.len()];

            let (left, right) = nodes.get(loc).unwrap();

            match dir {
                'L' => loc = left,
                'R' => loc = right,
                _ => panic!("Unknown direction {}", dir),
            }

            steps += 1;
        }

        steps
    });

    println!("{:?}", steps);

    steps.fold(1, |acc, s| lcm(acc as i64, s as i64))
}

// https://en.wikipedia.org/wiki/Greatest_common_divisor
// https://en.wikipedia.org/wiki/Euclidean_algorithm
fn gcd(a: i64, b: i64) -> i64 {
    match b {
        0 => a,
        // Rerun the function with b and the remainder of a / b
        _ => gcd(b, a % b),
    }
}

// https://en.wikipedia.org/wiki/Least_common_multiple
pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

#[test]
fn test_part_one_a() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    assert_eq!(part_one(input), 2);
}

#[test]
fn test_part_one_b() {
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    assert_eq!(part_one(input), 6);
}

#[test]
fn test_part_two() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    assert_eq!(part_two(input), 6);
}
