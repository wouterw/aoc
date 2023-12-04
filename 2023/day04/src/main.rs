use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn parse(input: &str) -> Vec<Vec<HashSet<usize>>> {
    input
        .lines()
        .map(|card| {
            card.split(":")
                .nth(1)
                .unwrap()
                .split("|")
                .map(|part| {
                    part.trim()
                        .split(" ")
                        .filter(|s| !s.is_empty())
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn part_one(input: &str) -> usize {
    let cards = parse(input);

    cards
        .iter()
        .flat_map(|card| {
            let winning_numbers = &card[0];
            let drawn_numbers = &card[1];

            let intersection: HashSet<_> = drawn_numbers.intersection(&winning_numbers).collect();

            if intersection.len() > 0 {
                Some(intersection.len())
            } else {
                None
            }
        })
        .fold(0, |acc, score| {
            let x: usize = 2;
            acc + x.pow((score - 1) as u32)
        })
}

fn part_two(input: &str) -> usize {
    let pile = parse(input);
    let mut copies = vec![1; pile.len()];

    for i in 0..pile.len() {
        let winning_numbers = &pile[i][0];
        let drawn_numbers = &pile[i][1];
        let intersection: HashSet<_> = drawn_numbers.intersection(&winning_numbers).collect();

        for j in 0..intersection.len() {
            copies[i + j + 1] += copies[i];
        }
    }

    copies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = include_str!("test.txt");
        assert_eq!(part_one(input), 13);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("test.txt");
        assert_eq!(part_two(input), 30);
    }
}
