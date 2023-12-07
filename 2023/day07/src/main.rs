use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {:?}", part_one(input));
    println!("Part 2: {:?}", part_two(input));
}

fn parse(input: &str) -> Vec<(Vec<char>, usize)> {
    input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(" ").unwrap();
            (a.chars().collect::<Vec<_>>(), b.parse::<usize>().unwrap())
        })
        .collect()
}

fn strength(cards: &[char]) -> (usize, Vec<u32>) {
    let cards = cards
        .iter()
        .map(|card| match *card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        })
        .collect::<Vec<_>>();

    let tally = cards.iter().fold(HashMap::new(), |mut acc, card| {
        acc.entry(card)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        acc
    });

    let mut counts = tally.values().map(|e| *e).collect::<Vec<usize>>();
    counts.sort();

    match &counts[..] {
        // Five of a kind
        [5] => (10, cards),
        // Four of a kind
        [1, 4] => (9, cards),
        // Full house
        [2, 3] => (8, cards),
        // Three of a kind
        [1, 1, 3] => (7, cards),
        // Two pair
        [1, 2, 2] => (6, cards),
        // One pair
        [1, 1, 1, 2] => (5, cards),
        // High card
        [1, 1, 1, 1, 1] => (4, cards),
        _ => panic!("Unknown hand: {:?}", tally),
    }
}

fn part_one(input: &str) -> usize {
    let mut hands = parse(input);

    hands.sort_by_key(|(cards, _)| (strength(&cards), cards.clone()));

    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum::<usize>();

    total_winnings
}

fn strength_with_jokers(cards: &[char]) -> (usize, Vec<u32>) {
    let jokers = cards.iter().filter(|c| **c == 'J').count() as u8;

    let cards = cards
        .iter()
        .map(|card| match *card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => card.to_digit(10).unwrap(),
        })
        .collect::<Vec<_>>();

    let tally = cards.iter().fold(HashMap::new(), |mut acc, card| {
        acc.entry(card)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
        acc
    });

    let mut counts = tally.values().map(|e| *e).collect::<Vec<usize>>();
    counts.sort();

    match (&counts[..], jokers) {
        // Five of a kind
        ([5], _) => (10, cards),

        // Four of a kind
        ([1, 4], 0) => (9, cards),
        ([1, 4], 1) => (10, cards),
        ([1, 4], 4) => (10, cards),

        // Full house
        ([2, 3], 0) => (8, cards),
        ([2, 3], 2) => (10, cards),
        ([2, 3], 3) => (10, cards),

        // Three of a kind
        ([1, 1, 3], 0) => (7, cards),
        ([1, 1, 3], 1) => (9, cards),
        ([1, 1, 3], 2) => (9, cards),
        ([1, 1, 3], 3) => (9, cards),

        // Two pair
        ([1, 2, 2], 0) => (6, cards),
        ([1, 2, 2], 1) => (8, cards),
        ([1, 2, 2], 2) => (9, cards),

        // One pair
        ([1, 1, 1, 2], 0) => (5, cards),
        ([1, 1, 1, 2], 1) => (7, cards),
        ([1, 1, 1, 2], 2) => (7, cards),

        // High card
        ([1, 1, 1, 1, 1], 0) => (4, cards),
        ([1, 1, 1, 1, 1], 1) => (5, cards),

        (c, j) => panic!("Unhandled card addition {:?} + {:?}", c, j),
    }
}

fn part_two(input: &str) -> usize {
    let mut hands = parse(input);

    hands.sort_by_key(|(cards, _)| (strength_with_jokers(&cards), cards.clone()));

    println!("{:?}", hands);

    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * bid)
        .sum::<usize>();

    total_winnings
}

#[test]
fn test_part_one() {
    let input = include_str!("test.txt");
    assert_eq!(part_one(input), 6440);
}

#[test]
fn test_part_two() {
    let input = include_str!("test.txt");
    assert_eq!(part_two(input), 5905);
}
