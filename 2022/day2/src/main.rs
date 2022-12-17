use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    let moves: Vec<(char, char)> = input
        .lines()
        .map(|s| {
            let chars: Vec<char> = s.chars().filter(|c| !c.is_whitespace()).collect();
            (chars[0], chars[1])
        })
        .collect();

    let total_score = total_score_according_to_strategy(&moves);
    let total_score_2 = total_score_according_to_strategy_2(&moves);

    println!("Part 1: {:?}", total_score);
    println!("Part 2: {:?}", total_score_2);
}

fn total_score_according_to_strategy(moves: &Vec<(char, char)>) -> usize {
    let mut score: usize = 0;
    let points_per_move: HashMap<char, usize> = HashMap::from([('X', 1), ('Y', 2), ('Z', 3)]);

    for m in moves {
        // Opponent plays ROCK
        if m.0 == 'A' {
            match m.1 {
                'X' => score += points_per_move.get(&m.1).unwrap() + 3, // ROCK
                'Y' => score += points_per_move.get(&m.1).unwrap() + 6, // PAPER
                'Z' => score += points_per_move.get(&m.1).unwrap(),     // SCISSORS
                _ => panic!("derp"),
            }
        }

        // Opponent plays PAPER
        if m.0 == 'B' {
            match m.1 {
                'X' => score += points_per_move.get(&m.1).unwrap(), // ROCK
                'Y' => score += points_per_move.get(&m.1).unwrap() + 3, // PAPER
                'Z' => score += points_per_move.get(&m.1).unwrap() + 6, // SCISSORS
                _ => panic!("derp"),
            }
        }

        // Opponent plays SCISSORS
        if m.0 == 'C' {
            match m.1 {
                'X' => score += points_per_move.get(&m.1).unwrap() + 6, // ROCK
                'Y' => score += points_per_move.get(&m.1).unwrap(),     // PAPER
                'Z' => score += points_per_move.get(&m.1).unwrap() + 3, // SCISSORS
                _ => panic!("derp"),
            }
        }
    }

    score
}

fn total_score_according_to_strategy_2(moves: &Vec<(char, char)>) -> usize {
    let mut score: usize = 0;
    let points_per_move: HashMap<char, usize> = HashMap::from([('A', 1), ('B', 2), ('C', 3)]);

    for m in moves {
        // You need to loss
        if m.1 == 'X' {
            match m.0 {
                'A' => score += points_per_move.get(&'C').unwrap(), // ROCK
                'B' => score += points_per_move.get(&'A').unwrap(), // PAPER
                'C' => score += points_per_move.get(&'B').unwrap(), // SCISSORS
                _ => panic!("derp"),
            }
        }

        // You need to draw
        if m.1 == 'Y' {
            match m.0 {
                'A' => score += points_per_move.get(&'A').unwrap() + 3, // ROCK
                'B' => score += points_per_move.get(&'B').unwrap() + 3, // PAPER
                'C' => score += points_per_move.get(&'C').unwrap() + 3, // SCISSORS
                _ => panic!("derp"),
            }
        }

        // You need to win
        if m.1 == 'Z' {
            match m.0 {
                'A' => score += points_per_move.get(&'B').unwrap() + 6, // ROCK
                'B' => score += points_per_move.get(&'C').unwrap() + 6, // PAPER
                'C' => score += points_per_move.get(&'A').unwrap() + 6, // SCISSORS
                _ => panic!("derp"),
            }
        }
    }

    score
}
