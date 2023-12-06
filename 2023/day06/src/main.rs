fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn parse_numbers(s: &str) -> Vec<usize> {
    s.split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

fn part_one(input: &str) -> usize {
    let (times, distances) = input.split_once('\n').unwrap();

    let times = parse_numbers(times);
    let distances = parse_numbers(distances);

    let races = times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| (*t, *d))
        .collect::<Vec<_>>();

    races
        .iter()
        .map(|(duration, record)| {
            (1..*duration).fold(0, |acc, t| {
                let speed = duration - t;
                let remaining_time = duration - speed;
                let distance = speed * remaining_time;

                if distance > *record {
                    acc + 1
                } else {
                    acc
                }
            })
        })
        .product::<usize>()
}

fn parse_number(s: &str) -> usize {
    s.chars()
        .filter(char::is_ascii_digit)
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn part_two(input: &str) -> usize {
    let (a, b) = input.split_once('\n').unwrap();

    let duration = parse_number(a);
    let record = parse_number(b);

    (1..duration).fold(0, |acc, t| {
        let speed = duration - t;
        let remaining_time = duration - speed;
        let distance = speed * remaining_time;

        if distance > record {
            acc + 1
        } else {
            acc
        }
    })
}

#[test]
fn test_part_one() {
    let input = include_str!("test.txt");
    assert_eq!(part_one(input), 288);
}

#[test]
fn test_part_two() {
    let input = include_str!("test.txt");
    assert_eq!(part_two(input), 71503);
}
